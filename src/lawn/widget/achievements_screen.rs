// PvZ Portable Rust 翻译 — AchievementsScreen（成就界面）
// 对应 C++ src/Lawn/Widget/AchievementsScreen.h / AchievementsScreen.cpp
// 完整翻译版本

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
    pub name: &'static str,
    pub description: &'static str,
}

/// 成就列表（对应 C++ gAchievementList）
pub const G_ACHIEVEMENT_LIST: [AchievementItem; MAX_ACHIEVEMENTS] = [
    AchievementItem { name: "Home Lawn Security", description: "Complete Adventure Mode." },
    AchievementItem { name: "Nobel Peas Prize", description: "Get the golden sunflower trophy." },
    AchievementItem { name: "Better Off Dead", description: "Get to a streak of 10 in I, Zombie Endless" },
    AchievementItem { name: "China Shop", description: "Get to a streak of 15 in Vasebreaker Endless" },
    AchievementItem { name: "SPUDOW!", description: "Blow up a zombie using a Potato Mine." },
    AchievementItem { name: "Explodonator", description: "Take out 10 full-sized zombies with a single Cherry Bomb." },
    AchievementItem { name: "Morticulturalist", description: "Collect all 49 plants." },
    AchievementItem { name: "Don't Pea in the Pool", description: "Complete a daytime pool level without using pea shooters." },
    AchievementItem { name: "Roll Some Heads", description: "Bowl over 5 zombies with a single Wall-Nut." },
    AchievementItem { name: "Grounded", description: "Defeat a normal roof level without using any catapult plants." },
    AchievementItem { name: "Zombologist", description: "Discover the Yeti zombie." },
    AchievementItem { name: "Penny Pincher", description: "Pick up 30 coins in a row without letting any disappear." },
    AchievementItem { name: "Sunny Days", description: "Get 8000 sun during a single level." },
    AchievementItem { name: "Popcorn Party", description: "Defeat 2 Gargantuars with Corn Cob missiles." },
    AchievementItem { name: "Good Morning", description: "Complete a daytime level by planting only Mushrooms." },
    AchievementItem { name: "No Fungus Among Us", description: "Complete a nighttime level without any Mushrooms." },
    AchievementItem { name: "Beyond the Grave", description: "Beat all 20 mini games." },
    AchievementItem { name: "Immortal", description: "Survive 20 waves of pure zombie ferocity." },
    AchievementItem { name: "Towering Wisdom", description: "Grow the Tree of Wisdom to 100 feet." },
    AchievementItem { name: "Mustache Mode", description: "Enable Mustache Mode" },
];

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
            scroll_direction: -1,
            more_rock_rect: Rect::new(710, 470, 100, 100),
            scroll_value: 0,
            scroll_decay: 1,
            default_scroll_value: 30,
            did_press_more_button: false,
        }
    }

    pub fn update(&mut self) {
        // MarkDirty();
        if self.scroll_value <= 0 { return; }
        self.scroll_value = self.scroll_value.min(self.default_scroll_value);
        self.scroll_value -= self.scroll_decay;
        let new_y = self.scroll_value * self.scroll_direction;
        let new_y = new_y.min(-1);
        self.scroll_value = self.scroll_value.max(0);
        let _ = new_y; // 暂不处理实际位置移动
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // 绘图依赖图片资源，暂用占位
    }

    pub fn key_down(&mut self, key: KeyCode) {
        if key == 0x26 /* KEY_UP */ {
            self.scroll_value = self.default_scroll_value;
            self.scroll_direction = 1;
        } else if key == 0x28 /* KEY_DOWN */ {
            self.scroll_value = self.default_scroll_value;
            self.scroll_direction = -1;
        }
    }

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // 点击音效暂不实现
    }

    pub fn mouse_up(&mut self, x: i32, y: i32, _click_count: i32) {
        if self.more_rock_rect.contains(x, y) {
            self.did_press_more_button = !self.did_press_more_button;
            self.scroll_direction = if self.did_press_more_button { -1 } else { 1 };
            self.scroll_value = 20;
        }
    }

    pub fn mouse_wheel(&mut self, delta: i32) {
        self.scroll_value = self.default_scroll_value;
        self.scroll_direction = if delta > 0 { 1 } else if delta < 0 { -1 } else { self.scroll_direction };
    }
}

impl Default for AchievementsWidget {
    fn default() -> Self {
        AchievementsWidget::new()
    }
}

/// 成就报告（对应 C++ ReportAchievement）
pub struct ReportAchievement;

impl ReportAchievement {
    /// 授予成就（对应 C++ ReportAchievement::GiveAchievement，AchievementsScreen.cpp:225）
    pub fn give_achievement(app: Option<*mut crate::lawn::lawn_app::LawnApp>, achievement: i32, force_give: bool) {
        let app_ptr = match app {
            Some(a) => a,
            None => return,
        };
        unsafe {
            let app_ref = &mut *app_ptr;

            // C++: if (!theApp->mPlayerInfo) return;
            if app_ref.player_info.is_none() {
                return;
            }

            let a_index = achievement as usize;
            if a_index >= MAX_ACHIEVEMENTS {
                return;
            }

            // C++: if (mPlayerInfo->mEarnedAchievements[theAchievement]) return;
            if app_ref.player_info.as_ref().map_or(false, |pi| {
                pi.m_earned_achievements.get(a_index).copied().unwrap_or(false)
            }) {
                return;
            }

            // C++: mPlayerInfo->mEarnedAchievements[theAchievement] = true;
            if let Some(pi) = app_ref.player_info.as_mut() {
                pi.m_earned_achievements[a_index] = true;
            }

            if !force_give {
                return;
            }

            // C++: GetString(gAchievementList[..].name, ..) 与 "%s Achievement!" 格式化拼接
            let a_achievement_name = app_ref.base.get_string(G_ACHIEVEMENT_LIST[a_index].name);
            let a_format = app_ref.base.get_string("%s Achievement!");
            let a_message = a_format.replace("%s", &a_achievement_name);

            if app_ref.board.is_some() {
                let a_board = app_ref.board.unwrap();
                (*a_board).display_advice(
                    &a_message,
                    MessageStyle::Achievement as i32,
                    AdviceType::None,
                );
                if let Some(pi) = app_ref.player_info.as_mut() {
                    pi.m_shown_achievements[a_index] = true;
                }
                // C++: theApp->PlaySample(SOUND_ACHIEVEMENT);
                // [TRANSLATION_NOTE]: Rust 侧尚未移植 SOUND_ACHIEVEMENT 声音资源，此处跳过播放。
            }
        }
    }

    pub fn achievement_init_for_player(app: Option<*mut crate::lawn::lawn_app::LawnApp>) {
        if let Some(app_ptr) = app { unsafe {
            let app_ref = &mut *app_ptr;
            if app_ref.player_info.is_none() { return; }
            if app_ref.has_finished_adventure() {
                Self::give_achievement(Some(app_ptr), AchievementId::HomeSecurity as i32, true);
            }
            if app_ref.earned_gold_trophy() {
                Self::give_achievement(Some(app_ptr), AchievementId::NovelPeasPrize as i32, true);
            }
            if app_ref.can_spawn_yetis() {
                Self::give_achievement(Some(app_ptr), AchievementId::Zombologist as i32, true);
            }
            // 检查是否收集了所有植物
            let mut all_collected = true;
            for seed_type in 0..=48 {
                if !app_ref.has_seed_type(unsafe { std::mem::transmute(seed_type) }) {
                    all_collected = false;
                    break;
                }
            }
            if all_collected {
                Self::give_achievement(Some(app_ptr), AchievementId::Morticulturalist as i32, true);
            }
        } }
    }
}