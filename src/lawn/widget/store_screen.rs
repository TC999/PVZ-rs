// PvZ Portable Rust 翻译 — StoreScreen（商店界面）
// 对应 C++ src/Lawn/Widget/StoreScreen.h / StoreScreen.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::key_codes::{KEYCODE_ESCAPE, KEYCODE_RETURN, KEYCODE_SPACE, KeyCode};
use crate::framework::widget::dialog::{BUTTONS_FOOTER, BUTTONS_YES_NO, ID_OK};
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::coin::Coin;
use crate::lawn::game_enums::*;
use crate::lawn::lawn_common::get_current_days_since_2000;
use crate::lawn::plant::Plant;
use crate::lawn::system::music::MusicTune;
use crate::lawn::system::player_info::PottedPlant;
use crate::lawn::widget::achievements_screen::{AchievementId, ReportAchievement};
use crate::lawn::widget::seed_chooser_screen::SeedChooserScreen;
use crate::todlib::tod_common::{
    rand_range_int, tod_animate_curve, tod_pick_from_weighted_array, tod_string_translate,
    TodWeightedArray,
};
use crate::todlib::tod_foley::FoleyType;

/// 页面插槽最大数（对应 C++ #define MAX_PAGE_SPOTS 8）
pub const MAX_PAGE_SPOTS: usize = 8;
/// 最大购买数（对应 C++ #define MAX_PURCHASES 80）
pub const MAX_PURCHASES: usize = 80;
/// 按钮 ID（对应 C++ StoreScreen.h StoreScreen_Back/Prev/Next）
pub const STORESCREEN_BACK: i32 = 100;
pub const STORESCREEN_PREV: i32 = 101;
pub const STORESCREEN_NEXT: i32 = 102;
/// 购买数量偏移（对应 C++ #define PURCHASE_COUNT_OFFSET 1000）
const PURCHASE_COUNT_OFFSET: i32 = 1000;
/// 商店绘制常量（对应 C++ StoreScreen.cpp 顶部 constexpr）
const STORESCREEN_ITEMOFFSET_1_X: i32 = 422;
const STORESCREEN_ITEMOFFSET_1_Y: i32 = 206;
const STORESCREEN_ITEMOFFSET_2_X: i32 = 372;
const STORESCREEN_ITEMOFFSET_2_Y: i32 = 310;
const STORESCREEN_ITEMSIZE: i32 = 74;
const STORESCREEN_COINBANK_X: i32 = 650;
const STORESCREEN_COINBANK_Y: i32 = 559;
const STORESCREEN_PAGESTRING_X: i32 = 470;
const STORESCREEN_PAGESTRING_Y: i32 = 500;
/// 对话框 ID（对应 C++ ConstEnums.h Dialogs 枚举值）
const DIALOG_NOT_ENOUGH_MONEY: i32 = 25;
const DIALOG_UPGRADED: i32 = 26;
const DIALOG_STORE_PURCHASE: i32 = 46;
const DIALOG_MESSAGE: i32 = 48;
/// 光标（对应 C++ CursorType；Rust CursorType 枚举缺 Hand 变体，直接用数值）
const CURSOR_POINTER: i32 = 0;
const CURSOR_HAND: i32 = 1;

/// 每页商品槽位表（对应 C++ 静态数组 gStoreItemSpots）
static G_STORE_ITEM_SPOTS: [[StoreItem; MAX_PAGE_SPOTS]; 4] = [
    [
        StoreItem::PacketUpgrade,
        StoreItem::PoolCleaner,
        StoreItem::Rake,
        StoreItem::RoofCleaner,
        StoreItem::PlantGatlingpea,
        StoreItem::PlantTwinsunflower,
        StoreItem::PlantGloomshroom,
        StoreItem::PlantCattail,
    ],
    [
        StoreItem::PlantSpikerock,
        StoreItem::PlantGoldMagnet,
        StoreItem::PlantWintermelon,
        StoreItem::PlantCobcannon,
        StoreItem::PlantImitater,
        StoreItem::Firstaid,
        StoreItem::Invalid,
        StoreItem::Invalid,
    ],
    [
        StoreItem::PottedMarigold1,
        StoreItem::PottedMarigold2,
        StoreItem::PottedMarigold3,
        StoreItem::GoldWateringcan,
        StoreItem::Fertilizer,
        StoreItem::BugSpray,
        StoreItem::Phonograph,
        StoreItem::GardeningGlove,
    ],
    [
        StoreItem::MushroomGarden,
        StoreItem::AquariumGarden,
        StoreItem::WheelBarrow,
        StoreItem::StinkyTheSnail,
        StoreItem::TreeOfWisdom,
        StoreItem::TreeFood,
        StoreItem::Invalid,
        StoreItem::Invalid,
    ],
];

/// 商店界面（对应 C++ StoreScreen）
pub struct StoreScreen {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub back_button: Option<*mut DialogButton>,
    pub prev_button: Option<*mut DialogButton>,
    pub next_button: Option<*mut DialogButton>,
    pub overlay_widget: Option<*mut Widget>,
    pub store_time: i32,
    pub bubble_text: String,
    pub bubble_count_down: i32,
    pub bubble_click_to_continue: bool,
    pub ambient_speech_count_down: i32,
    pub previous_ambient_speech_index: i32,
    pub page: StorePages,
    pub mouse_over_item: StoreItem,
    pub hatch_timer: i32,
    pub hatch_open: bool,
    pub shake_x: i32,
    pub shake_y: i32,
    pub start_dialog: i32,
    pub easy_buying_cheat: bool,
    pub wait_for_dialog: bool,
    pub potted_plant_specs: PottedPlant,
    pub coins: Vec<Coin>, // C++ DataArray<Coin>（1024 定长）简化：Vec push 即 DataArrayAlloc
    pub drawn_once: bool,
    pub go_to_tree_now: bool,
    pub purchased_full_version: bool,
    pub trial_locked_when_store_opened: bool,
    pub added_at_update_count: i32,
    pub result: i32, // 对应 C++ Dialog::mResult
    pub x: i32,      // 对应 C++ mX（Resize(0,0,BOARD_WIDTH,BOARD_HEIGHT)，恒 0）
    pub y: i32,      // 对应 C++ mY
}

impl StoreScreen {
    /// 对应 C++ StoreScreen 构造函数（StoreScreen.cpp 82-161）
    pub fn new(app: Option<*mut crate::lawn::lawn_app::LawnApp>) -> Self {
        let mut potted_plant_specs = PottedPlant::new();
        potted_plant_specs.initialize_potted_plant(SeedType::Marigold);
        potted_plant_specs.draw_variation = unsafe {
            std::mem::transmute::<i32, DrawVariation>(rand_range_int(
                DrawVariation::MarigoldWhite as i32,
                DrawVariation::MarigoldLightGreen as i32,
            ))
        };
        let mut screen = StoreScreen {
            app,
            back_button: None,
            prev_button: None,
            next_button: None,
            overlay_widget: None,
            store_time: 0,
            bubble_text: String::new(),
            bubble_count_down: 0,
            bubble_click_to_continue: false,
            ambient_speech_count_down: 200,
            previous_ambient_speech_index: -1,
            page: StorePages::SlotUpgrades,
            mouse_over_item: StoreItem::Invalid,
            hatch_timer: 0,
            hatch_open: true,
            shake_x: 0,
            shake_y: 0,
            start_dialog: -1,
            easy_buying_cheat: false,
            wait_for_dialog: false,
            potted_plant_specs,
            coins: Vec::new(),
            drawn_once: false,
            go_to_tree_now: false,
            purchased_full_version: false,
            trial_locked_when_store_opened: app.map_or(false, |a| unsafe { (*a).is_trial_stage_locked() }),
            added_at_update_count: app.map_or(0, |a| unsafe { (*a).base.m_update_count }),
            result: -1, // 对应 C++ Dialog 构造 mResult = ID_NONE
            x: 0,
            y: 0,
        };
        // [TRANSLATION_NOTE]: C++ 构造函数中创建 mBackButton/mPrevButton/mNextButton（NewLawnButton，
        // 依赖 IMAGE_STORE_* 皮肤资源，未接入，暂不创建；对应报告第四节 4）与 mOverlayWidget。
        screen
    }

    /// 对应 C++ StoreScreen::GetStoreItemType
    pub fn get_store_item_type(&self, spot_index: i32) -> StoreItem {
        if (self.page as i32) < (StorePages::NumPages as i32) && spot_index < MAX_PAGE_SPOTS as i32 {
            if self.page == StorePages::SlotUpgrades
                && spot_index == 6
                && self.app.map_or(false, |app| unsafe { (*app).is_trial_stage_locked() })
            {
                return StoreItem::Pvz;
            }
            return G_STORE_ITEM_SPOTS[self.page as usize][spot_index as usize];
        }
        StoreItem::Invalid
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

    /// 对应 C++ StoreScreen::DrawItemIcon（StoreScreen.cpp 309-416）
    pub fn draw_item_icon(&self, g: &mut Graphics, the_item_position: i32, the_item_type: StoreItem, the_is_for_highlight: bool) {
        if the_is_for_highlight {
            // C++: DRAWMODE_ADDITIVE + Color(255,255,255,96) 调亮
            g.set_draw_mode(1); // Graphics::DRAWMODE_ADDITIVE
            g.set_color(&crate::framework::color::Color::new(255, 255, 255, 96));
            g.set_colorize_images(true);
        }

        let (mut a_pos_x, mut a_pos_y) = (0, 0);
        Self::get_store_position(the_item_position, &mut a_pos_x, &mut a_pos_y);
        match the_item_type {
            StoreItem::PacketUpgrade => {
                let a_img = self.get_resource_image("IMAGE_STORE_PACKETUPGRADE");
                if !a_img.is_null() {
                    g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 7, a_pos_y + 7);
                }
                if the_is_for_highlight {
                    g.set_draw_mode(0); // Graphics::DRAWMODE_NORMAL
                    g.set_colorize_images(false);
                }
                // C++: [STORE_UPGRADE_SLOTS] = mPurchases[PACKET_UPGRADE] + 7，HOUSEOFTERROR16 居中
                let a_slot_text = self.app.map_or(String::new(), |app| unsafe {
                    (*app).player_info.as_ref().map_or(String::new(), |pi| {
                        let a_purchases = pi.m_purchases.get(StoreItem::PacketUpgrade as usize).copied().unwrap_or(0);
                        format!("{}", a_purchases + 7)
                    })
                });
                self.draw_item_label_centered(g, &a_slot_text, a_pos_x + 28, a_pos_y + 30, 16);
            }
            StoreItem::PoolCleaner => {
                let a_img = self.get_resource_image("IMAGE_ICON_POOLCLEANER");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x + 1, a_pos_y + 7); }
            }
            StoreItem::Rake => {
                let a_img = self.get_resource_image("IMAGE_ICON_RAKE");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 5, a_pos_y + 10); }
            }
            StoreItem::RoofCleaner => {
                let a_img = self.get_resource_image("IMAGE_ICON_ROOFCLEANER");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x, a_pos_y + 28); }
            }
            StoreItem::PlantImitater => {
                let a_img = self.get_resource_image("IMAGE_IMITATERSEED");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x, a_pos_y); }
            }
            StoreItem::MushroomGarden => {
                let a_img = self.get_resource_image("IMAGE_STORE_MUSHROOMGARDENICON");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 8, a_pos_y + 2); }
            }
            StoreItem::AquariumGarden => {
                let a_img = self.get_resource_image("IMAGE_STORE_AQUARIUMGARDENICON");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 8, a_pos_y + 2); }
            }
            StoreItem::TreeOfWisdom => {
                let a_img = self.get_resource_image("IMAGE_STORE_TREEOFWISDOMICON");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 8, a_pos_y + 2); }
            }
            StoreItem::Firstaid => {
                let a_img = self.get_resource_image("IMAGE_STORE_FIRSTAIDWALLNUTICON");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 1, a_pos_y + 13); }
            }
            StoreItem::Pvz => {
                let a_img = self.get_resource_image("IMAGE_STORE_PVZICON");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x, a_pos_y - 9); }
            }
            StoreItem::TreeFood => {
                let a_img = self.get_resource_image("IMAGE_TREEFOOD");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 8, a_pos_y - 2); }
            }
            StoreItem::StinkyTheSnail => {
                let a_img = self.get_resource_image("IMAGE_REANIM_STINKY_TURN3");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 24, a_pos_y + 14); }
            }
            StoreItem::GoldWateringcan => {
                let a_img = self.get_resource_image("IMAGE_WATERINGCANGOLD");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 14, a_pos_y - 4); }
            }
            StoreItem::Fertilizer => {
                let a_img = self.get_resource_image("IMAGE_FERTILIZER");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 11, a_pos_y - 2); }
                self.draw_item_label_right(g, "x5", a_pos_x + 56, a_pos_y + 62, 16);
            }
            StoreItem::Phonograph => {
                let a_img = self.get_resource_image("IMAGE_PHONOGRAPH");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 12, a_pos_y + 3); }
            }
            StoreItem::BugSpray => {
                let a_img = self.get_resource_image("IMAGE_BUG_SPRAY");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 12, a_pos_y + 3); }
                self.draw_item_label_right(g, "x5", a_pos_x + 56, a_pos_y + 62, 16);
            }
            StoreItem::GardeningGlove => {
                let a_img = self.get_resource_image("IMAGE_ZEN_GARDENGLOVE");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 12, a_pos_y + 3); }
            }
            StoreItem::WheelBarrow => {
                let a_img = self.get_resource_image("IMAGE_ZEN_WHEELBARROW");
                if !a_img.is_null() { g.draw_image_xy(unsafe { &*a_img }, a_pos_x - 12, a_pos_y + 3); }
            }
            _ => {
                if Self::is_potted_plant(the_item_type) {
                    // C++: mApp->mZenGarden->DrawPottedPlantIcon(g, aPosX, aPosY, &mPottedPlantSpecs)
                    if let Some(app) = self.app {
                        unsafe {
                            if let Some(zg) = (*app).zen_garden.as_mut() {
                                (**zg).draw_potted_plant_icon(g, a_pos_x as f32, a_pos_y as f32, &self.potted_plant_specs);
                            }
                        }
                    }
                } else {
                    // C++: DrawSeedPacket(g, aPosX, aPosY, (SeedType)(theItemType + 40), SEED_NONE, 0, 255, false, false)
                    let a_seed_type = unsafe { std::mem::transmute::<i32, crate::lawn::game_enums::SeedType>((the_item_type as i32) + 40) };
                    crate::lawn::seed_packet::draw_seed_packet(g, a_pos_x as f32, a_pos_y as f32, a_seed_type, crate::lawn::game_enums::SeedType::None, 0.0, 255, false, false);
                }
            }
        }

        g.set_draw_mode(0); // Graphics::DRAWMODE_NORMAL
        g.set_colorize_images(false);
    }

    /// 用 HOUSEOFTERROR 字号在 (x,y) 水平居中绘制商品标签文字（TRANSLATION_NOTE: PvzpDrawStringWrapped 简化）
    fn draw_item_label_centered(&self, g: &mut Graphics, a_text: &str, a_center_x: i32, a_y: i32, a_size: i32) {
        let mut a_font = crate::framework::graphics::font::Font::new("Houseofterror", a_size);
        a_font.ascent = 13;
        a_font.font_height = a_size;
        g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
        g.set_color(&crate::framework::color::Color::WHITE);
        let a_w = a_font.string_width(a_text);
        g.draw_string(a_text, a_center_x - a_w / 2, a_y);
    }

    /// 右对齐商品标签文字（对应 C++ DS_ALIGN_RIGHT）
    fn draw_item_label_right(&self, g: &mut Graphics, a_text: &str, a_right_x: i32, a_y: i32, a_size: i32) {
        let mut a_font = crate::framework::graphics::font::Font::new("Houseofterror", a_size);
        a_font.ascent = 13;
        a_font.font_height = a_size;
        g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
        g.set_color(&crate::framework::color::Color::WHITE);
        let a_w = a_font.string_width(a_text);
        g.draw_string(a_text, a_right_x - a_w, a_y);
    }
/// 对应 C++ StoreScreen::DrawItem（StoreScreen.cpp 418-460）
    pub fn draw_item(&self, g: &mut Graphics, the_item_position: i32, the_item_type: StoreItem) {
        if self.is_item_unavailable(the_item_type) {
            return;
        }

        self.draw_item_icon(g, the_item_position, the_item_type, false);

        let (mut a_pos_x, mut a_pos_y) = (0, 0);
        Self::get_store_position(the_item_position, &mut a_pos_x, &mut a_pos_y);
        if the_item_type != StoreItem::Pvz {
            // C++: IMAGE_STORE_PRICETAG + 价格（BRIANNETOD12 黑字居中）
            let a_price_img = self.get_resource_image("IMAGE_STORE_PRICETAG");
            if !a_price_img.is_null() {
                g.draw_image_xy(unsafe { &*a_price_img }, a_pos_x - 3, a_pos_y + 70);
            }
            let a_cost_string = crate::lawn::lawn_app::LawnApp::get_money_string(Self::get_item_cost(the_item_type));
            let mut a_font = crate::framework::graphics::font::Font::new("Briannetod", 12);
            a_font.ascent = 13;
            a_font.font_height = 12;
            g.set_font(&mut a_font as *mut crate::framework::graphics::font::Font);
            g.set_color(&crate::framework::color::Color::BLACK);
            let a_w = a_font.string_width(&a_cost_string);
            g.draw_string(&a_cost_string, a_pos_x + 23 - a_w / 2, a_pos_y + 85);
        }
        if self.is_coming_soon(the_item_type) {
            // C++: [COMING_SOON] 红字 HOUSEOFTERROR16 居中于商品区
            self.draw_item_label_centered(g, "[COMING_SOON]", a_pos_x + 30, a_pos_y + 28, 16);
            g.set_color(&crate::framework::color::Color::new(255, 0, 0, 255));
        } else if self.is_item_sold_out(the_item_type) {
            self.draw_item_label_centered(g, "[SOLD_OUT]", a_pos_x + 25, a_pos_y + 28, 16);
            g.set_color(&crate::framework::color::Color::new(255, 0, 0, 255));
        } else if self.mouse_over_item == the_item_type {
            if the_item_type as i32 >= 0 && the_item_type as i32 <= 8 {
                // C++: IMAGE_SEEDPACKETFLASH（种子包高亮）
                let a_flash = self.get_resource_image("IMAGE_SEEDPACKETFLASH");
                if !a_flash.is_null() {
                    g.draw_image_xy(unsafe { &*a_flash }, a_pos_x, a_pos_y);
                }
            } else {
                self.draw_item_icon(g, the_item_position, the_item_type, true);
            }
        }
    }
    /// 对应 C++ StoreScreen::Draw（StoreScreen.cpp 462-533）
    pub fn draw(&self, g: &mut Graphics) {
        g.set_linear_blend(true);
        // [TRANSLATION_NOTE]: C++ mDrawnOnce = true（&self 下不写成员）

        // C++: aStoreSignPosY = PvzpAnimateCurve(50, 110, mStoreTime, -150, 0, CURVE_EASE_IN_OUT)
        let a_store_sign_pos_y = crate::todlib::tod_common::tod_animate_curve(
            50, 110, self.store_time, -150, 0, TodCurves::EaseInOut,
        );

        // 背景（昼夜）
        let a_is_night = self.app.map_or(false, |app| unsafe { (*app).is_night() });
        let a_bg_key = if a_is_night { "IMAGE_STORE_BACKGROUNDNIGHT" } else { "IMAGE_STORE_BACKGROUND" };
        let a_bg = self.get_resource_image(a_bg_key);
        if !a_bg.is_null() {
            g.draw_image_xy(unsafe { &*a_bg }, 0, 0);
        }

        // 疯狂戴夫的车（后车厢开/合分支）
        if self.hatch_timer == 0 && self.hatch_open {
            let a_car = self.get_resource_image("IMAGE_STORE_CAR");
            if !a_car.is_null() {
                g.draw_image_xy(unsafe { &*a_car }, self.shake_x + 196, self.shake_y + 138);
            }
            let a_hatch = self.get_resource_image("IMAGE_STORE_HATCHBACKOPEN");
            if !a_hatch.is_null() {
                g.draw_image_xy(unsafe { &*a_hatch }, self.shake_x + 299, self.shake_y);
            }
            if a_is_night {
                let a_car_night = self.get_resource_image("IMAGE_STORE_CAR_NIGHT");
                if !a_car_night.is_null() {
                    g.draw_image_xy(unsafe { &*a_car_night }, self.shake_x + 688, self.shake_y + 193);
                }
            }
        } else {
            let a_car_closed = self.get_resource_image("IMAGE_STORE_CARCLOSED");
            if !a_car_closed.is_null() {
                g.draw_image_xy(unsafe { &*a_car_closed }, self.shake_x + 196, self.shake_y + 138);
            }
            if a_is_night {
                let a_car_night = self.get_resource_image("IMAGE_STORE_CAR_NIGHT");
                if !a_car_night.is_null() {
                    g.draw_image_xy(unsafe { &*a_car_night }, self.shake_x + 688, self.shake_y + 193);
                }
                let a_car_closed_night = self.get_resource_image("IMAGE_STORE_CARCLOSED_NIGHT");
                if !a_car_closed_night.is_null() {
                    g.draw_image_xy(unsafe { &*a_car_closed_night }, self.shake_x + 337, self.shake_y + 187);
                }
            }
        }

        // 商店招牌
        let a_sign = self.get_resource_image("IMAGE_STORE_SIGN");
        if !a_sign.is_null() {
            g.draw_image_xy(unsafe { &*a_sign }, 285, a_store_sign_pos_y);
        }

        // C++: Graphics gCrazyDave(*g) 平移后 mApp->DrawCrazyDave(&gCrazyDave)
        // [TRANSLATION_NOTE]: Graphics 复制 + mTransX/Y 偏移未模拟，直接调用等价绘制
        if let Some(app) = self.app {
            unsafe { (*app).draw_crazy_dave(g); }
        }

        // 商品循环
        if self.hatch_timer == 0 && self.hatch_open {
            for i in 0..MAX_PAGE_SPOTS {
                let a_store_item = self.get_store_item_type(i as i32);
                if a_store_item != StoreItem::Invalid {
                    self.draw_item(g, i as i32, a_store_item);
                }
            }
        }

        // coinbank + 金币数（C++: IMAGE_COINBANK + FONT_CONTINUUMBOLD14 右对齐）
        let a_coinbank = self.get_resource_image("IMAGE_COINBANK");
        if !a_coinbank.is_null() {
            g.draw_image_xy(unsafe { &*a_coinbank }, STORESCREEN_COINBANK_X, STORESCREEN_COINBANK_Y);
        }
        let a_coin_label = self.app.map_or(String::new(), |app| unsafe {
            (*app).player_info.as_ref().map_or(String::new(), |pi| {
                crate::lawn::lawn_app::LawnApp::get_money_string(pi.m_coins)
            })
        });
        let mut a_coin_font = crate::framework::graphics::font::Font::new("Continuumbold", 14);
        a_coin_font.ascent = 13;
        a_coin_font.font_height = 14;
        g.set_font(&mut a_coin_font as *mut crate::framework::graphics::font::Font);
        g.set_color(&crate::framework::color::Color::new(180, 255, 90, 255));
        let a_coin_w = a_coin_font.string_width(&a_coin_label);
        g.draw_string(&a_coin_label, STORESCREEN_COINBANK_X + 116 - a_coin_w, STORESCREEN_COINBANK_Y + 24);

        // 分页文本（C++: !mPrevButton->mDisabled 时 [STORE_PAGE]）
        let a_prev_disabled = self.prev_button.map_or(true, |b| unsafe {
            let b_ref = &*b;
            b_ref.disabled
        });
        if !a_prev_disabled {
            let mut a_num_pages = 0;
            // C++: for (StorePages aPage = STORE_PAGE_SLOT_UPGRADES; aPage < NUM_STORE_PAGES; aPage++)
            let mut a_page = StorePages::SlotUpgrades;
            while (a_page as i32) < (StorePages::NumPages as i32) {
                if self.is_page_shown(a_page) {
                    a_num_pages += 1;
                }
                a_page = unsafe { std::mem::transmute::<i32, StorePages>(a_page as i32 + 1) };
            }
            let a_page_string = format!("Page {} / {}", self.page as i32 + 1, a_num_pages);
            let mut a_page_font = crate::framework::graphics::font::Font::new("Briannetod", 12);
            a_page_font.ascent = 13;
            a_page_font.font_height = 12;
            g.set_font(&mut a_page_font as *mut crate::framework::graphics::font::Font);
            g.set_color(&crate::framework::color::Color::new(80, 80, 80, 255));
            let a_page_w = a_page_font.string_width(&a_page_string);
            g.draw_string(&a_page_string, STORESCREEN_PAGESTRING_X - a_page_w / 2, STORESCREEN_PAGESTRING_Y);
        }
    }
    pub fn draw_overlay(&self, g: &mut Graphics) {
        // 对应 C++ DrawOverlay：遍历未死亡硬币绘制
        for a_coin in &self.coins {
            if !a_coin.dead {
                a_coin.draw(g);
            }
        }
    }

    pub fn is_full_version_only(&self, item: StoreItem) -> bool {
        if let Some(app) = self.app {
            unsafe {
                if !(*app).is_trial_stage_locked() {
                    return false;
                }
                if item == StoreItem::PacketUpgrade
                    && (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::PacketUpgrade as usize] >= 2
                {
                    return true;
                }
                return item == StoreItem::PlantTwinsunflower;
            }
        } else {
            false
        }
    }

    pub fn is_potted_plant(item: StoreItem) -> bool {
        matches!(
            item,
            StoreItem::PottedMarigold1 | StoreItem::PottedMarigold2 | StoreItem::PottedMarigold3
        )
    }

    /// 对应 C++ StoreScreen::IsComingSoon
    pub fn is_coming_soon(&self, item: StoreItem) -> bool {
        if self.is_full_version_only(item) {
            return true;
        }
        if let Some(app) = self.app {
            unsafe {
                if item == StoreItem::WheelBarrow {
                    return (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::MushroomGarden as usize] == 0
                        && (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::AquariumGarden as usize] == 0;
                } else if Self::is_potted_plant(item) {
                    return !(*app).has_finished_adventure();
                } else if item == StoreItem::TreeFood {
                    return (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::TreeOfWisdom as usize] == 0
                        || (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::TreeFood as usize] < PURCHASE_COUNT_OFFSET;
                }
            }
        }
        false
    }

    /// 对应 C++ StoreScreen::IsItemSoldOut
    pub fn is_item_sold_out(&self, item: StoreItem) -> bool {
        if let Some(app) = self.app {
            unsafe {
                let a_player = (*app).player_info.as_ref().unwrap();
                if item == StoreItem::Invalid {
                    return false;
                } else if item == StoreItem::PacketUpgrade {
                    return a_player.m_purchases[StoreItem::PacketUpgrade as usize] >= 4;
                } else if item == StoreItem::Fertilizer || item == StoreItem::BugSpray {
                    return a_player.m_purchases[item as usize] > PURCHASE_COUNT_OFFSET + 15;
                } else if item == StoreItem::TreeFood {
                    return a_player.m_purchases[StoreItem::TreeFood as usize] >= PURCHASE_COUNT_OFFSET + 10;
                } else if item == StoreItem::BonusLawnMower {
                    return a_player.m_purchases[StoreItem::BonusLawnMower as usize] >= 2;
                } else if Self::is_potted_plant(item) {
                    let a_zen_full = (*app).zen_garden.map_or(false, |zg| unsafe { (*zg).is_zen_garden_full(true) });
                    return a_zen_full || a_player.m_purchases[item as usize] == get_current_days_since_2000();
                } else {
                    return a_player.m_purchases[item as usize] != 0;
                }
            }
        } else {
            false
        }
    }

    /// 对应 C++ StoreScreen::IsItemUnavailable
    pub fn is_item_unavailable(&self, item: StoreItem) -> bool {
        if self.easy_buying_cheat {
            return false;
        }
        let app_ref = match self.app {
            Some(app) => unsafe { &*app },
            None => return false,
        };
        let a_finished = app_ref.has_finished_adventure();
        let a_level = app_ref.player_info.as_ref().map_or(0, |p| p.m_level);

        if item == StoreItem::RoofCleaner {
            return app_ref.is_trial_stage_locked() || (!a_finished && a_level < 42);
        }
        if item == StoreItem::PlantGloomshroom {
            return app_ref.is_trial_stage_locked() || (!a_finished && a_level < 35);
        }
        if item == StoreItem::PlantCattail {
            return app_ref.is_trial_stage_locked() || (!a_finished && a_level < 35);
        }
        if item == StoreItem::PlantSpikerock {
            return !a_finished && a_level < 41;
        }
        if item == StoreItem::PlantGoldMagnet {
            return !a_finished && a_level < 41;
        }
        if item == StoreItem::PlantWintermelon
            || item == StoreItem::PlantCobcannon
            || item == StoreItem::PlantImitater
            || item == StoreItem::Firstaid
        {
            return !a_finished;
        }
        false
    }

    /// 对应 C++ StoreScreen::GetStorePosition（两行两列布局）
    pub fn get_store_position(spot_index: i32, pos_x: &mut i32, pos_y: &mut i32) {
        if spot_index <= 3 {
            *pos_x = STORESCREEN_ITEMOFFSET_1_X + STORESCREEN_ITEMSIZE * spot_index;
            *pos_y = STORESCREEN_ITEMOFFSET_1_Y;
        } else {
            *pos_x = STORESCREEN_ITEMOFFSET_2_X + STORESCREEN_ITEMSIZE * (spot_index - 4);
            *pos_y = STORESCREEN_ITEMOFFSET_2_Y;
        }
    }

    /// 对应 C++ StoreScreen::ButtonPress
    pub fn button_press(&mut self, the_id: i32) {
        // 翻页按钮不播点击音效，其余按钮播 SOUND_BUTTONCLICK
        if the_id != STORESCREEN_PREV && the_id != STORESCREEN_NEXT {
            if let Some(app) = self.app {
                unsafe {
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundButtonclick as i32);
                }
            }
        }
    }

    /// 对应 C++ StoreScreen::CanInteractWithButtons
    pub fn can_interact_with_buttons(&self) -> bool {
        self.store_time >= 120 && !self.bubble_click_to_continue && self.hatch_timer <= 0 && !self.wait_for_dialog
    }

    /// 对应 C++ StoreScreen::IsPageShown
    pub fn is_page_shown(&self, page: StorePages) -> bool {
        if let Some(app) = self.app {
            unsafe {
                if (*app).is_trial_stage_locked() {
                    return page == StorePages::SlotUpgrades;
                }
                if (*app).has_finished_adventure() {
                    return true;
                }
                if page == StorePages::PlantUpgrades {
                    return (*app).player_info.as_ref().unwrap().m_level >= 42;
                }
                if page == StorePages::Zen1 {
                    return (*app).player_info.as_ref().unwrap().m_level >= 45;
                }
                return page != StorePages::Zen2;
            }
        } else {
            false
        }
    }

    /// 对应 C++ StoreScreen::GetItemCost（含 BONUS_LAWN_MOWER 与 PACKET_UPGRADE 分级价格）
    pub fn get_item_cost(item: StoreItem) -> i32 {
        if item == StoreItem::BonusLawnMower {
            // C++: gLawnApp->mPlayerInfo->mPurchases[STORE_ITEM_BONUS_LAWN_MOWER] ? 500 : 200
            let a_purchase = crate::lawn::lawn_app::LawnApp::instance()
                .and_then(|app| app.player_info.as_ref())
                .map_or(0, |pi| pi.m_purchases[StoreItem::BonusLawnMower as usize]);
            return if a_purchase != 0 { 500 } else { 200 };
        }
        match item {
            StoreItem::PlantGatlingpea => 500,
            StoreItem::PlantTwinsunflower => 500,
            StoreItem::PlantGloomshroom => 750,
            StoreItem::PlantCattail => 1000,
            StoreItem::PlantWintermelon => 1000,
            StoreItem::PlantGoldMagnet => 300,
            StoreItem::PlantSpikerock => 750,
            StoreItem::PlantCobcannon => 2000,
            StoreItem::PlantImitater => 3000,
            StoreItem::PottedMarigold1 | StoreItem::PottedMarigold2 | StoreItem::PottedMarigold3 => 250,
            StoreItem::GoldWateringcan => 1000,
            StoreItem::Fertilizer => 75,
            StoreItem::BugSpray => 100,
            StoreItem::Phonograph => 1500,
            StoreItem::GardeningGlove => 100,
            StoreItem::MushroomGarden => 3000,
            StoreItem::WheelBarrow => 20,
            StoreItem::StinkyTheSnail => 300,
            StoreItem::PacketUpgrade => {
                // C++: aPurchase == 0 ? 75 : aPurchase == 1 ? 500 : aPurchase == 2 ? 2000 : 8000
                let a_purchase = crate::lawn::lawn_app::LawnApp::instance()
                    .and_then(|app| app.player_info.as_ref())
                    .map_or(0, |pi| pi.m_purchases[StoreItem::PacketUpgrade as usize]);
                if a_purchase == 0 {
                    75
                } else if a_purchase == 1 {
                    500
                } else if a_purchase == 2 {
                    2000
                } else {
                    8000
                }
            }
            StoreItem::PoolCleaner => 100,
            StoreItem::RoofCleaner => 300,
            StoreItem::Rake => 20,
            StoreItem::AquariumGarden => 3000,
            StoreItem::TreeOfWisdom => 1000,
            StoreItem::TreeFood => 250,
            StoreItem::Firstaid => 200,
            _ => 0,
        }
    }

    /// 对应 C++ StoreScreen::CanAffordItem
    pub fn can_afford_item(&self, item: StoreItem) -> bool {
        if let Some(app) = self.app {
            unsafe { (*app).player_info.as_ref().unwrap().m_coins >= Self::get_item_cost(item) }
        } else {
            false
        }
    }

    /// 对应 C++ StoreScreen::EnableButtons
    pub fn enable_buttons(&mut self, enable: bool) {
        let plant_upgrades_page = self.is_page_shown(StorePages::PlantUpgrades);
        // [TRANSLATION_NOTE]: C++ 中 mMouseVisible = theEnable；Rust 侧用 visible 近似
        if self.easy_buying_cheat || plant_upgrades_page || !enable {
            if let Some(btn) = self.next_button {
                unsafe {
                    (&mut *btn).widget.visible = enable;
                    (*btn).set_disabled(!enable);
                }
            }
            if let Some(btn) = self.prev_button {
                unsafe {
                    (&mut *btn).widget.visible = enable;
                    (*btn).set_disabled(!enable);
                }
            }
        }
        if let Some(btn) = self.back_button {
            unsafe {
                (&mut *btn).widget.visible = enable;
                (*btn).set_disabled(!enable);
            }
        }
    }

    /// 对应 C++ StoreScreen::SetupForIntro
    pub fn setup_for_intro(&mut self, dialog_index: i32) {
        self.start_dialog = dialog_index;
        self.hatch_open = false;
        if let Some(btn) = self.back_button {
            unsafe {
                (*btn).set_label(&tod_string_translate("[STORE_NEXT_LEVEL_BUTTON]"));
            }
        }
        self.enable_buttons(false);
    }
}

// ==================== 交互链（对应 C++ StoreScreen.cpp 交互函数） ====================

impl StoreScreen {
    /// 对应 C++ StoreScreen::SetBubbleText
    pub fn set_bubble_text(&mut self, the_crazy_dave_message: i32, the_time: i32, the_click_to_continue: bool) {
        if let Some(app) = self.app {
            unsafe {
                (*app).crazy_dave_talk_index(the_crazy_dave_message);
            }
        }
        self.bubble_count_down = the_time;
        self.bubble_click_to_continue = the_click_to_continue;
    }

    /// 对应 C++ StoreScreen::UpdateMouse
    pub fn update_mouse(&mut self) {
        self.mouse_over_item = StoreItem::Invalid;
        if self.store_time < 120 || self.bubble_click_to_continue || self.hatch_timer > 0 || self.wait_for_dialog {
            return;
        }
        let (a_mouse_x, a_mouse_y) = if let Some(app) = self.app {
            unsafe {
                if let Some(wm) = (*app).base.widget_manager {
                    (
                        wm.as_ref().unwrap().last_mouse_x - self.x,
                        wm.as_ref().unwrap().last_mouse_y - self.y,
                    )
                } else {
                    (0, 0)
                }
            }
        } else {
            (0, 0)
        };
        let mut a_show_finger = false;
        for a_item_pos in 0..MAX_PAGE_SPOTS {
            let a_item_type = self.get_store_item_type(a_item_pos as i32);
            if a_item_type != StoreItem::Invalid && !self.is_item_unavailable(a_item_type) {
                let mut a_item_x = 0;
                let mut a_item_y = 0;
                Self::get_store_position(a_item_pos as i32, &mut a_item_x, &mut a_item_y);
                // C++: Rect(aItemX, aItemY, 50, 87).Contains(aMouseX, aMouseY)
                if (a_item_x..a_item_x + 50).contains(&a_mouse_x) && (a_item_y..a_item_y + 87).contains(&a_mouse_y) {
                    self.mouse_over_item = a_item_type;
                    let mut a_message_index = -1;
                    match a_item_type {
                        StoreItem::PlantGatlingpea => a_message_index = 2000,
                        StoreItem::PlantTwinsunflower => a_message_index = 2001,
                        StoreItem::PlantGloomshroom => a_message_index = 2002,
                        StoreItem::PlantCattail => a_message_index = 2003,
                        StoreItem::PlantWintermelon => a_message_index = 2004,
                        StoreItem::PlantGoldMagnet => a_message_index = 2005,
                        StoreItem::PlantSpikerock => a_message_index = 2006,
                        StoreItem::PlantCobcannon => a_message_index = 2007,
                        StoreItem::PlantImitater => a_message_index = 2008,
                        StoreItem::BonusLawnMower => a_message_index = 2009,
                        StoreItem::PottedMarigold1 | StoreItem::PottedMarigold2 | StoreItem::PottedMarigold3 => {
                            a_message_index = 2010
                        }
                        StoreItem::GoldWateringcan => a_message_index = 2019,
                        StoreItem::Fertilizer => a_message_index = 2020,
                        StoreItem::BugSpray => a_message_index = 2022,
                        StoreItem::Phonograph => a_message_index = 2021,
                        StoreItem::GardeningGlove => a_message_index = 2023,
                        StoreItem::MushroomGarden => a_message_index = 2032,
                        StoreItem::WheelBarrow => a_message_index = 2024,
                        StoreItem::StinkyTheSnail => a_message_index = 2025,
                        StoreItem::PacketUpgrade => {
                            // C++: std::clamp(购买数 + 2011, 2011, 2014)
                            let a_purchase = self.app.map_or(0, |app| unsafe {
                                (*app).player_info.as_ref().unwrap().m_purchases[StoreItem::PacketUpgrade as usize]
                            });
                            a_message_index = (a_purchase + 2011).clamp(2011, 2014);
                        }
                        StoreItem::PoolCleaner => a_message_index = 2026,
                        StoreItem::RoofCleaner => a_message_index = 2027,
                        StoreItem::Rake => a_message_index = 2028,
                        StoreItem::AquariumGarden => a_message_index = 2029,
                        StoreItem::Chocolate => {}
                        StoreItem::TreeOfWisdom => a_message_index = 2030,
                        StoreItem::TreeFood => a_message_index = 2031,
                        StoreItem::Firstaid => a_message_index = 2033,
                        StoreItem::Pvz => a_message_index = 2034,
                        _ => {}
                    }
                    if self.app.map_or(false, |app| unsafe { (*app).m_crazy_dave_message_index != a_message_index }) {
                        self.set_bubble_text(a_message_index, 100, false);
                    } else {
                        self.bubble_count_down = 100;
                    }
                    if self.is_full_version_only(a_item_type)
                        || (!self.is_item_sold_out(a_item_type)
                            && !self.is_item_unavailable(a_item_type)
                            && !self.is_coming_soon(a_item_type))
                    {
                        a_show_finger = true;
                    }
                    break;
                }
            }
        }

        // C++: mApp->SetCursor(mBackButton->mIsOver || mPrevButton->mIsOver || mNextButton->mIsOver || aShowFinger ? CURSOR_HAND : CURSOR_POINTER)
        let a_back_over = self.back_button.map_or(false, |b| unsafe { (*b).widget.is_over });
        let a_prev_over = self.prev_button.map_or(false, |b| unsafe { (*b).widget.is_over });
        let a_next_over = self.next_button.map_or(false, |b| unsafe { (*b).widget.is_over });
        if let Some(app) = self.app {
            unsafe {
                (*app).base.set_cursor(if a_back_over || a_prev_over || a_next_over || a_show_finger {
                    CURSOR_HAND
                } else {
                    CURSOR_POINTER
                });
            }
        }
    }

    /// 对应 C++ StoreScreen::StorePreload
    pub fn store_preload(&mut self) {
        crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CrazyDave);
        crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::ZengardenFertilizer);
        if let Some(app) = self.app {
            unsafe {
                (*app).crazy_dave_enter();
            }
        }

        Plant::preload_plant_resources(SeedType::Garlic);
        Plant::preload_plant_resources(SeedType::Twinsunflower);

        if let Some(app) = self.app {
            unsafe {
                if (*app).has_finished_adventure() {
                    Plant::preload_plant_resources(SeedType::Gloomshroom);
                    Plant::preload_plant_resources(SeedType::Cattail);
                    Plant::preload_plant_resources(SeedType::Wintermelon);
                    Plant::preload_plant_resources(SeedType::GoldMagnet);
                    Plant::preload_plant_resources(SeedType::Spikerock);
                    Plant::preload_plant_resources(SeedType::Cobcannon);
                    Plant::preload_plant_resources(SeedType::Imitater);
                }
            }
        }
    }

    /// 对应 C++ StoreScreen::Update
    pub fn update(&mut self) {
        if let Some(app) = self.app {
            unsafe {
                if let Some(music) = (*app).music.as_mut() {
                    music.make_sure_music_is_playing(MusicTune::TitleCrazyDaveMainTheme);
                }
                (*app).update_crazy_dave();
            }
        }

        for a_coin in &mut self.coins {
            if !a_coin.dead {
                a_coin.update();
            }
        }

        if self.wait_for_dialog {
            return;
        }

        let a_crazy_dave_state = self.app.map_or(CrazyDaveState::Off, |app| unsafe { (*app).m_crazy_dave_state });
        if a_crazy_dave_state == CrazyDaveState::Off {
            // demo sessions preload by update tick instead of the frame-scheduled mDrawnOnce
            // [TRANSLATION_NOTE]: C++ 用 IsInDemoMode()；Rust 侧无 demo 模式，用恒 false 的 is_ice_demo() 近似
            let a_should_preload = if let Some(app) = self.app {
                unsafe {
                    if (*app).is_ice_demo() {
                        (*app).base.m_update_count - self.added_at_update_count >= 2
                    } else {
                        self.drawn_once
                    }
                }
            } else {
                self.drawn_once
            };
            if a_should_preload {
                self.store_preload();
            }
            return;
        }

        self.store_time += 1;
        if a_crazy_dave_state != CrazyDaveState::Off && a_crazy_dave_state != CrazyDaveState::Entering {
            if self.hatch_timer > 0 {
                self.hatch_timer -= 1;
                // 还原上一帧加的抖动位移
                if let Some(btn) = self.back_button {
                    unsafe {
                        (&mut *btn).widget.x -= self.shake_x;
                        (&mut *btn).widget.y -= self.shake_y;
                    }
                }
                if let Some(btn) = self.prev_button {
                    unsafe {
                        (&mut *btn).widget.x -= self.shake_x;
                        (&mut *btn).widget.y -= self.shake_y;
                    }
                }
                if let Some(btn) = self.next_button {
                    unsafe {
                        (&mut *btn).widget.x -= self.shake_x;
                        (&mut *btn).widget.y -= self.shake_y;
                    }
                }

                if self.hatch_timer == 0 {
                    self.enable_buttons(true);
                    self.shake_x = 0;
                    self.shake_y = 0;
                } else {
                    self.shake_x = 0;
                    if self.hatch_timer > 35 {
                        self.shake_y = rand_range_int(1, 3);
                    } else {
                        self.shake_y = 0;
                    }
                }

                if let Some(btn) = self.back_button {
                    unsafe {
                        (&mut *btn).widget.x += self.shake_x;
                        (&mut *btn).widget.y += self.shake_y;
                    }
                }
                if let Some(btn) = self.prev_button {
                    unsafe {
                        (&mut *btn).widget.x += self.shake_x;
                        (&mut *btn).widget.y += self.shake_y;
                    }
                }
                if let Some(btn) = self.next_button {
                    unsafe {
                        (&mut *btn).widget.x += self.shake_x;
                        (&mut *btn).widget.y += self.shake_y;
                    }
                }
            } else if self.start_dialog != -1 {
                self.set_bubble_text(self.start_dialog, 0, true);
                self.start_dialog = -1;
            } else if !self.bubble_click_to_continue {
                if self.bubble_count_down > 0 {
                    self.bubble_count_down -= 1;
                    if self.bubble_count_down == 0 {
                        let a_foley_playing = if let Some(app) = self.app {
                            unsafe {
                                if let Some(ss) = (*app).sound_system.as_ref() {
                                    ss.is_foley_playing(FoleyType::CrazyDaveShort)
                                        || ss.is_foley_playing(FoleyType::CrazyDaveLong)
                                        || ss.is_foley_playing(FoleyType::CrazyDaveExtraLong)
                                } else {
                                    false
                                }
                            }
                        } else {
                            false
                        };
                        if a_foley_playing {
                            self.bubble_count_down = 1;
                        } else if let Some(app) = self.app {
                            unsafe {
                                (*app).crazy_dave_stop_talking();
                            }
                        }
                    }
                } else {
                    self.ambient_speech_count_down -= 1;
                    if self.ambient_speech_count_down <= 0 {
                        // C++: PvzpWeightedArray aPickArray[4]（消息 2015..2018）
                        let mut a_pick_array: [TodWeightedArray; 4] = [TodWeightedArray { item: 0, weight: 0 }; 4];
                        for i in 0i32..4 {
                            let a_message = 2015 + i;
                            a_pick_array[i as usize].item = a_message as usize;
                            if self.previous_ambient_speech_index == a_message {
                                a_pick_array[i as usize].weight = 0;
                            } else if i == 3 {
                                let a_finished = self.app.map_or(false, |app| unsafe { (*app).has_finished_adventure() });
                                a_pick_array[i as usize].weight = if a_finished { 20 } else { 0 };
                            } else {
                                a_pick_array[i as usize].weight = 100;
                            }
                        }

                        let a_dave_message = tod_pick_from_weighted_array(&a_pick_array);
                        self.previous_ambient_speech_index = a_dave_message as i32;
                        self.set_bubble_text(a_dave_message as i32, 800, false);
                        self.ambient_speech_count_down = rand_range_int(500, 1000);
                    }
                }
            }
        }

        self.update_mouse();
        // store opened in trial mode and now unlocked: the player just purchased the full version
        if self.can_interact_with_buttons()
            && self.trial_locked_when_store_opened
            && self.app.map_or(false, |app| unsafe { !(*app).is_trial_stage_locked() })
        {
            self.purchased_full_version = true;
            self.result = ID_OK;
        } else {
            // C++: Widget::Update(); MarkDirty(); —— Rust 侧 StoreScreen 非 Widget 容器，等效空操作
        }
    }

    /// 对应 C++ StoreScreen::AddedToManager
    pub fn added_to_manager(&mut self, the_widget_manager: *mut WidgetManager) {
        // C++: WidgetContainer::AddedToManager + AddWidget(mBackButton/mPrevButton/mNextButton/mOverlayWidget)
        // [TRANSLATION_NOTE]: 按钮/overlay 未创建（见构造函数注释），注册逻辑保留调用点。
        if !the_widget_manager.is_null() {
            unsafe {
                if let Some(btn) = self.back_button {
                    (&mut *the_widget_manager).add_widget((&mut *btn).as_widget_ptr());
                }
                if let Some(btn) = self.prev_button {
                    (&mut *the_widget_manager).add_widget((&mut *btn).as_widget_ptr());
                }
                if let Some(btn) = self.next_button {
                    (&mut *the_widget_manager).add_widget((&mut *btn).as_widget_ptr());
                }
            }
        }
    }

    /// 对应 C++ StoreScreen::RemovedFromManager
    pub fn removed_from_manager(&mut self, the_widget_manager: *mut WidgetManager) {
        if !the_widget_manager.is_null() {
            unsafe {
                if let Some(btn) = self.back_button {
                    (&mut *the_widget_manager).remove_widget((&mut *btn).as_widget_ptr());
                }
                if let Some(btn) = self.prev_button {
                    (&mut *the_widget_manager).remove_widget((&mut *btn).as_widget_ptr());
                }
                if let Some(btn) = self.next_button {
                    (&mut *the_widget_manager).remove_widget((&mut *btn).as_widget_ptr());
                }
            }
        }
        if let Some(app) = self.app {
            unsafe {
                (*app).crazy_dave_die();
            }
        }
    }

    /// 对应 C++ StoreScreen::ButtonDepress
    pub fn button_depress(&mut self, the_id: i32) {
        if the_id == STORESCREEN_BACK {
            self.result = 1000;
        } else if the_id == STORESCREEN_PREV || the_id == STORESCREEN_NEXT {
            self.hatch_timer = 50;
            if let Some(app) = self.app {
                unsafe {
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundHatchbackClose as i32);
                }
            }
            self.bubble_count_down = 0;
            if let Some(app) = self.app {
                unsafe {
                    (*app).crazy_dave_stop_talking();
                }
            }
            self.enable_buttons(false);
            // do-while 翻页到第一个可见页
            loop {
                if the_id == STORESCREEN_PREV {
                    self.page = unsafe { std::mem::transmute::<i32, StorePages>(self.page as i32 - 1) };
                    if (self.page as i32) < (StorePages::SlotUpgrades as i32) {
                        self.page = StorePages::Zen2;
                    }
                } else {
                    self.page = unsafe { std::mem::transmute::<i32, StorePages>(self.page as i32 + 1) };
                    if (self.page as i32) >= (StorePages::NumPages as i32) {
                        self.page = StorePages::SlotUpgrades;
                    }
                }
                if self.is_page_shown(self.page) {
                    break;
                }
            }
        }
    }

    /// 对应 C++ StoreScreen::KeyDown
    pub fn key_down(&mut self, the_key: KeyCode) {
        if the_key == KEYCODE_ESCAPE {
            self.button_depress(STORESCREEN_BACK);
            return;
        }

        if self.bubble_click_to_continue && (the_key == KEYCODE_SPACE || the_key == KEYCODE_RETURN) {
            self.advance_crazy_dave_dialog();
            return;
        }

        // C++: Dialog::KeyDown(theKey) —— Rust 侧 Dialog 无按键处理，等效空实现
    }

    /// 对应 C++ StoreScreen::PurchaseItem
    pub fn purchase_item(&mut self, the_store_item: StoreItem) {
        if let Some(app) = self.app {
            unsafe {
                (*app).base.set_cursor(CURSOR_POINTER);
            }
        }
        self.bubble_count_down = 0;
        if let Some(app) = self.app {
            unsafe {
                (*app).crazy_dave_stop_talking();
            }
        }
        if !self.can_afford_item(the_store_item) {
            // the localization key names for this dialog are wrong（C++ 注释）
            let a_dialog = if let Some(app) = self.app {
                unsafe {
                    (*app).do_dialog(
                        DIALOG_NOT_ENOUGH_MONEY,
                        true,
                        "Not enough money",
                        "You can't afford this item yet. Earn more coins by killing zombies!",
                        "[DIALOG_BUTTON_OK]",
                        BUTTONS_FOOTER,
                    )
                }
            } else {
                None
            };
            self.wait_for_dialog = true;
            if let Some(d) = a_dialog {
                unsafe {
                    (&mut *d).wait_for_result(true);
                }
            }
            self.wait_for_dialog = false;
        } else {
            let a_confirm_dialog = if let Some(app) = self.app {
                unsafe {
                    (*app).do_dialog(
                        DIALOG_STORE_PURCHASE,
                        true,
                        "Buy this item?",
                        "Are you sure you want to buy this item?",
                        "",
                        BUTTONS_YES_NO,
                    )
                }
            } else {
                None
            };
            // [TRANSLATION_NOTE]: C++ (LawnDialog*)aComfirmDialog->mLawnYesButton->SetLabel(...)；
            // Rust do_dialog 返回 *mut Dialog 而非 LawnDialog，按钮标签用对话框默认值，暂跳过。

            self.wait_for_dialog = true;
            let a_confirm_result = a_confirm_dialog.map_or(0, |d| unsafe { (&mut *d).wait_for_result(true) });
            self.wait_for_dialog = false;

            if a_confirm_result == ID_OK {
                if let Some(app) = self.app {
                    unsafe {
                        let app_ref = &mut *app;
                        // [TRANSLATION_NOTE]: C++ 在 STINKY_THE_SNAIL 分支内调用 mApp->GetNowTime()；
                        // Rust 借用检查要求先于 a_player 可变借用求值（GetNowTime 无副作用，求值时机
                        // 提前至同一 tick 内，行为等价）。
                        let a_now_time = app_ref.get_now_time() as i32;
                        let a_player = app_ref.player_info.as_mut().unwrap();
                        a_player.add_coins(-Self::get_item_cost(the_store_item));

                        if the_store_item == StoreItem::PacketUpgrade {
                            a_player.m_purchases[StoreItem::PacketUpgrade as usize] += 1;
                            let a_dialog_lines = format!(
                                "Now you can choose to take {} seeds with you per level!",
                                6 + a_player.m_purchases[StoreItem::PacketUpgrade as usize]
                            );
                            // C++: StrFormat(GetString("NOW_YOU_CAN_CHOOSE_X_SEEDS", ...))
                            let a_dialog = app_ref.do_dialog(
                                DIALOG_UPGRADED,
                                true,
                                "More slots!",
                                &a_dialog_lines,
                                "[DIALOG_BUTTON_OK]",
                                BUTTONS_FOOTER,
                            );
                            self.wait_for_dialog = true;
                            if let Some(d) = a_dialog {
                                (&mut *d).wait_for_result(true);
                            }
                            self.wait_for_dialog = false;

                            // C++: mApp->mBoard->mSeedBank->UpdateWidth()
                            // [TRANSLATION_NOTE]: Rust 侧 seed_bank 为 Vec<SeedPacket>（架构不同），
                            // 对应语义：刷新种子槽数量与各槽 x 位置。
                            if let Some(board) = app_ref.board {
                                let a_num_packets = (*board).get_num_seeds_in_bank();
                                let _ = a_num_packets;
                                let bank_len = (*board).seed_bank.len();
                                let mut a_packet_xs: Vec<i32> = Vec::with_capacity(bank_len);
                                for i in 0..bank_len {
                                    a_packet_xs.push((*board).get_seed_packet_position_x(i as i32));
                                }
                                for i in 0..bank_len {
                                    // [TRANSLATION_NOTE]: C++ mSeedPackets[i].mX = ...；nightly 要求
                                    // 裸指针写操作显式解引用
                                    unsafe { (&mut *board).seed_bank[i].x = a_packet_xs[i]; }
                                }
                            }
                        } else if the_store_item == StoreItem::BonusLawnMower {
                            a_player.m_purchases[StoreItem::BonusLawnMower as usize] += 1;
                        } else if the_store_item == StoreItem::Rake {
                            a_player.m_purchases[StoreItem::Rake as usize] = 3;
                        } else if the_store_item == StoreItem::StinkyTheSnail {
                            let mut a_time = a_now_time;
                            if a_time == 0 {
                                a_time = 1;
                            }
                            a_player.m_purchases[StoreItem::StinkyTheSnail as usize] = a_time;
                        } else if the_store_item == StoreItem::Fertilizer || the_store_item == StoreItem::BugSpray {
                            if a_player.m_purchases[the_store_item as usize] < PURCHASE_COUNT_OFFSET {
                                a_player.m_purchases[the_store_item as usize] = PURCHASE_COUNT_OFFSET;
                            }
                            a_player.m_purchases[the_store_item as usize] += 5;
                        } else if the_store_item == StoreItem::TreeFood {
                            if a_player.m_purchases[StoreItem::TreeFood as usize] < PURCHASE_COUNT_OFFSET {
                                a_player.m_purchases[StoreItem::TreeFood as usize] = PURCHASE_COUNT_OFFSET;
                            }
                            a_player.m_purchases[StoreItem::TreeFood as usize] += 1;
                        } else if the_store_item == StoreItem::TreeOfWisdom {
                            a_player.m_purchases[StoreItem::TreeOfWisdom as usize] = 1;
                            // C++: mChallengeRecords[GAMEMODE_TREE_OF_WISDOM - GAMEMODE_SURVIVAL_NORMAL_STAGE_1] = 1
                            let a_record_index =
                                (GameMode::ChallengeTreeOfWisdom as i32 - GameMode::SurvivalNormalStage1 as i32) as usize;
                            if a_record_index < a_player.m_challenge_records.len() {
                                a_player.m_challenge_records[a_record_index] = 1;
                            }

                            let a_visit_dialog = app_ref.do_dialog(
                                DIALOG_STORE_PURCHASE,
                                true,
                                "[VISIT_TREE_HEADER]",
                                "[VISIT_TREE_BODY]",
                                "",
                                BUTTONS_YES_NO,
                            );
                            // [TRANSLATION_NOTE]: C++ 中 mLawnYesButton/mLawnNoButton SetLabel 跳过（同上）
                            self.wait_for_dialog = true;
                            let a_result = a_visit_dialog.map_or(0, |d| unsafe { (&mut *d).wait_for_result(true) });
                            self.wait_for_dialog = false;

                            if a_result == ID_OK {
                                self.go_to_tree_now = true;
                                self.result = a_result;
                            }
                        } else if Self::is_potted_plant(the_store_item) {
                            if let Some(zg) = app_ref.zen_garden {
                                (*zg).add_potted_plant(&mut self.potted_plant_specs);
                            }
                            self.potted_plant_specs.initialize_potted_plant(SeedType::Marigold);
                            self.potted_plant_specs.draw_variation = std::mem::transmute::<i32, DrawVariation>(
                                rand_range_int(
                                    DrawVariation::MarigoldWhite as i32,
                                    DrawVariation::MarigoldLightGreen as i32,
                                ),
                            );
                            a_player.m_purchases[the_store_item as usize] = get_current_days_since_2000();
                        } else {
                            // C++: PVZP_ASSERT(theStoreItem >= STORE_ITEM_PLANT_GATLINGPEA && theStoreItem < (StoreItem)MAX_PURCHASES)
                            a_player.m_purchases[the_store_item as usize] = 1;
                        }

                        if the_store_item == StoreItem::Firstaid {
                            self.set_bubble_text(3400, 800, false);
                        }

                        if let Some(seed_chooser) = app_ref.seed_chooser_screen {
                            (*(seed_chooser as *mut SeedChooserScreen)).update_after_purchase();
                        }

                        // Only give the achievement if the player bought a plant and has all plants purchased
                        // [TRANSLATION_NOTE]: C++ 用枚举比较 theStoreItem >= STORE_ITEM_PLANT_GATLINGPEA；
                        // Rust StoreItem 无 PartialOrd，改用底层整数值比较（repr(i32) 值序一致）。
                        let mut a_give_achievement = the_store_item as i32 >= StoreItem::PlantGatlingpea as i32
                            && the_store_item as i32 <= StoreItem::PlantImitater as i32;
                        if a_give_achievement {
                            for a_seed_type in (SeedType::Gatlingpea as i32)..=(SeedType::Imitater as i32) {
                                if !app_ref.has_seed_type(unsafe { std::mem::transmute::<i32, SeedType>(a_seed_type) }) {
                                    a_give_achievement = false;
                                }
                            }
                        }

                        if a_give_achievement {
                            ReportAchievement::give_achievement(
                                Some(app as *mut crate::lawn::lawn_app::LawnApp),
                                AchievementId::Morticulturalist as i32,
                                a_give_achievement,
                            );
                            self.set_bubble_text(4000, 800, false);
                        }

                        app_ref.write_current_user_config();
                    }
                }
            }
        }
    }

    /// 对应 C++ StoreScreen::AdvanceCrazyDaveDialog
    pub fn advance_crazy_dave_dialog(&mut self) {
        if !self.bubble_click_to_continue {
            return;
        }

        // "Hey neighbor! I've got some new things to sell!"
        if self.app.map_or(false, |app| unsafe { (*app).m_crazy_dave_message_index == 3100 }) {
            self.hatch_timer = 150;
            self.hatch_open = true;
            if let Some(app) = self.app {
                unsafe {
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundHatchbackOpen as i32);
                }
            }
        }
        let a_advanced = self.app.map_or(true, |app| unsafe { (*app).advance_crazy_dave_text() });
        if !a_advanced {
            if let Some(app) = self.app {
                unsafe {
                    (*app).crazy_dave_stop_talking();
                }
            }
            self.bubble_click_to_continue = false;
            self.bubble_count_down = 500;
            if self.hatch_timer == 0 {
                self.enable_buttons(true);
            }
        } else {
            let a_message_index = self.app.map_or(0, |app| unsafe { (*app).m_crazy_dave_message_index });
            self.set_bubble_text(a_message_index, 0, true);
        }

        let a_message = self.app.map_or(0, |app| unsafe { (*app).m_crazy_dave_message_index });
        if a_message == 303 || a_message == 606 || a_message == 2601 {
            self.hatch_timer = 150;
            self.hatch_open = true;
            if let Some(app) = self.app {
                unsafe {
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundHatchbackOpen as i32);
                }
            }
        } else if a_message == 603 {
            if let Some(app) = self.app {
                unsafe {
                    (*app).player_info.as_mut().unwrap().m_needs_magic_taco_reward = false;
                    (*app).write_current_user_config();
                    (*app).play_sample(crate::framework::resources::ResourceId::SoundDiamond as i32);
                }
            }
            self.coins.push(Coin::new());
            let a_coin = self.coins.last_mut().unwrap();
            a_coin.coin_initialize(80.0, 520.0, CoinType::Diamond, CoinMotion::FromPresent);
            a_coin.vel_x = 0.0;
            a_coin.vel_y = -5.0;
        } else if a_message == 902 || a_message == 1002 {
            if let Some(app) = self.app {
                unsafe {
                    (*app).player_info.as_mut().unwrap().add_coins(100);
                }
            }
        }
    }

    /// 对应 C++ StoreScreen::MouseDown
    pub fn mouse_down(&mut self, x: i32, y: i32, _the_click_count: i32) {
        if self.bubble_click_to_continue {
            self.advance_crazy_dave_dialog();
            return;
        }
        if !self.can_interact_with_buttons() {
            return;
        }
        for a_item_pos in 0..MAX_PAGE_SPOTS {
            let a_item_type = self.get_store_item_type(a_item_pos as i32);
            if a_item_type == StoreItem::Invalid {
                continue;
            }
            let mut a_item_x = 0;
            let mut a_item_y = 0;
            Self::get_store_position(a_item_pos as i32, &mut a_item_x, &mut a_item_y);
            // C++: Rect(aItemX, aItemY, 50, 87).Contains(x, y)
            if (a_item_x..a_item_x + 50).contains(&x) && (a_item_y..a_item_y + 87).contains(&y) {
                if self.is_full_version_only(a_item_type) {
                    self.wait_for_dialog = true;
                    if let Some(app) = self.app {
                        unsafe {
                            // C++: mApp->LawnMessageBox(DIALOG_MESSAGE, "[GET_FULL_VERSION_TITLE]", "[FULL_VERSION_TO_BUY]", "[DIALOG_BUTTON_OK]", "", BUTTONS_FOOTER)
                            let a_dialog = (*app).do_dialog(
                                DIALOG_MESSAGE,
                                true,
                                "[GET_FULL_VERSION_TITLE]",
                                "[FULL_VERSION_TO_BUY]",
                                "[DIALOG_BUTTON_OK]",
                                BUTTONS_FOOTER,
                            );
                            if let Some(d) = a_dialog {
                                (&mut *d).wait_for_result(true);
                            }
                        }
                    }
                    self.wait_for_dialog = false;
                } else if a_item_type == StoreItem::Pvz {
                    self.wait_for_dialog = true;
                    if let Some(app) = self.app {
                        unsafe {
                            // C++: LawnMessageBox(BUY_PVZ_TITLE/BUY_PVZ_BODY, GET_FULL_VERSION_YES/NO_BUTTON, BUTTONS_YES_NO)
                            let a_dialog = (*app).do_dialog(
                                DIALOG_MESSAGE,
                                true,
                                "[BUY_PVZ_TITLE]",
                                "[BUY_PVZ_BODY]",
                                "",
                                BUTTONS_YES_NO,
                            );
                            if let Some(d) = a_dialog {
                                let _a_result = (&mut *d).wait_for_result(true);
                            }
                        }
                    }
                    self.wait_for_dialog = false;
                } else if !self.is_item_sold_out(a_item_type)
                    && !self.is_item_unavailable(a_item_type)
                    && !self.is_coming_soon(a_item_type)
                {
                    self.purchase_item(a_item_type);
                }
                break;
            }
        }
    }
}

/// 商店界面叠加 Widget（对应 C++ StoreScreenOverlay）
pub struct StoreScreenOverlay {
    pub parent: Option<*mut StoreScreen>,
}

impl StoreScreenOverlay {
    pub fn new() -> Self {
        StoreScreenOverlay { parent: None }
    }
    pub fn draw(&self, g: &mut Graphics) {
        // 对应 C++ StoreScreenOverlay::Draw → mParent->DrawOverlay(g)
        if let Some(parent) = self.parent {
            unsafe {
                (*parent).draw_overlay(g);
            }
        }
    }
}

/// 商店界面 Widget 包装（对应 C++ StoreScreen 作为 Dialog 挂入 WidgetManager 的接入层）
/// [TRANSLATION_NOTE]: Rust 侧 StoreScreen 为独立结构体（非 Widget 体系），通过此包装
/// 注册进 WidgetManager，由主循环驱动 update/draw/鼠标/键盘事件。
pub struct StoreScreenImpl {
    /// 指向 LawnApp::store_screen 拥有（Box 持有）的 StoreScreen，非拥有指针
    pub store: *mut StoreScreen,
}

impl StoreScreenImpl {
    pub fn new(store: *mut StoreScreen) -> Self {
        StoreScreenImpl { store }
    }
}

impl WidgetImpl for StoreScreenImpl {
    fn update(&mut self, _widget: &mut Widget) {
        unsafe {
            (*self.store).update();
        }
    }

    fn draw(&mut self, _widget: &Widget, g: &mut Graphics) {
        unsafe {
            (*self.store).draw(g);
        }
    }

    fn draw_overlay(&mut self, _widget: &Widget, g: &mut Graphics) {
        unsafe {
            (*self.store).draw_overlay(g);
        }
    }

    fn key_down(&mut self, _widget: &mut Widget, key: KeyCode, _wm: &mut WidgetManager) {
        unsafe {
            (*self.store).key_down(key);
        }
    }

    fn mouse_down_btn(&mut self, _widget: &mut Widget, x: i32, y: i32, _btn: i32, click: i32) {
        // 对应 C++ StoreScreen::MouseDown(x, y, theClickCount)
        unsafe {
            (*self.store).mouse_down(x, y, click);
        }
    }

    fn mouse_move(&mut self, _widget: &mut Widget, _x: i32, _y: i32) {
        // C++ 中悬停提示状态由 Update 内 UpdateMouse() 每帧驱动（非 MouseMove 事件），无事件处理
    }
}