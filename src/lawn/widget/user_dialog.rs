// PvZ Portable Rust 翻译 — UserDialog（用户管理对话框）
// 对应 C++ src/Lawn/Widget/UserDialog.h / UserDialog.cpp

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::framework::color::Color;
use crate::framework::graphics::font::Font;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::framework::widget::button_listener::ButtonListener;
use crate::framework::widget::dialog::{BUTTONS_OK_CANCEL, ID_FOOTER};
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::Dialogs;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;

pub const USER_DIALOG_RENAME_USER: i32 = 0;
pub const USER_DIALOG_DELETE_USER: i32 = 1;

// C++ gUserListWidgetColors（UserDialog.cpp:37-43）
const USER_LIST_WIDGET_COLORS: [[u8; 3]; 5] = [
    [23,  24,  35],
    [0,   0,   0],
    [235, 225, 180],
    [255, 255, 255],
    [20,  180, 15],
];

// C++ 构造函数使用的对话框 ID
const DIALOG_USERDIALOG: i32 = Dialogs::UserDialog as i32;

/// ListWidget 布局标志（对应 C++ LayoutFlags 位掩码）
pub const LAY_SAME_LEFT: i32 = 1 << 0;
pub const LAY_SAME_TOP: i32 = 1 << 1;
pub const LAY_SAME_RIGHT: i32 = 1 << 2;
pub const LAY_SAME_BOTTOM: i32 = 1 << 3;
pub const LAY_SAME_WIDTH: i32 = 1 << 4;
pub const LAY_SAME_HEIGHT: i32 = 1 << 5;
pub const LAY_ABOVE: i32 = 1 << 6;
pub const LAY_BELOW: i32 = 1 << 7;
pub const LAY_LEFT: i32 = 1 << 8;
pub const LAY_RIGHT: i32 = 1 << 9;

/// 列表部件（对应 C++ ListWidget）
/// [TRANSLATION_NOTE]: C++ ListWidget 是完整 Widget 子类；Rust framework/widget/list_widget.rs
/// 未实现全部接口（SetColors/Justify/ItemHeight/Layout 等）。此处以独立简化版承载 UserDialog 所需功能。
pub struct ListWidget {
    // 位置尺寸
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
    /// 颜色数组（对应 C++ mColors）
    pub colors: Vec<Color>,
    /// 行文本（对应 C++ 行数据）
    pub lines: Vec<String>,
    /// 当前选中行索引（对应 mSelectIdx）
    pub select_index: i32,
    /// 是否绘制边框（对应 C++ mDrawOutline）
    pub draw_outline: bool,
    /// 文字对齐方式（对应 C++ mJustify；JUSTIFY_CENTER = 1）
    pub justify: i32,
    /// 每行高度（对应 C++ mItemHeight）
    pub item_height: i32,
    /// 字体（对应 C++ mFont）
    pub font: Option<Box<Font>>,
}

impl ListWidget {
    pub fn new() -> Self {
        ListWidget {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible: true,
            colors: Vec::new(),
            lines: Vec::new(),
            select_index: 0,
            draw_outline: false,
            justify: 0,
            item_height: 20,
            font: None,
        }
    }

    /// 移除一行（对应 C++ RemoveLine）
    pub fn remove_line(&mut self, index: i32) {
        if index >= 0 && (index as usize) < self.lines.len() {
            self.lines.remove(index as usize);
        }
    }

    /// 设置选中行（对应 C++ SetSelect）
    pub fn set_select(&mut self, index: i32) {
        self.select_index = index.clamp(0, (self.lines.len() as i32).saturating_sub(1));
    }

    /// 获取行数（对应 C++ GetLineCount）
    pub fn get_line_count(&self) -> i32 {
        self.lines.len() as i32
    }

    /// 添加一行（对应 C++ AddLine(text, append_at_bottom=false) → 返回新行索引）
    pub fn add_line(&mut self, text: &str) -> i32 {
        self.lines.push(text.to_string());
        (self.lines.len() as i32) - 1
    }

    /// 设置行文本（对应 C++ SetLine）
    pub fn set_line(&mut self, index: i32, text: &str) {
        if index >= 0 && (index as usize) < self.lines.len() {
            self.lines[index as usize] = text.to_string();
        }
    }

    /// 获取指定行文本（对应 C++ GetStringAt）
    pub fn get_string_at(&self, index: i32) -> String {
        if index >= 0 && (index as usize) < self.lines.len() {
            self.lines[index as usize].clone()
        } else {
            String::new()
        }
    }

    /// 设置颜色数组（对应 C++ SetColors）
    pub fn set_colors(&mut self, colors: &[[u8; 3]]) {
        self.colors = colors.iter().map(|c| Color::from_rgb(c[0], c[1], c[2])).collect();
    }

    /// 布局（对应 C++ ListWidget::Layout——简化版，仅记录 flags 与参考 Widget 的相对位置）
    /// [TRANSLATION_NOTE]: C++ ListWidget::Layout 基于 flags 位运算计算目标 x/y/w/h；
    /// 此处简化为将 self 与 target 对齐到同一位置。
    pub fn layout(&mut self, _flags: i32, target: &DialogButton, _dx: i32, _dy: i32, _dx2: i32, _dy2: i32) {
        // 简化：直接对齐到 target 的 x/y/w/h
        self.x = target.widget.x;
        self.y = target.widget.y;
        self.width = target.widget.width;
        self.height = target.widget.height;
    }

    /// 设置尺寸（对应 C++ ListWidget::Resize）
    pub fn resize(&mut self, the_x: i32, the_y: i32, the_w: i32, the_h: i32) {
        self.x = the_x;
        self.y = the_y;
        self.width = the_w;
        self.height = the_h;
    }
}

impl Default for ListWidget {
    fn default() -> Self {
        ListWidget::new()
    }
}

/// 弱引用按钮监听器（对应 C++ MakeButton(id, this, ...) 中 this 是 UserDialog）
struct WeakButtonListener {
    host: std::rc::Weak<RefCell<UserDialog>>,
}

impl ButtonListener for WeakButtonListener {
    fn button_press(&mut self, the_id: i32) {
        if let Some(h) = self.host.upgrade() {
            h.borrow_mut().button_press(the_id);
        }
    }

    fn button_depress(&mut self, the_id: i32) {
        if let Some(h) = self.host.upgrade() {
            h.borrow_mut().button_depress(the_id);
        }
    }

    fn button_down_tick(&mut self, the_id: i32) { let _ = the_id; }
    fn button_mouse_enter(&mut self, the_id: i32) { let _ = the_id; }
    fn button_mouse_leave(&mut self, the_id: i32) { let _ = the_id; }
    fn button_mouse_move(&mut self, the_id: i32, x: i32, y: i32) { let _ = (the_id, x, y); }
}

/// 用户管理对话框 — 重命名/删除用户
/// [TRANSLATION_NOTE]: C++ UserDialog 继承 LawnDialog + ListListener + EditListener；
/// Rust 采用组合（首字段为 LawnDialog），通过 Deref/DerefMut 转发 LawnDialog 字段
/// （`self.x` / `self.width` / `self.get_preferred_height` 等调用点保持不变）。
/// LawnApp::do_user_dialog 中的 `a_dialog.x` / `a_dialog.width` / `a_dialog.height` 通过 Deref 兼容。
pub struct UserDialog {
    /// LawnDialog 基类字段（组合实现 C++ 继承）
    pub dialog: LawnDialog,
    /// 用户列表（对应 C++ mUserList）
    pub user_list: Option<*mut ListWidget>,
    /// 重命名按钮（对应 C++ mRenameButton）
    pub rename_button: Option<*mut DialogButton>,
    /// 删除按钮（对应 C++ mDeleteButton）
    pub delete_button: Option<*mut DialogButton>,
    /// 用户数量（对应 C++ mNumUsers）
    pub num_users: i32,
}

impl std::ops::Deref for UserDialog {
    type Target = LawnDialog;
    fn deref(&self) -> &LawnDialog {
        &self.dialog
    }
}

impl std::ops::DerefMut for UserDialog {
    fn deref_mut(&mut self) -> &mut LawnDialog {
        &mut self.dialog
    }
}

impl UserDialog {
    /// 构造（对应 C++ UserDialog 构造函数，UserDialog.cpp:47-90）
    pub fn new(app: Option<*mut LawnApp>) -> Self {
        // C++: LawnDialog(theApp, Dialogs::DIALOG_USERDIALOG, true,
        //   theApp->GetString("WHO_ARE_YOU", "WHO ARE YOU?"),
        //   "", "", Dialog::BUTTONS_OK_CANCEL)
        let mut d = LawnDialog::new();
        d.app = app;
        d.id = DIALOG_USERDIALOG;
        d.is_modal = true;
        // C++: theApp->GetString("WHO_ARE_YOU", "WHO ARE YOU?")
        // [TRANSLATION_NOTE]: LawnApp::GetString 未实现，用 C++ fallback 文案。
        d.dialog_header = "WHO ARE YOU?".to_string();
        // C++: 第 5、6 参为空字符串（无正文、无 footer）
        d.dialog_lines = String::new();
        d.dialog_footer = String::new();
        // C++: mVerticalCenterText = false
        d.vertical_center_text = false;
        // C++: Dialog::BUTTONS_OK_CANCEL = 2 → LawnDialog 构造函数自动创建
        // mLawnYesButton = MakeButton(1000, this, "Ok") / mLawnNoButton = MakeButton(1001, this, "Cancel")
        // [TRANSLATION_NOTE]: Rust LawnDialog 未自动创建；此处显式创建以对齐 C++ Resize 中
        // mLawnYesButton / mLawnNoButton 的布局参考。
        d.is_modal = true;
        // [TRANSLATION_NOTE]: BUTTONS_OK_CANCEL 值保留供后续 LawnApp::ButtonDepress 路由参考
        let _ = BUTTONS_OK_CANCEL;

        // 建立 wrapper（Weak 需要强引用宿主存在）
        let wrapper: Rc<RefCell<UserDialog>> = Rc::new(RefCell::new(UserDialog {
            dialog: d,
            user_list: None,
            rename_button: None,
            delete_button: None,
            num_users: 0,
        }));

        // 1. 创建用户列表（对应 C++ mUserList = new ListWidget(0, FONT_BRIANNETOD16, this)）
        {
            let mut lw = ListWidget::new();
            // C++: mUserList->SetColors(gUserListWidgetColors, LENGTH(...))
            lw.set_colors(&USER_LIST_WIDGET_COLORS);
            // C++: mUserList->mDrawOutline = true
            lw.draw_outline = true;
            // C++: mUserList->mJustify = ListWidget::JUSTIFY_CENTER
            // [TRANSLATION_NOTE]: C++ JUSTIFY_CENTER 常量未在 Rust 定义；此处用占位 1。
            lw.justify = 1;
            // C++: mUserList->mItemHeight = 24
            lw.item_height = 24;
            // C++: FONT_BRIANNETOD16
            lw.font = Some(Box::new(Font::new("BRIANNETOD16", 16)));
            wrapper.borrow_mut().user_list = Some(Box::into_raw(Box::new(lw)));
        }

        // 2. 创建 mLawnYesButton / mLawnNoButton（BUTTONS_OK_CANCEL → 对应 C++ LawnDialog 构造函数自动生成）
        {
            let weak = Rc::downgrade(&wrapper);
            let listener: Box<dyn ButtonListener> = Box::new(WeakButtonListener { host: weak });
            let mut yes_btn = DialogButton::new(std::ptr::null_mut(), ID_FOOTER, Some(listener));
            yes_btn.label = "Ok".to_string();
            yes_btn.has_alpha = true;
            yes_btn.has_transparencies = true;
            wrapper.borrow_mut().dialog.lawn_yes_button = Some(Box::into_raw(Box::new(yes_btn)));
        }
        {
            let weak = Rc::downgrade(&wrapper);
            let listener: Box<dyn ButtonListener> = Box::new(WeakButtonListener { host: weak });
            let mut no_btn = DialogButton::new(std::ptr::null_mut(), crate::framework::widget::dialog::ID_NO, Some(listener));
            no_btn.label = "Cancel".to_string();
            no_btn.has_alpha = true;
            no_btn.has_transparencies = true;
            wrapper.borrow_mut().dialog.lawn_no_button = Some(Box::into_raw(Box::new(no_btn)));
        }

        // 3. 创建 Rename 按钮（C++ MakeButton(UserDialog_RenameUser, this, "Rename")）
        {
            let weak = Rc::downgrade(&wrapper);
            let listener: Box<dyn ButtonListener> = Box::new(WeakButtonListener { host: weak });
            let mut db = DialogButton::new(std::ptr::null_mut(), USER_DIALOG_RENAME_USER, Some(listener));
            db.label = "Rename".to_string();
            db.has_alpha = true;
            db.has_transparencies = true;
            wrapper.borrow_mut().rename_button = Some(Box::into_raw(Box::new(db)));
        }

        // 4. 创建 Delete 按钮（C++ MakeButton(UserDialog_DeleteUser, this, "Delete")）
        {
            let weak = Rc::downgrade(&wrapper);
            let listener: Box<dyn ButtonListener> = Box::new(WeakButtonListener { host: weak });
            let mut db = DialogButton::new(std::ptr::null_mut(), USER_DIALOG_DELETE_USER, Some(listener));
            db.label = "Delete".to_string();
            db.has_alpha = true;
            db.has_transparencies = true;
            wrapper.borrow_mut().delete_button = Some(Box::into_raw(Box::new(db)));
        }

        // 5. 填充用户列表（对应 C++ UserDialog 构造函数第 64-82 行）
        {
            let mut h = wrapper.borrow_mut();
            h.num_users = 0;
            // C++: if (theApp->mPlayerInfo) { mUserList->SetSelect(mUserList->AddLine(theApp->mPlayerInfo->mName, false)); mNumUsers++; }
            let cur_name = unsafe {
                app.and_then(|p| (*p).player_info.as_ref().map(|pi| pi.name.clone()))
            };
            if let Some(cur) = cur_name {
                let idx = unsafe { (*h.user_list.unwrap()).add_line(&cur) };
                unsafe { (*h.user_list.unwrap()).set_select(idx); }
                h.num_users += 1;
            }

            // C++: for (const_iterator : aMap) { if (mPlayerInfo && anItr->second.mName == mPlayerInfo->mName) continue;
            //     mUserList->AddLine(anItr->second.mName, false); mNumUsers++; }
            // [TRANSLATION_NOTE]: Rust ProfileMgr 内部结构未暴露迭代接口；此处保留循环占位注释。
            // 后续 ProfileMgr 支持 ProfileMap 迭代时补入。
        }

        // 6. C++: if (mNumUsers < 8) { mUserList->AddLine("(Create a New User)", false); }
        {
            let mut h = wrapper.borrow_mut();
            if h.num_users < 8 {
                unsafe { (*h.user_list.unwrap()).add_line("(Create a New User)"); }
            }
            // C++: mTallBottom = true
            h.dialog.tall_bottom = true;
            // C++: CalcSize(210, 270)
            h.dialog.calc_size(210, 270);
        }

        // 回收 wrapper 所有权
        let wrapper_cell = match Rc::try_unwrap(wrapper) {
            Ok(inner) => inner,
            Err(_) => unreachable!("wrapper Rc 在本函数中应有唯一强引用"),
        };
        wrapper_cell.into_inner()
    }

    /// 计算对话框尺寸（对应 C++ UserDialog::GetPreferredHeight）
    /// C++: return LawnDialog::GetPreferredHeight(theWidth) + 190;
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

        a_height += self.button_height;
        a_height + 190
    }

    /// 设置对话框尺寸并同步子部件位置（对应 C++ UserDialog::Resize）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void UserDialog::Resize(int theX, int theY, int theWidth, int theHeight)
    /// {
    ///     LawnDialog::Resize(theX, theY, theWidth, theHeight);
    ///     mUserList->Resize(GetLeft() + 30, GetTop() + 4, GetWidth() - 60, 200);
    ///     mRenameButton->Layout(LAY_SameLeft | LAY_Above | LAY_SameHeight | LAY_SameWidth, mLawnYesButton, 0, 0, 0, 0);
    ///     mDeleteButton->Layout(LAY_SameLeft | LAY_Above | LAY_SameHeight | LAY_SameWidth, mLawnNoButton, 0, 0, 0, 0);
    /// }
    /// ```
    pub fn resize(&mut self, the_x: i32, the_y: i32, the_width: i32, the_height: i32) {
        LawnDialog::resize(&mut self.dialog, the_x, the_y, the_width, the_height);
        // C++: mUserList->Resize(GetLeft() + 30, GetTop() + 4, GetWidth() - 60, 200);
        if let Some(list) = self.user_list {
            unsafe {
                let lw = &mut *list;
                let left = self.get_left() + 30;
                let top = self.get_top() + 4;
                let w = self.get_width() - 60;
                let h = 200;
                lw.resize(left, top, w, h);
            }
        }
        // C++: mRenameButton->Layout(..., mLawnYesButton, ...)
        if let Some(rb) = self.rename_button {
            if let Some(yes_btn) = self.lawn_yes_button {
                unsafe {
                    let rb_mut = &mut *rb;
                    let yes_btn_ref = &*yes_btn;
                    // LAY_SameLeft | LAY_Above | LAY_SameHeight | LAY_SameWidth
                    let flags = LAY_SAME_LEFT | LAY_ABOVE | LAY_SAME_HEIGHT | LAY_SAME_WIDTH;
                    // Rust DialogButton 无 Layout 方法；此处按 flags 语义直接设置 x/y/w/h
                    let target_x = yes_btn_ref.widget.x;
                    let target_y = yes_btn_ref.widget.y;
                    let target_w = yes_btn_ref.widget.width;
                    let target_h = yes_btn_ref.widget.height;
                    rb_mut.widget.x = target_x;
                    rb_mut.widget.y = target_y - target_h;
                    rb_mut.widget.width = target_w;
                    rb_mut.widget.height = target_h;
                    let _ = flags;
                }
            }
        }
        // C++: mDeleteButton->Layout(..., mLawnNoButton, ...)
        if let Some(db) = self.delete_button {
            if let Some(no_btn) = self.lawn_no_button {
                unsafe {
                    let db_mut = &mut *db;
                    let no_btn_ref = &*no_btn;
                    let flags = LAY_SAME_LEFT | LAY_ABOVE | LAY_SAME_HEIGHT | LAY_SAME_WIDTH;
                    let target_x = no_btn_ref.widget.x;
                    let target_y = no_btn_ref.widget.y;
                    let target_w = no_btn_ref.widget.width;
                    let target_h = no_btn_ref.widget.height;
                    db_mut.widget.x = target_x;
                    db_mut.widget.y = target_y - target_h;
                    db_mut.widget.width = target_w;
                    db_mut.widget.height = target_h;
                    let _ = flags;
                }
            }
        }
    }

    /// 挂载到 WidgetManager 时把三个子部件挂上（对应 C++ AddedToManager）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void UserDialog::AddedToManager(WidgetManager* theWidgetManager)
    /// {
    ///     LawnDialog::AddedToManager(theWidgetManager);
    ///     AddWidget(mUserList);
    ///     AddWidget(mDeleteButton);
    ///     AddWidget(mRenameButton);
    /// }
    /// ```
    pub fn added_to_manager(&mut self, the_widget_manager: &mut WidgetManager) {
        LawnDialog::added_to_manager(&mut self.dialog, the_widget_manager);
        // [TRANSLATION_NOTE]: Rust ListWidget 简化版不含 Widget 基座，无法 add_widget；
        // 只挂接两个 DialogButton。
        if let Some(db) = self.delete_button {
            unsafe {
                the_widget_manager.add_widget((&mut *db).as_widget_ptr());
            }
        }
        if let Some(rb) = self.rename_button {
            unsafe {
                the_widget_manager.add_widget((&mut *rb).as_widget_ptr());
            }
        }
    }

    /// 从 WidgetManager 移除时摘掉两个按钮（对应 C++ RemovedFromManager）
    pub fn removed_from_manager(&mut self, the_widget_manager: &mut WidgetManager) {
        LawnDialog::removed_from_manager(&mut self.dialog, the_widget_manager);
        if let Some(db) = self.delete_button {
            unsafe {
                the_widget_manager.remove_widget((&mut *db).as_widget_ptr());
            }
        }
        if let Some(rb) = self.rename_button {
            unsafe {
                the_widget_manager.remove_widget((&mut *rb).as_widget_ptr());
            }
        }
    }

    /// 获取选中用户名（对应 C++ GetSelName）
    ///
    /// C++ 源码：
    /// ```cpp
    /// std::string UserDialog::GetSelName()
    /// {
    ///     if (mUserList->mSelectIdx < 0 || mUserList->mSelectIdx >= mNumUsers) return "";
    ///     return mUserList->GetStringAt(mUserList->mSelectIdx);
    /// }
    /// ```
    pub fn get_sel_name(&self) -> String {
        let Some(list) = self.user_list else { return String::new() };
        let sel_idx = unsafe { (*list).select_index };
        if sel_idx < 0 || sel_idx >= self.num_users {
            return String::new();
        }
        unsafe { (*list).get_string_at(sel_idx) }
    }

    /// 删除选中用户（对应 C++ FinishDeleteUser）
    pub fn finish_delete_user(&mut self) {
        let Some(list) = self.user_list else { return };
        unsafe {
            let a_sel_idx = (*list).select_index;
            (*list).remove_line(a_sel_idx);

            let a_sel_idx = (a_sel_idx - 1).max(0);
            if (*list).get_line_count() > 0 {
                (*list).set_select(a_sel_idx);
            }

            self.num_users -= 1;
            // C++: if (mNumUsers == 7) mUserList->AddLine("(Create a New User)", false);
            if self.num_users == 7 {
                (*list).add_line("(Create a New User)");
            }
        }
    }

    /// 重命名选中用户（对应 C++ FinishRenameUser）
    pub fn finish_rename_user(&mut self, new_name: &str) {
        let Some(list) = self.user_list else { return };
        unsafe {
            if (*list).select_index < self.num_users {
                (*list).set_line((*list).select_index, new_name);
            }
        }
    }

    /// 列表点击回调（对应 C++ ListClicked）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void UserDialog::ListClicked(int theId, int theIdx, int theClickCount)
    /// {
    ///     (void)theId;
    ///     if (theIdx == mNumUsers) { mApp->DoCreateUserDialog(); }
    ///     else {
    ///         mUserList->SetSelect(theIdx);
    ///         if (theClickCount == 2) mApp->FinishUserDialog(true);
    ///     }
    /// }
    /// ```
    pub fn list_clicked(&mut self, _the_id: i32, the_idx: i32, the_click_count: i32) {
        if the_idx == self.num_users {
            // C++: mApp->DoCreateUserDialog();
            if let Some(app) = self.app {
                unsafe { (*app).do_create_user_dialog(); }
            }
        } else {
            if let Some(list) = self.user_list {
                unsafe { (*list).set_select(the_idx); }
            }
            if the_click_count == 2 {
                // C++: mApp->FinishUserDialog(true);
                if let Some(app) = self.app {
                    unsafe { (*app).finish_user_dialog(true); }
                }
            }
        }
    }

    /// 绘制对话框（对应 C++ UserDialog::Draw）
    ///
    /// C++ 源码：`void UserDialog::Draw(Graphics* g) { LawnDialog::Draw(g); }`
    pub fn draw(&self, g: &mut Graphics) {
        LawnDialog::draw(&self.dialog, g);
        // [TRANSLATION_NOTE]: Rust ListWidget 简化版无 draw 方法；子部件由 WidgetManager 自动绘制。
    }

    /// 更新（对应 C++ UserDialog::Update → LawnDialog::Update）
    pub fn update(&mut self) {
        LawnDialog::update(&mut self.dialog);
    }

    /// 键盘事件（对应 C++ UserDialog::KeyDown → LawnDialog::KeyDown）
    pub fn key_down(&mut self, key: KeyCode) {
        LawnDialog::key_down(&mut self.dialog, key);
    }

    /// 鼠标按下（对应 C++ UserDialog::MouseDown → LawnDialog::MouseDown 未实现，保持空体）
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {
        let _ = &self.dialog;
    }

    /// 按钮按下（对应 C++ LawnDialog::ButtonPress，由按钮触发）
    pub fn button_press(&mut self, the_id: i32) {
        LawnDialog::button_press(&mut self.dialog, the_id);
    }

    /// 按钮点击（对应 C++ UserDialog::ButtonDepress）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void UserDialog::ButtonDepress(int theId)
    /// {
    ///     LawnDialog::ButtonDepress(theId);
    ///     std::string aSelName = GetSelName();
    ///     if (!aSelName.empty()) {
    ///         switch (theId) {
    ///         case UserDialog::UserDialog_RenameUser:
    ///             mApp->DoRenameUserDialog(aSelName); break;
    ///         case UserDialog::UserDialog_DeleteUser:
    ///             mApp->DoConfirmDeleteUserDialog(aSelName); break;
    ///         }
    ///     }
    /// }
    /// ```
    pub fn button_depress(&mut self, the_id: i32) {
        // C++: LawnDialog::ButtonDepress(theId);
        LawnDialog::button_depress(&mut self.dialog, the_id);
        let a_sel_name = self.get_sel_name();
        if a_sel_name.is_empty() {
            return;
        }
        let Some(app) = self.app else { return };
        unsafe {
            match the_id {
                USER_DIALOG_RENAME_USER => {
                    (*app).do_rename_user_dialog(&a_sel_name);
                }
                USER_DIALOG_DELETE_USER => {
                    (*app).do_confirm_delete_user_dialog(&a_sel_name);
                }
                _ => {}
            }
        }
    }

    /// 编辑框文本变更回调（对应 C++ UserDialog::EditWidgetText）
    ///
    /// C++: `mApp->ButtonDepress(mId + 2000);`
    /// [TRANSLATION_NOTE]: LawnApp::ButtonDepress 未实现（SexyAppBase 处为空体），保留语义注释。
    pub fn edit_widget_text(&mut self, _the_id: i32, _the_string: &str) {
        let _ = (self.app, self.id);
    }

    /// 允许输入的字符（对应 C++ AllowChar）
    /// C++: `return isdigit(theChar);`
    pub fn allow_char(&self, _the_id: i32, c: u8) -> bool {
        let ch = c as char;
        ch.is_ascii_digit()
    }

    /// 释放内部三个子部件（对应 C++ ~UserDialog 的 delete mUserList/mRenameButton/mDeleteButton）
    pub fn clear_widgets(&mut self) {
        if let Some(pw) = self.user_list.take() {
            unsafe { drop(Box::from_raw(pw)); }
        }
        if let Some(pw) = self.rename_button.take() {
            unsafe { drop(Box::from_raw(pw)); }
        }
        if let Some(pw) = self.delete_button.take() {
            unsafe { drop(Box::from_raw(pw)); }
        }
    }
}

impl Drop for UserDialog {
    fn drop(&mut self) {
        // C++: ~UserDialog() 的 delete mUserList / mRenameButton / mDeleteButton
        self.clear_widgets();
        // LawnDialog 构造函数显式创建的 mLawnYesButton / mLawnNoButton
        if let Some(pw) = self.dialog.lawn_yes_button.take() {
            unsafe { drop(Box::from_raw(pw)); }
        }
        if let Some(pw) = self.dialog.lawn_no_button.take() {
            unsafe { drop(Box::from_raw(pw)); }
        }
    }
}

impl Default for UserDialog {
    fn default() -> Self {
        UserDialog::new(None)
    }
}
