// PvZ Portable Rust 翻译 — CursorObject（光标对象/拖放）
// 对应 C++ src/Lawn/CursorObject.h / CursorObject.cpp

use crate::lawn::game_enums::*;
use crate::lawn::game_object::GameObject;
use crate::lawn::board::{Board, HitResult};
use crate::framework::graphics::graphics::Graphics;

/// 光标对象 — 用于拖放植物、工具等操作
/// 对应 C++ CursorObject : GameObject（完整字段映射）
pub struct CursorObject {
    // 基础信息
    pub cursor_type: CursorType,
    pub seed_type: SeedType,
    pub imitater_type: SeedType,

    // 选中索引/ID
    pub seed_bank_index: i32,       // 对应 mSeedBankIndex（种子槽索引，-1 表示无效）
    pub coin_id: CoinID,            // 对应 mCoinID（从阳光/硬币中取出的种子）
    pub glove_plant_id: PlantID,    // 对应 mGlovePlantID（手套抓起的植物）
    pub duplicator_plant_id: PlantID, // 对应 mDuplicatorPlantID（复制器复制的植物）
    pub cob_cannon_plant_id: PlantID, // 对应 mCobCannonPlantID（玉米加农炮瞄准）

    // 锤子/动画
    pub hammer_down_counter: i32,      // 对应 mHammerDownCounter（锤子按下计数）
    pub reanim_cursor_id: ReanimationID, // 对应 mReanimCursorID（光标上的动画实例）

    // 位置与可见性
    pub x: i32,
    pub y: i32,
    pub mouse_x: i32,          // 用于内部记录的鼠标原始坐标
    pub mouse_y: i32,
    pub width: i32,
    pub height: i32,
    pub active: bool,          // 是否处于激活状态
    pub visible: bool,         // 对应 mVisible（从 GameObject 继承）
}

impl CursorObject {
    /// 构造函数（对应 C++ CursorObject::CursorObject()）
    pub fn new() -> Self {
        CursorObject {
            cursor_type: CursorType::Normal,
            seed_type: SeedType::None,
            imitater_type: SeedType::None,
            seed_bank_index: -1,
            coin_id: COINID_NULL,
            glove_plant_id: PLANTID_NULL,
            duplicator_plant_id: PLANTID_NULL,
            cob_cannon_plant_id: PLANTID_NULL,
            hammer_down_counter: 0,
            reanim_cursor_id: REANIMATIONID_NULL,
            x: 0,
            y: 0,
            mouse_x: 0,
            mouse_y: 0,
            width: 80,
            height: 80,
            active: false,
            visible: false,
        }
    }

    /// 初始化拖放（对应 C++ 中设置 mType/mCursorType）
    pub fn init(&mut self, ctype: CursorType, seed_type: SeedType) {
        self.cursor_type = ctype;
        self.seed_type = seed_type;
        self.active = true;
        self.visible = true;
    }

    /// 更新光标位置（从鼠标坐标）
    pub fn update_position(&mut self, mx: i32, my: i32) {
        self.mouse_x = mx;
        self.mouse_y = my;
        self.x = mx - 25;
        self.y = my - 35;
    }

    /// 更新（对应 C++ CursorObject::Update() L48-L71）
    /// 需要 app 的 game_scene 和 mouse 状态，以及 board 的 cutscene
    pub fn update(&mut self, mouse_x: i32, mouse_y: i32, is_playing: bool, _is_in_shovel_tutorial: bool) {
        if !is_playing {
            self.visible = false;
            return;
        }

        // 更新位置（C++ 中使用 WidgetManager::mLastMouseX/mY）
        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;
        self.x = mouse_x - 25;
        self.y = mouse_y - 35;
        self.visible = true;
    }

    /// 销毁（对应 C++ CursorObject::Die() L73-L77）
    /// 移除关联的 Reanimation 实例
    pub fn die(&mut self) {
        // 移除光标上的 Reanimation 动画（对应 C++ mApp->RemoveReanimation(mReanimCursorID)）
        // 由于 App 引用不可达，此处仅重置 ID
        self.reanim_cursor_id = REANIMATIONID_NULL;
    }

    /// 完成拖放/清除（对应 C++ ClearCursor 中的重置逻辑）
    pub fn deactivate(&mut self) {
        self.active = false;
        self.visible = false;
        self.seed_type = SeedType::None;
        self.cursor_type = CursorType::Normal;
        self.seed_bank_index = -1;
        self.coin_id = COINID_NULL;
        self.duplicator_plant_id = PLANTID_NULL;
        self.cob_cannon_plant_id = PLANTID_NULL;
        self.glove_plant_id = PLANTID_NULL;
        self.reanim_cursor_id = REANIMATIONID_NULL;
    }

    /// 绘制（对应 C++ CursorObject::Draw() L79-L224）
    /// 根据光标类型绘制不同的工具/植物图标
    pub fn draw(&self, _g: &mut Graphics) {
        // 原始 C++ 实现根据 mCursorType 分支绘制不同图像：
        //
        // CURSOR_TYPE_SHOVEL         → DrawImage(IMAGE_SHOVEL)
        // CURSOR_TYPE_WATERING_CAN   → DrawImage(IMAGE_WATERINGCAN / IMAGE_ZEN_GOLDTOOLRETICLE)
        // CURSOR_TYPE_FERTILIZER     → DrawImage(IMAGE_FERTILIZER)
        // CURSOR_TYPE_BUG_SPRAY      → DrawImage(IMAGE_BUG_SPRAY)
        // CURSOR_TYPE_PHONOGRAPH     → DrawImage(IMAGE_PHONOGRAPH)
        // CURSOR_TYPE_CHOCOLATE      → DrawImage(IMAGE_CHOCOLATE)
        // CURSOR_TYPE_GLOVE          → DrawImage(IMAGE_ZEN_GARDENGLOVE)
        // CURSOR_TYPE_MONEY_SIGN     → DrawImage(IMAGE_ZEN_MONEYSIGN)
        // CURSOR_TYPE_TREE_FOOD      → DrawImage(IMAGE_TREEFOOD)
        // CURSOR_TYPE_WHEEELBARROW   → DrawImage(IMAGE_ZEN_WHEELBARROW) + DrawPottedPlant
        // CURSOR_TYPE_PLANT_FROM_GLOVE → DrawPottedPlant
        // CURSOR_TYPE_PLANT_FROM_WHEEL_BARROW → DrawPottedPlant
        // CURSOR_TYPE_PLANT_FROM_BANK/FROM_USABLE_COIN/FROM_DUPLICATOR → DrawSeedType
        // CURSOR_TYPE_HAMMER         → ReanimationGet(mReanimCursorID)->Draw(g)
        // CURSOR_TYPE_COBCANNON_TARGET → MouseHitTest → DrawImageCel(IMAGE_COBCANNON_TARGET)
        // CURSOR_TYPE_NORMAL         → 不绘制
        //
        // 依赖：IMAGE_* 资源常量（Resources.h/Resources.cpp）和 ZenGarden::DrawPottedPlant
        // 待资源管理和 ZenGarden 翻译完成后补充完整实现
    }

    /// 是否需要更新光标（对应 C++ 中 mApp->SetCursor 调用）
    /// 返回当前光标的系统指针类型
    pub fn get_cursor_type_for_app(&self) -> i32 {
        match self.cursor_type {
            CursorType::Shovel | CursorType::WateringCan |
            CursorType::Fertilizer | CursorType::BugSpray |
            CursorType::Phonograph | CursorType::Chocolate |
            CursorType::Glove | CursorType::MoneySign |
            CursorType::TreeFood | CursorType::Wheelbarrow |
            CursorType::PlantFromGlove | CursorType::PlantFromWheelBarrow |
            CursorType::PlantFromBank | CursorType::PlantFromUsableCoin |
            CursorType::PlantFromDuplicator => {
                // CURSOR_HAND / CURSOR_DRAGGING
                0
            }
            CursorType::Hammer | CursorType::CobcannonTarget => {
                // CURSOR_NONE（隐藏系统指针）
                -1
            }
            CursorType::Normal => {
                // CURSOR_POINTER
                1
            }
        }
    }
}

impl Default for CursorObject {
    fn default() -> Self {
        CursorObject::new()
    }
}

/// 光标预览 — 显示植物种植位置的半透明预览
/// 对应 C++ CursorPreview : GameObject
pub struct CursorPreview {
    pub base: GameObject,
    pub grid_x: i32,
    pub grid_y: i32,
}

impl CursorPreview {
    /// 构造函数（对应 C++ CursorPreview::CursorPreview()）
    pub fn new() -> Self {
        CursorPreview {
            base: GameObject {
                x: 0,
                y: 0,
                width: 80,
                height: 80,
                visible: false,
                ..GameObject::new()
            },
            grid_x: 0,
            grid_y: 0,
        }
    }

    /// 更新预览位置（对应 C++ CursorPreview::Update()）
    /// 根据鼠标位置和 Board 状态计算是否显示种植预览
    pub fn update(&mut self, mouse_x: i32, mouse_y: i32) {
        // 先提取所有需要的 Board/App 值到局部变量，避免借用冲突
        let (scene_ok, seed_type, grid_x, grid_y, show_preview, pixel_x, pixel_y) = {
            let board = match self.base.get_board() {
                Some(b) => b,
                None => {
                    self.base.visible = false;
                    return;
                }
            };
            let app = match self.base.get_app() {
                Some(a) => a,
                None => {
                    self.base.visible = false;
                    return;
                }
            };

            // 检查游戏场景
            if app.game_scene != crate::lawn::lawn_app::GameScenes::Playing {
                (false, SeedType::None, 0, 0, false, 0, 0)
            } else {
                // 获取当前种子类型
                let seed_type = board.get_seed_type_in_cursor_simple();
                let gx = board.planting_pixel_to_grid_x(mouse_x, mouse_y, seed_type);
                let gy = board.planting_pixel_to_grid_y(mouse_x, mouse_y, seed_type);

                let mut show = false;
                if gx >= 0
                    && (gx as usize) < crate::lawn::board::MAX_GRID_SIZE_X
                    && gy >= 0
                    && (gy as usize) <= crate::lawn::board::MAX_GRID_SIZE_Y
                {
                    if board.is_plant_in_cursor()
                        && board.can_plant_at(gx, gy, seed_type) == crate::lawn::game_enums::PlantingReason::Ok
                    {
                        show = true;
                    }

                    if show {
                        let px = board.grid_to_pixel_x(gx, gy);
                        let py = board.grid_to_pixel_y(gx, gy);
                        (true, seed_type, gx, gy, true, px, py)
                    } else {
                        (true, seed_type, gx, gy, false, 0, 0)
                    }
                } else {
                    (true, seed_type, gx, gy, false, 0, 0)
                }
            }
        }; // 作用域结束，board/app 借用被释放

        if !scene_ok {
            self.base.visible = false;
            return;
        }

        self.grid_x = grid_x;
        self.grid_y = grid_y;

        if show_preview {
            self.base.x = pixel_x;
            self.base.y = pixel_y;
            self.base.visible = true;
        } else {
            self.base.visible = false;
        }
    }

    /// 绘制种植预览（对应 C++ CursorPreview::Draw()）
    /// 在种植位置显示半透明的植物图像
    pub fn draw(&self, g: &mut Graphics) {
        let board = match self.base.get_board() {
            Some(b) => b,
            None => return,
        };
        let app = match self.base.get_app() {
            Some(a) => a,
            None => return,
        };

        // 获取种子类型
        let seed_type = board.get_seed_type_in_cursor_simple();
        if seed_type == SeedType::None {
            return;
        }

        // 设置半透明颜色覆盖
        g.set_colorize_images(true);
        g.set_color(&crate::framework::color::Color {
            r: 255,
            g: 255,
            b: 255,
            a: 100,
        });

        // 手推车/手套模式下的盆栽植物绘制（依赖 ZenGarden，暂简化）
        // 原始 C++ 代码在此处检查 CURSOR_TYPE_WHEEELBARROW 和 CURSOR_TYPE_PLANT_FROM_GLOVE
        // 并调用 ZenGarden::DrawPottedPlant()，因依赖未翻译暂跳过

        // 普通种子类型的绘制
        let is_izombie = app.is_izombie_level();
        let offset_x;
        let offset_y;
        if is_izombie {
            // IZombie 模式偏移
            let height = self.plant_draw_height_offset(board, seed_type, self.grid_x, self.grid_y);
            offset_y = height - 78.0;
            offset_x = -49.0;
        } else {
            offset_y = self.plant_draw_height_offset(board, seed_type, self.grid_x, self.grid_y);
            offset_x = 0.0;
        }

        // 绘制种子类型预览
        plant_draw_seed_type(
            g,
            seed_type,
            self.grid_x,
            self.grid_y,
            offset_x,
            offset_y,
            board,
        );

        // COLUMN 挑战模式：在每一列都显示预览
        if app.game_mode == crate::lawn::game_enums::GameMode::ChallengeColumns {
            for y in 0..crate::lawn::board::MAX_GRID_SIZE_Y {
                let y_i32 = y as i32;
                if y_i32 != self.grid_y
                    && board.can_plant_at(self.grid_x, y_i32, seed_type)
                        == crate::lawn::game_enums::PlantingReason::Ok
                {
                    let y_offset = 85.0 * (y_i32 - self.grid_y) as f32
                        + self.plant_draw_height_offset(board, seed_type, self.grid_x, y_i32);
                    plant_draw_seed_type(g, seed_type, self.grid_x, y_i32, 0.0, y_offset, board);
                }
            }
        }

        // 恢复颜色绘制
        g.set_colorize_images(false);
    }

    /// 计算植物绘制高度偏移（简化版 PlantDrawHeightOffset）
    /// 原始 C++ 实现在 Plant.cpp 中，此处做基本计算
    fn plant_draw_height_offset(
        &self,
        _board: &Board,
        _seed_type: SeedType,
        _grid_x: i32,
        _grid_y: i32,
    ) -> f32 {
        // 完整实现在 Plant.cpp 中基于种子类型和背景类型计算偏移
        // 此处返回默认值 0.0，待 PlantDrawHeightOffset 翻译后替换
        0.0
    }
}

impl Default for CursorPreview {
    fn default() -> Self {
        CursorPreview::new()
    }
}

/// 绘制种子类型预览（简化版，对应 C++ Plant::DrawSeedType）
/// 完整实现在 Plant.cpp 中，此处仅做占位
fn plant_draw_seed_type(
    _g: &mut Graphics,
    _seed_type: SeedType,
    _grid_x: i32,
    _grid_y: i32,
    _offset_x: f32,
    _offset_y: f32,
    _board: &Board,
) {
    // 依赖 Plant::DrawSeedType 翻译
    // 原始实现在 Plant.cpp 中根据种子类型从资源加载对应图像并绘制
    // 待 Plant 模块完整翻译后替换
    // 目前保留函数签名确保编译通过
}
