// PvZ Portable Rust 翻译 — CursorObject（光标对象/拖放）
// 对应 C++ src/Lawn/CursorObject.h / CursorObject.cpp

use crate::lawn::game_enums::*;
use crate::lawn::lawn_app::GameScenes as AppGameScenes;
use crate::lawn::game_object::GameObject;
use crate::lawn::board::{Board, MAX_GRID_SIZE_X, MAX_GRID_SIZE_Y};
use crate::framework::graphics::graphics::Graphics;
use crate::framework::color::Color;

/// 光标对象 — 用于拖放植物等操作
pub struct CursorObject {
    pub cursor_type: CursorType,
    pub seed_type: SeedType,
    pub imitater_type: SeedType,
    pub x: i32,
    pub y: i32,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub active: bool,
}

impl CursorObject {
    pub fn new() -> Self {
        CursorObject {
            cursor_type: CursorType::Normal,
            seed_type: SeedType::None,
            imitater_type: SeedType::None,
            x: 0,
            y: 0,
            mouse_x: 0,
            mouse_y: 0,
            active: false,
        }
    }

    /// 初始化拖放
    pub fn init(&mut self, ctype: CursorType, seed_type: SeedType) {
        self.cursor_type = ctype;
        self.seed_type = seed_type;
        self.active = true;
    }

    /// 更新位置
    pub fn update_position(&mut self, mx: i32, my: i32) {
        self.mouse_x = mx;
        self.mouse_y = my;
        self.x = mx - 25;
        self.y = my - 25;
    }

    /// 绘制
    pub fn draw(&self, _g: &mut Graphics) {}

    /// 完成拖放
    pub fn deactivate(&mut self) {
        self.active = false;
        self.cursor_type = CursorType::Normal;
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

            // 检查游戏场景：非 SCENE_PLAYING 且非铲子教程时隐藏
            if app.game_scene != AppGameScenes::Playing {
                (false, SeedType::None, 0, 0, false, 0, 0)
            } else {
                // 获取当前种子类型
                let seed_type = board.get_seed_type_in_cursor_simple();
                let gx = board.planting_pixel_to_grid_x(mouse_x, mouse_y, seed_type);
                let gy = board.planting_pixel_to_grid_y(mouse_x, mouse_y, seed_type);

                let mut show = false;
                if gx >= 0
                    && (gx as usize) < MAX_GRID_SIZE_X
                    && gy >= 0
                    && (gy as usize) <= MAX_GRID_SIZE_Y
                {
                    if board.is_plant_in_cursor()
                        && board.can_plant_at(gx, gy, seed_type) == PlantingReason::Ok
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
        g.set_color(&Color {
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
            // IZombie 模式偏移（Gargantuar 特殊偏移因 C++ 枚举未翻译到 SeedType 中，暂使用统一偏移）
            let height = self.plant_draw_height_offset(board, seed_type, self.grid_x, self.grid_y);
            offset_y = height - 78.0;
            offset_x = -49.0;
        } else {
            offset_y = self.plant_draw_height_offset(board, seed_type, self.grid_x, self.grid_y);
            offset_x = 0.0;
        }

        // 绘制种子类型预览
        PlantDrawSeedType(
            g,
            seed_type,
            self.grid_x,
            self.grid_y,
            offset_x,
            offset_y,
            board,
        );

        // COLUMN 挑战模式：在每一列都显示预览
        if app.game_mode == GameMode::ChallengeColumns {
            for y in 0..MAX_GRID_SIZE_Y {
                let y_i32 = y as i32;
                if y_i32 != self.grid_y
                    && board.can_plant_at(self.grid_x, y_i32, seed_type)
                        == PlantingReason::Ok
                {
                    let y_offset = 85.0 * (y_i32 - self.grid_y) as f32
                        + self.plant_draw_height_offset(board, seed_type, self.grid_x, y_i32);
                    PlantDrawSeedType(g, seed_type, self.grid_x, y_i32, 0.0, y_offset, board);
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
fn PlantDrawSeedType(
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
