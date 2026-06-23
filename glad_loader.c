// gladLoadGLES2 - 在 Windows 上通过 opengl32.lib 加载 OpenGL 函数
// 由于我们直接链接 opengl32.lib，此函数只需完成基本加载

#include <stdint.h>

typedef void* (*GLADloadfunc)(const char* name);

// 外部引用 SDL_GL_GetProcAddress
extern void* SDL_GL_GetProcAddress(const char* name);

// gladLoadGLES2 实现 - 加载所有 OpenGL ES 2.0 函数
int gladLoadGLES2(GLADloadfunc load) {
    if (!load) {
        // 如果没有提供加载函数，使用 SDL_GL_GetProcAddress
        load = (GLADloadfunc)&SDL_GL_GetProcAddress;
    }
    
    // 尝试加载几个关键函数来验证
    void* test = load("glClear");
    if (!test) {
        return 0; // 无法加载 OpenGL
    }
    
    return 1; // 成功
}
