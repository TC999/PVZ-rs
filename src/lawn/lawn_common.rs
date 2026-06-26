// PvZ Portable Rust 翻译 — LawnCommon（草地公用工具）
// 对应 C++ src/Lawn/LawnCommon.h / LawnCommon.cpp

use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::widget::edit_widget::EditWidget;
use crate::framework::widget::dialog::Dialog;
use crate::framework::widget::checkbox::{Checkbox, CheckboxListener};
use crate::framework::widget::edit_widget::EditListener;
use crate::framework::key_codes::KEYCODE_ESCAPE;
use crate::framework::rect::Rect;

// ====================================================================================================
// ★ 编辑框控件颜色表（对应 C++ gLawnEditWidgetColors）
// ====================================================================================================
pub const LAWN_EDIT_WIDGET_COLORS: [[u8; 4]; 5] = [
    [0,   0,   0,   0],
    [0,   0,   0,   0],
    [240, 240, 255, 255],
    [255, 255, 255, 255],
    [0,   0,   0,   255],
];

// ====================================================================================================
// ★ 常用逻辑判断
// ====================================================================================================

/// 判断在 [theNumber - theRange, theNumber + theRange] 区间内是否存在 theMod 的整数倍数
pub fn mod_in_range(the_number: i32, the_mod: i32, the_range: i32) -> bool {
    let range = the_range.abs();
    for i in (the_number - range)..=(the_number + range) {
        if i % the_mod == 0 {
            return true;
        }
    }
    false
}

/// 判断点 (x1, y1) 是否位于点 (x2, y2) 周围的 (theRangeX, theRangeY) 范围内
pub fn grid_in_range(x1: i32, y1: i32, x2: i32, y2: i32, the_range_x: i32, the_range_y: i32) -> bool {
    x1 >= x2 - the_range_x && x1 <= x2 + the_range_x && y1 >= y2 - the_range_y && y1 <= y2 + the_range_y
}

// ====================================================================================================
// ★ 动画、特效与绘制相关
// ====================================================================================================

/// 水平平铺图片（每次绘制图像的一部分，直到填满指定宽度）
pub fn tile_image_horizontally(g: &mut Graphics, the_image: &Image, the_x: i32, the_y: i32, the_width: i32) {
    let mut x = the_x;
    let mut width = the_width;
    while width > 0 {
        let image_width = std::cmp::min(width, the_image.get_width());
        let src = Rect::new(0, 0, image_width, the_image.get_height());
        g.draw_image_src(the_image, x, the_y, &src);
        x += image_width;
        width -= image_width;
    }
}

/// 垂直平铺图片（每次绘制图像的一部分，直到填满指定高度）
pub fn tile_image_vertically(g: &mut Graphics, the_image: &Image, the_x: i32, the_y: i32, the_height: i32) {
    let mut y = the_y;
    let mut height = the_height;
    while height > 0 {
        let image_height = std::cmp::min(height, the_image.get_height());
        let src = Rect::new(0, 0, the_image.get_width(), image_height);
        g.draw_image_src(the_image, the_x, y, &src);
        y += image_height;
        height -= image_height;
    }
}

// ====================================================================================================
// ★ LawnEditWidget 类（继承 EditWidget）
// ====================================================================================================

/// 草地编辑框控件（对应 C++ LawnEditWidget : EditWidget）
pub struct LawnEditWidget {
    pub dialog: Option<*mut Dialog>,
    pub auto_cap_first_letter: bool,
}

impl LawnEditWidget {
    pub fn new(_the_id: i32, _the_listener: Option<&mut dyn EditListener>, the_dialog: Option<*mut Dialog>) -> Self {
        LawnEditWidget {
            dialog: the_dialog,
            auto_cap_first_letter: true,
        }
    }

    pub fn key_down(&mut self, the_key: i32) {
        // EditWidget::KeyDown(theKey) 基类处理——TODO: 实际需要调用基类方法
        if the_key == KEYCODE_ESCAPE {
            // 原始 C++ 调用 mDialog->KeyDown(KEYCODE_ESCAPE)
            // 但 Dialog::key_down 通过 WidgetImpl trait 提供，暂不调用
        }
    }

    pub fn key_char(&mut self, the_char: char) {
        if self.auto_cap_first_letter && the_char.is_ascii_alphabetic() {
            self.auto_cap_first_letter = false;
        }
        // EditWidget::KeyChar(theChar) 基类处理——TODO: 实际需要调用基类方法
    }
}

// ====================================================================================================
// ★ 控件工厂函数
// ====================================================================================================

/// 创建编辑框控件（对应 C++ CreateEditWidget）
pub fn create_edit_widget(_the_id: i32, _the_listener: Option<&mut dyn EditListener>, _the_dialog: Option<*mut Dialog>) -> LawnEditWidget {
    let widget = LawnEditWidget::new(_the_id, _the_listener, _the_dialog);
    // TODO: 颜色设置 SetColors(gLawnEditWidgetColors, NUM_COLORS)
    // TODO: 设置字体 SetFont(FONT_BRIANNETOD16)
    // TODO: mBlinkDelay = 14
    widget
}

/// 绘制编辑框背景（对应 C++ DrawEditBox）
pub fn draw_edit_box(_g: &mut Graphics, the_widget: &EditWidget) {
    let _dest = Rect::new(
        the_widget.x - 8,
        the_widget.y - 4,
        the_widget.width + 16,
        the_widget.height + 8,
    );
    // TODO: g->DrawImageBox(aDest, IMAGE_EDITBOX);
}

/// 创建新复选框（对应 C++ MakeNewCheckbox）
pub fn make_new_checkbox(the_id: i32, _the_listener: Option<Box<dyn CheckboxListener>>, the_default: bool) -> Checkbox {
    // 构造时传入空指针图像，实际图像资源在运行时设置
    let mut checkbox = Checkbox::new(
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        the_id,
        None,
    );
    checkbox.checked = the_default;
    checkbox.has_alpha = true;
    checkbox.has_transparencies = true;
    checkbox
}

// ====================================================================================================
// ★ 存档路径
// ====================================================================================================

/// 获取当前存档文件名（对应 C++ GetSavedGameName）
pub fn get_saved_game_name(the_game_mode: GameMode, the_profile_id: i32) -> String {
    format!("userdata/game{}_{}.v4", the_profile_id, the_game_mode as i32)
}

/// 获取旧版存档文件名（对应 C++ GetLegacySavedGameName）
pub fn get_legacy_saved_game_name(the_game_mode: GameMode, the_profile_id: i32) -> String {
    format!("userdata/game{}_{}.dat", the_profile_id, the_game_mode as i32)
}

/// 获取当前日期距离 2000 年 1 月 1 日的天数（对应 C++ GetCurrentDaysSince2000）
pub fn get_current_days_since_2000() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let days_since_epoch = (secs / 86400) as i32;
    // 1970-01-01 到 2000-01-01 共 10957 天
    days_since_epoch - 10957
}
