// PvZ Portable Rust 翻译 — AlmanacDialog（图鉴对话框）
// 对应 C++ src/Lawn/Widget/AlmanacDialog.h / AlmanacDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{KEYCODE_ESCAPE, KeyCode};
use crate::lawn::game_enums::*;
use crate::lawn::widget::game_button::GameButton;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::todlib::reanimator::Reanimation;

pub const NUM_ALMANAC_SEEDS: i32 = 49;
pub const NUM_ALMANAC_ZOMBIES: i32 = 26;
pub const ALMANAC_PLANT_POSITION_X: f32 = 578.0;
pub const ALMANAC_PLANT_POSITION_Y: f32 = 140.0;
pub const ALMANAC_ZOMBIE_POSITION_X: f32 = 559.0;
pub const ALMANAC_ZOMBIE_POSITION_Y: f32 = 175.0;
pub const ALMANAC_INDEXPLANT_POSITION_X: i32 = 167;
pub const ALMANAC_INDEXPLANT_POSITION_Y: i32 = 255;
pub const ALMANAC_INDEXZOMBIE_POSITION_X: f32 = 535.0;
pub const ALMANAC_INDEXZOMBIE_POSITION_Y: f32 = 215.0;

/// 图鉴对话框（对应 C++ AlmanacDialog）
pub struct AlmanacDialog {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub close_button: Option<*mut GameButton>,
    pub index_button: Option<*mut GameButton>,
    pub plant_button: Option<*mut GameButton>,
    pub zombie_button: Option<*mut GameButton>,
    pub open_page: AlmanacPage,
    pub reanim: [Option<*mut Reanimation>; 4],
    pub selected_seed: SeedType,
    pub selected_zombie: ZombieType,
    pub plant: Option<*mut crate::lawn::plant::Plant>,
    pub zombie: Option<*mut crate::lawn::zombie::Zombie>,
}

impl AlmanacDialog {
    pub fn new() -> Self {
        AlmanacDialog {
            app: None,
            close_button: None,
            index_button: None,
            plant_button: None,
            zombie_button: None,
            open_page: AlmanacPage::Index,
            reanim: [None, None, None, None],
            selected_seed: SeedType::None,
            selected_zombie: ZombieType::Invalid,
            plant: None,
            zombie: None,
        }
    }

    pub fn clear_plants_and_zombies(&mut self) {
        self.plant = None;
        self.zombie = None;
    }
    pub fn removed_from_manager(&mut self, _mgr: &mut WidgetManager) {
        self.clear_plants_and_zombies();
    }
    pub fn setup_plant(&mut self) {
        self.clear_plants_and_zombies();
        self.plant = None;
    }
    pub fn setup_zombie(&mut self) {
        self.clear_plants_and_zombies();
        self.zombie = None;
    }
    pub fn set_page(&mut self, page: AlmanacPage) {
        self.open_page = page;
        self.clear_plants_and_zombies();
    }

    /// 对应 C++ AlmanacDialog::KeyDown（AlmanacDialog.cpp）
    pub fn key_down(&mut self, key: KeyCode) {
        if key == KEYCODE_ESCAPE {
            if self.open_page == AlmanacPage::Index {
                if let Some(app) = self.app {
                    unsafe {
                        (*app).kill_almanac_dialog();
                    }
                }
            } else {
                self.set_page(AlmanacPage::Index);
            }
            return;
        }

        // C++: LawnDialog::KeyDown(theKey) —— 基类按键处理，Rust 侧无对应实现
    }
    pub fn update(&mut self) {
        if let Some(app) = self.app { unsafe {
            let mx = 0; let my = 0;
            if self.seed_hit_test(mx, my) != SeedType::None || self.zombie_hit_test(mx, my) != ZombieType::Invalid {}
        } }
    }
    /// 图鉴页索引绘制（对应 C++ DrawIndex，:267）
    fn draw_index(&self, g: &mut Graphics) {
        // 对应 C++: g->DrawImage(IMAGE_ALMANAC_INDEXBACK, 0, 0)
        draw_almanac_image(g, "IMAGE_ALMANAC_INDEXBACK", 0, 0);
        // [TRANSLATION_NOTE]: 标题 [SUBURBAN_ALMANAC_INDEX] 的 HOUSEOFTERROR28 字体以 Font 简化
        draw_almanac_text(g, "[SUBURBAN_ALMANAC_INDEX]", crate::lawn::game_enums::BOARD_WIDTH / 2, 60, 28,
            &crate::framework::color::Color { r: 220, g: 220, b: 220, a: 255 });

        // 对应 C++: 选中的植物/僵尸预览
        // [TRANSLATION_NOTE]: C++ 的 mPlant->BeginDraw（变换栈）+ Draw；Rust 用直接 Draw 近似
        if let Some(plant_ptr) = self.plant {
            unsafe { (*plant_ptr).draw(g); }
        }
        if let Some(zombie_ptr) = self.zombie {
            unsafe { (*zombie_ptr).draw(g); }
        }
    }

    /// 图鉴植物页绘制（对应 C++ DrawPlants，:286）
    fn draw_plants(&self, g: &mut Graphics) {
        draw_almanac_image(g, "IMAGE_ALMANAC_PLANTBACK", 0, 0);
        draw_almanac_text(g, "[SUBURBAN_ALMANAC_PLANTS]", crate::lawn::game_enums::BOARD_WIDTH / 2, 48, 20,
            &crate::framework::color::Color { r: 213, g: 159, b: 43, a: 255 });

        // 对应 C++: 遍历 0..NUM_ALMANAC_SEEDS 绘制已获得种子
        for a_seed_index in 0..crate::lawn::widget::almanac_dialog::NUM_ALMANAC_SEEDS {
            let a_seed_type = unsafe { std::mem::transmute::<i32, SeedType>(a_seed_index) };
            let a_has_seed = self.app.map_or(false, |app| unsafe { (*app).has_seed_type(a_seed_type) });
            if a_has_seed {
                let mut a_pos_x = 0;
                let mut a_pos_y = 0;
                self.get_seed_position(a_seed_type, &mut a_pos_x, &mut a_pos_y);
                if a_seed_type == SeedType::Imitater {
                    // 对应 C++: IMAGE_ALMANAC_IMITATER（命中高亮与本体两张）
                    draw_almanac_image(g, "IMAGE_ALMANAC_IMITATER", a_pos_x, a_pos_y);
                    draw_almanac_image(g, "IMAGE_ALMANAC_IMITATER", a_pos_x, a_pos_y);
                } else {
                    crate::lawn::seed_packet::draw_seed_packet(
                        g, a_pos_x as f32, a_pos_y as f32, a_seed_type, SeedType::None, 0.0, 255, true, false,
                    );
                    // [TRANSLATION_NOTE]: 鼠标命中亮框 IMAGE_SEEDPACKETFLASH 依赖命中检测，暂略
                }
            }
        }

        // 对应 C++: 地面背景（泳池/夜/屋顶）
        let a_night_ground = crate::lawn::plant::Plant::is_nocturnal(self.selected_seed)
            || self.selected_seed == SeedType::Gravebuster
            || self.selected_seed == SeedType::Plantern;
        if self.selected_seed == SeedType::Lilypad || self.selected_seed == SeedType::Tanglekelp
            || self.selected_seed == SeedType::Cattail || self.selected_seed == SeedType::Seashroom
        {
            // 对应 C++: 泳池地面（IMAGE_ALMANAC_GROUNDPOOL/NIGHTPOOL）+ 池效果（3D 加速分支标注）
            draw_almanac_image(
                g,
                if a_night_ground { "IMAGE_ALMANAC_GROUNDNIGHTPOOL" } else { "IMAGE_ALMANAC_GROUNDPOOL" },
                521, 107,
            );
        } else {
            let a_ground_key = if a_night_ground {
                "IMAGE_ALMANAC_GROUNDNIGHT"
            } else if self.selected_seed == SeedType::Flowerpot {
                "IMAGE_ALMANAC_GROUNDROOF"
            } else {
                "IMAGE_ALMANAC_GROUNDDAY"
            };
            draw_almanac_image(g, a_ground_key, 521, 107);
        }

        // 对应 C++: 选中植物预览
        if let Some(plant_ptr) = self.plant {
            unsafe { (*plant_ptr).draw(g); }
        }

        // 对应 C++: 植物卡片 + 名称/描述/成本/冷却
        draw_almanac_image(g, "IMAGE_ALMANAC_PLANTCARD", 459, 86);
        let a_name = crate::lawn::plant::Plant::get_name_string(self.selected_seed, SeedType::None);
        draw_almanac_text(g, &a_name, 617, 288, 18,
            &crate::framework::color::Color { r: 255, g: 255, b: 255, a: 255 });
        // [TRANSLATION_NOTE]: 描述/成本/冷却文案（PlantDefinition 表 + [XX_DESCRIPTION] 字符串系统）
        // 依赖 PlantDef 资源表，暂以名称呈现
    }

    /// 图鉴僵尸页绘制（对应 C++ DrawZombies，:366）
    fn draw_zombies(&self, g: &mut Graphics) {
        draw_almanac_image(g, "IMAGE_ALMANAC_ZOMBIEBACK", 0, 0);
        draw_almanac_text(g, "[SUBURBAN_ALMANAC_ZOMBIES]", crate::lawn::game_enums::BOARD_WIDTH / 2, 54, 24,
            &crate::framework::color::Color { r: 0, g: 196, b: 0, a: 255 });

        // 对应 C++: 遍历 26 个僵尸条目绘制窗口/剪影
        // [TRANSLATION_NOTE]: 完整实现依赖 GetZombieDefinition 表（起始关卡/名称）、
        // ZombieIsShown/Silhouette 判定与 ReanimatorCache::DrawCachedZombie；此处绘制骨架。
        for i in 0..crate::lawn::widget::almanac_dialog::NUM_ALMANAC_ZOMBIES {
            let _a_zombie_type = unsafe { std::mem::transmute::<i32, ZombieType>(i) };
            // 简化网格（6 列×5 行）；C++ 走 GetZombiePosition 定位
            let (a_pos_x, a_pos_y) = (23 + i % 6 * 85, 78 + i / 6 * 90);
            draw_almanac_image(g, "IMAGE_ALMANAC_ZOMBIEWINDOW", a_pos_x, a_pos_y);
            draw_almanac_image(g, "IMAGE_ALMANAC_ZOMBIEWINDOW2", a_pos_x, a_pos_y);
        }

        draw_almanac_image(g, "IMAGE_ALMANAC_GROUNDDAY", 518, 110);
        if let Some(zombie_ptr) = self.zombie {
            unsafe { (*zombie_ptr).draw(g); }
        }
        // [TRANSLATION_NOTE]: 僵尸卡片/名称/描述依赖 ZombieDefinition 表，暂略
        draw_almanac_image(g, "IMAGE_ALMANAC_ZOMBIECARD", 455, 78);
    }

    /// 绘制（对应 C++ AlmanacDialog::Draw，:507）
    pub fn draw(&self, g: &mut Graphics) {
        match self.open_page {
            crate::lawn::game_enums::AlmanacPage::Index => self.draw_index(g),
            crate::lawn::game_enums::AlmanacPage::Plants => self.draw_plants(g),
            crate::lawn::game_enums::AlmanacPage::Zombies => self.draw_zombies(g),
        }

        // 对应 C++: mCloseButton/mIndexButton/mPlantButton/mZombieButton->Draw(g)
        if let Some(btn) = self.close_button { unsafe { (&mut *btn).draw(g); } }
        if let Some(btn) = self.index_button { unsafe { (&mut *btn).draw(g); } }
        if let Some(btn) = self.plant_button { unsafe { (&mut *btn).draw(g); } }
        if let Some(btn) = self.zombie_button { unsafe { (&mut *btn).draw(g); } }
    }
    pub fn get_seed_position(&self, t: SeedType, x: &mut i32, y: &mut i32) {
        if t == SeedType::Imitater { *x = 20; *y = 23; }
        else { *x = (t as i32) % 8 * 52 + 26; *y = (t as i32) / 8 * 78 + 92; }
    }
    pub fn seed_hit_test(&self, x: i32, y: i32) -> SeedType {
        if self.open_page == AlmanacPage::Plants {
            for st in 0..NUM_ALMANAC_SEEDS {
                let seed_type = unsafe { std::mem::transmute::<i32, SeedType>(st) };
                if let Some(app) = self.app { unsafe {
                    if (*app).has_seed_type(seed_type) {
                        let (sx, sy) = (0, 0);
                        let rect = crate::framework::rect::Rect::new(sx, sy, 50, 70);
                        if rect.contains(x, y) { return seed_type; }
                    }
                } }
            }
        }
        SeedType::None
    }
    pub fn zombie_has_silhouette(&self, t: ZombieType) -> bool {
        if t == ZombieType::Yeti { if let Some(app) = self.app { unsafe { return !(*app).can_spawn_yetis(); } } }
        false
    }
    pub fn zombie_is_shown(&self, t: ZombieType) -> bool {
        if let Some(app) = self.app { unsafe {
            if (*app).is_trial_stage_locked() && (t as i32) > (ZombieType::Snorkel as i32) { return false; }
            if t == ZombieType::Yeti { return (*app).can_spawn_yetis() || self.zombie_has_silhouette(t); }
            if (t as i32) <= (ZombieType::Boss as i32) { return true; }
        } }
        false
    }
    pub fn zombie_has_description(&self, t: ZombieType) -> bool {
        if (t as i32) <= (ZombieType::Boss as i32) { return true; }
        if let Some(app) = self.app { unsafe { (*app).has_finished_adventure() } } else { false }
    }
    pub fn get_zombie_position(&self, t: ZombieType, x: &mut i32, y: &mut i32) {
        *x = (t as i32) % 5 * 53 + 10;
        *y = (t as i32) / 5 * 80 + 140;
    }
    pub fn zombie_hit_test(&self, x: i32, y: i32) -> ZombieType {
        if self.open_page == AlmanacPage::Zombies {
            for zt in 0..NUM_ALMANAC_ZOMBIES {
                let ztype = unsafe { std::mem::transmute::<i32, ZombieType>(zt) };
                let (zx, zy) = (0, 0);
                let rect = crate::framework::rect::Rect::new(zx, zy, 50, 70);
                if rect.contains(x, y) { return ztype; }
            }
        }
        ZombieType::Invalid
    }
    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // 对应 C++ MouseUp：按钮悬停时切换页面/关闭
        if self.plant_button.map_or(false, |p| unsafe { (*p).is_over }) {
            self.set_page(AlmanacPage::Plants);
        } else if self.zombie_button.map_or(false, |p| unsafe { (*p).is_over }) {
            self.set_page(AlmanacPage::Zombies);
        } else if self.close_button.map_or(false, |p| unsafe { (*p).is_over }) {
            if let Some(app) = self.app {
                unsafe { (*app).kill_almanac_dialog(); }
            }
        } else if self.index_button.map_or(false, |p| unsafe { (*p).is_over }) {
            self.set_page(AlmanacPage::Index);
        }
    }
    pub fn mouse_down(&mut self, x: i32, y: i32, _click_count: i32) {
        let seed = self.seed_hit_test(x, y);
        if seed != SeedType::None { self.show_plant(seed); return; }
        let zombie = self.zombie_hit_test(x, y);
        if zombie != ZombieType::Invalid { self.show_zombie(zombie); return; }
    }
    pub fn get_zombie_type(index: i32) -> ZombieType { ZombieType::Invalid }
    pub fn show_plant(&mut self, t: SeedType) {
        self.selected_seed = t;
        self.set_page(AlmanacPage::Plants);
    }
    pub fn show_zombie(&mut self, t: ZombieType) {
        self.selected_zombie = t;
        self.set_page(AlmanacPage::Zombies);
    }
}

impl Default for AlmanacDialog {
    fn default() -> Self { AlmanacDialog::new() }
}

/// 全局僵尸击败标记数组（对应 C++ gZombieDefeated）
pub static mut G_ZOMBIE_DEFEATED: [bool; NUM_ZOMBIE_TYPES as usize] = [false; NUM_ZOMBIE_TYPES as usize];

/// 初始化玩家图鉴数据（对应 C++ AlmanacInitForPlayer）
/// 重置所有僵尸的已击败标记
/// 经全局 ResourceManager 按 key 取图并绘制（对应 C++ Sexy::IMAGE_* 引用）
fn draw_almanac_image(g: &mut Graphics, a_key: &str, x: i32, y: i32) {
    let a_image = crate::lawn::lawn_app::LawnApp::instance().map_or(std::ptr::null_mut(), |app| {
        let a_rm = match app.base.resource_manager {
            Some(r) => r,
            None => return std::ptr::null_mut(),
        };
        unsafe { (*a_rm).get_image(a_key).as_image_ptr() }
    });
    if !a_image.is_null() {
        unsafe { g.draw_image_xy(&*a_image, x, y); }
    }
}

/// 居中绘制文本（TRANSLATION_NOTE: C++ 用 _Font* 字号与内嵌色；Rust 以 Font 简化）
fn draw_almanac_text(
    g: &mut Graphics,
    a_text: &str,
    a_center_x: i32,
    a_y: i32,
    a_size: i32,
    a_color: &crate::framework::color::Color,
) {
    let mut a_font = crate::framework::graphics::font::Font::new("Dwarventodcraft", a_size);
    a_font.ascent = (a_size as f32 * 0.72) as i32;
    a_font.font_height = a_size;
    g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
    g.set_color(a_color);
    let a_width = a_font.string_width(a_text);
    g.draw_string(a_text, a_center_x - a_width / 2, a_y);
}

pub fn almanac_init_for_player() {
    unsafe {
        for i in 0..NUM_ZOMBIE_TYPES as usize {
            G_ZOMBIE_DEFEATED[i] = false;
        }
    }
}

/// 记录玩家击败的僵尸类型（对应 C++ AlmanacPlayerDefeatedZombie）
/// 在 Zombie::DropLoot 中调用，用于图鉴的已击败标记
pub fn almanac_player_defeated_zombie(zombie_type: ZombieType) {
    unsafe {
        let idx = zombie_type as usize;
        if idx < G_ZOMBIE_DEFEATED.len() {
            G_ZOMBIE_DEFEATED[idx] = true;
        }
    }
}
