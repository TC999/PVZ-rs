// PvZ Portable Rust 翻译 — Board（游戏棋盘）
// 对应 C++ src/Lawn/Board.h / Board.cpp
//
// Board 是整个游戏的核心：管理植物、僵尸、子弹、硬币等所有实体的
// 更新和绘制。这是 PvZ 游戏逻辑的心脏。

use crate::lawn::game_enums::*;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::plant::Plant;
use crate::lawn::zombie::Zombie;
use crate::lawn::projectile::Projectile;
use crate::lawn::coin::Coin;
use crate::lawn::lawn_mower::LawnMower;
use crate::lawn::grid_item::GridItem;
use crate::lawn::seed_packet::SeedPacket;
use crate::lawn::challenge::Challenge;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;
use crate::framework::color::Color;
use crate::todlib::tod_particle::ParticleSystem;
use crate::todlib::reanimator::Reanimation;

pub const MAX_GRID_SIZE_X: usize = 9;
pub const MAX_GRID_SIZE_Y: usize = 6;
pub const MAX_PLANTS: usize = 200;
pub const MAX_ZOMBIES: usize = 200;
pub const MAX_PROJECTILES: usize = 200;
pub const MAX_COINS: usize = 200;
pub const MAX_LAWN_MOWERS: usize = 6;
pub const MAX_GRID_ITEMS: usize = 50;

/// 将行号转换为 Y 坐标
pub fn row_to_y(row: i32) -> i32 {
    80 + row * 100
}

/// 将 Y 坐标转换为行号
pub fn y_to_row(y: i32) -> i32 {
    ((y - 80) / 100).clamp(0, 5)
}

/// 将 X 坐标转换为列号
pub fn x_to_col(x: i32) -> i32 {
    ((x - 40) / 80).clamp(0, 8)
}

/// 游戏棋盘 — 管理整个关卡的所有游戏元素
pub struct Board {
    pub app: Option<*mut LawnApp>,

    // 游戏状态
    pub game_mode: GameMode,
    pub level: i32,
    pub m_width: i32,
    pub m_height: i32,
    pub m_board_result: BoardResult,
    pub m_advice: AdviceType,

    // 网格
    pub grid_plants: [[Option<PlantID>; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],

    // 实体列表
    pub plants: Vec<Plant>,
    pub zombies: Vec<Zombie>,
    pub projectiles: Vec<Projectile>,
    pub coins: Vec<Coin>,
    pub lawn_mowers: Vec<LawnMower>,
    pub grid_items: Vec<GridItem>,
    pub seed_bank: Vec<SeedPacket>,

    // 阳光
    pub m_sun_count: i32,
    pub m_sun_countdown: i32,

    // 金钱
    pub m_coin_bank: i64,

    // 关卡状态
    pub m_wave_count: i32,
    pub m_current_wave: i32,
    pub m_total_waves: i32,
    pub m_wave_countdown: i32,
    pub m_zombie_countdown: i32,
    pub m_flag_countdown: i32,
    pub m_level_complete: bool,
    pub m_game_over: bool,
    pub m_game_over_countdown: i32,

    // 草地
    pub m_background_type: BackgroundType,
    pub m_pool_occupied: [bool; MAX_GRID_SIZE_Y],
    pub m_roof: bool,
    pub m_fog: bool,

    // 挑战/小游戏
    pub challenge: Option<Challenge>,

    // 计时器
    pub m_tutorial_state: TutorialState,
    pub m_paused: bool,
    pub m_update_count: i32,

    // 冻结/减速效果
    pub m_ice_timer: i32,
    pub m_ice_flash_count: i32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            app: None,
            game_mode: GameMode::Adventure,
            level: 1,
            m_width: 800,
            m_height: 600,
            m_board_result: BoardResult::None,
            m_advice: AdviceType::None,
            grid_plants: [[None; MAX_GRID_SIZE_X]; MAX_GRID_SIZE_Y],
            plants: Vec::with_capacity(MAX_PLANTS),
            zombies: Vec::with_capacity(MAX_ZOMBIES),
            projectiles: Vec::with_capacity(MAX_PROJECTILES),
            coins: Vec::with_capacity(MAX_COINS),
            lawn_mowers: Vec::with_capacity(MAX_LAWN_MOWERS),
            grid_items: Vec::with_capacity(MAX_GRID_ITEMS),
            seed_bank: Vec::new(),
            m_sun_count: 50,
            m_sun_countdown: SUN_COUNTDOWN,
            m_coin_bank: 0,
            m_wave_count: 0,
            m_current_wave: 0,
            m_total_waves: 0,
            m_wave_countdown: 0,
            m_zombie_countdown: ZOMBIE_COUNTDOWN_FIRST_WAVE,
            m_flag_countdown: 0,
            m_level_complete: false,
            m_game_over: false,
            m_game_over_countdown: 0,
            m_background_type: BackgroundType::Day,
            m_pool_occupied: [false; MAX_GRID_SIZE_Y],
            m_roof: false,
            m_fog: false,
            challenge: None,
            m_tutorial_state: TutorialState::Off,
            m_paused: false,
            m_update_count: 0,
            m_ice_timer: 0,
            m_ice_flash_count: 0,
        }
    }

    /// 初始化关卡
    pub fn board_init(&mut self, app: *mut LawnApp) {
        self.app = Some(app);

        // 设置关卡参数
        self.setup_level();

        // 初始化草地类型
        self.setup_background();

        // 初始化割草机
        for row in 0..6 {
            let mut mower = LawnMower::new();
            mower.lawn_mower_initialize(row);
            self.lawn_mowers.push(mower);
        }

        // 初始化阳光槽
        for i in 0..SEEDBANK_MAX as usize {
            let mut packet = SeedPacket::new();
            packet.seed_packet_initialize(i as i32);
            self.seed_bank.push(packet);
        }

        // 设置波次信息
        self.setup_waves();
    }

    /// 设置关卡
    fn setup_level(&mut self) {
        // 根据关卡和游戏模式设置参数
        match self.level {
            1 => {
                self.m_total_waves = 1;
                self.m_background_type = BackgroundType::Day;
            },
            2..=10 => {
                self.m_total_waves = 2;
                self.m_background_type = BackgroundType::Day;
            },
            _ => {
                self.m_total_waves = 5;
            }
        }
    }

    /// 设置背景
    fn setup_background(&mut self) {
        match self.m_background_type {
            BackgroundType::Day => {},
            BackgroundType::Night => {},
            BackgroundType::Pool => {
                self.m_pool_occupied[2] = true;
                self.m_pool_occupied[3] = true;
            },
            BackgroundType::Fog => {
                self.m_pool_occupied[2] = true;
                self.m_pool_occupied[3] = true;
                self.m_fog = true;
            },
            BackgroundType::Roof => {
                self.m_roof = true;
            },
            _ => {}
        }
    }

    /// 设置波次
    fn setup_waves(&mut self) {
        // 根据关卡设置僵尸波次
    }

    /// 主更新循环
    pub fn update(&mut self) {
        if self.m_paused || self.m_board_result != BoardResult::None { return; }
        self.m_update_count += 1;

        // 更新阳光产生
        self.update_sun();

        // 更新植物
        for plant in &mut self.plants {
            plant.update();
        }

        // 更新僵尸
        for zombie in &mut self.zombies {
            zombie.update();
        }

        // 更新子弹
        for projectile in &mut self.projectiles {
            projectile.update();
        }

        // 更新硬币
        for coin in &mut self.coins {
            coin.update();
        }

        // 更新割草机
        for mower in &mut self.lawn_mowers {
            mower.update();
        }

        // 碰撞检测
        self.check_collisions();

        // 更新波次
        self.update_waves();

        // 清理已死亡的实体
        self.cleanup_dead();

        // 检查游戏状态
        self.check_game_state();
    }

    /// 更新阳光产生
    fn update_sun(&mut self) {
        self.m_sun_countdown -= 1;
        if self.m_sun_countdown <= 0 {
            // 自然产生阳光
            let x = crate::todlib::tod_common::rand_range_int(40, 760);
            self.add_coin(x as f32, -30.0, CoinType::Sun, CoinMotion::FromSky);
            self.m_sun_countdown = SUN_COUNTDOWN + crate::todlib::tod_common::rand_range_int(0, SUN_COUNTDOWN_RANGE);
        }
    }

    /// 更新波次
    fn update_waves(&mut self) {
        if self.m_current_wave >= self.m_total_waves { return; }

        self.m_zombie_countdown -= 1;
        if self.m_zombie_countdown <= 0 {
            self.spawn_zombie_wave();
        }
    }

    /// 生成一波僵尸
    fn spawn_zombie_wave(&mut self) {
        self.m_current_wave += 1;

        // 根据关卡和波次生成不同的僵尸
        let count = match self.level {
            1 => 3,
            2..=5 => 3 + self.m_current_wave,
            _ => 5 + self.m_current_wave,
        };

        for _ in 0..count {
            let ztype = if self.level >= 3 && crate::todlib::tod_common::rand_range_int(0, 100) < 30 {
                ZombieType::TrafficCone
            } else {
                ZombieType::Normal
            };

            let row = crate::todlib::tod_common::rand_range_int(0, 5);
            let mut zombie = Zombie::new();
            zombie.zombie_initialize(row, ztype, false, None, self.m_current_wave);
            self.zombies.push(zombie);
        }

        self.m_zombie_countdown = ZOMBIE_COUNTDOWN + crate::todlib::tod_common::rand_range_int(0, ZOMBIE_COUNTDOWN_RANGE);

        // 旗子波次
        if self.m_current_wave % 5 == 0 {
            self.m_flag_countdown = FLAG_RAISE_TIME;
        }
    }

    /// 碰撞检测
    fn check_collisions(&mut self) {
        // 子弹与僵尸碰撞
        let mut projectiles_to_remove = Vec::new();
        for (i, proj) in self.projectiles.iter().enumerate() {
            if proj.dead { continue; }
            let proj_rect = proj.get_projectile_rect();

            for zombie in &mut self.zombies {
                if zombie.dead { continue; }
                if zombie.base.row != proj.base.row { continue; }

                let z_rect = zombie.get_zombie_rect();
                if proj_rect.intersects(&z_rect) {
                    zombie.take_damage(proj.damage, proj.damage_flags);
                    projectiles_to_remove.push(i);
                    break;
                }
            }
        }

        for &i in projectiles_to_remove.iter().rev() {
            if i < self.projectiles.len() {
                self.projectiles[i].dead = true;
            }
        }

        // 僵尸与植物碰撞
        for zombie in &mut self.zombies {
            if zombie.dead || zombie.is_eating { continue; }
            let z_attack = zombie.get_zombie_attack_rect();

            for plant in &mut self.plants {
                if plant.dead || !plant.is_on_board { continue; }
                if plant.base.row != zombie.base.row { continue; }

                let p_rect = plant.plant_rect;
                if z_attack.intersects(&p_rect) {
                    zombie.eat_plant(plant);
                    break;
                }
            }
        }

        // 僵尸吃植物
        let zombie_eat_data: Vec<(usize, i32, i32)> = self.zombies.iter().enumerate()
            .filter(|(_, z)| !z.dead && z.is_eating && z.zombie_age % 4 == 0)
            .map(|(i, z)| (i, z.base.row, z.target_col))
            .collect();
        for (zombie_idx, row, target_col) in zombie_eat_data {
            if let Some(plant) = self.find_plant_at(row, target_col as usize) {
                plant.plant_health -= 20; // EAT_DAMAGE = 20
                if plant.plant_health <= 0 {
                    plant.die();
                    if let Some(zombie) = self.zombies.get_mut(zombie_idx) {
                        zombie.stop_eating();
                    }
                }
            } else {
                if let Some(zombie) = self.zombies.get_mut(zombie_idx) {
                    zombie.stop_eating();
                }
            }
        }

        // 僵尸与割草机碰撞
        for zombie in &mut self.zombies {
            if zombie.dead { continue; }
            let z_rect = zombie.get_zombie_rect();

            for mower in &mut self.lawn_mowers {
                if mower.mowing || mower.dead { continue; }
                if mower.base.row != zombie.base.row { continue; }

                let m_rect = mower.get_mower_rect();
                if z_rect.intersects(&m_rect) {
                    mower.start_mowing();
                    zombie.die_no_loot();
                }
            }
        }
    }

    /// 在指定行列查找植物
    pub fn find_plant_at(&mut self, row: i32, col: usize) -> Option<&mut Plant> {
        if col >= MAX_GRID_SIZE_X { return None; }
        let row_idx = row as usize;
        if row_idx >= MAX_GRID_SIZE_Y { return None; }

        if let Some(pid) = self.grid_plants[row_idx][col] {
            self.plants.iter_mut().find(|p| /* p.id == pid */ false)
        } else {
            None
        }
    }

    /// 在指定行查找僵尸（从指定列开始向左找最近的）
    pub fn find_zombie_in_row(&mut self, _row: i32, _from_col: i32) -> Option<&mut Zombie> {
        // 查找该行最左边的僵尸
        let first_row = self.zombies.first().map(|z| z.base.row).unwrap_or(-1);
        for zombie in &mut self.zombies {
            if zombie.dead { continue; }
            if zombie.base.row == first_row { // 简化：查找第一个
                return Some(zombie);
            }
        }
        None
    }

    /// 添加硬币
    pub fn add_coin(&mut self, x: f32, y: f32, coin_type: CoinType, motion: CoinMotion) {
        let mut coin = Coin::new();
        coin.coin_initialize(x, y, coin_type, motion);
        coin.base.board = Some(self as *mut Board); // 注意：这是 unsafe 的简化
        if let Some(app) = self.app {
            coin.base.app = Some(app);
        }
        self.coins.push(coin);
    }

    /// 添加子弹
    pub fn add_projectile(&mut self, x: f32, y: f32, row: i32, seed_type: SeedType) {
        let mut proj = Projectile::new();
        proj.projectile_initialize(x, y, row, seed_type);
        if let Some(app) = self.app {
            proj.base.app = Some(app);
        }
        proj.base.board = Some(self as *mut Board);
        self.projectiles.push(proj);
    }

    /// 添加植物到棋盘
    pub fn add_plant(&mut self, grid_x: i32, grid_y: i32, seed_type: SeedType, _imitater_type: SeedType) {
        let mut plant = Plant::new();
        plant.plant_initialize(grid_x, grid_y, seed_type, SeedType::None);
        plant.base.board = Some(self as *mut Board);
        if let Some(app) = self.app {
            plant.base.app = Some(app);
        }

        let x = LAWN_XMIN + grid_x * 80 + 10;
        let y = LAWN_YMIN + grid_y * 100 + 10;
        plant.base.x = x;
        plant.base.y = y;
        plant.pos_x = x as f32;
        plant.pos_y = y as f32;

        self.plants.push(plant);
    }

    /// 清理死亡实体
    fn cleanup_dead(&mut self) {
        self.plants.retain(|p| !p.dead);
        self.zombies.retain(|z| !z.dead);
        self.projectiles.retain(|p| !p.dead);
        self.coins.retain(|c| !c.dead);
    }

    /// 检查游戏状态（胜利/失败）
    fn check_game_state(&mut self) {
        if self.m_game_over { return; }
        if self.m_level_complete { return; }

        // 检查是否有僵尸到达房子
        for zombie in &self.zombies {
            if !zombie.dead && zombie.pos_x < -50.0 {
                self.game_over();
                return;
            }
        }

        // 检查是否所有波次已完成且无僵尸
        if self.m_current_wave >= self.m_total_waves && self.zombies.is_empty() {
            self.level_complete();
        }
    }

    /// 游戏结束
    pub fn game_over(&mut self) {
        self.m_game_over = true;
        self.m_board_result = BoardResult::Lost;
        self.m_game_over_countdown = 200;
    }

    /// 关卡完成
    pub fn level_complete(&mut self) {
        self.m_level_complete = true;
        self.m_board_result = BoardResult::Won;
    }

    /// 主绘制函数
    pub fn draw(&self, g: &mut Graphics) {
        // 绘制背景
        self.draw_background(g);

        // 绘制格子物品
        for item in &self.grid_items {
            item.draw(g);
        }

        // 绘制植物
        for plant in &self.plants {
            plant.draw(g);
        }

        // 绘制僵尸
        for zombie in &self.zombies {
            zombie.draw(g);
        }

        // 绘制子弹
        for projectile in &self.projectiles {
            projectile.draw(g);
        }

        // 绘制硬币
        for coin in &self.coins {
            coin.draw(g);
        }

        // 绘制割草机
        for mower in &self.lawn_mowers {
            mower.draw(g);
        }

        // 绘制 UI（阳光计数、种子槽等）
        self.draw_ui(g);
    }

    /// 绘制背景
    fn draw_background(&self, _g: &Graphics) {
        // 根据背景类型绘制草地/夜晚/水池/屋顶等
    }

    /// 绘制 UI
    fn draw_ui(&self, _g: &Graphics) {
        // 绘制阳光数量、种子选择器等
    }

    /// 点击事件
    pub fn mouse_down(&mut self, x: i32, y: i32, _click_count: i32) {
        // 检查是否点击了种子槽
        for packet in &self.seed_bank {
            // 点击种子槽开始种植
        }

        // 检查是否点击了阳光/硬币
        for coin in &mut self.coins {
            if coin.is_sun && !coin.dead {
                let coin_rect = Rect::new(
                    coin.pos_x as i32 - 15,
                    coin.pos_y as i32 - 15,
                    30,
                    30,
                );
                if coin_rect.contains(x, y) {
                    coin.collect();
                    if coin.coin_type == CoinType::Sun {
                        self.m_sun_count += coin.value;
                    }
                    break;
                }
            }
        }
    }

    /// 键盘事件
    pub fn key_down(&mut self, key: i32) {
        match key {
            // 暂停
            80 => { // 'P'
                self.m_paused = !self.m_paused;
            },
            _ => {}
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
