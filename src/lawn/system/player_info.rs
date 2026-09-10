#![allow(dead_code)]

use crate::lawn::game_enums::*;
use crate::lawn::system::data_sync::{DataSync, DataReader, DataWriter};
use crate::lawn::system::data_sync::DataReaderException;
use std::fs;
use std::path::Path;

/// 最大盆栽植物数量（对应 C++ MAX_POTTED_PLANTS）
pub const MAX_POTTED_PLANTS: usize = 48;

/// 盆栽植物面朝方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
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

#[derive(Debug, Clone)]
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
    /// 对应 C++ PlayerInfo::mPlayTimeActivePlayer
    pub m_play_time_active_player: u32,
    /// 对应 C++ PlayerInfo::mPlayTimeInactivePlayer
    pub m_play_time_inactive_player: u32,
    /// 对应 C++ PlayerInfo::mPlaceHolderPlayerStats
    pub m_place_holder_player_stats: u32,
    /// 对应 C++ PlayerInfo::mZombatarAccepted
    pub m_zombatar_accepted: u8,
    /// 对应 C++ PlayerInfo::mZombatarHeadCount
    pub m_zombatar_head_count: u32,
    /// 对应 C++ PlayerInfo::mZombatarData（原始 0x48 * count 字节）
    pub m_zombatar_data: Vec<u8>,
    /// 对应 C++ PlayerInfo::mZombatarCreatedBefore
    pub m_zombatar_created_before: u8,
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
            m_play_time_active_player: 0,
            m_play_time_inactive_player: 0,
            m_place_holder_player_stats: 0,
            m_zombatar_accepted: 0,
            m_zombatar_head_count: 0,
            m_zombatar_data: Vec::new(),
            m_zombatar_created_before: 0,
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

    /// 重置全部字段为默认值（对应 C++ PlayerInfo::Reset）
    pub fn reset(&mut self) {
        self.m_level = 1;
        self.m_coins = 0;
        self.m_finished_adventure = 0;
        for v in self.m_challenge_records.iter_mut() { *v = 0; }
        for v in self.m_purchases.iter_mut() { *v = 0; }
        self.m_play_time_active_player = 0;
        self.m_play_time_inactive_player = 0;
        self.m_has_used_cheat_keys = false;
        self.m_has_woken_stinky = 0;
        self.m_didnt_purchase_packet_upgrade = 0;
        self.m_last_stinky_chocolate_time = 0;
        self.stinky_pos_x = 0;
        self.stinky_pos_y = 0;
        self.m_has_unlocked_minigames = false;
        self.m_has_unlocked_puzzle_mode = false;
        self.m_has_new_mini_game = 0;
        self.m_has_new_scary_potter = 0;
        self.m_has_new_izombie = 0;
        self.m_has_new_survival = 0;
        self.m_has_unlocked_survival_mode = false;
        self.m_needs_message_on_game_selector = 0;
        self.m_needs_magic_taco_reward = false;
        self.m_has_seen_stinky = 0;
        self.m_has_seen_upsell = 0;
        self.m_place_holder_player_stats = 0;
        self.m_potted_plant.clear();
        self.m_num_potted_plants = 0;
        for v in self.m_earned_achievements.iter_mut() { *v = false; }
        for v in self.m_shown_achievements.iter_mut() { *v = false; }
        self.m_zombatar_accepted = 0;
        self.m_zombatar_head_count = 0;
        self.m_zombatar_data.clear();
        self.m_zombatar_created_before = 0;
    }

    /// 同步档案摘要（对应 C++ SyncSummary：名称/使用序号/ID）
    pub fn sync_summary(&mut self, the_sync: &mut DataSync) {
        the_sync.sync_string(&mut self.name);
        the_sync.sync_u32(&mut self.m_use_seq);
        the_sync.sync_u32(&mut self.m_id);
    }

    /// 同步档案详情（对应 C++ SyncDetails）
    pub fn sync_details(&mut self, the_sync: &mut DataSync) {
        let is_reader = the_sync.get_reader().is_some();
        if is_reader {
            self.reset();
        }

        // 版本号（对应 C++ gUserVersion = 12）
        let mut a_version: u32 = 12;
        the_sync.sync_u32(&mut a_version);
        the_sync.set_version(a_version as i32);
        if a_version != 12 {
            return;
        }

        // 基本字段（Rust i32 经 u32 中转，保持 C++ uint32 布局）
        let mut a_level = self.m_level as u32;
        the_sync.sync_u32(&mut a_level);
        self.m_level = a_level as i32;

        let mut a_coins = self.m_coins as u32;
        the_sync.sync_u32(&mut a_coins);
        self.m_coins = a_coins as i32;

        let mut a_finished = self.m_finished_adventure as u32;
        the_sync.sync_u32(&mut a_finished);
        self.m_finished_adventure = a_finished as i32;

        for i in 0..100 {
            let mut v = if i < self.m_challenge_records.len() { self.m_challenge_records[i] as u32 } else { 0 };
            the_sync.sync_u32(&mut v);
            if i < self.m_challenge_records.len() { self.m_challenge_records[i] = v as i32; }
        }
        for i in 0..80 {
            let mut v = if i < self.m_purchases.len() { self.m_purchases[i] as u32 } else { 0 };
            the_sync.sync_u32(&mut v);
            if i < self.m_purchases.len() { self.m_purchases[i] = v as i32; }
        }

        the_sync.sync_u32(&mut self.m_play_time_active_player);
        the_sync.sync_u32(&mut self.m_play_time_inactive_player);

        let mut a_cheat = if self.m_has_used_cheat_keys { 1 } else { 0 };
        the_sync.sync_u32(&mut a_cheat);
        self.m_has_used_cheat_keys = a_cheat != 0;

        let mut a_woken = self.m_has_woken_stinky as u32;
        the_sync.sync_u32(&mut a_woken);
        self.m_has_woken_stinky = a_woken as i32;

        let mut a_packet = self.m_didnt_purchase_packet_upgrade as u32;
        the_sync.sync_u32(&mut a_packet);
        self.m_didnt_purchase_packet_upgrade = a_packet as i32;

        the_sync.sync_u32(&mut self.m_last_stinky_chocolate_time);

        let mut a_pos_x = self.stinky_pos_x as u32;
        the_sync.sync_u32(&mut a_pos_x);
        self.stinky_pos_x = a_pos_x as i32;
        let mut a_pos_y = self.stinky_pos_y as u32;
        the_sync.sync_u32(&mut a_pos_y);
        self.stinky_pos_y = a_pos_y as i32;

        let mut a_minigames = if self.m_has_unlocked_minigames { 1 } else { 0 };
        the_sync.sync_u32(&mut a_minigames);
        self.m_has_unlocked_minigames = a_minigames != 0;
        let mut a_puzzle = if self.m_has_unlocked_puzzle_mode { 1 } else { 0 };
        the_sync.sync_u32(&mut a_puzzle);
        self.m_has_unlocked_puzzle_mode = a_puzzle != 0;

        let mut a_new_mini = self.m_has_new_mini_game as u32;
        the_sync.sync_u32(&mut a_new_mini);
        self.m_has_new_mini_game = a_new_mini as i32;
        let mut a_new_scary = self.m_has_new_scary_potter as u32;
        the_sync.sync_u32(&mut a_new_scary);
        self.m_has_new_scary_potter = a_new_scary as i32;
        let mut a_new_izombie = self.m_has_new_izombie as u32;
        the_sync.sync_u32(&mut a_new_izombie);
        self.m_has_new_izombie = a_new_izombie as i32;
        let mut a_new_survival = self.m_has_new_survival as u32;
        the_sync.sync_u32(&mut a_new_survival);
        self.m_has_new_survival = a_new_survival as i32;

        let mut a_survival = if self.m_has_unlocked_survival_mode { 1 } else { 0 };
        the_sync.sync_u32(&mut a_survival);
        self.m_has_unlocked_survival_mode = a_survival != 0;

        let mut a_msg_selector = self.m_needs_message_on_game_selector as u32;
        the_sync.sync_u32(&mut a_msg_selector);
        self.m_needs_message_on_game_selector = a_msg_selector as i32;
        let mut a_taco = if self.m_needs_magic_taco_reward { 1 } else { 0 };
        the_sync.sync_u32(&mut a_taco);
        self.m_needs_magic_taco_reward = a_taco != 0;

        let mut a_seen_stinky = self.m_has_seen_stinky as u32;
        the_sync.sync_u32(&mut a_seen_stinky);
        self.m_has_seen_stinky = a_seen_stinky as i32;
        let mut a_seen_upsell = self.m_has_seen_upsell as u32;
        the_sync.sync_u32(&mut a_seen_upsell);
        self.m_has_seen_upsell = a_seen_upsell as i32;

        the_sync.sync_u32(&mut self.m_place_holder_player_stats);

        let mut a_num_potted = self.m_num_potted_plants as u32;
        the_sync.sync_u32(&mut a_num_potted);
        self.m_num_potted_plants = a_num_potted as i32;

        // [TRANSLATION_NOTE]: C++ 用 SyncBytes(&mPottedPlant[i], sizeof(PottedPlant)) 同步原始字节；
        // Rust 侧 PottedPlant 为逻辑结构，字节级序列化未接入，仅同步数量
        if is_reader {
            self.m_potted_plant.clear();
        }

        // 成就：20 个 uint16（对应 C++ 存档格式）
        for i in 0..20 {
            let mut a_val: u16 = if i < self.m_earned_achievements.len() && self.m_earned_achievements[i] { 1 } else { 0 };
            the_sync.sync_u16(&mut a_val);
            if is_reader && i < self.m_earned_achievements.len() {
                self.m_earned_achievements[i] = a_val != 0;
                self.m_shown_achievements[i] = a_val != 0;
            }
        }

        // Zombatar 段（对应 C++ SyncDetails 中 reader 的 try/catch 与 writer 分支）
        if is_reader {
            // [TRANSLATION_NOTE]: C++ 中读取异常（DataReaderException）时重置 Zombatar 字段；
            // Rust DataReader 的 read 不抛异常，结构上保留读取
            let mut a_zombatar_accepted: u8 = 0;
            the_sync.sync_u8(&mut a_zombatar_accepted);
            self.m_zombatar_accepted = a_zombatar_accepted;
            let mut a_head_count: u32 = 0;
            the_sync.sync_u32(&mut a_head_count);
            if a_head_count > 100 { a_head_count = 0; }
            self.m_zombatar_head_count = a_head_count;
            let data_len = (a_head_count as usize) * 0x48usize;
            self.m_zombatar_data = vec![0u8; data_len];
            if data_len > 0 {
                the_sync.sync_bytes(&mut self.m_zombatar_data);
            }
            // 0x14 字节 mini-game flags（读取后丢弃，对应 C++ 存档兼容）
            let mut a_mini_game_flags = [0u8; 0x14];
            the_sync.sync_bytes(&mut a_mini_game_flags);
            let mut a_created_before: u8 = 0;
            the_sync.sync_u8(&mut a_created_before);
            self.m_zombatar_created_before = a_created_before;
        } else {
            let mut a_zombatar_accepted = self.m_zombatar_accepted;
            the_sync.sync_u8(&mut a_zombatar_accepted);
            let mut a_head_count = self.m_zombatar_head_count;
            the_sync.sync_u32(&mut a_head_count);
            let data_len = (a_head_count as usize) * 0x48usize;
            let mut a_data = self.m_zombatar_data.clone();
            a_data.resize(data_len, 0);
            if data_len > 0 {
                the_sync.sync_bytes(&mut a_data);
            }
            let mut a_mini_game_flags = [0u8; 0x14];
            for i in 0..0x14 {
                a_mini_game_flags[i] = if i + 0x0F < self.m_challenge_records.len() && self.m_challenge_records[i + 0x0F] > 0 { 1 } else { 0 };
            }
            the_sync.sync_bytes(&mut a_mini_game_flags);
            let mut a_created_before = self.m_zombatar_created_before;
            the_sync.sync_u8(&mut a_created_before);
        }
    }

    /// 加载档案详情（对应 C++ LoadDetails）
    pub fn load_details(&mut self) {
        let a_file_name = format!("userdata/user{}.dat", self.m_id);
        match DataReader::open_file(Path::new(&a_file_name)) {
            Some(reader) => {
                let mut a_sync = DataSync::from_reader(reader);
                self.sync_details(&mut a_sync);
            }
            None => {}
        }
    }

    /// 保存档案详情（对应 C++ SaveDetails）
    pub fn save_details(&self) {
        let mut a_sync = DataSync::from_writer(DataWriter::open_memory(0x20));
        // 借用：临时可变副本用于 sync_details（保持写入布局一致）
        let mut a_copy = self.clone_for_sync();
        a_copy.sync_details(&mut a_sync);

        let _ = fs::create_dir_all("userdata");
        let a_file_name = format!("userdata/user{}.dat", self.m_id);
        if let Some(writer) = a_sync.get_writer_mut() {
            writer.write_to_file(Path::new(&a_file_name));
        }
    }

    /// 删除该档案的所有文件（对应 C++ DeleteUserFiles）
    pub fn delete_user_files(&self) {
        let a_filename = format!("userdata/user{}.dat", self.m_id);
        let _ = fs::remove_file(Path::new(&a_filename));

        // 遍历全部 GameMode 删除当前存档与旧版存档（文件名格式与 lawn_common::get_saved_game_name 一致）
        for i in 0..(GameMode::Intro as i32 + 1) {
            let a_file_name = format!("userdata/game{}_{}.v4", self.m_id, i);
            let _ = fs::remove_file(Path::new(&a_file_name));
            let a_legacy_file_name = format!("userdata/game{}_{}.dat", self.m_id, i);
            let _ = fs::remove_file(Path::new(&a_legacy_file_name));
        }
    }

    /// 重置挑战记录（对应 C++ ResetChallengeRecord）
    pub fn reset_challenge_record(&mut self, the_game_mode: GameMode) {
        let a_game_mode = the_game_mode as i32 - GameMode::SurvivalNormalStage1 as i32;
        if a_game_mode >= 0 && (a_game_mode as usize) < self.m_challenge_records.len() {
            self.m_challenge_records[a_game_mode as usize] = 0;
        }
    }
}

/// 便于写入序列化的克隆辅助（Rust 借用约束：save_details 用 &self 时需先复制）
impl PlayerInfo {
    fn clone_for_sync(&self) -> PlayerInfo {
        PlayerInfo {
            name: self.name.clone(),
            m_id: self.m_id,
            m_use_seq: self.m_use_seq,
            m_level: self.m_level,
            m_finished_adventure: self.m_finished_adventure,
            m_has_unlocked_minigames: self.m_has_unlocked_minigames,
            m_has_unlocked_puzzle_mode: self.m_has_unlocked_puzzle_mode,
            m_has_unlocked_survival_mode: self.m_has_unlocked_survival_mode,
            m_has_used_cheat_keys: self.m_has_used_cheat_keys,
            m_needs_magic_taco_reward: self.m_needs_magic_taco_reward,
            m_has_seen_upsell: self.m_has_seen_upsell,
            m_didnt_purchase_packet_upgrade: self.m_didnt_purchase_packet_upgrade,
            m_needs_message_on_game_selector: self.m_needs_message_on_game_selector,
            m_has_new_mini_game: self.m_has_new_mini_game,
            m_has_new_scary_potter: self.m_has_new_scary_potter,
            m_has_new_izombie: self.m_has_new_izombie,
            m_has_new_survival: self.m_has_new_survival,
            m_num_potted_plants: self.m_num_potted_plants,
            m_coins: self.m_coins,
            m_purchases: self.m_purchases.clone(),
            m_challenge_records: self.m_challenge_records.clone(),
            m_potted_plant: self.m_potted_plant.clone(),
            m_has_woken_stinky: self.m_has_woken_stinky,
            m_last_stinky_chocolate_time: self.m_last_stinky_chocolate_time,
            m_has_seen_stinky: self.m_has_seen_stinky,
            stinky_pos_x: self.stinky_pos_x,
            stinky_pos_y: self.stinky_pos_y,
            m_earned_achievements: self.m_earned_achievements.clone(),
            m_shown_achievements: self.m_shown_achievements.clone(),
            m_play_time_active_player: self.m_play_time_active_player,
            m_play_time_inactive_player: self.m_play_time_inactive_player,
            m_place_holder_player_stats: self.m_place_holder_player_stats,
            m_zombatar_accepted: self.m_zombatar_accepted,
            m_zombatar_head_count: self.m_zombatar_head_count,
            m_zombatar_data: self.m_zombatar_data.clone(),
            m_zombatar_created_before: self.m_zombatar_created_before,
        }
    }
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self::new()
    }
}
