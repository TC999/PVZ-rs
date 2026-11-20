// PvZ Portable Rust 翻译 — GameSelector
// 对应 C++ src/Lawn/Widget/GameSelector.h / GameSelector.cpp
//
// 游戏选择界面（主菜单）— 冒险/小游戏/解谜/生存等模式选择

#![allow(dead_code)]

use crate::framework::color::Color;
use crate::framework::graphics::font::Font;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::key_codes::{KeyCode, KEYCODE_C};
use crate::framework::widget::dialog::BUTTONS_FOOTER;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::*;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::system::music::MusicTune;
use crate::lawn::tool_tip_widget::ToolTipWidget;
use crate::todlib::tod_common::*;
use std::ptr;

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
const FLOWER_CENTER: [(f32, f32); 3] = [(765.0, 483.0), (663.0, 455.0), (701.0, 439.0)];

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
    /// 对应 C++ TrackButton 绑定的动画轨道名（None = 不跟踪）
    track_name: Option<&'static str>,
    /// TrackButton 的位置偏移（对应 C++ theOffsetX/Y）
    track_offset_x: f32,
    track_offset_y: f32,
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
        id: i32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        btn_img: &'static str,
        over_img: &'static str,
        down_img: &'static str,
    ) -> Self {
        GameBtnData {
            id,
            x,
            y,
            width: w,
            height: h,
            is_over: false,
            is_down: false,
            disabled: false,
            visible: true,
            btn_no_draw: false,
            button_offset_x: 0,
            button_offset_y: 0,
            track_name: None,
            track_offset_x: 0.0,
            track_offset_y: 0.0,
            button_image_name: btn_img,
            over_image_name: over_img,
            down_image_name: down_img,
            cached_button_img: ptr::null_mut(),
            cached_over_img: ptr::null_mut(),
            cached_down_img: ptr::null_mut(),
        }
    }

    fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
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
    /// 对应 C++ mSelectorReanimID（选择器动画；哨兵用 REANIMATIONID_NONE，见 game_enums 说明）
    pub selector_reanim_id: ReanimationID,
    /// 对应 C++ mCloudReanimID[6]（GameSelector.h:75，飘动的云×6）
    pub cloud_reanim_id: [ReanimationID; 6],
    /// 对应 C++ mFlowerReanimID[3]（花×3，附着于背景轨道）
    pub flower_reanim_id: [ReanimationID; 3],
    /// 对应 C++ mLeafReanimID（草叶摆动）
    pub leaf_reanim_id: ReanimationID,
    /// 对应 C++ mCloudCounter[6]（云下次飘出的帧计数）
    pub cloud_counter: [i32; 6],
    /// 对应 C++ mLeafCounter（草叶摆动计数）
    pub leaf_counter: i32,
    /// 对应 C++ GameSelector::mTrophyParticleID（奖杯闪光粒子）
    pub m_trophy_particle_id: crate::lawn::game_enums::ParticleSystemID,

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

    /// 对应 C++ mHandReanimID（ClickedAdventure 创建的僵尸手动画，GameSelector.h:97）
    pub hand_reanim_id: ReanimationID,
}

/// 对应 C++ GameSelector::TrackButton（GameSelector.cpp:945-954）：
/// 按动画轨道当前变换设置按钮位置（Rust 侧作用于 GameBtnData）
fn track_button_position(
    app: *mut LawnApp,
    selector_reanim_id: ReanimationID,
    btn: &mut GameBtnData,
    the_track_name: &str,
    the_offset_x: f32,
    the_offset_y: f32,
) {
    unsafe {
        if let Some(a_reanim) = (*app).reanimation_get(selector_reanim_id) {
            let a_track_index = a_reanim.find_track_index(the_track_name);
            let mut a_transform = crate::todlib::definition::ReanimatorTransform::default();
            if a_reanim.get_current_transform(a_track_index, &mut a_transform) {
                btn.x = (a_transform.m_trans_x + the_offset_x) as i32;
                btn.y = (a_transform.m_trans_y + the_offset_y) as i32;
            }
        }
    }
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
            start_x: 0,
            start_y: 0,
            dest_x: 0,
            dest_y: 0,
            offset_x: 0,
            offset_y: 0,
            fonts_init: false,
            briannetod16: None,
            mouse_x: 0,
            mouse_y: 0,
            overlay_widget: None,
            synced: false,
            selector_reanim_id: crate::lawn::game_enums::REANIMATIONID_NONE,
            cloud_reanim_id: [crate::lawn::game_enums::REANIMATIONID_NONE; 6],
            flower_reanim_id: [crate::lawn::game_enums::REANIMATIONID_NONE; 3],
            leaf_reanim_id: crate::lawn::game_enums::REANIMATIONID_NONE,
            cloud_counter: [0; 6],
            leaf_counter: 50,
            m_trophy_particle_id: crate::lawn::game_enums::PARTICLESYSTEMID_NULL,
            hand_reanim_id: crate::lawn::game_enums::REANIMATIONID_NONE,
        };

        // 创建按钮。C++ 中按钮位置由构造时的 Resize 设定、首帧 Update 的 TrackButton 修正；
        // 各按钮的 mBtnNoDraw/mMouseVisible/mButtonOffset 初始态对应 C++ GameSelector.cpp:77-281。
        // C++ MakeNewButton（GameButton.cpp:403-420）默认 mButtonOffsetX/Y=0、COLOR_BKG=White。
        sel.buttons.push(GameBtnData::new(
            GS_ADVENTURE,
            0,
            0,
            330,
            125,
            "IMAGE_REANIM_SELECTORSCREEN_ADVENTURE_BUTTON",
            "IMAGE_REANIM_SELECTORSCREEN_ADVENTURE_HIGHLIGHT",
            "IMAGE_REANIM_SELECTORSCREEN_ADVENTURE_HIGHLIGHT",
        ));
        // C++ GameSelector.cpp:89-90: mBtnNoDraw = true; mMouseVisible = false（动画期间由 reanim 轨道绘制）
        sel.buttons.last_mut().unwrap().btn_no_draw = true;
        sel.buttons.push(GameBtnData::new(
            GS_MINIGAME,
            0,
            125,
            320,
            130,
            "IMAGE_REANIM_SELECTORSCREEN_SURVIVAL_BUTTON",
            "IMAGE_REANIM_SELECTORSCREEN_SURVIVAL_HIGHLIGHT",
            "IMAGE_REANIM_SELECTORSCREEN_SURVIVAL_HIGHLIGHT",
        ));
        // C++ GameSelector.cpp:108-109
        sel.buttons.last_mut().unwrap().btn_no_draw = true;
        sel.buttons.push(GameBtnData::new(
            GS_PUZZLE,
            0,
            255,
            290,
            121,
            "IMAGE_REANIM_SELECTORSCREEN_CHALLENGES_BUTTON",
            "IMAGE_REANIM_SELECTORSCREEN_CHALLENGES_HIGHLIGHT",
            "IMAGE_REANIM_SELECTORSCREEN_CHALLENGES_HIGHLIGHT",
        ));
        // C++ GameSelector.cpp:127-128
        sel.buttons.last_mut().unwrap().btn_no_draw = true;
        sel.buttons.push(GameBtnData::new(
            GS_SURVIVAL,
            0,
            376,
            275,
            124,
            "IMAGE_REANIM_SELECTORSCREEN_VASEBREAKER_BUTTON",
            "IMAGE_REANIM_SELECTORSCREEN_VASEBREAKER_HIGHLIGHT",
            "IMAGE_REANIM_SELECTORSCREEN_VASEBREAKER_HIGHLIGHT",
        ));
        // C++ GameSelector.cpp:146-147
        sel.buttons.last_mut().unwrap().btn_no_draw = true;
        sel.buttons.push(GameBtnData::new(
            GS_ZOMBATAR,
            0,
            0,
            100,
            60,
            "IMAGE_BLANK",
            "IMAGE_BLANK",
            "IMAGE_BLANK",
        ));
        // C++ GameSelector.cpp:165-166
        sel.buttons.last_mut().unwrap().btn_no_draw = true;
        sel.buttons.push(GameBtnData::new(
            GS_ACHIEVEMENTS,
            20,
            h - 100,
            100,
            65,
            "IMAGE_SELECTORSCREEN_ACHIEVEMENTS_PEDESTAL",
            "IMAGE_SELECTORSCREEN_ACHIEVEMENTS_PEDESTAL_PRESS",
            "IMAGE_SELECTORSCREEN_ACHIEVEMENTS_PEDESTAL_PRESS",
        ));
        // C++ GameSelector.cpp:179: mBtnNoDraw = mHasTrophy（构造时 mHasTrophy=false）
        sel.buttons.push(GameBtnData::new(
            GS_ZEN_GARDEN,
            0,
            500,
            130,
            130,
            "IMAGE_SELECTORSCREEN_ZENGARDEN",
            "IMAGE_SELECTORSCREEN_ZENGARDENHIGHLIGHT",
            "IMAGE_SELECTORSCREEN_ZENGARDENHIGHLIGHT",
        ));
        // C++ GameSelector.cpp:192-193: 仅 mMouseVisible=false + mClip=false
        sel.buttons.push(GameBtnData::new(
            GS_OPTIONS,
            0,
            0,
            100,
            100,
            "IMAGE_SELECTORSCREEN_OPTIONS1",
            "IMAGE_SELECTORSCREEN_OPTIONS2",
            "IMAGE_SELECTORSCREEN_OPTIONS2",
        ));
        // C++ GameSelector.cpp:204-208: Resize(0,0,imgW,imgH+23); mBtnNoDraw=true; mButtonOffsetY=15
        {
            let btn = sel.buttons.last_mut().unwrap();
            btn.btn_no_draw = true;
            btn.height += 23;
            btn.button_offset_y = 15;
        }
        sel.buttons.push(GameBtnData::new(
            GS_HELP,
            0,
            100,
            100,
            120,
            "IMAGE_SELECTORSCREEN_HELP1",
            "IMAGE_SELECTORSCREEN_HELP2",
            "IMAGE_SELECTORSCREEN_HELP2",
        ));
        // C++ GameSelector.cpp:219-223: Resize(0,0,imgW,imgH+33); mBtnNoDraw=true; mButtonOffsetY=30
        {
            let btn = sel.buttons.last_mut().unwrap();
            btn.btn_no_draw = true;
            btn.height += 33;
            btn.button_offset_y = 30;
        }
        sel.buttons.push(GameBtnData::new(
            GS_QUIT,
            0,
            220,
            100,
            100,
            "IMAGE_SELECTORSCREEN_QUIT1",
            "IMAGE_SELECTORSCREEN_QUIT2",
            "IMAGE_SELECTORSCREEN_QUIT2",
        ));
        // C++ GameSelector.cpp:234-239: Resize(0,0,imgW+10,imgH+10); mBtnNoDraw=true;
        // mButtonOffsetX=5; mButtonOffsetY=5
        {
            let btn = sel.buttons.last_mut().unwrap();
            btn.btn_no_draw = true;
            btn.width += 10;
            btn.height += 10;
            btn.button_offset_x = 5;
            btn.button_offset_y = 5;
        }
        sel.buttons.push(GameBtnData::new(
            GS_CHANGE_USER,
            20,
            h - 30,
            250,
            30,
            "IMAGE_BLANK",
            "IMAGE_BLANK",
            "IMAGE_BLANK",
        ));
        // C++ GameSelector.cpp:251-252: mBtnNoDraw = true; mMouseVisible = false
        sel.buttons.last_mut().unwrap().btn_no_draw = true;
        sel.buttons.push(GameBtnData::new(
            GS_STORE,
            405,
            484,
            80,
            80,
            "IMAGE_SELECTORSCREEN_STORE",
            "IMAGE_SELECTORSCREEN_STOREHIGHLIGHT",
            "IMAGE_SELECTORSCREEN_STOREHIGHLIGHT",
        ));
        // C++ GameSelector.cpp:268: mMouseVisible = false（无 mBtnNoDraw）
        sel.buttons.push(GameBtnData::new(
            GS_ALMANAC,
            327,
            428,
            80,
            80,
            "IMAGE_SELECTORSCREEN_ALMANAC",
            "IMAGE_SELECTORSCREEN_ALMANACHIGHLIGHT",
            "IMAGE_SELECTORSCREEN_ALMANACHIGHLIGHT",
        ));
        // C++ GameSelector.cpp:281: mMouseVisible = false（无 mBtnNoDraw）
        sel.buttons.push(GameBtnData::new(
            GS_QUICK_PLAY,
            650,
            455,
            100,
            50,
            "",
            "",
            "",
        ));

        // 对应 C++ GameSelector::GameSelector 的 reanim 初始化（GameSelector.cpp:294-337）：
        // SelectorScreen reanim 播放 anim_open，隐藏 flower/leaf 轨，背景轨道单独赋渲染组；
        // 云×6 与花×3 附着于背景轨道，草叶循环播放。
        unsafe {
            if let Some(a_selector_reanim) = (*app).add_reanimation(
                0.5,
                0.5,
                0,
                crate::lawn::game_enums::ReanimationType::SelectorScreen as i32,
            ) {
                let a_reanim = &mut *a_selector_reanim;
                a_reanim.play_reanim(
                    "anim_open",
                    crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold,
                    0,
                    30.0,
                );
                a_reanim.assign_render_group_to_prefix(
                    "flower",
                    crate::todlib::reanimator::RENDER_GROUP_HIDDEN,
                );
                a_reanim.assign_render_group_to_prefix(
                    "leaf",
                    crate::todlib::reanimator::RENDER_GROUP_HIDDEN,
                );
                a_reanim.assign_render_group_to_track("SelectorScreen_BG", 1);
                sel.selector_reanim_id = (*app).reanimation_get_id(a_selector_reanim);
            }
            // 对应 C++ GameSelector.cpp:305-320：云×6（i>1 时用 anim_cloud(i+2)）
            for i in 0..6i32 {
                let a_anim_name = if i > 1 {
                    format!("anim_cloud{}", i + 2)
                } else {
                    format!("anim_cloud{}", i + 1)
                };
                if let Some(a_cloud) = (*app).add_reanimation(
                    0.5,
                    0.5,
                    0,
                    crate::lawn::game_enums::ReanimationType::SelectorScreen as i32,
                ) {
                    let a_cloud_ref = &mut *a_cloud;
                    a_cloud_ref.play_reanim(
                        &a_anim_name,
                        crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold,
                        0,
                        0.0,
                    );
                    sel.cloud_reanim_id[i as usize] = (*app).reanimation_get_id(a_cloud);
                    sel.cloud_counter[i as usize] =
                        crate::framework::common::rand_range_int(-6000, 2000);
                    if sel.cloud_counter[i as usize] < 0 {
                        a_cloud_ref.m_anim_time = -sel.cloud_counter[i as usize] as f32 / 6000.0;
                        a_cloud_ref.m_anim_rate = 0.5;
                        sel.cloud_counter[i as usize] = 0;
                    } else {
                        a_cloud_ref.m_anim_rate = 0.0;
                    }
                }
            }
            // 对应 C++ GameSelector.cpp:322-331：花×3，附着于背景轨道右侧
            for i in 0..3i32 {
                let a_anim_name = format!("anim_flower{}", i + 1);
                if let Some(a_flower) = (*app).add_reanimation(
                    0.5,
                    0.5,
                    0,
                    crate::lawn::game_enums::ReanimationType::SelectorScreen as i32,
                ) {
                    let a_flower_ref = &mut *a_flower;
                    a_flower_ref.play_reanim(
                        &a_anim_name,
                        crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold,
                        0,
                        0.0,
                    );
                    a_flower_ref.m_anim_rate = 0.0;
                    if sel.selector_reanim_id != crate::lawn::game_enums::REANIMATIONID_NONE {
                        if let Some(a_attach) = (*app).reanimation_get_mut(sel.selector_reanim_id) {
                            a_flower_ref
                                .attach_to_another_reanimation(a_attach, "SelectorScreen_BG_Right");
                        }
                    }
                    a_flower_ref.m_is_attachment = false;
                    sel.flower_reanim_id[i as usize] = (*app).reanimation_get_id(a_flower);
                }
            }
            // 对应 C++ GameSelector.cpp:333-337：草叶摆动（停帧待 update 触发）
            if let Some(a_leaf) = (*app).add_reanimation(
                0.5,
                0.5,
                0,
                crate::lawn::game_enums::ReanimationType::SelectorScreen as i32,
            ) {
                let a_leaf_ref = &mut *a_leaf;
                a_leaf_ref.play_reanim(
                    "anim_grass",
                    crate::todlib::reanimator::ReanimLoopType::Loop,
                    0,
                    6.0,
                );
                a_leaf_ref.m_anim_rate = 0.0;
                sel.leaf_reanim_id = (*app).reanimation_get_id(a_leaf);
            }
        }

        // 对应 C++ GameSelector.cpp:928-940：按钮绑定动画轨道（TrackButton）
        // mShowStartButton=false 时 Adventure 按钮绑定 Adventure_button 轨
        for btn in sel.buttons.iter_mut() {
            btn.track_name = match btn.id {
                GS_ADVENTURE => Some(if sel.show_start_button {
                    "SelectorScreen_StartAdventure_button"
                } else {
                    "SelectorScreen_Adventure_button"
                }),
                GS_MINIGAME => Some("SelectorScreen_Survival_button"),
                GS_PUZZLE => Some("SelectorScreen_Challenges_button"),
                GS_SURVIVAL => Some("SelectorScreen_ZenGarden_button"),
                GS_ZEN_GARDEN => Some("SelectorScreen_BG_Right"),
                GS_OPTIONS => Some("SelectorScreen_BG_Right"),
                GS_HELP => Some("SelectorScreen_BG_Right"),
                GS_QUIT => Some("SelectorScreen_BG_Right"),
                GS_ALMANAC => Some("SelectorScreen_BG_Right"),
                GS_STORE => Some("SelectorScreen_BG_Right"),
                GS_CHANGE_USER => Some("woodsign2"),
                GS_ZOMBATAR => Some("woodsign3"),
                GS_ACHIEVEMENTS => Some("SelectorScreen_BG_Left"),
                _ => None,
            };
            // 对应 C++ 928-940 各调用的 offset 参数
            let (ox, oy) = match btn.id {
                GS_ZEN_GARDEN => (100.0f32, 360.0f32),
                GS_OPTIONS => (494.0, 434.0),
                GS_HELP => (576.0, 458.0),
                GS_QUIT => (644.0, 469.0),
                GS_ALMANAC => (256.0, 387.0),
                GS_STORE => (334.0, 441.0),
                GS_CHANGE_USER => (24.0, 10.0),
                GS_ACHIEVEMENTS => (20.0, 480.0),
                _ => (0.0, 0.0),
            };
            btn.track_offset_x = ox;
            btn.track_offset_y = oy;
        }
        // 对应 C++ 构造尾部的 TrackButton 逐按钮初始定位（此时动画在首帧）
        unsafe {
            let a_app = app;
            for btn in sel.buttons.iter_mut() {
                if let Some(a_track) = btn.track_name {
                    track_button_position(
                        a_app,
                        sel.selector_reanim_id,
                        btn,
                        a_track,
                        btn.track_offset_x,
                        btn.track_offset_y,
                    );
                }
            }
        }

        sel
    }

    fn ensure_fonts(&mut self) {
        if !self.fonts_init {
            // C++ 601: PvzpDrawStringMatrix(g, Sexy::FONT_BRIANNETOD16, ...) —— 全局位图字体
            // （pak 内 data/BrianneTod16.txt 含 CJK 字形，欢迎木牌玩家名可直接渲染中文）
            let a_font_ptr =
                crate::framework::graphics::bitmap_font::font_global("FONT_BRIANNETOD16");
            if !a_font_ptr.is_null() {
                let a_font: Font = unsafe { std::ptr::read(a_font_ptr) };
                self.briannetod16 = Some(Box::new(a_font));
            } else {
                // [TRANSLATION_NOTE]: 字体资源不可用时退回占位渲染（无位图字形，画矩形块）
                let mut f = Font::new("Briannetod", 16);
                f.ascent = 13;
                f.font_height = 16;
                self.briannetod16 = Some(Box::new(f));
            }
            self.fonts_init = true;
        }
    }

    fn img_width(img: *mut Image) -> i32 {
        if img.is_null() {
            0
        } else {
            unsafe { (*img).get_width() }
        }
    }

    fn img_height(img: *mut Image) -> i32 {
        if img.is_null() {
            0
        } else {
            unsafe { (*img).get_height() }
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

        // 对应 C++ GameSelector.cpp:498-503: 同步时销毁既有奖杯粒子并清空 ID
        if let Some(a_trophy_particle) =
            unsafe { (*self.app).particle_try_to_get(self.m_trophy_particle_id) }
        {
            a_trophy_particle.particle_system_die();
            self.m_trophy_particle_id = crate::lawn::game_enums::PARTICLESYSTEMID_NULL;
        }

        // 对应 C++ GameSelector::SyncProfile 尾部（GameSelector.cpp:547-550）：
        //     SyncButtons(); AlmanacInitForPlayer(); BoardInitForPlayer();
        //     ReportAchievement::AchievementInitForPlayer(mApp);
        self.sync_buttons();
        crate::lawn::widget::almanac_dialog::almanac_init_for_player();
        crate::lawn::board::board_init_for_player();
        crate::lawn::widget::achievements_screen::ReportAchievement::achievement_init_for_player(
            Some(self.app),
        );
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
    /// 对应 C++ GameSelector::ClickedAdventure（GameSelector.cpp:1136-1179）
    fn clicked_adventure(&mut self) {
        let app = unsafe { &mut *self.app };
        unsafe {
            // C++ 1138-1153：试用版通关后重玩确认，拒绝则返回
            if app.is_trial_stage_locked() && (self.level >= 25 || app.has_finished_adventure()) {
                let a_result = app.lawn_message_box(
                    Dialogs::Message,
                    "[REPLAY_LEVEL_HEADER]",
                    "[REPLAY_LEVEL_BODY]",
                    "[DIALOG_BUTTON_YES]",
                    "[DIALOG_BUTTON_NO]",
                    crate::framework::widget::dialog::BUTTONS_YES_NO,
                );
                if a_result == crate::framework::widget::dialog::ID_NO {
                    return;
                }

                if let Some(ref mut pi) = app.player_info {
                    pi.m_level = 24;
                    pi.m_finished_adventure = 0;
                }
                // C++ 1151-1152：删除冒险存档
                let a_profile_id = app.player_info.as_ref().map_or(0, |p| p.m_id) as i32;
                let a_file_name = crate::lawn::lawn_common::get_saved_game_name(
                    GameMode::Adventure,
                    a_profile_id,
                );
                let _ = std::fs::remove_file(&a_file_name);
                let a_legacy_name = crate::lawn::lawn_common::get_legacy_saved_game_name(
                    GameMode::Adventure,
                    a_profile_id,
                );
                let _ = std::fs::remove_file(&a_legacy_name);
            }

            // C++ 1155-1156：停音乐 + 播失败曲
            if let Some(a_music) = app.music.as_mut() {
                a_music.stop_all_music();
            }
            app.play_sample(crate::framework::resources::ResourceId::SoundLosemusic as i32);
            self.starting_game = true;
            // C++ 1158-1170：全部按钮禁用
            for btn in self.buttons.iter_mut() {
                btn.disabled = true;
            }

            // C++ 1172-1178：僵尸手动画（PLAY_ONCE_AND_HOLD），rock 轨道忽略裁剪
            if let Some(a_hand) =
                app.add_reanimation(-70.0, 10.0, 0, ReanimationType::ZombieHand as i32)
            {
                let a_hand_ref = &mut *a_hand;
                a_hand_ref.m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold;
                self.hand_reanim_id = app.reanimation_get_id(a_hand);
                // C++ 1176-1178：名字以 "rock" 开头的轨道 mIgnoreClipRect = true
                if let Some(def_ptr) = a_hand_ref.m_definition {
                    for (a_i, a_track_def) in (*def_ptr).m_tracks.iter().enumerate() {
                        if a_track_def.m_name.to_lowercase().starts_with("rock") {
                            if let Some(a_ti) = a_hand_ref.m_track_instances.get_mut(a_i) {
                                a_ti.m_ignore_clip_rect = true;
                            }
                        }
                    }
                }
            }
            app.play_foley(crate::todlib::tod_foley::FoleyType::DirtRise as i32);
        }
    }

    /// 更新工具提示（对应 C++ UpdateTooltip）
    pub fn update_tooltip(&mut self) {
        unsafe {
            if !(*self.app).has_finished_adventure()
                || (*self.app)
                    .base
                    .dialog_map
                    .contains_key(&(Dialogs::Message as i32))
            {
                return;
            }

            if self.has_trophy {
                if self.mouse_x >= 50
                    && self.mouse_x < 135
                    && self.mouse_y >= 275
                    && self.mouse_y < 500
                {
                    if (*self.app).earned_gold_trophy() {
                        let finished = (*self.app)
                            .player_info
                            .as_ref()
                            .map_or(0, |p| p.m_finished_adventure);
                        let label = crate::lawn::lawn_app::LawnApp::pluralize(
                            finished,
                            "[GOLD_SUNFLOWER_TOOLTIP]",
                            "[GOLD_SUNFLOWER_TOOLTIP_PLURAL]",
                        );
                        self.tool_tip.set_label(&label);
                        self.tool_tip.m_x = 32;
                        self.tool_tip.m_y = 510;
                        self.tool_tip.m_visible = true;
                    } else {
                        self.tool_tip.set_label("[SILVER_SUNFLOWER_TOOLTIP]");
                        self.tool_tip.m_x = 20;
                        self.tool_tip.m_y = 495;
                        self.tool_tip.m_visible = true;
                    }
                    return;
                }
            }

            self.tool_tip.m_visible = false;
            self.tool_tip.update();
        }
    }

    /// 跟踪按钮位置（对应 C++ TrackButton）
    /// 根据动画轨道位置设置按钮位置
    pub fn track_button(
        &self,
        button: *mut crate::framework::widget::dialog_button::DialogButton,
        track_name: &str,
        offset_x: f32,
        offset_y: f32,
    ) {
        // 对应 C++ TrackButton：按选择器动画轨道位置设置按钮位置
        unsafe {
            if let Some(reanim) = (*self.app).reanimation_get(self.selector_reanim_id) {
                let a_track_index = reanim.find_track_index(track_name);
                let mut a_transform = crate::todlib::definition::ReanimatorTransform::default();
                reanim.get_current_transform(a_track_index, &mut a_transform);
                if !button.is_null() {
                    (&mut *button).x = (a_transform.m_trans_x + offset_x) as i32;
                    (&mut *button).y = (a_transform.m_trans_y + offset_y) as i32;
                }
            }
        }
    }

    /// 添加预览配置文件（对应 C++ AddPreviewProfiles）
    /// 创建内置预览存档（调试/预览用途）
    pub fn add_preview_profiles(&mut self) {
        // 对应 C++ AddPreviewProfiles：创建 5 个内置预览存档
        unsafe {
            let Some(mgr) = (*self.app).profile_mgr.as_mut() else {
                return;
            };

            if let Some(p) = mgr.add_profile("2 Night") {
                p.m_level = 11;
            }
            if let Some(p) = mgr.add_profile("3 Pool") {
                p.m_level = 21;
                p.m_has_unlocked_minigames = true;
                p.m_coins = 400;
                if let Some(v) = p.m_purchases.get_mut(StoreItem::PacketUpgrade as usize) {
                    *v = 1;
                }
            }
            if let Some(p) = mgr.add_profile("4 Fog") {
                p.m_level = 31;
                p.m_has_unlocked_minigames = true;
                p.m_has_unlocked_survival_mode = true;
                if let Some(v) = p.m_purchases.get_mut(StoreItem::PacketUpgrade as usize) {
                    *v = 2;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::PoolCleaner as usize) {
                    *v = 1;
                }
                p.m_coins = 400;
            }
            if let Some(p) = mgr.add_profile("5 Roof") {
                p.m_level = 41;
                p.m_has_unlocked_minigames = true;
                p.m_has_unlocked_puzzle_mode = true;
                p.m_has_unlocked_survival_mode = true;
                if let Some(v) = p.m_purchases.get_mut(StoreItem::PacketUpgrade as usize) {
                    *v = 2;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::PoolCleaner as usize) {
                    *v = 1;
                }
                p.m_coins = 500;
            }
            if let Some(p) = mgr.add_profile("Complete") {
                p.m_level = 1;
                p.m_finished_adventure = 1;
                p.m_has_unlocked_minigames = true;
                p.m_has_unlocked_puzzle_mode = true;
                p.m_has_unlocked_survival_mode = true;
                if let Some(v) = p.m_purchases.get_mut(StoreItem::PacketUpgrade as usize) {
                    *v = 2;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::PoolCleaner as usize) {
                    *v = 1;
                }
                p.m_coins = 1000;
            }
            if let Some(p) = mgr.add_profile("Full Unlock") {
                p.m_level = 1;
                p.m_finished_adventure = 2;
                p.add_coins(50000);
                if let Some(v) = p.m_purchases.get_mut(StoreItem::Fertilizer as usize) {
                    *v = 1000 + 5;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::BugSpray as usize) {
                    *v = 1000 + 5;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::Chocolate as usize) {
                    *v = 1000 + 5;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::TreeOfWisdom as usize) {
                    *v = 1000 + 5;
                }
                p.m_has_unlocked_minigames = true;
                p.m_has_unlocked_puzzle_mode = true;
                for item in [
                    StoreItem::PlantGatlingpea,
                    StoreItem::PlantTwinsunflower,
                    StoreItem::PlantGloomshroom,
                    StoreItem::PlantCattail,
                    StoreItem::PlantWintermelon,
                    StoreItem::PlantGoldMagnet,
                    StoreItem::PlantSpikerock,
                    StoreItem::PlantCobcannon,
                    StoreItem::PlantImitater,
                ] {
                    if let Some(v) = p.m_purchases.get_mut(item as usize) {
                        *v = 1;
                    }
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::PacketUpgrade as usize) {
                    *v = 3;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::PoolCleaner as usize) {
                    *v = 1;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::Phonograph as usize) {
                    *v = 1;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::GardeningGlove as usize) {
                    *v = 1;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::MushroomGarden as usize) {
                    *v = 1;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::WheelBarrow as usize) {
                    *v = 1;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::AquariumGarden as usize) {
                    *v = 1;
                }
                if let Some(v) = p.m_purchases.get_mut(StoreItem::TreeOfWisdom as usize) {
                    *v = 1;
                }
                // [TRANSLATION_NOTE]: C++ 中 mChallengeRecords 挑战记录设定与 SaveDetails()；
                // Rust 侧挑战记录索引与持久化未完全对齐，暂略
            }
        }
    }
}

impl GameSelectorImpl {
    /// 提取的覆盖绘制逻辑（供 draw_overlay 和 GameSelectorOverlayImpl 共享）
    /// 对应 C++ GameSelector::DrawOverlay（GameSelector.cpp:606-709）
    pub fn draw_overlay_internal(&mut self, g: &mut Graphics) {
        let app = unsafe { &*self.app };
        g.set_linear_blend(true);

        if app.player_info.is_none() {
            return;
        }

        // ---- 关卡数字（C++ 612-671） ----
        if !app.is_ice_demo() && !self.show_start_button {
            // 按下偏移（C++ 614-624：mIsDown && mIsOver 时 +1）
            let (a_offset_x, a_offset_y) = self
                .buttons
                .iter()
                .find(|b| b.id == GS_ADVENTURE)
                .map_or((0.0f32, 0.0f32), |b| {
                    if b.is_down && b.is_over {
                        (1.0, 1.0)
                    } else {
                        (0.0, 0.0)
                    }
                });

            // C++ 626-631: 从 SelectorScreen_BG_Right 轨道取基准坐标
            let (a_trans_area_x, a_trans_area_y) = unsafe {
                (*app)
                    .reanimation_get(self.selector_reanim_id)
                    .and_then(|a_selector| {
                        let a_right_idx = a_selector.find_track_index("SelectorScreen_BG_Right");
                        if a_right_idx >= 0 {
                            let mut a_transform =
                                crate::todlib::definition::ReanimatorTransform::default();
                            if a_selector.get_current_transform(a_right_idx, &mut a_transform) {
                                return Some((a_transform.m_trans_x, a_transform.m_trans_y));
                            }
                        }
                        None
                    })
                    .unwrap_or((0.0, 0.0))
            };
            let a_trans_area_x = a_trans_area_x + a_offset_x;
            let a_trans_area_y = a_trans_area_y + a_offset_y;
            let mut a_trans_area_x_f = a_trans_area_x;
            let mut a_trans_area_y_f = a_trans_area_y;
            let mut a_trans_sub_x = a_trans_area_x;
            let a_trans_sub_y = a_trans_area_y;

            let mut a_stage = clamp_int((self.level - 1) / 10 + 1, 1, 6);
            let mut a_sub = self.level - (a_stage - 1) * 10;
            if app.is_trial_stage_locked() && (self.level >= 25 || app.has_finished_adventure()) {
                // C++ 637-641：试用版锁定 → 显示 3-4
                a_stage = 3;
                a_sub = 4;
            } else {
                // C++ 643-656：关卡阶段微调
                if a_stage == 1 {
                    a_trans_area_y_f += 1.0;
                } else if a_stage == 4 {
                    a_trans_area_x_f -= 1.0;
                }
                if a_sub == 3 {
                    a_trans_sub_x -= 1.0;
                }
            }

            let level_img = get_image(app, "IMAGE_SELECTORSCREEN_LEVELNUMBERS");
            if !level_img.is_null() {
                g.set_colorize_images(true);
                // C++ 659: g->SetColor(mAdventureButton->mColors[COLOR_BKG])，默认 White
                let a_bkg = self
                    .buttons
                    .iter()
                    .find(|b| b.id == GS_ADVENTURE)
                    .map_or(Color::WHITE, |_| Color::WHITE);
                g.set_color(&a_bkg);
                unsafe {
                    // C++ 660: PvzpDrawImageCelF(..., aStage, 0) — theCelCol=aStage（0 基列号）
                    g.draw_image_cel_rc(
                        &*level_img,
                        (a_trans_area_x_f + 486.0) as i32,
                        (a_trans_area_y_f + 47.0) as i32,
                        a_stage,
                        0,
                    );
                    if a_sub < 10 {
                        // C++ 663: theCelCol=aSub
                        g.draw_image_cel_rc(
                            &*level_img,
                            (a_trans_sub_x + 509.0) as i32,
                            (a_trans_sub_y + 50.0) as i32,
                            a_sub,
                            0,
                        );
                    } else if a_sub == 10 {
                        // C++ 665-669: "10" 由 "1"+"0" 两 cel 拼成
                        g.draw_image_cel_rc(
                            &*level_img,
                            (a_trans_sub_x + 509.0) as i32,
                            (a_trans_sub_y + 50.0) as i32,
                            1,
                            0,
                        );
                        g.draw_image_cel_rc(
                            &*level_img,
                            (a_trans_sub_x + 518.0) as i32,
                            (a_trans_sub_y + 51.0) as i32,
                            0,
                            0,
                        );
                    }
                }
                g.set_colorize_images(false);
            }
        }

        // ---- 浇水提醒气泡（C++ 673-677） ----
        unsafe {
            let a_zen_visible = self
                .buttons
                .iter()
                .find(|b| b.id == GS_ZEN_GARDEN)
                .map_or(false, |b| b.visible);
            if a_zen_visible {
                let a_need_water = app.zen_garden.map_or(false, |zg| (*zg).plants_need_water());
                if a_need_water {
                    if let Some(a_btn) = self.buttons.iter().find(|b| b.id == GS_ZEN_GARDEN) {
                        let bubble_img = get_image(app, "IMAGE_PLANTSPEECHBUBBLE");
                        let drop_img = get_image(app, "IMAGE_WATERDROP");
                        if !bubble_img.is_null() {
                            g.draw_image_xy(&*bubble_img, a_btn.x + 106, a_btn.y + 36);
                        }
                        if !drop_img.is_null() {
                            g.draw_image_xy(&*drop_img, a_btn.x + 123, a_btn.y + 45);
                        }
                    }
                }
            }
        }

        // ---- 僵尸手（C++ 679-685：裁剪到 BOARD 区域，底部留 40px） ----
        unsafe {
            if self.hand_reanim_id != crate::lawn::game_enums::REANIMATIONID_NONE {
                if let Some(a_hand) = (*app).reanimation_get(self.hand_reanim_id) {
                    g.set_clip_rect_xywh(
                        0,
                        0,
                        crate::lawn::game_enums::BOARD_WIDTH,
                        crate::lawn::game_enums::BOARD_HEIGHT - 40,
                    );
                    a_hand.draw(g);
                    g.clear_clip_rect();
                }
            }
            // ---- 草叶（C++ 686） ----
            if self.leaf_reanim_id != crate::lawn::game_enums::REANIMATIONID_NONE {
                if let Some(a_leaf) = (*app).reanimation_get(self.leaf_reanim_id) {
                    a_leaf.draw(g);
                }
            }
            // ---- 花×3（C++ 687-688） ----
            for i in 0..3usize {
                if self.flower_reanim_id[i] != crate::lawn::game_enums::REANIMATIONID_NONE {
                    if let Some(a_flower) = (*app).reanimation_get(self.flower_reanim_id[i]) {
                        a_flower.draw(g);
                    }
                }
            }
        }

        // ---- 奖杯（C++ 690-706：位置取自 SelectorScreen_BG_Left 轨道） ----
        unsafe {
            let (a_left_x, a_left_y) = (*app)
                .reanimation_get(self.selector_reanim_id)
                .and_then(|a_selector| {
                    let a_left_idx = a_selector.find_track_index("SelectorScreen_BG_Left");
                    if a_left_idx >= 0 {
                        let mut a_transform =
                            crate::todlib::definition::ReanimatorTransform::default();
                        if a_selector.get_current_transform(a_left_idx, &mut a_transform) {
                            return Some((a_transform.m_trans_x, a_transform.m_trans_y));
                        }
                    }
                    None
                })
                .unwrap_or((0.0, 0.0));
            if self.has_trophy {
                let trophy_img = get_image(app, "IMAGE_SUNFLOWER_TROPHY");
                if !trophy_img.is_null() {
                    let cel = if app.earned_gold_trophy() { 1 } else { 0 };
                    // C++ 699-701: PvzpDrawImageCelF(..., aTransformLeft.mTransX + 12, +345, cel, 0)
                    g.draw_image_cel_rc(
                        &*trophy_img,
                        (a_left_x + 12.0) as i32,
                        (a_left_y + 345.0) as i32,
                        cel,
                        0,
                    );
                }

                // C++ 703-705: 奖杯闪光粒子
                if let Some(a_trophy_particle) =
                    (*self.app).particle_try_to_get(self.m_trophy_particle_id)
                {
                    a_trophy_particle.draw(g);
                }
            }
        }

        // ---- 工具提示（C++ 708） ----
        self.tool_tip.draw(g);
    }
}

// ============================================================
// WidgetImpl — 实现
// ============================================================
impl WidgetImpl for GameSelectorImpl {
    fn draw(&mut self, _widget: &Widget, g: &mut Graphics) {
        let app = unsafe { &*self.app };
        g.set_linear_blend(true);

        // 对应 C++ NewLawnButton 构造（GameButton.cpp:403-420）后由 ResourceManager
        // 已加载的图像指针直接赋给 mButtonImage/mOverImage/mDownImage；Rust 侧按钮图
        // 按名惰性解析（GameBtnData::resolve_images），此处每帧确保缓存就绪
        // （sync_buttons 切换 Adventure 图后缓存已清空，重解析命中新名字）。
        for btn in self.buttons.iter_mut() {
            btn.resolve_images(app);
        }

        // 对应 C++ GameSelector::Draw（GameSelector.cpp:553-563）：
        // 先画 BG 渲染组（背景轨道），再画云×6，再画 NORMAL 组（含木牌按钮轨道）
        unsafe {
            if self.selector_reanim_id != crate::lawn::game_enums::REANIMATIONID_NONE {
                if let Some(a_reanim) = (*app).reanimation_get(self.selector_reanim_id) {
                    a_reanim.draw_render_group(g, 1); // "SelectorScreen_BG"
                }
                for i in 0..6 {
                    if self.cloud_reanim_id[i] != REANIMATIONID_NULL {
                        if let Some(a_cloud) = (*app).reanimation_get(self.cloud_reanim_id[i]) {
                            a_cloud.draw(g);
                        }
                    }
                }
                if let Some(a_reanim) = (*app).reanimation_get(self.selector_reanim_id) {
                    a_reanim.draw_render_group(g, 0); // RENDER_GROUP_NORMAL
                }
            }
        }

        // 绘制按钮（C++ 中 NewLawnButton 为子控件由 WidgetManager 绘制；此处内化）。
        // 位置已由 Update 末尾的 TrackButton 按 reanim 轨道维护（对应 C++ 928-940 每帧调用），
        // mBtnNoDraw=true 的按钮（开门动画期间）不画，由 reanim 的按钮轨道图承担。
        for btn in &self.buttons {
            self.draw_button_at(g, btn, self.offset_x as f32, self.offset_y as f32);
        }

        // 绘制欢迎文字（C++ GameSelector.cpp:589-603，含滑动偏移 mX/mY）
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

        // 滑动动画（C++ GameSelector.cpp:751-760：Move(aNewX, aNewY) 即 mX/mY；
        // Rust 侧 GameSelector 的 mX/mY 由 offset_x/offset_y 承担，按钮/绘制均按其平移）
        if self.slide_counter > 0 {
            let new_x = tod_animate_curve(
                75,
                0,
                self.slide_counter,
                self.start_x,
                self.dest_x,
                TodCurves::EaseInOut,
            );
            let new_y = tod_animate_curve(
                75,
                0,
                self.slide_counter,
                self.start_y,
                self.dest_y,
                TodCurves::EaseInOut,
            );
            self.offset_x = new_x;
            self.offset_y = new_y;
            // C++: mZombatarWidget->Move(aNewX + BOARD_WIDTH, aNewY);
            //      mAchievementsWidget->mY = aNewY + mApp->mHeight;
            // [TRANSLATION_NOTE]: zombatar/achievements widget 未接入（见 added_to_manager 占位），移动省略
            self.slide_counter -= 1;
        }

        // 更新工具提示（C++ GameSelector.cpp:749 UpdateTooltip）
        self.update_tooltip();

        // 更新按钮悬停
        for btn in self.buttons.iter_mut() {
            btn.is_over = btn.visible && !btn.disabled && btn.contains(self.mouse_x, self.mouse_y);
        }

        // 游戏开始倒计时（C++ GameSelector.cpp:768-806）
        if self.starting_game {
            self.starting_game_counter += 1;
            if self.starting_game_counter > 450 {
                let app = unsafe { &mut *self.app };

                // C++: mApp->KillGameSelector()
                app.kill_game_selector();

                if app.is_ice_demo() {
                    app.pre_new_game(GameMode::ChallengeIceLevel, false);
                    return;
                }
                if app.is_first_time_adventure_mode() && self.level == 1 && !app.save_file_exists()
                {
                    app.pre_new_game(GameMode::Intro, false);
                    return;
                }
                // C++: mNeedsMagicTacoReward && mLevel == 35 → 商店开场后进冒险
                if app
                    .player_info
                    .as_ref()
                    .map_or(false, |pi| pi.m_needs_magic_taco_reward)
                    && self.level == 35
                {
                    let a_store = crate::lawn::lawn_app::LawnApp::show_store_screen(Some(
                        app as *mut LawnApp,
                    ));
                    if let Some(a_store_ptr) = a_store {
                        unsafe {
                            let a_store =
                                a_store_ptr as *mut crate::lawn::widget::store_screen::StoreScreen;
                            (*a_store).setup_for_intro(601);
                            // C++: aStore->WaitForResult(true)
                            // [TRANSLATION_NOTE]: StoreScreen 无 WaitForResult；SetupForIntro 已驱动开场，等待省略
                        }
                    }
                    app.pre_new_game(GameMode::Adventure, false);
                    return;
                }
                if self.should_do_zen_tuturial_before_adventure() {
                    app.pre_new_game(GameMode::ChallengeZenGarden, false);
                    if let Some(zg) = app.zen_garden {
                        unsafe {
                            (*zg).setup_for_zen_tutorial();
                        }
                    }
                    return;
                }
                app.pre_new_game(GameMode::Adventure, true);
                return;
            }

            // C++: mAdventureButton->SetColor(COLOR_BKG, counter%20<10 ? (80,80,80) : White)
            // Rust 侧以 disabled 标记灰显（draw_button_at 对 disabled 做颜色化），语义等价
            for btn in self.buttons.iter_mut() {
                if btn.id == GS_ADVENTURE {
                    btn.disabled = self.starting_game_counter % 20 >= 10;
                }
            }
            // C++: if (mStartingGameCounter == 125) mApp->PlaySample(SOUND_EVILLAUGH);
            if self.starting_game_counter == 125 {
                let app = unsafe { &mut *self.app };
                // [TRANSLATION_NOTE]: SOUND_EVILLAUGH 资源未注册，以 LoseMusic 近似（见 Foley 表）
                app.play_sample(crate::framework::resources::ResourceId::SoundLosemusic as i32);
            }
        }

        // 对应 C++ GameSelector.cpp:764-766: 更新奖杯闪光粒子
        if let Some(a_trophy_particle) =
            unsafe { (*self.app).particle_try_to_get(self.m_trophy_particle_id) }
        {
            a_trophy_particle.update();
        }

        // ---- 选择器状态机（对应 C++ GameSelector.cpp:808-890） ----
        {
            let app = unsafe { &mut *self.app };
            unsafe {
                if self.selector_reanim_id != crate::lawn::game_enums::REANIMATIONID_NONE {
                    let a_reanim_loop_count = (*app)
                        .reanimation_get(self.selector_reanim_id)
                        .map_or(0, |r| r.m_loop_count);
                    match self.selector_state {
                        SelectorAnimState::Open => {
                            // C++: if (mWidgetManager) mWidgetManager->RehupMouse();
                            if let Some(wm) = app.base.widget_manager {
                                (*wm).rehup_mouse();
                            }
                            if a_reanim_loop_count > 0 {
                                // C++ 816-820：动画播完后隐藏 reanim 按钮轨道
                                if let Some(a_selector) =
                                    (*app).reanimation_get_mut(self.selector_reanim_id)
                                {
                                    for a_track in [
                                        "SelectorScreen_Adventure_button",
                                        "SelectorScreen_StartAdventure_button",
                                        "SelectorScreen_Survival_button",
                                        "SelectorScreen_Challenges_button",
                                        "SelectorScreen_ZenGarden_button",
                                    ] {
                                        a_selector.assign_render_group_to_track(
                                            a_track,
                                            crate::todlib::reanimator::RENDER_GROUP_HIDDEN,
                                        );
                                    }
                                }
                                // C++ 821-830：按钮转为 widget 绘制
                                for btn in self.buttons.iter_mut() {
                                    if matches!(
                                        btn.id,
                                        GS_ADVENTURE
                                            | GS_MINIGAME
                                            | GS_PUZZLE
                                            | GS_SURVIVAL
                                            | GS_ZEN_GARDEN
                                            | GS_HELP
                                            | GS_OPTIONS
                                            | GS_QUIT
                                            | GS_ZOMBATAR
                                            | GS_ACHIEVEMENTS
                                            | GS_CHANGE_USER
                                    ) {
                                        btn.btn_no_draw = false;
                                    }
                                }
                                // C++ 845-852：无玩家档案时弹建用户对话框
                                if app.player_info.is_none() {
                                    app.do_create_user_dialog();
                                    // C++: if (gIsPartnerBuild) AddPreviewProfiles();
                                    // [TRANSLATION_NOTE]: gIsPartnerBuild 标识未定义，Partner 分支不进入
                                    self.selector_state = SelectorAnimState::NewUser;
                                } else {
                                    // C++ 855-856：播放木牌动画并进入 IDLE
                                    if let Some(a_selector) =
                                        (*app).reanimation_get_mut(self.selector_reanim_id)
                                    {
                                        a_selector.play_reanim(
                                            "anim_sign",
                                            crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold,
                                            0,
                                            30.0,
                                        );
                                    }
                                    self.selector_state = SelectorAnimState::Idle;
                                }

                                // C++ 859-860: if (mHasTrophy) AddTrophySparkle();
                                if self.has_trophy {
                                    self.add_trophy_sparkle();
                                }

                                // C++ 862-874：完成冒险提示弹窗
                                if let Some(ref mut pi) = app.player_info {
                                    if pi.m_needs_message_on_game_selector != 0 {
                                        pi.m_needs_message_on_game_selector = 0;
                                        app.write_current_user_config();
                                        app.lawn_message_box(
                                            Dialogs::Message,
                                            "[ADVENTURE_COMPLETE_HEADER]",
                                            "[ADVENTURE_COMPLETE_BODY]",
                                            "[DIALOG_BUTTON_OK]",
                                            "",
                                            BUTTONS_FOOTER,
                                        );
                                    }
                                }
                            }
                        }
                        SelectorAnimState::NewUser => {
                            // C++ 877-883：建用户对话框关闭后播 anim_sign
                            if !app
                                .base
                                .dialog_map
                                .contains_key(&(Dialogs::CreateUser as i32))
                            {
                                if let Some(a_selector) =
                                    (*app).reanimation_get_mut(self.selector_reanim_id)
                                {
                                    a_selector.play_reanim(
                                        "anim_sign",
                                        crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold,
                                        0,
                                        30.0,
                                    );
                                }
                                self.selector_state = SelectorAnimState::ShowSign;
                            }
                        }
                        SelectorAnimState::ShowSign => {
                            // C++ 884-887：木牌动画播完 → IDLE
                            if a_reanim_loop_count > 0 {
                                self.selector_state = SelectorAnimState::Idle;
                            }
                        }
                        SelectorAnimState::Idle => {}
                    }
                }
            }
        }

        // 对应 C++ GameSelector.cpp:892-919：云循环、草叶摆动、选择器动画推进
        // （C++ 侧 GameSelector 界面无 EffectSystem::Update 驱动，reanim 由本函数显式推进）
        let app = unsafe { &mut *self.app };
        unsafe {
            for i in 0..6usize {
                let a_cloud = (*app).reanimation_get_mut(self.cloud_reanim_id[i]);
                let Some(a_cloud) = a_cloud else { continue };
                if self.cloud_counter[i] > 0 {
                    self.cloud_counter[i] -= 1;
                    if self.cloud_counter[i] == 0 {
                        a_cloud.m_loop_count = 0;
                        a_cloud.m_anim_time = 0.0;
                        a_cloud.m_anim_rate = 0.5;
                    }
                } else if a_cloud.m_loop_count > 0 {
                    self.cloud_counter[i] = crate::framework::common::rand_range_int(2000, 4000);
                }
            }
            // 对应 C++ 922-923: for (int i = 0; i < 6; i++) ReanimationGet(mCloudReanimID[i])->Update();
            for i in 0..6usize {
                if let Some(a_cloud) = (*app).reanimation_get_mut(self.cloud_reanim_id[i]) {
                    a_cloud.update();
                }
            }
            if let Some(a_selector) = (*app).reanimation_get_mut(self.selector_reanim_id) {
                a_selector.update();
            }
            // 对应 C++ 924-926: 僵尸手动画更新（ClickedAdventure 创建）
            if self.hand_reanim_id != crate::lawn::game_enums::REANIMATIONID_NONE {
                if let Some(a_hand) = (*app).reanimation_get_mut(self.hand_reanim_id) {
                    a_hand.update();
                }
            }
            if self.leaf_reanim_id != crate::lawn::game_enums::REANIMATIONID_NONE {
                // 对应 C++ 911-914：先取草叶应贴的背景轨道位置（不可变借用）
                let a_leaf_pos = if self.selector_reanim_id
                    != crate::lawn::game_enums::REANIMATIONID_NONE
                {
                    (*app)
                        .reanimation_get(self.selector_reanim_id)
                        .and_then(|a_selector| {
                            let a_leaf_track =
                                a_selector.find_track_index("SelectorScreen_BG_Right");
                            if a_leaf_track >= 0 {
                                let mut a_transform =
                                    crate::todlib::definition::ReanimatorTransform::default();
                                if a_selector.get_current_transform(a_leaf_track, &mut a_transform)
                                {
                                    return Some((
                                        a_transform.m_trans_x - 71.0,
                                        a_transform.m_trans_y - 41.0,
                                    ));
                                }
                            }
                            None
                        })
                } else {
                    None
                };
                if let Some(a_leaf) = (*app).reanimation_get_mut(self.leaf_reanim_id) {
                    a_leaf.update();
                    if let Some((px, py)) = a_leaf_pos {
                        a_leaf.set_position(px, py);
                    }
                    self.leaf_counter -= 1;
                    if self.leaf_counter == 0 {
                        let a_rate = crate::framework::common::rand_range_float(3.0, 12.0);
                        a_leaf.play_reanim(
                            "anim_grass",
                            crate::todlib::reanimator::ReanimLoopType::Loop,
                            20,
                            a_rate,
                        );
                        self.leaf_counter = crate::framework::common::rand_range_int(200, 400);
                    }
                }
            }

            // ---- 每帧 TrackButton（对应 C++ GameSelector.cpp:928-940） ----
            // 按钮位置每帧从 reanim 轨道取，保证跟随开门动画与滑动偏移
            {
                let a_app = app as *mut LawnApp;
                let a_sel_ptr = self as *mut GameSelectorImpl;
                // 以只读方式取轨道变换（避免与上面可变借用冲突，已无借用存续）
                for a_btn in (*a_sel_ptr).buttons.iter_mut() {
                    if let Some(a_track) = a_btn.track_name {
                        track_button_position(
                            a_app,
                            (*a_sel_ptr).selector_reanim_id,
                            a_btn,
                            a_track,
                            a_btn.track_offset_x,
                            a_btn.track_offset_y,
                        );
                    }
                }
            }
            // 对应 C++ 941-942：woodsign2/3 悬停/按下时替换按下图
            if self.selector_reanim_id != crate::lawn::game_enums::REANIMATIONID_NONE {
                let a_change_user_over = self
                    .buttons
                    .iter()
                    .find(|b| b.id == GS_CHANGE_USER)
                    .map_or(false, |b| b.is_over || b.is_down);
                let a_zombatar_over = self
                    .buttons
                    .iter()
                    .find(|b| b.id == GS_ZOMBATAR)
                    .map_or(false, |b| b.is_over || b.is_down);
                let a_press2 = get_image(app, "IMAGE_REANIM_SELECTORSCREEN_WOODSIGN2_PRESS");
                let a_press3 = get_image(app, "IMAGE_REANIM_SELECTORSCREEN_WOODSIGN3_PRESS");
                if let Some(a_selector) = (*app).reanimation_get_mut(self.selector_reanim_id) {
                    a_selector.set_image_override(
                        "woodsign2",
                        if a_change_user_over {
                            a_press2
                        } else {
                            std::ptr::null_mut()
                        },
                    );
                    a_selector.set_image_override(
                        "woodsign3",
                        if a_zombatar_over {
                            a_press3
                        } else {
                            std::ptr::null_mut()
                        },
                    );
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
            for b in self.buttons.iter_mut() {
                b.is_down = false;
            }
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
                // C++ 1224-1226: mApp->ConfirmQuit()
                app.confirm_quit();
            }
            GS_HELP => {
                // C++ 1227-1230
                app.kill_game_selector();
                app.show_award_screen(AwardType::HelpZombieNote as i32, false);
            }
            GS_OPTIONS => {
                // C++ 1231-1233: mApp->DoNewOptions(true)
                app.do_new_options(true);
            }
            GS_CHANGE_USER => {
                // C++ 1234-1236: mApp->DoUserDialog()
                app.do_user_dialog();
            }
            GS_STORE => {
                // C++ 1237-1250：开商店，等待返回后按 mGoToTreeNow 决定是否进智慧树
                let a_store =
                    crate::lawn::lawn_app::LawnApp::show_store_screen(Some(app as *mut LawnApp));
                if let Some(a_store_ptr) = a_store {
                    unsafe {
                        let a_store =
                            a_store_ptr as *mut crate::lawn::widget::store_screen::StoreScreen;
                        // C++: aStore->WaitForResult(true)
                        // [TRANSLATION_NOTE]: StoreScreen 无 WaitForResult 循环；mGoToTreeNow 分支按状态读取
                        if (*a_store).go_to_tree_now {
                            app.kill_game_selector();
                            app.pre_new_game(GameMode::ChallengeTreeOfWisdom, false);
                        } else if let Some(a_music) = app.music.as_mut() {
                            a_music.make_sure_music_is_playing(MusicTune::TitleCrazyDaveMainTheme);
                        }
                    }
                }
            }
            GS_ALMANAC => {
                // C++ 1251-1254：打开图鉴后恢复标题音乐
                app.do_almanac_dialog(SeedType::None, ZombieType::Invalid);
                if let Some(a_music) = app.music.as_mut() {
                    a_music.make_sure_music_is_playing(MusicTune::TitleCrazyDaveMainTheme);
                }
            }
            GS_ZEN_GARDEN => {
                // C++ 1255-1260：进禅境花园，必要时先做禅教程
                app.kill_game_selector();
                app.pre_new_game(GameMode::ChallengeZenGarden, false);
                if self.should_do_zen_tuturial_before_adventure() {
                    if let Some(zg) = app.zen_garden {
                        unsafe {
                            (*zg).setup_for_zen_tutorial();
                        }
                    }
                }
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
    /// 对应 C++ NewLawnButton::Draw（GameButton.cpp:328-393）：
    /// COLOR_BKG 颜色化 + 按下时 mTranslateX/Y(1,1) 偏移。
    fn draw_button_at(&self, g: &mut Graphics, btn: &GameBtnData, offset_x: f32, offset_y: f32) {
        if btn.btn_no_draw || !btn.visible {
            return;
        }
        let img = btn.get_draw_image();
        if img.is_null() {
            return;
        }

        // C++ 中按钮为子控件，位置含容器 mX/mY 平移（滑动时按钮随容器移动）
        let draw_x = btn.x + btn.button_offset_x + offset_x as i32;
        let draw_y = btn.y + btn.button_offset_y + offset_y as i32;

        // 对应 DrawButtonImage 的 mTranslateX/Y(1,1)：按下时右下偏移 1px
        let tx = if btn.is_down && btn.is_over && !btn.disabled {
            1
        } else {
            0
        };
        let ty = if btn.is_down && btn.is_over && !btn.disabled {
            1
        } else {
            0
        };

        g.set_colorize_images(true);
        if btn.disabled {
            // SyncButtons 对锁定按钮 SetColor(COLOR_BKG, Color(128,128,128))
            g.set_color(&Color::new(128, 128, 128, 255));
        } else {
            // NewLawnButton 构造默认 SetColor(COLOR_BKG, Color::White)
            g.set_color(&Color::WHITE);
        }
        unsafe {
            g.draw_image_xy(&*img, draw_x + tx, draw_y + ty);
        }
        g.set_colorize_images(false);
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::AddedToManager
    pub fn added_to_manager(&mut self, _the_widget_manager: *mut WidgetManager) {
        // C++: WidgetManager::AddWidget(mZombatarWidget) + AddWidget(mAchievementsWidget)
        // [TRANSLATION_NOTE]: zombatar/achievements widget 未接入（zombatar.rs 刚起步），暂占位
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::RemovedFromManager
    pub fn removed_from_manager(&mut self, _the_widget_manager: *mut WidgetManager) {
        // C++: RemoveWidget(mZombatarWidget) + RemoveWidget(mAchievementsWidget)；同上占位
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::ButtonPress
    pub fn button_press(&mut self, the_id: i32) {
        // C++: 主菜单栏按钮播 SOUND_GRAVEBUTTON，其余播 SOUND_TAP
        let a_is_stone_button = the_id == 100  // GameSelector_Adventure
            || the_id == 101                 // GameSelector_Minigame
            || the_id == 102                 // GameSelector_Puzzle
            || the_id == 110                 // GameSelector_Survival
            || the_id == 111; // GameSelector_Zombatar
        if let Some(app) = unsafe { self.app.as_mut() } {
            unsafe {
                let a_sound = if a_is_stone_button {
                    crate::framework::resources::ResourceId::SoundGravebutton
                } else {
                    crate::framework::resources::ResourceId::SoundTap
                };
                (*app).play_sample(a_sound as i32);
            }
        }
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::ButtonMouseEnter
    pub fn button_mouse_enter(&mut self, the_id: i32) {
        let a_locked = (the_id == 101 && self.minigames_locked)
            || (the_id == 102 && self.puzzle_locked)
            || (the_id == 110 && self.survival_locked);
        if a_locked {
            return;
        }
        // C++: mApp->PlayFoley(FOLEY_BLEEP)；Rust FoleyType 以 Beep 对应
        if let Some(app) = unsafe { self.app.as_mut() } {
            unsafe {
                (*app).play_foley(crate::todlib::tod_foley::FoleyType::Beep as i32);
            }
        }
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::OrderInManagerChanged
    pub fn order_in_manager_changed(&mut self) {
        // C++: PutInfront(mAchievementsWidget, this) + BringToFront(mZombatarWidget)
        // [TRANSLATION_NOTE]: Rust WidgetManager 无 PutInfront/BringToFront，暂占位
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::SlideTo
    pub fn slide_to(&mut self, the_x: i32, the_y: i32) {
        if self.slide_counter > 0 {
            return;
        }
        self.slide_counter = 75;
        self.dest_x = the_x;
        self.dest_y = the_y;
        self.start_x = self.offset_x;
        self.start_y = self.offset_y;
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::ShowZombatarScreen
    pub fn show_zombatar_screen(&mut self) {
        // C++: if (!mZombatarWidget) return;
        //      if (mApp->mPlayerInfo && !mApp->mPlayerInfo->mZombatarAccepted) ShowZombatarTOS();
        //      else mZombatarWidget->Open();
        // [TRANSLATION_NOTE]: Rust 侧 zombatar widget / ShowZombatarTOS 未接入，读取状态后占位
        let _a_accepted = if let Some(app) = unsafe { self.app.as_mut() } {
            unsafe {
                (*app)
                    .player_info
                    .as_ref()
                    .map_or(true, |pi| pi.m_zombatar_accepted != 0)
            }
        } else {
            true
        };
    }

    /// 对应 C++ GameSelector::AddTrophySparkle（GameSelector.cpp:478-483）
    pub fn add_trophy_sparkle(&mut self) {
        // C++: AddPvzpParticle(85.0f, 330.0f, RENDER_LAYER_TOP, PARTICLE_TROPHY_SPARKLE) + ParticleGetID
        if let Some(app) = unsafe { self.app.as_mut() } {
            let a_trophy_particle = app.add_tod_particle(
                85.0,
                330.0,
                crate::lawn::game_enums::RENDER_LAYER_TOP,
                crate::lawn::game_enums::ParticleEffect::TrophySparkle as i32,
            );
            if let Some(a_particle_ptr) = a_trophy_particle {
                self.m_trophy_particle_id = app.particle_get_id(a_particle_ptr);
            }
        }
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::ShowAchievementsScreen
    pub fn show_achievements_screen(&mut self) {
        // C++: SlideTo(0, -mApp->mHeight) + mWidgetManager->SetFocus(mAchievementsWidget)
        let a_height = if let Some(app) = unsafe { self.app.as_mut() } {
            unsafe { (*app).base.height }
        } else {
            0
        };
        self.slide_to(0, -a_height);
        // [TRANSLATION_NOTE]: mAchievementsWidget 未接入，SetFocus 省略
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::ShouldDoZenTuturialBeforeAdventure
    pub fn should_do_zen_tuturial_before_adventure(&self) -> bool {
        // C++: !HasFinishedAdventure && PlayerInfo->GetLevel() == 45 && mNumPottedPlants == 0
        if let Some(app) = unsafe { self.app.as_mut() } {
            unsafe {
                if (*app).has_finished_adventure() {
                    return false;
                }
                if let Some(pi) = (*app).player_info.as_ref() {
                    return pi.m_level == 45 && pi.m_num_potted_plants == 0;
                }
            }
        }
        false
    }

    /// [TRANSLATION_NOTE]: C++ GameSelector::ButtonDepress
    pub fn button_depress(&mut self, the_id: i32) {
        if self.slide_counter > 0 {
            return;
        }

        // 锁定模式弹窗（C++ LawnMessageBox → Rust do_dialog + wait_for_result）
        let a_locked_message: Option<&str> = if the_id == 101 && self.minigames_locked {
            Some("[MINIGAME_LOCKED_MESSAGE]")
        } else if the_id == 102 && self.puzzle_locked {
            Some("[PUZZLE_LOCKED_MESSAGE]")
        } else if the_id == 110 && self.survival_locked {
            Some("[SURVIVAL_LOCKED_MESSAGE]")
        } else {
            None
        };
        if let Some(a_msg) = a_locked_message {
            if let Some(app) = unsafe { self.app.as_mut() } {
                unsafe {
                    let a_dialog = (*app).do_dialog(
                        48,
                        true,
                        "[MODE_LOCKED]",
                        a_msg,
                        "[DIALOG_BUTTON_OK]",
                        BUTTONS_FOOTER,
                    );
                    if let Some(d) = a_dialog {
                        let _ = (&mut *d).wait_for_result(true);
                    }
                }
            }
            return;
        }

        match the_id {
            100 => {
                // GameSelector_Adventure
                self.clicked_adventure();
            }
            101 | 102 | 110 => {
                // Minigame / Puzzle / Survival → 挑战选择页
                if let Some(app) = unsafe { self.app.as_mut() } {
                    unsafe {
                        (*app).kill_game_selector();
                        let a_page = if the_id == 101 {
                            ChallengePage::Challenge
                        } else if the_id == 102 {
                            ChallengePage::Puzzle
                        } else {
                            ChallengePage::Survival
                        };
                        (*app).show_challenge_screen(a_page as i32);
                    }
                }
            }
            105 => {
                // GameSelector_Quit
                if let Some(app) = unsafe { self.app.as_mut() } {
                    unsafe {
                        (*app).confirm_quit();
                    }
                }
            }
            104 => {
                // GameSelector_Help
                if let Some(app) = unsafe { self.app.as_mut() } {
                    unsafe {
                        (*app).kill_game_selector();
                        (*app).show_award_screen(AwardType::HelpZombieNote as i32, false);
                    }
                }
            }
            103 => {
                // GameSelector_Options
                // C++: mApp->DoNewOptions(true)
                if let Some(app) = unsafe { self.app.as_mut() } {
                    (*app).do_new_options(true);
                }
            }
            106 => {
                // GameSelector_ChangeUser
                // C++: mApp->DoUserDialog()
                if let Some(app) = unsafe { self.app.as_mut() } {
                    (*app).do_user_dialog();
                }
            }
            107 => {
                // GameSelector_Store
                if let Some(app) = unsafe { self.app.as_mut() } {
                    unsafe {
                        let a_store = LawnApp::show_store_screen(Some(app as *const _ as *mut _));
                        // [TRANSLATION_NOTE]: C++ WaitForResult(true) 模态等待；Rust 商店为非模态 widget 驱动
                        let a_go_to_tree = a_store.map_or(false, |s| {
                            (*(s as *mut crate::lawn::widget::store_screen::StoreScreen))
                                .go_to_tree_now
                        });
                        if a_go_to_tree {
                            (*app).kill_game_selector();
                            (*app).pre_new_game(GameMode::ChallengeTreeOfWisdom, false);
                        } else if let Some(music) = (*app).music.as_mut() {
                            music.make_sure_music_is_playing(MusicTune::TitleCrazyDaveMainTheme);
                        }
                    }
                }
            }
            108 => {
                // GameSelector_Almanac
                if let Some(app) = unsafe { self.app.as_mut() } {
                    unsafe {
                        // C++: DoAlmanacDialog()->WaitForResult(true)；Rust 版返回 ()
                        (*app).do_almanac_dialog(SeedType::None, ZombieType::Invalid);
                        if let Some(music) = (*app).music.as_mut() {
                            music.make_sure_music_is_playing(MusicTune::TitleCrazyDaveMainTheme);
                        }
                    }
                }
            }
            109 => {
                // GameSelector_ZenGarden
                if let Some(app) = unsafe { self.app.as_mut() } {
                    unsafe {
                        (*app).kill_game_selector();
                        (*app).pre_new_game(GameMode::ChallengeZenGarden, false);
                        if self.should_do_zen_tuturial_before_adventure() {
                            if let Some(zg) = (*app).zen_garden {
                                (*zg).setup_for_zen_tutorial();
                            }
                        }
                    }
                }
            }
            111 => {
                // GameSelector_Zombatar
                self.show_zombatar_screen();
            }
            112 => {
                // GameSelector_Achievements
                self.show_achievements_screen();
            }
            _ => {}
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
