// PvZ Portable Rust 翻译 — OpenGL ES 2.0 FFI 绑定（运行时加载）
//
// 所有 GL 函数通过 SDL_GL_GetProcAddress 运行时加载，
// 不使用 raw-dylib 静态链接 opengl32.dll（后者只导出 OpenGL 1.1）。
//
// 用法：在创建 OpenGL 上下文后调用 init_gl() 初始化函数表，
// 然后即可直接调用 glXxx(...) 函数。

#![allow(non_camel_case_types, dead_code)]

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_void, c_float};
use std::sync::OnceLock;
use sdl2::sys as sdl_sys;

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
// GLES2 常量（与原始文件一致）
// ============================================================

pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE: GLboolean = 1;
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

// 像素存储、视图、查询等（保留全部常量）
pub const GL_UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const GL_PACK_ALIGNMENT: GLenum = 0x0D05;
pub const GL_glViewport: GLenum = 0x0BA2;
pub const GL_SCISSOR_BOX: GLenum = 0x0C10;
pub const GL_MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub const GL_SUBPIXEL_BITS: GLenum = 0x0D50;
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
// 运行时加载的 GL 函数表
// ============================================================

/// 所有 OpenGL 函数的函数指针表。
/// 在创建 OpenGL 上下文后通过 init_gl() 填充。
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GlProcs {
    pub glActiveTexture: unsafe extern "C" fn(texture: GLenum),
    pub glAttachShader: unsafe extern "C" fn(program: GLuint, shader: GLuint),
    pub glBindAttribLocation: unsafe extern "C" fn(program: GLuint, index: GLuint, name: *const GLchar),
    pub glBindBuffer: unsafe extern "C" fn(target: GLenum, buffer: GLuint),
    pub glBindFramebuffer: unsafe extern "C" fn(target: GLenum, framebuffer: GLuint),
    pub glBindTexture: unsafe extern "C" fn(target: GLenum, texture: GLuint),
    pub glBlendFunc: unsafe extern "C" fn(sfactor: GLenum, dfactor: GLenum),
    pub glBlendFuncSeparate: unsafe extern "C" fn(srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum),
    pub glBlendEquation: unsafe extern "C" fn(mode: GLenum),
    pub glBlendEquationSeparate: unsafe extern "C" fn(modeRGB: GLenum, modeAlpha: GLenum),
    pub glBlendColor: unsafe extern "C" fn(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf),
    pub glBufferData: unsafe extern "C" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum),
    pub glBufferSubData: unsafe extern "C" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void),
    pub glCheckFramebufferStatus: unsafe extern "C" fn(target: GLenum) -> GLenum,
    pub glClear: unsafe extern "C" fn(mask: GLbitfield),
    pub glClearColor: unsafe extern "C" fn(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf),
    pub glClearDepthf: unsafe extern "C" fn(d: GLclampf),
    pub glClearStencil: unsafe extern "C" fn(s: GLint),
    pub glColorMask: unsafe extern "C" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean),
    pub glCompileShader: unsafe extern "C" fn(shader: GLuint),
    pub glCreateProgram: unsafe extern "C" fn() -> GLuint,
    pub glCreateShader: unsafe extern "C" fn(type_: GLenum) -> GLuint,
    pub glCullFace: unsafe extern "C" fn(mode: GLenum),
    pub glDeleteBuffers: unsafe extern "C" fn(n: GLsizei, buffers: *const GLuint),
    pub glDeleteFramebuffers: unsafe extern "C" fn(n: GLsizei, framebuffers: *const GLuint),
    pub glDeleteProgram: unsafe extern "C" fn(program: GLuint),
    pub glDeleteShader: unsafe extern "C" fn(shader: GLuint),
    pub glDeleteTextures: unsafe extern "C" fn(n: GLsizei, textures: *const GLuint),
    pub glDepthFunc: unsafe extern "C" fn(func: GLenum),
    pub glDepthMask: unsafe extern "C" fn(flag: GLboolean),
    pub glDepthRangef: unsafe extern "C" fn(n: GLclampf, f: GLclampf),
    pub glDetachShader: unsafe extern "C" fn(program: GLuint, shader: GLuint),
    pub glDisable: unsafe extern "C" fn(cap: GLenum),
    pub glDisableVertexAttribArray: unsafe extern "C" fn(index: GLuint),
    pub glDrawArrays: unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei),
    pub glDrawElements: unsafe extern "C" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void),
    pub glEnable: unsafe extern "C" fn(cap: GLenum),
    pub glEnableVertexAttribArray: unsafe extern "C" fn(index: GLuint),
    pub glFinish: unsafe extern "C" fn(),
    pub glFlush: unsafe extern "C" fn(),
    pub glFramebufferRenderbuffer: unsafe extern "C" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint),
    pub glFramebufferTexture2D: unsafe extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint),
    pub glFrontFace: unsafe extern "C" fn(mode: GLenum),
    pub glGenBuffers: unsafe extern "C" fn(n: GLsizei, buffers: *mut GLuint),
    pub glGenFramebuffers: unsafe extern "C" fn(n: GLsizei, framebuffers: *mut GLuint),
    pub glGenRenderbuffers: unsafe extern "C" fn(n: GLsizei, renderbuffers: *mut GLuint),
    pub glGenTextures: unsafe extern "C" fn(n: GLsizei, textures: *mut GLuint),
    pub glGenerateMipmap: unsafe extern "C" fn(target: GLenum),
    pub glGetAttribLocation: unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint,
    pub glGetBooleanv: unsafe extern "C" fn(pname: GLenum, data: *mut GLboolean),
    pub glGetBufferParameteriv: unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint),
    pub glGetError: unsafe extern "C" fn() -> GLenum,
    pub glGetFloatv: unsafe extern "C" fn(pname: GLenum, data: *mut GLfloat),
    pub glGetIntegerv: unsafe extern "C" fn(pname: GLenum, data: *mut GLint),
    pub glGetProgramInfoLog: unsafe extern "C" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar),
    pub glGetProgramiv: unsafe extern "C" fn(program: GLuint, pname: GLenum, params: *mut GLint),
    pub glGetShaderInfoLog: unsafe extern "C" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar),
    pub glGetShaderiv: unsafe extern "C" fn(shader: GLuint, pname: GLenum, params: *mut GLint),
    pub glGetString: unsafe extern "C" fn(name: GLenum) -> *const GLubyte,
    pub glGetStringi: unsafe extern "C" fn(name: GLenum, index: GLuint) -> *const GLubyte,
    pub glGetUniformLocation: unsafe extern "C" fn(program: GLuint, name: *const GLchar) -> GLint,
    pub glIsBuffer: unsafe extern "C" fn(buffer: GLuint) -> GLboolean,
    pub glIsEnabled: unsafe extern "C" fn(cap: GLenum) -> GLboolean,
    pub glIsProgram: unsafe extern "C" fn(program: GLuint) -> GLboolean,
    pub glIsShader: unsafe extern "C" fn(shader: GLuint) -> GLboolean,
    pub glIsTexture: unsafe extern "C" fn(texture: GLuint) -> GLboolean,
    pub glLineWidth: unsafe extern "C" fn(width: GLfloat),
    pub glLinkProgram: unsafe extern "C" fn(program: GLuint),
    pub glPixelStorei: unsafe extern "C" fn(pname: GLenum, param: GLint),
    pub glPolygonOffset: unsafe extern "C" fn(factor: GLfloat, units: GLfloat),
    pub glReadPixels: unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void),
    pub glRenderbufferStorage: unsafe extern "C" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei),
    pub glSampleCoverage: unsafe extern "C" fn(value: GLclampf, invert: GLboolean),
    pub glScissor: unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei),
    pub glShaderSource: unsafe extern "C" fn(shader: GLuint, count: GLsizei, string_: *const *const GLchar, length: *const GLint),
    pub glStencilFunc: unsafe extern "C" fn(func: GLenum, ref_: GLint, mask: GLuint),
    pub glStencilFuncSeparate: unsafe extern "C" fn(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint),
    pub glStencilMask: unsafe extern "C" fn(mask: GLuint),
    pub glStencilMaskSeparate: unsafe extern "C" fn(face: GLenum, mask: GLuint),
    pub glStencilOp: unsafe extern "C" fn(fail: GLenum, zfail: GLenum, zpass: GLenum),
    pub glStencilOpSeparate: unsafe extern "C" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum),
    pub glTexImage2D: unsafe extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void),
    pub glTexParameterf: unsafe extern "C" fn(target: GLenum, pname: GLenum, param: GLfloat),
    pub glTexParameterfv: unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLfloat),
    pub glTexParameteri: unsafe extern "C" fn(target: GLenum, pname: GLenum, param: GLint),
    pub glTexParameteriv: unsafe extern "C" fn(target: GLenum, pname: GLenum, params: *const GLint),
    pub glTexSubImage2D: unsafe extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void),
    pub glUniform1f: unsafe extern "C" fn(location: GLint, v0: GLfloat),
    pub glUniform1fv: unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat),
    pub glUniform1i: unsafe extern "C" fn(location: GLint, v0: GLint),
    pub glUniform1iv: unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLint),
    pub glUniform2f: unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat),
    pub glUniform2fv: unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat),
    pub glUniform3f: unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat),
    pub glUniform3fv: unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat),
    pub glUniform4f: unsafe extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat),
    pub glUniform4fv: unsafe extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat),
    pub glUniformMatrix3fv: unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat),
    pub glUniformMatrix4fv: unsafe extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat),
    pub glUseProgram: unsafe extern "C" fn(program: GLuint),
    pub glValidateProgram: unsafe extern "C" fn(program: GLuint),
    pub glVertexAttrib1f: unsafe extern "C" fn(index: GLuint, x: GLfloat),
    pub glVertexAttrib2f: unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat),
    pub glVertexAttrib3f: unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat),
    pub glVertexAttrib4f: unsafe extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat),
    pub glVertexAttribPointer: unsafe extern "C" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void),
    pub glViewport: unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei),
    pub glDeleteRenderbuffers: unsafe extern "C" fn(n: GLsizei, renderbuffers: *const GLuint),
    pub glBindRenderbuffer: unsafe extern "C" fn(target: GLenum, renderbuffer: GLuint),
}

// ============================================================
// 全局函数表实例（OnceLock，初始化一次）
// ============================================================

pub static GL: OnceLock<GlProcs> = OnceLock::new();

/// 加载单个 GL 函数指针。
/// 内部使用 SDL_GL_GetProcAddress（通过 sdl2 crate 获取）。
unsafe fn load_gl_func(name: &str) -> *mut c_void {
    let cname = CString::new(name).unwrap();
    sdl_sys::SDL_GL_GetProcAddress(cname.as_ptr())
}

/// 初始化 OpenGL 函数表。
/// 必须在创建并激活 OpenGL 上下文后调用，且只调用一次。
pub fn init_gl() -> bool {
    if GL.get().is_some() {
        return true; // 已经初始化
    }

    // 辅助宏：加载函数并转成函数指针
    macro_rules! load {
        ($name:literal) => {{
            let ptr = unsafe { load_gl_func($name) };
            if ptr.is_null() {
                eprintln!("OpenGL 函数加载失败: {}", $name);
                return false;
            }
            unsafe { std::mem::transmute::<*mut c_void, _>(ptr) }
        }};
    }

    let procs = GlProcs {
        glActiveTexture: load!("glActiveTexture"),
        glAttachShader: load!("glAttachShader"),
        glBindAttribLocation: load!("glBindAttribLocation"),
        glBindBuffer: load!("glBindBuffer"),
        glBindFramebuffer: load!("glBindFramebuffer"),
        glBindTexture: load!("glBindTexture"),
        glBlendFunc: load!("glBlendFunc"),
        glBlendFuncSeparate: load!("glBlendFuncSeparate"),
        glBlendEquation: load!("glBlendEquation"),
        glBlendEquationSeparate: load!("glBlendEquationSeparate"),
        glBlendColor: load!("glBlendColor"),
        glBufferData: load!("glBufferData"),
        glBufferSubData: load!("glBufferSubData"),
        glCheckFramebufferStatus: load!("glCheckFramebufferStatus"),
        glClear: load!("glClear"),
        glClearColor: load!("glClearColor"),
        glClearDepthf: load!("glClearDepthf"),
        glClearStencil: load!("glClearStencil"),
        glColorMask: load!("glColorMask"),
        glCompileShader: load!("glCompileShader"),
        glCreateProgram: load!("glCreateProgram"),
        glCreateShader: load!("glCreateShader"),
        glCullFace: load!("glCullFace"),
        glDeleteBuffers: load!("glDeleteBuffers"),
        glDeleteFramebuffers: load!("glDeleteFramebuffers"),
        glDeleteProgram: load!("glDeleteProgram"),
        glDeleteShader: load!("glDeleteShader"),
        glDeleteTextures: load!("glDeleteTextures"),
        glDepthFunc: load!("glDepthFunc"),
        glDepthMask: load!("glDepthMask"),
        glDepthRangef: load!("glDepthRangef"),
        glDetachShader: load!("glDetachShader"),
        glDisable: load!("glDisable"),
        glDisableVertexAttribArray: load!("glDisableVertexAttribArray"),
        glDrawArrays: load!("glDrawArrays"),
        glDrawElements: load!("glDrawElements"),
        glEnable: load!("glEnable"),
        glEnableVertexAttribArray: load!("glEnableVertexAttribArray"),
        glFinish: load!("glFinish"),
        glFlush: load!("glFlush"),
        glFramebufferRenderbuffer: load!("glFramebufferRenderbuffer"),
        glFramebufferTexture2D: load!("glFramebufferTexture2D"),
        glFrontFace: load!("glFrontFace"),
        glGenBuffers: load!("glGenBuffers"),
        glGenFramebuffers: load!("glGenFramebuffers"),
        glGenRenderbuffers: load!("glGenRenderbuffers"),
        glGenTextures: load!("glGenTextures"),
        glGenerateMipmap: load!("glGenerateMipmap"),
        glGetAttribLocation: load!("glGetAttribLocation"),
        glGetBooleanv: load!("glGetBooleanv"),
        glGetBufferParameteriv: load!("glGetBufferParameteriv"),
        glGetError: load!("glGetError"),
        glGetFloatv: load!("glGetFloatv"),
        glGetIntegerv: load!("glGetIntegerv"),
        glGetProgramInfoLog: load!("glGetProgramInfoLog"),
        glGetProgramiv: load!("glGetProgramiv"),
        glGetShaderInfoLog: load!("glGetShaderInfoLog"),
        glGetShaderiv: load!("glGetShaderiv"),
        glGetString: load!("glGetString"),
        glGetStringi: load!("glGetStringi"),
        glGetUniformLocation: load!("glGetUniformLocation"),
        glIsBuffer: load!("glIsBuffer"),
        glIsEnabled: load!("glIsEnabled"),
        glIsProgram: load!("glIsProgram"),
        glIsShader: load!("glIsShader"),
        glIsTexture: load!("glIsTexture"),
        glLineWidth: load!("glLineWidth"),
        glLinkProgram: load!("glLinkProgram"),
        glPixelStorei: load!("glPixelStorei"),
        glPolygonOffset: load!("glPolygonOffset"),
        glReadPixels: load!("glReadPixels"),
        glRenderbufferStorage: load!("glRenderbufferStorage"),
        glSampleCoverage: load!("glSampleCoverage"),
        glScissor: load!("glScissor"),
        glShaderSource: load!("glShaderSource"),
        glStencilFunc: load!("glStencilFunc"),
        glStencilFuncSeparate: load!("glStencilFuncSeparate"),
        glStencilMask: load!("glStencilMask"),
        glStencilMaskSeparate: load!("glStencilMaskSeparate"),
        glStencilOp: load!("glStencilOp"),
        glStencilOpSeparate: load!("glStencilOpSeparate"),
        glTexImage2D: load!("glTexImage2D"),
        glTexParameterf: load!("glTexParameterf"),
        glTexParameterfv: load!("glTexParameterfv"),
        glTexParameteri: load!("glTexParameteri"),
        glTexParameteriv: load!("glTexParameteriv"),
        glTexSubImage2D: load!("glTexSubImage2D"),
        glUniform1f: load!("glUniform1f"),
        glUniform1fv: load!("glUniform1fv"),
        glUniform1i: load!("glUniform1i"),
        glUniform1iv: load!("glUniform1iv"),
        glUniform2f: load!("glUniform2f"),
        glUniform2fv: load!("glUniform2fv"),
        glUniform3f: load!("glUniform3f"),
        glUniform3fv: load!("glUniform3fv"),
        glUniform4f: load!("glUniform4f"),
        glUniform4fv: load!("glUniform4fv"),
        glUniformMatrix3fv: load!("glUniformMatrix3fv"),
        glUniformMatrix4fv: load!("glUniformMatrix4fv"),
        glUseProgram: load!("glUseProgram"),
        glValidateProgram: load!("glValidateProgram"),
        glVertexAttrib1f: load!("glVertexAttrib1f"),
        glVertexAttrib2f: load!("glVertexAttrib2f"),
        glVertexAttrib3f: load!("glVertexAttrib3f"),
        glVertexAttrib4f: load!("glVertexAttrib4f"),
        glVertexAttribPointer: load!("glVertexAttribPointer"),
        glViewport: load!("glViewport"),
        glDeleteRenderbuffers: load!("glDeleteRenderbuffers"),
        glBindRenderbuffer: load!("glBindRenderbuffer"),
    };

    GL.set(procs).map_err(|_| ()).is_ok()
}

// ============================================================
// 方便调用的函数包装（保持与原始 opengl.rs 相同的函数签名）
// 这些函数通过全局 GL 函数表间接调用
// ============================================================

macro_rules! gl_wrapper {
    ($name:ident, ($($arg:ident: $t:ty),*) -> $ret:ty) => {
        #[inline]
        pub unsafe fn $name($($arg: $t),*) -> $ret {
            (GL.get().unwrap().$name)($($arg),*)
        }
    };
    ($name:ident, ($($arg:ident: $t:ty),*)) => {
        #[inline]
        pub unsafe fn $name($($arg: $t),*) {
            (GL.get().unwrap().$name)($($arg),*)
        }
    };
    ($name:ident) => {
        #[inline]
        pub unsafe fn $name() {
            (GL.get().unwrap().$name)()
        }
    };
}

gl_wrapper!(glActiveTexture, (texture: GLenum));
gl_wrapper!(glAttachShader, (program: GLuint, shader: GLuint));
gl_wrapper!(glBindAttribLocation, (program: GLuint, index: GLuint, name: *const GLchar));
gl_wrapper!(glBindBuffer, (target: GLenum, buffer: GLuint));
gl_wrapper!(glBindFramebuffer, (target: GLenum, framebuffer: GLuint));
gl_wrapper!(glBindTexture, (target: GLenum, texture: GLuint));
gl_wrapper!(glBlendFunc, (sfactor: GLenum, dfactor: GLenum));
gl_wrapper!(glBlendFuncSeparate, (srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum));
gl_wrapper!(glBlendEquation, (mode: GLenum));
gl_wrapper!(glBlendEquationSeparate, (modeRGB: GLenum, modeAlpha: GLenum));
gl_wrapper!(glBlendColor, (red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf));
gl_wrapper!(glBufferData, (target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum));
gl_wrapper!(glBufferSubData, (target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void));
gl_wrapper!(glCheckFramebufferStatus, (target: GLenum) -> GLenum);
gl_wrapper!(glClear, (mask: GLbitfield));
gl_wrapper!(glClearColor, (red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf));
gl_wrapper!(glClearDepthf, (d: GLclampf));
gl_wrapper!(glClearStencil, (s: GLint));
gl_wrapper!(glColorMask, (red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean));
gl_wrapper!(glCompileShader, (shader: GLuint));
gl_wrapper!(glCreateProgram, () -> GLuint);
gl_wrapper!(glCreateShader, (type_: GLenum) -> GLuint);
gl_wrapper!(glCullFace, (mode: GLenum));
gl_wrapper!(glDeleteBuffers, (n: GLsizei, buffers: *const GLuint));
gl_wrapper!(glDeleteFramebuffers, (n: GLsizei, framebuffers: *const GLuint));
gl_wrapper!(glDeleteProgram, (program: GLuint));
gl_wrapper!(glDeleteShader, (shader: GLuint));
gl_wrapper!(glDeleteTextures, (n: GLsizei, textures: *const GLuint));
gl_wrapper!(glDepthFunc, (func: GLenum));
gl_wrapper!(glDepthMask, (flag: GLboolean));
gl_wrapper!(glDepthRangef, (n: GLclampf, f: GLclampf));
gl_wrapper!(glDetachShader, (program: GLuint, shader: GLuint));
gl_wrapper!(glDisable, (cap: GLenum));
gl_wrapper!(glDisableVertexAttribArray, (index: GLuint));
gl_wrapper!(glDrawArrays, (mode: GLenum, first: GLint, count: GLsizei));
gl_wrapper!(glDrawElements, (mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void));
gl_wrapper!(glEnable, (cap: GLenum));
gl_wrapper!(glEnableVertexAttribArray, (index: GLuint));
gl_wrapper!(glFinish, ());
gl_wrapper!(glFlush, ());
gl_wrapper!(glFramebufferRenderbuffer, (target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint));
gl_wrapper!(glFramebufferTexture2D, (target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint));
gl_wrapper!(glFrontFace, (mode: GLenum));
gl_wrapper!(glGenBuffers, (n: GLsizei, buffers: *mut GLuint));
gl_wrapper!(glGenFramebuffers, (n: GLsizei, framebuffers: *mut GLuint));
gl_wrapper!(glGenRenderbuffers, (n: GLsizei, renderbuffers: *mut GLuint));
gl_wrapper!(glGenTextures, (n: GLsizei, textures: *mut GLuint));
gl_wrapper!(glGenerateMipmap, (target: GLenum));
gl_wrapper!(glGetAttribLocation, (program: GLuint, name: *const GLchar) -> GLint);
gl_wrapper!(glGetBooleanv, (pname: GLenum, data: *mut GLboolean));
gl_wrapper!(glGetBufferParameteriv, (target: GLenum, pname: GLenum, params: *mut GLint));
gl_wrapper!(glGetError, () -> GLenum);
gl_wrapper!(glGetFloatv, (pname: GLenum, data: *mut GLfloat));
gl_wrapper!(glGetIntegerv, (pname: GLenum, data: *mut GLint));
gl_wrapper!(glGetProgramInfoLog, (program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar));
gl_wrapper!(glGetProgramiv, (program: GLuint, pname: GLenum, params: *mut GLint));
gl_wrapper!(glGetShaderInfoLog, (shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar));
gl_wrapper!(glGetShaderiv, (shader: GLuint, pname: GLenum, params: *mut GLint));
gl_wrapper!(glGetString, (name: GLenum) -> *const GLubyte);
gl_wrapper!(glGetStringi, (name: GLenum, index: GLuint) -> *const GLubyte);
gl_wrapper!(glGetUniformLocation, (program: GLuint, name: *const GLchar) -> GLint);
gl_wrapper!(glIsBuffer, (buffer: GLuint) -> GLboolean);
gl_wrapper!(glIsEnabled, (cap: GLenum) -> GLboolean);
gl_wrapper!(glIsProgram, (program: GLuint) -> GLboolean);
gl_wrapper!(glIsShader, (shader: GLuint) -> GLboolean);
gl_wrapper!(glIsTexture, (texture: GLuint) -> GLboolean);
gl_wrapper!(glLineWidth, (width: GLfloat));
gl_wrapper!(glLinkProgram, (program: GLuint));
gl_wrapper!(glPixelStorei, (pname: GLenum, param: GLint));
gl_wrapper!(glPolygonOffset, (factor: GLfloat, units: GLfloat));
gl_wrapper!(glReadPixels, (x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void));
gl_wrapper!(glRenderbufferStorage, (target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei));
gl_wrapper!(glSampleCoverage, (value: GLclampf, invert: GLboolean));
gl_wrapper!(glScissor, (x: GLint, y: GLint, width: GLsizei, height: GLsizei));
gl_wrapper!(glShaderSource, (shader: GLuint, count: GLsizei, string_: *const *const GLchar, length: *const GLint));
gl_wrapper!(glStencilFunc, (func: GLenum, ref_: GLint, mask: GLuint));
gl_wrapper!(glStencilFuncSeparate, (face: GLenum, func: GLenum, ref_: GLint, mask: GLuint));
gl_wrapper!(glStencilMask, (mask: GLuint));
gl_wrapper!(glStencilMaskSeparate, (face: GLenum, mask: GLuint));
gl_wrapper!(glStencilOp, (fail: GLenum, zfail: GLenum, zpass: GLenum));
gl_wrapper!(glStencilOpSeparate, (face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum));
gl_wrapper!(glTexImage2D, (target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void));
gl_wrapper!(glTexParameterf, (target: GLenum, pname: GLenum, param: GLfloat));
gl_wrapper!(glTexParameterfv, (target: GLenum, pname: GLenum, params: *const GLfloat));
gl_wrapper!(glTexParameteri, (target: GLenum, pname: GLenum, param: GLint));
gl_wrapper!(glTexParameteriv, (target: GLenum, pname: GLenum, params: *const GLint));
gl_wrapper!(glTexSubImage2D, (target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void));
gl_wrapper!(glUniform1f, (location: GLint, v0: GLfloat));
gl_wrapper!(glUniform1fv, (location: GLint, count: GLsizei, value: *const GLfloat));
gl_wrapper!(glUniform1i, (location: GLint, v0: GLint));
gl_wrapper!(glUniform1iv, (location: GLint, count: GLsizei, value: *const GLint));
gl_wrapper!(glUniform2f, (location: GLint, v0: GLfloat, v1: GLfloat));
gl_wrapper!(glUniform2fv, (location: GLint, count: GLsizei, value: *const GLfloat));
gl_wrapper!(glUniform3f, (location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat));
gl_wrapper!(glUniform3fv, (location: GLint, count: GLsizei, value: *const GLfloat));
gl_wrapper!(glUniform4f, (location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat));
gl_wrapper!(glUniform4fv, (location: GLint, count: GLsizei, value: *const GLfloat));
gl_wrapper!(glUniformMatrix3fv, (location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat));
gl_wrapper!(glUniformMatrix4fv, (location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat));
gl_wrapper!(glUseProgram, (program: GLuint));
gl_wrapper!(glValidateProgram, (program: GLuint));
gl_wrapper!(glVertexAttrib1f, (index: GLuint, x: GLfloat));
gl_wrapper!(glVertexAttrib2f, (index: GLuint, x: GLfloat, y: GLfloat));
gl_wrapper!(glVertexAttrib3f, (index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat));
gl_wrapper!(glVertexAttrib4f, (index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat));
gl_wrapper!(glVertexAttribPointer, (index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void));
gl_wrapper!(glViewport, (x: GLint, y: GLint, width: GLsizei, height: GLsizei));

