// PvZ Portable Rust 翻译 — LawnCommon（草地公用工具）
// 对应 C++ src/Lawn/LawnCommon.h / LawnCommon.cpp

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::key_codes::KEYCODE_ESCAPE;
use crate::framework::rect::Rect;
use crate::framework::widget::checkbox::{Checkbox, CheckboxListener};
use crate::framework::widget::dialog::Dialog;
use crate::framework::widget::edit_widget::EditListener;
use crate::framework::widget::edit_widget::EditWidget;
use crate::lawn::game_enums::*;

// ====================================================================================================
// ★ 编辑框控件颜色表（对应 C++ gLawnEditWidgetColors）
// ====================================================================================================
#[allow(dead_code)]
// NOT-WIRED: C++ gLawnEditWidgetColors（LawnCommon.cpp:35）由 CreateEditWidget 的 SetColors 使用；Rust create_edit_widget 的颜色设置尚未实装（TODO），待接线后引用
pub const LAWN_EDIT_WIDGET_COLORS: [[u8; 4]; 5] = [
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [240, 240, 255, 255],
    [255, 255, 255, 255],
    [0, 0, 0, 255],
];

// ====================================================================================================
// ★ 常用逻辑判断
// ====================================================================================================

/// 判断在 [theNumber - theRange, theNumber + theRange] 区间内是否存在 theMod 的整数倍数
#[allow(dead_code)]
// NOT-WIRED: C++ ModInRange（LawnCommon.cpp:44）全 cpp/ 仅定义/声明、无调用点（UNUSED_CPP_USAGE.md §2）
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
#[allow(dead_code)]
// NOT-WIRED: 与 board::grid_in_range（board.rs 内的同名实现，已接线且对应 C++ GridInRange 三个调用点）重复；本函数为早期拆分遗留（UNUSED_CPP_USAGE.md §3）
pub fn grid_in_range(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    the_range_x: i32,
    the_range_y: i32,
) -> bool {
    x1 >= x2 - the_range_x
        && x1 <= x2 + the_range_x
        && y1 >= y2 - the_range_y
        && y1 <= y2 + the_range_y
}

// ====================================================================================================
// ★ 动画、特效与绘制相关
// ====================================================================================================

/// 水平平铺图片（每次绘制图像的一部分，直到填满指定宽度）
#[allow(dead_code)]
// NOT-WIRED: C++ TileImageHorizontally（LawnCommon.cpp:58）全 cpp/ 仅定义/声明、无调用点（§2）
pub fn tile_image_horizontally(
    g: &mut Graphics,
    the_image: &Image,
    the_x: i32,
    the_y: i32,
    the_width: i32,
) {
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
#[allow(dead_code)]
// NOT-WIRED: C++ TileImageVertically（LawnCommon.cpp:69）全 cpp/ 仅定义/声明、无调用点（§2）
pub fn tile_image_vertically(
    g: &mut Graphics,
    the_image: &Image,
    the_x: i32,
    the_y: i32,
    the_height: i32,
) {
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
#[allow(dead_code)]
// NOT-WIRED: C++ LawnEditWidget : EditWidget（LawnCommon.cpp:80）；Rust 侧 NewUserDialog/CheatDialog 直接使用 EditWidget（未继承桥），本类与工厂待重构接线
pub struct LawnEditWidget {
    pub dialog: Option<*mut Dialog>,
    pub auto_cap_first_letter: bool,
}

impl LawnEditWidget {
    #[allow(dead_code)]
    // NOT-WIRED: C++ CreateEditWidget 构造 LawnEditWidget；Rust 侧对话框直接使用 EditWidget，本构造未接线
    pub fn new(
        _the_id: i32,
        _the_listener: Option<&mut dyn EditListener>,
        the_dialog: Option<*mut Dialog>,
    ) -> Self {
        LawnEditWidget {
            dialog: the_dialog,
            auto_cap_first_letter: true,
        }
    }

    /// KeyDown 的 ESC 转发段（对应 C++ LawnEditWidget::KeyDown，LawnCommon.cpp:90-95：
    /// EditWidget::KeyDown(theKey) 基类段由调用方先调，本方法做 mDialog->KeyDown 转发）
    pub fn key_down_forward_escape(&mut self, the_key: i32) {
        if the_key == KEYCODE_ESCAPE {
            if let Some(a_dialog) = self.dialog {
                unsafe {
                    (*a_dialog).dialog_key_down(the_key);
                }
            }
        }
    }

    /// KeyChar 的首字母大写段（对应 C++ LawnEditWidget::KeyChar，LawnCommon.cpp:109-115）：
    /// AutoCapChar 原地改写首个可大写字母后停止自动大写；返回改写后的字符，
    /// 基类 EditWidget::KeyChar 由调用方以返回值继续调用（Rust 无继承）
    pub fn key_char(&mut self, the_char: char) -> char {
        let mut a_char = the_char;
        if self.auto_cap_first_letter && auto_cap_char(&mut a_char) {
            self.auto_cap_first_letter = false;
        }
        a_char
    }

    /// KeyText 的首字母大写段（对应 C++ LawnEditWidget::KeyText，LawnCommon.cpp:117-136）：
    /// 未启用自动大写时原文直通（基类 EditWidget::KeyText 由调用方继续）；
    /// 启用时对整段做 AutoCapChar，返回改写后的文本
    pub fn key_text(&mut self, the_text: &str) -> String {
        if !self.auto_cap_first_letter {
            // C++ 119-123: 未启用自动大写，原文直通（基类 EditWidget::KeyText 由调用方继续）
            return the_text.to_string();
        }

        // C++ 125-133: for (char& aCh : aText) { if (AutoCapChar(aCh)) { mAutoCapFirstLetter = false; break; } }
        let mut a_text = the_text.to_string();
        for a_byte in unsafe { a_text.as_bytes_mut() } {
            if auto_cap_char_byte(a_byte) {
                self.auto_cap_first_letter = false;
                break;
            }
        }
        a_text
    }
}

/// C++ LawnCommon.cpp:98-107 AutoCapChar：小写 a-z 原地转大写返回 true；大写 A-Z 返回 true；其余 false
fn auto_cap_char(the_char: &mut char) -> bool {
    if (*the_char as u8) >= b'a' && (*the_char as u8) <= b'z' {
        *the_char = ((*the_char as u8) - b'a' + b'A') as char;
        return true;
    }
    (*the_char as u8) >= b'A' && (*the_char as u8) <= b'Z'
}

/// 字节版 AutoCapChar（对 String 就地改写；locale 无关，ASCII 安全）
fn auto_cap_char_byte(the_char: &mut u8) -> bool {
    if *the_char >= b'a' && *the_char <= b'z' {
        *the_char = *the_char - b'a' + b'A';
        return true;
    }
    *the_char >= b'A' && *the_char <= b'Z'
}

// ====================================================================================================
// ★ 控件工厂函数
// ====================================================================================================

/// 创建编辑框控件（对应 C++ CreateEditWidget）
#[allow(dead_code)]
// NOT-WIRED: C++ CreateEditWidget（LawnCommon.cpp:138）调用点为 CheatDialog.cpp:36 / NewUserDialog.cpp:40；Rust 两处直接构造 EditWidget，待 LawnEditWidget 继承链接线
pub fn create_edit_widget(
    _the_id: i32,
    _the_listener: Option<&mut dyn EditListener>,
    _the_dialog: Option<*mut Dialog>,
) -> LawnEditWidget {
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

/// 创建新复选框（对应 C++ MakeNewCheckbox，LawnCommon.cpp:154-162）
/// C++ 直接使用全局 Sexy::IMAGE_OPTIONS_CHECKBOX0/1 资源句柄；Rust 无资源全局变量，
/// 经 LawnApp 的 ResourceManager 按同名 key 解析（与 CursorObject::Draw 等既有取图方式一致）。
pub fn make_new_checkbox(
    the_id: i32,
    the_listener: Option<Box<dyn CheckboxListener>>,
    the_default: bool,
) -> Checkbox {
    let (a_image0, a_image1) = crate::lawn::lawn_app::LawnApp::instance().map_or(
        (std::ptr::null_mut(), std::ptr::null_mut()),
        |app| {
            let get = |a_key: &str| -> *mut Image {
                app.base
                    .resource_manager
                    .map_or(std::ptr::null_mut(), |rm| unsafe {
                        (*rm).get_image(a_key).as_image_ptr()
                    })
            };
            (
                get("IMAGE_OPTIONS_CHECKBOX0"),
                get("IMAGE_OPTIONS_CHECKBOX1"),
            )
        },
    );
    let mut checkbox = Checkbox::new(a_image0, a_image1, the_id, the_listener);
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
    format!(
        "userdata/game{}_{}.v4",
        the_profile_id, the_game_mode as i32
    )
}

/// 获取旧版存档文件名（对应 C++ GetLegacySavedGameName）
pub fn get_legacy_saved_game_name(the_game_mode: GameMode, the_profile_id: i32) -> String {
    format!(
        "userdata/game{}_{}.dat",
        the_profile_id, the_game_mode as i32
    )
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
