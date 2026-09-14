// PvZ Portable Rust 翻译 — ImitaterDialog（模仿者选择对话框）
// 对应 C++ src/Lawn/Widget/ImitaterDialog.h / ImitaterDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::rect::Rect;
use crate::lawn::game_enums::*;
use crate::lawn::lawn_app::LawnApp;
use crate::lawn::tool_tip_widget::ToolTipWidget;

/// 模仿者对话框 — 选择模仿者要模仿的植物
/// 对应 C++ ImitaterDialog : LawnDialog
pub struct ImitaterDialog {
    // 继承自 LawnDialog / Widget 的字段
    pub app: Option<*mut LawnApp>,
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
    pub clip: bool,

    // ImitaterDialog 特有字段
    pub tool_tip: Option<Box<ToolTipWidget>>,
    pub tool_tip_seed: SeedType,
}

impl ImitaterDialog {
    /// 构造函数（对应 C++ ImitaterDialog::ImitaterDialog() L32-L43）
    pub fn new(app: *mut LawnApp, width: i32, height: i32) -> Self {
        let mut dialog = ImitaterDialog {
            app: Some(app),
            id: 0,           // 对应 DIALOG_IMITATER
            x: 0,
            y: 0,
            width,
            height,
            visible: true,
            clip: false,
            tool_tip: Some(Box::new(ToolTipWidget::new())),
            tool_tip_seed: SeedType::None,
        };

        // C++ 中调用 CalcSize(IMITATER_DIALOG_WIDTH - mWidth, IMITATER_DIALOG_HEIGHT - mHeight)
        // 此处 width/height 已通过参数传入
        dialog.calc_size(width - dialog.width, height - dialog.height);

        dialog
    }

    /// 计算尺寸（对应 C++ LawnDialog::CalcSize 简化版）
    fn calc_size(&mut self, _extra_x: i32, _extra_y: i32) {
        // C++ 中计算实际对话框尺寸，暂简化
    }

    /// 种子命中检测（对应 C++ SeedHitTest L50-L65）
    /// 遍历所有种子类型检查鼠标点击是否落在种子包区域内
    pub fn seed_hit_test(&self, x: i32, y: i32) -> SeedType {
        let app = match self.app {
            Some(a) => unsafe { &*a },
            None => return SeedType::None,
        };

        // 遍历 SEED_PEASHOOTER 到 SEED_GATLINGPEA 之间的所有种子类型
        for seed_val in 0..(SeedType::Gatlingpea as i32) {
            let seed_type = match num_to_seed_type(seed_val) {
                Some(st) => st,
                None => continue,
            };
            if app.has_seed_type(seed_type) {
                let (seed_x, seed_y) = self.get_seed_position(seed_type);
                let r = Rect::new(seed_x, seed_y, SEED_PACKET_WIDTH, SEED_PACKET_HEIGHT);
                if r.contains(x, y) {
                    return seed_type;
                }
            }
        }
        SeedType::None
    }

    /// 更新光标（对应 C++ UpdateCursor L67-L78）
    pub fn update_cursor(&self) {
        // 简化版：C++ 中通过 mApp->SetCursor(CURSOR_HAND/CURSOR_POINTER)
        // 依赖系统光标管理
    }

    /// 更新（对应 C++ Update L80-L85）
    pub fn update(&mut self) {
        // C++: LawnDialog::Update(); ShowToolTip(); UpdateCursor();
        self.show_tool_tip();
    }

    /// 获取种子位置（对应 C++ GetSeedPosition L87-L91）
    pub fn get_seed_position(&self, index: SeedType) -> (i32, i32) {
        let idx = index as i32;
        let x = (idx % 8) * (SEED_PACKET_WIDTH + 1) + self.width / 2 - 210;
        let y = (idx / 8) * (SEED_PACKET_HEIGHT + 1) + 112;
        (x, y)
    }

    /// 绘制（对应 C++ ImitaterDialog::Draw，ImitaterDialog.cpp:93）
    pub fn draw(&self, g: &mut Graphics) {
        // [TRANSLATION_NOTE]: C++ 先调 LawnDialog::Draw 画对话框背景（IMAGE_IMITATERDIALOG 等
        // 资源未接入时跳过背景）；种子包按对话框局部坐标 + self.x/self.y 偏移绘制
        let a_back = self.app.map_or(std::ptr::null_mut(), |app| unsafe {
            let a_rm = match (*app).base.resource_manager {
                Some(r) => r,
                None => return std::ptr::null_mut(),
            };
            (*a_rm).get_image("IMAGE_IMITATERDIALOG").as_image_ptr()
        });
        if !a_back.is_null() {
            unsafe { g.draw_image_xy(&*a_back, self.x, self.y); }
        }
        g.set_linear_blend(true);

        // C++: for (SeedType aSeedType = 0; aSeedType < SEED_GATLINGPEA; aSeedType++)
        //      if (mApp->HasSeedType) DrawSeedPacket(g, x, y, SEED_IMITATER, aSeedType, 0, aGrayed?115:255, true, false)
        let Some(app) = self.app else { return };
        unsafe {
            let a_seed_chooser = (*app).seed_chooser_screen.unwrap_or(std::ptr::null_mut()) as *mut crate::lawn::widget::seed_chooser_screen::SeedChooserScreen;
            for a_seed_val in 0..(SeedType::Gatlingpea as i32) {
                let a_seed_type = match num_to_seed_type(a_seed_val) {
                    Some(st) => st,
                    None => continue,
                };
                if (*app).has_seed_type(a_seed_type) {
                    let (a_seed_x, a_seed_y) = self.get_seed_position(a_seed_type);
                    // C++: aGrayed = mSeedChooserScreen->SeedNotAllowedToPick || SeedNotRecommendedToPick
                    let a_grayed = if a_seed_chooser.is_null() {
                        false
                    } else {
                        (*a_seed_chooser).seed_not_allowed_to_pick(a_seed_type)
                            || (*a_seed_chooser).seed_not_recommended_to_pick(a_seed_type) != 0
                    };
                    crate::lawn::seed_packet::draw_seed_packet(
                        g, (self.x + a_seed_x) as f32, (self.y + a_seed_y) as f32,
                        SeedType::Imitater, a_seed_type, 0.0, if a_grayed { 115 } else { 255 }, true, false,
                    );
                }
            }
        }

        // C++: mToolTip->Draw(g)
        if let Some(ref tip) = self.tool_tip {
            if tip.m_visible {
                tip.draw(g);
            }
        }
    }

    /// 显示工具提示（对应 C++ ShowToolTip L110-L156）
    pub fn show_tool_tip(&mut self) {
        // C++ 中检查 mMouseIn 和 mActive 和 app 状态
        let seed_type = self.seed_hit_test(0, 0);
        if seed_type == SeedType::None {
            self.remove_tool_tip();
        } else if seed_type != self.tool_tip_seed {
            self.remove_tool_tip();

            // 预先计算位置
            let (seed_x, seed_y) = self.get_seed_position(seed_type);

            // C++ 中设置警告文本，暂简化
            if let Some(ref mut tip) = self.tool_tip {
                // C++: mToolTip->SetTitle(Plant::GetNameString(SEED_IMITATER, aSeedType));
                // C++: mToolTip->SetLabel(Plant::GetToolTip(aSeedType));
                tip.set_title(&crate::lawn::plant::Plant::get_name_string(SeedType::Imitater, seed_type));
                tip.set_label(&crate::lawn::plant::Plant::get_tool_tip(seed_type));
                tip.m_x = (SEED_PACKET_WIDTH - tip.m_width) / 2 + seed_x;
                tip.m_y = SEED_PACKET_HEIGHT + seed_y;
                tip.m_visible = true;
            }
            self.tool_tip_seed = seed_type;
        }
    }

    /// 移除工具提示（对应 C++ RemoveToolTip L158-L162）
    pub fn remove_tool_tip(&mut self) {
        if let Some(ref mut tip) = self.tool_tip {
            tip.m_visible = false;
        }
        self.tool_tip_seed = SeedType::None;
    }

    /// 鼠标按下（对应 C++ ImitaterDialog::MouseDown，ImitaterDialog.cpp:164）
    pub fn mouse_down(&mut self, x: i32, y: i32, _click_count: i32) {
        let a_seed_type = self.seed_hit_test(x, y);
        if a_seed_type != SeedType::None {
            let Some(app) = self.app else { return };
            unsafe {
                let a_seed_chooser = (*app).seed_chooser_screen.unwrap_or(std::ptr::null_mut()) as *mut crate::lawn::widget::seed_chooser_screen::SeedChooserScreen;
                if a_seed_chooser.is_null() {
                    return;
                }
                // C++: if (!aSeedChooser->SeedNotAllowedToPick(aSeedType))
                if !(*a_seed_chooser).seed_not_allowed_to_pick(a_seed_type) {
                    // C++: ChosenSeed& aImitater = aSeedChooser->mChosenSeeds[SEED_IMITATER];
                    //      mSeedState = SEED_IN_CHOOSER; mImitaterType = aSeedType;
                    //      mX/mY = mImitaterButton 位置
                    let a_imitater_idx = SeedType::Imitater as usize;
                    let a_button_pos = (*a_seed_chooser).imitater_button.map_or((0, 0), |b| unsafe {
                        ((*b).x, (*b).y)
                    });
                    // nightly 禁止裸指针隐式 autoref：先显式解引用
                    let a_chooser_ref = &mut *a_seed_chooser;
                    let a_imitater_ptr = match a_chooser_ref.chosen_seeds.get_mut(a_imitater_idx) {
                        Some(s) => s as *mut crate::lawn::widget::seed_chooser_screen::ChosenSeed,
                        None => return,
                    };
                    (*a_imitater_ptr).seed_state = ChosenSeedState::InChooser;
                    (*a_imitater_ptr).imitater_type = a_seed_type;
                    (*a_imitater_ptr).x = a_button_pos.0;
                    (*a_imitater_ptr).y = a_button_pos.1;
                    // C++: aSeedChooser->ClickedSeedInChooser(aImitater); UpdateImitaterButton();
                    (*a_seed_chooser).clicked_seed_in_chooser(&mut *a_imitater_ptr);
                    (*a_seed_chooser).update_imitater_button();
                    // C++: mApp->KillDialog(mId) —— Rust 由持有方（update 转发）检测 visible 释放
                    self.visible = false;
                }
            }
        }
        // C++ else: Widget::MouseDown —— Rust 无 Widget 基类，等效空操作
    }
}

fn num_to_seed_type(val: i32) -> Option<SeedType> {
    // 对应 SeedType 的枚举值 0..=39（SEED_PEASHOOTER 到 SEED_MELONPULT）
    match val {
        0 => Some(SeedType::Peashooter),
        1 => Some(SeedType::Sunflower),
        2 => Some(SeedType::Cherrybomb),
        3 => Some(SeedType::Wallnut),
        4 => Some(SeedType::PotatoMine),
        5 => Some(SeedType::Snowpea),
        6 => Some(SeedType::Chomper),
        7 => Some(SeedType::Repeater),
        8 => Some(SeedType::Puffshroom),
        9 => Some(SeedType::Sunshroom),
        10 => Some(SeedType::Fumeshroom),
        11 => Some(SeedType::Gravebuster),
        12 => Some(SeedType::Hypnoshroom),
        13 => Some(SeedType::Scaredyshroom),
        14 => Some(SeedType::Iceshroom),
        15 => Some(SeedType::Doomshroom),
        16 => Some(SeedType::Lilypad),
        17 => Some(SeedType::Squash),
        18 => Some(SeedType::Threepeater),
        19 => Some(SeedType::Tanglekelp),
        20 => Some(SeedType::Jalapeno),
        21 => Some(SeedType::Spikeweed),
        22 => Some(SeedType::Torchwood),
        23 => Some(SeedType::Tallnut),
        24 => Some(SeedType::Seashroom),
        25 => Some(SeedType::Plantern),
        26 => Some(SeedType::Cactus),
        27 => Some(SeedType::Blover),
        28 => Some(SeedType::Splitpea),
        29 => Some(SeedType::Starfruit),
        30 => Some(SeedType::Pumpkinshell),
        31 => Some(SeedType::Magnetshroom),
        32 => Some(SeedType::Cabbagepult),
        33 => Some(SeedType::Flowerpot),
        34 => Some(SeedType::Kernelpult),
        35 => Some(SeedType::InstantCoffee),
        36 => Some(SeedType::Garlic),
        37 => Some(SeedType::Umbrella),
        38 => Some(SeedType::Marigold),
        39 => Some(SeedType::Melonpult),
        _ => None,
    }
}
