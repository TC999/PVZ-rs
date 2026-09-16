// PvZ Portable Rust 翻译 — NewOptionsDialog
// 对应 C++ src/Lawn/Widget/NewOptionsDialog.h / NewOptionsDialog.cpp

#![allow(dead_code)]
use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::checkbox::{Checkbox, CheckboxListener};
use crate::framework::widget::dialog::{BUTTONS_YES_NO, ID_YES};
use crate::framework::widget::slider::{Slider, SliderListener};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::widget::button_listener::ButtonListener;
use crate::lawn::game_enums::*;
use crate::lawn::widget::game_button::{self, LawnStoneButton, NewLawnButton};

/// 按钮 ID（对应 C++ NewOptionsDialog.h NewOptionsDialog_ 枚举）
const NEWOPTIONS_ALMANAC: i32 = 0;
const NEWOPTIONS_MAIN_MENU: i32 = 1;
const NEWOPTIONS_RESTART: i32 = 2;
const NEWOPTIONS_UPDATE: i32 = 3;
/// 对应 C++ ConstEnums.h Dialogs::DIALOG_CONFIRM_RESTART
const DIALOG_CONFIRM_RESTART: i32 = 23;

pub struct NewOptionsDialog {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub visible: bool,
    /// 对应 C++ mApp（用于读取/设置音量等）
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    /// 对应 C++ mMusicVolumeSlider / mSfxVolumeSlider（简化：记录值 + 拖动标记）
    pub music_volume: f64,
    pub sfx_volume: f64,
    pub sfx_slider_dragging: bool,
    /// 对应 C++ mFullscreenCheckbox / mHardwareAccelerationCheckbox（简化布尔态）
    pub fullscreen_checked: bool,
    pub hardware_acceleration_checked: bool,
    /// 对应 C++ mFromGameSelector
    pub from_game_selector: bool,
    // 对应 C++ 8 个子控件（C++ 经 widget_manager AddWidget 挂接；Rust Slider/Checkbox
    // 非 Widget 类型，由对话框自行绘制/转发输入，TRANSLATION_NOTE）
    pub music_slider: Option<*mut Slider>,
    pub sfx_slider: Option<*mut Slider>,
    pub fullscreen_checkbox: Option<*mut Checkbox>,
    pub hardware_checkbox: Option<*mut Checkbox>,
    pub almanac_button: Option<*mut LawnStoneButton>,
    pub restart_button: Option<*mut LawnStoneButton>,
    pub back_to_main_button: Option<*mut LawnStoneButton>,
    pub back_to_game_button: Option<*mut NewLawnButton>,
}

impl NewOptionsDialog {
    pub fn new() -> Self {
        NewOptionsDialog {
            x: 0, y: 0, width: 0, height: 0, visible: true,
            app: None,
            music_volume: 0.85,
            sfx_volume: 1.0,
            sfx_slider_dragging: false,
            fullscreen_checked: true,
            hardware_acceleration_checked: false,
            from_game_selector: false,
            music_slider: None,
            sfx_slider: None,
            fullscreen_checkbox: None,
            hardware_checkbox: None,
            almanac_button: None,
            restart_button: None,
            back_to_main_button: None,
            back_to_game_button: None,
        }
    }
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.x = x; self.y = y; self.width = w; self.height = h;
        // 对应 C++ Resize（NewOptionsDialog.cpp）：布局 8 个子控件
        if let Some(p) = self.music_slider { unsafe { (*p).x = 199; (*p).y = 116; (*p).width = 135; (*p).height = 40; } }
        if let Some(p) = self.sfx_slider { unsafe { (*p).x = 199; (*p).y = 143; (*p).width = 135; (*p).height = 40; } }
        if let Some(p) = self.hardware_checkbox { unsafe { (*p).x = 283; (*p).y = 175; (*p).width = 46; (*p).height = 45; } }
        if let Some(p) = self.fullscreen_checkbox { unsafe { (*p).x = 284; (*p).y = 206; (*p).width = 46; (*p).height = 45; } }
        if let Some(p) = self.almanac_button { unsafe { (*p).dialog_button.widget.resize(107, 241, 209, 46); } }
        if let Some(p) = self.restart_button { unsafe { (*p).dialog_button.widget.resize(107, 241 + 43, 209, 46); } }
        if let Some(p) = self.back_to_main_button { unsafe { (*p).dialog_button.widget.resize(107, 241 + 43 + 43, 209, 46); } }
        if let Some(p) = self.back_to_game_button { unsafe { (*p).dialog_button.widget.resize(30, 381, (*p).dialog_button.widget.width, (*p).dialog_button.widget.height); } }
        if self.from_game_selector {
            // C++: mFromGameSelector 时滑块/复选框下移 5/10/15/20
            if let Some(p) = self.music_slider { unsafe { (*p).y += 5; } }
            if let Some(p) = self.sfx_slider { unsafe { (*p).y += 10; } }
            if let Some(p) = self.hardware_checkbox { unsafe { (*p).y += 15; } }
            if let Some(p) = self.fullscreen_checkbox { unsafe { (*p).y += 20; } }
        }
        // C++: ZenGarden/TreeOfWisdom 时 mAlmanacButton->mY += 43
        let a_zen_mode = self.app.map_or(false, |app| unsafe {
            let gm = (*app).game_mode;
            gm == GameMode::ChallengeZenGarden || gm == GameMode::ChallengeTreeOfWisdom
        });
        if a_zen_mode {
            if let Some(p) = self.almanac_button { unsafe { (*p).dialog_button.widget.y += 43; } }
        }
    }
    /// 创建 8 个子控件（对应 C++ NewOptionsDialog 构造函数：按钮/滑块/复选框 + 初值）
    /// [TRANSLATION_NOTE]: C++ 经 widget_manager AddWidget 挂接；Rust Slider/Checkbox 非
    /// Widget 类型，由对话框持有并自行绘制/转发输入（见 draw/mouse_down/mouse_move）
    pub fn setup_controls(&mut self) {
        let host = self as *mut NewOptionsDialog;
        // C++: mMusicVolumeSlider = new Slider(IMAGE_OPTIONS_SLIDERSLOT, IMAGE_OPTIONS_SLIDERKNOB2, ..., this);
        //      SetValue(GetMusicVolume() clamp 0..1)
        let a_slot = self.get_resource_image("IMAGE_OPTIONS_SLIDERSLOT");
        let a_knob = self.get_resource_image("IMAGE_OPTIONS_SLIDERKNOB2");
        let mut a_music_slider = Box::new(Slider::new(
            a_slot, a_knob, 4, Some(Box::new(RawSliderListener { host })),
        ));
        if let Some(app) = self.app {
            unsafe {
                let a_music_volume = (*app).base.music_volume.clamp(0.0, 1.0);
                a_music_slider.set_value(a_music_volume);
            }
        }
        self.music_slider = Some(Box::into_raw(a_music_slider));

        // C++: mSfxVolumeSlider = new Slider(...); SetValue(GetSfxVolume() / 0.65)
        let mut a_sfx_slider = Box::new(Slider::new(
            a_slot, a_knob, 5, Some(Box::new(RawSliderListener { host })),
        ));
        if let Some(app) = self.app {
            unsafe {
                let a_sfx_volume = (*app).base.sfx_volume / 0.65;
                a_sfx_slider.set_value(a_sfx_volume);
            }
        }
        self.sfx_slider = Some(Box::into_raw(a_sfx_slider));

        // C++: mFullscreenCheckbox = MakeNewCheckbox(Fullscreen, this, !mIsWindowed)
        //      mHardwareAccelerationCheckbox = MakeNewCheckbox(HardwareAcceleration, this, Is3DAccelerated())
        let a_cb0 = self.get_resource_image("IMAGE_OPTIONS_CHECKBOX0");
        let a_cb1 = self.get_resource_image("IMAGE_OPTIONS_CHECKBOX1");
        let mut a_fullscreen_checkbox = Box::new(Checkbox::new(
            a_cb0, a_cb1, 6, Some(Box::new(RawCheckboxListener { host })),
        ));
        let a_windowed = self.app.map_or(true, |app| unsafe { (*app).base.is_windowed });
        a_fullscreen_checkbox.set_checked(!a_windowed, false);
        self.fullscreen_checkbox = Some(Box::into_raw(a_fullscreen_checkbox));

        // C++: Is3DAccelerated()；Rust 无 3D 加速检测，以当前复选框状态为初值
        let mut a_hardware_checkbox = Box::new(Checkbox::new(
            a_cb0, a_cb1, 7, Some(Box::new(RawCheckboxListener { host })),
        ));
        a_hardware_checkbox.set_checked(self.hardware_acceleration_checked, false);
        self.hardware_checkbox = Some(Box::into_raw(a_hardware_checkbox));

        // C++: mAlmanacButton/mRestartButton/mBackToMainButton = MakeButton(...)
        let mut a_almanac_button = game_button::make_button(NEWOPTIONS_ALMANAC, Some(Box::new(RawButtonListener { host })), "[VIEW_ALMANAC_BUTTON]");
        a_almanac_button.dialog_button.widget.resize(107, 241, 209, 46);
        self.almanac_button = Some(Box::into_raw(Box::new(a_almanac_button)));

        let mut a_restart_button = game_button::make_button(NEWOPTIONS_RESTART, Some(Box::new(RawButtonListener { host })), "[RESTART_LEVEL]");
        a_restart_button.dialog_button.widget.resize(107, 241 + 43, 209, 46);
        self.restart_button = Some(Box::into_raw(Box::new(a_restart_button)));

        let mut a_back_to_main_button = game_button::make_button(NEWOPTIONS_MAIN_MENU, Some(Box::new(RawButtonListener { host })), "[MAIN_MENU_BUTTON]");
        a_back_to_main_button.dialog_button.widget.resize(107, 241 + 43 + 43, 209, 46);
        self.back_to_main_button = Some(Box::into_raw(Box::new(a_back_to_main_button)));

        // C++: mBackToGameButton = MakeNewButton(ID_OK, ..., IMAGE_OPTIONS_BACKTOGAMEBUTTON0/0/2)
        //（三态图片与偏移以默认 NewLawnButton 近似，TRANSLATION_NOTE）
        let mut a_back_to_game_button = game_button::make_new_button(
            ID_YES, Some(Box::new(RawButtonListener { host })), "[BACK_TO_GAME]",
        );
        a_back_to_game_button.dialog_button.widget.resize(30, 381, 209, 46);
        self.back_to_game_button = Some(Box::into_raw(Box::new(a_back_to_game_button)));

        // C++: mFromGameSelector 时按钮标签/可见性调整
        if self.from_game_selector {
            if let Some(p) = self.restart_button { unsafe { (*p).dialog_button.widget.visible = false; } }
            if let Some(p) = self.back_to_game_button { unsafe { (*p).set_label("[DIALOG_BUTTON_OK]"); } }
        }
        self.resize(self.x, self.y, self.width, self.height);
    }

    /// 释放 8 个子控件（对话框析构前调用；C++ 中 KillDialog → RemovedFromManager → RemoveWidget）
    pub fn free_controls(&mut self) {
        for p in [self.music_slider.take(), self.sfx_slider.take()] {
            if let Some(p) = p { unsafe { let _ = Box::from_raw(p); } }
        }
        for p in [self.fullscreen_checkbox.take(), self.hardware_checkbox.take()] {
            if let Some(p) = p { unsafe { let _ = Box::from_raw(p); } }
        }
        for p in [
            self.almanac_button.take(),
            self.restart_button.take(),
            self.back_to_main_button.take(),
        ] {
            if let Some(p) = p { unsafe { let _ = Box::from_raw(p); } }
        }
        if let Some(p) = self.back_to_game_button.take() {
            unsafe { let _ = Box::from_raw(p); }
        }
    }

    /// 从 ResourceManager 按 key 取图（对应 C++ IMAGE_* 全局资源；未接入资源表时返回 null）
    fn get_resource_image(&self, a_key: &str) -> *mut crate::framework::graphics::image::Image {
        let Some(app) = self.app else { return std::ptr::null_mut() };
        unsafe {
            let app_ref = &*app;
            let Some(rm) = app_ref.base.resource_manager else { return std::ptr::null_mut() };
            let rm_ref = &*rm;
            rm_ref.get_image(a_key).as_image_ptr()
        }
    }

    /// FONT_DWARVENTODCRAFT18 右对齐标签（对应 C++ PvzpDrawString DS_ALIGN_RIGHT）
    fn draw_label_right(&self, g: &mut Graphics, a_text: &str, a_right_x: i32, a_y: i32) {
        let mut a_font = crate::framework::graphics::font::Font::new("Dwarventodcraft", 18);
        a_font.ascent = 13;
        a_font.font_height = 18;
        g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
        g.set_color(&crate::framework::color::Color::new(107, 109, 145, 255));
        let a_w = a_font.string_width(a_text);
        g.draw_string(a_text, a_right_x - a_w, a_y);
    }

    /// 对应 C++ NewOptionsDialog::Draw（NewOptionsDialog.cpp 184-207）
    pub fn draw(&self, g: &mut Graphics) {
        let Some(app) = self.app else { return };
        unsafe {
        let a_back = self.get_resource_image("IMAGE_OPTIONS_MENUBACK");
        if !a_back.is_null() {
            g.draw_image_xy(unsafe { &*a_back }, 0, 0);
        }

        // C++: mFromGameSelector 时四行标签分别下移 5/10/15/20
        let a_music_offset = if self.from_game_selector { 5 } else { 0 };
        let a_sfx_offset = if self.from_game_selector { 10 } else { 0 };
        let a_3d_accel_offset = if self.from_game_selector { 15 } else { 0 };
        let a_full_screen_offset = if self.from_game_selector { 20 } else { 0 };

        // C++: aSliderLabelsX = mApp->GetInteger("OPTION_DLG_SLIDER_LABELS_OFFSET_X", 186);
        //      aCheckboxLabelsX = mApp->GetInteger("OPTION_DLG_CHECKBOX_LABELS_OFFSET_X", 274)
        //      （GetString 本地化标签以默认英文文案呈现）
        let a_slider_labels_x = (*app).base.get_integer_default("OPTION_DLG_SLIDER_LABELS_OFFSET_X", 186);
        let a_checkbox_labels_x = (*app).base.get_integer_default("OPTION_DLG_CHECKBOX_LABELS_OFFSET_X", 274);
        // C++: aFontScale = mApp->GetDouble("OPTION_DLG_LABEL_FONT_SCALE", 1.0)；
        //      非 1.0 时 SetScale 缩放标签，绘制后恢复
        let a_font_scale = (*app).base.get_double_default("OPTION_DLG_LABEL_FONT_SCALE", 1.0);
        if a_font_scale != 1.0 {
            g.set_scale(a_font_scale as f32, a_font_scale as f32, 0.0, 0.0);
        }
        self.draw_label_right(g, "Music", a_slider_labels_x, 140 + a_music_offset);
        self.draw_label_right(g, "Sound FX", a_slider_labels_x, 167 + a_sfx_offset);
        self.draw_label_right(g, "3D Acceleration", a_checkbox_labels_x, 197 + a_3d_accel_offset);
        self.draw_label_right(g, "Full Screen", a_checkbox_labels_x, 229 + a_full_screen_offset);
        if a_font_scale != 1.0 {
            g.set_scale(1.0, 1.0, 0.0, 0.0);
        }

        // C++: 子控件由 Dialog::Draw 绘制（Rust 对话框驱动，手动按控件局部坐标绘制）
        for p in [self.music_slider, self.sfx_slider] {
            if let Some(p) = p {
                let a_old_trans_x = g.trans_x;
                let a_old_trans_y = g.trans_y;
                g.trans_x += self.x as f64;
                g.trans_y += self.y as f64;
                (*p).draw(g);
                g.trans_x = a_old_trans_x;
                g.trans_y = a_old_trans_y;
            }
        }
        for p in [self.fullscreen_checkbox, self.hardware_checkbox] {
            if let Some(p) = p {
                let a_old_trans_x = g.trans_x;
                let a_old_trans_y = g.trans_y;
                g.trans_x += self.x as f64;
                g.trans_y += self.y as f64;
                (*p).draw(g);
                g.trans_x = a_old_trans_x;
                g.trans_y = a_old_trans_y;
            }
        }
        for p in [self.almanac_button, self.restart_button, self.back_to_main_button] {
            if let Some(p) = p {
                let a_old_trans_x = g.trans_x;
                let a_old_trans_y = g.trans_y;
                g.trans_x += self.x as f64;
                g.trans_y += self.y as f64;
                (&mut *p).draw(g);
                g.trans_x = a_old_trans_x;
                g.trans_y = a_old_trans_y;
            }
        }
        if let Some(p) = self.back_to_game_button {
            let a_old_trans_x = g.trans_x;
            let a_old_trans_y = g.trans_y;
            g.trans_x += self.x as f64;
            g.trans_y += self.y as f64;
            (&mut *p).draw(g);
            g.trans_x = a_old_trans_x;
            g.trans_y = a_old_trans_y;
        }
        }
    }
    pub fn update(&mut self) {}
    pub fn key_down(&mut self, key: i32) {
        // 对应 C++ KeyDown
        if let Some(app) = self.app {
            unsafe {
                if let Some(board) = (*app).board {
                    (*board).do_typing_check(key);
                }
            }
        }
        // C++ 中空格/回车 → ID_OK、ESC → ID_CANCEL（Dialogs 按钮处理）
        let _ = key;
    }
    pub fn mouse_down(&mut self, x: i32, y: i32, _btn: i32) {
        // 对应 C++ Dialog 基类将鼠标按下分发给子控件（Rust 对话框驱动，控件非 Widget 类型）
        let a_local_x = x - self.x;
        let a_local_y = y - self.y;
        // 滑块/复选框命中判定（局部坐标矩形）
        for p in [self.music_slider, self.sfx_slider] {
            if let Some(p) = p {
                unsafe {
                    if a_local_x >= (*p).x && a_local_x < (*p).x + (*p).width
                        && a_local_y >= (*p).y && a_local_y < (*p).y + (*p).height
                    {
                        (*p).mouse_down(a_local_x, a_local_y, 0);
                    }
                }
            }
        }
        for p in [self.fullscreen_checkbox, self.hardware_checkbox] {
            if let Some(p) = p {
                unsafe {
                    if a_local_x >= (*p).x && a_local_x < (*p).x + (*p).width
                        && a_local_y >= (*p).y && a_local_y < (*p).y + (*p).height
                    {
                        (*p).mouse_down_btn(a_local_x, a_local_y, 0, 1);
                    }
                }
            }
        }
    }

    /// 鼠标移动（对应 C++ Dialog 基类 MouseMove 分发；用于滑块拖动）
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        let a_local_x = x - self.x;
        let a_local_y = y - self.y;
        let wm = self.app.and_then(|app| unsafe { (*app).base.widget_manager });
        for p in [self.music_slider, self.sfx_slider] {
            if let Some(p) = p {
                unsafe {
                    if let Some(wm) = wm {
                        (*p).mouse_move(a_local_x, a_local_y, &mut *wm);
                    } else {
                        (*p).mouse_move(a_local_x, a_local_y, &mut WidgetManager::new());
                    }
                }
            }
        }
    }

    /// 鼠标释放（对应 C++ Dialog 基类 MouseUp 分发）
    pub fn mouse_up(&mut self, _x: i32, _y: i32) {
        for p in [self.music_slider, self.sfx_slider] {
            if let Some(p) = p {
                unsafe { (*p).mouse_up(_x - self.x, _y - self.y); }
            }
        }
    }

    /// 音量滑块变化（对应 C++ SliderVal）
    pub fn slider_val(&mut self, the_id: i32, the_val: f64) {
        let Some(app) = self.app else { return };
        unsafe {
            match the_id {
                4 => { // NewOptionsDialog_MusicVolume
                    self.music_volume = the_val;
                    (*app).base.set_music_volume(the_val);
                    if let Some(ss) = (*app).sound_system.as_ref() {
                        ss.rehookup_sound_with_music_volume();
                    }
                }
                5 => { // NewOptionsDialog_SoundVolume
                    self.sfx_volume = the_val * 0.65;
                    (*app).base.set_sfx_volume(the_val * 0.65);
                    if let Some(ss) = (*app).sound_system.as_ref() {
                        ss.rehookup_sound_with_music_volume();
                    }
                    if !self.sfx_slider_dragging {
                        // [TRANSLATION_NOTE]: C++ 中 PlaySample(SOUND_BUTTONCLICK)
                    }
                }
                _ => {}
            }
        }
    }

    /// 复选框变化（对应 C++ CheckboxChecked）
    pub fn checkbox_checked(&mut self, the_id: i32, checked: bool) {
        let Some(app) = self.app else { return };
        unsafe {
            match the_id {
                6 => { // NewOptionsDialog_Fullscreen
                    // C++: !checked && mApp->mForceFullscreen 时 DoDialog(DIALOG_COLORDEPTH_EXP)
                    // 并还原复选框；Rust 无 mForceFullscreen（窗口模式限制弹窗），直接反映勾选状态
                    self.fullscreen_checked = checked;
                }
                7 => { // NewOptionsDialog_HardwareAcceleration
                    if checked {
                        // [TRANSLATION_NOTE]: C++ 中 Is3DAccelerationSupported/Recommended 判定
                        // 并弹出 DIALOG_INFO 警告；Rust 侧无 3D 加速检测，直接启用
                        self.hardware_acceleration_checked = true;
                    } else {
                        self.hardware_acceleration_checked = false;
                    }
                }
                _ => {}
            }
        }
    }

    /// 对应 C++ NewOptionsDialog::AddedToManager
    pub fn added_to_manager(&mut self, _the_widget_manager: *mut WidgetManager) {
        // C++: Dialog::AddedToManager + AddWidget(mAlmanacButton/mRestartButton/mBackToMainButton/
        //      mMusicVolumeSlider/mSfxVolumeSlider/mHardwareAccelerationCheckbox/mFullscreenCheckbox/mBackToGameButton)
        // [TRANSLATION_NOTE]: Rust 侧这些子控件未创建（以简化布尔/数值态替代），注册链保留调用点
    }

    /// 对应 C++ NewOptionsDialog::RemovedFromManager
    pub fn removed_from_manager(&mut self, _the_widget_manager: *mut WidgetManager) {
        // C++: Dialog::RemovedFromManager + RemoveWidget（同上 8 个子控件）
        // [TRANSLATION_NOTE]: 同上，子控件未创建
    }

    /// 对应 C++ NewOptionsDialog::ButtonPress
    pub fn button_press(&mut self, _the_id: i32) {
        // C++: mApp->PlaySample(SOUND_GRAVEBUTTON)
        if let Some(app) = self.app {
            unsafe {
                (*app).play_sample(crate::framework::resources::ResourceId::SoundGravebutton as i32);
            }
        }
    }

    /// 对应 C++ NewOptionsDialog::GetPreferredHeight（NewOptionsDialog.cpp:129）
    pub fn get_preferred_height(&self) -> i32 {
        // C++: return IMAGE_OPTIONS_MENUBACK->mWidth;（C++ 原样返回宽度）
        let a_img = self.get_resource_image("IMAGE_OPTIONS_MENUBACK");
        if a_img.is_null() {
            0
        } else {
            unsafe { (*a_img).get_width() }
        }
    }

    /// 对应 C++ NewOptionsDialog::ButtonDepress
    pub fn button_depress(&mut self, the_id: i32) {
        // C++: Dialog::ButtonDepress(theId) —— 基类处理，Rust 无对应
        match the_id {
            NEWOPTIONS_ALMANAC => {
                let Some(app) = self.app else { return };
                unsafe {
                    // C++: DoAlmanacDialog(SEED_NONE, ZOMBIE_INVALID)->WaitForResult(true)；Rust 版返回 ()
                    (*app).do_almanac_dialog(SeedType::None, ZombieType::Invalid);
                }
            }
            NEWOPTIONS_MAIN_MENU => {
                let Some(app) = self.app else { return };
                unsafe {
                    if self.from_game_selector {
                        (*app).kill_new_options_dialog();
                        (*app).kill_game_selector();
                        (*app).show_award_screen(AwardType::CreditsZombieNote as i32, false);
                    } else if (*app).board.map_or(false, |b| unsafe { (*b).need_save_game() }) {
                        // C++: mApp->DoConfirmBackToMain() —— Rust 侧未实现，保留调用点
                    } else {
                        // C++: else if (mApp->mBoard && mApp->mBoard->mCutScene && mCutScene->IsSurvivalRepick())
                        //       DoConfirmBackToMain() —— Rust 侧 mCutScene 未接入，同上省略
                        if let Some(board) = (*app).board {
                            (*board).m_board_result = BoardResult::Quit;
                        }
                        (*app).do_back_to_main();
                    }
                }
            }
            NEWOPTIONS_RESTART => {
                let Some(app) = self.app else { return };
                unsafe {
                    if (*app).board.is_some() {
                        let a_dialog_title;
                        let a_dialog_message;
                        if (*app).is_puzzle_mode() {
                            a_dialog_title = "[RESTART_PUZZLE_HEADER]";
                            a_dialog_message = "[RESTART_PUZZLE_BODY]";
                        } else if (*app).is_challenge_mode() {
                            a_dialog_title = "[RESTART_CHALLENGE_HEADER]";
                            a_dialog_message = "[RESTART_CHALLENGE_BODY]";
                        } else if (*app).is_survival_mode() {
                            a_dialog_title = "[RESTART_SURVIVAL_HEADER]";
                            a_dialog_message = "[RESTART_SURVIVAL_BODY]";
                        } else {
                            a_dialog_title = "[RESTART_LEVEL_HEADER]";
                            a_dialog_message = "[RESTART_LEVEL_BODY]";
                        }

                        // C++: (LawnDialog*)DoDialog(DIALOG_CONFIRM_RESTART, ...) 并设置 Yes/No 标签；
                        // Rust do_dialog 返回 *mut Dialog，按钮标签跳过（同 StoreScreen PurchaseItem 注释）
                        let a_dialog = (*app).do_dialog(
                            DIALOG_CONFIRM_RESTART,
                            true,
                            a_dialog_title,
                            a_dialog_message,
                            "",
                            BUTTONS_YES_NO,
                        );
                        let a_result = a_dialog.map_or(0, |d| unsafe { (&mut *d).wait_for_result(true) });
                        if a_result == ID_YES {
                            if let Some(music) = (*app).music.as_mut() {
                                music.stop_all_music();
                            }
                            if let Some(ss) = (*app).sound_system.as_ref() {
                                ss.cancel_paused_foley();
                            }
                            (*app).kill_new_options_dialog();
                            if let Some(board) = (*app).board {
                                (&mut *board).m_board_result = BoardResult::Restart;
                                (*app).m_saw_yeti = (*board).m_killed_yeti;
                            }
                            (*app).pre_new_game((*app).game_mode, false);
                        }
                    }
                }
            }
            NEWOPTIONS_UPDATE => {
                // C++: mApp->CheckForUpdates() —— Rust 侧未实现，保留调用点
            }
            _ => {}
        }
    }
}


/// 滑块监听器（裸指针转发；控件与对话框同生命周期，free_controls 先于对话框析构释放）
struct RawSliderListener {
    host: *mut NewOptionsDialog,
}

impl SliderListener for RawSliderListener {
    fn slider_val(&mut self, id: i32, val: f64) {
        unsafe { (*self.host).slider_val(id, val); }
    }
}

/// 复选框监听器（对应 C++ CheckboxListener）
struct RawCheckboxListener {
    host: *mut NewOptionsDialog,
}

impl CheckboxListener for RawCheckboxListener {
    fn checkbox_checked(&mut self, id: i32, checked: bool) {
        unsafe { (*self.host).checkbox_checked(id, checked); }
    }
}

/// 按钮监听器（对应 C++ ButtonListener）
struct RawButtonListener {
    host: *mut NewOptionsDialog,
}

impl ButtonListener for RawButtonListener {
    fn button_press(&mut self, the_id: i32) {
        unsafe { (*self.host).button_press(the_id); }
    }
    fn button_depress(&mut self, the_id: i32) {
        unsafe { (*self.host).button_depress(the_id); }
    }
    fn button_down_tick(&mut self, _the_id: i32) {}
    fn button_mouse_enter(&mut self, _the_id: i32) {}
    fn button_mouse_leave(&mut self, _the_id: i32) {}
    fn button_mouse_move(&mut self, _the_id: i32, _the_x: i32, _the_y: i32) {}
}
