// PvZ Portable Rust 翻译 — CheatDialog（作弊对话框）
// 对应 C++ src/Lawn/Widget/CheatDialog.h / CheatDialog.cpp

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::framework::graphics::font::Font;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::framework::widget::dialog::BUTTONS_OK_CANCEL;
use crate::framework::widget::edit_widget::{EditListener, EditWidget, WidthCheck};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::{Dialogs, GameMode};
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;

// C++ 构造函数使用的对话框 ID
const DIALOG_CHEAT: i32 = Dialogs::Cheat as i32;

// C++ GameConstants.h 常量
const LEVELS_PER_AREA: i32 = 10;
// C++ NUM_CHALLENGE_MODES = NUM_GAME_MODES - 1
// [TRANSLATION_NOTE]: Rust GameMode 枚举项数需运行时验证；此处用 C++ 数值。
const NUM_CHALLENGE_MODES: i32 = 72;

/// 弱引用编辑框监听器（对应 C++ CreateEditWidget(0, this, this)）
struct WeakEditListener {
    host: std::rc::Weak<RefCell<CheatDialog>>,
}

impl EditListener for WeakEditListener {
    fn edit_widget_text(&mut self, the_id: i32, the_string: &str) {
        if let Some(h) = self.host.upgrade() {
            h.borrow_mut().edit_widget_text(the_id, the_string);
        }
    }

    fn allow_char(&mut self, the_id: i32, c: u8) -> bool {
        if let Some(h) = self.host.upgrade() {
            h.borrow().allow_char(the_id, c)
        } else {
            false
        }
    }
}

/// 作弊对话框 — 输入关卡编号直接跳关
/// [TRANSLATION_NOTE]: C++ CheatDialog 继承 LawnDialog + EditListener；
/// Rust 采用组合（首字段为 LawnDialog），通过 Deref/DerefMut 转发 LawnDialog 字段
/// （`self.x` / `self.width` / `self.height` 等调用点保持不变）。
pub struct CheatDialog {
    /// LawnDialog 基类字段（组合实现 C++ 继承）
    pub dialog: LawnDialog,
    /// 关卡输入编辑框（对应 C++ mLevelEditWidget）
    pub level_edit_widget: Option<*mut EditWidget>,
    /// 是否可见（保留旧字段名，兼容 LawnApp 调用点）
    pub visible: bool,
}

impl std::ops::Deref for CheatDialog {
    type Target = LawnDialog;
    fn deref(&self) -> &LawnDialog {
        &self.dialog
    }
}

impl std::ops::DerefMut for CheatDialog {
    fn deref_mut(&mut self) -> &mut LawnDialog {
        &mut self.dialog
    }
}

impl CheatDialog {
    /// 构造（对应 C++ CheatDialog 构造函数，C cheatDialog.cpp:37-63）
    pub fn new(app: Option<*mut LawnApp>) -> Self {
        // C++: LawnDialog(theApp, Dialogs::DIALOG_CHEAT, true, "CHEAT", "Enter New Level:", "",
        //       Dialog::BUTTONS_OK_CANCEL)
        let mut d = LawnDialog::new();
        d.app = app;
        d.id = DIALOG_CHEAT;
        d.is_modal = true;
        d.dialog_header = "CHEAT".to_string();
        d.dialog_lines = "Enter New Level:".to_string();
        d.dialog_footer = String::new();
        // C++: mVerticalCenterText = false
        d.vertical_center_text = false;
        // C++: Dialog::BUTTONS_OK_CANCEL = 2
        let _ = BUTTONS_OK_CANCEL;

        // 先构造 EditWidget（不依赖 wrapper，此时无 listener 回指；listener 用 Weak 到外部 Rc）
        // C++: mLevelEditWidget = CreateEditWidget(0, this, this)
        //      mLevelEditWidget->mMaxChars = 12
        //      mLevelEditWidget->AddWidthCheckFont(FONT_BRIANNETOD12, 220)
        let weak_host_slot: std::rc::Weak<RefCell<CheatDialog>>;
        let wrapper: Rc<RefCell<CheatDialog>> = Rc::new(RefCell::new(CheatDialog {
            dialog: d,
            level_edit_widget: None,
            visible: true,
        }));
        weak_host_slot = Rc::downgrade(&wrapper);
        let listener: Box<dyn EditListener> = Box::new(WeakEditListener { host: weak_host_slot });

        let mut ew = EditWidget::new(0, Some(listener));
        // C++: SetFont(FONT_BRIANNETOD12)
        ew.set_font(&Font::new("BRIANNETOD12", 12));
        // C++: mLevelEditWidget->mMaxChars = 12
        ew.max_chars = 12;
        // C++: mLevelEditWidget->AddWidthCheckFont(FONT_BRIANNETOD12, 220)
        ew.width_check_list.push(WidthCheck::new(Font::new("BRIANNETOD12", 12), 220));

        // 挂入 wrapper 的 level_edit_widget 字段
        wrapper.borrow_mut().level_edit_widget = Some(Box::into_raw(Box::new(ew)));

        // C++ 构造函数第 45-58 行：按当前 game_mode 设置初始字符串
        // ```cpp
        // std::string aCheatStr;
        // if (mApp->mGameMode != GameMode::GAMEMODE_ADVENTURE) {
        //     aCheatStr = StrFormat("C%d", static_cast<int>(mApp->mGameMode));
        // }
        // else if (mApp->HasFinishedAdventure()) {
        //     aCheatStr = StrFormat("F%s", mApp->GetStageString(mApp->mPlayerInfo->GetLevel()).c_str());
        // }
        // else {
        //     aCheatStr = mApp->GetStageString(mApp->mPlayerInfo->GetLevel());
        // }
        // mLevelEditWidget->SetText(aCheatStr, true);
        // ```
        {
            let a_cheat_str = unsafe {
                app.map_or(String::new(), |p| {
                    let app_ref = &*p;
                    if app_ref.game_mode != GameMode::Adventure {
                        // C++: StrFormat("C%d", static_cast<int>(mApp->mGameMode))
                        format!("C{}", app_ref.game_mode as i32)
                    } else if app_ref.has_finished_adventure() {
                        // C++: StrFormat("F%s", GetStageString(mPlayerInfo->GetLevel()))
                        let level = app_ref.player_info.as_ref().map(|pi| pi.get_level()).unwrap_or(0);
                        let stage = LawnApp::get_stage_string(level);
                        format!("F{}", stage)
                    } else {
                        // C++: mApp->GetStageString(mPlayerInfo->GetLevel())
                        let level = app_ref.player_info.as_ref().map(|pi| pi.get_level()).unwrap_or(0);
                        LawnApp::get_stage_string(level)
                    }
                })
            };
            let mut h = wrapper.borrow_mut();
            if let Some(ew) = h.level_edit_widget {
                unsafe {
                    let ew_ref = &mut *ew;
                    // C++: mLevelEditWidget->SetText(aCheatStr, true)
                    ew_ref.set_text(&a_cheat_str, true);
                }
            }
        }

        // C++: CalcSize(110, 40)
        wrapper.borrow_mut().dialog.calc_size(110, 40);

        // 回收 wrapper 所有权（wrapper 是本函数创建的 Rc，此处唯一强引用）
        let wrapper_cell = match Rc::try_unwrap(wrapper) {
            Ok(inner) => inner,
            Err(_) => unreachable!("wrapper Rc 在本函数中应有唯一强引用"),
        };
        wrapper_cell.into_inner()
    }

    /// 计算对话框尺寸（对应 C++ CheatDialog::GetPreferredHeight）
    /// C++: return LawnDialog::GetPreferredHeight(theWidth);  （无 +40 加成）
    pub fn get_preferred_height(&self, the_width: i32) -> i32 {
        let ci = self.content_insets;
        let bi = self.background_insets;
        let mut a_height = ci.top + ci.bottom + bi.top + bi.bottom;

        let mut need_space = false;
        if !self.dialog_header.is_empty() {
            if let Some(font) = &self.header_font {
                a_height += font.get_height() - font.get_ascent_padding();
            }
            need_space = true;
        }

        if !self.dialog_lines.is_empty() {
            if need_space {
                a_height += self.space_after_header;
            }
            if let Some(font) = &self.lines_font {
                let avail = the_width - ci.left - ci.right - bi.left - bi.right - 4;
                let line_spacing = font.line_spacing + self.line_spacing_offset;
                let approx_chars_per_line = if avail > 0 {
                    (avail / 8).max(1)
                } else {
                    1
                };
                let approx_lines = (self.dialog_lines.len() as i32 / approx_chars_per_line).max(1);
                a_height += approx_lines * line_spacing;
            }
            need_space = true;
        }

        a_height + self.button_height
    }

    /// 设置对话框尺寸并同步编辑框位置（对应 C++ CheatDialog::Resize）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void CheatDialog::Resize(int theX, int theY, int theWidth, int theHeight)
    /// {
    ///     LawnDialog::Resize(theX, theY, theWidth, theHeight);
    ///     mLevelEditWidget->Resize(mContentInsets.mLeft + 12, mHeight - 155,
    ///                              mWidth - mContentInsets.mLeft - mContentInsets.mRight - 24, 28);
    /// }
    /// ```
    pub fn resize(&mut self, the_x: i32, the_y: i32, the_width: i32, the_height: i32) {
        LawnDialog::resize(&mut self.dialog, the_x, the_y, the_width, the_height);
        if let Some(ew) = self.level_edit_widget {
            unsafe {
                let ew_ref = &mut *ew;
                let left = self.content_insets.left + 12;
                let top = self.height - 155;
                let w = self.width - self.content_insets.left - self.content_insets.right - 24;
                let h = 28;
                ew_ref.resize(left, top, w, h);
            }
        }
    }

    /// 挂载到 WidgetManager 时挂上编辑框并设置焦点（对应 C++ AddedToManager）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void CheatDialog::AddedToManager(WidgetManager* theWidgetManager)
    /// {
    ///     LawnDialog::AddedToManager(theWidgetManager);
    ///     AddWidget(mLevelEditWidget);
    ///     theWidgetManager->SetFocus(mLevelEditWidget);
    /// }
    /// ```
    pub fn added_to_manager(&mut self, the_widget_manager: &mut WidgetManager) {
        LawnDialog::added_to_manager(&mut self.dialog, the_widget_manager);
        if let Some(ew) = self.level_edit_widget {
            unsafe {
                let w_ptr = &mut (*ew).widget as *mut crate::framework::widget::widget::Widget;
                the_widget_manager.add_widget(w_ptr);
                the_widget_manager.set_focus(Some(w_ptr));
            }
        }
    }

    /// 从 WidgetManager 移除时摘掉编辑框（对应 C++ RemovedFromManager）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void CheatDialog::RemovedFromManager(WidgetManager* theWidgetManager)
    /// {
    ///     LawnDialog::RemovedFromManager(theWidgetManager);
    ///     RemoveWidget(mLevelEditWidget);
    /// }
    /// ```
    pub fn removed_from_manager(&mut self, the_widget_manager: &mut WidgetManager) {
        LawnDialog::removed_from_manager(&mut self.dialog, the_widget_manager);
        if let Some(ew) = self.level_edit_widget {
            unsafe {
                let w_ptr = &mut (*ew).widget as *mut crate::framework::widget::widget::Widget;
                the_widget_manager.remove_widget(w_ptr);
            }
        }
    }

    /// 绘制对话框与编辑框背景（对应 C++ CheatDialog::Draw）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void CheatDialog::Draw(Graphics* g)
    /// {
    ///     LawnDialog::Draw(g);
    ///     DrawEditBox(g, mLevelEditWidget);
    /// }
    /// ```
    pub fn draw(&self, g: &mut Graphics) {
        LawnDialog::draw(&self.dialog, g);
        if let Some(ew) = self.level_edit_widget {
            unsafe {
                crate::lawn::lawn_common::draw_edit_box(g, &*ew);
            }
        }
    }

    /// 更新（对应 C++ CheatDialog::Update → LawnDialog::Update）
    pub fn update(&mut self) {
        LawnDialog::update(&mut self.dialog);
    }

    /// 键盘事件（对应 C++ CheatDialog::KeyDown → LawnDialog::KeyDown）
    pub fn key_down(&mut self, key: KeyCode) {
        LawnDialog::key_down(&mut self.dialog, key);
    }

    /// 鼠标按下（对应 C++ CheatDialog::MouseDown → LawnDialog::MouseDown 未实现，保持空体）
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {
        let _ = &self.dialog;
    }

    /// 编辑框文本变更回调（对应 C++ CheatDialog::EditWidgetText）
    ///
    /// C++: `mApp->ButtonDepress(mId + 2000);`
    /// [TRANSLATION_NOTE]: LawnApp::ButtonDepress 未实现（SexyAppBase 处为空体），保留语义注释。
    pub fn edit_widget_text(&mut self, _the_id: i32, _the_string: &str) {
        // C++: mApp->ButtonDepress(mId + 2000)
        // [TRANSLATION_NOTE]: LawnApp::ButtonDepress 已接入（commit 见 plan_step_11）
        if let Some(app) = self.app {
            unsafe {
                let app_ref = &mut *app;
                app_ref.button_depress(self.id + 2000);
            }
        }
    }

    /// 允许输入的字符（对应 C++ CheatDialog::AllowChar）
    /// C++: `return isdigit(theChar) || theChar == '-' || theChar == 'c' || theChar == 'C' ||
    ///      theChar == 'f' || theChar == 'F';`
    pub fn allow_char(&self, _the_id: i32, c: u8) -> bool {
        let ch = c as char;
        ch.is_ascii_digit() || ch == '-' || ch == 'c' || ch == 'C' || ch == 'f' || ch == 'F'
    }

    /// 尝试解析前缀 + 数字（对应 sscanf 匹配）
    fn parse_prefixed_number<'a>(input: &'a str, prefix: char) -> Option<i32> {
        if let Some(rest) = input.strip_prefix(prefix) {
            let rest_trim = rest.trim_start();
            // sscanf 允许前导空白，剩余部分整体 parse
            if let Ok(n) = rest_trim.parse::<i32>() {
                return Some(n);
            }
            // sscanf 也允许尾部有非数字（例如 "f3-x"）——C++ sscanf("f%d") 会读到 3
            if let Some(n) = rest_trim.split(|c: char| !c.is_ascii_digit()).next().and_then(|s| s.parse::<i32>().ok()) {
                return Some(n);
            }
        }
        None
    }

    /// 尝试解析前缀 + "%d-%d"（对应 sscanf("f%d-%d") 等）
    fn parse_prefixed_area_sub<'a>(input: &'a str, prefix: char) -> Option<(i32, i32)> {
        if let Some(rest) = input.strip_prefix(prefix) {
            let rest_trim = rest.trim_start();
            let dash_pos = rest_trim.find('-')?;
            let a = rest_trim[..dash_pos].trim();
            let b = rest_trim[dash_pos + 1..].trim();
            // 匹配数字（ sscanf 语义：允许尾部非数字）
            let a_num = a.split(|c: char| !c.is_ascii_digit()).next()?;
            let b_num = b.split(|c: char| !c.is_ascii_digit()).next()?;
            if let (Ok(an), Ok(bn)) = (a_num.parse::<i32>(), b_num.parse::<i32>()) {
                return Some((an, bn));
            }
        }
        None
    }

    /// 尝试解析纯数字（对应 sscanf("%d") 等）
    fn parse_plain_number<'a>(input: &'a str) -> Option<i32> {
        let s = input.trim_start();
        let s = s.split(|c: char| !c.is_ascii_digit()).next()?;
        s.parse::<i32>().ok()
    }

    /// 尝试解析 "%d-%d"（对应 sscanf("%d-%d")）
    fn parse_area_sub<'a>(input: &'a str) -> Option<(i32, i32)> {
        let s = input.trim_start();
        let dash_pos = s.find('-')?;
        let a = s[..dash_pos].trim();
        let b = s[dash_pos + 1..].trim();
        let a_num = a.split(|c: char| !c.is_ascii_digit()).next()?;
        let b_num = b.split(|c: char| !c.is_ascii_digit()).next()?;
        if let (Ok(an), Ok(bn)) = (a_num.parse::<i32>(), b_num.parse::<i32>()) {
            Some((an, bn))
        } else {
            None
        }
    }

    /// 应用作弊码（对应 C++ ApplyCheat，CheatDialog.cpp:110-165）
    ///
    /// C++ 使用 sscanf 顺序解析：
    /// 1. `c%d` / `C%d` → 挑战模式索引
    /// 2. `f%d-%d` / `F%d-%d` → 完成冒险 + 关卡
    /// 3. `f%d` / `F%d` → 完成冒险 + 关卡
    /// 4. `%d-%d` → 关卡
    /// 5. `%d` → 关卡
    pub fn apply_cheat(&mut self) -> bool {
        let Some(app) = self.app else { return false };
        let input = self.level_edit_widget.map_or(String::new(), |pw| unsafe { (*pw).text.clone() });
        let input = input.trim();

        // 1. C++: sscanf(mString, "c%d", &aChallengeIndex) == 1 || sscanf(..., "C%d", ...) == 1
        if let Some(idx) = Self::parse_prefixed_number(input, 'c').or_else(|| Self::parse_prefixed_number(input, 'C')) {
            unsafe {
                // C++: mApp->mGameMode = (GameMode)std::clamp(aChallengeIndex, 0, NUM_CHALLENGE_MODES);
                let clamped = idx.clamp(0, NUM_CHALLENGE_MODES);
                (*app).game_mode = std::mem::transmute::<i32, GameMode>(clamped);
            }
            return true;
        }

        let mut a_level: i32 = -1;
        let mut a_finished_adventure: i32 = 0;

        // 2. C++: sscanf(mString, "f%d-%d", &aArea, &aSubArea) == 2 || sscanf(..., "F%d-%d", ...) == 2
        if let Some((area, sub)) = Self::parse_prefixed_area_sub(input, 'f').or_else(|| Self::parse_prefixed_area_sub(input, 'F')) {
            a_level = (area - 1) * LEVELS_PER_AREA + sub;
            a_finished_adventure = 1;
        }
        // 3. C++: sscanf(mString, "f%d", &aLevel) == 1 || sscanf(..., "F%d", &aLevel) == 1
        else if let Some(level) = Self::parse_prefixed_number(input, 'f').or_else(|| Self::parse_prefixed_number(input, 'F')) {
            a_level = level;
            a_finished_adventure = 1;
        }
        // 4. C++: sscanf(mString, "%d-%d", &aArea, &aSubArea) == 2
        else if let Some((area, sub)) = Self::parse_area_sub(input) {
            a_level = (area - 1) * LEVELS_PER_AREA + sub;
        }
        // 5. C++: else sscanf(mString, "%d", &aLevel)
        else if let Some(level) = Self::parse_plain_number(input) {
            a_level = level;
        }

        if a_level <= 0 {
            unsafe {
                (*app).do_dialog(
                    Dialogs::CheatError as i32,
                    true,
                    "Enter Level",
                    "Invalid Level. Do 'number' or 'area-subarea' or 'Cnumber' or 'Farea-subarea'.",
                    "[DIALOG_BUTTON_OK]",
                    crate::framework::widget::dialog::BUTTONS_FOOTER,
                );
            }
            return false;
        }

        unsafe {
            // C++: mApp->mGameMode = GameMode::GAMEMODE_ADVENTURE;
            (*app).game_mode = GameMode::Adventure;
            // C++: mApp->mPlayerInfo->SetLevel(aLevel);
            // C++: mApp->mPlayerInfo->mFinishedAdventure = aFinishedAdventure;
            if let Some(pi) = (*app).player_info.as_mut() {
                pi.set_level(a_level);
                pi.m_finished_adventure = a_finished_adventure;
            }
            // C++: mApp->WriteCurrentUserConfig();
            (*app).write_current_user_config();
        }
        true
    }

    /// 释放内部编辑框（对应 C++ ~CheatDialog 的 delete mLevelEditWidget）
    pub fn clear_level_edit_widget(&mut self) {
        if let Some(pw) = self.level_edit_widget.take() {
            unsafe { drop(Box::from_raw(pw)); }
        }
    }
}

impl Drop for CheatDialog {
    fn drop(&mut self) {
        // C++: ~CheatDialog() 的 delete mLevelEditWidget
        self.clear_level_edit_widget();
    }
}

impl Default for CheatDialog {
    fn default() -> Self {
        CheatDialog::new(None)
    }
}
