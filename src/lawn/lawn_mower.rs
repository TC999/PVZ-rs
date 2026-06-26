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

    /// 启动割草机
    pub fn start_mower(&mut self) {
        self.mowing = true;
        self.mower_state = LawnMowerState::RollingIn;
    }

    /// 启动割草机（与 start_mower 逻辑相同）
    pub fn start_mowing(&mut self) {
        self.start_mower();
    }

    /// 更新割草机状态
    pub fn update(&mut self) {
        if self.dead { return; }
        // TODO: 完整更新逻辑（来自 LawnMower.cpp）
    }

    /// 绘制割草机
    pub fn draw(&self, _g: &mut Graphics) {
        // TODO: 根据 mower_state 和 mower_type 绘制
    }

    /// 获取割草机攻击碰撞矩形
    pub fn get_mower_rect(&self) -> Rect {
        Rect::new(self.pos_x as i32, self.pos_y as i32, 80, 80)
    }

    /// 割草机死亡
    pub fn die(&mut self) {
        self.dead = true;
    }

    /// 粉碎割草机
    pub fn squish_mower(&mut self) {
        self.mower_state = LawnMowerState::Squished;
        self.dead = true;
    }

    /// 启用超级割草机
    pub fn enable_super_mower(&mut self, _enable: bool) {
        // TODO: 设置 mowerType = SuperMower
    }

    /// 水池高度更新
    pub fn update_pool(&mut self) {
        // TODO: 泳池模式高度更新逻辑
    }

    /// 碾压僵尸
    pub fn mow_zombie(&mut self, _zombie: *mut crate::lawn::zombie::Zombie) {
        // TODO: 僵尸碾压逻辑
    }
}

impl Default for LawnMower {
    fn default() -> Self {
        LawnMower::new()
    }
}
