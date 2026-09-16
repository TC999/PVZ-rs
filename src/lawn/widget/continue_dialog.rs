// PvZ Portable Rust 翻译 — ContinueDialog（继续游戏对话框）
// 对应 C++ src/Lawn/Widget/ContinueDialog.h / ContinueDialog.cpp

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::{KeyCode, KEYCODE_ESCAPE, KEYCODE_RETURN, KEYCODE_SPACE};
use crate::framework::widget::button_listener::ButtonListener;
use crate::framework::widget::dialog::ID_FOOTER;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::{BoardResult, ChallengePage, Dialogs, GameMode};
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::todlib::tod_foley::FoleyType;

pub const CONTINUE_DIALOG_CONTINUE: i32 = 0;
pub const CONTINUE_DIALOG_NEW_GAME: i32 = 1;

// C++ 构造按钮时使用的标签 key（对应 C++ PvzpStringTranslate 中的字符串 key）
const CONTINUE_BUTTON_LABEL: &str = "[CONTINUE_BUTTON]";
const RESTART_LEVEL_LABEL: &str = "[RESTART_LEVEL]";
const NEW_GAME_BUTTON_LABEL: &str = "[NEW_GAME_BUTTON]";

/// 弱引用按钮监听器：把按钮事件转发到 ContinueDialog
/// [TRANSLATION_NOTE]: C++ `MakeButton(id, this, ...)` 中 `this` 是 LawnDialog/ContinueDialog；
/// Rust 侧通过 Weak<RefCell<ContinueDialog>> 回指，避免 DialogButton 字段与宿主形成循环引用。
struct WeakButtonListener {
    host: std::rc::Weak<RefCell<ContinueDialog>>,
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

    fn button_down_tick(&mut self, the_id: i32) {
        let _ = the_id;
    }

    fn button_mouse_enter(&mut self, the_id: i32) {
        let _ = the_id;
    }

    fn button_mouse_leave(&mut self, the_id: i32) {
        let _ = the_id;
    }

    fn button_mouse_move(&mut self, the_id: i32, x: i32, y: i32) {
        let _ = (the_id, x, y);
    }
}

/// 继续游戏对话框 — 选择继续或开新游戏
/// [TRANSLATION_NOTE]: C++ ContinueDialog 继承 LawnDialog；Rust 采用组合（首字段为 LawnDialog），
/// 并通过 Deref/DerefMut 转发 LawnDialog 字段（`self.x` / `self.width` / `self.app` 等
/// 调用点保持不变）。旧字段名 `continue_button/new_game_button/visible` 等全部通过 Deref
/// 兼容 LawnApp 调用点。
pub struct ContinueDialog {
    /// LawnDialog 基类字段（组合实现 C++ 继承）
    pub dialog: LawnDialog,
    /// 继续按钮（对应 C++ mContinueButton）
    pub continue_button: Option<*mut DialogButton>,
    /// 新游戏按钮（对应 C++ mNewGameButton）
    pub new_game_button: Option<*mut DialogButton>,
}

impl std::ops::Deref for ContinueDialog {
    type Target = LawnDialog;
    fn deref(&self) -> &LawnDialog {
        &self.dialog
    }
}

impl std::ops::DerefMut for ContinueDialog {
    fn deref_mut(&mut self) -> &mut LawnDialog {
        &mut self.dialog
    }
}

impl ContinueDialog {
    /// 构造（对应 C++ ContinueDialog 构造函数，ContinueDialog.cpp:38-91）
    pub fn new(app: Option<*mut LawnApp>) -> Self {
        // C++: LawnDialog(theApp, Dialogs::DIALOG_CONTINUE, true,
        //   theApp->GetString("CONTINUE_GAME_HEADER", "CONTINUE GAME?"),
        //   "", "[DIALOG_BUTTON_CANCEL]", Dialog::BUTTONS_FOOTER)
        let is_adventure = unsafe {
            app.map_or(false, |p| (*p).is_adventure_mode())
        };

        let mut d = LawnDialog::new();
        d.app = app;
        d.id = Dialogs::Continue as i32;
        d.is_modal = true;
        // C++: theApp->GetString("CONTINUE_GAME_HEADER", "CONTINUE GAME?")
        // 对应 SexyAppBase::get_string_default（= C++ GetString(theId, theDefault)）
        d.dialog_header = app.map_or_else(
            || "CONTINUE GAME?".to_string(),
            |app| unsafe {
                (*app)
                    .base
                    .get_string_default("CONTINUE_GAME_HEADER", "CONTINUE GAME?")
            },
        );
        // C++: 构造函数第 5 参为空字符串；mDialogLines 后续按模式设置
        d.dialog_lines = String::new();
        // C++: "[DIALOG_BUTTON_CANCEL]"
        d.dialog_footer = "[DIALOG_BUTTON_CANCEL]".to_string();
        // C++: Dialog::BUTTONS_FOOTER → LawnDialog 构造函数中 mButtonMode = 3，mLawnYesButton = MakeButton(1000, this, theDialogFooter)
        // [TRANSLATION_NOTE]: Rust LawnDialog 未模拟 C++ 中按 mButtonMode 自动创建 mLawnYesButton 分支；
        // 此处显式创建以对齐 C++ ContinueDialog::Resize 中依赖的 mLawnYesButton。
        let footer_listener: Box<dyn ButtonListener> = {
            // 先建 wrapper，然后回填
            Box::new(EmptyButtonListener)
        };
        // 占位——见下方 wrapper 建立后回填。
        let _ = footer_listener;

        // [TRANSLATION_NOTE]: 因 LawnDialog 在 Rust 侧不自动创建按钮，需自建一个 footer 按钮
        // 挂在 m_lawn_yes_button 位置，供 ContinueDialog::Resize 使用（C++ 依赖它做位置参考）。
        // 具体创建放到 wrapper 建立后（需要 Weak 回指宿主）。

        // C++: mTallBottom = true
        d.tall_bottom = true;
        // C++: CalcSize(10, 60)
        d.calc_size(10, 60);

        // 建立 wrapper（Weak 需要强引用宿主存在）
        let wrapper: Rc<RefCell<ContinueDialog>> = Rc::new(RefCell::new(ContinueDialog {
            dialog: d,
            continue_button: None,
            new_game_button: None,
        }));

        // 1. 创建 footer 按钮（对应 C++ LawnDialog 构造函数 mButtonMode==3 分支生成的 mLawnYesButton）
        //    ContinueDialog::Resize 依赖 mLawnYesButton 作为位置参考，必须显式创建。
        {
            let weak = Rc::downgrade(&wrapper);
            let listener: Box<dyn ButtonListener> = Box::new(WeakButtonListener { host: weak });
            let mut db = DialogButton::new(std::ptr::null_mut(), ID_FOOTER, Some(listener));
            db.label = "[DIALOG_BUTTON_CANCEL]".to_string();
            db.has_alpha = true;
            db.has_transparencies = true;
            wrapper.borrow_mut().dialog.lawn_yes_button = Some(Box::into_raw(Box::new(db)));
        }

        // 2. 创建 Continue 按钮（C++ MakeButton(ContinueDialog_Continue, this, "[CONTINUE_BUTTON]")）
        {
            let weak = Rc::downgrade(&wrapper);
            let listener: Box<dyn ButtonListener> = Box::new(WeakButtonListener { host: weak });
            let mut db = DialogButton::new(std::ptr::null_mut(), CONTINUE_DIALOG_CONTINUE, Some(listener));
            db.label = CONTINUE_BUTTON_LABEL.to_string();
            db.has_alpha = true;
            db.has_transparencies = true;
            wrapper.borrow_mut().continue_button = Some(Box::into_raw(Box::new(db)));
        }

        // 3. 创建 New Game 按钮（C++ MakeButton(ContinueDialog_NewGame, this,
        //    IsAdventureMode ? "[RESTART_LEVEL]" : "[NEW_GAME_BUTTON]")）
        {
            let weak = Rc::downgrade(&wrapper);
            let listener: Box<dyn ButtonListener> = Box::new(WeakButtonListener { host: weak });
            let mut db = DialogButton::new(std::ptr::null_mut(), CONTINUE_DIALOG_NEW_GAME, Some(listener));
            let label = if is_adventure {
                RESTART_LEVEL_LABEL
            } else {
                NEW_GAME_BUTTON_LABEL
            };
            db.label = label.to_string();
            db.has_alpha = true;
            db.has_transparencies = true;
            wrapper.borrow_mut().new_game_button = Some(Box::into_raw(Box::new(db)));
        }

        // 4. mDialogLines 按模式设置
        {
            let mut h = wrapper.borrow_mut();
            h.dialog.dialog_lines = if is_adventure {
                "Do you want to continue your current game or restart the level?".to_string()
            } else {
                "Do you want to continue your current game or start a new game?".to_string()
            };
        }

        // C++ 构造函数尾部：按 IMAGE_BUTTON 尺寸重算最小宽度
        // [TRANSLATION_NOTE]: Rust 未接线 IMAGE_BUTTON_LEFT/MIDDLE/RIGHT 图片资源，
        // 无法计算 aBtnWidth/aMinCancelWidth/aSteps/aMinWidth；此处保留注释并跳过重算。
        // 若后续 IMAGE_BUTTON 图片接线，需在此处补上：
        //   let a_btn_left = IMAGE_BUTTON_LEFT.width;
        //   let a_btn_mid = IMAGE_BUTTON_MIDDLE.width;
        //   let a_btn_right = IMAGE_BUTTON_RIGHT.width;
        //   let a_btn_width = a_btn_left + a_btn_mid * 3 + a_btn_right;
        //   let a_inset_h = ci.left + ci.right + bi.left + bi.right;
        //   let a_min_cancel_width = 2 * a_btn_width - 40;
        //   let a_steps = (a_min_cancel_width - a_btn_left - a_btn_right + a_btn_mid - 1) / a_btn_mid;
        //   let a_min_width = a_btn_left + a_btn_right + a_steps * a_btn_mid + a_inset_h - 8;
        //   if m_width < a_min_width { ... 重算并 Resize ... }

        // 回收 wrapper 所有权（wrapper 是本函数创建的 Rc，此处唯一强引用）
        let wrapper_cell = match Rc::try_unwrap(wrapper) {
            Ok(inner) => inner,
            Err(_) => unreachable!("wrapper Rc 在本函数中应有唯一强引用"),
        };
        wrapper_cell.into_inner()
    }

    /// 计算对话框尺寸（对应 C++ ContinueDialog::GetPreferredHeight）
    /// C++: return LawnDialog::GetPreferredHeight(theWidth) + 40;
    /// [TRANSLATION_NOTE]: Rust LawnDialog 无 GetPreferredHeight 实现；此处与 new_user_dialog
    /// 使用同一套 C++ Dialog::GetPreferredHeight 展开公式 + 40。
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
        a_height + 40
    }

    /// 设置对话框尺寸并同步子按钮位置（对应 C++ ContinueDialog::Resize）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void ContinueDialog::Resize(int theX, int theY, int theWidth, int theHeight)
    /// {
    ///     LawnDialog::Resize(theX, theY, theWidth, theHeight);
    ///     int aBtnWidth = IMAGE_BUTTON_LEFT->mWidth + IMAGE_BUTTON_MIDDLE->mWidth * 3 + IMAGE_BUTTON_RIGHT->mWidth;
    ///     int aBtnHeight = mLawnYesButton->mHeight;
    ///     mContinueButton->Resize(mLawnYesButton->mX - 20, mLawnYesButton->mY - aBtnHeight, aBtnWidth, aBtnHeight);
    ///     mNewGameButton->Resize(mLawnYesButton->mX + mLawnYesButton->mWidth - aBtnWidth + 20,
    ///                            mContinueButton->mY, aBtnWidth, aBtnHeight);
    /// }
    /// ```
    ///
    /// [TRANSLATION_NOTE]: IMAGE_BUTTON_LEFT/MIDDLE/RIGHT 图片资源未接线，aBtnWidth 使用占位
    /// 数值（4×18+36+36=144，对应 Rust LawnDialog::get_image 中使用的 dialog 组件图尺寸近似）。
    pub fn resize(&mut self, the_x: i32, the_y: i32, the_width: i32, the_height: i32) {
        LawnDialog::resize(&mut self.dialog, the_x, the_y, the_width, the_height);

        // 对应 C++ ContinueDialog::Resize:
        // aBtnWidth = IMAGE_BUTTON_LEFT->mWidth + IMAGE_BUTTON_MIDDLE->mWidth*3 + IMAGE_BUTTON_RIGHT->mWidth
        // 图片未接入时回退到原来的占位尺寸（36 / 18 / 36）
        let a_btn_left_w = self
            .dialog
            .get_image("IMAGE_BUTTON_LEFT")
            .map_or(36, |img| img.get_width());
        let a_btn_mid_w = self
            .dialog
            .get_image("IMAGE_BUTTON_MIDDLE")
            .map_or(18, |img| img.get_width());
        let a_btn_right_w = self
            .dialog
            .get_image("IMAGE_BUTTON_RIGHT")
            .map_or(36, |img| img.get_width());
        let a_btn_width = a_btn_left_w + a_btn_mid_w * 3 + a_btn_right_w;

        // C++: int aBtnHeight = mLawnYesButton->mHeight;
        let a_btn_height = if let Some(yes_btn) = self.lawn_yes_button {
            unsafe {
                let yes_btn_ref = &*yes_btn;
                yes_btn_ref.widget.height
            }
        } else {
            return; // C++ 中无 yes button 则此分支不会被执行（BUTTONS_FOOTER 保证创建）
        };

        if let Some(yes_btn) = self.lawn_yes_button {
            let yes_btn_ref = unsafe { &*yes_btn };
            let yes_x = yes_btn_ref.widget.x;
            let yes_y = yes_btn_ref.widget.y;
            let yes_w = yes_btn_ref.widget.width;

            if let Some(cbtn) = self.continue_button {
                unsafe {
                    let cb = &mut *cbtn;
                    cb.resize(yes_x - 20, yes_y - a_btn_height, a_btn_width, a_btn_height);
                }
            }

            if let Some(nbtn) = self.new_game_button {
                unsafe {
                    let nb = &mut *nbtn;
                    // C++: mLawnYesButton->mX + mLawnYesButton->mWidth - aBtnWidth + 20
                    let nx = yes_x + yes_w - a_btn_width + 20;
                    // C++: mContinueButton->mY（等于上方 yes_y - a_btn_height）
                    let ny = yes_y - a_btn_height;
                    nb.resize(nx, ny, a_btn_width, a_btn_height);
                }
            }
        }
    }

    /// 挂载到 WidgetManager 时把两个子按钮挂上（对应 C++ AddedToManager）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void ContinueDialog::AddedToManager(WidgetManager* theWidgetManager)
    /// {
    ///     LawnDialog::AddedToManager(theWidgetManager);
    ///     AddWidget(mContinueButton);
    ///     AddWidget(mNewGameButton);
    /// }
    /// ```
    pub fn added_to_manager(&mut self, the_widget_manager: &mut WidgetManager) {
        LawnDialog::added_to_manager(&mut self.dialog, the_widget_manager);
        if let Some(cb) = self.continue_button {
            unsafe {
                the_widget_manager.add_widget((&mut *cb).as_widget_ptr());
            }
        }
        if let Some(nb) = self.new_game_button {
            unsafe {
                the_widget_manager.add_widget((&mut *nb).as_widget_ptr());
            }
        }
    }

    /// 从 WidgetManager 移除时摘掉两个子按钮（对应 C++ RemovedFromManager）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void ContinueDialog::RemovedFromManager(WidgetManager* theWidgetManager)
    /// {
    ///     LawnDialog::RemovedFromManager(theWidgetManager);
    ///     RemoveWidget(mContinueButton);
    ///     RemoveWidget(mNewGameButton);
    /// }
    /// ```
    pub fn removed_from_manager(&mut self, the_widget_manager: &mut WidgetManager) {
        LawnDialog::removed_from_manager(&mut self.dialog, the_widget_manager);
        if let Some(cb) = self.continue_button {
            unsafe {
                the_widget_manager.remove_widget((&mut *cb).as_widget_ptr());
            }
        }
        if let Some(nb) = self.new_game_button {
            unsafe {
                the_widget_manager.remove_widget((&mut *nb).as_widget_ptr());
            }
        }
    }

    /// 绘制对话框与两个按钮（对应 C++ ContinueDialog::Draw → LawnDialog::Draw + WidgetManager 自动绘制子按钮）
    ///
    /// C++ LawnDialog::Draw 已经包含子 Widget 的绘制（通过 Widget 树继承），子按钮由 WidgetManager 自动绘制。
    pub fn draw(&mut self, g: &mut Graphics) {
        LawnDialog::draw(&self.dialog, g);
        // 手动绘制两个子按钮（因 C++ 中 Widget 树绘制依赖 manager；Rust 侧 LawnDialog::draw 不遍历子 widget）
        if let Some(cb) = self.continue_button {
            unsafe {
                (&mut *cb).draw(g);
            }
        }
        if let Some(nb) = self.new_game_button {
            unsafe {
                (&mut *nb).draw(g);
            }
        }
    }

    /// 更新（对应 C++ ContinueDialog::Update → LawnDialog::Update）
    pub fn update(&mut self) {
        LawnDialog::update(&mut self.dialog);
    }

    /// 键盘事件（对应 C++ ContinueDialog::KeyDown）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void ContinueDialog::KeyDown(KeyCode theKey)
    /// {
    ///     if (theKey == KeyCode::KEYCODE_ESCAPE) {
    ///         ButtonDepress(Dialog::ID_FOOTER);
    ///         return;
    ///     }
    ///     if (theKey == KeyCode::KEYCODE_RETURN || theKey == KeyCode::KEYCODE_SPACE) {
    ///         ButtonDepress(ContinueDialog::ContinueDialog_Continue);
    ///         return;
    ///     }
    ///     LawnDialog::KeyDown(theKey);
    /// }
    /// ```
    pub fn key_down(&mut self, key: KeyCode) {
        if key == KEYCODE_ESCAPE {
            self.button_depress(ID_FOOTER);
            return;
        }
        if key == KEYCODE_RETURN || key == KEYCODE_SPACE {
            self.button_depress(CONTINUE_DIALOG_CONTINUE);
            return;
        }
        LawnDialog::key_down(&mut self.dialog, key);
    }

    /// 鼠标按下（对应 C++ ContinueDialog::MouseDown → LawnDialog::MouseDown）
    /// C++ LawnDialog::MouseDown 未实现（继承自 Dialog::MouseDown 拖动分支），保持空体。
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _btn: i32) {
        let _ = &self.dialog;
    }

    /// 按钮按下（对应 C++ LawnDialog::ButtonPress，由按钮触发）
    pub fn button_press(&mut self, the_id: i32) {
        LawnDialog::button_press(&mut self.dialog, the_id);
    }

    /// 按钮点击（对应 C++ ContinueDialog::ButtonDepress）
    ///
    /// C++ 源码见 ContinueDialog.cpp:154-204。此处 1:1 翻译。
    pub fn button_depress(&mut self, the_id: i32) {
        let Some(app) = self.app else { return };
        unsafe {
            if the_id == CONTINUE_DIALOG_CONTINUE {
                // C++: RestartLoopingSounds(); mApp->KillDialog(mId);
                self.restart_looping_sounds();
                (*app).kill_dialog(Dialogs::Continue);
            } else if the_id == CONTINUE_DIALOG_NEW_GAME {
                // C++: mApp->DoDialog(Dialogs::DIALOG_RESTARTCONFIRM, ...) → aDialog->mLawnYesButton->mLabel = ...
                if (*app).is_adventure_mode() {
                    // C++: theApp->GetString 未实现，使用 fallback
                    let dlg = (*app).do_dialog(
                        Dialogs::RestartConfirm as i32,
                        true,
                        "[RESTART_LEVEL_HEADER]",
                        "[RESTART_LEVEL_BODY]",
                        "",
                        crate::framework::widget::dialog::BUTTONS_OK_CANCEL,
                    );
                    // [TRANSLATION_NOTE]: C++ 中 aDialog 是 LawnDialog* 并通过 mLawnYesButton->mLabel 修改按钮文字；
                    // Rust LawnApp::do_dialog 返回 Option<*mut Dialog>，无 LawnDialog 侧的 yes_button 字段，
                    // 无法执行 aDialog->mLawnYesButton->mLabel = PvzpStringTranslate("[RESTART_BUTTON]")。
                    let _ = dlg;
                } else {
                    let dlg = (*app).do_dialog(
                        Dialogs::RestartConfirm as i32,
                        true,
                        "New Game?",
                        "Are you sure that you want to start a new game?",
                        "",
                        crate::framework::widget::dialog::BUTTONS_OK_CANCEL,
                    );
                    // [TRANSLATION_NOTE]: 同上，无法设置 mLawnYesButton->mLabel
                    let _ = dlg;
                }
            } else {
                // C++: mApp->KillDialog(mId); mApp->mBoardResult = BOARDRESULT_QUIT;
                (*app).kill_dialog(Dialogs::Continue);
                (*app).board_result = BoardResult::Quit;
                if (*app).is_adventure_mode() {
                    // C++: mApp->ShowGameSelector();
                    (*app).show_game_selector();
                } else if (*app).is_survival_mode() {
                    (*app).kill_board();
                    (*app).show_challenge_screen(ChallengePage::Survival as i32);
                } else if (*app).is_puzzle_mode() {
                    (*app).kill_board();
                    (*app).show_challenge_screen(ChallengePage::Puzzle as i32);
                } else {
                    (*app).kill_board();
                    (*app).show_challenge_screen(ChallengePage::Challenge as i32);
                }
            }
        }
    }

    /// 重新启动循环音效（对应 C++ RestartLoopingSounds）
    ///
    /// C++ 源码：
    /// ```cpp
    /// void ContinueDialog::RestartLoopingSounds()
    /// {
    ///     if (mApp->mGameMode == GameMode::GAMEMODE_CHALLENGE_RAINING_SEEDS || mApp->IsStormyNightLevel()) {
    ///         mApp->PlayFoley(FoleyType::FOLEY_RAIN);
    ///     }
    ///     for (Zombie* aZombie : mApp->mBoard->mZombies) {
    ///         if (aZombie->mDead) continue;
    ///         if (aZombie->mPlayingSong) aZombie->StartZombieSound();
    ///     }
    /// }
    /// ```
    pub fn restart_looping_sounds(&mut self) {
        let Some(app) = self.app else { return };
        unsafe {
            let app_ref = &mut *app;
            if app_ref.game_mode == GameMode::ChallengeRainingSeeds
                || app_ref.is_stormy_night_level()
            {
                app_ref.play_foley(FoleyType::Rain as i32);
            }
            if let Some(board) = app_ref.board.as_mut() {
                for zombie in &mut (**board).zombies {
                    if zombie.dead {
                        continue;
                    }
                    if zombie.playing_song {
                        zombie.start_zombie_sound();
                    }
                }
            }
        }
    }

    /// 释放内部两个子按钮（对应 C++ ~ContinueDialog 的 delete mContinueButton/mNewGameButton）
    /// [TRANSLATION_NOTE]: Option<*mut T> 不会自动释放，调用方须在 drop ContinueDialog 前调用此方法。
    pub fn clear_buttons(&mut self) {
        if let Some(pw) = self.continue_button.take() {
            unsafe {
                drop(Box::from_raw(pw));
            }
        }
        if let Some(pw) = self.new_game_button.take() {
            unsafe {
                drop(Box::from_raw(pw));
            }
        }
    }
}

impl Drop for ContinueDialog {
    fn drop(&mut self) {
        // C++: ~ContinueDialog() 中的 delete mContinueButton / delete mNewGameButton
        self.clear_buttons();
        // lawn_yes_button（footer 按钮，C++ LawnDialog 构造时创建）也需清理
        if let Some(pw) = self.dialog.lawn_yes_button.take() {
            unsafe {
                drop(Box::from_raw(pw));
            }
        }
    }
}

/// 空按钮监听器（用于 LawnDialog 构造函数阶段的占位；实际 listener 在 wrapper 建立后替换）
struct EmptyButtonListener;

impl ButtonListener for EmptyButtonListener {
    fn button_press(&mut self, _the_id: i32) {}
    fn button_depress(&mut self, _the_id: i32) {}
    fn button_down_tick(&mut self, _the_id: i32) {}
    fn button_mouse_enter(&mut self, _the_id: i32) {}
    fn button_mouse_leave(&mut self, _the_id: i32) {}
    fn button_mouse_move(&mut self, _the_id: i32, _x: i32, _y: i32) {}
}

impl Default for ContinueDialog {
    fn default() -> Self {
        ContinueDialog::new(None)
    }
}
