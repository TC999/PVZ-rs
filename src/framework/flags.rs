// PvZ Portable Rust 翻译 — Flags 位标志管理
// 对应 C++ SexyAppFramework/misc/Flags.h / Flags.cpp

#![allow(dead_code)]

/// 位标志修改器（对应 C++ FlagsMod）
#[derive(Debug, Clone, Copy)]
pub struct FlagsMod {
    pub add_flags: i32,
    pub remove_flags: i32,
}

impl FlagsMod {
    pub const fn new() -> Self { FlagsMod { add_flags: 0, remove_flags: 0 } }
}

/// 修改标志位（对应 C++ ModFlags）
pub fn mod_flags(flags: &mut i32, fm: &FlagsMod) {
    *flags = (*flags | fm.add_flags) & !fm.remove_flags;
}

/// 获取修改后标志位
pub fn get_mod_flags(flags: i32, fm: &FlagsMod) -> i32 {
    (flags | fm.add_flags) & !fm.remove_flags
}

/// 模态标志（对应 C++ ModalFlags）
#[derive(Debug, Clone, Copy)]
pub struct ModalFlags {
    pub over_flags: i32,
    pub under_flags: i32,
    pub is_over: bool,
}

impl ModalFlags {
    pub fn new() -> Self { ModalFlags { over_flags: 0, under_flags: 0, is_over: false } }

    pub fn mod_flags_self(&mut self, fm: &FlagsMod) {
        mod_flags(&mut self.over_flags, fm);
        mod_flags(&mut self.under_flags, fm);
    }

    pub fn get_flags(&self) -> i32 {
        if self.is_over { self.over_flags } else { self.under_flags }
    }
}

/// 自动模态标志 RAII（对应 C++ AutoModalFlags）
pub struct AutoModalFlags {
    modal_flags: *mut ModalFlags,
    old_over: i32,
    old_under: i32,
}

impl AutoModalFlags {
    pub fn new(modal_flags: *mut ModalFlags, fm: &FlagsMod) -> Self {
        let old_over = unsafe { (*modal_flags).over_flags };
        let old_under = unsafe { (*modal_flags).under_flags };
        unsafe { (*modal_flags).mod_flags_self(fm); }
        AutoModalFlags { modal_flags, old_over, old_under }
    }
}

impl Drop for AutoModalFlags {
    fn drop(&mut self) {
        unsafe {
            (*self.modal_flags).over_flags = self.old_over;
            (*self.modal_flags).under_flags = self.old_under;
        }
    }
}
