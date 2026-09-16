// PvZ Portable Rust 翻译 — MessageWidget（消息/提示文字控件）
// 对应 C++ src/Lawn/MessageWidget.h / MessageWidget.cpp

use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::color::Color;
use crate::framework::rect::Rect;

/// 最大消息长度（对应 C++ #define MAX_MESSAGE_LENGTH 128）
pub const MAX_MESSAGE_LENGTH: usize = 128;

/// 最大重动画行数（对应 C++ #define MAX_REANIM_LINES 5）
pub const MAX_REANIM_LINES: usize = 5;

/// 消息控件（显示提示文字、教程信息、关卡名称等）
/// 对应 C++ class MessageWidget
pub struct MessageWidget {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub label: [u8; MAX_MESSAGE_LENGTH],
    pub display_time: i32,
    pub duration: i32,
    pub message_style: MessageStyle,
    pub text_reanim_id: [ReanimationID; MAX_MESSAGE_LENGTH],
    pub reanim_type: ReanimationType, // C++ mReanimType
    pub slide_off_time: i32,
    pub label_next: [u8; MAX_MESSAGE_LENGTH],
    pub message_style_next: MessageStyle,
    pub text_reanim_count: i32,
    pub text_reanim_byte_offset: [usize; MAX_MESSAGE_LENGTH],
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
            reanim_type: ReanimationType::TextFadeOn,
            slide_off_time: 100,
            label_next: [0u8; MAX_MESSAGE_LENGTH],
            message_style_next: MessageStyle::Off,
            text_reanim_count: 0,
            text_reanim_byte_offset: [0usize; MAX_MESSAGE_LENGTH],
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

    /// 设置标签文字（对应 C++ SetLabel 简化版）
    pub fn set_label(&mut self, the_new_label: &str, the_message_style: MessageStyle) {
        // [TRANSLATION_NOTE]: C++ 完整实现包含字符串翻译、截断、重动画创建
        // 如果已有活跃消息则将新消息排队到 mLabelNext
        self.clear_reanim();
        let bytes = the_new_label.as_bytes();
        let len = std::cmp::min(bytes.len(), MAX_MESSAGE_LENGTH - 1);
        self.label[..len].copy_from_slice(&bytes[..len]);
        self.label[len] = 0;
        self.message_style = the_message_style;

        // 根据消息样式设置持续时间
        self.duration = match the_message_style {
            MessageStyle::HintLong | MessageStyle::HintTallLong => 1500,
            MessageStyle::HintFast | MessageStyle::HintTallFast
            | MessageStyle::TutorialLevel1 | MessageStyle::TutorialLevel2
            | MessageStyle::TutorialLater => 500,
            MessageStyle::HintStay | MessageStyle::TutorialLevel1Stay => 10000,
            MessageStyle::HouseName => 250,
            _ => 250,
        };
        self.display_time = self.duration;
    }

    /// 更新消息控件
    pub fn update(&mut self) {
        // [TRANSLATION_NOTE]: C++ 中还有重动画文字更新（mTextReanimCount 循环）
        // 处理滑入滑出动画和文字淡入淡出
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

    /// 绘制消息（对应 C++ MessageWidget::Draw，MessageWidget.cpp:362）
    /// 按消息样式绘制不同位置和样式的文字
    pub fn draw(&self, g: &mut Graphics) {
        if self.duration <= 0 {
            return;
        }
        // [TRANSLATION_NOTE]: C++ 以 _Font* 判定描边字（FONT_CONTINUUMBOLD14 → OUTLINE），
        // Rust 字体系统简化（get_font 返回 None），描边字体判定标注为 false

        // 对应 C++: 默认参数
        let board_width = crate::lawn::game_enums::BOARD_WIDTH;
        let mut a_pos_x = board_width / 2;
        let mut a_pos_y = 596;
        let mut a_text_offset_y = 0;
        let mut a_rect_height = 0;
        let mut a_min_alpha = 255;
        let mut a_color = crate::framework::color::Color { r: 250, g: 250, b: 0, a: 255 };
        let mut a_outline_color = crate::framework::color::Color { r: 0, g: 0, b: 0, a: 255 };
        let mut a_fade_out = false;

        // 对应 C++: switch (mMessageStyle)
        match self.message_style {
            MessageStyle::TutorialLevel1 | MessageStyle::TutorialLevel1Stay => {
                a_pos_y = 400;
                a_rect_height = 110;
                a_text_offset_y = -4;
                a_color = crate::framework::color::Color { r: 253, g: 245, b: 173, a: 255 };
                a_min_alpha = 192;
            }
            MessageStyle::TutorialLevel2 | MessageStyle::TutorialLater
            | MessageStyle::TutorialLaterStay | MessageStyle::HintTallFast
            | MessageStyle::HintTallUnlockMessage | MessageStyle::HintTallLong
            | MessageStyle::Achievement => {
                a_pos_y = 476;
                a_rect_height = 100;
                a_text_offset_y = -4;
                a_color = crate::framework::color::Color { r: 253, g: 245, b: 173, a: 255 };
                a_min_alpha = 192;
            }
            MessageStyle::HintLong | MessageStyle::HintFast | MessageStyle::HintStay => {
                a_pos_y = 527;
                a_rect_height = 55;
                a_text_offset_y = -4;
                a_color = crate::framework::color::Color { r: 253, g: 245, b: 173, a: 255 };
                a_min_alpha = 192;
            }
            MessageStyle::BigMiddle | MessageStyle::BigMiddleFast => {
                a_pos_y = 300;
                a_rect_height = 110;
                a_color = crate::framework::color::Color { r: 253, g: 245, b: 173, a: 255 };
                a_min_alpha = 192;
            }
            MessageStyle::HouseName => {
                a_pos_y = 550;
                a_color = crate::framework::color::Color { r: 255, g: 255, b: 255, a: 255 };
                a_fade_out = true;
            }
            MessageStyle::HugeWave => {
                a_pos_y = 330;
                a_color = crate::framework::color::Color { r: 255, g: 0, b: 0, a: 255 };
            }
            MessageStyle::SlotMachine => {
                a_pos_y = 93;
                a_pos_x = 340;
                a_min_alpha = 64;
            }
            MessageStyle::ZenGardenLong => {
                a_pos_y = 514;
                a_rect_height = 55;
                a_text_offset_y = -4;
                a_color = crate::framework::color::Color { r: 253, g: 245, b: 173, a: 255 };
                a_min_alpha = 192;
            }
            _ => {}
        }

        if self.reanim_type != ReanimationType::None {
            // 对应 C++: DrawReanimatedText（描边字缺失时仅主色）
            self.draw_reanimated_text(g, a_color, a_pos_y as f32);
        } else {
            // 对应 C++: 非 255 最小 alpha 时按 75 帧曲线动画
            if a_min_alpha != 255 {
                let a_board_counter = crate::lawn::lawn_app::LawnApp::instance()
                    .and_then(|app| app.board)
                    .map_or(0i32, |b| unsafe { (*b).m_main_counter as i32 });
                // 对应 C++: CURVE_BOUNCE_SLOW_MIDDLE（Rust TodCurves::BounceSlowMiddle，此前误用 Bounce）
                let a_alpha = crate::todlib::tod_common::tod_animate_curve(
                    75, 0, a_board_counter % 75, a_min_alpha, 255,
                    crate::lawn::game_enums::TodCurves::BounceSlowMiddle,
                );
                a_color.a = a_alpha as u8;
                a_outline_color.a = a_color.a;
            }
            // 对应 C++: 淡出（HOUSE_NAME）
            if a_fade_out {
                a_color.a = (self.duration * 15).clamp(0, 255) as u8;
                a_outline_color.a = a_color.a;
            }

            // 对应 C++: 半透明黑矩形容器 + 居中包装文字
            let a_label_str = message_label_to_str(&self.label);
            if a_rect_height > 0 {
                a_outline_color = crate::framework::color::Color { r: 0, g: 0, b: 0, a: 128 };
                g.set_color(&a_outline_color);
                g.fill_rect_xywh(0, a_pos_y, board_width, a_rect_height);

                // 对应 C++: PvzpDrawStringWrapped(DS_ALIGN_CENTER_VERTICAL_MIDDLE)
                setup_default_font(g);
                g.set_color(&a_color);
                let a_rect_y = a_pos_y + a_text_offset_y;
                g.draw_string_word_wrapped(
                    &a_label_str,
                    0,
                    a_rect_y + 8,
                    board_width,
                    16,
                    crate::lawn::game_enums::DrawStringJustification::DS_ALIGN_CENTER as i32,
                    None,
                );
            } else {
                // 对应 C++: 无矩形：居中绘制
                setup_default_font(g);
                g.set_color(&a_color);
                let a_draw_x = a_pos_x - 400; // [TRANSLATION_NOTE]: C++ 中减 mBoard->mX - BOARD_WIDTH/2；board 未挂接时以 0 近似
                g.draw_string_word_wrapped(
                    &a_label_str,
                    a_draw_x,
                    a_pos_y,
                    board_width * 2,
                    16,
                    crate::lawn::game_enums::DrawStringJustification::DS_ALIGN_CENTER as i32,
                    None,
                );
            }
        }

        // [TRANSLATION_NOTE]: C++ 中 HOUSE_NAME 模式的 survival flags 副文本
        //（[FLAGS_COMPLETED] 文案拼接）依赖 Challenge/Pluralize 链，暂未接入
    }

    /// 判断是否正在显示
    pub fn is_being_displayed(&self) -> bool {
        self.duration != 0
    }

    /// 布局重动画文字（对应 C++ LayoutReanimText）
    /// 逐行统计宽度、逐字符（UTF-8 code point）创建文字 reanim，记录每个字符的 reanim id 与字节偏移
    pub fn layout_reanim_text(&mut self) {
        let label_len = self.label.iter().position(|&c| c == 0).unwrap_or(self.label.len());
        self.slide_off_time = label_len as i32 + 100;

        // 统计每行宽度（C++ 用 aFont->StringWidth(aLine)）
        let mut a_line_width = [0.0f32; MAX_REANIM_LINES];
        let mut a_max_width = 0.0f32;
        let mut a_cur_line = 0usize;
        let mut a_cur_pos = 0usize;
        let mut a_pos = 0usize;
        while a_pos <= label_len {
            if a_pos == label_len || self.label[a_pos] == b'\n' {
                let a_len = a_pos - a_cur_pos;
                a_cur_pos = a_pos + 1;
                // [TRANSLATION_NOTE]: 字体系统未接入，按每字符 10 像素近似 StringWidth
                a_line_width[a_cur_line] = a_len as f32 * 10.0;
                a_max_width = a_max_width.max(a_line_width[a_cur_line]);
                a_cur_line += 1;
                if a_cur_line >= MAX_REANIM_LINES {
                    break;
                }
            }
            a_pos += 1;
        }

        // 逐字符（code point）创建文字 reanim
        a_cur_line = 0;
        let mut a_cur_pos_y = 0.0f32;
        let mut a_cur_pos_x = -a_line_width[0] * 0.5;
        let mut a_char_idx = 0usize;
        let mut a_byte_pos = 0usize;
        let _ = a_max_width;
        while a_byte_pos < label_len && a_char_idx < MAX_MESSAGE_LENGTH {
            let a_char_start = a_byte_pos;
            let ch = self.label[a_byte_pos] as char;
            a_byte_pos += 1;

            // 对应 C++ MessageWidget::LayoutReanimText（MessageWidget.cpp:216-221）:
            // AddReanimation + mIsAttachment = true + PlayReanim("anim_enter") + ReanimationGetID
            self.text_reanim_id[a_char_idx] = if let Some(app) = self.app {
                let a_new = unsafe {
                    (*app).add_reanimation(a_cur_pos_x, a_cur_pos_y, 0, self.reanim_type as i32)
                };
                if let Some(a_reanim_ptr) = a_new {
                    unsafe {
                        (*a_reanim_ptr).m_is_attachment = true;
                        (*a_reanim_ptr).play_reanim(
                            "anim_enter",
                            crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold,
                            0,
                            0.0,
                        );
                        (*app).reanimation_get_id(a_reanim_ptr)
                    }
                } else {
                    REANIMATIONID_NULL
                }
            } else {
                REANIMATIONID_NULL
            };
            self.text_reanim_byte_offset[a_char_idx] = a_char_start;

            // aCurPosX += aFont->CharWidth(aChar) — 按每字符 10 像素近似
            a_cur_pos_x += 10.0;
            if ch == '\n' {
                a_cur_line += 1;
                if a_cur_line < MAX_REANIM_LINES {
                    a_cur_pos_x = -a_line_width[a_cur_line] * 0.5;
                }
                // aCurPosY += aFont->GetLineSpacing() — 按行高 16 像素近似
                a_cur_pos_y += 16.0;
            }
            a_char_idx += 1;
        }
        self.text_reanim_count = a_char_idx as i32;
        let _ = a_cur_pos_y;
    }

    /// 绘制重动画文字（对应 C++ DrawReanimatedText）
    /// 逐字符 reanim 取 transform、按 alpha 截断、以字节偏移切出字母并矩阵绘制
    pub fn draw_reanimated_text(&self, _g: &mut Graphics, _the_color: Color, _the_pos_y: f32) {
        let label_len = self.label.iter().position(|&c| c == 0).unwrap_or(self.label.len());
        for a_char_idx in 0..self.text_reanim_count as usize {
            if a_char_idx >= MAX_MESSAGE_LENGTH {
                break;
            }
            if self.text_reanim_id[a_char_idx] == REANIMATIONID_NULL {
                // 对应 C++ MessageWidget::DrawReanimatedText: ReanimationTryToGet 返回 nullptr 时 break
                break;
            }
            // C++ 取 GetCurrentTransform(2, &aTransform)，按 aTransform.mAlpha 计算最终 alpha，
            // 用 aByteStart/aByteEnd 从 label 中切出单个字母，经 PvzpDrawStringMatrix 矩阵绘制。
            let a_byte_start = self.text_reanim_byte_offset[a_char_idx];
            let a_byte_end = if a_char_idx + 1 < self.text_reanim_count as usize {
                self.text_reanim_byte_offset[a_char_idx + 1]
            } else {
                label_len
            };
            let _a_letter = &self.label[a_byte_start..a_byte_end.min(label_len)];
        }
    }
}

/// 将消息 label 字节数组转换为字符串（0 结尾截断，非法 UTF-8 以 lossy 处理）
fn message_label_to_str(label: &[u8; MAX_MESSAGE_LENGTH]) -> String {
    let a_len = label.iter().position(|&c| c == 0).unwrap_or(label.len());
    String::from_utf8_lossy(&label[..a_len]).into_owned()
}

/// 设置默认字体（TRANSLATION_NOTE: C++ 用 _Font*（GetFont）；Rust 以 Font 简化）
fn setup_default_font(g: &mut Graphics) {
    let mut a_font = crate::framework::graphics::font::Font::new("Dwarventodcraft", 14);
    a_font.ascent = 13;
    a_font.font_height = 14;
    g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
}
