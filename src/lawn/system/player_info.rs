#![allow(dead_code)]

pub struct PlayerInfo {
    pub name: String,
    pub m_level: i32,
    pub m_finished_adventure: i32,
    pub m_has_unlocked_minigames: bool,
    pub m_has_unlocked_puzzle_mode: bool,
    pub m_has_unlocked_survival_mode: bool,
    pub m_has_used_cheat_keys: bool,
    pub m_needs_magic_taco_reward: bool,
    pub m_num_potted_plants: i32,
    pub m_coins: i32,
    pub m_purchases: Vec<i32>,
    pub m_challenge_records: Vec<i32>,
}

impl PlayerInfo {
    pub fn new() -> Self {
        PlayerInfo {
            name: String::new(),
            m_level: 1,
            m_finished_adventure: 0,
            m_has_unlocked_minigames: false,
            m_has_unlocked_puzzle_mode: false,
            m_has_unlocked_survival_mode: false,
            m_has_used_cheat_keys: false,
            m_needs_magic_taco_reward: false,
            m_num_potted_plants: 0,
            m_coins: 0,
            m_purchases: vec![0; 200],
            m_challenge_records: vec![0; 200],
        }
    }

    pub fn get_level(&self) -> i32 {
        self.m_level
    }
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self::new()
    }
}
