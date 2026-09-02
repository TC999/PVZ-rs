#![allow(dead_code)]

use crate::lawn::game_enums::*;

/// 最大盆栽植物数量（对应 C++ MAX_POTTED_PLANTS）
pub const MAX_POTTED_PLANTS: usize = 48;

/// 盆栽植物面朝方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacingDirection {
    Right = 0,
    Left = 1,
}

/// 盆栽植物（对应 C++ PottedPlant）
#[derive(Debug, Clone)]
pub struct PottedPlant {
    pub seed_type: SeedType,
    pub which_zen_garden: GardenType,
    pub x: i32,
    pub y: i32,
    pub facing: FacingDirection,
    pub last_watered_time: i64,
    pub draw_variation: DrawVariation,
    pub plant_age: PottedPlantAge,
    pub times_fed: i32,
    pub feedings_per_grow: i32,
    pub plant_need: PottedPlantNeed,
    pub last_need_fulfilled_time: i64,
    pub last_fertilized_time: i64,
    pub last_chocolate_time: i64,
}

impl PottedPlant {
    pub fn new() -> Self {
        PottedPlant {
            seed_type: SeedType::None,
            which_zen_garden: GardenType::Main,
            x: 0,
            y: 0,
            facing: FacingDirection::Right,
            last_watered_time: 0,
            draw_variation: DrawVariation::Normal,
            plant_age: PottedPlantAge::Sprout,
            times_fed: 0,
            feedings_per_grow: 5,
            plant_need: PottedPlantNeed::None,
            last_need_fulfilled_time: 0,
            last_fertilized_time: 0,
            last_chocolate_time: 0,
        }
    }

    pub fn initialize_potted_plant(&mut self, the_seed_type: SeedType) {
        self.seed_type = the_seed_type;
    }
}

impl Default for PottedPlant {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PlayerInfo {
    pub name: String,
    pub m_id: u32,
    pub m_use_seq: u32,
    pub m_level: i32,
    pub m_finished_adventure: i32,
    pub m_has_unlocked_minigames: bool,
    pub m_has_unlocked_puzzle_mode: bool,
    pub m_has_unlocked_survival_mode: bool,
    pub m_has_used_cheat_keys: bool,
    pub m_needs_magic_taco_reward: bool,
    /// 对应 C++ PlayerInfo::mHasSeenUpsell
    pub m_has_seen_upsell: i32,
    /// 对应 C++ PlayerInfo::mDidntPurchasePacketUpgrade
    pub m_didnt_purchase_packet_upgrade: i32,
    /// 对应 C++ PlayerInfo::mNeedsMessageOnGameSelector
    pub m_needs_message_on_game_selector: i32,
    /// 对应 C++ PlayerInfo::mHasNewMiniGame
    pub m_has_new_mini_game: i32,
    /// 对应 C++ PlayerInfo::mHasNewScaryPotter
    pub m_has_new_scary_potter: i32,
    /// 对应 C++ PlayerInfo::mHasNewIZombie
    pub m_has_new_izombie: i32,
    /// 对应 C++ PlayerInfo::mHasNewSurvival
    pub m_has_new_survival: i32,
    pub m_num_potted_plants: i32,
    pub m_coins: i32,
    pub m_purchases: Vec<i32>,
    pub m_challenge_records: Vec<i32>,
    /// 对应 C++ PlayerInfo::mPottedPlant[MAX_POTTED_PLANTS]
    pub m_potted_plant: Vec<PottedPlant>,
    /// 对应 C++ PlayerInfo::mHasWokenStinky
    pub m_has_woken_stinky: i32,
    /// 对应 C++ PlayerInfo::mLastStinkyChocolateTime
    pub m_last_stinky_chocolate_time: u32,
    /// 对应 C++ PlayerInfo::mHasSeenStinky
    pub m_has_seen_stinky: i32,
    /// 对应 C++ PlayerInfo::mStinkyPosX / mStinkyPosY
    pub stinky_pos_x: i32,
    pub stinky_pos_y: i32,
    /// 对应 C++ PlayerInfo::mEarnedAchievements[MAX_ACHIEVEMENTS]
    pub m_earned_achievements: Vec<bool>,
    /// 对应 C++ PlayerInfo::mShownAchievements[MAX_ACHIEVEMENTS]
    pub m_shown_achievements: Vec<bool>,
}

impl PlayerInfo {
    pub fn new() -> Self {
        PlayerInfo {
            name: String::new(),
            m_id: 0,
            m_use_seq: 0,
            m_level: 1,
            m_finished_adventure: 0,
            m_has_unlocked_minigames: false,
            m_has_unlocked_puzzle_mode: false,
            m_has_unlocked_survival_mode: false,
            m_has_used_cheat_keys: false,
            m_needs_magic_taco_reward: false,
            m_has_seen_upsell: 0,
            m_didnt_purchase_packet_upgrade: 0,
            m_needs_message_on_game_selector: 0,
            m_has_new_mini_game: 0,
            m_has_new_scary_potter: 0,
            m_has_new_izombie: 0,
            m_has_new_survival: 0,
            m_num_potted_plants: 0,
            m_coins: 0,
            m_purchases: vec![0; 200],
            m_challenge_records: vec![0; 200],
            m_potted_plant: Vec::new(),
            m_has_woken_stinky: 0,
            m_last_stinky_chocolate_time: 0,
            m_has_seen_stinky: 0,
            stinky_pos_x: 0,
            stinky_pos_y: 0,
            m_earned_achievements: vec![false; crate::lawn::widget::achievements_screen::MAX_ACHIEVEMENTS],
            m_shown_achievements: vec![false; crate::lawn::widget::achievements_screen::MAX_ACHIEVEMENTS],
        }
    }

    pub fn get_level(&self) -> i32 {
        self.m_level
    }

    /// 设置等级（对应 C++ PlayerInfo::SetLevel）
    pub fn set_level(&mut self, the_level: i32) {
        self.m_level = the_level;
    }

    /// 增加金币（对应 C++ PlayerInfo::AddCoins）
    pub fn add_coins(&mut self, amount: i32) {
        self.m_coins += amount;
        self.m_coins = self.m_coins.clamp(0, 99999);
    }
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self::new()
    }
}
