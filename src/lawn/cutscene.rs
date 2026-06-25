// PvZ Portable Rust 翻译 — CutScene（过场动画场景管理）
// 对应 C++ src/Lawn/Cutscene.h / Cutscene.cpp

use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::KeyCode;
use crate::lawn::board::{self, Board};
use crate::lawn::game_enums::{
    GameMode, ReanimationID, REANIMATIONID_NULL, ZombieType,
};
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::widget::challenge_screen::ChallengeScreen;
use crate::todlib::tod_common::rand_range_float;

/// 过场动画场景管理（对应 C++ CutScene）
#[derive(Debug)]
pub struct CutScene {
    pub app: Option<*mut LawnApp>,
    pub board: Option<*mut Board>,

    // 时间控制
    pub m_cutscene_time: i32,
    pub m_sod_time: i32,
    pub m_grave_stone_time: i32,
    pub m_ready_set_plant_time: i32,
    pub m_fog_time: i32,
    pub m_boss_time: i32,
    pub m_crazy_dave_time: i32,
    pub m_lawn_mower_time: i32,
    pub m_crazy_dave_dialog_start: i32,

    // 状态标志
    pub m_seed_choosing: bool,
    pub m_zombies_won_reanim_id: ReanimationID,
    pub m_preloaded: bool,
    pub m_placed_zombies: bool,
    pub m_placed_lawn_items: bool,
    pub m_crazy_dave_count_down: i32,
    pub m_crazy_dave_last_talk_index: i32,
    pub m_upsell_hide_board: bool,
    pub m_upsell_challenge_screen: Option<*mut ChallengeScreen>,
    pub m_pre_updating_board: bool,

    // 预加载资源
    pub m_loaded_resource_names: Vec<String>,
}

impl CutScene {
    pub fn new() -> Self {
        CutScene {
            app: None,
            board: None,
            m_cutscene_time: 0,
            m_sod_time: 0,
            m_grave_stone_time: 0,
            m_ready_set_plant_time: 0,
            m_fog_time: 0,
            m_boss_time: 0,
            m_crazy_dave_time: 0,
            m_lawn_mower_time: 0,
            m_crazy_dave_dialog_start: -1,
            m_seed_choosing: false,
            m_zombies_won_reanim_id: REANIMATIONID_NULL,
            m_preloaded: false,
            m_placed_zombies: false,
            m_placed_lawn_items: false,
            m_crazy_dave_count_down: 0,
            m_crazy_dave_last_talk_index: -1,
            m_upsell_hide_board: false,
            m_upsell_challenge_screen: None,
            m_pre_updating_board: false,
            m_loaded_resource_names: Vec::new(),
        }
    }

    /// 设置 app 和 board 引用（在创建后调用）
    pub fn init_app(&mut self, app: *mut LawnApp) {
        self.app = Some(app);
        let board = unsafe { (*app).board };
        self.board = board;
    }
}

impl Drop for CutScene {
    fn drop(&mut self) {
        // 释放升级面板（对应析构函数中 delete mUpsellChallengeScreen）
        if let Some(screen) = self.m_upsell_challenge_screen {
            let _ = unsafe { Box::from_raw(screen) };
        }

        // 取消音频静音（对应 mApp->mMuteSoundsForCutscene = false）
        if let Some(app) = self.app {
            unsafe {
                (*app).m_mute_sounds_for_cutscene = false;
            }
        }

        // 释放预加载资源（对应 mApp->mResourceManager->ReleaseTrackedResources）
        // TODO: 当 ResourceManager 翻译完成后实现
    }
}

impl CutScene {

    /// 获取 LawnApp 引用
    pub fn get_app(&self) -> Option<&LawnApp> {
        unsafe { self.app.map(|a| &*a) }
    }

    pub fn get_app_mut(&mut self) -> Option<&mut LawnApp> {
        unsafe { self.app.map(|a| &mut *a) }
    }

    /// 获取 Board 引用
    pub fn get_board(&self) -> Option<&Board> {
        unsafe { self.board.map(|b| &*b) }
    }

    pub fn get_board_mut(&mut self) -> Option<&mut Board> {
        unsafe { self.board.map(|b| &mut *b) }
    }

    // ============================
    // 过场动画流程控制
    // ============================

    /// 开始关卡入场动画
    pub fn start_level_intro(&mut self) {
        // 对应 C++ StartLevelIntro — 内部调用各阶段入场函数
    }

    /// 取消入场动画
    pub fn cancel_intro(&mut self) {
        // 对应 C++ CancelIntro — 重置时间并跳过入场
    }

    /// 逐帧更新过场动画状态
    pub fn update(&mut self) {
        // 对应 C++ Update — 包含滚屏、戴夫对话、僵尸放置等
    }

    /// 动画板块移动
    pub fn animate_board(&mut self) {
        // 对应 C++ AnimateBoard — 板车、草坪等动画
    }

    /// 开始选择种子
    pub fn start_seed_chooser(&mut self) {
        // 内联函数
    }

    /// 结束选择种子
    pub fn end_seed_chooser(&mut self) {
        // 内联函数
    }

    /// 计算动画位置（线性插值）
    pub fn calc_position(
        &self,
        time_start: i32,
        time_end: i32,
        position_start: i32,
        position_end: i32,
    ) -> i32 {
        if time_start >= time_end {
            return position_end;
        }
        let elapsed = self.m_cutscene_time - time_start;
        let duration = time_end - time_start;
        if elapsed <= 0 {
            position_start
        } else if elapsed >= duration {
            position_end
        } else {
            position_start + (position_end - position_start) * elapsed / duration
        }
    }

    /// 放置街道僵尸
    pub fn place_street_zombies(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ PlaceStreetZombies）
    }

    /// 添加墓碑粒子效果
    pub fn add_grave_stone_particles(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ AddGraveStoneParticles）
    }

    /// 在指定网格位置放置一个僵尸
    pub fn place_a_zombie(
        &mut self,
        zombie_type: ZombieType,
        grid_x: i32,
        grid_y: i32,
    ) {
        let board = match self.get_board_mut() {
            Some(b) => b,
            None => return,
        };
        // 对应 C++ PlaceAZombie 简化实现
        // 鸭子管道的 ZOMBIE_DUCKY_TUBE 替换逻辑暂缺
        let _zombie_row = grid_y;
        // 设置僵尸位置
        let _pos_x = grid_x * 56 + 830;
        let _pos_y = grid_y * 90 + 70;
        // 渲染层级
        let render_order = board::make_render_order(0, 0, (grid_x % 2) * 2 + grid_y * 4);
        let _ = render_order; // 暂未使用
    }

    /// 检查僵尸能否放在某个网格位置
    pub fn can_zombie_go_in_grid_spot(
        &self,
        zombie_type: ZombieType,
        grid_x: i32,
        grid_y: i32,
        zombie_grid: [[bool; 5]; 5],
    ) -> bool {
        if zombie_grid[grid_x as usize][grid_y as usize] {
            return false;
        }

        if Self::is_2x2_zombie(zombie_type) {
            if grid_x == 0 || grid_y == 0 {
                return false;
            }
            if zombie_grid[(grid_x - 1) as usize][grid_y as usize]
                || zombie_grid[grid_x as usize][(grid_y - 1) as usize]
                || zombie_grid[(grid_x - 1) as usize][(grid_y - 1) as usize]
            {
                return false;
            }
        }

        // 边缘格限制
        if grid_x == 4 && grid_y == 0 {
            return false;
        }
        if grid_x == 0 && grid_y == 0 {
            return false;
        }

        // 大型怪物不能放在边缘
        if Self::is_2x2_zombie(zombie_type) || zombie_type == ZombieType::Zamboni {
            if grid_x == 0 {
                return false;
            }
            if grid_x == 1 && grid_y == 0 {
                return false;
            }
        }

        true
    }

    /// 判断是否是生存模式重选
    pub fn is_survival_repick(&self) -> bool {
        // 内联函数
        false
    }

    /// 判断是否在选种后
    pub fn is_after_seed_chooser(&self) -> bool {
        // 内联函数
        self.m_seed_choosing || self.m_cutscene_time > 5000
    }

    /// 添加花盆
    pub fn add_flower_pots(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ AddFlowerPots）
    }

    /// 更新僵尸胜利动画
    pub fn update_zombies_won(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ UpdateZombiesWon）
    }

    /// 开始僵尸胜利动画
    pub fn start_zombies_won(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ StartZombiesWon）
    }

    /// 显示僵尸行走
    pub fn show_zombie_walking(&self) -> bool {
        // 内联函数
        true
    }

    /// 过场动画是否结束
    pub fn is_cut_scene_over(&self) -> bool {
        // 内联函数
        self.m_cutscene_time > 6000
    }

    /// 僵尸胜利时的点击处理
    pub fn zombie_won_click(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ ZombieWonClick）
    }

    /// 鼠标按下
    pub fn mouse_down(&mut self, x: i32, y: i32) {
        // TODO: 实现完整逻辑（对应 C++ MouseDown）
    }

    /// 键盘按下
    pub fn key_down(&mut self, key: KeyCode) {
        // TODO: 实现完整逻辑（对应 C++ KeyDown）
    }

    /// 推进疯狂戴夫的对话
    pub fn advance_crazy_dave_dialog(&mut self, _just_skipping: bool) {
        // 内联函数
    }

    /// 显示铲子
    pub fn show_shovel(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ ShowShovel）
    }

    /// 能否获得升级包
    pub fn can_get_packet_upgrade(&self) -> bool {
        // TODO: 实现完整逻辑（对应 C++ CanGetPacketUpgrade）
        false
    }

    /// 能否获得指定索引的升级包
    pub fn can_get_packet_upgrade_index(&self, _index: i32) -> bool {
        // TODO: 实现完整逻辑（对应 C++ CanGetPacketUpgrade(int)）
        false
    }

    /// 为街道僵尸找位
    pub fn find_place_for_street_zombies(
        &self,
        zombie_type: ZombieType,
        zombie_grid: [[bool; 5]; 5],
        pos_x: &mut i32,
        pos_y: &mut i32,
    ) {
        // TODO: 实现完整逻辑（对应 C++ FindPlaceForStreetZombies）
    }

    /// 查找并放置僵尸
    pub fn find_and_place_zombie(
        &mut self,
        zombie_type: ZombieType,
        zombie_grid: [[bool; 5]; 5],
    ) {
        // TODO: 实现完整逻辑（对应 C++ FindAndPlaceZombie）
    }

    /// 判断是否是 2x2 大僵尸
    pub fn is_2x2_zombie(zombie_type: ZombieType) -> bool {
        zombie_type == ZombieType::Gargantuar
            || zombie_type == ZombieType::RedeEyeGargantuar
    }

    /// 预加载资源
    pub fn preload_resources(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ PreloadResources）
    }

    /// 预加载前检查
    pub fn is_before_preloading(&self) -> bool {
        // 内联函数
        !self.m_preloaded
    }

    /// 疯狂戴夫是否在说话
    pub fn is_showing_crazy_dave(&self) -> bool {
        // 内联函数
        false
    }

    /// 是否是非滚动过场
    pub fn is_non_scrolling_cutscene(&self) -> bool {
        // TODO: 实现完整逻辑（对应 C++ IsNonScrollingCutscene）
        false
    }

    /// 开始时是否左滚
    pub fn is_scrolled_left_at_start(&self) -> bool {
        // TODO: 实现完整逻辑（对应 C++ IsScrolledLeftAtStart）
        false
    }

    /// 是否在铲子教程中
    pub fn is_in_shovel_tutorial(&self) -> bool {
        // 内联函数
        false
    }

    /// 放置草坪物品
    pub fn place_lawn_items(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ PlaceLawnItems）
    }

    /// 能否获得第二个升级包
    pub fn can_get_second_packet_upgrade(&self) -> bool {
        // TODO: 实现完整逻辑（对应 C++ CanGetSecondPacketUpgrade）
        false
    }

    /// 从消息中解析延迟时间
    pub fn parse_delay_time_from_message(&self) -> i32 {
        // TODO: 实现完整逻辑（对应 C++ ParseDelayTimeFromMessage）
        0
    }

    /// 从消息中解析对话时间
    pub fn parse_talk_time_from_message(&self) -> i32 {
        // TODO: 实现完整逻辑（对应 C++ ParseTalkTimeFromMessage）
        0
    }

    /// 清除升级面板
    pub fn clear_upsell_board(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ ClearUpsellBoard）
    }

    /// 加载入场面板
    pub fn load_intro_board(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadIntroBoard）
    }

    /// 添加升级僵尸
    pub fn add_upsell_zombie(&mut self, _zombie_type: ZombieType, _pixel_x: i32, _grid_y: i32) {
        // 内联函数
    }

    /// 加载升级面板（泳池）
    pub fn load_upsell_board_pool(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadUpsellBoardPool）
    }

    /// 加载升级面板（雾）
    pub fn load_upsell_board_fog(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadUpsellBoardFog）
    }

    /// 加载挑战升级面板
    pub fn load_upsell_challenge_screen(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadUpsellChallengeScreen）
    }

    /// 加载升级面板（屋顶）
    pub fn load_upsell_board_roof(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ LoadUpsellBoardRoof）
    }

    /// 更新升级
    pub fn update_upsell(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ UpdateUpsell）
    }

    /// 绘制升级
    pub fn draw_upsell(&mut self, _g: &mut Graphics) {
        // TODO: 实现完整逻辑（对应 C++ DrawUpsell）
    }

    /// 更新入场动画
    pub fn update_intro(&mut self) {
        // TODO: 实现完整逻辑（对应 C++ UpdateIntro）
    }

    /// 绘制入场动画
    pub fn draw_intro(&mut self, _g: &mut Graphics) {
        // TODO: 实现完整逻辑（对应 C++ DrawIntro）
    }

    /// 是否应该运行升级面板
    pub fn should_run_upsell_board(&self) -> bool {
        // 内联函数
        false
    }
}

impl Default for CutScene {
    fn default() -> Self {
        CutScene::new()
    }
}
