// PvZ Portable Rust 翻译 — GridItem（格子物品：墓碑/卵石/梯子/传送门等）
// 对应 C++ src/Lawn/GridItem.h / GridItem.cpp

use crate::lawn::game_enums::*;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;

/// 运动轨迹帧数（对应 C++ #define NUM_MOTION_TRAIL_FRAMES 12）
pub const NUM_MOTION_TRAIL_FRAMES: usize = 12;

/// 运动轨迹帧（对应 C++ MotionTrailFrame）
/// 用于记录物体运动的历史位置，实现残影效果
#[derive(Debug, Clone, Copy)]
pub struct MotionTrailFrame {
    pub pos_x: f32,
    pub pos_y: f32,
    pub anim_time: f32,
}

impl MotionTrailFrame {
    pub fn new() -> Self {
        MotionTrailFrame {
            pos_x: 0.0,
            pos_y: 0.0,
            anim_time: 0.0,
        }
    }
}

impl Default for MotionTrailFrame {
    fn default() -> Self {
        MotionTrailFrame::new()
    }
}

/// 格子物品类型（本地扩展版本，与 game_enums 中的 GridItemType 不同）
/// 用于 board.rs 中的实际游戏逻辑
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GridItemType {
    None = 0,
    Grave,
    Crater,
    Ladder,
    PortalCrystalBall,
    PortalSquare,
    PlantStinky,
    ScaryPot,
    DanceEggplant,
    Stone,
    WaterPlant,
    TreeOfWisdom,
    ZenTool,
    Cart,
    Rake,
    /// 松鼠（对应 C++ GRIDITEM_SQUIRREL）
    Squirrel,
    /// 大脑（对应 C++ GRIDITEM_BRAIN / GRIDITEM_IZOMBIE_BRAIN）
    Brain,
}

/// 格子物品（地形元素等）
/// 对应 C++ class GridItem（独立类，不继承 GameObject）
pub struct GridItem {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub board: Option<*mut crate::lawn::board::Board>,

    pub grid_item_type: GridItemType,
    pub grid_item_state: GridItemState,
    pub grid_x: i32,
    pub grid_y: i32,
    pub counter: i32,
    pub render_order: i32,
    pub dead: bool,
    pub pos_x: f32,
    pub pos_y: f32,
    pub goal_x: f32,
    pub goal_y: f32,
    pub grid_item_reanim_id: ReanimationID,
    pub grid_item_particle_id: ParticleSystemID,
    pub zombie_type: ZombieType,
    pub seed_type: SeedType,
    pub scary_pot_type: ScaryPotType,
    pub highlighted: bool,
    pub transparent_counter: i32,
    pub sun_count: i32,
    pub motion_trail_frames: [MotionTrailFrame; NUM_MOTION_TRAIL_FRAMES],
    pub motion_trail_count: i32,
}

impl GridItem {
    pub fn new() -> Self {
        GridItem {
            app: None,
            board: None,
            grid_item_type: GridItemType::None,
            grid_item_state: GridItemState::Normal,
            grid_x: 0,
            grid_y: 0,
            counter: 0,
            render_order: 0,
            dead: false,
            pos_x: 0.0,
            pos_y: 0.0,
            goal_x: 0.0,
            goal_y: 0.0,
            grid_item_reanim_id: REANIMATIONID_NULL,
            grid_item_particle_id: PARTICLESYSTEMID_NULL,
            zombie_type: ZombieType::Invalid,
            seed_type: SeedType::Peashooter,
            scary_pot_type: ScaryPotType::None,
            highlighted: false,
            transparent_counter: 0,
            sun_count: 0,
            motion_trail_frames: [MotionTrailFrame::new(); NUM_MOTION_TRAIL_FRAMES],
            motion_trail_count: 0,
        }
    }

    /// 初始化格子物品（对应 C++ GridItemCreate 逻辑）
    pub fn grid_item_initialize(&mut self, item_type: GridItemType, grid_x: i32, grid_y: i32) {
        self.grid_item_type = item_type;
        self.grid_x = grid_x;
        self.grid_y = grid_y;
        self.pos_x = (40 + grid_x * 80) as f32;
        self.pos_y = (80 + grid_y * 100) as f32;
    }

    /// 更新（对应 C++ GridItem::Update）
    pub fn update(&mut self) {
        if self.dead { return; }

        // [TRANSLATION_NOTE]: Reanimation/Particle 更新暂未实现

        match self.grid_item_type {
            GridItemType::PortalCrystalBall | GridItemType::PortalSquare => {
                self.update_portal();
            }
            GridItemType::ScaryPot => {
                self.update_scary_pot();
            }
            GridItemType::Rake => {
                self.update_rake();
            }
            GridItemType::DanceEggplant => {
                self.update_brain();
            }
            _ => {}
        }
    }

    /// 更新恐怖罐子（对应 C++ UpdateScaryPot）
    /// 灯笼植物（Plantern）靠近时使罐子半透明，显示内部内容
    pub fn update_scary_pot(&mut self) {
        // [TRANSLATION_NOTE]: 作弊键 Shift 加速透明暂未实现

        // 附近有灯笼植物时变为半透明
        // [TRANSLATION_NOTE]: 遍历植物检测 Plantern 暂未实现

        if self.transparent_counter > 0 {
            self.transparent_counter -= 1;
        }
    }

    /// 更新传送门（对应 C++ UpdatePortal）
    /// 关闭动画完成后消亡，开启动画完成后进入脉冲循环+粒子效果
    pub fn update_portal(&mut self) {
        if self.grid_item_state == GridItemState::PortalClosed {
            // [TRANSLATION_NOTE]: mLoopCount > 0 检测暂未实现
            // self.grid_item_die();
        }
        // [TRANSLATION_NOTE]: 开启动画完成后切换到脉冲循环+粒子效果暂未实现
    }

    /// 更新大脑（对应 C++ UpdateBrain）- I, Zombie 模式
    pub fn update_brain(&mut self) {
        if self.grid_item_state == GridItemState::BrainSquished {
            self.counter -= 1;
            if self.counter <= 0 {
                self.grid_item_die();
            }
        }
        if self.transparent_counter > 0 {
            self.transparent_counter -= 1;
        }
    }

    /// 更新耙子（对应 C++ UpdateRake）
    /// 吸引→触发→伤害僵尸→消亡
    pub fn update_rake(&mut self) {
        if self.grid_item_state == GridItemState::RakeAttracting || self.grid_item_state == GridItemState::RakeWaiting {
            if self.rake_find_zombie().is_some() {
                self.counter = 200;
                self.grid_item_state = GridItemState::RakeTriggered;
                // [TRANSLATION_NOTE]: PlayFoley(FOLEY_SWING) 暂未实现
            }
        } else if self.grid_item_state == GridItemState::RakeTriggered {
            if let Some(zombie_idx) = self.rake_find_zombie() {
                // [TRANSLATION_NOTE]: ShouldTriggerTimedEvent(0.8f) + TakeDamage(1800, 0) 暂未实现
                let _ = zombie_idx;
            }
            self.counter -= 1;
            if self.counter == 0 {
                self.grid_item_die();
            }
        }
    }

    /// 耙子找僵尸（对应 C++ RakeFindZombie）
    pub fn rake_find_zombie(&self) -> Option<usize> {
        let rake_rect = Rect::new(self.pos_x as i32, self.pos_y as i32, 63, 80);
        if let Some(board) = self.board {
            let board = unsafe { &*board };
            for (idx, zombie) in board.zombies.iter().enumerate() {
                if zombie.dead { continue; }
                if zombie.base.row != self.grid_y { continue; }
                let z_rect = zombie.get_zombie_rect();
                if crate::lawn::board::get_rect_overlap(&rake_rect, &z_rect) >= 0 {
                    return Some(idx);
                }
            }
        }
        None
    }

    /// 更新传送门（对应 C++ UpdatePortal，stub）

    /// 绘制（对应 C++ GridItem::DrawGridItem + DrawGridItemOverlay 入口）
    pub fn draw(&self, g: &mut Graphics) {
        match self.grid_item_type {
            GridItemType::Grave => self.draw_grave_stone(g),
            GridItemType::Crater => self.draw_crater(g),
            GridItemType::Ladder => self.draw_ladder(g),
            GridItemType::PortalCrystalBall => {}
            GridItemType::PortalSquare => {}
            GridItemType::ZenTool => {}
            GridItemType::Rake => {}
            // C++: g->DrawImageF(IMAGE_BRAIN, mPosX, mPosY)
            GridItemType::Brain => self.draw_i_zombie_brain(g),
            GridItemType::ScaryPot => self.draw_scary_pot(g),
            // C++ 注释：松鼠在原版中不可见
            GridItemType::Squirrel => {}
            GridItemType::PlantStinky => self.draw_stinky(g),
            _ => {}
        }

        // C++: 绘制附着动画
        if let Some(app) = self.app {
            let app_ref = unsafe { &*app };
            if let Some(reanim) = app_ref.reanimation_get(self.grid_item_reanim_id) {
                reanim.draw(g);
            }
        }

        // C++: 绘制附着粒子
        // [TRANSLATION_NOTE]: Rust 侧粒子系统为 stub，ParticleTryToGet 暂未接入
    }

    /// 绘制叠加层（对应 C++ GridItem::DrawGridItemOverlay）
    /// 臭鼬 + 巧克力光标时显示对话气泡与巧克力
    pub fn draw_grid_item_overlay(&self, g: &mut Graphics) {
        if self.grid_item_type != GridItemType::PlantStinky {
            return;
        }
        let show_chocolate = self.board.map_or(false, |b| {
            let board = unsafe { &*b };
            board.cursor_object.cursor_type == CursorType::Chocolate
                // [TRANSLATION_NOTE]: IsStinkyHighOnChocolate 依赖 ZenGarden 状态，暂以 false 近似
        });
        if show_chocolate {
            if let Some(img) = self.get_grid_image("plantspeechbubble") {
                g.draw_image_f_xy(img, self.pos_x + 50.0, self.pos_y - 36.0);
            }
            if let Some(choco) = self.get_grid_image("chocolate") {
                // C++: PvzpDrawImageScaledF(IMAGE_CHOCOLATE, ..., 0.44f, 0.44f)
                g.set_scale(0.44, 0.44, 0.0, 0.0);
                g.draw_image_f_xy(choco, self.pos_x + 63.0, self.pos_y - 28.0);
                g.set_scale(1.0, 1.0, 0.0, 0.0);
            }
        }
    }

    /// 绘制 I, Zombie 模式的大脑（对应 C++ GridItem::DrawIZombieBrain）
    pub fn draw_i_zombie_brain(&self, g: &mut Graphics) {
        if self.grid_item_state == GridItemState::BrainSquished {
            // C++: PvzpDrawImageScaledF(IMAGE_BRAIN, ..., 1.0f, 0.25f)
            if let Some(img) = self.get_grid_image("brain") {
                g.set_scale(1.0, 0.25, 0.0, 0.0);
                g.draw_image_f_xy(img, self.pos_x, self.pos_y + 20.0);
                g.set_scale(1.0, 1.0, 0.0, 0.0);
            }
            return;
        }

        // C++: 提示"吃掉所有大脑"期间闪烁
        let should_flash = self.board.map_or(false, |b| {
            let board = unsafe { &*b };
            board.m_advice != AdviceType::None
        });
        if should_flash {
            let main_counter = self.board.map_or(0, |b| unsafe { (*b).m_main_counter });
            let a_flashing_color = crate::todlib::tod_common::get_flashing_color(main_counter, 75);
            g.set_colorize_images(true);
            g.set_color(&a_flashing_color);
        }

        if let Some(img) = self.get_grid_image("brain") {
            g.draw_image_f_xy(img, self.pos_x, self.pos_y);
        }
        if self.transparent_counter > 0 {
            // C++: 加法混合 + 半透明重绘
            g.set_draw_mode(1); // DRAWMODE_ADDITIVE
            g.set_colorize_images(true);
            let alpha = (self.transparent_counter * 3).clamp(0, 255) as u8;
            g.set_color(&crate::framework::color::Color::new(255, 255, 255, alpha));
            if let Some(img) = self.get_grid_image("brain") {
                g.draw_image_f_xy(img, self.pos_x, self.pos_y);
            }
            g.set_draw_mode(0); // DRAWMODE_NORMAL
        }

        g.set_colorize_images(false);
    }

    /// 绘制墓碑（对应 C++ GridItem::DrawGraveStone）
    pub fn draw_grave_stone(&self, g: &mut Graphics) {
        if self.counter <= 0 {
            return;
        }

        // C++: PvzpAnimateCurve 系列（EASE_IN_OUT）
        let a_height_position = crate::todlib::tod_common::tod_animate_curve(
            0, 100, self.counter, 1000, 0, TodCurves::EaseInOut,
        );

        let board = match self.board {
            Some(b) => unsafe { &*b },
            None => return,
        };
        // C++: mGridCelLook[mGridX][mGridY]，Rust 声明为 [Y][X]，故用 [grid_y][grid_x]
        let a_grid_cel_look = board.grid_cel_look[self.grid_y as usize][self.grid_x as usize];
        let a_grid_cel_offset_x = board.grid_cel_offset[self.grid_y as usize][self.grid_x as usize][0];
        let a_grid_cel_offset_y = board.grid_cel_offset[self.grid_y as usize][self.grid_x as usize][1];

        let a_cel_width = 85;  // IMAGE_TOMBSTONES 单元宽
        let a_cel_height = 110; // IMAGE_TOMBSTONES 单元高
        let a_grave_col = a_grid_cel_look % 5;
        let a_grave_row = if self.grid_y == 0 {
            1
        } else if self.grid_item_state == GridItemState::GravestoneSpecial {
            0
        } else {
            2 + a_grid_cel_look % 2
        };

        // C++: PvzpAnimateCurve 计算可见高度与底部裁剪
        let a_visible_height = crate::todlib::tod_common::tod_animate_curve(
            0, 1000, a_height_position, a_cel_height, 0, TodCurves::EaseInOut,
        );
        let a_extra_bottom_clip = crate::todlib::tod_common::tod_animate_curve(
            0, 50, a_height_position, 0, 14, TodCurves::EaseInOut,
        );
        let a_visible_height_dirt = crate::todlib::tod_common::tod_animate_curve(
            500, 1000, a_height_position, a_cel_height, 0, TodCurves::EaseInOut,
        );
        let mut a_extra_top_clip = 0;

        // C++: 吃墓碑的植物正在啃食时顶部额外裁剪
        if let Some(plant) = board.get_top_plant_at(self.grid_x, self.grid_y) {
            if plant.state == PlantState::GravebusterEating {
                a_extra_top_clip = crate::todlib::tod_common::tod_animate_curve_float(
                    400, 0, plant.state_countdown, 10.0, 40.0, TodCurves::Linear,
                ) as i32;
            }
        }

        // C++: Rect aSrcRect / aSrcRectDirt
        let a_src_rect = Rect::new(
            a_cel_width * a_grave_col,
            a_cel_height * a_grave_row + a_extra_top_clip,
            a_cel_width,
            (a_visible_height - a_extra_bottom_clip - a_extra_top_clip).max(0),
        );
        let a_src_rect_dirt = Rect::new(
            a_cel_width * a_grave_col,
            a_cel_height * a_grave_row,
            a_cel_width,
            a_visible_height_dirt.max(0),
        );
        let x = board.grid_to_pixel_x(self.grid_x, self.grid_y) + a_grid_cel_offset_x - 4;
        let y = board.grid_to_pixel_y(self.grid_x, self.grid_y) + a_cel_height + a_grid_cel_offset_y - 9;

        // C++: DrawImage(IMAGE_TOMBSTONES, ...) + DrawImage(IMAGE_TOMBSTONE_MOUNDS, ...)
        if let Some(img) = self.get_grid_image("tombstones") {
            g.draw_image_f_src(img, x as f32, (y - a_visible_height + a_extra_top_clip) as f32, &a_src_rect);
        }
        if let Some(dirt) = self.get_grid_image("tombstone_mounds") {
            g.draw_image_f_src(dirt, x as f32, (y - a_visible_height_dirt) as f32, &a_src_rect_dirt);
        }
    }

    /// 绘制臭鼬（对应 C++ GridItem::DrawStinky，含运动轨迹残影）
    pub fn draw_stinky(&self, g: &mut Graphics) {
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        // C++: 通过可变引用在绘制残影帧时改写 mAnimTime
        let a_stinky_reanim = unsafe {
            match (*app).reanimation_get_mut(self.grid_item_reanim_id) {
                Some(r) => r,
                None => return,
            }
        };
        let a_original_time = a_stinky_reanim.m_anim_time;

        // C++: 反向绘制残影帧（仅奇数帧）
        let mut i = self.motion_trail_count - 1;
        while i >= 0 {
            if i % 2 == 1 {
                let a_frame = &self.motion_trail_frames[i as usize];
                let a_diff_x = a_frame.pos_x - self.pos_x;
                let a_diff_y = a_frame.pos_y - self.pos_y;
                let an_alpha = crate::todlib::tod_common::tod_animate_curve(
                    0, 11, i, 64, 16, TodCurves::Linear,
                ) as u8;
                g.set_color(&crate::framework::color::Color::new(255, 255, 255, an_alpha));
                g.set_colorize_images(true);
                // C++: aStinkyReanim->mAnimTime = aFrame.mAnimTime
                a_stinky_reanim.m_anim_time = a_frame.anim_time;
                g.push_state();
                g.translate_f(a_diff_x, a_diff_y);
                a_stinky_reanim.draw(g);
                g.pop_state();
                g.set_colorize_images(false);
            }
            i -= 1;
        }
        // C++: aStinkyReanim->mAnimTime = aOriginalTime
        a_stinky_reanim.m_anim_time = a_original_time;

        // C++: 高亮时附加加法绘制
        let mut a_draw_highlight = false;
        if self.grid_item_type == GridItemType::PlantStinky && self.highlighted {
            a_draw_highlight = true;
        }
        if a_draw_highlight {
            // [TRANSLATION_NOTE]: mEnableExtraAdditiveDraw / mExtraAdditiveColor 在 Rust reanim 侧为 stub，
            // 以半透明白色近似高亮
            g.set_colorize_images(true);
            g.set_color(&crate::framework::color::Color::new(255, 255, 255, 196));
        }
        a_stinky_reanim.draw(g);
        g.set_colorize_images(false);
    }

    /// 绘制弹坑（对应 C++ GridItem::DrawCrater）
    pub fn draw_crater(&self, g: &mut Graphics) {
        let board = match self.board {
            Some(b) => unsafe { &*b },
            None => return,
        };
        let mut a_x_pos = (board.grid_to_pixel_x(self.grid_x, self.grid_y) - 8) as f32;
        let mut a_y_pos = (board.grid_to_pixel_y(self.grid_x, self.grid_y) + 40) as f32;
        if self.counter < 25 {
            let an_alpha = crate::todlib::tod_common::tod_animate_curve(
                25, 0, self.counter, 255, 0, TodCurves::Linear,
            ) as u8;
            g.set_color(&crate::framework::color::Color::new(255, 255, 255, an_alpha));
            g.set_colorize_images(true);
        }

        let fading = self.counter < 9000;
        let mut a_image_name = "crater";
        let mut a_cel_col = 0;

        if board.is_pool_square(self.grid_x, self.grid_y) {
            if board.stage_is_night() {
                a_image_name = "crater_water_night";
            } else {
                a_image_name = "crater_water_day";
            }
            if fading {
                a_cel_col = 1;
            }
            // C++: 水面摆动
            const CRATER_ANIM_PERIOD: u32 = 200;
            let a_pos = self.grid_y as f32 * std::f32::consts::PI + self.grid_x as f32 * std::f32::consts::PI * 0.25;
            let a_time = (board.m_main_counter % CRATER_ANIM_PERIOD) as f32 * (std::f32::consts::PI * 2.0 / CRATER_ANIM_PERIOD as f32);
            a_y_pos += (a_pos + a_time).sin() * 2.0;
        } else if board.stage_has_roof() {
            if self.grid_x < 5 {
                a_image_name = "crater_roof_left";
                a_x_pos += 16.0;
                a_y_pos += -16.0;
            } else {
                a_image_name = "crater_roof_center";
                a_x_pos += 18.0;
                a_y_pos += -9.0;
            }
            if fading {
                a_cel_col = 1;
            }
        } else if board.stage_is_night() {
            a_cel_col = 1;
            if fading {
                a_image_name = "crater_fading";
            }
        } else if fading {
            a_image_name = "crater_fading";
        }

        // C++: PvzpDrawImageCelF(g, aImage, aXPos, aYPos, aCelCol, 0)
        if let Some(img) = self.get_grid_image(a_image_name) {
            let cel_w = img.get_cel_width();
            let cel_h = img.get_cel_height();
            let src = Rect::new(cel_w * a_cel_col, 0, cel_w, cel_h);
            g.draw_image_f_src(img, a_x_pos, a_y_pos, &src);
        }
        g.set_colorize_images(false);
    }

    /// 绘制恐怖罐子（对应 C++ GridItem::DrawScaryPot）
    pub fn draw_scary_pot(&self, g: &mut Graphics) {
        let a_image_col = (self.grid_item_state as i32) - (GridItemState::ScaryPotQuestion as i32);
        if a_image_col < 0 || a_image_col >= 3 {
            return;
        }
        let board = match self.board {
            Some(b) => unsafe { &*b },
            None => return,
        };
        let a_x_pos = board.grid_to_pixel_x(self.grid_x, self.grid_y) - 5;
        let a_y_pos = board.grid_to_pixel_y(self.grid_x, self.grid_y) - 15;

        // C++: PvzpDrawImageCelCenterScaledF(IMAGE_PLANTSHADOW2, ..., 0, 1.3, 1.3)
        if let Some(shadow) = self.get_grid_image("plantshadow2") {
            g.set_scale(1.3, 1.3, 0.0, 0.0);
            g.draw_image_f_xy(shadow, (a_x_pos - 5) as f32, (a_y_pos + 72) as f32);
            g.set_scale(1.0, 1.0, 0.0, 0.0);
        }

        if self.transparent_counter > 0 {
            // C++: 半透明时绘制罐子内容
            if let Some(img) = self.get_grid_image("scary_pot") {
                g.draw_image_cel_rc(img, a_x_pos, a_y_pos, a_image_col, 0);
            }
            // [TRANSLATION_NOTE]: C++ 的 DrawSeedPacket / DrawCachedZombie / ScaryPotterCountSunInPot
            // 分别依赖种子包/僵尸缓存绘制与挑战模式，Rust 侧暂以占位，后续轮次补全
            let _ = (self.scary_pot_type, self.seed_type, self.zombie_type);
            let _ = board.challenge;

            let a_alpha = crate::todlib::tod_common::tod_animate_curve(
                0, 50, self.transparent_counter, 255, 58, TodCurves::Linear,
            ) as u8;
            g.set_colorize_images(true);
            g.set_color(&crate::framework::color::Color::new(255, 255, 255, a_alpha));
        }

        if let Some(img) = self.get_grid_image("scary_pot") {
            g.draw_image_cel_rc(img, a_x_pos, a_y_pos, a_image_col, 1);
        }
        if self.highlighted {
            g.set_draw_mode(1); // DRAWMODE_ADDITIVE
            g.set_colorize_images(true);
            if self.transparent_counter == 0 {
                g.set_color(&crate::framework::color::Color::new(255, 255, 255, 196));
            }
            if let Some(img) = self.get_grid_image("scary_pot") {
                g.draw_image_cel_rc(img, a_x_pos, a_y_pos, a_image_col, 1);
            }
            g.set_draw_mode(0); // DRAWMODE_NORMAL
        }

        g.set_colorize_images(false);
    }

    /// 绘制梯子（对应 C++ GridItem::DrawLadder）
    pub fn draw_ladder(&self, g: &mut Graphics) {
        let board = match self.board {
            Some(b) => unsafe { &*b },
            None => return,
        };
        let a_x_pos = board.grid_to_pixel_x(self.grid_x, self.grid_y);
        let a_y_pos = board.grid_to_pixel_y(self.grid_x, self.grid_y);
        // C++: PvzpDrawImageScaledF(IMAGE_REANIM_ZOMBIE_LADDER_5, ..., 0.8, 0.8)
        if let Some(img) = self.get_grid_image("reanim_zombie_ladder_5") {
            g.set_scale(0.8, 0.8, 0.0, 0.0);
            g.draw_image_f_xy(img, (a_x_pos + 25) as f32, (a_y_pos - 4) as f32);
            g.set_scale(1.0, 1.0, 0.0, 0.0);
        }
    }

    /// 添加墓碑出现粒子（对应 C++ GridItem::AddGraveStoneParticles）
    pub fn add_grave_stone_particles(&mut self) {
        let board = match self.board {
            Some(b) => unsafe { &*b },
            None => return,
        };
        let a_x_offset = board.grid_cel_offset[self.grid_y as usize][self.grid_x as usize][0];
        let a_y_offset = board.grid_cel_offset[self.grid_y as usize][self.grid_x as usize][1];
        let a_x_pos = board.grid_to_pixel_x(self.grid_x, self.grid_y) + 14 + a_x_offset;
        let a_y_pos = board.grid_to_pixel_y(self.grid_x, self.grid_y) + 78 + a_y_offset;
        // C++: AddPvzpParticle(PARTICLE_GRAVE_STONE_RISE) + PlayFoley(FOLEY_DIRT_RISE)
        if let Some(app) = self.app {
            let app_ref = unsafe { &mut *app };
            app_ref.add_tod_particle(a_x_pos as f32, a_y_pos as f32, self.render_order + 1, ParticleEffect::GraveStoneRise as i32);
            app_ref.play_foley(crate::todlib::tod_foley::FoleyType::DirtRise as i32);
        }
    }

    /// 获取网格物品图片（通过资源管理器按小写 id 获取 Image 指针）
    fn get_grid_image(&self, id: &str) -> Option<&crate::framework::graphics::image::Image> {
        let app = self.app?;
        let app_ref = unsafe { &*app };
        let rm = app_ref.base.resource_manager?;
        let shared = unsafe { (*rm).get_image(id) };
        unsafe {
            if !shared.unshared_image.is_null() {
                return Some(&(*(shared.unshared_image)).base);
            }
            if !shared.shared_image.is_null() {
                return Some(&(*(*(shared.shared_image)).image).base.base);
            }
        }
        None
    }

    /// 格子物品消亡（对应 C++ GridItemDie）
    /// 设置 dead 标记，等待清理
    pub fn grid_item_die(&mut self) {
        self.dead = true;
    }

    /// 打开传送门（对应 C++ GridItem::OpenPortal）
    pub fn open_portal(&mut self) {
        self.grid_item_state = GridItemState::PortalOpen;
    }

    /// 关闭传送门（对应 C++ GridItem::ClosePortal）
    pub fn close_portal(&mut self) {
        self.grid_item_state = GridItemState::PortalClosed;
    }

    /// 是否为打开的传送门（对应 C++ GridItem::IsOpenPortal）
    /// Rust 的 PortalCrystalBall = C++ GRIDITEM_PORTAL_CIRCLE
    pub fn is_open_portal(&self) -> bool {
        self.grid_item_state == GridItemState::PortalOpen
            && (self.grid_item_type == GridItemType::PortalCrystalBall
                || self.grid_item_type == GridItemType::PortalSquare)
    }
}

impl Default for GridItem {
    fn default() -> Self {
        GridItem::new()
    }
}
