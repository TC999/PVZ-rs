// PvZ Portable Rust 翻译 — SeedPacket（种子槽）
// 对应 C++ src/Lawn/SeedPacket.h / SeedPacket.cpp

use crate::lawn::game_enums::*;
use crate::lawn::game_object::GameObject;
use crate::lawn::board::HitResult;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;
use crate::todlib::tod_common::TodWeightedArray;

/// 种子槽 — 玩家选择植物的 UI 元素
pub struct SeedPacket {
    pub seed_type: SeedType,
    pub imitater_type: SeedType,
    pub packet_index: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub countdown: i32,  // 冷却时间
    pub refresh_time: i32,
    pub refreshing: bool,
    pub active: bool,
    pub can_afford: bool,
    pub slot_machine_countdown: i32,
    pub slot_machine_position: f32,
    pub slot_machine_next_seed: SeedType,
    pub times_used: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub board: Option<*mut crate::lawn::board::Board>,
}

impl SeedPacket {
    pub fn new() -> Self {
        SeedPacket {
            seed_type: SeedType::None,
            imitater_type: SeedType::None,
            packet_index: 0,
            x: 0,
            y: 516,
            width: SEED_PACKET_WIDTH,
            height: SEED_PACKET_HEIGHT,
            countdown: 0,
            refresh_time: 0,
            refreshing: false,
            active: true,
            can_afford: false,
            slot_machine_countdown: 0,
            slot_machine_position: 0.0,
            slot_machine_next_seed: SeedType::None,
            times_used: 0,
            offset_x: 0,
            offset_y: 0,
            app: None,
            board: None,
        }
    }

    /// 初始化种子槽
    pub fn seed_packet_initialize(&mut self, _index: i32) {
        self.packet_index = _index;
    }

    /// 设置种子类型（对应 C++ SetPacketType）
    pub fn set_packet_type(&mut self, seed_type: SeedType, imitater_type: SeedType) {
        self.seed_type = seed_type;
        self.imitater_type = imitater_type;
        self.refreshing = false;
        self.countdown = 0;
        self.refresh_time = 0;
        self.active = true;
    }

    /// 激活（对应 C++ Activate）
    pub fn activate(&mut self) {
        // PVZP_ASSERT(mPacketType != SEED_NONE)
        self.active = true;
    }

    /// 停用（对应 C++ Deactivate：复位所有冷却/刷新状态）
    pub fn deactivate(&mut self) {
        self.active = false;
        self.countdown = 0;
        self.refresh_time = 0;
        self.refreshing = false;
    }

    /// 设置激活状态（对应 C++ SetActivate）
    pub fn set_activate(&mut self, active: bool) {
        if active {
            self.activate();
        } else {
            self.deactivate();
        }
    }

    /// 能否拾取（对应 C++ CanPickUp）
    pub fn can_pick_up(&self) -> bool {
        self.active && self.countdown <= 0 && self.seed_type != SeedType::None
    }

    /// 准备就绪闪烁（对应 C++ FlashIfReady）
    pub fn flash_if_ready(&mut self) {
        if !self.can_pick_up() {
            return;
        }
        if let Some(app) = self.app {
            unsafe {
                if (*app).m_easy_planting_cheat {
                    return;
                }
            }
        }

        if let Some(board) = self.board {
            unsafe {
                let b = &mut *board;
                // [TRANSLATION_NOTE]: 非传送带模式加 PARTICLE_SEED_PACKET_FLASH 粒子未接入
                if !b.has_conveyor_belt_seed_bank() {
                    // 粒子位置：mX + mSeedBank->mX, mY + mSeedBank->mY
                }

                if b.m_tutorial_state == TutorialState::Level1RefreshPeashooter {
                    b.set_tutorial_state(TutorialState::Level1PickUpPeashooter);
                } else if b.m_tutorial_state == TutorialState::Level2RefreshSunflower
                    && self.seed_type == SeedType::Sunflower
                {
                    b.set_tutorial_state(TutorialState::Level2PickUpSunflower);
                } else if b.m_tutorial_state == TutorialState::MoreSunRefreshSunflower
                    && self.seed_type == SeedType::Sunflower
                {
                    b.set_tutorial_state(TutorialState::MoreSunPickUpSunflower);
                }
            }
        }
    }

    /// 老虎机选种子（对应 C++ PickNextSlotMachineSeed）
    pub fn pick_next_slot_machine_seed(&mut self) {
        let a_peas_count = match self.board {
            Some(b) => unsafe { (*b).count_plant_by_type(SeedType::Peashooter) },
            None => 0,
        };

        let slot_seed_types = [
            SeedType::Sunflower,
            SeedType::Peashooter,
            SeedType::Snowpea,
            SeedType::Wallnut,
            SeedType::SlotMachineSun,
            SeedType::SlotMachineDiamond,
        ];

        let mut a_seed_weight_array = [TodWeightedArray { item: 0, weight: 0 }; NUM_SEED_TYPES];
        let mut a_seeds_count = 0usize;
        for &a_seed_type in &slot_seed_types {
            let mut a_weight = 100;
            if a_seed_type == SeedType::Peashooter {
                a_weight = crate::todlib::tod_common::tod_animate_curve(
                    0, 5, a_peas_count, 200, 100, TodCurves::Linear,
                );
            } else if a_seed_type == SeedType::SlotMachineDiamond {
                a_weight = 30;
            }

            if self.packet_index == 2 && a_seed_type != SeedType::SlotMachineDiamond {
                if let Some(board) = self.board {
                    unsafe {
                        let b = &*board;
                        if b.seed_bank.len() > 1
                            && (a_seed_type == b.seed_bank[0].slot_machine_next_seed
                                || a_seed_type == b.seed_bank[1].slot_machine_next_seed)
                        {
                            a_weight += a_weight / 2;
                        }
                    }
                }
            }

            a_seed_weight_array[a_seeds_count].item = a_seed_type as usize;
            a_seed_weight_array[a_seeds_count].weight = a_weight;
            a_seeds_count += 1;
        }

        let a_pick = crate::todlib::tod_common::tod_pick_from_weighted_array(&a_seed_weight_array[..a_seeds_count]);
        self.slot_machine_next_seed = slot_seed_types[a_pick.clamp(0, slot_seed_types.len() as isize - 1) as usize];
    }

    /// 老虎机启动（对应 C++ SlotMachineStart）
    pub fn slot_machine_start(&mut self) {
        self.slot_machine_countdown = 300;
        self.slot_machine_position = 0.0;
        self.pick_next_slot_machine_seed();
    }

    /// 更新冷却（对应 C++ SeedPacket::Update）
    pub fn update(&mut self) {
        if self.seed_type == SeedType::None {
            return;
        }
        // [TRANSLATION_NOTE]: C++ 先检查 mGameScene == SCENE_PLAYING 才更新 — 场景状态未接入

        if let Some(board) = self.board {
            unsafe {
                // mMainCounter == 0 时闪烁
                if (*board).m_main_counter == 0 {
                    self.flash_if_ready();
                }
            }
        }

        // 冷却刷新
        if !self.active && self.refreshing {
            self.countdown += 1;
            if self.countdown > self.refresh_time {
                self.countdown = 0;
                self.refreshing = false;
                self.activate();
                self.flash_if_ready();
            }
        }

        // 老虎机滚动（对应 C++ Update：翻转速度曲线）
        if self.slot_machine_countdown > 0 {
            self.slot_machine_countdown -= 1;
            let a_flips_per_second = crate::todlib::tod_common::tod_animate_curve_float(
                400, 0, self.slot_machine_countdown, 6.0, 2.0, TodCurves::Linear,
            );
            self.slot_machine_position += a_flips_per_second * 0.01;

            if self.slot_machine_position >= 1.0 {
                self.seed_type = self.slot_machine_next_seed;
                if self.slot_machine_countdown == 0 {
                    self.activate();
                    self.slot_machine_position = 0.0;
                } else {
                    self.slot_machine_position -= 1.0;
                    self.pick_next_slot_machine_seed();
                }
            } else if self.slot_machine_countdown == 0 {
                self.slot_machine_countdown = 1;
            }
        }
    }

    /// 绘制
    pub fn draw(&self, g: &mut Graphics) {
        let mut a_percent_dark = 0.0f32;
        if !self.active {
            if self.refresh_time == 0 {
                a_percent_dark = 1.0;
            } else {
                a_percent_dark = (self.refresh_time - self.countdown) as f32 / self.refresh_time as f32;
            }
        }

        if self.slot_machine_countdown > 0 {
            // 老虎机滚动中：裁剪绘制当前种与下一种子（两张叠画）
            let a_offset_y = (-self.height as f32 * self.slot_machine_position).round() as i32;
            draw_seed_packet(g, 0.0, a_offset_y as f32, self.seed_type, SeedType::None, 0.0, 128, false, false);
            draw_seed_packet(g, 0.0, (self.height + a_offset_y) as f32, self.slot_machine_next_seed, SeedType::None, 0.0, 128, false, false);
        } else {
            let mut a_use_seed_type = self.seed_type;
            if self.seed_type == SeedType::Imitater && self.imitater_type != SeedType::None {
                a_use_seed_type = self.imitater_type;
            }

            let mut a_draw_cost = true;
            let mut a_cost = 0;
            let mut a_conveyor = false;
            let mut a_slot_level = false;
            let mut a_grayness = 255;
            if let Some(board) = self.board {
                unsafe {
                    let b = &*board;
                    a_conveyor = b.has_conveyor_belt_seed_bank();
                    a_cost = b.get_current_plant_cost(self.seed_type, self.imitater_type);
                    if a_conveyor {
                        a_draw_cost = false;
                    }
                    if let Some(app) = self.app {
                        if (*app).is_slot_machine_level() {
                            a_slot_level = true;
                            a_draw_cost = false;
                        }
                        if ((*app).game_mode == GameMode::ChallengeBeghouled && !self.active)
                            || ((*app).game_mode == GameMode::ChallengeBeghouledTwist && !self.active)
                        {
                            a_grayness = 64;
                        } else if (*app).m_easy_planting_cheat {
                            a_percent_dark = 0.0;
                        } else if (a_draw_cost && !b.can_take_sun_money(a_cost))
                            || a_percent_dark > 1.0
                            || !b.planting_requirements_met(a_use_seed_type)
                        {
                            a_grayness = 128;
                        }
                    }
                }
            }
            // [TRANSLATION_NOTE]: C++ 中 mGameScene != SCENE_PLAYING 用 mCutSceneDarken、教程高亮闪烁灰化未接入
            let _ = (a_conveyor, a_slot_level, a_cost, a_grayness);

            draw_seed_packet(g, self.offset_x as f32, 0.0, self.seed_type, self.imitater_type, a_percent_dark, a_grayness, a_draw_cost, true);
        }
    }

    /// 设置冷却时间
    pub fn set_countdown(&mut self, refresh_time: i32) {
        self.countdown = refresh_time;
    }

    /// 种植后处理（对应 C++ WasPlanted）
    pub fn was_planted(&mut self) {
        // PVZP_ASSERT(mPacketType != SEED_NONE)
        let mut conveyor = false;
        let mut slot_level = false;
        let mut last_stand_before_onslaught = false;
        if let Some(board) = self.board {
            unsafe {
                let b = &*board;
                conveyor = b.has_conveyor_belt_seed_bank();
                // C++: mGameMode == GAMEMODE_CHALLENGE_LAST_STAND && mChallenge->mChallengeState != STATECHALLENGE_LAST_STAND_ONSLAUGHT
                if b.challenge.as_ref().map_or(false, |c| c.challenge_state != ChallengeState::LastStandOnslaught) {
                    last_stand_before_onslaught = true;
                }
            }
        }
        if let Some(app) = self.app {
            unsafe {
                slot_level = (*app).is_slot_machine_level();
                if (*app).game_mode != GameMode::ChallengeLastStand {
                    last_stand_before_onslaught = false;
                }
            }
        }

        if conveyor {
            // [TRANSLATION_NOTE]: C++ mBoard->mSeedBank->RemoveSeed(mIndex) — Rust 侧
            // board.seed_bank 为 Vec<SeedPacket>，移除逻辑由 Board 层处理，此处暂略
        } else if slot_level {
            self.deactivate();
        } else if last_stand_before_onslaught {
            self.times_used += 1;
            self.active = true;
            self.flash_if_ready();
        } else {
            self.times_used += 1;
            self.refreshing = true;
            self.refresh_time = crate::lawn::plant::Plant::get_refresh_time(self.seed_type, self.imitater_type);
        }
    }

    /// 鼠标点击（对应 C++ MouseDown）
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {
        let board_ptr = match self.board { Some(b) => b, None => return };
        let app_ptr = match self.app { Some(a) => a, None => return };
        unsafe {
            let b = &mut *board_ptr;
            let app = &mut *app_ptr;
            // C++: mBoard->mPaused || mApp->mGameScene != SCENE_PLAYING || mPacketType == SEED_NONE
            if b.m_paused || self.seed_type == SeedType::None {
                return;
            }
            // [TRANSLATION_NOTE]: mApp->mGameScene != SCENE_PLAYING 检查未接入

            // 老虎机模式
            if app.is_slot_machine_level() {
                // [TRANSLATION_NOTE]: mBoard->mAdvice->IsBeingDisplayed() 未接入，直接显示
                b.display_advice("[ADVICE_SLOT_MACHINE_PULL]", MessageStyle::HintTallFast as i32, AdviceType::SlotMachinePull);
                // [TRANSLATION_NOTE]: mChallenge->mSlotMachineRollCount = min(roll, 2) 未接入
                return;
            }

            let mut a_use_seed_type = self.seed_type;
            if self.seed_type == SeedType::Imitater && self.imitater_type != SeedType::None {
                a_use_seed_type = self.imitater_type;
            }

            if !app.m_easy_planting_cheat {
                if !self.active {
                    // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                    app.play_sample(crate::todlib::tod_foley::SOUND_BUZZER);
                    if app.is_first_time_adventure_mode() && b.level == 1 {
                        b.display_advice("[ADVICE_SEED_REFRESH]", MessageStyle::TutorialLevel1 as i32, AdviceType::SeedRefresh);
                    }
                    return;
                }

                let a_cost = b.get_current_plant_cost(self.seed_type, self.imitater_type);
                if !b.can_take_sun_money(a_cost) && !b.has_conveyor_belt_seed_bank() {
                    // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                    app.play_sample(crate::todlib::tod_foley::SOUND_BUZZER);
                    b.m_out_of_money_counter = 70;
                    if app.is_first_time_adventure_mode() && b.level == 1 {
                        b.display_advice("[ADVICE_CANT_AFFORD_PLANT]", MessageStyle::TutorialLevel1 as i32, AdviceType::CantAffordPlant);
                    }
                    return;
                }

                if !b.planting_requirements_met(a_use_seed_type) {
                    // C++: mApp->PlaySample(Sexy::SOUND_BUZZER);
                    app.play_sample(crate::todlib::tod_foley::SOUND_BUZZER);
                    let (advice, style) = match a_use_seed_type {
                        SeedType::Gatlingpea => (AdviceType::PlantNeedsRepeater, MessageStyle::HintLong),
                        SeedType::Wintermelon => (AdviceType::PlantNeedsMelonpult, MessageStyle::HintLong),
                        SeedType::Twinsunflower => (AdviceType::PlantNeedsSunflower, MessageStyle::HintLong),
                        SeedType::Spikerock => (AdviceType::PlantNeedsSpikeweed, MessageStyle::HintLong),
                        SeedType::Cobcannon => (AdviceType::PlantNeedsKernelpult, MessageStyle::HintLong),
                        SeedType::GoldMagnet => (AdviceType::PlantNeedsMagnetshroom, MessageStyle::HintLong),
                        SeedType::Gloomshroom => (AdviceType::PlantNeedsFumeshroom, MessageStyle::HintLong),
                        SeedType::Cattail => (AdviceType::PlantNeedsLilypad, MessageStyle::HintLong),
                        _ => (AdviceType::None, MessageStyle::HintLong),
                    };
                    let advice_str = match advice {
                        AdviceType::PlantNeedsRepeater => "[ADVICE_PLANT_NEEDS_REPEATER]",
                        AdviceType::PlantNeedsMelonpult => "[ADVICE_PLANT_NEEDS_MELONPULT]",
                        AdviceType::PlantNeedsSunflower => "[ADVICE_PLANT_NEEDS_SUNFLOWER]",
                        AdviceType::PlantNeedsSpikeweed => "[ADVICE_PLANT_NEEDS_SPIKEWEED]",
                        AdviceType::PlantNeedsKernelpult => "[ADVICE_PLANT_NEEDS_KERNELPULT]",
                        AdviceType::PlantNeedsMagnetshroom => "[ADVICE_PLANT_NEEDS_MAGNETSHROOM]",
                        AdviceType::PlantNeedsFumeshroom => "[ADVICE_PLANT_NEEDS_FUMESHROOM]",
                        AdviceType::PlantNeedsLilypad => "[ADVICE_PLANT_NEEDS_LILYPAD]",
                        _ => "",
                    };
                    if !advice_str.is_empty() {
                        b.display_advice(advice_str, style as i32, advice);
                    }
                    return;
                }
            }

            b.clear_advice(AdviceType::CantAffordPlant);
            b.clear_advice(AdviceType::PlantNeedsRepeater);
            b.clear_advice(AdviceType::PlantNeedsMelonpult);
            b.clear_advice(AdviceType::PlantNeedsSunflower);
            b.clear_advice(AdviceType::PlantNeedsKernelpult);
            b.clear_advice(AdviceType::PlantNeedsSpikeweed);
            b.clear_advice(AdviceType::PlantNeedsMagnetshroom);
            b.clear_advice(AdviceType::PlantNeedsFumeshroom);
            b.clear_advice(AdviceType::PlantNeedsLilypad);

            // 三消/水族馆模式转发给 Challenge
            if app.game_mode == GameMode::ChallengeBeghouled || app.game_mode == GameMode::ChallengeBeghouledTwist {
                if let Some(challenge) = b.challenge.as_mut() {
                    challenge.beghouled_packet_clicked(self);
                }
            } else if app.game_mode == GameMode::ChallengeZombiquarium {
                if let Some(challenge) = b.challenge.as_mut() {
                    challenge.zombiquarium_packet_clicked(self);
                }
            } else {
                // 普通模式：设置光标 + 音效 + 教程推进 + Deactivate
                b.cursor_object.seed_type = self.seed_type;
                b.cursor_object.imitater_type = self.imitater_type;
                b.cursor_object.cursor_type = CursorType::PlantFromBank;
                b.cursor_object.seed_bank_index = self.packet_index;
                // C++: mApp->PlaySample(Sexy::SOUND_SEEDLIFT);
                app.play_sample(crate::todlib::tod_foley::SOUND_SEEDLIFT);

                if b.m_tutorial_state == TutorialState::Level1PickUpPeashooter {
                    b.set_tutorial_state(TutorialState::Level1PlantPeashooter);
                } else if b.m_tutorial_state == TutorialState::Level2PickUpSunflower {
                    if self.seed_type == SeedType::Sunflower {
                        b.set_tutorial_state(TutorialState::Level2PlantSunflower);
                    } else {
                        b.set_tutorial_state(TutorialState::Level2RefreshSunflower);
                    }
                } else if b.m_tutorial_state == TutorialState::MoreSunPickUpSunflower {
                    if self.seed_type == SeedType::Sunflower {
                        b.set_tutorial_state(TutorialState::MoreSunPlantSunflower);
                    } else {
                        b.set_tutorial_state(TutorialState::MoreSunRefreshSunflower);
                    }
                } else if b.m_tutorial_state == TutorialState::WhackAZombiePickSeed
                    || b.m_tutorial_state == TutorialState::WhackAZombieBeforePickSeed
                {
                    b.set_tutorial_state(TutorialState::WhackAZombieCompleted);
                }

                self.deactivate();
            }
        }
    }

    /// 鼠标命中测试（对应 C++ MouseHitTest）
    pub fn mouse_hit_test(&self, x: i32, y: i32, hit_result: &mut HitResult) -> bool {
        if self.slot_machine_countdown > 0 || self.seed_type == SeedType::None {
            return false;
        }
        if x >= self.x + self.offset_x && x < self.x + self.offset_x + self.width
            && y >= self.y && y < self.y + self.height
        {
            hit_result.object = None;
            hit_result.object_type = GameObjectType::SeedPacket;
            return true;
        }
        false
    }
}


impl Default for SeedPacket {
    fn default() -> Self {
        SeedPacket::new()
    }
}

/// 种子银行 — 管理多个种子槽（对应 C++ SeedBank : GameObject）
pub struct SeedBank {
    pub base: GameObject,
    pub num_packets: i32,
    pub seed_packets: [SeedPacket; SEEDBANK_MAX as usize],
    pub cut_scene_darken: i32,
    pub conveyor_belt_counter: i32,
}

impl SeedBank {
    pub fn new() -> Self {
        SeedBank {
            base: GameObject::new(),
            num_packets: SEEDBANK_MAX,
            seed_packets: std::array::from_fn(|_| SeedPacket::new()),
            cut_scene_darken: 0,
            conveyor_belt_counter: 0,
        }
    }

    /// 绘制种子银行
    pub fn draw(&self, _g: &mut Graphics) {
        // 待 SeedPacket.cpp 翻译时实现完整绘制逻辑
        for packet in &self.seed_packets {
            // 每个种子槽的绘制由 SeedPacket::draw 处理
        }
    }

    /// 鼠标点击测试
    pub fn mouse_hit_test(&self, _x: i32, _y: i32, _hit_result: &mut HitResult) -> bool {
        false
    }
}

impl Default for SeedBank {
    fn default() -> Self {
        SeedBank::new()
    }
}

/// 绘制种子包（对应 C++ 全局函数 DrawSeedPacket，SeedPacket.cpp:285）
pub fn draw_seed_packet(
    g: &mut Graphics,
    x: f32,
    y: f32,
    seed_type: SeedType,
    imitater_type: SeedType,
    percent_dark: f32,
    grayness: i32,
    draw_cost: bool,
    use_current_cost: bool,
) {
    // 对应 C++: 模仿者替换为被模仿种子
    let mut a_seed_type = seed_type;
    if a_seed_type == SeedType::Imitater && imitater_type != SeedType::None {
        a_seed_type = imitater_type;
    }

    // 对应 C++: 灰化/变暗颜色化
    if grayness != 255 {
        g.set_color(&crate::framework::color::Color { r: grayness as u8, g: grayness as u8, b: grayness as u8, a: 255 });
        g.set_colorize_images(true);
    } else if percent_dark > 0.0 {
        g.set_color(&crate::framework::color::Color { r: 128, g: 128, b: 128, a: 255 });
        g.set_colorize_images(true);
    }

    // 对应 C++: 种子包背景类型（0-8）
    let a_packet_background = if seed_type == SeedType::Imitater {
        0
    } else if crate::lawn::plant::Plant::is_upgrade(a_seed_type) {
        1
    } else if seed_type == SeedType::BeghouledButtonCrater {
        3
    } else if seed_type == SeedType::BeghouledButtonShuffle {
        4
    } else if seed_type == SeedType::SlotMachineSun {
        5
    } else if seed_type == SeedType::SlotMachineDiamond {
        6
    } else if seed_type == SeedType::ZombiquariumSnorkle {
        7
    } else if seed_type == SeedType::ZombiquariumTrophy {
        8
    } else {
        2
    };

    // 对应 C++: 背景绘制（放大态/普通态图集 cel）
    let get_image = |a_key: &str| -> *mut crate::framework::graphics::image::Image {
        crate::lawn::lawn_app::LawnApp::instance().map_or(std::ptr::null_mut(), |app| {
            let a_rm = match app.base.resource_manager {
                Some(r) => r,
                None => return std::ptr::null_mut(),
            };
            unsafe { (*a_rm).get_image(a_key).as_image_ptr() }
        })
    };
    let a_scale_x = g.scale_x as f32;
    let a_scale_y = g.scale_y as f32;
    if a_scale_x > 1.0 {
        // 对应 C++: PvzpDrawImageCelScaledF(IMAGE_SEEDPACKET_LARGER, x, y, 0, 0, scaleX*0.5, scaleY*0.5)
        let a_larger = get_image("IMAGE_SEEDPACKET_LARGER");
        if !a_larger.is_null() {
            unsafe { g.draw_image_cel_dest(&*a_larger, &crate::framework::rect::Rect::new(x as i32, y as i32, (80.0 * a_scale_x * 0.5) as i32, (86.0 * a_scale_y * 0.5) as i32), 0); }
        }
    } else {
        // 对应 C++: PvzpDrawImageCelScaledF(IMAGE_SEEDS, x, y, aPacketBackground, 0, scaleX, scaleY)
        let a_seeds = get_image("IMAGE_SEEDS");
        if !a_seeds.is_null() {
            unsafe { g.draw_image_cel_dest(&*a_seeds, &crate::framework::rect::Rect::new(x as i32, y as i32, (80.0 * a_scale_x) as i32, (86.0 * a_scale_y) as i32), a_packet_background); }
        }
    }

    // 对应 C++: 图标缩放/偏移表（switch 约 40 项）
    let mut a_scale = 0.5f32;
    let mut a_draw_seed_in_middle = true;
    let mut a_offset_x = 5.0f32;
    let mut a_offset_y = 8.0f32;
    match a_seed_type {
        SeedType::Tallnut => { a_scale = 0.3; a_offset_x = 12.0; a_offset_y = 22.0; }
        SeedType::InstantCoffee => { a_scale = 0.55; a_offset_x = 0.0; a_offset_y = 9.0; }
        SeedType::Cobcannon => { a_scale = 0.26; a_offset_x = 6.0; a_offset_y = 22.0; }
        SeedType::Cactus => { a_offset_x = 9.0; a_offset_y = 13.0; }
        SeedType::PotatoMine => { a_scale = 0.4; a_offset_x = 8.0; a_offset_y = 12.0; }
        SeedType::Magnetshroom => { a_offset_y = 12.0; }
        SeedType::Fumeshroom | SeedType::Pumpkinshell | SeedType::Chomper
        | SeedType::Doomshroom | SeedType::Squash | SeedType::Hypnoshroom
        | SeedType::Spikeweed | SeedType::Spikerock | SeedType::Plantern
        | SeedType::Torchwood | SeedType::Tanglekelp => {
            a_scale = 0.4; a_offset_x = 8.0; a_offset_y = 12.0;
        }
        SeedType::Twinsunflower | SeedType::Gloomshroom => {
            a_scale = 0.45; a_offset_x = 7.0; a_offset_y = 14.0;
        }
        SeedType::Cattail => { a_scale = 0.45; a_offset_x = 8.0; a_offset_y = 13.0; }
        SeedType::Umbrella => { a_scale = 0.5; a_offset_x = 5.0; a_offset_y = 10.0; }
        SeedType::Kernelpult => { a_scale = 0.4; a_offset_x = 13.0; a_offset_y = 14.0; }
        SeedType::Cabbagepult => { a_scale = 0.4; a_offset_x = 15.0; a_offset_y = 14.0; }
        SeedType::Melonpult | SeedType::Wintermelon => {
            a_scale = 0.35; a_offset_x = 18.0; a_offset_y = 19.0;
        }
        SeedType::Gravebuster => { a_scale = 0.4; a_offset_x = 10.0; a_offset_y = 15.0; }
        SeedType::Splitpea => { a_scale = 0.45; a_offset_x = 12.0; a_offset_y = 12.0; }
        SeedType::Blover => { a_scale = 0.4; a_offset_x = 8.0; a_offset_y = 17.0; }
        SeedType::Starfruit => { a_scale = 0.5; a_offset_x = 6.0; a_offset_y = 8.0; }
        SeedType::Threepeater => { a_scale = 0.5; a_offset_x = 5.0; a_offset_y = 10.0; }
        SeedType::Gatlingpea => { a_scale = 0.5; a_offset_x = 2.0; a_offset_y = 8.0; }
        SeedType::ZombieNormal | SeedType::ZombieTrafficCone | SeedType::ZombiePail => {
            a_scale = 0.35; a_offset_x = -3.0; a_offset_y = -7.0;
        }
        SeedType::ZombieDancer => { a_scale = 0.375; a_offset_x = -19.0; a_offset_y = -40.0; }
        SeedType::ZombiePolevaulter => { a_scale = 0.35; a_offset_x = -8.0; a_offset_y = -12.0; }
        SeedType::ZombieLadder | SeedType::ZombieDigger | SeedType::ZombieScreenDoor
        | SeedType::ZombiePogo => {
            a_scale = 0.35; a_offset_x = -3.0; a_offset_y = -10.0;
        }
        SeedType::ZombieBungee => { a_scale = 0.3; a_offset_x = 1.0; a_offset_y = -1.0; }
        SeedType::ZombieFootball => { a_scale = 0.33; a_offset_x = -7.0; a_offset_y = -9.0; }
        SeedType::ZombieBalloon => { a_scale = 0.35; a_offset_x = -3.0; a_offset_y = -5.0; }
        SeedType::ZombieImp => { a_scale = 0.4; a_offset_x = -12.0; a_offset_y = -17.0; }
        SeedType::Zomboni => { a_scale = 0.23; a_offset_x = 12.0; a_offset_y = 3.0; }
        SeedType::ZombieGargantuar => { a_scale = 0.23; a_offset_x = 4.0; a_offset_y = 3.0; }
        SeedType::BeghouledButtonShuffle | SeedType::BeghouledButtonCrater
        | SeedType::SlotMachineSun | SeedType::SlotMachineDiamond
        | SeedType::ZombiquariumSnorkle | SeedType::ZombiquariumTrophy => {
            a_draw_seed_in_middle = false;
        }
        _ => {}
    }

    // 对应 C++: BigTime 特判
    if crate::lawn::lawn_app::LawnApp::instance()
        .map_or(false, |app| unsafe { (*app).game_mode == GameMode::ChallengeBigTime })
    {
        if a_seed_type == SeedType::Wallnut || a_seed_type == SeedType::Sunflower || a_seed_type == SeedType::Marigold {
            a_offset_x = 16.0;
            a_offset_y = 34.0;
        }
    }
    // 对应 C++: 巨型核桃缩放
    if a_seed_type == SeedType::GiantWallnut {
        a_scale *= 0.75;
        a_offset_x = 52.0;
        a_offset_y = 58.0;
    }
    a_offset_x = a_scale_x * a_offset_x;
    a_offset_y = a_scale_y * (a_offset_y + 1.0);

    // [TRANSLATION_NOTE]: C++ SeedPacketDrawSeed（种子图标绘制）依赖 IMAGE_SEED_BANK 等
    // 资源 cel 映射表，Rust 侧资源体系未全接入，暂略（aDrawSeedInMiddle 判定保留）

    if percent_dark > 0.0 {
        let a_darkness_height = (68.0 * percent_dark) as i32 + 2;
        // 对应 C++: 变暗区域 ClipRect + 灰 64 重绘背景与图标
        g.set_color(&crate::framework::color::Color { r: 64, g: 64, b: 64, a: 255 });
        g.set_colorize_images(true);
        g.clip_rect_xywh(x as i32, y as i32, 80, a_darkness_height);
        let a_seeds = get_image("IMAGE_SEEDS");
        if !a_seeds.is_null() {
            unsafe { g.draw_image_cel_dest(&*a_seeds, &crate::framework::rect::Rect::new(x as i32, y as i32, (80.0 * a_scale_x) as i32, (86.0 * a_scale_y) as i32), a_packet_background); }
        }
    }

    if draw_cost {
        // 对应 C++: 成本字符串（加速定价依赖 board，简化为 Plant::GetCost）
        // [TRANSLATION_NOTE]: PlantUsesAcceleratedPricing/GetCurrentPlantCost 依赖 Board，暂略
        let a_cost = crate::lawn::plant::Plant::get_cost(a_seed_type, imitater_type);
        let a_cost_str = format!("{}", a_cost);
        // 对应 C++: FONT_PICO129 黑字绘制（aTextOffsetX = 32 - StringWidth）
        let mut a_font = crate::framework::graphics::font::Font::new("Pico", 12);
        a_font.ascent = 13;
        a_font.font_height = 12;
        g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
        g.set_color(&crate::framework::color::Color { r: 0, g: 0, b: 0, a: 255 });
        let a_text_width = a_font.string_width(&a_cost_str);
        let a_text_offset_x = 32 - a_text_width;
        let a_text_offset_y = 13 + 54;
        g.draw_string(&a_cost_str, (x + a_text_offset_x as f32) as i32, (y + a_text_offset_y as f32) as i32);
    }

    g.set_colorize_images(false);
}



