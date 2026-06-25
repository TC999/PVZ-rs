// PvZ Portable Rust 翻译 — TypingCheck（打字检测辅助类）
// 对应 C++ src/Lawn/System/TypingCheck.h / TypingCheck.cpp

use crate::framework::key_codes::KeyCode;

/// 打字检测类：用于检测玩家是否输入了特定短语
/// （如秘籍代码、作弊码等）
#[derive(Debug)]
pub struct TypingCheck {
    /// 目标短语（连续的键码序列）
    m_phrase: Vec<KeyCode>,
    /// 最近输入的键码缓冲区
    m_recent_typing: Vec<KeyCode>,
}

impl TypingCheck {
    pub fn new() -> Self {
        TypingCheck {
            m_phrase: Vec::new(),
            m_recent_typing: Vec::new(),
        }
    }

    /// 从字符串构造（将字符串转换为小写键码序列）
    pub fn with_phrase(phrase: &str) -> Self {
        let mut check = TypingCheck::new();
        check.set_phrase(phrase);
        check
    }

    /// 设置目标短语（自动将字符转换为小写键码）
    pub fn set_phrase(&mut self, phrase: &str) {
        self.m_phrase.clear();
        for c in phrase.chars() {
            self.add_char(c);
        }
    }

    /// 添加一个键码到目标短语
    pub fn add_key_code(&mut self, key_code: KeyCode) {
        self.m_phrase.push(key_code);
    }

    /// 添加一个字符到目标短语（转为小写后获取其键码）
    pub fn add_char(&mut self, ch: char) {
        let lower = ch.to_ascii_lowercase();
        // 将字符的小写 ASCII 值作为键码添加
        if lower.is_ascii() {
            self.m_phrase.push(lower as i32);
        }
    }

    /// 检查最近输入是否匹配目标短语（不添加新输入）
    pub fn check(&self) -> bool {
        if self.m_recent_typing == self.m_phrase {
            true
        } else {
            false
        }
    }

    /// 检查键码输入（添加键码到缓冲区后检查匹配）
    /// 匹配成功时清空缓冲区
    pub fn check_key(&mut self, key_code: KeyCode) -> bool {
        self.m_recent_typing.push(key_code);

        let len = self.m_phrase.len();
        if len == 0 {
            return false;
        }

        // 保持缓冲区不超过目标短语长度
        if self.m_recent_typing.len() > len {
            let excess = self.m_recent_typing.len() - len;
            self.m_recent_typing = self.m_recent_typing.split_off(excess);
        }

        if self.check() {
            self.m_recent_typing.clear();
            true
        } else {
            false
        }
    }
}

impl Default for TypingCheck {
    fn default() -> Self {
        Self::new()
    }
}
