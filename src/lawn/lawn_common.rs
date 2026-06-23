// PvZ Portable Rust 翻译 — LawnCommon（草地图标/常亮工具）
// 对应 C++ src/Lawn/LawnCommon.h / LawnCommon.cpp

use crate::lawn::game_enums::*;
use crate::lawn::plant::Plant;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::color::Color;

/// 游戏公用工具函数
pub struct LawnCommon;

impl LawnCommon {
    /// 根据种子类型获取图像
    pub fn get_seed_image(seed_type: SeedType) -> Option<&'static Image> {
        // 从资源管理器获取种子图标
        None
    }

    /// 获取种子价格
    pub fn get_seed_cost(seed_type: SeedType) -> i32 {
        Plant::get_cost(seed_type, SeedType::None)
    }

    /// 获取种子名称
    pub fn get_seed_name(seed_type: SeedType) -> &'static str {
        match seed_type {
            SeedType::Peashooter => "Peashooter",
            SeedType::Sunflower => "Sunflower",
            SeedType::Cherrybomb => "Cherry Bomb",
            SeedType::Wallnut => "Wall-Nut",
            SeedType::PotatoMine => "Potato Mine",
            SeedType::Snowpea => "Snow Pea",
            SeedType::Chomper => "Chomper",
            SeedType::Repeater => "Repeater",
            SeedType::Puffshroom => "Puff-shroom",
            SeedType::Sunshroom => "Sun-shroom",
            SeedType::Fumeshroom => "Fume-shroom",
            SeedType::Gravebuster => "Grave Buster",
            SeedType::Hypnoshroom => "Hypno-shroom",
            SeedType::Scaredyshroom => "Scaredy-shroom",
            SeedType::Iceshroom => "Ice-shroom",
            SeedType::Doomshroom => "Doom-shroom",
            SeedType::Lilypad => "Lily Pad",
            SeedType::Squash => "Squash",
            SeedType::Threepeater => "Threepeater",
            SeedType::Tanglekelp => "Tangle Kelp",
            SeedType::Jalapeno => "Jalapeno",
            SeedType::Spikeweed => "Spikeweed",
            SeedType::Torchwood => "Torchwood",
            SeedType::Tallnut => "Tall-Nut",
            SeedType::Seashroom => "Sea-shroom",
            SeedType::Plantern => "Plantern",
            SeedType::Cactus => "Cactus",
            SeedType::Blover => "Blover",
            SeedType::Splitpea => "Split Pea",
            SeedType::Starfruit => "Starfruit",
            SeedType::Pumpkinshell => "Pumpkin",
            SeedType::Magnetshroom => "Magnet-shroom",
            SeedType::Cabbagepult => "Cabbage-pult",
            SeedType::Kernelpult => "Kernel-pult",
            SeedType::InstantCoffee => "Coffee Bean",
            SeedType::Garlic => "Garlic",
            SeedType::Umbrella => "Umbrella Leaf",
            SeedType::Marigold => "Marigold",
            SeedType::Melonpult => "Melon-pult",
            SeedType::Gatlingpea => "Gatling Pea",
            SeedType::Twinsunflower => "Twin Sunflower",
            SeedType::Gloomshroom => "Gloom-shroom",
            SeedType::Cattail => "Cattail",
            SeedType::Wintermelon => "Winter Melon",
            SeedType::GoldMagnet => "Gold Magnet",
            SeedType::Spikerock => "Spikerock",
            SeedType::Cobcannon => "Cob Cannon",
            SeedType::Imitater => "Imitater",
            SeedType::ExplodeONut => "Explode-o-Nut",
            SeedType::GiantWallnut => "Giant Wall-Nut",
            _ => "Unknown",
        }
    }
}
