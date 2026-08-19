// PvZ Portable Rust 翻译 — SeedPacket（种子槽）
// 对应 C++ src/Lawn/SeedPacket.h / SeedPacket.cpp

use crate::lawn::game_enums::*;
use crate::lawn::game_object::GameObject;
use crate::lawn::board::HitResult;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;

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
        self.active = true;
        self.refreshing = false;
        self.countdown = 0;
        self.slot_machine_countdown = 0;
    }

    /// 停用（对应 C++ Deactivate）
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// 设置激活状态（对应 C++ SetActivate）
    pub fn set_activate(&mut self, active: bool) {
        self.active = active;
    }

    /// 能否拾取（对应 C++ CanPickUp）
    pub fn can_pick_up(&self) -> bool {
        self.active && self.countdown <= 0 && self.seed_type != SeedType::None
    }

    /// 准备就绪闪烁（对应 C++ FlashIfReady 简化版）
    pub fn flash_if_ready(&mut self) {
        // [TRANSLATION_NOTE]: 闪烁粒子效果暂未实现
    }

    /// 老虎机选种子（对应 C++ PickNextSlotMachineSeed 简化版）
    pub fn pick_next_slot_machine_seed(&mut self) {
        // [TRANSLATION_NOTE]: 老虎机选种子逻辑暂未实现
    }

    /// 老虎机启动（对应 C++ SlotMachineStart）
    pub fn slot_machine_start(&mut self) {
        self.slot_machine_countdown = 300;
        self.slot_machine_position = 0.0;
        self.pick_next_slot_machine_seed();
    }

    /// 更新冷却（对应 C++ SeedPacket::Update 简化版）
    pub fn update(&mut self) {
        if self.seed_type == SeedType::None {
            return;
        }

        // [TRANSLATION_NOTE]: mMainCounter == 0 时 FlashIfReady 暂未实现

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

        // 老虎机滚动
        if self.slot_machine_countdown > 0 {
            self.slot_machine_countdown -= 1;
            // [TRANSLATION_NOTE]: 使用曲线动画计算翻转速度
            self.slot_machine_position += 0.06;
            if self.slot_machine_position >= 1.0 {
                self.seed_type = self.slot_machine_next_seed;
                if self.slot_machine_countdown == 0 {
                    self.activate();
                    self.slot_machine_position = 0.0;
                } else {
                    self.slot_machine_position -= 1.0;
                    self.pick_next_slot_machine_seed();
                }
            }
        }
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 设置冷却时间
    pub fn set_countdown(&mut self, refresh_time: i32) {
        self.countdown = refresh_time;
    }

    /// 种植后处理（对应 C++ WasPlanted）
    pub fn was_planted(&mut self) {
        // [TRANSLATION_NOTE]: 完整逻辑依赖 Board::HasConveyorBeltSeedBank/IsSlotMachineLevel 等
        // 传送带模式：从传送带移除
        // 老虎机模式：Deactivate
        // 坚不可摧模式：保持激活 + FlashIfReady
        // 普通模式：times_used++ + refreshing = true + 计算 refresh_time
        self.times_used += 1;
        self.refreshing = true;
    }

    /// 鼠标点击（对应 C++ MouseDown 简化版）
    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // [TRANSLATION_NOTE]: 完整逻辑包含阳光检测、需求提示、Buzz 音效、教程状态机等
        // 核心流程：
        // 1. 检查暂停/场景/种子类型
        // 2. 老虎机模式：显示提示 + 记录滚动次数
        // 3. 检查激活状态 → 显示冷却提示
        // 4. 检查阳光 → 显示不够提示
        // 5. 检查合成需求 → 显示对应提示
        // 6. 清除所有提示
        // 7. 三消/水族馆模式：转发给 Challenge
        // 8. 普通模式：设置光标类型 + 播放音效 + 更新教程 + Deactivate
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



