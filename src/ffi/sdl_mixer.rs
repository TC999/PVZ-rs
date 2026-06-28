// SDL_mixer FFI 绑定 — 通过 sdl2::sys::mixer 模块编译时链接
//
// Windows: SDL2_mixer.lib 由 build.rs 从 dev 包提供
// Linux: 通过 pkg-config / -lSDL2_mixer 链接系统库
//
// 保持与原有 sdl_mixer.rs 相同的函数签名（参数为 *mut c_void），
// 内部转换为 sdl2-sys 的具体类型，避免调用方大量修改

#![allow(non_camel_case_types, dead_code)]

use std::os::raw::{c_int, c_uchar, c_void, c_char};

// 为向后兼容保留的类型别名（调用方以 *mut c_void 方式使用）
// sdl2-sys 中的实际类型是 mixer::Mix_Chunk (struct) 和 _Mix_Music (opaque)

pub const MAX_VOLUME: i32 = 128;

// ============ 编译时链接的 SDL2_mixer 函数 ============
// 所有内部转换均基于 sdl2::sys::mixer 模块

pub fn open_audio(freq: c_int, fmt: u16, ch: c_int, size: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_OpenAudio(freq, fmt, ch, size) }
}

pub fn close_audio() {
    unsafe { sdl2::sys::mixer::Mix_CloseAudio() }
}

pub fn allocate_channels(n: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_AllocateChannels(n) }
}

pub fn load_wav_rw(rw: *mut c_void, freesrc: c_int) -> *mut c_void {
    unsafe {
        sdl2::sys::mixer::Mix_LoadWAV_RW(rw as *mut sdl2::sys::SDL_RWops, freesrc) as *mut c_void
    }
}

pub fn free_chunk(c: *mut c_void) {
    unsafe { sdl2::sys::mixer::Mix_FreeChunk(c as *mut sdl2::sys::mixer::Mix_Chunk) }
}

pub fn play_channel(ch: c_int, ck: *mut c_void, loops: c_int) -> c_int {
    unsafe {
        sdl2::sys::mixer::Mix_PlayChannelTimed(ch, ck as *mut sdl2::sys::mixer::Mix_Chunk, loops, -1)
    }
}

pub fn halt_channel(ch: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_HaltChannel(ch) }
}

pub fn volume(ch: c_int, vol: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_Volume(ch, vol) }
}

pub fn set_panning(ch: c_int, l: u8, r: u8) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_SetPanning(ch, l, r) }
}

pub fn load_mus(path: *const c_char) -> *mut c_void {
    unsafe { sdl2::sys::mixer::Mix_LoadMUS(path) as *mut c_void }
}

pub fn free_music(m: *mut c_void) {
    unsafe { sdl2::sys::mixer::Mix_FreeMusic(m as *mut sdl2::sys::mixer::Mix_Music) }
}

pub fn play_music(m: *mut c_void, loops: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_PlayMusic(m as *mut sdl2::sys::mixer::Mix_Music, loops) }
}

pub fn halt_music() -> c_int {
    unsafe { sdl2::sys::mixer::Mix_HaltMusic() }
}

pub fn volume_music(v: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_VolumeMusic(v) }
}

pub fn pause_music() {
    unsafe { sdl2::sys::mixer::Mix_PauseMusic() }
}

pub fn resume_music() {
    unsafe { sdl2::sys::mixer::Mix_ResumeMusic() }
}

/// SDL_RWFromConstMem — 通过 sdl2-sys 编译时链接
pub unsafe fn rw_from_constmem(data: *const c_void, size: c_int) -> *mut c_void {
    sdl2::sys::SDL_RWFromConstMem(data, size) as *mut c_void
}

/// 总是可用（编译时链接不存在运行时加载问题）
pub fn is_available() -> bool {
    true
}

/// 不再需要运行时加载
pub fn load_library() -> bool {
    true
}

// ============ SDL2_mixer_ext (Mixer X) 扩展 FFI ============
// 以下函数来自 SDL2_mixer_ext，由 SDL2_mixer.dll 导出
// 在 Windows 上通过 build.rs 下载的 SDL2_mixer 开发包提供

extern "C" {
    // ── 音乐效果 ──
    fn Mix_SetMusicEffectPosition(music: *mut c_void, angle: i16, distance: u8) -> c_int;
    fn Mix_SetMusicEffectDistance(music: *mut c_void, distance: u8) -> c_int;
    fn Mix_SetMusicEffectReverseStereo(music: *mut c_void, flip: c_int) -> c_int;

    // ── Mod 音乐 ──
    fn Mix_ModMusicSetChannelMute(channel: c_int, mute: c_int) -> c_int;
    fn Mix_ModMusicSetChannelVolume(channel: c_int, volume: c_int) -> c_int;

    // ── 轨道控制 ──
    fn Mix_StartTrack(music: *mut c_void, track: c_int) -> c_int;
    fn Mix_GetNumTracks(music: *mut c_void) -> c_int;
    fn Mix_SetMusicPositionStream(music: *mut c_void, position: f64) -> c_int;

    // ── 音乐速度/时间 ──
    fn Mix_SetMusicTempo(music: *mut c_void, tempo: f64) -> c_int;
    fn Mix_GetMusicTempo(music: *mut c_void) -> f64;
    fn Mix_GetMusicLoopStartTime(music: *mut c_void) -> f64;

    // ── ADLMIDI ──
    fn Mix_ADLMIDI_getBankNames() -> *mut *const c_char;
    fn Mix_ADLMIDI_getBankID() -> c_int;
    fn Mix_ADLMIDI_setBankID(bnk: c_int);
    fn Mix_ADLMIDI_getTremolo() -> c_int;
    fn Mix_ADLMIDI_setTremolo(tr: c_int);
    fn Mix_ADLMIDI_getVibrato() -> c_int;
    fn Mix_ADLMIDI_setVibrato(vib: c_int);
    fn Mix_ADLMIDI_getScaleMod() -> c_int;
    fn Mix_ADLMIDI_setScaleMod(sc: c_int);
    fn Mix_ADLMIDI_getAdLibMode() -> c_int;
    fn Mix_ADLMIDI_setAdLibMode(mode: c_int);
    fn Mix_ADLMIDI_getLogarithmicVolumes() -> c_int;
    fn Mix_ADLMIDI_setLogarithmicVolumes(lv: c_int);
    fn Mix_ADLMIDI_getVolumeModel() -> c_int;
    fn Mix_ADLMIDI_setVolumeModel(vm: c_int);
    fn Mix_ADLMIDI_getFullRangeBrightness() -> c_int;
    fn Mix_ADLMIDI_setFullRangeBrightness(frb: c_int);
    fn Mix_ADLMIDI_getAutoArpeggio() -> c_int;
    fn Mix_ADLMIDI_setAutoArpeggio(aa_en: c_int);
    fn Mix_ADLMIDI_getChannelAllocMode() -> c_int;
    fn Mix_ADLMIDI_setChannelAllocMode(ca: c_int);
    fn Mix_ADLMIDI_getFullPanStereo() -> c_int;
    fn Mix_ADLMIDI_setFullPanStereo(fp: c_int);
    fn Mix_ADLMIDI_getEmulator() -> c_int;
    fn Mix_ADLMIDI_setEmulator(emu: c_int);
    fn Mix_ADLMIDI_getChipsCount() -> c_int;
    fn Mix_ADLMIDI_setChipsCount(chips: c_int);
    fn Mix_ADLMIDI_setSetDefaults();
    fn Mix_ADLMIDI_setCustomBankFile(path: *const c_char);

    // ── OPNMIDI ──
    fn Mix_OPNMIDI_setSetDefaults();
    fn Mix_OPNMIDI_getVolumeModel() -> c_int;
    fn Mix_OPNMIDI_setVolumeModel(vm: c_int);
    fn Mix_OPNMIDI_getFullRangeBrightness() -> c_int;
    fn Mix_OPNMIDI_setFullRangeBrightness(frb: c_int);
    fn Mix_OPNMIDI_getAutoArpeggio() -> c_int;
    fn Mix_OPNMIDI_setAutoArpeggio(aa_en: c_int);
    fn Mix_OPNMIDI_getChannelAllocMode() -> c_int;
    fn Mix_OPNMIDI_setChannelAllocMode(ca: c_int);
    fn Mix_OPNMIDI_getFullPanStereo() -> c_int;
    fn Mix_OPNMIDI_setFullPanStereo(fp: c_int);
    fn Mix_OPNMIDI_getEmulator() -> c_int;
    fn Mix_OPNMIDI_setEmulator(emu: c_int);
    fn Mix_OPNMIDI_getChipsCount() -> c_int;
    fn Mix_OPNMIDI_setChipsCount(chips: c_int);
    fn Mix_OPNMIDI_setCustomBankFile(path: *const c_char);

    // ── GME ──
    fn Mix_GME_SetSpcEchoDisabled(music: *mut c_void, disabled: *mut c_int);
    fn Mix_GME_GetSpcEchoDisabled(music: *mut c_void) -> c_int;

    // ── MIDI 播放器选择 ──
    fn Mix_GetMidiPlayer() -> c_int;
    fn Mix_GetNextMidiPlayer(player: c_int) -> c_int;
    fn Mix_SetMidiPlayer(player: c_int) -> c_int;
    fn Mix_SetLockMIDIArgs(lock: c_int);
    fn Mix_GetMidiDevice() -> *const c_char;
    fn Mix_GetNextMidiDevice(device: *const c_char) -> *const c_char;
    fn Mix_SetMidiDevice(device: *const c_char) -> c_int;
}

// ── 安全 Rust 包装 ──

pub fn set_music_effect_position(music: *mut c_void, angle: i16, distance: u8) -> c_int {
    unsafe { Mix_SetMusicEffectPosition(music, angle, distance) }
}

pub fn set_music_effect_distance(music: *mut c_void, distance: u8) -> c_int {
    unsafe { Mix_SetMusicEffectDistance(music, distance) }
}

pub fn set_music_effect_reverse_stereo(music: *mut c_void, flip: c_int) -> c_int {
    unsafe { Mix_SetMusicEffectReverseStereo(music, flip) }
}

pub fn reserve_channels(num: c_int) -> c_int {
    unsafe { sdl2::sys::mixer::Mix_ReserveChannels(num) }
}

pub fn mod_music_set_channel_mute(channel: c_int, mute: c_int) -> c_int {
    unsafe { Mix_ModMusicSetChannelMute(channel, mute) }
}

pub fn mod_music_set_channel_volume(channel: c_int, volume: c_int) -> c_int {
    unsafe { Mix_ModMusicSetChannelVolume(channel, volume) }
}

pub fn start_track(music: *mut c_void, track: c_int) -> c_int {
    unsafe { Mix_StartTrack(music, track) }
}

pub fn get_num_tracks(music: *mut c_void) -> c_int {
    unsafe { Mix_GetNumTracks(music) }
}

pub fn set_music_position_stream(music: *mut c_void, position: f64) -> c_int {
    unsafe { Mix_SetMusicPositionStream(music, position) }
}

pub fn set_music_tempo(music: *mut c_void, tempo: f64) -> c_int {
    unsafe { Mix_SetMusicTempo(music, tempo) }
}

pub fn get_music_tempo(music: *mut c_void) -> f64 {
    unsafe { Mix_GetMusicTempo(music) }
}

pub fn get_music_loop_start_time(music: *mut c_void) -> f64 {
    unsafe { Mix_GetMusicLoopStartTime(music) }
}

// ── ADLMIDI 封装 ──

pub fn adlmidi_get_bank_names() -> *mut *const c_char {
    unsafe { Mix_ADLMIDI_getBankNames() }
}

pub fn adlmidi_get_bank_id() -> c_int {
    unsafe { Mix_ADLMIDI_getBankID() }
}

pub fn adlmidi_set_bank_id(bnk: c_int) {
    unsafe { Mix_ADLMIDI_setBankID(bnk) }
}

pub fn adlmidi_get_tremolo() -> c_int {
    unsafe { Mix_ADLMIDI_getTremolo() }
}

pub fn adlmidi_set_tremolo(tr: c_int) {
    unsafe { Mix_ADLMIDI_setTremolo(tr) }
}

pub fn adlmidi_get_vibrato() -> c_int {
    unsafe { Mix_ADLMIDI_getVibrato() }
}

pub fn adlmidi_set_vibrato(vib: c_int) {
    unsafe { Mix_ADLMIDI_setVibrato(vib) }
}

pub fn adlmidi_get_scale_mod() -> c_int {
    unsafe { Mix_ADLMIDI_getScaleMod() }
}

pub fn adlmidi_set_scale_mod(sc: c_int) {
    unsafe { Mix_ADLMIDI_setScaleMod(sc) }
}

pub fn adlmidi_get_adlib_mode() -> c_int {
    unsafe { Mix_ADLMIDI_getAdLibMode() }
}

pub fn adlmidi_set_adlib_mode(mode: c_int) {
    unsafe { Mix_ADLMIDI_setAdLibMode(mode) }
}

pub fn adlmidi_get_logarithmic_volumes() -> c_int {
    unsafe { Mix_ADLMIDI_getLogarithmicVolumes() }
}

pub fn adlmidi_set_logarithmic_volumes(lv: c_int) {
    unsafe { Mix_ADLMIDI_setLogarithmicVolumes(lv) }
}

pub fn adlmidi_get_volume_model() -> c_int {
    unsafe { Mix_ADLMIDI_getVolumeModel() }
}

pub fn adlmidi_set_volume_model(vm: c_int) {
    unsafe { Mix_ADLMIDI_setVolumeModel(vm) }
}

pub fn adlmidi_get_full_range_brightness() -> c_int {
    unsafe { Mix_ADLMIDI_getFullRangeBrightness() }
}

pub fn adlmidi_set_full_range_brightness(frb: c_int) {
    unsafe { Mix_ADLMIDI_setFullRangeBrightness(frb) }
}

pub fn adlmidi_get_auto_arpeggio() -> c_int {
    unsafe { Mix_ADLMIDI_getAutoArpeggio() }
}

pub fn adlmidi_set_auto_arpeggio(aa_en: c_int) {
    unsafe { Mix_ADLMIDI_setAutoArpeggio(aa_en) }
}

pub fn adlmidi_get_channel_alloc_mode() -> c_int {
    unsafe { Mix_ADLMIDI_getChannelAllocMode() }
}

pub fn adlmidi_set_channel_alloc_mode(ca: c_int) {
    unsafe { Mix_ADLMIDI_setChannelAllocMode(ca) }
}

pub fn adlmidi_get_full_pan_stereo() -> c_int {
    unsafe { Mix_ADLMIDI_getFullPanStereo() }
}

pub fn adlmidi_set_full_pan_stereo(fp: c_int) {
    unsafe { Mix_ADLMIDI_setFullPanStereo(fp) }
}

pub fn adlmidi_get_emulator() -> c_int {
    unsafe { Mix_ADLMIDI_getEmulator() }
}

pub fn adlmidi_set_emulator(emu: c_int) {
    unsafe { Mix_ADLMIDI_setEmulator(emu) }
}

pub fn adlmidi_get_chips_count() -> c_int {
    unsafe { Mix_ADLMIDI_getChipsCount() }
}

pub fn adlmidi_set_chips_count(chips: c_int) {
    unsafe { Mix_ADLMIDI_setChipsCount(chips) }
}

pub fn adlmidi_set_set_defaults() {
    unsafe { Mix_ADLMIDI_setSetDefaults() }
}

pub fn adlmidi_set_custom_bank_file(path: *const c_char) {
    unsafe { Mix_ADLMIDI_setCustomBankFile(path) }
}

// ── OPNMIDI 封装 ──

pub fn opnmidi_set_set_defaults() {
    unsafe { Mix_OPNMIDI_setSetDefaults() }
}

pub fn opnmidi_get_volume_model() -> c_int {
    unsafe { Mix_OPNMIDI_getVolumeModel() }
}

pub fn opnmidi_set_volume_model(vm: c_int) {
    unsafe { Mix_OPNMIDI_setVolumeModel(vm) }
}

pub fn opnmidi_get_full_range_brightness() -> c_int {
    unsafe { Mix_OPNMIDI_getFullRangeBrightness() }
}

pub fn opnmidi_set_full_range_brightness(frb: c_int) {
    unsafe { Mix_OPNMIDI_setFullRangeBrightness(frb) }
}

pub fn opnmidi_get_auto_arpeggio() -> c_int {
    unsafe { Mix_OPNMIDI_getAutoArpeggio() }
}

pub fn opnmidi_set_auto_arpeggio(aa_en: c_int) {
    unsafe { Mix_OPNMIDI_setAutoArpeggio(aa_en) }
}

pub fn opnmidi_get_channel_alloc_mode() -> c_int {
    unsafe { Mix_OPNMIDI_getChannelAllocMode() }
}

pub fn opnmidi_set_channel_alloc_mode(ca: c_int) {
    unsafe { Mix_OPNMIDI_setChannelAllocMode(ca) }
}

pub fn opnmidi_get_full_pan_stereo() -> c_int {
    unsafe { Mix_OPNMIDI_getFullPanStereo() }
}

pub fn opnmidi_set_full_pan_stereo(fp: c_int) {
    unsafe { Mix_OPNMIDI_setFullPanStereo(fp) }
}

pub fn opnmidi_get_emulator() -> c_int {
    unsafe { Mix_OPNMIDI_getEmulator() }
}

pub fn opnmidi_set_emulator(emu: c_int) {
    unsafe { Mix_OPNMIDI_setEmulator(emu) }
}

pub fn opnmidi_get_chips_count() -> c_int {
    unsafe { Mix_OPNMIDI_getChipsCount() }
}

pub fn opnmidi_set_chips_count(chips: c_int) {
    unsafe { Mix_OPNMIDI_setChipsCount(chips) }
}

pub fn opnmidi_set_custom_bank_file(path: *const c_char) {
    unsafe { Mix_OPNMIDI_setCustomBankFile(path) }
}

// ── GME 封装 ──

pub fn gme_set_spc_echo_disabled(music: *mut c_void, disabled: &mut c_int) {
    unsafe { Mix_GME_SetSpcEchoDisabled(music, disabled) }
}

pub fn gme_get_spc_echo_disabled(music: *mut c_void) -> c_int {
    unsafe { Mix_GME_GetSpcEchoDisabled(music) }
}

// ── MIDI 播放器封装 ──

pub fn get_midi_player() -> c_int {
    unsafe { Mix_GetMidiPlayer() }
}

pub fn get_next_midi_player(player: c_int) -> c_int {
    unsafe { Mix_GetNextMidiPlayer(player) }
}

pub fn set_midi_player(player: c_int) -> c_int {
    unsafe { Mix_SetMidiPlayer(player) }
}

pub fn set_lock_midi_args(lock: c_int) {
    unsafe { Mix_SetLockMIDIArgs(lock) }
}

pub fn get_midi_device() -> *const c_char {
    unsafe { Mix_GetMidiDevice() }
}

pub fn get_next_midi_device(device: *const c_char) -> *const c_char {
    unsafe { Mix_GetNextMidiDevice(device) }
}

pub fn set_midi_device(device: *const c_char) -> c_int {
    unsafe { Mix_SetMidiDevice(device) }
}