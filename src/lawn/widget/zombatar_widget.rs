// PvZ Portable Rust 翻译 — ZombatarWidget（僵尸头像定制界面）
// 对应 C++ src/Lawn/Widget/ZombatarWidget.h / ZombatarWidget.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::key_codes::{KEYCODE_ESCAPE, KeyCode};
use crate::framework::rect::Rect;
use crate::framework::widget::dialog::{BUTTONS_YES_NO, ID_YES};
use crate::framework::widget::widget_manager::WidgetManager;
use crate::lawn::game_enums::*;
use crate::lawn::zombatar::{
    zombatar_read_signed_record_slot, zombatar_read_record_slot, ZombatarPage,
};
use crate::lawn::widget::game_selector::GameSelectorImpl;

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

    /// 对应 C++ ZombatarWidget::ShowMaxHeadsMessage —— [TRANSLATION_NOTE]: 弹窗占位
    pub fn show_max_heads_message(&mut self) {}

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

    /// 对应 C++ ZombatarWidget::GetBackgroundImage —— [TRANSLATION_NOTE]: 图片未接入返回 null
    pub fn get_background_image(&self, the_index: i32) -> *mut Image {
        let _ = the_index.clamp(0, 4); // C++: aImages[clamp(index, 0, 4)]
        std::ptr::null_mut()
    }

    /// 对应 C++ ZombatarWidget::GetPartImage —— [TRANSLATION_NOTE]: 图片未接入返回 null，索引修正保留
    pub fn get_part_image(&self, the_page: ZombatarPage, the_index: i32) -> *mut Image {
        if the_index < 0 {
            return std::ptr::null_mut();
        }
        match the_page {
            ZombatarPage::Clothes => { let _ = the_index < 12; }
            ZombatarPage::Hats => { let _ = the_index < 14; }
            ZombatarPage::Hair => { let _ = the_index < 16; }
            ZombatarPage::Eyewear => { let _ = the_index < 16; }
            ZombatarPage::FacialHair => {
                let mut a_idx = the_index;
                if a_idx > 16 {
                    a_idx -= a_idx / 17;
                }
                let _ = a_idx < 24;
            }
            ZombatarPage::Tidbits => { let _ = the_index < 14; }
            ZombatarPage::Accessory => { let _ = the_index < 15; }
            _ => {}
        }
        std::ptr::null_mut()
    }

    /// 对应 C++ ZombatarWidget::GetPartMaskImage —— [TRANSLATION_NOTE]: 图片未接入返回 null
    pub fn get_part_mask_image(&self, the_page: ZombatarPage, the_index: i32) -> *mut Image {
        if the_index < 0 {
            return std::ptr::null_mut();
        }
        let _ = the_page;
        let _ = the_index;
        std::ptr::null_mut()
    }

    /// 对应 C++ ZombatarWidget::GetCategoryImage —— [TRANSLATION_NOTE]: 图片未接入返回 null
    pub fn get_category_image(&self, _the_page: ZombatarPage, _the_selected: bool, _the_over: bool) -> *mut Image {
        std::ptr::null_mut()
    }

    /// [TRANSLATION_NOTE]: C++ ZombatarWidget::CreatePreviewZombie —— 预览僵尸创建占位
    pub fn create_preview_zombie(&mut self) {}

    /// [TRANSLATION_NOTE]: C++ ZombatarWidget::DestroyPreviewZombie —— 预览僵尸销毁占位
    pub fn destroy_preview_zombie(&mut self) {}

    /// [TRANSLATION_NOTE]: C++ ZombatarWidget::Update —— transition_timer 逻辑完整，按钮/预览占位
    pub fn update(&mut self) {
        // C++: Widget::Update() + 预览僵尸生命周期（未接入，占位）
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
}

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