// PvZ Portable Rust 翻译 — AchievementsScreen（成就界面）
// 对应 C++ src/Lawn/Widget/AchievementsScreen.h / AchievementsScreen.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::framework::rect::Rect;
use crate::lawn::game_enums::*;

/// 成就 ID 枚举（对应 C++ AchievementId）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AchievementId {
    HomeSecurity = 0,
    NovelPeasPrize,
    BetterOffDead,
    ChinaShop,
    Spudow,
    Explodonator,
    Morticulturalist,
    DontPea,
    RollSomeHeads,
    Grounded,
    Zombologist,
    PennyPincher,
    SunnyDays,
    PopcornParty,
    GoodMorning,
    NoFungusAmongUs,
    BeyondTheGrave,
    Immortal,
    ToweringWisdom,
    MustacheMode,
    MaxAchievements,
}

pub const MAX_ACHIEVEMENTS: usize = AchievementId::MaxAchievements as usize;

/// 成就条目（对应 C++ AchievementItem）
pub struct AchievementItem {
    pub name: String,
    pub description: String,
}

/// 成就界面 Widget（对应 C++ AchievementsWidget）
pub struct AchievementsWidget {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub scroll_direction: i32,
    pub more_rock_rect: Rect,
    pub scroll_value: i32,
    pub scroll_decay: i32,
    pub default_scroll_value: i32,
    pub did_press_more_button: bool,
}

impl AchievementsWidget {
    pub fn new() -> Self {
        AchievementsWidget {
            app: None,
            scroll_direction: 0,
            more_rock_rect: Rect::new(0, 0, 0, 0),
            scroll_value: 0,
            scroll_decay: 0,
            default_scroll_value: 0,
            did_press_more_button: false,
        }
    }

    pub fn update(&mut self) {
        // TODO: 从 AchievementsScreen.cpp 翻译
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // TODO: 从 AchievementsScreen.cpp 翻译
    }

    pub fn key_down(&mut self, _key: KeyCode) {
        // TODO: 从 AchievementsScreen.cpp 翻译
    }

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // TODO: 从 AchievementsScreen.cpp 翻译
    }

    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // TODO: 从 AchievementsScreen.cpp 翻译
    }

    pub fn mouse_wheel(&mut self, _delta: i32) {
        // TODO: 从 AchievementsScreen.cpp 翻译
    }
}

impl Default for AchievementsWidget {
    fn default() -> Self {
        AchievementsWidget::new()
    }
}

/// 成就报告（对应 C++ ReportAchievement，所有方法均为静态）
pub struct ReportAchievement;

impl ReportAchievement {
    pub fn give_achievement(app: Option<*mut crate::lawn::lawn_app::LawnApp>, _achievement: i32, _force_give: bool) {
        // TODO: 从 AchievementsScreen.cpp 翻译
    }

    pub fn achievement_init_for_player(app: Option<*mut crate::lawn::lawn_app::LawnApp>) {
        // TODO: 从 AchievementsScreen.cpp 翻译
    }
}

// --- 全局变量 ---
// 对应 C++: extern AchievementItem gAchievementList[MAX_ACHIEVEMENTS];
// 后续从 AchievementsScreen.cpp 翻译具体数据
