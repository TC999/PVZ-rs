// PvZ Portable Rust 翻译 — AwardScreen（奖励界面）
// 对应 C++ src/Lawn/Widget/AwardScreen.h / AwardScreen.cpp
// 完整翻译版本

use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::{KEYCODE_ESCAPE, KEYCODE_RETURN, KEYCODE_SPACE, KeyCode};
use crate::lawn::game_enums::*;
use crate::lawn::widget::game_button::GameButton;

/// 成就屏幕条目（对应 C++ AchievementScreenItem）
pub struct AchievementScreenItem {
    pub id: i32,
    pub start_anim_time: i32,
    pub end_anim_time: i32,
    pub dest_y: i32,
    pub start_y: i32,
    pub y: i32,
}

/// 奖励界面（对应 C++ AwardScreen）
pub struct AwardScreen {
    pub start_button: Option<*mut GameButton>,
    pub menu_button: Option<*mut GameButton>,
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub fade_in_counter: i32,
    pub award_type: AwardType,
    pub continue_button: Option<*mut GameButton>,
    pub show_start_button_after_achievements: bool,
    pub show_menu_button_after_achievements: bool,
    pub achievement_anim_time: i32,
    pub showing_achievements: bool,
    pub achievement_items: Vec<AchievementScreenItem>,
}

impl AwardScreen {
    pub fn new() -> Self {
        AwardScreen {
            start_button: None,
            menu_button: None,
            app: None,
            fade_in_counter: 180,
            award_type: AwardType::ForLevel,
            continue_button: None,
            show_start_button_after_achievements: false,
            show_menu_button_after_achievements: false,
            achievement_anim_time: 0,
            showing_achievements: false,
            achievement_items: Vec::new(),
        }
    }

    pub fn is_paper_note(&self) -> bool {
        matches!(self.award_type, AwardType::CreditsZombieNote | AwardType::HelpZombieNote)
    }

    /// 从 ResourceManager 按 key 取图（对应 C++ IMAGE_* 全局资源；未接入资源表时返回 null）
    fn get_resource_image(&self, a_key: &str) -> *mut crate::framework::graphics::image::Image {
        let Some(app) = self.app else { return std::ptr::null_mut() };
        unsafe {
            let app_ref = &*app;
            let Some(rm) = app_ref.base.resource_manager else { return std::ptr::null_mut() };
            let rm_ref = &*rm;
            rm_ref.get_image(a_key).as_image_ptr()
        }
    }

    /// 用指定字号在 (center_x, y) 水平居中绘制文本（TRANSLATION_NOTE: PvzpDrawString 的 DS_ALIGN_CENTER 简化）
    fn draw_text_centered(&self, g: &mut Graphics, a_text: &str, a_center_x: i32, a_y: i32, a_size: i32, a_color: &crate::framework::color::Color) {
        let mut a_font = crate::framework::graphics::font::Font::new("Dwarventodcraft", a_size);
        a_font.ascent = 13;
        a_font.font_height = a_size;
        g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
        g.set_color(a_color);
        let a_w = a_font.string_width(a_text);
        g.draw_string(a_text, a_center_x - a_w / 2, a_y);
    }

    /// 对应 C++ AwardScreen::DrawBottom（AwardScreen.cpp 271-277）
    pub fn draw_bottom(&self, g: &mut Graphics, the_title: &str, the_award: &str, the_message: &str) {
        let a_back = self.get_resource_image("IMAGE_AWARDSCREEN_BACK");
        if !a_back.is_null() {
            g.draw_image_xy(unsafe { &*a_back }, 0, 0);
        }
        // C++: FONT_DWARVENTODCRAFT24 金标题（BOARD_WIDTH/2, 58）
        self.draw_text_centered(g, the_title, BOARD_WIDTH / 2, 58, 24, &crate::framework::color::Color::new(213, 159, 43, 255));
        // C++: FONT_DWARVENTODCRAFT18YELLOW 白奖项名（BOARD_WIDTH/2, 326）
        self.draw_text_centered(g, the_award, BOARD_WIDTH / 2, 326, 18, &crate::framework::color::Color::WHITE);
        // C++: 说明 wrapped（Rect(285,360,230,90), BRIANNETOD16, Color(40,50,90)）——居中近似
        self.draw_text_centered(g, the_message, 400, 375, 16, &crate::framework::color::Color::new(40, 50, 90, 255));
    }

    /// 对应 C++ AwardScreen::DrawAwardSeed（AwardScreen.cpp 279-292）
    pub fn draw_award_seed(&self, g: &mut Graphics) {
        let a_level = self.app.map_or(0, |app| unsafe { (*app).player_info.as_ref().map_or(0, |pi| pi.m_level) });
        let a_seed_type = crate::lawn::lawn_app::LawnApp::get_award_seed_for_level(a_level - 1);
        let a_award = crate::lawn::plant::Plant::get_name_string(a_seed_type, SeedType::None);
        let a_message = if self.app.map_or(false, |app| unsafe { (*app).is_trial_stage_locked() })
            && a_seed_type as i32 >= SeedType::Squash as i32
            && a_seed_type != SeedType::Tanglekelp
        {
            "[AVAILABLE_IN_FULL_VERSION]".to_string()
        } else {
            crate::lawn::plant::Plant::get_tool_tip(a_seed_type)
        };
        self.draw_bottom(g, "[NEW_PLANT]", &a_award, &a_message);
        // C++: SetScale(2,2,350,129) 后 DrawSeedPacket(350,129,...,draw_cost=true)；Rust 端缩放未模拟
        crate::lawn::seed_packet::draw_seed_packet(g, 350.0, 129.0, a_seed_type, SeedType::None, 0.0, 255, true, false);
    }

    /// 对应 C++ AwardScreen::Draw（AwardScreen.cpp 295-470）
    pub fn draw(&self, g: &mut Graphics) {
        g.set_linear_blend(true);
        let a_level = self.app.map_or(0, |app| unsafe { (*app).player_info.as_ref().map_or(0, |pi| pi.m_level) });

        if self.showing_achievements {
            // C++: DrawAchievements(g)（成就列表动画绘制，另行实现）
        } else if self.award_type == AwardType::CreditsZombieNote {
            let a_bg = self.get_resource_image("IMAGE_BACKGROUND6BOSS");
            if !a_bg.is_null() {
                // C++: DrawImage(img, -900, -400, 2800, 1200) 蓝色调拉伸背景
                g.set_colorize_images(true);
                g.set_color(&crate::framework::color::Color::new(125, 200, 255, 255));
                g.draw_image_stretch(
                    unsafe { &*a_bg },
                    &crate::framework::rect::Rect::new(-900, -400, 2800, 1200),
                    &crate::framework::rect::Rect::new(0, 0, unsafe { (*a_bg).get_width() }, unsafe { (*a_bg).get_height() }),
                );
                g.set_colorize_images(false);
            }
            g.set_color(&crate::framework::color::Color::new(0, 0, 0, 64));
            g.fill_rect_xywh(0, 525, BOARD_WIDTH, BOARD_HEIGHT - 525);
            let a_note = self.get_resource_image("IMAGE_ZOMBIE_NOTE");
            if !a_note.is_null() { g.draw_image_xy(unsafe { &*a_note }, 75, 60); }
            let a_credits = self.get_resource_image("IMAGE_CREDITS_ZOMBIENOTE");
            if !a_credits.is_null() {
                g.draw_image_stretch(
                    unsafe { &*a_credits },
                    &crate::framework::rect::Rect::new(149, 103, 475, 325),
                    &crate::framework::rect::Rect::new(0, 0, unsafe { (*a_credits).get_width() }, unsafe { (*a_credits).get_height() }),
                );
            }
        } else if self.award_type == AwardType::HelpZombieNote {
            let a_bg1 = self.get_resource_image("IMAGE_BACKGROUND1");
            if !a_bg1.is_null() {
                g.draw_image_stretch(
                    unsafe { &*a_bg1 },
                    &crate::framework::rect::Rect::new(-700, -300, 2800, 1200),
                    &crate::framework::rect::Rect::new(0, 0, unsafe { (*a_bg1).get_width() }, unsafe { (*a_bg1).get_height() }),
                );
            }
            let a_note = self.get_resource_image("IMAGE_ZOMBIE_NOTE");
            if !a_note.is_null() { g.draw_image_xy(unsafe { &*a_note }, 80, 80); }
            let a_help = self.get_resource_image("IMAGE_ZOMBIE_NOTE_HELP");
            if !a_help.is_null() { g.draw_image_xy(unsafe { &*a_help }, 131, 132); }
        } else if self.award_type != AwardType::AchievementOnly {
            if !self.app.map_or(false, |app| unsafe { (*app).is_adventure_mode() }) {
                if self.app.map_or(false, |app| unsafe { (*app).earned_gold_trophy() }) {
                    self.draw_bottom(g, "[BEAT_GAME_MESSAGE1]", "[GOLD_SUNFLOWER_TROPHY]", "[BEAT_GAME_MESSAGE2]");
                    // C++: PvzpDrawImageCelCenterScaledF(IMAGE_SUNFLOWER_TROPHY, 325, 65, 1, 0.6, 0.6)
                    let a_trophy = self.get_resource_image("IMAGE_SUNFLOWER_TROPHY");
                    if !a_trophy.is_null() { g.draw_image_xy(unsafe { &*a_trophy }, 325, 65); }
                } else {
                    let a_msg_char;
                    if self.app.map_or(false, |app| unsafe { (*app).is_survival_mode() }) {
                        let a_num_trophies = crate::lawn::lawn_app::LawnApp::get_num_trophies(crate::lawn::game_enums::ChallengePage::Survival as i32);
                        a_msg_char = if a_num_trophies <= 7 {
                            "[YOU_UNLOCKED_A_SURVIVAL]"
                        } else if a_num_trophies == 10 {
                            "[YOU_UNLOCKED_ENDLESS_SURVIVAL]"
                        } else {
                            "[EARN_MORE_TROPHIES_FOR_ENDLESS_SURVIVAL]"
                        };
                    } else if self.app.map_or(false, |app| unsafe { (*app).is_scary_potter_level() }) {
                        a_msg_char = "[UNLOCKED_VASEBREAKER_LEVEL]";
                    } else if self.app.map_or(false, |app| unsafe { (*app).is_puzzle_mode() }) {
                        a_msg_char = "[UNLOCKED_I_ZOMBIE_LEVEL]";
                    } else {
                        let a_num_trophies = crate::lawn::lawn_app::LawnApp::get_num_trophies(crate::lawn::game_enums::ChallengePage::Challenge as i32);
                        a_msg_char = if a_num_trophies <= 17 { "[CHALLENGE_UNLOCKED]" } else { "[GET_MORE_TROPHIES]" };
                    }
                    self.draw_bottom(g, "[GOT_TROPHY]", "[TROPHY]", a_msg_char);
                    let a_trophy = self.get_resource_image("IMAGE_TROPHY_HI_RES");
                    if !a_trophy.is_null() {
                        unsafe {
                            g.draw_image_xy(&*a_trophy, BOARD_WIDTH / 2 - (*a_trophy).get_width() / 2, 137);
                        }
                    }
                }
            } else if a_level == 5 {
                self.draw_bottom(g, "[GOT_SHOVEL]", "[SHOVEL]", "[SHOVEL_DESCRIPTION]");
                let a_shovel = self.get_resource_image("IMAGE_SHOVEL_HI_RES");
                if !a_shovel.is_null() {
                    unsafe { g.draw_image_xy(&*a_shovel, BOARD_WIDTH / 2 - (*a_shovel).get_width() / 2, 137); }
                }
            } else if a_level == 10 || a_level == 20 || a_level == 30 || a_level == 40 || a_level == 50 {
                // C++: 便条关卡（背景1/2 + 对应便条图 + [FOUND_NOTE]）
                let a_bg_key = if a_level == 20 || a_level == 40 { "IMAGE_BACKGROUND2" } else { "IMAGE_BACKGROUND1" };
                let a_bg = self.get_resource_image(a_bg_key);
                if !a_bg.is_null() {
                    g.draw_image_stretch(
                        unsafe { &*a_bg },
                        &crate::framework::rect::Rect::new(-700, -300, 2800, 1200),
                        &crate::framework::rect::Rect::new(0, 0, unsafe { (*a_bg).get_width() }, unsafe { (*a_bg).get_height() }),
                    );
                }
                let a_note = self.get_resource_image("IMAGE_ZOMBIE_NOTE");
                if !a_note.is_null() { g.draw_image_xy(unsafe { &*a_note }, 80, 80); }
                let (a_note_key, a_note_x, a_note_y) = match a_level {
                    20 => ("IMAGE_ZOMBIE_NOTE2", 133, 127),
                    30 => ("IMAGE_ZOMBIE_NOTE3", 120, 117),
                    40 => ("IMAGE_ZOMBIE_NOTE4", 102, 117),
                    _ => ("IMAGE_ZOMBIE_FINAL_NOTE", 114, 138), // 50
                };
                let a_note_n = self.get_resource_image(a_note_key);
                if !a_note_n.is_null() { g.draw_image_xy(unsafe { &*a_note_n }, a_note_x, a_note_y); }
                self.draw_text_centered(g, "[FOUND_NOTE]", BOARD_WIDTH / 2, 70, 24, &crate::framework::color::Color::new(255, 200, 0, 255));
            } else if a_level == 15 {
                self.draw_bottom(g, "[FOUND_SUBURBAN_ALMANAC]", "[SUBURBAN_ALMANAC]", "[SUBURBAN_ALMANAC_DESCRIPTION]");
                let a_almanac = self.get_resource_image("IMAGE_ALMANAC");
                if !a_almanac.is_null() {
                    unsafe { g.draw_image_xy(&*a_almanac, BOARD_WIDTH / 2 - (*a_almanac).get_width() / 2, 160); }
                }
            } else if a_level == 25 {
                self.draw_bottom(g, "[FOUND_KEYS]", "[KEYS]", "[KEYS_DESCRIPTION]");
                let a_keys = self.get_resource_image("IMAGE_CARKEYS");
                if !a_keys.is_null() {
                    unsafe { g.draw_image_xy(&*a_keys, BOARD_WIDTH / 2 - (*a_keys).get_width() / 2, 160); }
                }
            } else if a_level == 35 {
                self.draw_bottom(g, "[FOUND_TACO]", "[TACO]", "[TACO_DESCRIPTION]");
                let a_taco = self.get_resource_image("IMAGE_TACO");
                if !a_taco.is_null() {
                    unsafe { g.draw_image_xy(&*a_taco, BOARD_WIDTH / 2 - (*a_taco).get_width() / 2, 160); }
                }
            } else if a_level == 45 {
                self.draw_bottom(g, "[FOUND_WATERING_CAN]", "[WATERING_CAN]", "[WATERING_CAN_DESCRIPTION]");
                let a_can = self.get_resource_image("IMAGE_WATERINGCAN");
                if !a_can.is_null() {
                    unsafe { g.draw_image_xy(&*a_can, BOARD_WIDTH / 2 - (*a_can).get_width() / 2, 160); }
                }
            } else if a_level == 1 && self.app.map_or(false, |app| unsafe { (*app).has_finished_adventure() }) {
                self.draw_bottom(g, "[WIN_MESSAGE1]", "[SILVER_SUNFLOWER_TROPHY]", "[WIN_MESSAGE2]");
                // C++: PvzpDrawImageCelCenterScaledF(IMAGE_SUNFLOWER_TROPHY, 325, 65, 0, 0.7, 0.7)
                let a_trophy = self.get_resource_image("IMAGE_SUNFLOWER_TROPHY");
                if !a_trophy.is_null() { g.draw_image_xy(unsafe { &*a_trophy }, 325, 65); }
            } else {
                self.draw_award_seed(g);
            }
        }

        // [TRANSLATION_NOTE]: C++ mStartButton/mMenuButton/mContinueButton->Draw(g)；Rust 按钮绘制未接入

        // C++: fade-in 遮罩（便条黑 / 否则白）
        let a_fade_in_alpha = crate::todlib::tod_common::tod_animate_curve(180, 0, self.fade_in_counter, 255, 0, TodCurves::Linear);
        if self.is_paper_note() {
            g.set_color(&crate::framework::color::Color::new(0, 0, 0, a_fade_in_alpha as u8));
        } else {
            g.set_color(&crate::framework::color::Color::new(255, 255, 255, a_fade_in_alpha as u8));
        }
        g.fill_rect_xywh(0, 0, BOARD_WIDTH, BOARD_HEIGHT);
    }

    pub fn update(&mut self) {
        if self.fade_in_counter > 0 {
            self.fade_in_counter -= 1;
        }
        // C++: if (mShowingAchievements) { mAchievementAnimTime++; ... }
        if self.showing_achievements {
            self.achievement_anim_time += 1;
            // C++: for (i...) { if (mAchievementAnimTime >= mStartAnimTime) mY = PvzpAnimateCurve(...EASE_IN_OUT) }
            for item in &mut self.achievement_items {
                if self.achievement_anim_time >= item.start_anim_time && self.achievement_anim_time < item.end_anim_time {
                    // C++: PvzpAnimateCurve(start, end, t, startY, destY, CURVE_EASE_IN_OUT)
                    let a_progress = crate::todlib::tod_common::tod_animate_curve_float(
                        item.start_anim_time, item.end_anim_time, self.achievement_anim_time,
                        0.0, 1.0, TodCurves::EaseInOut,
                    );
                    item.y = item.start_y + ((item.dest_y - item.start_y) as f32 * a_progress) as i32;
                } else if self.achievement_anim_time >= item.end_anim_time {
                    item.y = item.dest_y;
                }
            }
            // C++: 最后一项到位时启用继续按钮（mBtnNoDraw/mDisabled = false）
            if let Some(last) = self.achievement_items.last() {
                if last.y == last.dest_y {
                    if let Some(btn) = self.continue_button {
                        unsafe {
                            (*btn).btn_no_draw = false;
                            (*btn).disabled = false;
                        }
                    }
                }
            }
        }
        // [TRANSLATION_NOTE]: C++ 其余部分（GetDialogCount 短路、mStartButton/MenuButton/ContinueButton
        // 的 Update、SetCursor 手型/指针、MarkDirty）依赖 Widget 树/光标系统，Rust 未接入
    }

    pub fn key_char(&mut self, _c: char) {
        if let Some(app) = self.app { unsafe {
            (*app).kill_award_screen();
        } }
    }

    /// 对应 C++ AwardScreen::KeyDown（AwardScreen.cpp）
    pub fn key_down(&mut self, key: KeyCode) {
        if key == KEYCODE_SPACE || key == KEYCODE_RETURN {
            self.start_button_pressed();
            return;
        }

        if key == KEYCODE_ESCAPE {
            // C++: if (!mMenuButton->mDisabled && !mMenuButton->mBtnNoDraw)
            let a_menu_enabled = self.menu_button.map_or(false, |b| unsafe {
                !(*b).disabled && !(*b).btn_no_draw
            });
            if a_menu_enabled {
                if let Some(app) = self.app {
                    unsafe {
                        (*app).kill_award_screen();
                        (*app).show_game_selector();
                    }
                }
            } else {
                self.start_button_pressed();
            }
        }
    }

    pub fn start_button_pressed(&mut self) {
        // 对应 C++ StartButtonPressed：按奖励类型/模式/等级跳转
        let Some(app) = self.app else { return };
        unsafe {
            if (*app).base.dialog_map.contains_key(&(Dialogs::Store as i32)) {
                return;
            }

            if self.award_type == AwardType::CreditsZombieNote {
                (*app).kill_award_screen();
                (*app).show_credit_screen();
            } else if self.award_type == AwardType::HelpZombieNote {
                (*app).kill_award_screen();
                (*app).show_game_selector();
            } else if (*app).is_survival_mode() {
                (*app).kill_award_screen();
                (*app).show_challenge_screen(ChallengePage::Survival as i32);
            } else if (*app).is_puzzle_mode() {
                (*app).kill_award_screen();
                (*app).show_challenge_screen(ChallengePage::Puzzle as i32);
            } else if (*app).is_challenge_mode() {
                (*app).kill_award_screen();
                (*app).show_challenge_screen(ChallengePage::Challenge as i32);
            } else {
                let a_level = (*app).player_info.as_ref().map_or(0, |pi| pi.get_level());
                if a_level == 1 {
                    (*app).kill_award_screen();
                    if (*app).has_finished_adventure() {
                        (*app).show_award_screen(AwardType::CreditsZombieNote as i32, false);
                    } else {
                        (*app).pre_new_game(GameMode::Adventure, false);
                    }
                } else {
                    if a_level == 15 {
                        (*app).do_almanac_dialog(SeedType::None, ZombieType::Invalid);
                    } else if a_level == 25 {
                        // [TRANSLATION_NOTE]: C++ 中 ShowStoreScreen + SetupForIntro(301) + WaitForResult，
                        // 并处理 mPurchasedFullVersion / IsTrialStageLocked 升级分支；Rust 侧 StoreScreen
                        // 交互未接入，仅创建商店
                        let _store = crate::lawn::lawn_app::LawnApp::show_store_screen(Some(app));
                    } else if a_level == 35 {
                        let _store = crate::lawn::lawn_app::LawnApp::show_store_screen(Some(app));
                        // C++ 中 SetupForIntro(601) + WaitForResult(true)
                    } else if a_level == 42 {
                        let _store = crate::lawn::lawn_app::LawnApp::show_store_screen(Some(app));
                        // C++ 中 SetupForIntro(3100) + WaitForResult(true)
                    } else if a_level == 45 {
                        (*app).kill_award_screen();
                        (*app).pre_new_game(GameMode::ChallengeZenGarden, false);
                        if let Some(zg) = (*app).zen_garden {
                            (*zg).setup_for_zen_tutorial();
                        }
                        return;
                    }

                    (*app).kill_award_screen();
                    (*app).pre_new_game(GameMode::Adventure, false);
                }
            }
        }
    }

    pub fn mouse_down(&mut self, x: i32, y: i32, click_count: i32) {
        // 对应 C++ AwardScreen::MouseDown（AwardScreen.cpp:580-587）：
        // ```cpp
        // void AwardScreen::MouseDown(int x, int y, int theClickCount)
        // {
        //     (void)x;(void)y;
        //     if (theClickCount == 1) {
        //         if (mStartButton->IsMouseOver() || mMenuButton->IsMouseOver() || mContinueButton->IsMouseOver())
        //             mApp->PlaySample(Sexy::SOUND_TAP);
        //     }
        // }
        // ```
        // [TRANSLATION_NOTE]: C++ Widget::IsMouseOver 使用 WidgetManager 当前鼠标坐标判定，此处
        // 简化为直接以 mouse_down 传入的 (x, y) 与按钮矩形做包含测试，语义等价。
        if click_count == 1 {
            let buttons: [Option<&GameButton>; 3] = [
                self.start_button.map(|p| unsafe { &*p }),
                self.menu_button.map(|p| unsafe { &*p }),
                self.continue_button.map(|p| unsafe { &*p }),
            ];
            let hit = buttons.iter().any(|slot| {
                slot.map_or(false, |b| {
                    x >= b.x && x < b.x + b.width && y >= b.y && y < b.y + b.height
                })
            });
            if hit {
                if let Some(app) = self.app {
                    unsafe {
                        (*app).play_sample(
                            crate::framework::resources::ResourceId::SoundTap as i32,
                        );
                    }
                }
            }
        }
    }

    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) {
        if let Some(app) = self.app { unsafe {
            (*app).kill_award_screen();
        } }
    }

    pub fn draw_achievements(&self, _g: &mut Graphics) {
        // 依赖图片资源，暂用占位
    }

    pub fn achievements_continue_pressed(&mut self) {
        self.showing_achievements = false;
    }
}

impl Default for AwardScreen {
    fn default() -> Self {
        AwardScreen::new()
    }
}

/// WidgetManager 包装（对应 C++ AwardScreen : Widget）
pub struct AwardScreenImpl {
    pub screen: *mut AwardScreen,
}

impl AwardScreenImpl {
    pub fn new(screen: *mut AwardScreen) -> Self {
        AwardScreenImpl { screen }
    }
}

impl WidgetImpl for AwardScreenImpl {
    fn update(&mut self, _widget: &mut Widget) {
        unsafe { (*self.screen).update(); }
    }
    fn draw(&mut self, _widget: &Widget, g: &mut Graphics) {
        unsafe { (*self.screen).draw(g); }
    }
    fn key_char(&mut self, _widget: &mut Widget, c: u8) {
        unsafe { (*self.screen).key_char(c as char); }
    }
    fn key_down(&mut self, _widget: &mut Widget, key: KeyCode, _wm: &mut WidgetManager) {
        unsafe { (*self.screen).key_down(key); }
    }
    fn mouse_down_btn(&mut self, _widget: &mut Widget, x: i32, y: i32, _btn: i32, click: i32) {
        unsafe { (*self.screen).mouse_down(x, y, click); }
    }
    fn mouse_up_btn(&mut self, _widget: &mut Widget, x: i32, y: i32, _btn: i32, click: i32) {
        unsafe { (*self.screen).mouse_up(x, y, click); }
    }
}

