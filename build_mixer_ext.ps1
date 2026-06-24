# PvZ Portable Rust — Build SDL2_mixer_ext.dll from SDL-Mixer-X source
param([switch]$SkipBuild)

$RustDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $RustDir
$MixerXDir = Join-Path $ProjectRoot "src/SexyAppFramework/sound/SDL-Mixer-X"
$TargetDir = Join-Path $RustDir "target/debug"
$BuildDir = Join-Path $RustDir "target/mixer_build"
$VcpkgDir = Join-Path $RustDir "target/vcpkg"
$VcpkgTriplet = "x64-windows"

Write-Host "=== Build SDL2_mixer_ext.dll with libopenmpt (MO3) ==="

# 0. 确保 vcpkg 存在
$VcpkgExe = $null
$envRoot = [Environment]::GetEnvironmentVariable("VCPKG_ROOT", "Machine")
if ($envRoot -and (Test-Path (Join-Path $envRoot "vcpkg.exe"))) {
    $VcpkgExe = Join-Path $envRoot "vcpkg.exe"
} elseif (Test-Path (Join-Path $VcpkgDir "vcpkg.exe")) {
    $VcpkgExe = Join-Path $VcpkgDir "vcpkg.exe"
}

if (-not $VcpkgExe) {
    Write-Host "正在克隆 vcpkg..."
    git clone https://github.com/Microsoft/vcpkg.git $VcpkgDir 2>&1 | Out-Null
    & (Join-Path $VcpkgDir "bootstrap-vcpkg.bat") 2>&1 | Out-Null
    $VcpkgExe = Join-Path $VcpkgDir "vcpkg.exe"
}

if (-not (Test-Path $VcpkgExe)) { Write-Host "vcpkg 安装失败"; exit 1 }
Write-Host "vcpkg: $VcpkgExe"

# 1. 安装 libopenmpt（含 MO3 解码支持）
Write-Host "安装 libopenmpt:x64-windows..."
& $VcpkgExe install libopenmpt:$VcpkgTriplet --recurse 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) { Write-Host "libopenmpt 安装失败"; exit 1 }
Write-Host "libopenmpt 安装完成"

# 设置 vcpkg toolchain 供后续 CMake 使用
$VcpkgCmake = Join-Path $VcpkgDir "scripts/buildsystems/vcpkg.cmake"

# 1. find CMake
$cmake = Get-Command "cmake" -ErrorAction SilentlyContinue
if (-not $cmake) { Write-Host "CMake not found"; exit 1 }
Write-Host "CMake: $($cmake.Source)"

# 2. find sdl2-sys bundled SDL2
$SdlOutDir = $null
$SdlCmakeDir = $null
foreach ($dir in Get-ChildItem "$RustDir\target\debug\build\sdl2-sys-*" -Directory) {
    $testPath = Join-Path $dir.FullName "out"
    if (Test-Path (Join-Path $testPath "include/SDL2/SDL.h")) {
        $SdlOutDir = $testPath
        $SdlCmakeDir = Join-Path $testPath "cmake"
        if (-not (Test-Path $SdlCmakeDir)) { $SdlCmakeDir = Join-Path $testPath "build" }
        break
    }
}
if (-not $SdlOutDir) { Write-Host "sdl2-sys build output not found"; exit 1 }

Write-Host "SDL2 out: $SdlOutDir"
Write-Host "SDL2 cmake: $SdlCmakeDir"

# 3. patch SDL-Mixer-X CMakeLists.txt for CMake 4.x compatibility
$MixerCMake = Join-Path $MixerXDir "CMakeLists.txt"
$MixerCMakeContent = Get-Content $MixerCMake -Raw
if ($MixerCMakeContent -notmatch "cmake_minimum_required") {
    $MixerCMakeContent = "cmake_minimum_required(VERSION 3.5)`n$MixerCMakeContent"
    Set-Content -Path $MixerCMake -Value $MixerCMakeContent -NoNewline
    Write-Host "Patched CMakeLists.txt for CMake 4.x"
}

# 4. clean build dir
if (Test-Path $BuildDir) { Remove-Item -Recurse -Force $BuildDir }
New-Item -ItemType Directory -Force -Path $BuildDir | Out-Null

# 4. CMake configure
$CmakeArgs = @(
    "-S", $MixerXDir, "-B", $BuildDir,
    "-G", "Ninja",
    "-DCMAKE_BUILD_TYPE=Release",
    "-DCMAKE_POLICY_VERSION_MINIMUM=3.5",
    "-DSDL_MIXER_X_STATIC=OFF",
    "-DSDL_MIXER_X_SHARED=ON",
    "-DSDL2_DIR=$SdlCmakeDir",
    "-DUSE_MIDI=OFF",
    "-DUSE_MODPLUG=OFF",
    "-DUSE_XMP=OFF",
    "-DUSE_OPUS=OFF",
    "-DUSE_DRFLAC=ON",
    "-DUSE_PXTONE=OFF",
    "-DUSE_VORBIS=OFF",
    "-DUSE_MPG123=OFF",
    # 如果已安装 libopenmpt（通过 vcpkg），CMake 会自动找到并启用 MO3 支持
    "-DCMAKE_DISABLE_FIND_PACKAGE_Ogg=ON",
    "-DCMAKE_DISABLE_FIND_PACKAGE_Vorbis=ON",
    "-DCMAKE_DISABLE_FIND_PACKAGE_FLAC=ON",
    "-DCMAKE_DISABLE_FIND_PACKAGE_MPG123=ON",
    "-DCMAKE_DISABLE_FIND_PACKAGE_Opus=ON",
    "-DCMAKE_DISABLE_FIND_PACKAGE_ModPlug=ON",
    "-DCMAKE_DISABLE_FIND_PACKAGE_XMP=ON",
    "-DCMAKE_DISABLE_FIND_PACKAGE_FluidLite=ON",
    "-DCMAKE_DISABLE_FIND_PACKAGE_FluidSynth=ON",
    "-DCMAKE_DISABLE_FIND_PACKAGE_GME=ON"
)

Write-Host "Configuring CMake..."
& cmake @CmakeArgs 2>&1
if ($LASTEXITCODE -ne 0) { Write-Host "CMake configure failed"; exit 1 }

# 5. Build
Write-Host "Building..."
& cmake --build $BuildDir --config Release 2>&1
if ($LASTEXITCODE -ne 0) { Write-Host "Build failed"; exit 1 }

# 6. Find and copy DLL
$OutputDll = Get-ChildItem -Path $BuildDir -Filter "SDL2_mixer_ext.dll" -Recurse | Select-Object -First 1
if (-not $OutputDll) { Write-Host "SDL2_mixer_ext.dll not found in build output"; exit 1 }

Write-Host "Built: $($OutputDll.FullName) ($($OutputDll.Length) bytes)"
Copy-Item -Force $OutputDll.FullName (Join-Path $TargetDir "SDL2_mixer_ext.dll")
Write-Host "Copied to: $TargetDir/SDL2_mixer_ext.dll"
Write-Host "=== Done ==="
