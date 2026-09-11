// PvZ Portable Rust 翻译 — GameOverDialog（游戏结束对话框）
// 对应 C++ src/Lawn/Widget/LawnDialog.h:94-108 / LawnDialog.cpp:451-560

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::framework::key_codes::KEYCODE_ESCAPE;
use crate::framework::widget::button_listener::ButtonListener;
use crate::framework::widget::dialog::ID_FOOTER;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::{ChallengePage, Dialogs};
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;

/// 主菜单按钮 id（对应 C++ GameOverDialog::ButtonDepress 的 theId == 1）
pub const GAME_OVER_MENU_BUTTON: i32 = 1;

/// 弱引用按钮监听器：把按钮事件转发到 GameOverDialog
/// [TRANSLATION_NOTE]: C++ `MakeButton(id, this, ...)` 的 `this` 是对话框本体；
/// Rust 侧通过 Weak<RefCell<GameOverDialog>> 回指，避免循环引用。
struct WeakButtonListener {
    host: std::rc::Weak<RefCell<GameOverDialog>>,
}

impl ButtonListener for WeakButtonListener {
    fn button_press(&mut self, the_id: i32) {
        if let Some(h) = self.host.upgrade() {
            h.borrow_mut().button_depress(the_id);
        }
    }

    fn button_depress(&mut self, the_id: i32) {
        if let Some(h) = self.host.upgrade() {
            h.borrow_mut().button_depress(the_id);
        }
    }

    fn button_down_tick(&mut self, _the_id: i32) {}

    fn button_mouse_enter(&mut self, _the_id: i32) {}

    fn button_mouse_leave(&mut self, _the_id: i32) {}

    fn button_mouse_move(&mut self, _the_id: i32, _x: i32, _y: i32) {}
}

/// 游戏结束对话框
/// [TRANSLATION_NOTE]: C++ GameOverDialog 继承 LawnDialog；Rust 采用组合（首字段为 LawnDialog），
/// 并通过 Deref/DerefMut 转发 LawnDialog 字段（`self.x` / `self.width` / `self.app` 等）。
pub struct GameOverDialog {
    /// LawnDialog 基类字段（组合实现 C++ 继承）
    pub dialog: LawnDialog,
    /// 主菜单按钮（对应 C++ mMenuButton）
    pub menu_button: Option<*mut DialogButton>,
    /// 按钮模式（C++ Dialog::BUTTONS_FOOTER）
    pub button_mode: i32,
}

impl std::ops::Deref for GameOverDialog {
    type Target = LawnDialog;
    fn deref(&self) -> &LawnDialog {
        &self.dialog
    }
}

impl std::ops::DerefMut for GameOverDialog {
    fn deref_mut(&mut self) -> &mut LawnDialog {
        &mut self.dialog
    }
}

impl GameOverDialog {
    /// 构造（对应 C++ GameOverDialog::GameOverDialog，LawnDialog.cpp:451-480）
    pub fn new(app: Option<*mut LawnApp>, the_message: &str, the_show_challenge_name: bool) -> Self {
        // C++: LawnDialog(gLawnApp, DIALOG_GAME_OVER, true, "[GAME_OVER]", theMessage, "", Dialog::BUTTONS_FOOTER)
        let mut d = LawnDialog::new();
        d.app = app;
        d.id = Dialogs::GameOver as i32;
        d.is_modal = true;
        d.dialog_header = "[GAME_OVER]".to_string();
        d.dialog_lines = the_message.to_string();
        d.dialog_footer = String::new();

        // C++: if (theShowChallengeName) mDialogHeader = PvzpStringTranslate(mApp->GetCurrentChallengeDef().mChallengeName);
        if the_show_challenge_name {
            // [TRANSLATION_NOTE]: LawnApp::get_current_challenge_def 为 stub（返回 0），挑战名暂以 key 占位
            d.dialog_header = "[CHALLENGE_NAME]".to_string();
        }
        // C++: if (theMessage.size() == 0) mContentInsets.mTop += 15;
        if the_message.is_empty() {
            d.content_insets.top += 15;
        }
        // C++: CalcSize(0, 0); mApp->CenterDialog(this, mWidth, mHeight); mClip = false;
        d.calc_size(0, 0);
        // [TRANSLATION_NOTE]: CenterDialog 需 Dialog 基类指针、mClip 属 Widget 裁剪标志；
        // Rust 对话框以 LawnApp 字段持有，不挂 Widget 树，跳过。

        let wrapper: Rc<RefCell<GameOverDialog>> = Rc::new(RefCell::new(GameOverDialog {
            dialog: d,
            menu_button: None,
            button_mode: 3, // C++ Dialog::BUTTONS_FOOTER
        }));

        // 1. footer 按钮（对应 C++ LawnDialog 构造 BUTTONS_FOOTER 分支生成的 mLawnYesButton）
        {
            let weak = Rc::downgrade(&wrapper);
            let listener: Box<dyn ButtonListener> = Box::new(WeakButtonListener { host: weak });
            let mut db = DialogButton::new(std::ptr::null_mut(), ID_FOOTER, Some(listener));
            db.label = "[TRY_AGAIN]".to_string(); // C++: mLawnYesButton->SetLabel("[TRY_AGAIN]")
            db.has_alpha = true;
            db.has_transparencies = true;
            wrapper.borrow_mut().dialog.lawn_yes_button = Some(Box::into_raw(Box::new(db)));
        }

        // 2. 主菜单按钮（C++ MakeButton(1, this, "[MAIN_MENU_BUTTON]")）
        {
            let weak = Rc::downgrade(&wrapper);
            let listener: Box<dyn ButtonListener> = Box::new(WeakButtonListener { host: weak });
            let mut db = DialogButton::new(std::ptr::null_mut(), GAME_OVER_MENU_BUTTON, Some(listener));
            db.label = "[MAIN_MENU_BUTTON]".to_string();
            db.has_alpha = true;
            db.has_transparencies = true;
            wrapper.borrow_mut().menu_button = Some(Box::into_raw(Box::new(db)));
        }

        // C++: mMenuButton->Resize(635 - mX, -10 - mY, 163, 46)
        {
            let x = wrapper.borrow().dialog.x;
            let y = wrapper.borrow().dialog.y;
            if let Some(btn) = wrapper.borrow_mut().menu_button {
                unsafe {
                    (&mut *btn).x = 635 - x;
                    (&mut *btn).y = -10 - y;
                    (&mut *btn).width = 163;
                    (&mut *btn).height = 46;
                }
            }
        }

        // C++: gLawnApp->mBoard->mShowShovel = false; gLawnApp->mBoard->mMenuButton->mBtnNoDraw = true;
        if let Some(app_ptr) = app {
            unsafe {
                if let Some(board) = (*app_ptr).board {
                    (*board).m_show_shovel = false;
                    if let Some(menu) = (*board).menu_button {
                        (*menu).btn_no_draw = true;
                    }
                }
            }
        }

        // 回收 wrapper 所有权（wrapper 是本函数创建的 Rc，此处唯一强引用）
        let wrapper_cell = match Rc::try_unwrap(wrapper) {
            Ok(inner) => inner,
            Err(_) => unreachable!("wrapper Rc 在本函数中应有唯一强引用"),
        };
        wrapper_cell.into_inner()
    }

    /// 按键（对应 C++ GameOverDialog::KeyDown，LawnDialog.cpp:487-496）
    pub fn key_down(&mut self, the_key: i32) {
        if the_key == KEYCODE_ESCAPE {
            self.button_depress(GAME_OVER_MENU_BUTTON);
            return;
        }
        // LawnDialog::KeyDown(theKey) 基类处理
        self.dialog.key_down(the_key);
    }

    /// 按钮按下（对应 C++ GameOverDialog::ButtonDepress，LawnDialog.cpp:498-526）
    pub fn button_depress(&mut self, the_id: i32) {
        let app = self.dialog.app;
        if the_id == GAME_OVER_MENU_BUTTON {
            if let Some(app) = app {
                unsafe {
                    // C++: mApp->KillDialog(Dialogs::DIALOG_GAME_OVER)
                    (*app).game_over_dialog = None;
                    // C++: mApp->KillBoard()
                    (*app).kill_board();
                    // C++: 模式分派
                    if (*app).is_survival_mode() {
                        (*app).show_challenge_screen(ChallengePage::Survival as i32);
                    } else if (*app).is_puzzle_mode() {
                        (*app).show_challenge_screen(ChallengePage::Puzzle as i32);
                    } else if (*app).is_adventure_mode() {
                        (*app).show_game_selector();
                    } else {
                        (*app).show_challenge_screen(ChallengePage::Challenge as i32);
                    }
                }
            }
        } else if the_id == ID_FOOTER {
            if let Some(app) = app {
                unsafe {
                    // C++: mApp->KillDialog(Dialogs::DIALOG_GAME_OVER); mApp->EndLevel()
                    (*app).game_over_dialog = None;
                    (*app).end_level();
                }
            }
        }
    }

    /// 鼠标拖动（对应 C++ GameOverDialog::MouseDrag，LawnDialog.cpp:546-552）
    pub fn mouse_drag(&mut self, x: i32, y: i32) {
        let _ = (x, y);
        // C++: LawnDialog::MouseDrag(x, y) 基类处理 —— [TRANSLATION_NOTE]: Rust LawnDialog 无基类 MouseDrag
        // C++: if (mMenuButton) mMenuButton->Resize(635 - mX, -10 - mY, 163, 46)
        if let Some(btn) = self.menu_button {
            unsafe {
                (&mut *btn).x = 635 - self.dialog.x;
                (&mut *btn).y = -10 - self.dialog.y;
                (&mut *btn).width = 163;
                (&mut *btn).height = 46;
            }
        }
    }

    /// 挂接到管理器（对应 C++ AddedToManager，LawnDialog.cpp:528-535）
    pub fn added_to_manager(&mut self, manager: &mut WidgetManager) {
        self.dialog.added_to_manager(manager);
        if let Some(btn) = self.menu_button {
            unsafe {
                manager.add_widget((&mut *btn).as_widget_ptr());
            }
        }
    }

    /// 从管理器摘除（对应 C++ RemovedFromManager，LawnDialog.cpp:537-544）
    pub fn removed_from_manager(&mut self, manager: &mut WidgetManager) {
        self.dialog.removed_from_manager(manager);
        if let Some(btn) = self.menu_button {
            unsafe {
                manager.remove_widget((&mut *btn).as_widget_ptr());
            }
        }
    }
}
