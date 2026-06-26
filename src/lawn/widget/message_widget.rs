// PvZ Portable Rust 翻译 — MessageWidget（消息/提示文字控件）
// 对应 C++ src/Lawn/MessageWidget.h / MessageWidget.cpp

use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::color::Color;
use crate::framework::rect::Rect;

/// 最大消息长度（对应 C++ #define MAX_MESSAGE_LENGTH 128）
pub const MAX_MESSAGE_LENGTH: usize = 128;

/// 消息控件（显示提示文字、教程信息、关卡名称等）
/// 对应 C++ class MessageWidget
pub struct MessageWidget {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub label: [u8; MAX_MESSAGE_LENGTH],
    pub display_time: i32,
    pub duration: i32,
    pub message_style: MessageStyle,
    pub text_reanim_id: [ReanimationID; MAX_MESSAGE_LENGTH],
    // pub reanim_type: ReanimationType, // C++ mReanimType
    pub slide_off_time: i32,
    pub label_next: [u8; MAX_MESSAGE_LENGTH],
    pub message_style_next: MessageStyle,
}

impl MessageWidget {
    pub fn new(the_app: Option<*mut crate::lawn::lawn_app::LawnApp>) -> Self {
        MessageWidget {
            app: the_app,
            label: [0u8; MAX_MESSAGE_LENGTH],
            display_time: 0,
            duration: 0,
            message_style: MessageStyle::Off,
            text_reanim_id: [REANIMATIONID_NULL; MAX_MESSAGE_LENGTH],
            slide_off_time: 100,
            label_next: [0u8; MAX_MESSAGE_LENGTH],
            message_style_next: MessageStyle::Off,
        }
    }

    /// 清除所有重动画
    pub fn clear_reanim(&mut self) {
        for i in 0..MAX_MESSAGE_LENGTH {
            self.text_reanim_id[i] = REANIMATIONID_NULL;
        }
    }

    /// 清除标签
    pub fn clear_label(&mut self) {
        self.duration = 0;
    }

    /// 设置标签文字
    pub fn set_label(&mut self, the_new_label: &str, the_message_style: MessageStyle) {
        self.clear_reanim();
        let bytes = the_new_label.as_bytes();
        let len = std::cmp::min(bytes.len(), MAX_MESSAGE_LENGTH - 1);
        self.label[..len].copy_from_slice(&bytes[..len]);
        self.label[len] = 0;
        self.message_style = the_message_style;

        // 根据消息样式设置持续时间
        self.duration = match the_message_style {
            MessageStyle::HintLong | MessageStyle::HintMedium => 1500,
            MessageStyle::HintFast | MessageStyle::TutorialLevel1
            | MessageStyle::TutorialLevel2 | MessageStyle::TutorialLater => 500,
            MessageStyle::HintStay | MessageStyle::TutorialLevel1Stay => 10000,
            _ => 250,
        };
        self.display_time = self.duration;
    }

    /// 更新消息控件
    pub fn update(&mut self) {
        if self.duration < 10000 && self.duration > 0 {
            self.duration -= 1;
            if self.duration == 0 {
                self.message_style = MessageStyle::Off;
                if self.message_style_next != MessageStyle::Off {
                    // SetLabel(mLabelNext, mMessageStyleNext)
                    self.message_style_next = MessageStyle::Off;
                }
            }
        }
    }

    /// 获取字体（简化实现）
    pub fn get_font(&self) -> Option<&'static Image> {
        None
    }

    /// 绘制消息
    pub fn draw(&self, _g: &mut Graphics) {
        if self.duration <= 0 {
            return;
        }
        // TODO: 完整绘制逻辑（来自 C++ MessageWidget::Draw, 约 150 行）
    }

    /// 判断是否正在显示
    pub fn is_being_displayed(&self) -> bool {
        self.duration != 0
    }
}
