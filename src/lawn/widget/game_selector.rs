// PvZ Portable Rust 翻译 — GameSelector
// 对应 C++ src/Lawn/Widget/GameSelector.h / GameSelector.cpp
//
// 游戏选择界面（主菜单）— 冒险/小游戏/解谜/生存等模式选择

#![allow(dead_code)]

use std::ptr;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::graphics::image::Image;
use crate::framework::color::Color;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{KeyCode, KEYCODE_C};
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::game_enums::*;
use crate::todlib::tod_common::*;
use crate::lawn::tool_tip_widget::ToolTipWidget;

// ============================================================
// 按钮 ID（对应 C++ GameSelector 内部 enum）
// ============================================================
const GS_ADVENTURE: i32 = 100;
const GS_MINIGAME: i32 = 101;
const GS_PUZZLE: i32 = 102;
const GS_OPTIONS: i32 = 103;
const GS_HELP: i32 = 104;
const GS_QUIT: i32 = 105;
const GS_CHANGE_USER: i32 = 106;
const GS_STORE: i32 = 107;
const GS_ALMANAC: i32 = 108;
const GS_ZEN_GARDEN: i32 = 109;
const GS_SURVIVAL: i32 = 110;
const GS_ZOMBATAR: i32 = 111;
const GS_ACHIEVEMENTS_BACK: i32 = 112;
const GS_ACHIEVEMENTS: i32 = 113;
const GS_QUICK_PLAY: i32 = 114;

// ============================================================
// SelectorAnimState — 选择器动画状态（对应 C++ SelectorAnimState）
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SelectorAnimState {
    Open = 0,
    NewUser,
    ShowSign,
    Idle,
}

// ============================================================
// 花朵中心位置（对应 C++ gFlowerCenter）
// ============================================================
const FLOWER_CENTER: [(f32, f32); 3] = [
    (765.0, 483.0),
    (663.0, 455.0),
    (701.0, 439.0),
];

// ============================================================
// 按钮数据结构（替代 NewLawnButton）
// ============================================================
pub struct GameBtnData {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    is_over: bool,
    is_down: bool,
    disabled: bool,
    visible: bool,
    btn_no_draw: bool,
    button_offset_x: i32,
    button_offset_y: i32,
    /// 正常状态图片名
    button_image_name: &'static str,
    /// 高亮图片名
    over_image_name: &'static str,
    /// 按下图片名
    down_image_name: &'static str,
    /// 缓存的图片指针
    cached_button_img: *mut Image,
    cached_over_img: *mut Image,
    cached_down_img: *mut Image,
}

impl GameBtnData {
    fn new(
        id: i32, x: i32, y: i32, w: i32, h: i32,
        btn_img: &'static str, over_img: &'static str, down_img: &'static str,
    ) -> Self {
        GameBtnData {
            id, x, y, width: w, height: h,
            is_over: false, is_down: false, disabled: false, visible: true,
            btn_no_draw: false,
            button_offset_x: 0, button_offset_y: 0,
            button_image_name: btn_img, over_image_name: over_img, down_image_name: down_img,
            cached_button_img: ptr::null_mut(),
            cached_over_img: ptr::null_mut(),
            cached_down_img: ptr::null_mut(),
        }
    }

    fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.width
            && py >= self.y && py < self.y + self.height
    }

    fn resolve_images(&mut self, app: &LawnApp) {
        if self.cached_button_img.is_null() && !self.button_image_name.is_empty() {
            self.cached_button_img = get_image(app, self.button_image_name);
        }
        if self.cached_over_img.is_null() && !self.over_image_name.is_empty() {
            self.cached_over_img = get_image(app, self.over_image_name);
        }
        if self.cached_down_img.is_null() && !self.down_image_name.is_empty() {
            self.cached_down_img = get_image(app, self.down_image_name);
        }
    }

    fn get_draw_image(&self) -> *mut Image {
        if self.disabled && !self.cached_down_img.is_null() {
            return self.cached_down_img;
        }
        if self.is_down && self.is_over {
            if !self.cached_down_img.is_null() {
                return self.cached_down_img;
            }
            if !self.cached_over_img.is_null() {
                return self.cached_over_img;
            }
        }
        if self.is_over && !self.cached_over_img.is_null() {
            return self.cached_over_img;
        }
        self.cached_button_img
    }
}

/// 工具函数：从资源管理器获取图片
fn get_image(app: &LawnApp, name: &str) -> *mut Image {
    if let Some(rm_ptr) = app.base.resource_manager {
        unsafe {
            let rm = &*rm_ptr;
            let shared = rm.get_image(name);
            let img_ptr = shared.as_image_ptr();
            if !img_ptr.is_null() {
                return img_ptr;
            }
            // 尝试小写
            let lower = crate::framework::common::string_to_lower(name);
            let shared2 = rm.get_image(&lower);
            let img_ptr2 = shared2.as_image_ptr();
            if !img_ptr2.is_null() {
                return img_ptr2;
            }
        }
    }
    ptr::null_mut()
}

// ============================================================
// GameSelectorImpl
// ============================================================
pub struct GameSelectorImpl {
    pub app: *mut LawnApp,

    // ---- 按钮 ----
    pub buttons: Vec<GameBtnData>,

    // ---- 状态 ----
    pub starting_game: bool,
    pub starting_game_counter: i32,
    pub minigames_locked: bool,
    pub puzzle_locked: bool,
    pub survival_locked: bool,
    pub show_start_button: bool,
    pub has_trophy: bool,
    pub unlock_selector_cheat: bool,
    pub level: i32,
    pub loading: bool,

    // ---- 动画状态 ----
    pub selector_state: SelectorAnimState,

    // ---- 工具提示 ----
    pub tool_tip: ToolTipWidget,

    // ---- 滑动动画（对应 GOTY SlideTo） ----
    pub slide_counter: i32,
    pub start_x: i32,
    pub start_y: i32,
    pub dest_x: i32,
    pub dest_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,

    // ---- 字体 ----
    pub fonts_init: bool,
    pub briannetod16: Option<Box<Font>>,

    // ---- 鼠标位置 ----
    pub mouse_x: i32,
    pub mouse_y: i32,

    // ---- 覆盖绘制子控件（对应 C++ mOverlayWidget） ----
    pub overlay_widget: Option<Box<Widget>>,

    // ---- 是否已初始化 ----
    pub synced: bool,
}

impl GameSelectorImpl {
    pub fn new(app: *mut LawnApp) -> Self {
        let app_ref = unsafe { &*app };
        let h = app_ref.base.height;

        let mut sel = GameSelectorImpl {
            app,
            buttons: Vec::new(),
            starting_game: false,
            starting_game_counter: 0,
            minigames_locked: true,
            puzzle_locked: true,
            survival_locked: true,
            show_start_button: false,
            has_trophy: false,
            unlock_selector_cheat: false,
            level: 1,
            loading: false,
            selector_state: SelectorAnimState::Open,
            tool_tip: ToolTipWidget::new(),
            slide_counter: 0,
            start_x: 0, start_y: 0, dest_x: 0, dest_y: 0,
            offset_x: 0, offset_y: 0,
            fonts_init: false,
            briannetod16: None,
            mouse_x: 0, mouse_y: 0,
            overlay_widget: None,
            synced: false,
        };

        // 创建按钮（位置为近似值，实际坐标由 update_button_positions 修正）
        sel.buttons.push(GameBtnData::new(
            GS_ADVENTURE, 0, 0, 330, 125,
            "IMAGE_REANIM_SELECTORSCREEN_ADVENTURE_BUTTON",
            "IMAGE_REANIM_SELECTORSCREEN_ADVENTURE_HIGHLIGHT",
            "IMAGE_REANIM_SELECTORSCREEN_ADVENTURE_HIGHLIGHT",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_MINIGAME, 0, 125, 320, 130,
            "IMAGE_REANIM_SELECTORSCREEN_SURVIVAL_BUTTON",
            "IMAGE_REANIM_SELECTORSCREEN_SURVIVAL_HIGHLIGHT",
            "IMAGE_REANIM_SELECTORSCREEN_SURVIVAL_HIGHLIGHT",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_PUZZLE, 0, 255, 290, 121,
            "IMAGE_REANIM_SELECTORSCREEN_CHALLENGES_BUTTON",
            "IMAGE_REANIM_SELECTORSCREEN_CHALLENGES_HIGHLIGHT",
            "IMAGE_REANIM_SELECTORSCREEN_CHALLENGES_HIGHLIGHT",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_SURVIVAL, 0, 376, 275, 124,
            "IMAGE_REANIM_SELECTORSCREEN_VASEBREAKER_BUTTON",
            "IMAGE_REANIM_SELECTORSCREEN_VASEBREAKER_HIGHLIGHT",
            "IMAGE_REANIM_SELECTORSCREEN_VASEBREAKER_HIGHLIGHT",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_OPTIONS, 0, 0, 100, 100,
            "IMAGE_SELECTORSCREEN_OPTIONS1",
            "IMAGE_SELECTORSCREEN_OPTIONS2",
            "IMAGE_SELECTORSCREEN_OPTIONS2",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_HELP, 0, 100, 100, 120,
            "IMAGE_SELECTORSCREEN_HELP1",
            "IMAGE_SELECTORSCREEN_HELP2",
            "IMAGE_SELECTORSCREEN_HELP2",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_QUIT, 0, 220, 100, 100,
            "IMAGE_SELECTORSCREEN_QUIT1",
            "IMAGE_SELECTORSCREEN_QUIT2",
            "IMAGE_SELECTORSCREEN_QUIT2",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_STORE, 405, 484, 80, 80,
            "IMAGE_SELECTORSCREEN_STORE",
            "IMAGE_SELECTORSCREEN_STOREHIGHLIGHT",
            "IMAGE_SELECTORSCREEN_STOREHIGHLIGHT",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_ALMANAC, 327, 428, 80, 80,
            "IMAGE_SELECTORSCREEN_ALMANAC",
            "IMAGE_SELECTORSCREEN_ALMANACHIGHLIGHT",
            "IMAGE_SELECTORSCREEN_ALMANACHIGHLIGHT",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_ZEN_GARDEN, 0, 500, 130, 130,
            "IMAGE_SELECTORSCREEN_ZENGARDEN",
            "IMAGE_SELECTORSCREEN_ZENGARDENHIGHLIGHT",
            "IMAGE_SELECTORSCREEN_ZENGARDENHIGHLIGHT",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_ACHIEVEMENTS, 20, h - 100, 100, 65,
            "IMAGE_SELECTORSCREEN_ACHIEVEMENTS_PEDESTAL",
            "IMAGE_SELECTORSCREEN_ACHIEVEMENTS_PEDESTAL_PRESS",
            "IMAGE_SELECTORSCREEN_ACHIEVEMENTS_PEDESTAL_PRESS",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_QUICK_PLAY, 650, 455, 100, 50,
            "", "", "",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_ZOMBATAR, 0, 0, 100, 60,
            "IMAGE_BLANK", "IMAGE_BLANK", "IMAGE_BLANK",
        ));
        sel.buttons.push(GameBtnData::new(
            GS_CHANGE_USER, 20, h - 30, 250, 30,
            "IMAGE_BLANK", "IMAGE_BLANK", "IMAGE_BLANK",
        ));

        sel
    }

    fn ensure_fonts(&mut self) {
        if !self.fonts_init {
            let mut f = Font::new("Briannetod", 16);
            f.ascent = 13;
            f.font_height = 16;
            self.briannetod16 = Some(Box::new(f));
            self.fonts_init = true;
        }
    }

    fn img_width(img: *mut Image) -> i32 {
        if img.is_null() { 0 } else { unsafe { (*img).get_width() } }
    }

    fn img_height(img: *mut Image) -> i32 {
        if img.is_null() { 0 } else { unsafe { (*img).get_height() } }
    }

    /// 更新按钮位置（基于偏移量）
    fn update_button_positions(&mut self, widget: &Widget) {
        let app = unsafe { &*self.app };
        for btn in self.buttons.iter_mut() {
            btn.resolve_images(app);
            match btn.id {
                GS_OPTIONS => {
                    let w = Self::img_width(btn.cached_button_img);
                    let h = Self::img_height(btn.cached_button_img);
                    btn.x = widget.width - w - 10 + self.offset_x;
                    btn.y = 15 + self.offset_y;
                    btn.width = w;
                    btn.height = h + 23;
                    btn.button_offset_y = 15;
                }
                GS_HELP => {
                    let w = Self::img_width(btn.cached_button_img);
                    let h = Self::img_height(btn.cached_button_img);
                    btn.x = widget.width - w - 10 + self.offset_x;
                    btn.y = 100 + self.offset_y;
                    btn.width = w;
                    btn.height = h + 33;
                    btn.button_offset_y = 30;
                }
                GS_QUIT => {
                    let w = Self::img_width(btn.cached_button_img);
                    let h = Self::img_height(btn.cached_button_img);
                    btn.x = widget.width - w - 10 + self.offset_x;
                    btn.y = 220 + self.offset_y;
                    btn.width = w + 10;
                    btn.height = h + 10;
                    btn.button_offset_x = 5;
                    btn.button_offset_y = 5;
                }
                GS_STORE => {
                    btn.x = 405 + self.offset_x;
                    btn.y = 484 + self.offset_y;
                }
                GS_ALMANAC => {
                    btn.x = 327 + self.offset_x;
                    btn.y = 428 + self.offset_y;
                }
                GS_ZEN_GARDEN => {
                    btn.x = 0 + self.offset_x;
                    btn.y = 500 + self.offset_y;
                }
                GS_ACHIEVEMENTS => {
                    let h = Self::img_height(btn.cached_button_img);
                    btn.x = 20 + self.offset_x;
                    btn.y = widget.height - h - 35 + self.offset_y;
                    btn.width = Self::img_width(btn.cached_button_img);
                    btn.height = h;
                }
                GS_QUICK_PLAY => {
                    btn.x = widget.width - 150 + self.offset_x;
                    btn.y = 455 + self.offset_y;
                }
                GS_ADVENTURE => {
                    btn.x = self.offset_x;
                    btn.y = self.offset_y;
                }
                GS_MINIGAME => {
                    btn.x = self.offset_x;
                    btn.y = 125 + self.offset_y;
                }
                GS_PUZZLE => {
                    btn.x = self.offset_x;
                    btn.y = 255 + self.offset_y;
                }
                GS_SURVIVAL => {
                    btn.x = self.offset_x;
                    btn.y = 376 + self.offset_y;
                }
                GS_ZOMBATAR => {
                    btn.x = self.offset_x;
                    btn.y = 500 + self.offset_y;
                }
                GS_CHANGE_USER => {
                    btn.x = 20 + self.offset_x;
                    btn.y = widget.height - 30 + self.offset_y;
                }
                _ => {}
            }
        }
    }

    // ============================================================
    // SyncProfile — 同步玩家档案
    // ============================================================
    pub fn sync_profile(&mut self, _the_show_loading: bool) {
        let app = unsafe { &mut *self.app };

        self.level = 1;
        if let Some(ref player_info) = app.player_info {
            self.level = player_info.get_level();
        }
        self.show_start_button = true;
        self.minigames_locked = true;
        self.puzzle_locked = true;
        self.survival_locked = true;

        if app.player_info.is_some() && !app.is_ice_demo() {
            if self.level >= 2 {
                self.show_start_button = false;
            }

            if app.has_finished_adventure() {
                self.minigames_locked = false;
                self.survival_locked = false;
                self.puzzle_locked = false;
                self.show_start_button = false;
            }

            if let Some(ref player_info) = app.player_info {
                if player_info.m_has_unlocked_minigames {
                    self.minigames_locked = false;
                }
                if player_info.m_has_unlocked_puzzle_mode {
                    self.puzzle_locked = false;
                }
                if player_info.m_has_unlocked_survival_mode {
                    self.survival_locked = false;
                }
            }

            if app.is_trial_stage_locked() {
                self.puzzle_locked = true;
                self.survival_locked = true;
            }
        }

        let earned_trophy = app.has_finished_adventure() && !app.is_trial_stage_locked();
        self.has_trophy = earned_trophy;

        self.sync_buttons();
    }

    // ============================================================
    // SyncButtons — 同步按钮状态
    // ============================================================
    pub fn sync_buttons(&mut self) {
        let app = unsafe { &*self.app };

        let almanac_available = app.can_show_almanac() || self.unlock_selector_cheat;
        let store_open = app.can_show_store() || self.unlock_selector_cheat;
        let zen_garden_open = app.can_show_zen_garden() || self.unlock_selector_cheat;

        for btn in self.buttons.iter_mut() {
            match btn.id {
                GS_ALMANAC => {
                    btn.disabled = !almanac_available;
                    btn.visible = almanac_available;
                }
                GS_STORE => {
                    btn.disabled = !store_open;
                    btn.visible = store_open;
                }
                GS_ZEN_GARDEN => {
                    btn.disabled = !zen_garden_open;
                    btn.visible = zen_garden_open;
                }
                GS_MINIGAME => {
                    btn.disabled = self.minigames_locked;
                }
                GS_PUZZLE => {
                    btn.disabled = self.puzzle_locked;
                }
                GS_SURVIVAL => {
                    btn.disabled = self.survival_locked;
                }
                _ => {}
            }
        }

        // 切换 Adventure 按钮图片（StartAdventure vs Adventure）
        if let Some(btn) = self.buttons.iter_mut().find(|b| b.id == GS_ADVENTURE) {
            if self.show_start_button {
                btn.button_image_name = "IMAGE_REANIM_SELECTORSCREEN_STARTADVENTURE_BUTTON";
                btn.over_image_name = "IMAGE_REANIM_SELECTORSCREEN_STARTADVENTURE_HIGHLIGHT";
                btn.down_image_name = "IMAGE_REANIM_SELECTORSCREEN_STARTADVENTURE_HIGHLIGHT";
            } else {
                btn.button_image_name = "IMAGE_REANIM_SELECTORSCREEN_ADVENTURE_BUTTON";
                btn.over_image_name = "IMAGE_REANIM_SELECTORSCREEN_ADVENTURE_HIGHLIGHT";
                btn.down_image_name = "IMAGE_REANIM_SELECTORSCREEN_ADVENTURE_HIGHLIGHT";
            }
            btn.cached_button_img = ptr::null_mut();
            btn.cached_over_img = ptr::null_mut();
            btn.cached_down_img = ptr::null_mut();
        }
    }

    // ============================================================
    // ClickedAdventure — 点击冒险模式
    // ============================================================
    fn clicked_adventure(&mut self) {
        self.starting_game = true;
        for btn in self.buttons.iter_mut() {
            btn.disabled = true;
        }
    }
}

impl GameSelectorImpl {
    /// 提取的覆盖绘制逻辑（供 draw_overlay 和 GameSelectorOverlayImpl 共享）
    pub fn draw_overlay_internal(&mut self, g: &mut Graphics) {
        let app = unsafe { &*self.app };
        g.set_linear_blend(true);

        if app.player_info.is_none() {
            return;
        }

        // 绘制关卡编号
        if !app.is_ice_demo() && !self.show_start_button {
            let stage = clamp_int((self.level - 1) / 10 + 1, 1, 6);
            let sub = self.level - (stage - 1) * 10;
            let tx = self.offset_x as f32;
            let ty = self.offset_y as f32;

            let level_img = get_image(app, "IMAGE_SELECTORSCREEN_LEVELNUMBERS");
            if !level_img.is_null() {
                g.set_colorize_images(true);
                g.set_color(&Color::WHITE);
                unsafe {
                    g.draw_image_cel_rc(&*level_img,
                        (tx + 486.0) as i32, (ty + 47.0) as i32,
                        stage - 1, 0);
                    if sub < 10 {
                        g.draw_image_cel_rc(&*level_img,
                            (tx + 509.0) as i32, (ty + 50.0) as i32,
                            sub - 1, 0);
                    }
                }
                g.set_colorize_images(false);
            }
        }

        // 绘制其他按钮（Store、Almanac、ZenGarden、Achievements）
        for btn in &self.buttons {
            match btn.id {
                GS_STORE | GS_ALMANAC | GS_ZEN_GARDEN | GS_ACHIEVEMENTS => {
                    self.draw_button_at(g, btn, 0.0, 0.0);
                }
                _ => {}
            }
        }

        // 绘制奖杯
        if self.has_trophy {
            let trophy_img = get_image(app, "IMAGE_SUNFLOWER_TROPHY");
            if !trophy_img.is_null() {
                let cel = if app.earned_gold_trophy() { 1 } else { 0 };
                unsafe {
                    g.draw_image_cel(&*trophy_img,
                        self.offset_x + 12, self.offset_y + 345, cel);
                }
            }
        }
    }
}

// ============================================================
// WidgetImpl — 实现
// ============================================================
impl WidgetImpl for GameSelectorImpl {
    fn draw(&mut self, widget: &Widget, g: &mut Graphics) {
        let app = unsafe { &*self.app };
        g.set_linear_blend(true);

        // 绘制背景
        let bg_img = get_image(app, "IMAGE_REANIM_SELECTORSCREEN_BG");
        if !bg_img.is_null() {
            unsafe { g.draw_image_xy(&*bg_img, 0, 0); }
        } else {
            let fallback = get_image(app, "titlescreen");
            if !fallback.is_null() {
                unsafe { g.draw_image_xy(&*fallback, 0, 0); }
            }
        }

        // 更新按钮位置
        self.update_button_positions(widget);

        // 绘制左侧主按钮
        let tx = self.offset_x as f32;
        let ty = self.offset_y as f32;
        for btn in &self.buttons {
            match btn.id {
                GS_ADVENTURE | GS_MINIGAME | GS_PUZZLE | GS_SURVIVAL => {
                    self.draw_button_at(g, btn, tx, ty);
                }
                _ => {}
            }
        }

        // 绘制 Options / Help / Quit（仅 SELECTOR_OPEN 状态）
        if self.selector_state == SelectorAnimState::Open {
            for btn in &self.buttons {
                match btn.id {
                    GS_OPTIONS | GS_HELP | GS_QUIT => {
                        self.draw_button_at(g, btn, 0.0, 0.0);
                    }
                    _ => {}
                }
            }
        }

        // 绘制欢迎文字
        if let Some(ref player_info) = app.player_info {
            if !player_info.name.is_empty()
                && self.selector_state != SelectorAnimState::Open
                && self.selector_state != SelectorAnimState::NewUser
            {
                self.ensure_fonts();
                if let Some(ref font) = self.briannetod16 {
                    let welcome_str = format!("{}!", player_info.name);
                    let string_w = font.string_width(&welcome_str);
                    let text_x = 170 + self.offset_x - (string_w / 2);
                    let text_y = 102 + self.offset_y + font.get_ascent();
                    g.set_color(&Color::new(255, 245, 200, 255));
                    let fptr = &**font as *const Font as *mut Font;
                    g.set_font(fptr);
                    g.draw_string(&welcome_str, text_x, text_y);
                }
            }
        }
    }

    fn draw_overlay(&mut self, widget: &Widget, g: &mut Graphics) {
        let _ = widget;
        self.draw_overlay_internal(g);
    }

    fn update(&mut self, widget: &mut Widget) {
        widget.mark_dirty();

        if !self.synced {
            self.synced = true;
            self.sync_profile(false);
        }

        // 滑动动画
        if self.slide_counter > 0 {
            let new_x = tod_animate_curve(
                75, 0, self.slide_counter,
                self.start_x, self.dest_x, TodCurves::EaseInOut,
            );
            let new_y = tod_animate_curve(
                75, 0, self.slide_counter,
                self.start_y, self.dest_y, TodCurves::EaseInOut,
            );
            self.offset_x = new_x;
            self.offset_y = new_y;
            self.slide_counter -= 1;
        }

        // 更新按钮悬停
        for btn in self.buttons.iter_mut() {
            btn.is_over = btn.visible && !btn.disabled && btn.contains(self.mouse_x, self.mouse_y);
        }

        // 游戏开始倒计时
        if self.starting_game {
            self.starting_game_counter += 1;
            if self.starting_game_counter > 450 {
                let app = unsafe { &mut *self.app };

                // 闪烁 Adventure 按钮
                for btn in self.buttons.iter_mut() {
                    if btn.id == GS_ADVENTURE {
                        btn.disabled = self.starting_game_counter % 20 >= 10;
                    }
                }

                // 从 WidgetManager 移除
                if let Some(wm) = app.base.widget_manager {
                    unsafe {
                        (*wm).remove_widget(widget as *mut Widget);
                    }
                }

                if app.is_ice_demo() {
                    app.pre_new_game(GameMode::ChallengeIceLevel, false);
                    return;
                }
                if app.is_first_time_adventure_mode() && self.level == 1 && !app.save_file_exists() {
                    app.pre_new_game(GameMode::Adventure, false);
                    return;
                }
                app.pre_new_game(GameMode::Adventure, true);
                return;
            }

            for btn in self.buttons.iter_mut() {
                if btn.id == GS_ADVENTURE {
                    btn.disabled = self.starting_game_counter % 20 >= 10;
                }
            }
        }
    }

    fn key_down(&mut self, _widget: &mut Widget, key: KeyCode, _wm: &mut WidgetManager) {
        if self.starting_game {
            return;
        }

        let app = unsafe { &mut *self.app };
        if app.m_debug_keys_enabled {
            match key {
                KEYCODE_C => {
                    self.minigames_locked = false;
                    self.puzzle_locked = false;
                    self.survival_locked = false;
                    self.sync_buttons();
                }
                _ => {}
            }
        }

        // 调试键 'u' — 解锁全部
        // 处理在 key_char 中
    }

    fn key_char(&mut self, _widget: &mut Widget, c: u8) {
        if self.starting_game {
            return;
        }

        let app = unsafe { &mut *self.app };
        if (app.m_tod_cheat_keys || app.m_debug_keys_enabled) && c == b'u' {
            if let Some(ref mut player_info) = app.player_info {
                player_info.m_finished_adventure = 2;
                player_info.m_has_used_cheat_keys = true;
                player_info.m_has_unlocked_minigames = true;
                player_info.m_has_unlocked_puzzle_mode = true;
                player_info.m_has_unlocked_survival_mode = true;
            }
            self.sync_profile(true);
        }
    }

    fn mouse_down_btn(&mut self, _widget: &mut Widget, x: i32, y: i32, _btn: i32, _click: i32) {
        // 花朵彩蛋
        for &(fx, fy) in FLOWER_CENTER.iter() {
            let dist = distance(x as f32, y as f32, fx, fy);
            if dist < 20.0 {
                // 播放音效（在 Rust 中简化）
                eprintln!("[GameSelector] 点击花朵");
            }
        }

        // 作弊：跳过倒计时
        let app = unsafe { &*self.app };
        if app.m_tod_cheat_keys && self.starting_game && self.starting_game_counter < 450 {
            self.starting_game_counter = 450;
        }

        // 按钮按下
        for btn in self.buttons.iter_mut() {
            if btn.visible && !btn.disabled && btn.contains(x, y) {
                btn.is_down = true;
                break;
            }
        }
    }

    fn mouse_up_btn(&mut self, _widget: &mut Widget, _x: i32, _y: i32, _btn: i32, _click: i32) {
        if self.slide_counter > 0 {
            for b in self.buttons.iter_mut() { b.is_down = false; }
            return;
        }

        let mut clicked_id: Option<i32> = None;
        for btn in self.buttons.iter_mut() {
            if btn.is_down {
                btn.is_down = false;
                if btn.is_over && btn.visible && !btn.disabled {
                    clicked_id = Some(btn.id);
                }
                break;
            }
        }

        let the_id = match clicked_id {
            Some(id) => id,
            None => return,
        };

        // 锁定模式检查
        match the_id {
            GS_MINIGAME if self.minigames_locked => {
                eprintln!("[GameSelector] Minigames locked");
                return;
            }
            GS_PUZZLE if self.puzzle_locked => {
                eprintln!("[GameSelector] Puzzle locked");
                return;
            }
            GS_SURVIVAL if self.survival_locked => {
                eprintln!("[GameSelector] Survival locked");
                return;
            }
            _ => {}
        }

        let app = unsafe { &mut *self.app };

        match the_id {
            GS_ADVENTURE => self.clicked_adventure(),
            GS_MINIGAME => {
                app.kill_game_selector();
                app.show_challenge_screen(ChallengePage::Challenge as i32);
            }
            GS_PUZZLE => {
                app.kill_game_selector();
                app.show_challenge_screen(ChallengePage::Puzzle as i32);
            }
            GS_SURVIVAL => {
                app.kill_game_selector();
                app.show_challenge_screen(ChallengePage::Survival as i32);
            }
            GS_QUIT => {
                app.m_close_request = true;
            }
            GS_HELP => {
                app.kill_game_selector();
                app.show_award_screen(AwardType::HelpZombieNote as i32, false);
            }
            GS_OPTIONS => {
                eprintln!("[GameSelector] Options — 等待 NewOptionsDialog");
            }
            GS_CHANGE_USER => {
                eprintln!("[GameSelector] ChangeUser — 等待 UserDialog");
            }
            GS_STORE => {
                eprintln!("[GameSelector] Store — 等待 StoreScreen");
            }
            GS_ALMANAC => {
                eprintln!("[GameSelector] Almanac — 等待 AlmanacDialog");
            }
            GS_ZEN_GARDEN => {
                app.kill_game_selector();
                app.pre_new_game(GameMode::ChallengeZenGarden, false);
            }
            GS_ZOMBATAR => {
                eprintln!("[GameSelector] Zombatar — 未实现");
            }
            GS_ACHIEVEMENTS => {
                if self.slide_counter == 0 {
                    let h = app.base.height;
                    self.slide_counter = 75;
                    self.dest_x = 0;
                    self.dest_y = -h;
                    self.start_x = self.offset_x;
                    self.start_y = self.offset_y;
                }
            }
            GS_QUICK_PLAY => {
                eprintln!("[GameSelector] QuickPlay — 未实现");
            }
            _ => {}
        }
    }

    fn mouse_move(&mut self, _widget: &mut Widget, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }
}

impl GameSelectorImpl {
    fn draw_button_at(
        &self, g: &mut Graphics, btn: &GameBtnData,
        _offset_x: f32, _offset_y: f32,
    ) {
        if btn.btn_no_draw || !btn.visible {
            return;
        }
        let img = btn.get_draw_image();
        if img.is_null() {
            return;
        }

        let draw_x = btn.x + btn.button_offset_x;
        let draw_y = btn.y + btn.button_offset_y;

        if btn.disabled {
            g.set_colorize_images(true);
            g.set_color(&Color::new(128, 128, 128, 200));
            unsafe { g.draw_image_xy(&*img, draw_x, draw_y); }
            g.set_colorize_images(false);
        } else if btn.is_down && btn.is_over {
            unsafe { g.draw_image_xy(&*img, draw_x + 1, draw_y + 1); }
        } else {
            unsafe { g.draw_image_xy(&*img, draw_x, draw_y); }
        }
    }
}

// ============================================================
// GameSelectorOverlayImpl — 覆盖绘制子控件（对应 C++ GameSelectorOverlay）
// ============================================================
/// 作为 GameSelector 的子 widget，其 Draw 委托回父控件的覆盖绘制逻辑。
/// 在 C++ 中，GameSelectorOverlay::Draw(g) 调用 mParent->DrawOverlay(g)。
pub struct GameSelectorOverlayImpl {
    pub parent: *mut GameSelectorImpl,
}

impl GameSelectorOverlayImpl {
    pub fn new(parent: *mut GameSelectorImpl) -> Self {
        GameSelectorOverlayImpl { parent }
    }

    /// 创建包装此实现的新 Widget，并设置初始属性
    ///（对应 C++ 构造函数中 mMouseVisible = false, mHasAlpha = true）
    pub fn create_widget(parent: *mut GameSelectorImpl) -> Widget {
        let mut w = Widget::new();
        w.has_alpha = true;
        w.mouse_visible = false;
        w.impl_ = Some(Box::new(GameSelectorOverlayImpl::new(parent)));
        w
    }
}

impl WidgetImpl for GameSelectorOverlayImpl {
    fn draw(&mut self, widget: &Widget, g: &mut Graphics) {
        let _ = widget;
        let parent = unsafe { &mut *self.parent };
        parent.draw_overlay_internal(g);
    }
}
