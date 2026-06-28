// PvZ Portable Rust 翻译 — TodDebug（调试工具和断言宏）
// 对应 C++ src/Sexy.TodLib/TodDebug.h / TodDebug.cpp

#![allow(dead_code)]

use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

/// 性能计时括号（对应 C++ TodHesitationBracket）
/// 用于测量一段代码的执行时间
pub struct TodHesitationBracket {
    message: [u8; 256],
    start_time: i32,
}

impl TodHesitationBracket {
    pub fn new(message: &str) -> Self {
        let mut buf = [0u8; 256];
        let bytes = message.as_bytes();
        let len = bytes.len().min(255);
        buf[..len].copy_from_slice(&bytes[..len]);
        buf[len] = 0;
        TodHesitationBracket {
            message: buf,
            start_time: 0,
        }
    }

    pub fn end_bracket(&mut self) {
        // 目前为空实现（对应 C++ 空实现）
    }
}

/// 全局日志文件名缓冲区
static mut G_LOG_FILE_NAME: [u8; 512] = [0; 512];
/// 全局调试数据文件夹路径缓冲区
static mut G_DEBUG_DATA_FOLDER: [u8; 512] = [0; 512];

/// 记录日志（自动追加换行）
pub fn tod_log(format: &str) {
    let mut msg = format.to_string();
    if !msg.ends_with('\n') {
        msg.push('\n');
    }
    tod_log_string(&msg);
}

/// 记录日志字符串
pub fn tod_log_string(msg: &str) {
    #[cfg(debug_assertions)]
    {
        eprint!("{}", msg);
    }
}

/// 输出调试跟踪（对应 C++ TodTrace）
pub fn tod_trace(format: &str) {
    let mut msg = format.to_string();
    if !msg.ends_with('\n') {
        msg.push('\n');
    }
    eprint!("{}", msg);
}

/// 跟踪内存信息（对应 C++ TodTraceMemory，当前为空实现）
pub fn tod_trace_memory() {}

/// 跟踪并记录日志
pub fn tod_trace_and_log(format: &str) {
    tod_trace(format);
    tod_log_string(format);
}

/// 防刷屏跟踪（每秒最多输出一次）
pub fn tod_trace_without_spamming(format: &str) {
    use std::sync::atomic::{AtomicU64, Ordering};
    static LAST_TRACE_TIME: AtomicU64 = AtomicU64::new(0);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let last = LAST_TRACE_TIME.load(Ordering::Relaxed);
    if now < last {
        return;
    }
    LAST_TRACE_TIME.store(now, Ordering::Relaxed);

    let mut msg = format.to_string();
    if !msg.ends_with('\n') {
        msg.push('\n');
    }
    eprint!("{}", msg);
}

/// 空函数 — 对应 C++ TodHesitationTrace（空实现）
pub fn tod_hesitation_trace() {}

/// 断言失败处理
/// 对应 C++ TodAssertFailed
pub fn tod_assert_failed(
    condition: &str,
    file: &str,
    line: i32,
    msg: &str,
) {
    let formatted = if !condition.is_empty() {
        format!(
            "\n{}({})\nassertion failed: '{}'\n{}\n",
            file, line, condition, msg
        )
    } else {
        format!(
            "\n{}({})\nassertion failed: {}\n",
            file, line, msg
        )
    };
    eprint!("{}", formatted);

    // 弹错误框（对应 C++ TodErrorMessageBox）
    eprintln!("Assertion failed: {}", formatted);
    process::exit(1);
}

/// 初始化断言系统（对应 C++ TodAssertInitForApp）
pub fn tod_assert_init_for_app() {
    // 在 Rust 中，日志路径初始化简化为空操作
    // 原 C++ 会创建 userdata 目录并打开日志文件
    tod_log(&format!(
        "Started {}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    ));
}

/// TOD_ASSERT 宏 — 调试断言
/// 对应 C++ 的 TOD_ASSERT(condition, ...)
#[macro_export]
macro_rules! tod_assert {
    ($condition:expr) => {
        if cfg!(debug_assertions) {
            if !$condition {
                $crate::todlib::tod_debug::tod_assert_failed(
                    stringify!($condition),
                    file!(),
                    line!() as i32,
                    "",
                );
            }
        }
    };
    ($condition:expr, $($arg:tt)+) => {
        if cfg!(debug_assertions) {
            if !$condition {
                $crate::todlib::tod_debug::tod_assert_failed(
                    stringify!($condition),
                    file!(),
                    line!() as i32,
                    &format!($($arg)+),
                );
            }
        }
    };
}

/// 断言崩溃（对应 C++ TodCrash）
/// 用于不可恢复的错误场景
pub fn tod_crash() {
    tod_assert!(false, "Crash!!!!");
}
