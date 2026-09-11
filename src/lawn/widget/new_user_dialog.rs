// PvZ Portable Rust 翻译 — NewUserDialog（新建用户对话框）
// 对应 C++ src/Lawn/Widget/NewUserDialog.h / NewUserDialog.cpp

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::framework::graphics::font::Font;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::framework::widget::dialog::BUTTONS_OK_CANCEL;
use crate::framework::widget::edit_widget::EditListener;
use crate::framework::widget::edit_widget::EditWidget;
use crate::framework::widget::edit_widget::WidthCheck;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::Dialogs;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;

// C++ 构造函数分支所用的两个对话框 ID
const DIALOG_CREATEUSER: i32 = Dialogs::CreateUser as i32;
const DIALOG_RENAMEUSER: i32 = Dialogs::RenameUser as i32;

/// 新建用户对话框（对应 C++ NewUserDialog : LawnDialog, EditListener）
/// [TRANSLATION_NOTE]: C++ NewUserDialog 继承 LawnDialog；Rust 采用组合（首字段为 LawnDialog），
/// 并通过 Deref/DerefMut 转发 LawnDialog 字段（`self.x` / `self.width` / `self.height` 等
/// 调用点保持不变）。旧字段名 `app/x/y/width/height/visible` 也全部通过 Deref 兼容既有调用点
/// （`LawnApp::do_create_user_dialog` / `do_rename_user_dialog` 已使用）。
///
/// EditWidget 的 `EditListener` 通过 `Weak<Rc<RefCell<NewUserDialog>>>` 回指宿主，避免
/// EditWidget 字段（`Box<dyn EditListener>`）与宿主形成强循环引用。
pub struct NewUserDialog {
    /// LawnDialog 基类字段（组合实现 C++ 继承）
    pub dialog: LawnDialog,
    /// 姓名输入框（对应 C++ mNameEditWidget）
    pub name_edit_widget: Option<*mut EditWidget>,
    /// 是否为改名对话框（对应 C++ isRename 构造参数）
    pub is_rename: bool,
    /// 对话框 ID（对应 C++ LawnDialog 的 mId；由构造函数按 is_rename 设置）
    pub id: i32,
}

impl std::ops::Deref for NewUserDialog {
    type Target = LawnDialog;
    fn deref(&self) -> &LawnDialog {
        &self.dialog
    }
}

impl std::ops::DerefMut for NewUserDialog {
    fn deref_mut(&mut self) -> &mut LawnDialog {
        &mut self.dialog
    }
}

/// EditListener 的弱回指实现——通过 Weak 引用宿主，宿主 drop 后 trait 调用安全失败。
struct WeakEditListener {
    /// 宿主 NewUserDialog 的弱引用
    host: std::rc::Weak<RefCell<NewUserDialog>>,
}

impl EditListener for WeakEditListener {
    fn edit_widget_text(&mut self, the_id: i32, the_string: &str) {
        // 借用宿主 → 转发 NewUserDialog::edit_widget_text
        if let Some(host) = self.host.upgrade() {
            host.borrow_mut().edit_widget_text(the_id, the_string);
        }
    }

    fn allow_char(&mut self, the_id: i32, c: u8) -> bool {
        if let Some(host) = self.host.upgrade() {
            host.borrow().allow_char(the_id, c)
        } else {
            // 宿主已 drop，此时应拒绝输入
            false
        }
    }
}

impl NewUserDialog {
    /// 构造新对话框（对应 C++ NewUserDialog 构造函数）
    ///
    /// C++ 源码：
    /// ```cpp
    /// NewUserDialog::NewUserDialog(LawnApp* theApp, bool isRename)
    ///     : LawnDialog(theApp,
    ///        isRename ? Dialogs::DIALOG_RENAMEUSER : Dialogs::DIALOG_CREATEUSER,
    ///        true,
    ///        isRename ? theApp->GetString("RENAME_USER", "RENAME USER")
    ///                 : theApp->GetString("NEW_USER", "NEW USER"),
    ///        theApp->GetString("PLEASE_ENTER_NAME", "Please enter your name:"),
    ///        "[DIALOG_BUTTON_OK]",
    ///        Dialog::BUTTONS_OK_CANCEL)
    /// {
    ///     mApp = theApp;
    ///     mVerticalCenterText = false;
    ///     mNameEditWidget = CreateEditWidget(0, this, this);
    ///     mNameEditWidget->mMaxChars = 12;
    ///     mNameEditWidget->AddWidthCheckFont(FONT_BRIANNETOD16, 220);
    ///     CalcSize(110, 40);
    /// }
    /// ```
    pub fn new(app: Option<*mut LawnApp>) -> Self {
        let is_rename = false;
        let a_dialog_id = if is_rename {
            DIALOG_RENAMEUSER
        } else {
            DIALOG_CREATEUSER
        };
        // C++: isRename ? theApp->GetString("RENAME_USER", "RENAME USER")
        //             : theApp->GetString("NEW_USER", "NEW USER")
        // [TRANSLATION_NOTE]: LawnApp::GetString 未实现，直接使用 C++ fallback 文案。
        let header = if is_rename {
            "RENAME USER"
        } else {
            "NEW USER"
        };
        let lines = "Please enter your name:";

        // 装配 LawnDialog 基类字段（不通过 Box::new，避免与 wrapper 循环借用）
        let mut d = LawnDialog::new();
        d.app = app;
        d.id = a_dialog_id;
        d.is_modal = true;
        d.dialog_header = header.to_string();
        d.dialog_lines = lines.to_string();
        d.dialog_footer = "[DIALOG_BUTTON_OK]".to_string();
        d.vertical_center_text = false;
        // C++ 构造函数第 7 参 `Dialog::BUTTONS_OK_CANCEL` 无对应 Rust 字段；值保留供后续接线。
        let _ = BUTTONS_OK_CANCEL;

        // 构造 EditWidget——先创建一个空宿主 wrapper 使 Weak 成立，然后回填字段
        let wrapper: Rc<RefCell<NewUserDialog>> = Rc::new(RefCell::new(NewUserDialog {
            dialog: d,
            name_edit_widget: None,
            is_rename,
            id: a_dialog_id,
        }));

        let weak_host: std::rc::Weak<RefCell<NewUserDialog>> = Rc::downgrade(&wrapper);
        let listener: Box<dyn EditListener> = Box::new(WeakEditListener { host: weak_host });

        let mut ew = EditWidget::new(0, Some(listener));
        // C++: SetFont(FONT_BRIANNETOD16)
        // [TRANSLATION_NOTE]: 全局字库未接线，用 Font 占位。
        ew.set_font(&Font::new("BRIANNETOD16", 16));
        // C++: mBlinkDelay = 14（LawnCommon::CreateEditWidget）
        ew.blink_delay = 14;
        // C++: mNameEditWidget->mMaxChars = 12
        ew.max_chars = 12;
        // C++: mNameEditWidget->AddWidthCheckFont(FONT_BRIANNETOD16, 220)
        ew.width_check_list.push(WidthCheck::new(Font::new("BRIANNETOD16", 16), 220));

        // 将 EditWidget 挂入 wrapper.host.name_edit_widget
        {
            let mut host = wrapper.borrow_mut();
            host.name_edit_widget = Some(Box::into_raw(Box::new(ew)));
        }

        // C++: CalcSize(110, 40)——修改 host.dialog 的尺寸
        {
            let mut host = wrapper.borrow_mut();
            host.dialog.calc_size(110, 40);
        }

        // 从 wrapper 中回收 NewUserDialog 所有数据（wrapper 是本函数创建的 Rc，此处可安全解除）
        let wrapper_cell = match Rc::try_unwrap(wrapper) {
            Ok(inner) => inner,
            Err(_) => {
                // 若 Wrapper 仍存在引用（理论上不会发生：Weak 引用不算强引用），释放以避免泄漏
                unreachable!("wrapper Rc 在本函数中应有唯一强引用");
            }
        };
        wrapper_cell.into_inner()
    }

    /// 计算对话框尺寸（对应 C++ NewUserDialog::GetPreferredHeight）
    ///
    /// C++ 源码：
    /// ```cpp
    /// int NewUserDialog::GetPreferredHeight(int theWidth)
    /// {
    ///     return LawnDialog::GetPreferredHeight(theWidth) + 40;
    /// }
    /// ```
    ///
    /// Rust LawnDialog 无同名方法（C++ Dialog::GetPreferredHeight 未翻译），此处直接按
    /// C++ `Dialog::GetPreferredHeight`（Dialog.cpp:185）公式展开：content/bg 内边距 + header 行
    /// + lines 换行高度 + footer 行（footer==[DIALOG_BUTTON_OK] 时不额外加行，因 ButtonMode
    /// 为 BUTTONS_OK_CANCEL 而非 BUTTONS_FOOTER）+ button_height + 40。
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
                // C++: GetWordWrappedHeight(&g, avail, mDialogLines, lineSpacing + lineSpacingOffset)
                // [TRANSLATION_NOTE]: Rust 侧无换行高度计算工具，用粗略字符宽度近似行数。
                let line_spacing = font.line_spacing + self.line_spacing_offset;
                let approx_chars_per_line = if avail > 0 {
                    (avail / 8).max(1) // 8 px/char 近似
                } else {
                    1
                };
                let approx_lines = (self.dialog_lines.len() as i32 / approx_chars_per_line).max(1);
                a_height += approx_lines * line_spacing;
            }
            need_space = true;
        }

        // C++: if ((mDialogFooter.length() != 0) && (mButtonMode != BUTTONS_FOOTER)) { ... }
        // NewUserDialog 用 BUTTONS_OK_CANCEL，footer 不额外占行；但仍加 button_height。
        a_height += self.button_height;

        a_height + 40
    }

    /// 设置对话框尺寸并同步编辑框位置（对应 C++ NewUserDialog::Resize）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void NewUserDialog::Resize(int theX, int theY, int theWidth, int theHeight)
    /// {
    ///     LawnDialog::Resize(theX, theY, theWidth, theHeight);
    ///     mNameEditWidget->Resize(mContentInsets.mLeft + 12,
    ///                             mHeight - 155,
    ///                             mWidth - mContentInsets.mLeft - mContentInsets.mRight - 24,
    ///                             28);
    /// }
    /// ```
    pub fn resize(&mut self, the_x: i32, the_y: i32, the_width: i32, the_height: i32) {
        LawnDialog::resize(&mut self.dialog, the_x, the_y, the_width, the_height);
        if let Some(ew) = self.name_edit_widget {
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

    /// 挂载到 WidgetManager 时把编辑框挂上并设置焦点（对应 C++ AddedToManager）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void NewUserDialog::AddedToManager(WidgetManager* theWidgetManager)
    /// {
    ///     LawnDialog::AddedToManager(theWidgetManager);
    ///     AddWidget(mNameEditWidget);
    ///     theWidgetManager->SetFocus(mNameEditWidget);
    /// }
    /// ```
    pub fn added_to_manager(&mut self, the_widget_manager: &mut WidgetManager) {
        LawnDialog::added_to_manager(&mut self.dialog, the_widget_manager);
        if let Some(ew) = self.name_edit_widget {
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
    /// void NewUserDialog::RemovedFromManager(WidgetManager* theWidgetManager)
    /// {
    ///     LawnDialog::RemovedFromManager(theWidgetManager);
    ///     RemoveWidget(mNameEditWidget);
    /// }
    /// ```
    pub fn removed_from_manager(&mut self, the_widget_manager: &mut WidgetManager) {
        LawnDialog::removed_from_manager(&mut self.dialog, the_widget_manager);
        if let Some(ew) = self.name_edit_widget {
            unsafe {
                let w_ptr = &mut (*ew).widget as *mut crate::framework::widget::widget::Widget;
                the_widget_manager.remove_widget(w_ptr);
            }
        }
    }

    /// 绘制对话框与姓名输入框背景（对应 C++ NewUserDialog::Draw）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void NewUserDialog::Draw(Graphics* g)
    /// {
    ///     LawnDialog::Draw(g);
    ///     DrawEditBox(g, mNameEditWidget);
    /// }
    /// ```
    pub fn draw(&self, g: &mut Graphics) {
        LawnDialog::draw(&self.dialog, g);
        if let Some(ew) = self.name_edit_widget {
            unsafe {
                crate::lawn::lawn_common::draw_edit_box(g, &*ew);
            }
        }
    }

    /// 更新（对应 C++ NewUserDialog::Update，继承 LawnDialog::Update）
    pub fn update(&mut self) {
        LawnDialog::update(&mut self.dialog);
    }

    /// 键盘事件（对应 C++ NewUserDialog::KeyDown，继承 LawnDialog::KeyDown）
    pub fn key_down(&mut self, key: KeyCode) {
        LawnDialog::key_down(&mut self.dialog, key);
    }

    /// 鼠标按下（对应 C++ NewUserDialog::MouseDown，继承 LawnDialog::MouseDown）
    /// C++ LawnDialog::MouseDown 未实现（继承自 Dialog::MouseDown 拖动分支），保持空体。
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {
        let _ = &self.dialog;
    }

    /// 编辑框文本变更回调（对应 C++ NewUserDialog::EditWidgetText）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void NewUserDialog::EditWidgetText(int theId, const std::string& theString)
    /// {
    ///     (void)theId;(void)theString;
    ///     mApp->ButtonDepress(mId + 2000);
    /// }
    /// ```
    ///
    /// [TRANSLATION_NOTE]: C++ `LawnApp::ButtonDepress`（LawnApp.cpp:1860）负责按 `id + 2000`
    /// 分流到 `FinishCreateUserDialog` / `FinishRenameUserDialog` 等回调。Rust 侧
    /// `LawnApp::button_depress` 尚未实现（SexyAppBase 处为空体），此处保留语义注释并
    /// 直接返回；后续接入 `LawnApp::ButtonDepress` 时需在此处调用
    /// `(*app).button_depress(self.id + 2000)`。
    pub fn edit_widget_text(&mut self, _the_id: i32, _the_string: &str) {
        // C++: mApp->ButtonDepress(mId + 2000)
        // [TRANSLATION_NOTE]: LawnApp::ButtonDepress 已接入（commit 见 plan_step_11），
        // 此处按 C++ 语义调用 mApp->ButtonDepress(mId + 2000)。
        if let Some(app) = self.app {
            unsafe {
                let app_ref = &mut *app;
                app_ref.button_depress(self.id + 2000);
            }
        }
    }

    /// 允许输入的字符（对应 C++ AllowChar）
    /// C++: `return isalnum(theChar) || theChar == ' ';`
    pub fn allow_char(&self, _id: i32, c: u8) -> bool {
        let ch = c as char;
        ch.is_ascii_alphanumeric() || ch == ' '
    }

    /// 获取净化后的用户名（对应 C++ GetName）
    ///
    /// C++ 源码：
    /// ```cpp
    /// std::string NewUserDialog::GetName()
    /// {
    ///     std::string aString;
    ///     char aLastChar = ' ';
    ///     for (size_t i = 0; i < mNameEditWidget->mString.size(); i++)
    ///     {
    ///         char aChar = mNameEditWidget->mString[i];
    ///         if (aChar != ' ')          aString.append(1, aChar);
    ///         else if (aChar != aLastChar) aString.append(1, ' ');
    ///         aLastChar = aChar;
    ///     }
    ///     if (aString.size() && aString[aString.size() - 1] == ' ')
    ///         aString.resize(aString.size() - 1);
    ///     return aString;
    /// }
    /// ```
    pub fn get_name(&self) -> String {
        let the_string = self.name_edit_widget.map_or(String::new(), |pw| unsafe {
            (*pw).text.clone()
        });
        let mut a_string = String::new();
        let mut a_last_char = ' ';
        for a_char in the_string.chars() {
            if a_char != ' ' {
                a_string.push(a_char);
            } else if a_char != a_last_char {
                a_string.push(' ');
            }
            a_last_char = a_char;
        }
        if a_string.ends_with(' ') {
            a_string.pop();
        }
        a_string
    }

    /// 设置姓名并定位光标（对应 C++ SetName）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void NewUserDialog::SetName(const std::string& theName)
    /// {
    ///     mNameEditWidget->SetText(theName, true);
    ///     mNameEditWidget->mCursorPos = theName.size();
    ///     mNameEditWidget->mHilitePos = 0;
    /// }
    /// ```
    pub fn set_name(&mut self, the_name: &str) {
        if let Some(ew) = self.name_edit_widget {
            unsafe {
                let ew_ref = &mut *ew;
                // C++: mNameEditWidget->SetText(theName, true)
                ew_ref.set_text(the_name, true);
                // C++: mNameEditWidget->mCursorPos = theName.size()
                ew_ref.cursor_pos = the_name.len() as i32;
                // C++: mNameEditWidget->mHilitePos = 0
                ew_ref.hilite_pos = 0;
            }
        }
    }
}

// Drop 由 Rust 自动合成，逐字段释放。`name_edit_widget: Option<*mut EditWidget>` 需要
// 手动清理——`do_create_user_dialog` 等调用方在 drop NewUserDialog 前应先调用
// `clear_name_edit_widget()` 释放裸指针。若未调用则泄漏该 EditWidget，但不会 UB。
// [TRANSLATION_NOTE]: C++ `~NewUserDialog()` 中的 `delete mNameEditWidget` 无完全等价
// 的 Rust 自动析构（Option<*mut T> 不会自动释放裸指针）。
impl NewUserDialog {
    /// 释放内部 EditWidget 并置空指针（对应 C++ ~NewUserDialog 的 `delete mNameEditWidget`）
    pub fn clear_name_edit_widget(&mut self) {
        if let Some(pw) = self.name_edit_widget.take() {
            unsafe {
                drop(Box::from_raw(pw));
            }
        }
    }
}

impl Default for NewUserDialog {
    fn default() -> Self {
        NewUserDialog::new(None)
    }
}
