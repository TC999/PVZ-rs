// PvZ Portable Rust 翻译 — SharedImage 共享图像管理
// 对应 C++ SexyAppFramework/graphics/SharedImage.h / SharedImage.cpp
//
// SharedImage 是引用计数的图像包装器，用于资源管理器中的图像缓存。
// SharedImageRef 是智能指针，类似 Rc<RefCell<Image>>。

#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use crate::framework::graphics::image::Image;
use crate::framework::graphics::gl_image::GLImage;
use crate::framework::graphics::memory_image::MemoryImage;

/// 共享图像（对应 C++ SharedImage）
/// 存储 GLImage 指针和原子引用计数
#[derive(Debug)]
pub struct SharedImage {
    /// OpenGL 纹理图像
    pub image: *mut GLImage,
    /// 原子引用计数
    pub ref_count: AtomicI32,
}

impl SharedImage {
    pub fn new() -> Self {
        SharedImage {
            image: std::ptr::null_mut(),
            ref_count: AtomicI32::new(0),
        }
    }
}

/// 全局共享图像清理标志
/// 对应 C++ gSexyAppBase->mCleanupSharedImages
pub static mut CLEANUP_SHARED_IMAGES: AtomicBool = AtomicBool::new(false);

/// 共享图像引用（对应 C++ SharedImageRef）
/// 智能指针包装，支持引用计数和隐式类型转换
#[derive(Debug)]
pub struct SharedImageRef {
    /// 共享图像对象（引用计数）
    pub shared_image: *mut SharedImage,
    /// 非共享图像（独占所有权）
    pub unshared_image: *mut MemoryImage,
    /// 是否拥有非共享图像（需要析构时释放）
    pub owns_unshared: bool,
}

impl SharedImageRef {
    pub fn new() -> Self {
        SharedImageRef {
            shared_image: std::ptr::null_mut(),
            unshared_image: std::ptr::null_mut(),
            owns_unshared: false,
        }
    }

    /// 从 SharedImage 指针构造（增加引用计数）
    pub fn from_shared(si: *mut SharedImage) -> Self {
        if !si.is_null() {
            unsafe {
                (*si).ref_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        SharedImageRef {
            shared_image: si,
            unshared_image: std::ptr::null_mut(),
            owns_unshared: false,
        }
    }

    /// 释放资源（对应 C++ Release）
    pub fn release(&mut self) {
        if self.owns_unshared && !self.unshared_image.is_null() {
            // 在 Rust 中，需要手动释放 MemoryImage
            // 但 MemoryImage 应该已经由 Rust 的所有权系统管理
            // 这里不调用 drop，因为 mUnsharedImage 可能是栈分配或由其他所有者管理
            unsafe {
                let _ = Box::from_raw(self.unshared_image);
            }
        }
        self.unshared_image = std::ptr::null_mut();

        if !self.shared_image.is_null() {
            unsafe {
                let prev = (*self.shared_image).ref_count.fetch_sub(1, Ordering::Relaxed);
                if prev == 1 {
                    // 引用计数归零，标记清理
                    CLEANUP_SHARED_IMAGES.store(true, Ordering::Relaxed);
                }
            }
        }
        self.shared_image = std::ptr::null_mut();
        self.owns_unshared = false;
    }

    /// 复制（增加引用计数）
    pub fn clone_from(&mut self, other: &SharedImageRef) {
        self.release();
        self.shared_image = other.shared_image;
        if !self.shared_image.is_null() {
            unsafe {
                (*self.shared_image).ref_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        self.unshared_image = other.unshared_image;
        self.owns_unshared = false; // 不拥有其他对象的 unshared
    }

    /// 设置为 SharedImage 指针
    pub fn set_shared(&mut self, si: *mut SharedImage) {
        self.release();
        self.shared_image = si;
        if !si.is_null() {
            unsafe {
                (*si).ref_count.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    /// 设置为 MemoryImage（独占）
    pub fn set_unshared(&mut self, mi: *mut MemoryImage) {
        self.release();
        self.unshared_image = mi;
        // owns_unshared 保持 false，因为所有权由调用方管理
    }

    /// 获取 Image 指针（对应 C++ operator Image*）
    pub fn as_image_ptr(&self) -> *mut Image {
        self.as_memory_image_ptr() as *mut Image
    }

    /// 获取 MemoryImage 指针（对应 C++ operator MemoryImage*）
    pub fn as_memory_image_ptr(&self) -> *mut MemoryImage {
        if !self.unshared_image.is_null() {
            self.unshared_image
        } else {
            self.as_gl_image_ptr() as *mut MemoryImage
        }
    }

    /// 获取 GLImage 指针（对应 C++ operator GLImage*）
    pub fn as_gl_image_ptr(&self) -> *mut GLImage {
        if !self.shared_image.is_null() {
            unsafe { (*self.shared_image).image }
        } else {
            std::ptr::null_mut()
        }
    }
}

impl Clone for SharedImageRef {
    fn clone(&self) -> Self {
        let mut new_ref = SharedImageRef::new();
        new_ref.shared_image = self.shared_image;
        if !new_ref.shared_image.is_null() {
            unsafe {
                (*new_ref.shared_image).ref_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        new_ref.unshared_image = self.unshared_image;
        new_ref.owns_unshared = false;
        new_ref
    }
}

impl Drop for SharedImageRef {
    fn drop(&mut self) {
        self.release();
    }
}
