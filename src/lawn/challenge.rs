// PvZ Portable Rust 翻译 — Challenge（挑战模式）
// 对应 C++ src/Lawn/Challenge.h / Challenge.cpp

use crate::lawn::game_enums::*;

/// 挑战模式
pub struct Challenge {
    pub challenge_state: ChallengeState,
    pub challenge_mode: GameMode,
    pub stage: i32,
    pub time_counter: i32,
}

impl Challenge {
    pub fn new(mode: GameMode) -> Self {
        Challenge {
            challenge_state: ChallengeState::Normal,
            challenge_mode: mode,
            stage: 0,
            time_counter: 0,
        }
    }

    pub fn update(&mut self) {
        self.time_counter += 1;
    }
}

impl Default for Challenge {
    fn default() -> Self {
        Challenge::new(GameMode::Adventure)
    }
}
