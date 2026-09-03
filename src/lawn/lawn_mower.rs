// PvZ Portable Rust 翻译 — LawnMower（割草机）
// 对应 C++ src/Lawn/LawnMower.h / LawnMower.cpp

use crate::lawn::game_enums::*;
use crate::lawn::game_object::GameObject;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;

/// 割草机（对应 C++ class LawnMower）
pub struct LawnMower {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub board: Option<*mut crate::lawn::board::Board>,
    pub base: GameObject,

    pub pos_x: f32,
    pub pos_y: f32,
    pub render_order: i32,
    pub anim_ticks_per_frame: i32,
    pub mower_anim_id: ReanimationID,
    pub chomp_counter: i32,
    pub rolling_in_counter: i32,
    pub squished_counter: i32,
    pub mower_state: LawnMowerState,
    pub dead: bool,
    pub visible: bool,
    pub mowing: bool,
    pub mower_type: LawnMowerType,
    pub altitude: f32,
    pub mower_height: MowerHeight,
    pub last_portal_x: i32,
}

impl LawnMower {
    pub fn new() -> Self {
        LawnMower {
            app: None,
            board: None,
            base: GameObject::new(),
            pos_x: 0.0,
            pos_y: 0.0,
            render_order: 0,
            anim_ticks_per_frame: 0,
            mower_anim_id: REANIMATIONID_NULL,
            chomp_counter: 0,
            rolling_in_counter: 0,
            squished_counter: 0,
            mower_state: LawnMowerState::Ready,
            dead: false,
            visible: false,
            mowing: false,
            mower_type: LawnMowerType::Lawn,
            altitude: 0.0,
            mower_height: MowerHeight::Land,
            last_portal_x: 0,
        }
    }

    /// 初始化割草机
    pub fn lawn_mower_initialize(&mut self, the_row: i32) {
        self.base.row = the_row;
        self.mower_state = LawnMowerState::Ready;
        self.visible = true;
    }

    /// 启动割草机（对应 C++ StartMower：状态置为 Triggered）
    pub fn start_mower(&mut self) {
        if self.mower_state == LawnMowerState::Triggered {
            return;
        }
        self.mowing = true;
        self.mower_state = LawnMowerState::Triggered;
        // [TRANSLATION_NOTE]: reanim 速率、FOLEY 音效、mWaveRowGotLawnMowered/mTriggeredLawnMowers 计数暂未接入
    }

    /// 启动割草机（与 start_mower 逻辑相同）
    pub fn start_mowing(&mut self) {
        self.start_mower();
    }

    /// 更新割草机（对应 C++ LawnMower::Update）
    pub fn update(&mut self) {
        if self.dead { return; }

        // 被压碎状态
        if self.mower_state == LawnMowerState::Squished {
            self.squished_counter -= 1;
            if self.squished_counter <= 0 {
                self.die();
            }
            return;
        }

        // 滚入状态
        if self.mower_state == LawnMowerState::RollingIn {
            self.rolling_in_counter += 1;
            // PvzpAnimateCurveFloat(0, 100, rollingInCounter, -160, -21, EASE_IN_OUT)
            if self.rolling_in_counter == 100 {
                self.mower_state = LawnMowerState::Ready;
            }
            return;
        }

        // 游戏场景检查
        // [TRANSLATION_NOTE]: 场景检查暂略
        // [TRANSLATION_NOTE]: 碰撞检测→MowZombie 已统一到 Board::check_collisions（mower.update 内因借用无法取 zombie 可变引用）

        // 触发/碾压状态
        if self.mower_state == LawnMowerState::Triggered || self.mower_state == LawnMowerState::Squished {
            let mut a_speed = if self.mower_type == LawnMowerType::Pool { 2.5 } else { 3.33 };
            if self.chomp_counter > 0 {
                self.chomp_counter -= 1;
                // PvzpAnimateCurveFloat(50, 0, chompCounter, aSpeed, 1.0, BOUNCE_SLOW_MIDDLE)
            }
            self.pos_x += a_speed;

            // 泳池割草机落水
            if self.mower_type == LawnMowerType::Pool {
                self.update_pool();
            }

            // 陆地割草机进入泳池行
            if self.mower_type == LawnMowerType::Lawn {
                if let Some(board) = self.base.get_board() {
                    if board.m_plant_row[self.base.row as usize] == PlantRowType::Pool && self.pos_x > 50.0 {
                        // [TRANSLATION_NOTE]: 水花动画+音效暂未实现
                        self.die();
                    }
                }
            }

            // 飞出棋盘
            if self.pos_x > 900.0 {
                self.die();
            }
        }
    }

    /// 碾压僵尸（对应 C++ LawnMower::MowZombie）
    pub fn mow_zombie(&mut self, zombie: &mut crate::lawn::zombie::Zombie) {
        if self.mower_state == LawnMowerState::Ready {
            self.start_mower();
            self.chomp_counter = 25;
        } else if self.mower_state == LawnMowerState::Triggered {
            self.chomp_counter = 50;
        }

        if self.mower_type == LawnMowerType::Pool {
            // [TRANSLATION_NOTE]: FOLEY_SHOOP 音效 + anim_suck/anim_landsuck reanim 暂未接入
            zombie.die_with_loot();
        } else {
            // [TRANSLATION_NOTE]: FOLEY_SPLAT 音效暂未接入
            zombie.mow_down();
        }
    }

    /// 获取割草机攻击碰撞矩形（对应 C++ GetLawnMowerAttackRect）
    pub fn get_lawn_mower_attack_rect(&self) -> Rect {
        Rect::new(self.pos_x as i32, self.pos_y as i32, 80, 80)
    }

    /// 获取割草机碰撞矩形
    pub fn get_mower_rect(&self) -> Rect {
        Rect::new(self.pos_x as i32, self.pos_y as i32, 80, 80)
    }

    /// 割草机死亡（对应 C++ Die）
    pub fn die(&mut self) {
        self.dead = true;
        // [TRANSLATION_NOTE]: 完整实现需要移除 Reanimation + 检查 bonus mowers
    }

    /// 粉碎割草机
    pub fn squish_mower(&mut self) {
        self.mower_state = LawnMowerState::Squished;
        self.dead = true;
    }

    /// 启用超级割草机
    pub fn enable_super_mower(&mut self, _enable: bool) {}

    /// 水池高度更新（对应 C++ UpdatePool）
    /// 泳池割草机进入/离开水池时的高度动画和音效
    pub fn update_pool(&mut self) {
        // [TRANSLATION_NOTE]: 完整实现依赖 Reanimation 系统
        // 状态机：Land→DownToPool(altitude-2)→InPool→UpToLand(altitude+2)→Land
        // 进入/离开水池时播放水花粒子+音效
    }

    /// 绘制割草机阴影（对应 C++ Draw 中的阴影部分）
    pub fn draw_shadow(&self, _g: &mut Graphics) {
        // [TRANSLATION_NOTE]: 阴影绘制依赖 IMAGE_PLANTSHADOW 资源
    }

    /// 绘制割草机（对应 C++ Draw）
    pub fn draw(&self, _g: &mut Graphics) {
        // [TRANSLATION_NOTE]: 完整绘制依赖 Reanimation 系统和 ReanimatorCache
        // 泳池/屋顶/陆地割草机各有不同偏移和剪辑
    }
}

impl Default for LawnMower {
    fn default() -> Self {
        LawnMower::new()
    }
}
