// PvZ Portable Rust 翻译 — ZombatarWidget（僵尸头像定制界面）
// 对应 C++ src/Lawn/Widget/ZombatarWidget.h / ZombatarWidget.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::font::Font;
use crate::framework::graphics::image::Image;
use crate::framework::color::Color;
use crate::framework::key_codes::{KEYCODE_ESCAPE, KeyCode};
use crate::framework::rect::Rect;
use crate::framework::widget::dialog::{BUTTONS_YES_NO, ID_YES};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::*;
use crate::lawn::zombatar::{
    get_part_layout, zombatar_get_color, zombatar_read_signed_record_slot,
    zombatar_read_record_slot, ZombatarPage,
};
use crate::lawn::widget::game_selector::GameSelectorImpl;
use crate::todlib::tod_common::tod_animate_curve;

/// 对应 C++ ZombatarWidget.h 按钮 ID 枚举
pub const ZOMBATAR_BTN_BACK: i32 = 300;
pub const ZOMBATAR_BTN_VIEW: i32 = 301;
pub const ZOMBATAR_BTN_FINISHED: i32 = 302;
pub const ZOMBATAR_BTN_NEW: i32 = 303;
pub const ZOMBATAR_BTN_CONFIRM_BACK: i32 = 304;
pub const ZOMBATAR_BTN_PREV_PORTRAIT: i32 = 305;
pub const ZOMBATAR_BTN_NEXT_PORTRAIT: i32 = 306;
pub const ZOMBATAR_BTN_PREV_PAGE: i32 = 307;
pub const ZOMBATAR_BTN_NEXT_PAGE: i32 = 308;
/// 可制作的最大僵尸头像数（对应 C++ PlayerInfo.h MAX_ZOMBATAR_HEADS=100）
pub const MAX_ZOMBATAR_HEADS: i32 = 100;
/// 单条记录字节数（对应 C++ ZOMBATAR_RECORD_SIZE = 0x48）
pub const ZOMBATAR_RECORD_SIZE: usize = 0x48;
/// 网格每页项数（对应 C++ ZOMBATAR_GRID_PAGE = 17）
pub const ZOMBATAR_GRID_PAGE: i32 = 17;
/// 状态切换动画时长（对应 C++ ZOMBATAR_TRANSITION_TICKS = 90）
pub const ZOMBATAR_TRANSITION_TICKS: i32 = 90;
/// 每页部件数（对应 C++ ZOMBATAR_ITEMS_PER_PAGE，按 ZombatarPage 顺序）
pub const ZOMBATAR_ITEMS_PER_PAGE: [i32; 9] = [0, 16, 24, 14, 16, 12, 15, 14, 5];
/// 布局常量（对应 C++ ZombatarWidget.cpp 顶部 constexpr）
pub const ZOMBATAR_GRID_GAP: i32 = -4;
pub const ZOMBATAR_GRID_BIAS_X: i32 = 50;
pub const ZOMBATAR_CELL_INSET: i32 = 9;
pub const ZOMBATAR_GRID_COLS: i32 = 6;
pub const ZOMBATAR_PANEL_X: i32 = 25;
pub const ZOMBATAR_PANEL_Y: i32 = 25;
pub const ZOMBATAR_PANEL_WIDTH: i32 = 560;
pub const ZOMBATAR_PREVIEW_X: i32 = 592;
pub const ZOMBATAR_PREVIEW_Y: i32 = 115;
pub const ZOMBATAR_TABS_X: i32 = 58;
pub const ZOMBATAR_TABS_Y0: i32 = 128;
pub const ZOMBATAR_COLOR_COLS: i32 = 9;
pub const ZOMBATAR_COLOR_GAP: i32 = 4;
pub const ZOMBATAR_COLOR_X: i32 = 238;
pub const ZOMBATAR_COLOR_Y: i32 = 367;
//
pub const ZOMBATAR_COLOR_NONE: i32 = -1;
pub const ZOMBATAR_SKIN_COLOR_COUNT: i32 = 12;
pub const ZOMBATAR_PART_COLOR_COUNT: i32 = 18;
pub const ZOMBATAR_PART_COLOR_BASE: i32 = 12;
pub const ZOMBATAR_PART_COLOR_BASE_2: i32 = 30;
pub const ZOMBATAR_PART_COLOR_NONE_1: i32 = 29;
pub const ZOMBATAR_PART_COLOR_NONE_2: i32 = 47;
#[allow(non_upper_case_globals)]
pub const ZOMBATAR_COLOR_MODE_SKIN: i32 = 0;
#[allow(non_upper_case_globals)]
pub const ZOMBATAR_COLOR_MODE_1: i32 = 1;
#[allow(non_upper_case_globals)]
pub const ZOMBATAR_COLOR_MODE_2: i32 = 2;
#[allow(non_upper_case_globals)]
pub const ZOMBATAR_COLOR_MODE_NONE: i32 = 4;

/// 定制界面状态（对应 C++ ZombatarWidgetState）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombatarWidgetState {
    List = 0,
    Create,
    Confirm,
    ToConfirm,
    FromConfirm,
}

/// 页面索引助手（对应 C++ ZombatarPage 枚举序）
pub const ZOMBATAR_PAGE_SKIN: usize = 0;
pub const ZOMBATAR_PAGE_HAIR: usize = 1;
pub const ZOMBATAR_PAGE_FACIAL_HAIR: usize = 2;
pub const ZOMBATAR_PAGE_TIDBITS: usize = 3;
pub const ZOMBATAR_PAGE_EYEWEAR: usize = 4;
pub const ZOMBATAR_PAGE_CLOTHES: usize = 5;
pub const ZOMBATAR_PAGE_ACCESSORY: usize = 6;
pub const ZOMBATAR_PAGE_HATS: usize = 7;
pub const ZOMBATAR_PAGE_BACKDROPS: usize = 8;

/// 记录槽位（对应 C++ ZombatarRecordSlot）
pub const ZOMBATAR_SLOT_SKIN_PART: i32 = 0;
pub const ZOMBATAR_SLOT_SKIN_COLOR: i32 = 1;
pub const ZOMBATAR_SLOT_CLOTHES: i32 = 2;
pub const ZOMBATAR_SLOT_CLOTHES_COLOR: i32 = 3;
pub const ZOMBATAR_SLOT_TIDBITS: i32 = 4;
pub const ZOMBATAR_SLOT_TIDBITS_COLOR: i32 = 5;
pub const ZOMBATAR_SLOT_ACCESSORY: i32 = 6;
pub const ZOMBATAR_SLOT_ACCESSORY_COLOR: i32 = 7;
pub const ZOMBATAR_SLOT_FACIAL_HAIR: i32 = 8;
pub const ZOMBATAR_SLOT_FACIAL_HAIR_COLOR: i32 = 9;
pub const ZOMBATAR_SLOT_HAIR: i32 = 10;
pub const ZOMBATAR_SLOT_HAIR_COLOR: i32 = 11;
pub const ZOMBATAR_SLOT_EYEWEAR: i32 = 12;
pub const ZOMBATAR_SLOT_EYEWEAR_COLOR: i32 = 13;
pub const ZOMBATAR_SLOT_HATS: i32 = 14;
pub const ZOMBATAR_SLOT_HATS_COLOR: i32 = 15;
pub const ZOMBATAR_SLOT_BACKGROUND: i32 = 16;
pub const ZOMBATAR_SLOT_BACKGROUND_COLOR: i32 = 17;

/// 部件槽位（对应 C++ SlotForPart 表）
pub fn slot_for_part(the_page: ZombatarPage) -> i32 {
    match the_page {
        ZombatarPage::Skin => ZOMBATAR_SLOT_SKIN_PART,
        ZombatarPage::Clothes => ZOMBATAR_SLOT_CLOTHES,
        ZombatarPage::Tidbits => ZOMBATAR_SLOT_TIDBITS,
        ZombatarPage::Accessory => ZOMBATAR_SLOT_ACCESSORY,
        ZombatarPage::FacialHair => ZOMBATAR_SLOT_FACIAL_HAIR,
        ZombatarPage::Hair => ZOMBATAR_SLOT_HAIR,
        ZombatarPage::Eyewear => ZOMBATAR_SLOT_EYEWEAR,
        ZombatarPage::Hats => ZOMBATAR_SLOT_HATS,
        ZombatarPage::Backdrops => ZOMBATAR_SLOT_BACKGROUND,
    }
}

/// 颜色槽位（对应 C++ SlotForColor 表）
pub fn slot_for_color(the_page: ZombatarPage) -> i32 {
    match the_page {
        ZombatarPage::Skin => ZOMBATAR_SLOT_SKIN_COLOR,
        ZombatarPage::Clothes => ZOMBATAR_SLOT_CLOTHES_COLOR,
        ZombatarPage::Tidbits => ZOMBATAR_SLOT_TIDBITS_COLOR,
        ZombatarPage::Accessory => ZOMBATAR_SLOT_ACCESSORY_COLOR,
        ZombatarPage::FacialHair => ZOMBATAR_SLOT_FACIAL_HAIR_COLOR,
        ZombatarPage::Hair => ZOMBATAR_SLOT_HAIR_COLOR,
        ZombatarPage::Eyewear => ZOMBATAR_SLOT_EYEWEAR_COLOR,
        ZombatarPage::Hats => ZOMBATAR_SLOT_HATS_COLOR,
        ZombatarPage::Backdrops => ZOMBATAR_SLOT_BACKGROUND_COLOR,
    }
}

/// 部件颜色模式（对应 C++ GetPartColorMode 表）
pub fn get_part_color_mode(the_page: ZombatarPage, the_part_index: i32) -> i32 {
    match the_page {
        ZombatarPage::Skin => ZOMBATAR_COLOR_MODE_SKIN,
        ZombatarPage::Clothes => ZOMBATAR_COLOR_MODE_NONE,
        ZombatarPage::FacialHair => ZOMBATAR_COLOR_MODE_1,
        ZombatarPage::Hair => {
            if the_part_index == 2 {
                ZOMBATAR_COLOR_MODE_NONE
            } else {
                ZOMBATAR_COLOR_MODE_1
            }
        }
        ZombatarPage::Backdrops => {
            if the_part_index == 4 {
                ZOMBATAR_COLOR_MODE_2
            } else {
                ZOMBATAR_COLOR_MODE_NONE
            }
        }
        ZombatarPage::Tidbits => match the_part_index {
            0 | 1 | 2 | 9 | 10 | 11 => ZOMBATAR_COLOR_MODE_2,
            _ => ZOMBATAR_COLOR_MODE_NONE,
        },
        ZombatarPage::Eyewear => {
            if the_part_index < 12 {
                ZOMBATAR_COLOR_MODE_2
            } else {
                ZOMBATAR_COLOR_MODE_NONE
            }
        }
        ZombatarPage::Accessory => match the_part_index {
            7 | 9 | 11 | 12 => ZOMBATAR_COLOR_MODE_2,
            _ => ZOMBATAR_COLOR_MODE_NONE,
        },
        ZombatarPage::Hats => {
            if the_part_index == 12 {
                ZOMBATAR_COLOR_MODE_NONE
            } else {
                ZOMBATAR_COLOR_MODE_2
            }
        }
    }
}

/// 颜色基数（对应 C++ ZombatarColorBaseForMode）
pub fn zombatar_color_base_for_mode(the_mode: i32) -> i32 {
    if the_mode == ZOMBATAR_COLOR_MODE_1 {
        ZOMBATAR_PART_COLOR_BASE
    } else {
        ZOMBATAR_PART_COLOR_BASE_2
    }
}

/// 每页总部件数（对应 C++ GetTotalItemsForPage）
pub fn get_total_items_for_page(the_page: ZombatarPage) -> i32 {
    let a_index = (the_page as i32).clamp(0, (ZOMBATAR_ITEMS_PER_PAGE.len() - 1) as i32);
    ZOMBATAR_ITEMS_PER_PAGE[a_index as usize]
}

/// 僵尸头像定制界面（对应 C++ ZombatarWidget）
pub struct ZombatarWidget {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    /// 对应 C++ mGameSelector（用于滑动返回选择器）
    pub game_selector: Option<*mut GameSelectorImpl>,
    pub state: ZombatarWidgetState,
    pub page: ZombatarPage,
    pub current_index: i32,
    pub sub_page: i32,
    pub max_sub_pages: i32,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub hover_grid_cell: i32,
    pub hover_color_cell: i32,
    pub hover_tab: i32,
    pub delete_hover: bool,
    pub transition_timer: i32,
    /// 对应 C++ mPart[NUM_ZOMBATAR_PAGES] / mColor[NUM_ZOMBATAR_PAGES]
    pub part: [i32; 9],
    pub color: [i32; 9],
    /// 对应 C++ mPreviewZombie（预览僵尸，Box 所有权；销毁时先 DieNoLoot）
    pub preview_zombie: Option<Box<crate::lawn::zombie::Zombie>>,
    /// 懒加载字体缓存（对应 C++ FONT_BRIANNETOD12 等全局字体，图片字库未接线）
    /// [0]=Briannetod12 [1]=Dwarventodcraft12 [2]=Dwarventodcraft15 [3]=Houseofterror28 [4]=Continuumbold14
    pub fonts: [Option<Box<Font>>; 5],
}

impl ZombatarWidget {
    pub fn new(app: Option<*mut crate::lawn::lawn_app::LawnApp>, game_selector: Option<*mut GameSelectorImpl>) -> Self {
        // [TRANSLATION_NOTE]: C++ 构造函数创建 9 个 NewLawnButton（IMAGE_ZOMBATAR_* 皮肤图未接入，
        // 按钮暂以字段占位），并置 mPreviewZombie=nullptr。
        ZombatarWidget {
            app,
            game_selector,
            state: ZombatarWidgetState::List,
            page: ZombatarPage::Skin,
            current_index: 0,
            sub_page: 0,
            max_sub_pages: 0,
            mouse_x: 0,
            mouse_y: 0,
            hover_grid_cell: -1,
            hover_color_cell: -1,
            hover_tab: -1,
            delete_hover: false,
            transition_timer: 0,
            part: [-1; 9],
            color: [ZOMBATAR_COLOR_NONE; 9],
            preview_zombie: None,
            fonts: [None, None, None, None, None],
        }
    }

    /// 对应 C++ ZombatarWidget::GetHeadCount
    pub fn get_head_count(&self) -> i32 {
        self.app
            .and_then(|app| unsafe { (*app).player_info.as_ref() })
            .map_or(0, |pi| pi.m_zombatar_data.len() as i32 / ZOMBATAR_RECORD_SIZE as i32)
    }

    /// 对应 C++ ZombatarWidget::CanSaveNewHead
    pub fn can_save_new_head(&self) -> bool {
        self.get_head_count() < MAX_ZOMBATAR_HEADS
    }

    /// 对应 C++ ZombatarWidget::ClampCurrentIndex
    pub fn clamp_current_index(&mut self) {
        self.current_index = self.current_index.clamp(0, self.get_head_count() - 1);
    }

    /// 对应 C++ ZombatarWidget::ResetDraft
    pub fn reset_draft(&mut self) {
        for i in 0..9 {
            self.part[i] = -1;
            self.color[i] = ZOMBATAR_COLOR_NONE;
        }
        self.part[ZOMBATAR_PAGE_SKIN] = 0;
        self.color[ZOMBATAR_PAGE_SKIN] = 0;
        self.part[ZOMBATAR_PAGE_BACKDROPS] = 4;
        self.color[ZOMBATAR_PAGE_BACKDROPS] = ZOMBATAR_PART_COLOR_NONE_2;
    }

    /// 对应 C++ ZombatarWidget::Open
    pub fn open(&mut self) {
        if self.app.map_or(true, |app| unsafe { (*app).player_info.is_none() }) {
            return;
        }
        self.sub_page = 0;
        self.max_sub_pages = 0;
        self.page = ZombatarPage::Skin;
        self.hover_grid_cell = -1;
        self.hover_color_cell = -1;
        self.delete_hover = false;
        self.current_index = 0;
        self.clamp_current_index();

        if self.get_head_count() > 0 {
            self.change_state(ZombatarWidgetState::List);
            self.load_current_to_draft();
        } else {
            self.change_state(ZombatarWidgetState::Create);
            self.reset_draft();
        }

        // C++: mGameSelector->SlideTo(-BOARD_WIDTH, 0)
        if let Some(gs) = self.game_selector {
            unsafe {
                (*gs).slide_to(-crate::lawn::game_enums::BOARD_WIDTH, 0);
            }
        }
        // C++: mWidgetManager->BringToFront(this) + SetFocus(this) —— Rust 侧占位
    }

    /// 对应 C++ ZombatarWidget::BackToSelector
    pub fn back_to_selector(&mut self) {
        self.page = ZombatarPage::Skin;
        self.sub_page = 0;
        self.change_state(if self.get_head_count() > 0 {
            ZombatarWidgetState::List
        } else {
            ZombatarWidgetState::Create
        });
        self.reset_draft();
        if let Some(gs) = self.game_selector {
            unsafe {
                (*gs).slide_to(0, 0);
            }
        }
    }

    /// 对应 C++ ZombatarWidget::ChangeState
    pub fn change_state(&mut self, the_state: ZombatarWidgetState) {
        self.state = the_state;
        if the_state == ZombatarWidgetState::ToConfirm || the_state == ZombatarWidgetState::FromConfirm {
            self.transition_timer = ZOMBATAR_TRANSITION_TICKS;
        } else {
            self.hover_grid_cell = -1;
            self.hover_color_cell = -1;
            self.hover_tab = -1;
            self.delete_hover = false;
        }
        self.update_button_state();
    }

    /// 对应 C++ ZombatarWidget::ChangePage
    pub fn change_page(&mut self, the_page: ZombatarPage) {
        self.page = the_page;
        self.sub_page = 0;
        self.hover_grid_cell = -1;
        self.hover_color_cell = -1;
        self.update_button_state();
    }

    /// 对应 C++ ZombatarWidget::UpdateButtonState —— [TRANSLATION_NOTE]: 按钮未创建（图片未接入），占位

    /// 对应 C++ ZombatarWidget::ShowMaxHeadsMessage（ZombatarWidget.cpp）：
    /// ```cpp
    /// void ZombatarWidget::ShowMaxHeadsMessage()
    /// {
    ///     mApp->LawnMessageBox(DIALOG_MESSAGE, "Zombatar Limit Reached",
    ///         "This profile already has the maximum number of saved Zombatars.",
    ///         "[DIALOG_BUTTON_OK]", "", Dialog::BUTTONS_FOOTER);
    /// }
    /// ```
    ///
    /// [TRANSLATION_NOTE]: C++ LawnMessageBox 是 LawnDialog 快捷构造函数（创建模态对话框
    /// 并等待返回）。Rust LawnApp 无 LawnMessageBox 方法，此处用 do_dialog 等价替代。
    pub fn show_max_heads_message(&mut self) {
        let Some(app) = self.app else { return };
        unsafe {
            (*app).do_dialog(
                crate::lawn::game_enums::Dialogs::Message as i32,
                true,
                "Zombatar Limit Reached",
                "This profile already has the maximum number of saved Zombatars.",
                "",
                crate::framework::widget::dialog::BUTTONS_FOOTER,
            );
        }
    }

    /// 对应 C++ ZombatarWidget::KeyDown
    pub fn key_down(&mut self, the_key: KeyCode) {
        if the_key != KEYCODE_ESCAPE {
            return;
        }
        match self.state {
            ZombatarWidgetState::ToConfirm => {
                self.state = ZombatarWidgetState::FromConfirm;
                self.transition_timer = ZOMBATAR_TRANSITION_TICKS - self.transition_timer;
            }
            ZombatarWidgetState::Confirm => {
                self.change_state(ZombatarWidgetState::FromConfirm);
            }
            _ => {
                self.back_to_selector();
            }
        }
    }

    /// 对应 C++ ZombatarWidget::ButtonDepress
    pub fn button_depress(&mut self, the_id: i32) {
        if self.app.map_or(true, |app| unsafe { (*app).player_info.is_none() }) {
            return;
        }
        match the_id {
            ZOMBATAR_BTN_BACK => self.back_to_selector(),
            ZOMBATAR_BTN_CONFIRM_BACK => {
                if self.state == ZombatarWidgetState::Confirm {
                    self.change_state(ZombatarWidgetState::FromConfirm);
                }
            }
            ZOMBATAR_BTN_VIEW => {
                self.reset_draft();
                self.change_state(ZombatarWidgetState::List);
            }
            ZOMBATAR_BTN_FINISHED => {
                if self.state == ZombatarWidgetState::Create {
                    if self.can_save_new_head() {
                        self.change_state(ZombatarWidgetState::ToConfirm);
                    } else {
                        self.show_max_heads_message();
                    }
                } else if self.state == ZombatarWidgetState::Confirm {
                    if self.save_draft() {
                        self.change_state(ZombatarWidgetState::List);
                    }
                }
            }
            ZOMBATAR_BTN_NEW => {
                if self.can_save_new_head() {
                    self.reset_draft();
                    self.page = ZombatarPage::Skin;
                    self.sub_page = 0;
                    self.change_state(ZombatarWidgetState::Create);
                } else {
                    self.show_max_heads_message();
                }
            }
            ZOMBATAR_BTN_PREV_PORTRAIT => {
                if self.current_index > 0 {
                    self.current_index -= 1;
                    self.load_current_to_draft();
                    self.update_button_state();
                }
            }
            ZOMBATAR_BTN_NEXT_PORTRAIT => {
                if self.current_index + 1 < self.get_head_count() {
                    self.current_index += 1;
                    self.load_current_to_draft();
                    self.update_button_state();
                }
            }
            ZOMBATAR_BTN_PREV_PAGE => {
                if self.sub_page > 0 {
                    self.sub_page -= 1;
                    self.hover_grid_cell = -1;
                    self.update_button_state();
                }
            }
            ZOMBATAR_BTN_NEXT_PAGE => {
                if self.sub_page < self.max_sub_pages {
                    self.sub_page += 1;
                    self.hover_grid_cell = -1;
                    self.update_button_state();
                }
            }
            _ => {}
        }
    }

    /// 对应 C++ ZombatarWidget::HandleGridClick
    pub fn handle_grid_click(&mut self, the_x: i32, the_y: i32) {
        if self.state != ZombatarWidgetState::Create {
            return;
        }
        let a_item_count = self.get_sub_page_item_count();
        let a_base_index = self.sub_page * ZOMBATAR_GRID_PAGE;
        for i in 0..a_item_count {
            if self.get_item_hit_rect(i).contains(the_x, the_y) {
                let mut a_part_index = a_base_index + i;
                if a_part_index > 16 {
                    a_part_index += a_part_index / 17;
                }
                self.part[self.page as usize] = a_part_index;
                if get_part_color_mode(self.page, a_part_index) == ZOMBATAR_COLOR_MODE_NONE {
                    self.color[self.page as usize] = ZOMBATAR_COLOR_NONE;
                }
                self.update_button_state();
                return;
            }
        }
        if self.page_allows_none() && self.get_item_hit_rect(a_item_count).contains(the_x, the_y) {
            self.part[self.page as usize] = -1;
            self.color[self.page as usize] = ZOMBATAR_COLOR_NONE;
            self.update_button_state();
        }
    }

    /// 对应 C++ ZombatarWidget::HandleColorClick
    pub fn handle_color_click(&mut self, the_x: i32, the_y: i32) {
        if self.state != ZombatarWidgetState::Create || !self.page_allows_colors() {
            return;
        }
        if self.page == ZombatarPage::Skin {
            for i in 0..ZOMBATAR_SKIN_COLOR_COUNT {
                if self.get_color_rect(i).contains(the_x, the_y) {
                    self.color[ZOMBATAR_PAGE_SKIN] = i;
                    return;
                }
            }
            return;
        }
        let a_mode = get_part_color_mode(self.page, self.part[self.page as usize]);
        let a_base = zombatar_color_base_for_mode(a_mode);
        for i in 0..ZOMBATAR_PART_COLOR_COUNT {
            if self.get_color_rect(i).contains(the_x, the_y) {
                self.color[self.page as usize] = a_base + i;
                return;
            }
        }
    }

    /// 对应 C++ ZombatarWidget::LoadCurrentToDraft
    pub fn load_current_to_draft(&mut self) {
        let a_record = self
            .app
            .and_then(|app| unsafe { (*app).player_info.as_ref() })
            .map(|pi| pi.m_zombatar_data.clone());
        if self.get_head_count() <= 0 {
            self.reset_draft();
            return;
        }
        self.clamp_current_index();
        if let Some(a_data) = a_record {
            let a_offset = self.current_index as usize * ZOMBATAR_RECORD_SIZE;
            if a_offset + ZOMBATAR_RECORD_SIZE <= a_data.len() {
                self.decode_record(&a_data[a_offset..a_offset + ZOMBATAR_RECORD_SIZE]);
                return;
            }
        }
        self.reset_draft();
    }

    /// 对应 C++ ZombatarWidget::SaveDraft
    pub fn save_draft(&mut self) -> bool {
        let Some(app) = self.app else { return false };
        unsafe {
            let Some(a_player_info) = (*app).player_info.as_mut() else { return false };
            if !self.can_save_new_head() {
                return false;
            }
            let a_offset = a_player_info.m_zombatar_data.len();
            a_player_info.m_zombatar_data.resize(a_offset + ZOMBATAR_RECORD_SIZE, 0);
            let mut a_record = vec![0u8; ZOMBATAR_RECORD_SIZE];
            self.encode_record(&mut a_record);
            if a_offset + ZOMBATAR_RECORD_SIZE <= a_player_info.m_zombatar_data.len() {
                let dst = &mut a_player_info.m_zombatar_data[a_offset..a_offset + ZOMBATAR_RECORD_SIZE];
                dst.copy_from_slice(&a_record);
            }
            a_player_info.m_zombatar_head_count = self.get_head_count() as u32;
            self.current_index = (a_offset / ZOMBATAR_RECORD_SIZE) as i32;
            a_player_info.m_zombatar_created_before = 1;
        }
        if let Some(app) = self.app {
            unsafe {
                if let Some(pi) = (*app).player_info.as_ref() {
                    pi.save_details();
                }
            }
        }
        self.reset_draft();
        self.page = ZombatarPage::Skin;
        self.sub_page = 0;
        true
    }

    /// 对应 C++ ZombatarWidget::DeleteCurrent
    pub fn delete_current(&mut self) {
        let a_count = self.get_head_count();
        let Some(app) = self.app else { return };
        unsafe {
            let Some(a_player_info) = (*app).player_info.as_mut() else { return };
            if a_count <= 0 {
                return;
            }
            self.clamp_current_index();
            let a_offset = self.current_index as usize * ZOMBATAR_RECORD_SIZE;
            let a_tail_offset = a_offset + ZOMBATAR_RECORD_SIZE;
            // C++: memmove（删除当前记录）
            if a_tail_offset <= a_player_info.m_zombatar_data.len() {
                a_player_info.m_zombatar_data.drain(a_offset..a_tail_offset);
            }
            a_player_info.m_zombatar_head_count = self.get_head_count() as u32;
            self.clamp_current_index();
        }
        if self.get_head_count() > 0 {
            self.load_current_to_draft();
            self.change_state(ZombatarWidgetState::List);
        } else {
            self.reset_draft();
            self.change_state(ZombatarWidgetState::Create);
        }
        if let Some(app) = self.app {
            unsafe {
                if let Some(pi) = (*app).player_info.as_ref() {
                    pi.save_details();
                }
            }
        }
    }

    /// 写入记录槽位 LE u32（对应 C++ ZombatarWriteRecordSlot）
    fn write_record_slot(record: &mut [u8], the_slot: i32, the_value: i32) {
        let an_offset = (the_slot * 4) as usize;
        if an_offset + 4 <= record.len() {
            record[an_offset..an_offset + 4].copy_from_slice(&(the_value as u32).to_le_bytes());
        }
    }

    /// 对应 C++ ZombatarWidget::EncodeRecord
    pub fn encode_record(&self, the_record: &mut [u8]) {
        for b in the_record.iter_mut() {
            *b = 0;
        }
        Self::write_record_slot(the_record, ZOMBATAR_SLOT_SKIN_PART, ZOMBATAR_COLOR_NONE);
        Self::write_record_slot(
            the_record,
            ZOMBATAR_SLOT_SKIN_COLOR,
            self.color[ZOMBATAR_PAGE_SKIN].clamp(0, 11),
        );
        for i in ZOMBATAR_PAGE_HAIR..9 {
            let a_page = page_for_index(i);
            Self::write_record_slot(the_record, slot_for_part(a_page), self.part[i]);
            Self::write_record_slot(the_record, slot_for_color(a_page), self.color[i]);
        }
    }

    /// 对应 C++ ZombatarWidget::DecodeRecord
    pub fn decode_record(&mut self, the_record: &[u8]) {
        for i in 0..9 {
            self.part[i] = -1;
            self.color[i] = ZOMBATAR_COLOR_NONE;
        }
        self.part[ZOMBATAR_PAGE_SKIN] = 0;
        let a_skin_color = zombatar_read_signed_record_slot(the_record, ZOMBATAR_SLOT_SKIN_COLOR);
        self.color[ZOMBATAR_PAGE_SKIN] = if a_skin_color < 0 { 0 } else { a_skin_color.min(11) };

        for i in ZOMBATAR_PAGE_HAIR..9 {
            let a_page = page_for_index(i);
            let mut a_item_count = get_total_items_for_page(a_page);
            if a_item_count > 16 {
                a_item_count += a_item_count / 17;
            }
            let a_part = zombatar_read_signed_record_slot(the_record, slot_for_part(a_page));
            let a_color = zombatar_read_signed_record_slot(the_record, slot_for_color(a_page));
            if a_page == ZombatarPage::Backdrops {
                self.part[i] = a_part.clamp(0, a_item_count - 1);
            } else {
                self.part[i] = if a_part >= 0 && a_part < a_item_count { a_part } else { -1 };
            }
            self.color[i] = if a_color < 0 { ZOMBATAR_COLOR_NONE } else { a_color };
        }
    }

    // ------------------------------------------------------------------
    // 布局与绘制（依赖图片资源，当前以近似返回/占位；图片接入后回填）
    // ------------------------------------------------------------------

    /// 对应 C++ ZombatarWidget::GetSubPageItemCount
    pub fn get_sub_page_item_count(&self) -> i32 {
        let a_total = get_total_items_for_page(self.page);
        let a_remaining = a_total - self.sub_page * ZOMBATAR_GRID_PAGE;
        a_remaining.clamp(0, ZOMBATAR_GRID_PAGE)
    }

    /// 对应 C++ ZombatarWidget::PageAllowsColors
    pub fn page_allows_colors(&self) -> bool {
        if self.page == ZombatarPage::Skin {
            return true;
        }
        if self.part[self.page as usize] < 0 {
            return false;
        }
        let a_mode = get_part_color_mode(self.page, self.part[self.page as usize]);
        a_mode == ZOMBATAR_COLOR_MODE_1 || a_mode == ZOMBATAR_COLOR_MODE_2
    }

    /// 对应 C++ ZombatarWidget::PageAllowsNone
    pub fn page_allows_none(&self) -> bool {
        self.page != ZombatarPage::Skin && self.page != ZombatarPage::Backdrops
    }

    /// 对应 C++ ZombatarWidget::GetItemHitRect —— [TRANSLATION_NOTE]: 布局坐标依赖图片网格，暂返回空矩形
    pub fn get_item_hit_rect(&self, the_index: i32) -> Rect {
        let a_rect = self.get_item_rect(the_index);
        Rect::new(
            a_rect.x + ZOMBATAR_CELL_INSET,
            a_rect.y + ZOMBATAR_CELL_INSET,
            a_rect.width - 2 * ZOMBATAR_CELL_INSET,
            a_rect.height - 2 * ZOMBATAR_CELL_INSET,
        )
    }

    /// 对应 C++ ZombatarWidget::GetItemRect（图片未接入，用默认 cell 尺寸 67）
    pub fn get_item_rect(&self, the_index: i32) -> Rect {
        let a_cell_w = 67; // C++ IMAGE_ZOMBATAR_ACCESSORY_BG->mWidth
        let a_cell_h = 67; // C++ IMAGE_ZOMBATAR_ACCESSORY_BG->mHeight
        let a_step_x = a_cell_w + ZOMBATAR_GRID_GAP;
        let a_step_y = a_cell_h + 3 + ZOMBATAR_GRID_GAP;
        let a_origin_x = ZOMBATAR_PANEL_X + (ZOMBATAR_PANEL_WIDTH - ZOMBATAR_GRID_COLS * a_step_x) / 2 + ZOMBATAR_GRID_BIAS_X;
        let a_origin_y = ZOMBATAR_PANEL_Y + 2 * ZOMBATAR_GRID_GAP + 120;
        let a_col = the_index % ZOMBATAR_GRID_COLS;
        let a_row = the_index / ZOMBATAR_GRID_COLS;
        Rect::new(a_origin_x + a_col * a_step_x, a_origin_y + a_row * a_step_y, a_cell_w, a_cell_h)
    }

    /// 对应 C++ ZombatarWidget::GetCategoryRect
    pub fn get_category_rect(&self, the_index: i32) -> Rect {
        let a_tab_h = 39; // C++ GetCategoryImage 未接入默认
        let a_tab_w = 104;
        let a_step = (a_tab_h - 3).max(8);
        Rect::new(ZOMBATAR_TABS_X, ZOMBATAR_TABS_Y0 + the_index * a_step, a_tab_w, a_tab_h)
    }

    /// 对应 C++ ZombatarWidget::GetColorRect —— [TRANSLATION_NOTE]: 图片未接入，色块默认 21x20
    pub fn get_color_rect(&self, the_index: i32) -> Rect {
        let a_swatch_w = 21; // C++ IMAGE_ZOMBATAR_COLORPICKER->mWidth
        let a_swatch_h = 20; // C++ IMAGE_ZOMBATAR_COLORPICKER->mHeight
        let a_step_x = a_swatch_w + ZOMBATAR_COLOR_GAP;
        let a_step_y = a_swatch_h + ZOMBATAR_COLOR_GAP;
        let a_col = the_index % ZOMBATAR_COLOR_COLS;
        let a_row = the_index / ZOMBATAR_COLOR_COLS;
        Rect::new(ZOMBATAR_COLOR_X + a_col * a_step_x, ZOMBATAR_COLOR_Y + a_row * a_step_y, a_swatch_w, a_swatch_h)
    }

    /// 对应 C++ ZombatarWidget::GetBackgroundImage
    pub fn get_background_image(&self, the_index: i32) -> *mut Image {
        // C++: aImages[clamp(theIndex, 0, 4)]
        let a_key = match the_index.clamp(0, 4) {
            0 => "IMAGE_ZOMBATAR_BACKGROUND_CRAZYDAVE",
            1 => "IMAGE_ZOMBATAR_BACKGROUND_MENU",
            2 => "IMAGE_ZOMBATAR_BACKGROUND_MENU_DOS",
            3 => "IMAGE_ZOMBATAR_BACKGROUND_ROOF",
            _ => "IMAGE_ZOMBATAR_BACKGROUND_BLANK",
        };
        self.get_resource_image(a_key)
    }

    /// 对应 C++ ZombatarWidget::GetPartImage
    pub fn get_part_image(&self, the_page: ZombatarPage, the_index: i32) -> *mut Image {
        if the_index < 0 {
            return std::ptr::null_mut();
        }
        let a_key: Option<String> = match the_page {
            ZombatarPage::Clothes => {
                if the_index < 12 { Some(format!("IMAGE_ZOMBATAR_CLOTHES_{}", the_index + 1)) } else { None }
            }
            ZombatarPage::Hats => {
                if the_index < 14 { Some(format!("IMAGE_ZOMBATAR_HATS_{}", the_index + 1)) } else { None }
            }
            ZombatarPage::Hair => {
                if the_index < 16 { Some(format!("IMAGE_ZOMBATAR_HAIR_{}", the_index + 1)) } else { None }
            }
            ZombatarPage::Eyewear => {
                if the_index < 16 { Some(format!("IMAGE_ZOMBATAR_EYEWEAR_{}", the_index + 1)) } else { None }
            }
            ZombatarPage::FacialHair => {
                let mut a_idx = the_index;
                if a_idx > 16 {
                    a_idx -= a_idx / 17;
                }
                if a_idx < 24 { Some(format!("IMAGE_ZOMBATAR_FACIALHAIR_{}", a_idx + 1)) } else { None }
            }
            ZombatarPage::Tidbits => {
                if the_index < 14 { Some(format!("IMAGE_ZOMBATAR_TIDBITS_{}", the_index + 1)) } else { None }
            }
            ZombatarPage::Accessory => {
                // C++: aAccessory[] = {1,2,3,4,5,6,8,9,...,16}（无 7）
                if the_index < 15 {
                    let a_res_index = if the_index >= 6 { the_index + 2 } else { the_index + 1 };
                    Some(format!("IMAGE_ZOMBATAR_ACCESSORY_{}", a_res_index))
                } else { None }
            }
            _ => None,
        };
        match a_key {
            Some(ref k) => self.get_resource_image(k),
            None => std::ptr::null_mut(),
        }
    }

    /// 对应 C++ ZombatarWidget::GetPartMaskImage
    pub fn get_part_mask_image(&self, the_page: ZombatarPage, the_index: i32) -> *mut Image {
        if the_index < 0 {
            return std::ptr::null_mut();
        }
        // C++ 掩码表（0 = 该部件无掩码，只染本体图）
        const A_HATS_MASKS: [i32; 14] = [1, 0, 3, 0, 0, 6, 7, 8, 9, 0, 11, 0, 0, 0];
        const A_HAIR_MASKS: [i32; 16] = [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 11, 12, 13, 14, 15, 0];
        const A_EYEWEAR_MASKS: [i32; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0, 0, 0, 0];
        const A_FACIAL_MASKS: [i32; 24] =
            [1, 0, 0, 4, 0, 0, 0, 8, 9, 10, 11, 12, 0, 14, 15, 16, 0, 18, 0, 0, 21, 22, 23, 24];
        let a_mask_index: Option<i32> = match the_page {
            ZombatarPage::Hats => {
                if the_index < 14 { Some(A_HATS_MASKS[the_index as usize]) } else { None }
            }
            ZombatarPage::Hair => {
                if the_index < 16 { Some(A_HAIR_MASKS[the_index as usize]) } else { None }
            }
            ZombatarPage::Eyewear => {
                if the_index < 16 { Some(A_EYEWEAR_MASKS[the_index as usize]) } else { None }
            }
            ZombatarPage::FacialHair => {
                let mut a_idx = the_index;
                if a_idx > 16 {
                    a_idx -= a_idx / 17;
                }
                if a_idx < 24 { Some(A_FACIAL_MASKS[a_idx as usize]) } else { None }
            }
            _ => None,
        };
        let a_mask_index = a_mask_index.filter(|&v| v > 0);
        match (the_page, a_mask_index) {
            (ZombatarPage::Hats, Some(a_idx)) => {
                self.get_resource_image(&format!("IMAGE_ZOMBATAR_HATS_{}_MASK", a_idx))
            }
            (ZombatarPage::Hair, Some(a_idx)) => {
                self.get_resource_image(&format!("IMAGE_ZOMBATAR_HAIR_{}_MASK", a_idx))
            }
            (ZombatarPage::Eyewear, Some(a_idx)) => {
                self.get_resource_image(&format!("IMAGE_ZOMBATAR_EYEWEAR_{}_MASK", a_idx))
            }
            (ZombatarPage::FacialHair, Some(a_idx)) => {
                self.get_resource_image(&format!("IMAGE_ZOMBATAR_FACIALHAIR_{}_MASK", a_idx))
            }
            _ => std::ptr::null_mut(),
        }
    }

    /// 对应 C++ ZombatarWidget::GetCategoryImage
    pub fn get_category_image(&self, the_page: ZombatarPage, the_selected: bool, the_over: bool) -> *mut Image {
        let a_page_name = match the_page {
            ZombatarPage::Skin => "SKIN",
            ZombatarPage::Hair => "HAIR",
            ZombatarPage::FacialHair => "FACIAL_HAIR",
            ZombatarPage::Tidbits => "TIDBITS",
            ZombatarPage::Eyewear => "EYEWEAR",
            ZombatarPage::Clothes => "CLOTHES",
            ZombatarPage::Accessory => "ACCESSORY",
            ZombatarPage::Hats => "HATS",
            ZombatarPage::Backdrops => "BACKDROPS",
        };
        // C++: Skin 无独立 over 图，over 时回退到 highlight
        let a_suffix = if the_selected {
            "_HIGHLIGHT"
        } else if the_over {
            if the_page == ZombatarPage::Skin { "_HIGHLIGHT" } else { "_OVER" }
        } else {
            ""
        };
        self.get_resource_image(&format!("IMAGE_ZOMBATAR_{}_BUTTON{}", a_page_name, a_suffix))
    }

    /// 对应 C++ ZombatarWidget::CreatePreviewZombie
    pub fn create_preview_zombie(&mut self) {
        if self.preview_zombie.is_some() {
            return;
        }
        let mut a_preview = Box::new(crate::lawn::zombie::Zombie::new());
        // C++: mPreviewZombie->mApp = mApp; mPreviewZombie->mBoard = nullptr;
        a_preview.base.app = self.app;
        a_preview.base.board = None;
        a_preview.zombie_initialize(0, ZombieType::Flag, false, None, crate::lawn::zombie::Zombie::ZOMBIE_WAVE_UI);
        a_preview.pos_x = 0.0;
        a_preview.pos_y = 0.0;
        self.preview_zombie = Some(a_preview);
    }

    /// 对应 C++ ZombatarWidget::DestroyPreviewZombie
    pub fn destroy_preview_zombie(&mut self) {
        if let Some(mut a_preview) = self.preview_zombie.take() {
            // C++: mPreviewZombie->DieNoLoot(); delete mPreviewZombie;
            a_preview.die_no_loot();
        }
    }

    /// 从 ResourceManager 按 key 取图（对应 C++ IMAGE_* 全局资源；未接入资源表时返回 null）
    fn get_resource_image(&self, a_key: &str) -> *mut Image {
        let Some(app) = self.app else { return std::ptr::null_mut() };
        unsafe {
            let app_ref = &*app;
            let Some(rm) = app_ref.base.resource_manager else { return std::ptr::null_mut() };
            let rm_ref = &*rm;
            rm_ref.get_image(a_key).as_image_ptr()
        }
    }

    /// 懒加载字体（对应 C++ FONT_BRIANNETOD12 等全局字体常量；图片字库未接线用 Font 对象近似）
    fn font(&mut self, the_kind: usize) -> *mut Font {
        let slot = &mut self.fonts[the_kind.min(4)];
        if slot.is_none() {
            let (a_name, a_size) = match the_kind.min(4) {
                0 => ("Briannetod", 12),
                1 => ("Dwarventodcraft", 12),
                2 => ("Dwarventodcraft", 15),
                3 => ("Houseofterror", 28),
                _ => ("Continuumbold", 14),
            };
            let mut f = Font::new(a_name, a_size);
            f.ascent = 13;
            f.font_height = a_size;
            *slot = Some(Box::new(f));
        }
        &**slot.as_ref().unwrap() as *const Font as *mut Font
    }

    /// [TRANSLATION_NOTE]: C++ ZombatarWidget::Update —— transition_timer 逻辑完整，按钮/预览占位
    pub fn update(&mut self) {
        // C++: Widget::Update() 后预览僵尸生命周期（mX >= BOARD_WIDTH 滑出判定因无坐标字段省略）
        if let Some(a_preview) = self.preview_zombie.as_mut() {
            a_preview.update();
        } else if self.app.map_or(false, |app| unsafe { (*app).player_info.is_some() }) {
            self.create_preview_zombie();
        }
        // 对应 C++ ZombatarWidget::Update 的 transition_timer 逻辑
        if self.state == ZombatarWidgetState::ToConfirm || self.state == ZombatarWidgetState::FromConfirm {
            self.transition_timer -= 1; // dec-then-use
            let a_to_confirm = self.state == ZombatarWidgetState::ToConfirm;
            let a_from_x = if a_to_confirm { ZOMBATAR_FINISHED_X } else { ZOMBATAR_ACCEPT_X };
            let a_from_y = if a_to_confirm { ZOMBATAR_FINISHED_Y } else { ZOMBATAR_CONFIRM_BTN_Y };
            let a_to_x = if a_to_confirm { ZOMBATAR_ACCEPT_X } else { ZOMBATAR_FINISHED_X };
            let a_to_y = if a_to_confirm { ZOMBATAR_CONFIRM_BTN_Y } else { ZOMBATAR_FINISHED_Y };
            // C++: mFinishedButton->Resize(AnimateCurve, AnimateCurve, 103, 26)
            let _curve_x = crate::todlib::tod_common::tod_animate_curve(
                ZOMBATAR_TRANSITION_TICKS, 0, self.transition_timer, a_from_x, a_to_x, TodCurves::Linear,
            );
            let _curve_y = crate::todlib::tod_common::tod_animate_curve(
                ZOMBATAR_TRANSITION_TICKS, 0, self.transition_timer, a_from_y, a_to_y, TodCurves::Linear,
            );
            let _ = (_curve_x, _curve_y);
            if self.transition_timer <= 0 {
                self.transition_timer = 0;
                self.change_state(if a_to_confirm {
                    ZombatarWidgetState::Confirm
                } else {
                    ZombatarWidgetState::Create
                });
            }
        }
        // C++: MarkDirty() —— Rust 无脏区系统，省略
    }

    /// 对应 C++ ZombatarWidget::UpdateButtonState —— 分页逻辑完整，按钮可见性占位
    pub fn update_button_state(&mut self) {
        let a_count = self.get_head_count();
        let a_list = self.state == ZombatarWidgetState::List;
        let a_create = self.state == ZombatarWidgetState::Create;
        let a_confirm = self.state == ZombatarWidgetState::Confirm;
        let a_transition = self.state == ZombatarWidgetState::ToConfirm
            || self.state == ZombatarWidgetState::FromConfirm;

        let a_total = get_total_items_for_page(self.page);
        self.max_sub_pages = if a_total > ZOMBATAR_GRID_PAGE {
            (a_total - 1) / ZOMBATAR_GRID_PAGE
        } else {
            0
        };
        self.sub_page = self.sub_page.clamp(0, self.max_sub_pages);
        let _a_paged = a_create && self.max_sub_pages > 0;

        // C++: 各按钮 SetVisible/SetDisabled —— 按钮未创建，状态判定保留
        let _ = (a_list, a_create, a_confirm, a_transition, a_count, _a_paged);
        let _ = (ZOMBATAR_ACCEPT_X, ZOMBATAR_CONFIRM_BTN_Y, ZOMBATAR_FINISHED_X, ZOMBATAR_FINISHED_Y);
    }

    /// 对应 C++ ZombatarWidget::MouseMove（hover 追踪逻辑）
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.hover_grid_cell = -1;
        self.hover_color_cell = -1;
        self.hover_tab = -1;

        if self.state == ZombatarWidgetState::List {
            self.delete_hover = Rect::new(
                ZOMBATAR_LIST_DELETE_RECT_X,
                ZOMBATAR_LIST_DELETE_RECT_Y,
                ZOMBATAR_LIST_DELETE_RECT_WIDTH,
                ZOMBATAR_LIST_DELETE_RECT_HEIGHT,
            )
            .contains(x, y);
            return;
        }

        if self.state != ZombatarWidgetState::Create {
            return;
        }

        for i in 0..9 {
            if self.get_category_rect(i as i32).contains(x, y) {
                self.hover_tab = i as i32;
                break;
            }
        }

        let a_item_count = self.get_sub_page_item_count();
        for i in 0..a_item_count {
            if self.get_item_hit_rect(i).contains(x, y) {
                self.hover_grid_cell = i;
                break;
            }
        }
        if self.hover_grid_cell < 0 && self.page_allows_none() && self.get_item_hit_rect(a_item_count).contains(x, y) {
            self.hover_grid_cell = a_item_count;
        }

        if self.page_allows_colors() {
            let a_color_count = if self.page == ZombatarPage::Skin {
                ZOMBATAR_SKIN_COLOR_COUNT
            } else {
                ZOMBATAR_PART_COLOR_COUNT
            };
            for i in 0..a_color_count {
                if self.get_color_rect(i).contains(x, y) {
                    self.hover_color_cell = i;
                    break;
                }
            }
        }
    }

    /// 对应 C++ ZombatarWidget::MouseUp
    pub fn mouse_up(&mut self, x: i32, y: i32) {
        if self.app.map_or(true, |app| unsafe { (*app).player_info.is_none() }) {
            return;
        }
        if self.state == ZombatarWidgetState::Create {
            for i in 0..9 {
                if self.get_category_rect(i as i32).contains(x, y) {
                    self.change_page(match i {
                        ZOMBATAR_PAGE_HAIR => ZombatarPage::Hair,
                        ZOMBATAR_PAGE_FACIAL_HAIR => ZombatarPage::FacialHair,
                        ZOMBATAR_PAGE_TIDBITS => ZombatarPage::Tidbits,
                        ZOMBATAR_PAGE_EYEWEAR => ZombatarPage::Eyewear,
                        ZOMBATAR_PAGE_CLOTHES => ZombatarPage::Clothes,
                        ZOMBATAR_PAGE_ACCESSORY => ZombatarPage::Accessory,
                        ZOMBATAR_PAGE_HATS => ZombatarPage::Hats,
                        ZOMBATAR_PAGE_BACKDROPS => ZombatarPage::Backdrops,
                        _ => ZombatarPage::Skin,
                    });
                    return;
                }
            }
            self.handle_grid_click(x, y);
            self.handle_color_click(x, y);
        } else if self.state == ZombatarWidgetState::List {
            // C++: 删除矩形 → LawnMessageBox(DIALOG_ZOMBATAR_DELETE=52) 确认 → DeleteCurrent
            if self.get_head_count() > 0
                && Rect::new(
                    ZOMBATAR_LIST_DELETE_RECT_X,
                    ZOMBATAR_LIST_DELETE_RECT_Y,
                    ZOMBATAR_LIST_DELETE_RECT_WIDTH,
                    ZOMBATAR_LIST_DELETE_RECT_HEIGHT,
                )
                .contains(x, y)
            {
                if let Some(app) = self.app {
                    unsafe {
                        let a_dialog = (*app).do_dialog(
                            52, // DIALOG_ZOMBATAR_DELETE
                            true,
                            "[ZOMBATAR_DELETE_HEADER]",
                            "[ZOMBATAR_DELETE_BODY]",
                            "[ZOMBATAR_DELETE_BUTTON]",
                            BUTTONS_YES_NO,
                        );
                        let a_result = a_dialog.map_or(0, |d| unsafe { (&mut *d).wait_for_result(true) });
                        if a_result == ID_YES {
                            self.delete_current();
                        }
                    }
                }
            }
        }
    }

    /// 对应 C++ ZombatarWidget::ButtonPress（空实现）
    pub fn button_press(&mut self, _the_id: i32) {}

    /// 对应 C++ ZombatarWidget::DrawImageColorized
    fn draw_image_colorized(&self, g: &mut Graphics, the_image: &Image, the_x: i32, the_y: i32, the_color_index: i32) {
        g.set_colorize_images(true);
        g.set_color(&zombatar_get_color(the_color_index));
        g.draw_image_xy(the_image, the_x, the_y);
        g.set_colorize_images(false);
        g.set_color(&Color::WHITE);
    }

    /// 对应 C++ ZombatarWidget::DrawPartImage
    fn draw_part_image(&self, g: &mut Graphics, the_page: ZombatarPage, the_index: i32, the_x: i32, the_y: i32, the_color_index: i32) {
        let a_image = self.get_part_image(the_page, the_index);
        let a_mask = self.get_part_mask_image(the_page, the_index);
        if a_image.is_null() {
            return;
        }
        let a_layout = get_part_layout(the_page, the_index);
        let a_pos_x = the_x + a_layout.map_or(0, |l| l.m_offset_x);
        let a_pos_y = the_y + a_layout.map_or(0, |l| l.m_offset_y);

        if get_part_color_mode(the_page, the_index) == ZOMBATAR_COLOR_MODE_NONE {
            g.draw_image_xy(unsafe { &*a_image }, a_pos_x, a_pos_y);
            return;
        }

        if !a_mask.is_null() {
            let a_color_off_x = a_layout.map_or(0, |l| l.m_color_offset_x);
            let a_color_off_y = a_layout.map_or(0, |l| l.m_color_offset_y);
            self.draw_image_colorized(g, unsafe { &*a_mask }, a_pos_x + a_color_off_x, a_pos_y + a_color_off_y, the_color_index);
            g.draw_image_xy(unsafe { &*a_image }, a_pos_x, a_pos_y);
        } else {
            self.draw_image_colorized(g, unsafe { &*a_image }, a_pos_x, a_pos_y, the_color_index);
        }
    }

    /// 对应 C++ ZombatarWidget::DrawAvatar（局部解码，不污染成员 mPart/mColor）
    pub fn draw_avatar(&self, g: &mut Graphics, the_x: i32, the_y: i32, the_record: &[u8]) {
        // C++: int aPart[NUM_ZOMBATAR_PAGES]; int aColor[NUM_ZOMBATAR_PAGES]; DecodeRecord(theRecord, aPart, aColor);
        let mut a_part = [-1i32; 9];
        let mut a_color = [ZOMBATAR_COLOR_NONE; 9];
        a_part[ZOMBATAR_PAGE_SKIN] = 0;
        let a_skin_color = zombatar_read_signed_record_slot(the_record, ZOMBATAR_SLOT_SKIN_COLOR);
        a_color[ZOMBATAR_PAGE_SKIN] = if a_skin_color < 0 { 0 } else { a_skin_color.min(11) };
        for i in ZOMBATAR_PAGE_HAIR..9 {
            let a_page = page_for_index(i);
            let mut a_item_count = get_total_items_for_page(a_page);
            if a_item_count > 16 {
                a_item_count += a_item_count / 17;
            }
            let a_part_val = zombatar_read_signed_record_slot(the_record, slot_for_part(a_page));
            let a_color_val = zombatar_read_signed_record_slot(the_record, slot_for_color(a_page));
            if a_page == ZombatarPage::Backdrops {
                a_part[i] = a_part_val.clamp(0, a_item_count - 1);
            } else {
                a_part[i] = if a_part_val >= 0 && a_part_val < a_item_count { a_part_val } else { -1 };
            }
            a_color[i] = if a_color_val < 0 { ZOMBATAR_COLOR_NONE } else { a_color_val };
        }

        // 背景
        let a_background = self.get_background_image(a_part[ZOMBATAR_PAGE_BACKDROPS]);
        if !a_background.is_null() {
            if get_part_color_mode(ZombatarPage::Backdrops, a_part[ZOMBATAR_PAGE_BACKDROPS]) == ZOMBATAR_COLOR_MODE_2 {
                self.draw_image_colorized(g, unsafe { &*a_background }, the_x, the_y, a_color[ZOMBATAR_PAGE_BACKDROPS]);
            } else {
                g.draw_image_xy(unsafe { &*a_background }, the_x, the_y);
            }
        }

        // blank 皮肤
        let mut a_blank_x = the_x;
        let mut a_blank_y = the_y;
        let a_bg_blank = self.get_resource_image("IMAGE_ZOMBATAR_BACKGROUND_BLANK");
        let a_zombie_blank = self.get_resource_image("IMAGE_ZOMBATAR_ZOMBIE_BLANK");
        let a_zombie_blank_skin = self.get_resource_image("IMAGE_ZOMBATAR_ZOMBIE_BLANK_SKIN");
        if !a_bg_blank.is_null() && !a_zombie_blank.is_null() {
            unsafe {
                let a_bg = &*a_bg_blank;
                let a_zb = &*a_zombie_blank;
                a_blank_x = the_x + (a_bg.get_width() - a_zb.get_width());
                a_blank_y = the_y + (a_bg.get_height() - a_zb.get_height());
            }
        }
        if !a_zombie_blank_skin.is_null() {
            self.draw_image_colorized(g, unsafe { &*a_zombie_blank_skin }, a_blank_x, a_blank_y, a_color[ZOMBATAR_PAGE_SKIN]);
        }
        if !a_zombie_blank.is_null() {
            g.draw_image_xy(unsafe { &*a_zombie_blank }, a_blank_x, a_blank_y);
        }

        // 部件按绘制顺序稳定排序后逐层绘制
        let mut a_parts: Vec<(ZombatarPage, i32, i32, i32)> = Vec::new(); // (page, index, color, drawOrder)
        for i in ZOMBATAR_PAGE_HAIR..ZOMBATAR_PAGE_BACKDROPS {
            let a_page = page_for_index(i);
            let a_index = a_part[i];
            if a_index >= 0 {
                let a_draw_order = get_part_layout(a_page, a_index).map_or(0, |l| l.m_draw_order);
                a_parts.push((a_page, a_index, a_color[i], a_draw_order));
            }
        }
        a_parts.sort_by_key(|p| p.3); // C++ std::stable_sort（Rust sort 稳定）
        for (a_page, a_index, a_color_i, _) in a_parts {
            self.draw_part_image(g, a_page, a_index, the_x, the_y, a_color_i);
        }
    }

    /// 对应 C++ ZombatarWidget::DrawDraftAvatar
    pub fn draw_draft_avatar(&self, g: &mut Graphics, the_x: i32, the_y: i32) {
        let mut a_record = vec![0u8; ZOMBATAR_RECORD_SIZE];
        self.encode_record(&mut a_record);
        self.draw_avatar(g, the_x, the_y, &a_record);
    }

    /// 对应 C++ ZombatarWidget::DrawColorSwatches
    pub fn draw_color_swatches(&self, g: &mut Graphics, the_palette_base: i32, the_count: i32, the_saved_color: i32) {
        g.set_colorize_images(true);
        for i in 0..the_count {
            let a_palette = the_palette_base + i;
            let mut a_alpha = 0x40;
            if a_palette == the_saved_color {
                a_alpha = 0xff;
            } else if i == self.hover_color_cell {
                a_alpha = 0x80;
            }
            let a_color = zombatar_get_color(a_palette);
            g.set_color(&Color::new(a_color.r, a_color.g, a_color.b, a_alpha as u8));
            let a_is_none = a_palette == ZOMBATAR_PART_COLOR_NONE_1 || a_palette == ZOMBATAR_PART_COLOR_NONE_2;
            let a_image = if a_is_none {
                self.get_resource_image("IMAGE_ZOMBATAR_COLORPICKER_NONE")
            } else {
                self.get_resource_image("IMAGE_ZOMBATAR_COLORPICKER")
            };
            if !a_image.is_null() {
                let a_rect = self.get_color_rect(i);
                g.draw_image_xy(unsafe { &*a_image }, a_rect.x, a_rect.y);
            }
        }
        g.set_colorize_images(false);
        g.set_color(&Color::WHITE);
    }

    /// 对应 C++ ZombatarWidget::DrawAvatarBox
    pub fn draw_avatar_box(&mut self, g: &mut Graphics) {
        if self.preview_zombie.is_none() {
            return;
        }

        // C++: aRecord 取值（列表模式读存档 / 否则编码草稿）
        let a_record: Vec<u8>;
        let a_record_ref: &[u8];
        if self.state == ZombatarWidgetState::List {
            if self.get_head_count() <= 0 {
                return;
            }
            self.clamp_current_index();
            a_record = self
                .app
                .and_then(|app| unsafe { (*app).player_info.as_ref() })
                .map(|pi| {
                    let an_offset = self.current_index as usize * ZOMBATAR_RECORD_SIZE;
                    let a_data = pi.m_zombatar_data.clone();
                    if an_offset + ZOMBATAR_RECORD_SIZE <= a_data.len() {
                        a_data[an_offset..an_offset + ZOMBATAR_RECORD_SIZE].to_vec()
                    } else {
                        Vec::new()
                    }
                })
                .unwrap_or_default();
            a_record_ref = &a_record;
        } else {
            let mut a_draft = vec![0u8; ZOMBATAR_RECORD_SIZE];
            self.encode_record(&mut a_draft);
            a_record = a_draft;
            a_record_ref = &a_record;
        }

        if let Some(a_preview) = self.preview_zombie.as_mut() {
            a_preview.apply_zombatar_head(a_record_ref);
        }

        // 地面（C++: IMAGE_ALMANAC_GROUNDDAY + 裁剪）
        let a_ground = self.get_resource_image("IMAGE_ALMANAC_GROUNDDAY");
        if !a_ground.is_null() {
            unsafe {
                let a_ground_ref = &*a_ground;
                g.set_clip_rect(&Rect::new(
                    ZOMBATAR_AVATAR_GROUND_X,
                    ZOMBATAR_AVATAR_GROUND_Y + ZOMBATAR_AVATAR_GROUND_CLIP_TOP,
                    a_ground_ref.get_width() - ZOMBATAR_AVATAR_GROUND_CLIP_RIGHT,
                    a_ground_ref.get_height() - ZOMBATAR_AVATAR_GROUND_CLIP_TOP - ZOMBATAR_AVATAR_GROUND_CLIP_BOTTOM,
                ));
                g.draw_image_xy(a_ground_ref, ZOMBATAR_AVATAR_GROUND_X, ZOMBATAR_AVATAR_GROUND_Y);
                g.clear_clip_rect();
            }
        }

        // C++: mPreviewZombie->mPosX/mPosY/mX/mY 设置 + BeginDraw + Draw
        if let Some(a_preview) = self.preview_zombie.as_mut() {
            a_preview.pos_x = ZOMBATAR_AVATAR_ZOMBIE_X as f32;
            a_preview.pos_y = ZOMBATAR_AVATAR_ZOMBIE_Y as f32;
            a_preview.base.x = ZOMBATAR_AVATAR_ZOMBIE_X;
            a_preview.base.y = ZOMBATAR_AVATAR_ZOMBIE_Y;
            // [TRANSLATION_NOTE]: C++ Graphics aZombieGraphics(*g) 复制图形上下文后 BeginDraw（translate）；
            // Rust 端 Zombie::draw 按自身坐标渲染，直接复用 g，等价。
            a_preview.draw(g);
        }
    }

    /// 对应 C++ ZombatarWidget::Draw
    pub fn draw(&mut self, g: &mut Graphics) {
        // C++: if (!mApp->mPlayerInfo || mX >= BOARD_WIDTH) return;
        // [TRANSLATION_NOTE]: Rust 端无 widget 坐标字段（C++ mX >= BOARD_WIDTH 为滑出屏幕判定），省略。
        if self.app.map_or(true, |app| unsafe { (*app).player_info.is_none() }) {
            return;
        }

        self.draw_main_background(g);
        self.draw_avatar_box(g);
        if self.state != ZombatarWidgetState::List {
            self.draw_draft_avatar(g, ZOMBATAR_PREVIEW_X, ZOMBATAR_PREVIEW_Y);
        }
        match self.state {
            ZombatarWidgetState::List => self.draw_list(g),
            ZombatarWidgetState::Confirm => self.draw_confirm(g),
            ZombatarWidgetState::Create => self.draw_create(g),
            _ => self.draw_transition(g),
        }

        let a_window = self.get_resource_image("IMAGE_ZOMBATAR_DISPLAY_WINDOW");
        if !a_window.is_null() {
            g.draw_image_xy(unsafe { &*a_window }, 5, 0);
        }
    }

    /// 对应 C++ ZombatarWidget::DrawMainBackground
    fn draw_main_background(&self, g: &mut Graphics) {
        let a_bg = self.get_resource_image("IMAGE_ZOMBATAR_MAIN_BG");
        if !a_bg.is_null() {
            g.draw_image_xy(unsafe { &*a_bg }, 0, 0);
        }
    }

    fn get_image_width(&self, a_key: &str, a_default: i32) -> i32 {
        let a_image = self.get_resource_image(a_key);
        if a_image.is_null() {
            a_default
        } else {
            unsafe { (*a_image).get_width() }
        }
    }

    fn get_image_height(&self, a_key: &str, a_default: i32) -> i32 {
        let a_image = self.get_resource_image(a_key);
        if a_image.is_null() {
            a_default
        } else {
            unsafe { (*a_image).get_height() }
        }
    }

    /// 对应 C++ FitIconRect（static）
    fn fit_icon_rect(a_image: &Image, a_cell: &Rect, a_align: i32) -> Rect {
        let a_cell_w = a_cell.width;
        let a_cell_h = a_cell.height;
        let a_avail_w = a_cell_w - 2 * ZOMBATAR_CELL_INSET;
        let a_avail_h = a_cell_h - 2 * ZOMBATAR_CELL_INSET;
        let mut a_w = a_image.get_width();
        let mut a_h = a_image.get_height();
        let a_ovfl_w = a_w as f32 > a_avail_w as f32;
        let a_ovfl_h = a_h as f32 > a_avail_h as f32;
        if a_ovfl_w && (!a_ovfl_h || a_w >= a_h) {
            a_h = (a_h as f32 * (a_avail_w as f32 / a_w as f32)) as i32;
            a_w = a_avail_w;
        } else if a_ovfl_h {
            a_w = (a_w as f32 * (a_avail_h as f32 / a_h as f32)) as i32;
            a_h = a_avail_h;
        }
        let mut a_x = a_cell.x + (a_cell_w - a_w) / 2;
        let mut a_y = a_cell.y + (a_cell_h - a_h) / 2;
        if a_align & ZOMBATAR_ALIGN_TOP != 0 {
            a_y = a_cell.y + ZOMBATAR_CELL_INSET;
        } else if a_align & ZOMBATAR_ALIGN_BOTTOM != 0 {
            a_y = a_cell.y + a_cell_h - ZOMBATAR_CELL_INSET - a_h;
        }
        if a_align & ZOMBATAR_ALIGN_LEFT != 0 {
            a_x = a_cell.x + ZOMBATAR_CELL_INSET;
        } else if a_align & ZOMBATAR_ALIGN_RIGHT != 0 {
            a_x = a_cell.x + a_cell_w - ZOMBATAR_CELL_INSET - a_w;
        }
        Rect::new(a_x, a_y, a_w, a_h)
    }

    /// 对应 C++ ZombatarGridAlign（static）
    fn zombatar_grid_align(the_page: ZombatarPage, the_index: i32) -> i32 {
        match the_page {
            ZombatarPage::Clothes => {
                if the_index == 5 { 0 } else { ZOMBATAR_ALIGN_RIGHT | ZOMBATAR_ALIGN_BOTTOM }
            }
            ZombatarPage::FacialHair => {
                if the_index == 14 || the_index == 21 || the_index == 23 { ZOMBATAR_ALIGN_BOTTOM } else { 0 }
            }
            ZombatarPage::Hair => {
                if the_index == 11 || the_index == 15 { ZOMBATAR_ALIGN_TOP } else { 0 }
            }
            ZombatarPage::Hats => {
                if (6..=8).contains(&the_index) { ZOMBATAR_ALIGN_TOP } else { 0 }
            }
            _ => 0,
        }
    }

    /// 对应 C++ ZombatarWidget::DrawList
    pub fn draw_list(&mut self, g: &mut Graphics) {
        let a_bg = self.get_resource_image("IMAGE_ZOMBATAR_WIDGET_BG");
        if !a_bg.is_null() {
            g.draw_image_xy(unsafe { &*a_bg }, ZOMBATAR_PANEL_X, ZOMBATAR_PANEL_Y);
        }

        let a_count = self.get_head_count();
        self.clamp_current_index();
        if a_count <= 0 {
            return;
        }

        // C++: aBlankW（BACKGROUND_BLANK 宽，缺省 ZOMBATAR_LIST_BLANK_WIDTH）
        let a_blank_w = self.get_image_width("IMAGE_ZOMBATAR_BACKGROUND_BLANK", ZOMBATAR_LIST_BLANK_WIDTH);
        let a_portrait_x = ZOMBATAR_PANEL_X + (ZOMBATAR_PANEL_WIDTH - ZOMBATAR_LIST_CENTER_INSET) / 2
            + ZOMBATAR_LIST_SPACING / 2 - a_blank_w / 2;

        g.set_clip_rect(&Rect::new(
            ZOMBATAR_LIST_CLIP_X,
            ZOMBATAR_LIST_CLIP_Y,
            ZOMBATAR_PANEL_WIDTH - ZOMBATAR_LIST_CLIP_RIGHT,
            ZOMBATAR_LIST_CLIP_HEIGHT,
        ));
        g.set_color(&Color::BLACK);
        g.fill_rect_xywh(
            ZOMBATAR_LIST_CLIP_X,
            ZOMBATAR_LIST_CLIP_Y,
            ZOMBATAR_PANEL_WIDTH - ZOMBATAR_LIST_CLIP_RIGHT,
            ZOMBATAR_LIST_FILL_HEIGHT,
        );

        let a_data = self
            .app
            .and_then(|app| unsafe { (*app).player_info.as_ref() })
            .map(|pi| pi.m_zombatar_data.clone())
            .unwrap_or_default();
        // C++: for (int i = -1; i <= 1; i++) 画相邻三幅小头像
        for i in -1i32..=1 {
            let a_index = self.current_index + i;
            if a_index >= 0 && a_index < a_count {
                let an_offset = a_index as usize * ZOMBATAR_RECORD_SIZE;
                if an_offset + ZOMBATAR_RECORD_SIZE <= a_data.len() {
                    let a_record = &a_data[an_offset..an_offset + ZOMBATAR_RECORD_SIZE];
                    self.draw_avatar(g, a_portrait_x + i * (ZOMBATAR_LIST_SPACING + a_blank_w), ZOMBATAR_LIST_PORTRAIT_Y, a_record);
                }
            }
        }
        g.clear_clip_rect();
        g.set_color(&Color::WHITE);

        // 大预览（当前头像）
        let a_cur_offset = self.current_index as usize * ZOMBATAR_RECORD_SIZE;
        if a_cur_offset + ZOMBATAR_RECORD_SIZE <= a_data.len() {
            let a_record = &a_data[a_cur_offset..a_cur_offset + ZOMBATAR_RECORD_SIZE];
            self.draw_avatar(g, ZOMBATAR_PREVIEW_X, ZOMBATAR_PREVIEW_Y, a_record);
        }

        // C++: DrawString(StrFormat("%d / %d"), 计数器) —— FONT_BRIANNETOD12
        let a_counter = format!("{} / {}", self.current_index + 1, a_count);
        g.set_font(self.font(0));
        g.set_color(&Color::WHITE);
        g.draw_string(&a_counter, ZOMBATAR_LIST_COUNTER_X, ZOMBATAR_LIST_COUNTER_Y);

        // C++: "Delete?" —— FONT_DWARVENTODCRAFT12，hover 变色
        g.set_font(self.font(1));
        if self.delete_hover {
            g.set_color(&Color::new(22, 253, 5, 255));
        } else {
            g.set_color(&Color::WHITE);
        }
        g.draw_string("Delete?", ZOMBATAR_LIST_DELETE_X, ZOMBATAR_LIST_DELETE_Y);
        g.set_color(&Color::WHITE);
    }

    /// 对应 C++ ZombatarWidget::DrawCreate
    pub fn draw_create(&mut self, g: &mut Graphics) {
        let a_bg = self.get_resource_image("IMAGE_ZOMBATAR_WIDGET_BG");
        if !a_bg.is_null() {
            g.draw_image_xy(unsafe { &*a_bg }, ZOMBATAR_PANEL_X, ZOMBATAR_PANEL_Y);
        }
        let a_inner_bg = self.get_resource_image("IMAGE_ZOMBATAR_WIDGET_INNER_BG");
        if !a_inner_bg.is_null() {
            g.draw_image_xy(unsafe { &*a_inner_bg }, ZOMBATAR_INNER_X, ZOMBATAR_INNER_Y);
        }

        // 分类页签（对应 C++ for i < NUM_ZOMBATAR_PAGES）
        for i in 0..9 {
            let a_rect = self.get_category_rect(i as i32);
            let a_page = page_for_index(i);
            let a_category = self.get_category_image(a_page, a_page == self.page, i == self.hover_tab as usize);
            if !a_category.is_null() {
                g.draw_image_xy(unsafe { &*a_category }, a_rect.x, a_rect.y);
            }
        }

        // 皮肤页起始说明（对应 C++ [ZOMBATAR_START_TEXT]
        if self.page == ZombatarPage::Skin {
            self.draw_string_wrapped(
                g,
                "[ZOMBATAR_START_TEXT]",
                &Rect::new(
                    ZOMBATAR_PANEL_X + ZOMBATAR_PANEL_WIDTH / 2 - 200,
                    ZOMBATAR_START_TEXT_Y,
                    ZOMBATAR_START_TEXT_WIDTH,
                    ZOMBATAR_START_TEXT_HEIGHT,
                ),
                2, // FONT_DWARVENTODCRAFT15
                &Color::new(254, 227, 0, 175),
                DS_ALIGN_CENTER,
            );
            g.set_color(&Color::WHITE);
        }

        // 部件格子（对应 C++ GetSubPageItemCount 循环）
        let a_item_count = self.get_sub_page_item_count();
        let a_base_index = self.sub_page * ZOMBATAR_GRID_PAGE;
        for i in 0..a_item_count {
            let a_rect = self.get_item_rect(i);
            let mut a_part_index = a_base_index + i;
            if a_part_index > 16 {
                a_part_index += a_part_index / 17;
            }
            let a_selected = self.part[self.page as usize] == a_part_index;
            let a_hover = i == self.hover_grid_cell;
            let a_dim = !a_selected && !a_hover;

            if a_dim {
                g.set_colorize_images(true);
                g.set_color(&ZOMBATAR_CELL_DIM_COLOR);
            }
            let a_cell_img = if a_selected {
                self.get_resource_image("IMAGE_ZOMBATAR_ACCESSORY_BG_HIGHLIGHT")
            } else {
                self.get_resource_image("IMAGE_ZOMBATAR_ACCESSORY_BG")
            };
            if !a_cell_img.is_null() {
                g.draw_image_xy(unsafe { &*a_cell_img }, a_rect.x, a_rect.y);
            }

            // 衣服页：格子内骷髅底（皮肤色 blank）预览
            if self.page == ZombatarPage::Clothes {
                let a_blank_skin = self.get_resource_image("IMAGE_ZOMBATAR_ZOMBIE_BLANK_SKIN");
                let a_blank = self.get_resource_image("IMAGE_ZOMBATAR_ZOMBIE_BLANK");
                if !a_blank_skin.is_null() && !a_blank.is_null() {
                    unsafe {
                        let a_skin_c = zombatar_get_color(self.color[ZOMBATAR_PAGE_SKIN]);
                        let a_skin_c = Color::new(a_skin_c.r, a_skin_c.g, a_skin_c.b, if a_dim { 0x80 } else { 0xff });
                        let a_cell_rect = Rect::new(
                            a_rect.x - ZOMBATAR_CELL_ZOMBIE_MARGIN,
                            a_rect.y - ZOMBATAR_CELL_ZOMBIE_MARGIN,
                            a_rect.width,
                            a_rect.height,
                        );
                        let a_src_blank = &*a_blank;
                        let a_src_blank_skin = &*a_blank_skin;
                        let a_src_rect = Rect::new(0, 0, a_src_blank.get_width(), a_src_blank.get_height());
                        g.set_clip_rect(&Rect::new(
                            a_rect.x + ZOMBATAR_CELL_INSET,
                            a_rect.y + ZOMBATAR_CELL_INSET,
                            a_rect.width - ZOMBATAR_CELL_INSET,
                            a_rect.height - ZOMBATAR_CELL_INSET,
                        ));
                        g.set_colorize_images(true);
                        g.set_color(&a_skin_c);
                        g.draw_image_stretch(a_src_blank_skin, &a_cell_rect, &a_src_rect);
                        g.set_colorize_images(false);
                        g.draw_image_stretch(a_src_blank, &a_cell_rect, &a_src_rect);
                        g.clear_clip_rect();
                        if a_dim {
                            g.set_colorize_images(true);
                            g.set_color(&ZOMBATAR_CELL_DIM_COLOR);
                        }
                    }
                }
            }

            // 部件图标（+ 掩码染色）
            let a_image = if self.page == ZombatarPage::Backdrops {
                self.get_background_image(a_part_index)
            } else {
                self.get_part_image(self.page, a_part_index)
            };
            if !a_image.is_null() {
                unsafe {
                    let a_image_ref = &*a_image;
                    let a_icon_rect = Self::fit_icon_rect(a_image_ref, &a_rect, Self::zombatar_grid_align(self.page, a_part_index));
                    let a_mask = self.get_part_mask_image(self.page, a_part_index);
                    if !a_mask.is_null() {
                        let a_mask_ref = &*a_mask;
                        let a_layout = get_part_layout(self.page, a_part_index);
                        let a_scale_x = a_icon_rect.width as f32 / a_image_ref.get_width() as f32;
                        let a_scale_y = a_icon_rect.height as f32 / a_image_ref.get_height() as f32;
                        let a_mask_x = a_icon_rect.x + a_layout.map_or(0, |l| (l.m_color_offset_x as f32 * a_scale_x) as i32);
                        let a_mask_y = a_icon_rect.y + a_layout.map_or(0, |l| (l.m_color_offset_y as f32 * a_scale_y) as i32);
                        g.draw_image_stretch(
                            a_mask_ref,
                            &Rect::new(
                                a_mask_x,
                                a_mask_y,
                                (a_mask_ref.get_width() as f32 * a_scale_x) as i32,
                                (a_mask_ref.get_height() as f32 * a_scale_y) as i32,
                            ),
                            &Rect::new(0, 0, a_mask_ref.get_width(), a_mask_ref.get_height()),
                        );
                    }
                    g.draw_image_stretch(
                        a_image_ref,
                        &a_icon_rect,
                        &Rect::new(0, 0, a_image_ref.get_width(), a_image_ref.get_height()),
                    );
                }
            }

            if a_dim {
                g.set_colorize_images(false);
                g.set_color(&Color::WHITE);
            }
        }

        // 页末"无部件"格（对应 C++ PageAllowsNone）
        if self.page_allows_none() {
            let a_rect = self.get_item_rect(a_item_count);
            let a_selected = self.part[self.page as usize] < 0;
            let a_hover = self.hover_grid_cell == a_item_count;
            let a_dim = !a_selected && !a_hover;
            if a_dim {
                g.set_colorize_images(true);
                g.set_color(&ZOMBATAR_CELL_DIM_COLOR);
            }
            let a_cell_img = if a_selected {
                self.get_resource_image("IMAGE_ZOMBATAR_ACCESSORY_BG_HIGHLIGHT")
            } else {
                self.get_resource_image("IMAGE_ZOMBATAR_ACCESSORY_BG")
            };
            if !a_cell_img.is_null() {
                g.draw_image_xy(unsafe { &*a_cell_img }, a_rect.x, a_rect.y);
            }
            let a_none_img = self.get_resource_image("IMAGE_ZOMBATAR_ACCESSORY_BG_NONE");
            if !a_none_img.is_null() {
                g.draw_image_xy(unsafe { &*a_none_img }, a_rect.x, a_rect.y);
            }
            if a_dim {
                g.set_colorize_images(false);
                g.set_color(&Color::WHITE);
            }
        }

        // 色彩区（对应 C++ IMAGE_ZOMBATAR_COLORS_BG）
        let a_colors_bg = self.get_resource_image("IMAGE_ZOMBATAR_COLORS_BG");
        if !a_colors_bg.is_null() {
            g.draw_image_xy(unsafe { &*a_colors_bg }, ZOMBATAR_COLORS_X, ZOMBATAR_COLORS_Y);
        }
        if self.page == ZombatarPage::Skin {
            self.draw_color_swatches(g, 0, ZOMBATAR_SKIN_COLOR_COUNT, self.color[ZOMBATAR_PAGE_SKIN]);
        } else {
            let a_mode = if self.part[self.page as usize] < 0 {
                ZOMBATAR_COLOR_MODE_NONE
            } else {
                get_part_color_mode(self.page, self.part[self.page as usize])
            };
            if a_mode == ZOMBATAR_COLOR_MODE_NONE {
                let a_key = if self.part[self.page as usize] < 0 {
                    "[ZOMBATAR_COLOR_ITEM_NOT_CHOSEN]"
                } else {
                    "[ZOMBATAR_COLOR_NOT_APPLICABLE]"
                };
                let a_colors_w = self.get_image_width("IMAGE_ZOMBATAR_COLORS_BG", 261);
                let a_colors_h = self.get_image_height("IMAGE_ZOMBATAR_COLORS_BG", 96);
                self.draw_string_wrapped(
                    g,
                    a_key,
                    &Rect::new(ZOMBATAR_COLOR_HINT_X, ZOMBATAR_COLOR_HINT_Y, a_colors_w - 40, a_colors_h),
                    0, // FONT_BRIANNETOD12
                    &Color::WHITE,
                    DS_ALIGN_LEFT,
                );
            } else {
                self.draw_color_swatches(g, zombatar_color_base_for_mode(a_mode), ZOMBATAR_PART_COLOR_COUNT, self.color[self.page as usize]);
            }
        }

        // 分页文本（对应 C++ StrFormat("Page %d / %d", ...)）
        if self.max_sub_pages > 0 {
            let a_page_text = format!("Page {} / {}", self.sub_page + 1, self.max_sub_pages + 1);
            g.set_font(self.font(0));
            g.set_color(&Color::WHITE);
            g.draw_string(&a_page_text, 321, 441);
        }
    }

    /// 对应 C++ ZombatarWidget::DrawTransition
    fn draw_transition(&mut self, g: &mut Graphics) {
        self.draw_create(g);

        // C++: 页按钮随表单绘制（此处以禁用色调）
        if self.max_sub_pages > 0 {
            g.set_colorize_images(true);
            g.set_color(&ZOMBATAR_PAGE_BTN_DISABLED_TINT);
            let a_prev = self.get_resource_image("IMAGE_ZOMBATAR_PREV_BUTTON");
            let a_next = self.get_resource_image("IMAGE_ZOMBATAR_NEXT_BUTTON");
            if !a_prev.is_null() {
                g.draw_image_xy(unsafe { &*a_prev }, ZOMBATAR_PREV_PAGE_X, ZOMBATAR_PAGE_BTN_Y);
            }
            if !a_next.is_null() {
                g.draw_image_xy(unsafe { &*a_next }, ZOMBATAR_NEXT_PAGE_X, ZOMBATAR_PAGE_BTN_Y);
            }
            g.set_colorize_images(false);
        }

        let a_alpha = tod_animate_curve(
            ZOMBATAR_TRANSITION_TICKS,
            0,
            self.transition_timer,
            if self.state == ZombatarWidgetState::ToConfirm { 0 } else { 255 },
            if self.state == ZombatarWidgetState::ToConfirm { 255 } else { 0 },
            TodCurves::Linear,
        );
        g.set_color(&Color::new(0, 0, 0, a_alpha as u8));
        g.fill_rect_xywh(
            ZOMBATAR_VEIL_X,
            ZOMBATAR_VEIL_Y,
            ZOMBATAR_PANEL_WIDTH - ZOMBATAR_VEIL_RIGHT_INSET,
            ZOMBATAR_VEIL_HEIGHT,
        );
        g.set_color(&Color::WHITE);
    }

    /// 对应 C++ ZombatarWidget::DrawConfirm
    fn draw_confirm(&mut self, g: &mut Graphics) {
        let a_bg = self.get_resource_image("IMAGE_ZOMBATAR_WIDGET_BG");
        if !a_bg.is_null() {
            g.draw_image_xy(unsafe { &*a_bg }, ZOMBATAR_PANEL_X, ZOMBATAR_PANEL_Y);
        }

        // 头标题（FONT_HOUSEOFTERROR28，居中）
        let a_header = "[ZOMBATAR_FINISHED_WARNING_HEADER]";
        g.set_font(self.font(3));
        g.set_color(&Color::new(254, 227, 0, 255));
        let a_header_w = unsafe { (*self.font(3)).string_width(a_header) };
        g.draw_string(a_header, ZOMBATAR_CONFIRM_HEADER_X - a_header_w / 2, ZOMBATAR_CONFIRM_HEADER_Y);

        // 正文（FONT_CONTINUUMBOLD14，wrapped 居中）
        self.draw_string_wrapped(
            g,
            "[ZOMBATAR_FINISHED_WARNING_TEXT]",
            &Rect::new(
                ZOMBATAR_CONFIRM_TEXT_X,
                ZOMBATAR_CONFIRM_TEXT_Y,
                ZOMBATAR_CONFIRM_TEXT_WIDTH,
                ZOMBATAR_CONFIRM_TEXT_HEIGHT,
            ),
            4, // FONT_CONTINUUMBOLD14
            &Color::WHITE,
            DS_ALIGN_CENTER,
        );

        // 接受 / 返回标签（FONT_BRIANNETOD12，居中于按钮）
        g.set_font(self.font(0));
        g.set_color(&Color::WHITE);
        let a_accept = "[ZOMBATAR_FINISHED_BUTTON_TEXT]";
        let a_accept_w = unsafe { (*self.font(0)).string_width(a_accept) };
        g.draw_string(a_accept, ZOMBATAR_CONFIRM_ACCEPT_LABEL_X - a_accept_w / 2, ZOMBATAR_CONFIRM_LABEL_Y);
        let a_back = "[ZOMBATAR_BACK_BUTTON_TEXT]";
        let a_back_w = unsafe { (*self.font(0)).string_width(a_back) };
        g.draw_string(a_back, ZOMBATAR_CONFIRM_BACK_LABEL_X - a_back_w / 2, ZOMBATAR_CONFIRM_LABEL_Y);
    }

    /// [TRANSLATION_NOTE]: C++ PvzpDrawStringWrapped（PvzpStringFile.cpp 通用格式系统：
    /// UTF8 解码 + "{...}" 内联格式 + 断行）Rust 端未翻译（独立底层基础设施）；
    /// 此处按语义实现空格断行 + 矩形内堆叠 + 水平对齐，供 Zombatar 提示文字使用。
    fn draw_string_wrapped(&mut self, g: &mut Graphics, the_text: &str, the_rect: &Rect, the_font_kind: usize, the_color: &Color, the_justification: i32) {
        let a_font = self.font(the_font_kind);
        let mut a_lines: Vec<String> = Vec::new();
        let mut a_cur = String::new();
        for a_word in the_text.split(' ') {
            if a_cur.is_empty() {
                a_cur = a_word.to_string();
                continue;
            }
            let a_test = format!("{} {}", a_cur, a_word);
            if unsafe { (*a_font).string_width(&a_test) } <= the_rect.width {
                a_cur = a_test;
            } else {
                a_lines.push(a_cur);
                a_cur = a_word.to_string();
            }
        }
        if !a_cur.is_empty() {
            a_lines.push(a_cur);
        }
        g.set_font(a_font);
        let a_ascent = unsafe { (*a_font).get_ascent() };
        let mut a_y = the_rect.y + a_ascent;
        for a_line in &a_lines {
            let a_line_w = unsafe { (*a_font).string_width(a_line) };
            let a_x = if the_justification == DS_ALIGN_CENTER {
                the_rect.x + (the_rect.width - a_line_w) / 2
            } else {
                the_rect.x
            };
            g.set_color(the_color);
            g.draw_string(a_line, a_x, a_y);
            a_y += a_ascent + 2;
        }
    }
} // impl ZombatarWidget 结束

// ============================================================
// 布局常量（对应 C++ ZombatarWidget.cpp 顶部 constexpr，绘制用）
// ============================================================
/// 内层面板起点（对应 C++ ZOMBATAR_INNER_X/Y）
pub const ZOMBATAR_INNER_X: i32 = 152;
pub const ZOMBATAR_INNER_Y: i32 = 125;
/// 色板区起点（对应 C++ ZOMBATAR_COLORS_X/Y、ZOMBATAR_COLOR_HINT_X/Y）
pub const ZOMBATAR_COLORS_X: i32 = 221;
pub const ZOMBATAR_COLORS_Y: i32 = 335;
pub const ZOMBATAR_COLOR_HINT_X: i32 = 240;
pub const ZOMBATAR_COLOR_HINT_Y: i32 = 380;
/// 皮肤页起始说明文字（对应 C++ ZOMBATAR_START_TEXT_*）
pub const ZOMBATAR_START_TEXT_Y: i32 = 185;
pub const ZOMBATAR_START_TEXT_WIDTH: i32 = 500;
pub const ZOMBATAR_START_TEXT_HEIGHT: i32 = 100;
/// 网格内骷髅底边距（对应 C++ ZOMBATAR_CELL_ZOMBIE_MARGIN）
pub const ZOMBATAR_CELL_ZOMBIE_MARGIN: i32 = 10;
/// 图标对齐标志（对应 C++ ZOMBATAR_ALIGN_*）
pub const ZOMBATAR_ALIGN_TOP: i32 = 0x02;
pub const ZOMBATAR_ALIGN_BOTTOM: i32 = 0x04;
pub const ZOMBATAR_ALIGN_LEFT: i32 = 0x08;
pub const ZOMBATAR_ALIGN_RIGHT: i32 = 0x10;
/// 未选中且未悬停格子的调暗色（对应 C++ ZOMBATAR_CELL_DIM_COLOR）
pub const ZOMBATAR_CELL_DIM_COLOR: Color = Color { r: 0x80, g: 0x80, b: 0x80, a: 0x80 };
/// 列表模式（对应 C++ ZOMBATAR_LIST_*）
pub const ZOMBATAR_LIST_PORTRAIT_Y: i32 = 175;
pub const ZOMBATAR_LIST_SPACING: i32 = 40;
pub const ZOMBATAR_LIST_CENTER_INSET: i32 = 30;
pub const ZOMBATAR_LIST_CLIP_X: i32 = 58;
pub const ZOMBATAR_LIST_CLIP_Y: i32 = 125;
pub const ZOMBATAR_LIST_CLIP_RIGHT: i32 = 63;
pub const ZOMBATAR_LIST_CLIP_HEIGHT: i32 = 437;
pub const ZOMBATAR_LIST_FILL_HEIGHT: i32 = 331;
pub const ZOMBATAR_LIST_BLANK_WIDTH: i32 = 179;
pub const ZOMBATAR_LIST_COUNTER_X: i32 = 221;
pub const ZOMBATAR_LIST_COUNTER_Y: i32 = 161;
pub const ZOMBATAR_LIST_DELETE_X: i32 = 351;
pub const ZOMBATAR_LIST_DELETE_Y: i32 = 161;
/// 头像框（对应 C++ ZOMBATAR_AVATAR_*）
pub const ZOMBATAR_AVATAR_GROUND_X: i32 = 600;
pub const ZOMBATAR_AVATAR_GROUND_Y: i32 = 300;
pub const ZOMBATAR_AVATAR_ZOMBIE_X: i32 = 640;
pub const ZOMBATAR_AVATAR_ZOMBIE_Y: i32 = 350;
pub const ZOMBATAR_AVATAR_GROUND_CLIP_TOP: i32 = 10;
pub const ZOMBATAR_AVATAR_GROUND_CLIP_RIGHT: i32 = 40;
pub const ZOMBATAR_AVATAR_GROUND_CLIP_BOTTOM: i32 = 10;
/// 确认页（对应 C++ ZOMBATAR_CONFIRM_*）
pub const ZOMBATAR_CONFIRM_HEADER_X: i32 = 305;
pub const ZOMBATAR_CONFIRM_HEADER_Y: i32 = 185;
pub const ZOMBATAR_CONFIRM_TEXT_X: i32 = 60;
pub const ZOMBATAR_CONFIRM_TEXT_Y: i32 = 225;
pub const ZOMBATAR_CONFIRM_TEXT_WIDTH: i32 = 500;
pub const ZOMBATAR_CONFIRM_TEXT_HEIGHT: i32 = 100;
pub const ZOMBATAR_CONFIRM_ACCEPT_LABEL_X: i32 = 195;
pub const ZOMBATAR_CONFIRM_BACK_LABEL_X: i32 = 435;
pub const ZOMBATAR_CONFIRM_LABEL_Y: i32 = 335;
/// 过渡遮罩（对应 C++ ZOMBATAR_VEIL_*）
pub const ZOMBATAR_VEIL_X: i32 = 58;
pub const ZOMBATAR_VEIL_Y: i32 = 125;
pub const ZOMBATAR_VEIL_RIGHT_INSET: i32 = 63;
pub const ZOMBATAR_VEIL_HEIGHT: i32 = 331;
/// 分页按钮（对应 C++ ZOMBATAR_PREV/NEXT_PAGE_X、ZOMBATAR_PAGE_BTN_Y）
pub const ZOMBATAR_PREV_PAGE_X: i32 = 175;
pub const ZOMBATAR_NEXT_PAGE_X: i32 = 497;
pub const ZOMBATAR_PAGE_BTN_Y: i32 = 372;
pub const ZOMBATAR_PAGE_BTN_DISABLED_TINT: Color = Color { r: 108, g: 109, b: 140, a: 40 };
/// 文本对齐（对应 C++ DrawStringJustification 的 DS_ALIGN_LEFT/CENTER）
#[allow(non_upper_case_globals)]
pub const DS_ALIGN_LEFT: i32 = 0;
#[allow(non_upper_case_globals)]
pub const DS_ALIGN_CENTER: i32 = 1;

/// 列表删除矩形（对应 C++ ZOMBATAR_LIST_DELETE_RECT_*）
pub const ZOMBATAR_LIST_DELETE_RECT_X: i32 = 346;
pub const ZOMBATAR_LIST_DELETE_RECT_Y: i32 = 146;
pub const ZOMBATAR_LIST_DELETE_RECT_WIDTH: i32 = 60;
pub const ZOMBATAR_LIST_DELETE_RECT_HEIGHT: i32 = 20;
/// 过渡动画按钮坐标（对应 C++ ZOMBATAR_* 常量）
pub const ZOMBATAR_FINISHED_X: i32 = 445;
pub const ZOMBATAR_FINISHED_Y: i32 = 472;
pub const ZOMBATAR_ACCEPT_X: i32 = 155;
pub const ZOMBATAR_CONFIRM_BTN_Y: i32 = 345;

/// 页面索引转枚举（对应 C++ (ZombatarPage)i 循环转换）
pub fn page_for_index(the_index: usize) -> ZombatarPage {
    match the_index {
        ZOMBATAR_PAGE_HAIR => ZombatarPage::Hair,
        ZOMBATAR_PAGE_FACIAL_HAIR => ZombatarPage::FacialHair,
        ZOMBATAR_PAGE_TIDBITS => ZombatarPage::Tidbits,
        ZOMBATAR_PAGE_EYEWEAR => ZombatarPage::Eyewear,
        ZOMBATAR_PAGE_CLOTHES => ZombatarPage::Clothes,
        ZOMBATAR_PAGE_ACCESSORY => ZombatarPage::Accessory,
        ZOMBATAR_PAGE_HATS => ZombatarPage::Hats,
        _ => ZombatarPage::Backdrops,
    }
}