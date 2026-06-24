// PvZ Portable Rust 翻译 — OpenGL ES 2.0 FFI 绑定
//
// 使用 extern "C" 声明所需的 GLES2 API。
// 这些函数在运行时通过 glad（或 SDL_GL_GetProcAddress）加载。

#![allow(non_camel_case_types, dead_code)]

use std::os::raw::{c_char, c_int, c_uint, c_void, c_float};

// ============================================================
// 类型定义
// ============================================================

pub type GLenum = c_uint;
pub type GLboolean = u8;
pub type GLbitfield = c_uint;
pub type GLbyte = i8;
pub type GLshort = i16;
pub type GLint = c_int;
pub type GLsizei = c_int;
pub type GLubyte = u8;
pub type GLushort = u16;
pub type GLuint = c_uint;
pub type GLfloat = c_float;
pub type GLclampf = c_float;
pub type GLchar = c_char;
pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLuint64 = u64;
pub type GLint64 = i64;

// ============================================================
// GLES2 常量
// ============================================================

pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE: GLboolean = 1;

// 清除缓冲位
pub const GL_DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
pub const GL_STENCIL_BUFFER_BIT: GLbitfield = 0x00000400;
pub const GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000;

// 基本图元
pub const GL_POINTS: GLenum = 0x0000;
pub const GL_LINES: GLenum = 0x0001;
pub const GL_LINE_LOOP: GLenum = 0x0002;
pub const GL_LINE_STRIP: GLenum = 0x0003;
pub const GL_TRIANGLES: GLenum = 0x0004;
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
pub const GL_TRIANGLE_FAN: GLenum = 0x0006;

// 混合
pub const GL_ZERO: GLenum = 0;
pub const GL_ONE: GLenum = 1;
pub const GL_SRC_COLOR: GLenum = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const GL_SRC_ALPHA: GLenum = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const GL_DST_ALPHA: GLenum = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const GL_DST_COLOR: GLenum = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const GL_SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub const GL_FUNC_ADD: GLenum = 0x8006;
pub const GL_BLEND_EQUATION: GLenum = 0x8009;
pub const GL_BLEND_EQUATION_RGB: GLenum = 0x8009;
pub const GL_BLEND_EQUATION_ALPHA: GLenum = 0x883D;
pub const GL_FUNC_SUBTRACT: GLenum = 0x800A;
pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub const GL_BLEND_DST_RGB: GLenum = 0x80C8;
pub const GL_BLEND_SRC_RGB: GLenum = 0x80C9;
pub const GL_BLEND_DST_ALPHA: GLenum = 0x80CA;
pub const GL_BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub const GL_CONSTANT_COLOR: GLenum = 0x8001;
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const GL_CONSTANT_ALPHA: GLenum = 0x8003;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub const GL_BLEND_COLOR: GLenum = 0x8005;

// 纹理
pub const GL_TEXTURE_2D: GLenum = 0x0DE1;
pub const GL_TEXTURE0: GLenum = 0x84C0;
pub const GL_TEXTURE1: GLenum = 0x84C1;
pub const GL_TEXTURE_BINDING_2D: GLenum = 0x8069;
pub const GL_ACTIVE_TEXTURE: GLenum = 0x84E0;
pub const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
pub const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
pub const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const GL_NEAREST: GLenum = 0x2600;
pub const GL_LINEAR: GLenum = 0x2601;
pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub const GL_CLAMP_TO_EDGE: GLenum = 0x812F;
pub const GL_REPEAT: GLenum = 0x2901;
pub const GL_MIRRORED_REPEAT: GLenum = 0x8370;
pub const GL_RGB: GLenum = 0x1907;
pub const GL_RGBA: GLenum = 0x1908;
pub const GL_ALPHA: GLenum = 0x1906;
pub const GL_LUMINANCE: GLenum = 0x1909;
pub const GL_LUMINANCE_ALPHA: GLenum = 0x190A;
pub const GL_RGB565: GLenum = 0x8D62;
pub const GL_RGBA4: GLenum = 0x8056;
pub const GL_RGB5_A1: GLenum = 0x8057;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
pub const GL_UNSIGNED_INT: GLenum = 0x1405;
pub const GL_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FE;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FF;

// 着色器
pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
pub const GL_LINK_STATUS: GLenum = 0x8B82;
pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const GL_DELETE_STATUS: GLenum = 0x8B80;
pub const GL_SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
pub const GL_SHADER_TYPE: GLenum = 0x8B4F;
pub const GL_CURRENT_PROGRAM: GLenum = 0x8B8D;
pub const GL_ATTACHED_SHADERS: GLenum = 0x8B85;
pub const GL_ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub const GL_ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub const GL_FLOAT: GLenum = 0x1406;
pub const GL_FLOAT_VEC2: GLenum = 0x8B50;
pub const GL_FLOAT_VEC3: GLenum = 0x8B51;
pub const GL_FLOAT_VEC4: GLenum = 0x8B52;
pub const GL_FLOAT_MAT4: GLenum = 0x8B5C;
pub const GL_INT: GLenum = 0x1404;
pub const GL_INT_VEC2: GLenum = 0x8B53;
pub const GL_INT_VEC3: GLenum = 0x8B54;
pub const GL_INT_VEC4: GLenum = 0x8B55;
pub const GL_BOOL: GLenum = 0x8B56;
pub const GL_BOOL_VEC2: GLenum = 0x8B57;
pub const GL_BOOL_VEC3: GLenum = 0x8B58;
pub const GL_BOOL_VEC4: GLenum = 0x8B59;
pub const GL_SAMPLER_2D: GLenum = 0x8B5E;
pub const GL_SAMPLER_CUBE: GLenum = 0x8B60;

// 缓冲对象
pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const GL_ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub const GL_STREAM_DRAW: GLenum = 0x88E0;
pub const GL_STATIC_DRAW: GLenum = 0x88E4;
pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
pub const GL_BUFFER_SIZE: GLenum = 0x8764;
pub const GL_BUFFER_USAGE: GLenum = 0x8765;

// 顶点属性
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub const GL_MAX_VERTEX_ATTRIBS: GLenum = 0x8869;

// 启用/禁用
pub const GL_BLEND: GLenum = 0x0BE2;
pub const GL_DEPTH_TEST: GLenum = 0x0B71;
pub const GL_CULL_FACE: GLenum = 0x0B44;
pub const GL_SCISSOR_TEST: GLenum = 0x0C11;
pub const GL_DITHER: GLenum = 0x0BD0;
pub const GL_STENCIL_TEST: GLenum = 0x0B90;
pub const GL_POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const GL_SAMPLE_COVERAGE: GLenum = 0x80A0;

// 帧缓冲
pub const GL_FRAMEBUFFER: GLenum = 0x8D40;
pub const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const GL_RENDERBUFFER: GLenum = 0x8D41;
pub const GL_RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub const GL_COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const GL_DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const GL_STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub const GL_FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 0x8CD9;
pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub const GL_STENCIL_INDEX8: GLenum = 0x8D48;
pub const GL_DEPTH_COMPONENT16: GLenum = 0x81A5;

// 像素存储
pub const GL_UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const GL_PACK_ALIGNMENT: GLenum = 0x0D05;

// 视图
pub const GL_VIEWPORT: GLenum = 0x0BA2;
pub const GL_SCISSOR_BOX: GLenum = 0x0C10;
pub const GL_MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub const GL_SUBPIXEL_BITS: GLenum = 0x0D50;

// 查询
pub const GL_MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub const GL_ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub const GL_MAX_VARYING_VECTORS: GLenum = 0x8DFC;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
pub const GL_RED_BITS: GLenum = 0x0D52;
pub const GL_GREEN_BITS: GLenum = 0x0D53;
pub const GL_BLUE_BITS: GLenum = 0x0D54;
pub const GL_ALPHA_BITS: GLenum = 0x0D55;
pub const GL_DEPTH_BITS: GLenum = 0x0D56;
pub const GL_STENCIL_BITS: GLenum = 0x0D57;
pub const GL_SAMPLES: GLenum = 0x80A9;
pub const GL_SAMPLE_BUFFERS: GLenum = 0x80A8;
pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub const GL_NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
pub const GL_SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
pub const GL_SHADER_COMPILER: GLenum = 0x8DFA;

// 状态
pub const GL_VENDOR: GLenum = 0x1F00;
pub const GL_RENDERER: GLenum = 0x1F01;
pub const GL_VERSION: GLenum = 0x1F02;
pub const GL_EXTENSIONS: GLenum = 0x1F03;
pub const GL_NUM_EXTENSIONS: GLenum = 0x821D;
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;

// 面
pub const GL_FRONT: GLenum = 0x0404;
pub const GL_BACK: GLenum = 0x0405;
pub const GL_FRONT_AND_BACK: GLenum = 0x0408;
pub const GL_CULL_FACE_MODE: GLenum = 0x0B45;
pub const GL_FRONT_FACE: GLenum = 0x0B46;
pub const GL_CW: GLenum = 0x0900;
pub const GL_CCW: GLenum = 0x0901;

// 深度/模板
pub const GL_NEVER: GLenum = 0x0200;
pub const GL_LESS: GLenum = 0x0201;
pub const GL_EQUAL: GLenum = 0x0202;
pub const GL_LEQUAL: GLenum = 0x0203;
pub const GL_GREATER: GLenum = 0x0204;
pub const GL_NOTEQUAL: GLenum = 0x0205;
pub const GL_GEQUAL: GLenum = 0x0206;
pub const GL_ALWAYS: GLenum = 0x0207;
pub const GL_DEPTH_FUNC: GLenum = 0x0B74;
pub const GL_DEPTH_WRITEMASK: GLenum = 0x0B72;
pub const GL_DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub const GL_DEPTH_RANGE: GLenum = 0x0B70;
pub const GL_STENCIL_FUNC: GLenum = 0x0B92;
pub const GL_STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub const GL_STENCIL_REF: GLenum = 0x0B97;
pub const GL_STENCIL_FAIL: GLenum = 0x0B94;
pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub const GL_STENCIL_WRITEMASK: GLenum = 0x0B98;
pub const GL_STENCIL_BACK_FUNC: GLenum = 0x8800;
pub const GL_STENCIL_BACK_FAIL: GLenum = 0x8801;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub const GL_STENCIL_BACK_REF: GLenum = 0x8CA3;
pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub const GL_STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
pub const GL_COLOR_WRITEMASK: GLenum = 0x0C23;
pub const GL_COLOR_CLEAR_VALUE: GLenum = 0x0C22;

// 擦除
pub const GL_KEEP: GLenum = 0x1E00;
pub const GL_REPLACE: GLenum = 0x1E01;
pub const GL_INCR: GLenum = 0x1E02;
pub const GL_DECR: GLenum = 0x1E03;
pub const GL_INVERT: GLenum = 0x150A;
pub const GL_INCR_WRAP: GLenum = 0x8507;
pub const GL_DECR_WRAP: GLenum = 0x8508;

// 错误码
pub const GL_NO_ERROR: GLenum = 0;
pub const GL_INVALID_ENUM: GLenum = 0x0500;
pub const GL_INVALID_VALUE: GLenum = 0x0501;
pub const GL_INVALID_OPERATION: GLenum = 0x0502;
pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;

// 提示
pub const GL_DONT_CARE: GLenum = 0x1100;
pub const GL_FASTEST: GLenum = 0x1101;
pub const GL_NICEST: GLenum = 0x1102;
pub const GL_GENERATE_MIPMAP_HINT: GLenum = 0x8192;

// 多边形偏移
pub const GL_POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub const GL_POLYGON_OFFSET_UNITS: GLenum = 0x2A00;

// 立方体贴图
pub const GL_TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;

// 其他
pub const GL_FIXED: GLenum = 0x140C;
pub const GL_BYTE: GLenum = 0x1400;
pub const GL_SHORT: GLenum = 0x1402;

// ============================================================
// GLES2 函数声明
// glad 和 OpenGL 函数声明
// opengl32.dll 是 Windows 系统 DLL，用 raw-dylib 链接
#[link(name = "opengl32", kind = "raw-dylib")]
unsafe extern "C" {
    // 顶点缓冲
    pub fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);
    pub fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint);
    pub fn glBindBuffer(target: GLenum, buffer: GLuint);
    pub fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
    pub fn glBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void);

    // 纹理
    pub fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    pub fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
    pub fn glBindTexture(target: GLenum, texture: GLuint);
    pub fn glTexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
    pub fn glTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
    pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    pub fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
    pub fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint);
    pub fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat);
    pub fn glGenerateMipmap(target: GLenum);
    pub fn glActiveTexture(texture: GLenum);

    // 着色器
    pub fn glCreateShader(type_: GLenum) -> GLuint;
    pub fn glDeleteShader(shader: GLuint);
    pub fn glShaderSource(shader: GLuint, count: GLsizei, string_: *const *const GLchar, length: *const GLint);
    pub fn glCompileShader(shader: GLuint);
    pub fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
    pub fn glGetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
    pub fn glIsShader(shader: GLuint) -> GLboolean;

    // 程序
    pub fn glCreateProgram() -> GLuint;
    pub fn glDeleteProgram(program: GLuint);
    pub fn glAttachShader(program: GLuint, shader: GLuint);
    pub fn glDetachShader(program: GLuint, shader: GLuint);
    pub fn glLinkProgram(program: GLuint);
    pub fn glUseProgram(program: GLuint);
    pub fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
    pub fn glGetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
    pub fn glValidateProgram(program: GLuint);
    pub fn glIsProgram(program: GLuint) -> GLboolean;
    pub fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar);

    // 属性和 uniform
    pub fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;
    pub fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
    pub fn glUniform1i(location: GLint, v0: GLint);
    pub fn glUniform1f(location: GLint, v0: GLfloat);
    pub fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat);
    pub fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
    pub fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
    pub fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint);
    pub fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat);
    pub fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat);
    pub fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat);
    pub fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat);
    pub fn glUniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
    pub fn glUniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

    // 顶点属性
    pub fn glVertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
    pub fn glEnableVertexAttribArray(index: GLuint);
    pub fn glDisableVertexAttribArray(index: GLuint);
    pub fn glVertexAttrib1f(index: GLuint, x: GLfloat);
    pub fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat);
    pub fn glVertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glVertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

    // 绘制
    pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
    pub fn glDrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void);

    // 清除
    pub fn glClear(mask: GLbitfield);
    pub fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    pub fn glClearDepthf(d: GLclampf);
    pub fn glClearStencil(s: GLint);

    // 状态
    pub fn glEnable(cap: GLenum);
    pub fn glDisable(cap: GLenum);
    pub fn glIsEnabled(cap: GLenum) -> GLboolean;
    pub fn glGetBooleanv(pname: GLenum, data: *mut GLboolean);
    pub fn glGetIntegerv(pname: GLenum, data: *mut GLint);
    pub fn glGetFloatv(pname: GLenum, data: *mut GLfloat);
    pub fn glGetString(name: GLenum) -> *const GLubyte;
    pub fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte;
    pub fn glGetError() -> GLenum;

    // 视图
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glDepthRangef(n: GLclampf, f: GLclampf);
    pub fn glLineWidth(width: GLfloat);

    // 混合
    pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    pub fn glBlendFuncSeparate(srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
    pub fn glBlendEquation(mode: GLenum);
    pub fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);
    pub fn glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);

    // 颜色
    pub fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

    // 深度/模板
    pub fn glDepthFunc(func: GLenum);
    pub fn glDepthMask(flag: GLboolean);
    pub fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint);
    pub fn glStencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
    pub fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);
    pub fn glStencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
    pub fn glStencilMask(mask: GLuint);
    pub fn glStencilMaskSeparate(face: GLenum, mask: GLuint);

    // 光栅化
    pub fn glFrontFace(mode: GLenum);
    pub fn glCullFace(mode: GLenum);
    pub fn glPolygonOffset(factor: GLfloat, units: GLfloat);
    pub fn glSampleCoverage(value: GLclampf, invert: GLboolean);

    // 像素存储
    pub fn glPixelStorei(pname: GLenum, param: GLint);

    // 帧缓冲
    pub fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
    pub fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
    pub fn glBindFramebuffer(target: GLenum, framebuffer: GLuint);
    pub fn glFramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
    pub fn glFramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
    pub fn glCheckFramebufferStatus(target: GLenum) -> GLenum;
    pub fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);
    pub fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint);
    pub fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint);
    pub fn glRenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);
    pub fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void);

    // 完成
    pub fn glFinish();
    pub fn glFlush();

    // 缓冲子数据
    pub fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
}

// glad 加载函数
pub type GLADloadfunc = Option<unsafe extern "C" fn(name: *const GLchar) -> *mut c_void>;

unsafe extern "C" {
    pub fn gladLoadGLES2(loadfunc: GLADloadfunc) -> c_int;
}
