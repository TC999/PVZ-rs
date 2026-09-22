// PvZ Portable Rust 翻译 — CreditScreen（制作人员列表）
// 对应 C++ src/Lawn/Widget/CreditScreen.h / CreditScreen.cpp

#![allow(dead_code)]

use crate::framework::widget::widget::{Widget, WidgetImpl};
use crate::framework::graphics::graphics::Graphics;
use crate::lawn::system::music::MusicTune;
use crate::todlib::tod_foley::FoleyType;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::KeyCode;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::color::Color;
use crate::lawn::game_enums::*;
use crate::lawn::widget::game_button::GameButton;
use crate::todlib::reanimator::Reanimation;

/// 制作人员阶段（对应 C++ CreditsPhase）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditsPhase {
    Main1,
    Main2,
    Main3,
    End,
}

/// 制作人员图层（对应 C++ CreditLayer）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditLayer {
    Background,
    Zombie,
    Top,
}

/// 文字类型（对应 C++ CreditWordType）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditWordType {
    Aa,
    Ee,
    Aw,
    Oh,
    Off,
}

/// 脑子动画类型（对应 C++ CreditBrainType）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditBrainType {
    FlyOn,
    FastOn,
    NextWord,
    FastOff,
    FlyOff,
    Off,
}

/// 制作人员时间控制（对应 C++ CreditsTiming）
pub struct CreditsTiming {
    pub frame: f32,
    pub word_type: CreditWordType,
    pub word_x: i32,
    pub brain_type: CreditBrainType,
}

pub static CREDITS_TIMING: [CreditsTiming; 268] = [
    CreditsTiming { frame: 128.5, word_type: CreditWordType::Aw, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 133.0, word_type: CreditWordType::Oh, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 136.5, word_type: CreditWordType::Ee, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 140.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 141.0, word_type: CreditWordType::Aw, word_x: 214, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 143.0, word_type: CreditWordType::Aw, word_x: 297, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 145.0, word_type: CreditWordType::Aw, word_x: 348, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 149.0, word_type: CreditWordType::Ee, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 153.0, word_type: CreditWordType::Aw, word_x: 455, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 155.0, word_type: CreditWordType::Oh, word_x: 523, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 159.0, word_type: CreditWordType::Aw, word_x: 593, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 163.0, word_type: CreditWordType::Aw, word_x: 619, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 171.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 172.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 173.0, word_type: CreditWordType::Aw, word_x: 214, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 175.0, word_type: CreditWordType::Aw, word_x: 297, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 177.0, word_type: CreditWordType::Aw, word_x: 348, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 181.0, word_type: CreditWordType::Ee, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 185.0, word_type: CreditWordType::Aw, word_x: 455, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 187.0, word_type: CreditWordType::Oh, word_x: 523, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 191.0, word_type: CreditWordType::Aw, word_x: 593, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 193.0, word_type: CreditWordType::Aw, word_x: 619, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 199.0, word_type: CreditWordType::Aa, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 203.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 205.0, word_type: CreditWordType::Aw, word_x: 214, brain_type: CreditBrainType::FlyOn },
    CreditsTiming { frame: 207.0, word_type: CreditWordType::Aw, word_x: 297, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 209.0, word_type: CreditWordType::Aw, word_x: 348, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 213.0, word_type: CreditWordType::Ee, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 217.0, word_type: CreditWordType::Aw, word_x: 455, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 219.0, word_type: CreditWordType::Oh, word_x: 523, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 223.0, word_type: CreditWordType::Aw, word_x: 593, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 227.0, word_type: CreditWordType::Aw, word_x: 619, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 231.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 234.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 235.0, word_type: CreditWordType::Ee, word_x: 150, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 237.0, word_type: CreditWordType::Oh, word_x: 220, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 239.0, word_type: CreditWordType::Aw, word_x: 307, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 241.0, word_type: CreditWordType::Aw, word_x: 390, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 245.0, word_type: CreditWordType::Ee, word_x: 452, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 249.0, word_type: CreditWordType::Aw, word_x: 512, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 251.0, word_type: CreditWordType::Aw, word_x: 573, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 255.0, word_type: CreditWordType::Aw, word_x: 630, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 257.0, word_type: CreditWordType::Aw, word_x: 656, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 261.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 262.0, word_type: CreditWordType::Aa, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 266.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 266.5, word_type: CreditWordType::Aw, word_x: 96, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 268.5, word_type: CreditWordType::Oh, word_x: 154, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 270.5, word_type: CreditWordType::Oh, word_x: 244, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 272.5, word_type: CreditWordType::Aw, word_x: 329, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 276.5, word_type: CreditWordType::Aw, word_x: 419, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 279.5, word_type: CreditWordType::Aw, word_x: 506, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 281.5, word_type: CreditWordType::Aw, word_x: 597, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 284.5, word_type: CreditWordType::Aw, word_x: 671, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 286.5, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 287.0, word_type: CreditWordType::Oh, word_x: 48, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 288.0, word_type: CreditWordType::Aw, word_x: 125, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 290.0, word_type: CreditWordType::Oh, word_x: 193, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 291.0, word_type: CreditWordType::Ee, word_x: 254, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 294.5, word_type: CreditWordType::Aw, word_x: 318, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 295.0, word_type: CreditWordType::Aw, word_x: 375, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 296.0, word_type: CreditWordType::Aw, word_x: 438, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 297.0, word_type: CreditWordType::Aw, word_x: 480, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 299.0, word_type: CreditWordType::Aw, word_x: 556, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 301.0, word_type: CreditWordType::Aw, word_x: 619, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 303.0, word_type: CreditWordType::Aw, word_x: 675, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 305.0, word_type: CreditWordType::Aw, word_x: 744, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 307.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 309.5, word_type: CreditWordType::Off, word_x: 207, brain_type: CreditBrainType::FlyOn },
    CreditsTiming { frame: 310.5, word_type: CreditWordType::Off, word_x: 287, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 311.5, word_type: CreditWordType::Off, word_x: 365, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 313.5, word_type: CreditWordType::Off, word_x: 435, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 315.5, word_type: CreditWordType::Off, word_x: 518, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 317.5, word_type: CreditWordType::Off, word_x: 603, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 318.5, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FastOff },
    CreditsTiming { frame: 319.5, word_type: CreditWordType::Off, word_x: 198, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 320.5, word_type: CreditWordType::Off, word_x: 264, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 322.5, word_type: CreditWordType::Off, word_x: 335, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 323.5, word_type: CreditWordType::Off, word_x: 411, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 324.5, word_type: CreditWordType::Off, word_x: 474, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 326.5, word_type: CreditWordType::Off, word_x: 527, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 328.5, word_type: CreditWordType::Off, word_x: 595, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 332.5, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 337.5, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 339.5, word_type: CreditWordType::Aw, word_x: 190, brain_type: CreditBrainType::FlyOn },
    CreditsTiming { frame: 340.5, word_type: CreditWordType::Aw, word_x: 260, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 342.5, word_type: CreditWordType::Aa, word_x: 314, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 344.5, word_type: CreditWordType::Aw, word_x: 364, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 347.5, word_type: CreditWordType::Aw, word_x: 426, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 349.5, word_type: CreditWordType::Oh, word_x: 474, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 350.5, word_type: CreditWordType::Aw, word_x: 538, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 352.5, word_type: CreditWordType::Ee, word_x: 606, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 353.5, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FastOff },
    CreditsTiming { frame: 354.5, word_type: CreditWordType::Ee, word_x: 187, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 356.5, word_type: CreditWordType::Aw, word_x: 242, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 358.5, word_type: CreditWordType::Oh, word_x: 280, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 359.5, word_type: CreditWordType::Aw, word_x: 340, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 360.5, word_type: CreditWordType::Aw, word_x: 394, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 361.5, word_type: CreditWordType::Ee, word_x: 439, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 363.5, word_type: CreditWordType::Aw, word_x: 500, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 364.5, word_type: CreditWordType::Aw, word_x: 550, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 366.5, word_type: CreditWordType::Ee, word_x: 606, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 369.5, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 371.5, word_type: CreditWordType::Off, word_x: 200, brain_type: CreditBrainType::FlyOn },
    CreditsTiming { frame: 372.5, word_type: CreditWordType::Oh, word_x: 258, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 374.5, word_type: CreditWordType::Off, word_x: 332, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 376.5, word_type: CreditWordType::Off, word_x: 416, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 378.5, word_type: CreditWordType::Off, word_x: 494, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 380.5, word_type: CreditWordType::Off, word_x: 576, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 381.5, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FastOff },
    CreditsTiming { frame: 382.5, word_type: CreditWordType::Off, word_x: 255, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 384.5, word_type: CreditWordType::Off, word_x: 322, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 386.5, word_type: CreditWordType::Off, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 388.5, word_type: CreditWordType::Off, word_x: 474, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 390.5, word_type: CreditWordType::Off, word_x: 533, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 394.5, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 522.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 523.0, word_type: CreditWordType::Aw, word_x: 214, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 525.0, word_type: CreditWordType::Aw, word_x: 297, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 527.0, word_type: CreditWordType::Aw, word_x: 348, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 531.0, word_type: CreditWordType::Ee, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 535.0, word_type: CreditWordType::Aw, word_x: 455, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 537.0, word_type: CreditWordType::Oh, word_x: 523, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 541.0, word_type: CreditWordType::Aw, word_x: 593, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 545.0, word_type: CreditWordType::Aw, word_x: 619, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 549.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 554.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 555.0, word_type: CreditWordType::Aw, word_x: 214, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 557.0, word_type: CreditWordType::Aw, word_x: 297, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 559.0, word_type: CreditWordType::Aw, word_x: 348, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 563.0, word_type: CreditWordType::Ee, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 567.0, word_type: CreditWordType::Aw, word_x: 455, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 569.0, word_type: CreditWordType::Oh, word_x: 523, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 573.0, word_type: CreditWordType::Aw, word_x: 593, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 575.0, word_type: CreditWordType::Aw, word_x: 619, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 581.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 582.0, word_type: CreditWordType::Aa, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 586.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 587.0, word_type: CreditWordType::Aw, word_x: 214, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 589.0, word_type: CreditWordType::Aw, word_x: 297, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 591.0, word_type: CreditWordType::Aw, word_x: 348, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 595.0, word_type: CreditWordType::Ee, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 599.0, word_type: CreditWordType::Aw, word_x: 455, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 601.0, word_type: CreditWordType::Oh, word_x: 523, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 605.0, word_type: CreditWordType::Aw, word_x: 593, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 609.0, word_type: CreditWordType::Aw, word_x: 619, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 613.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 616.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 617.0, word_type: CreditWordType::Ee, word_x: 150, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 619.0, word_type: CreditWordType::Oh, word_x: 220, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 621.0, word_type: CreditWordType::Aw, word_x: 307, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 623.0, word_type: CreditWordType::Aw, word_x: 390, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 627.0, word_type: CreditWordType::Ee, word_x: 452, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 631.0, word_type: CreditWordType::Aw, word_x: 512, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 633.0, word_type: CreditWordType::Aw, word_x: 573, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 637.0, word_type: CreditWordType::Aw, word_x: 630, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 639.0, word_type: CreditWordType::Aw, word_x: 656, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 643.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 644.0, word_type: CreditWordType::Aa, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 648.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 649.0, word_type: CreditWordType::Aa, word_x: 196, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 651.0, word_type: CreditWordType::Ee, word_x: 247, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 653.0, word_type: CreditWordType::Aw, word_x: 299, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 655.0, word_type: CreditWordType::Aw, word_x: 371, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 658.0, word_type: CreditWordType::Oh, word_x: 443, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 659.0, word_type: CreditWordType::Ee, word_x: 475, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 661.0, word_type: CreditWordType::Aw, word_x: 512, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 662.0, word_type: CreditWordType::Aa, word_x: 544, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 664.0, word_type: CreditWordType::Oh, word_x: 573, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 667.0, word_type: CreditWordType::Aa, word_x: 610, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 669.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FastOff },
    CreditsTiming { frame: 670.0, word_type: CreditWordType::Off, word_x: 48, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 671.0, word_type: CreditWordType::Off, word_x: 110, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 673.0, word_type: CreditWordType::Off, word_x: 185, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 674.0, word_type: CreditWordType::Off, word_x: 262, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 676.0, word_type: CreditWordType::Off, word_x: 317, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 677.0, word_type: CreditWordType::Off, word_x: 357, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 678.0, word_type: CreditWordType::Off, word_x: 417, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 679.0, word_type: CreditWordType::Off, word_x: 491, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 682.0, word_type: CreditWordType::Off, word_x: 558, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 685.0, word_type: CreditWordType::Off, word_x: 628, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 687.0, word_type: CreditWordType::Off, word_x: 720, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 689.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 690.0, word_type: CreditWordType::Off, word_x: 172, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 692.0, word_type: CreditWordType::Off, word_x: 263, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 694.0, word_type: CreditWordType::Off, word_x: 346, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 696.0, word_type: CreditWordType::Off, word_x: 423, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 698.0, word_type: CreditWordType::Off, word_x: 480, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 700.0, word_type: CreditWordType::Off, word_x: 536, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 702.0, word_type: CreditWordType::Off, word_x: 583, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 705.0, word_type: CreditWordType::Off, word_x: 633, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 708.0, word_type: CreditWordType::Off, word_x: 668, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 712.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 719.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 720.0, word_type: CreditWordType::Off, word_x: 182, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 722.0, word_type: CreditWordType::Off, word_x: 267, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 724.0, word_type: CreditWordType::Off, word_x: 331, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 726.0, word_type: CreditWordType::Off, word_x: 371, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 729.0, word_type: CreditWordType::Off, word_x: 434, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 731.0, word_type: CreditWordType::Off, word_x: 486, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 732.0, word_type: CreditWordType::Off, word_x: 562, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 734.0, word_type: CreditWordType::Off, word_x: 617, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 735.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FastOff },
    CreditsTiming { frame: 736.0, word_type: CreditWordType::Aw, word_x: 148, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 738.0, word_type: CreditWordType::Aw, word_x: 211, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 740.0, word_type: CreditWordType::Ee, word_x: 298, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 742.0, word_type: CreditWordType::Oh, word_x: 367, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 744.0, word_type: CreditWordType::Aw, word_x: 440, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 746.0, word_type: CreditWordType::Oh, word_x: 506, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 747.0, word_type: CreditWordType::Aw, word_x: 533, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 748.0, word_type: CreditWordType::Aw, word_x: 601, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 749.0, word_type: CreditWordType::Aw, word_x: 645, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 750.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FastOff },
    CreditsTiming { frame: 753.0, word_type: CreditWordType::Off, word_x: 123, brain_type: CreditBrainType::FlyOn },
    CreditsTiming { frame: 755.0, word_type: CreditWordType::Off, word_x: 195, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 757.0, word_type: CreditWordType::Off, word_x: 255, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 759.0, word_type: CreditWordType::Off, word_x: 312, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 761.0, word_type: CreditWordType::Off, word_x: 378, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 763.0, word_type: CreditWordType::Off, word_x: 443, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 765.0, word_type: CreditWordType::Off, word_x: 516, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 767.0, word_type: CreditWordType::Off, word_x: 563, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 770.0, word_type: CreditWordType::Off, word_x: 588, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 773.0, word_type: CreditWordType::Off, word_x: 657, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 777.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 907.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 908.0, word_type: CreditWordType::Aw, word_x: 214, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 910.0, word_type: CreditWordType::Aw, word_x: 297, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 912.0, word_type: CreditWordType::Aw, word_x: 348, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 916.0, word_type: CreditWordType::Ee, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 920.0, word_type: CreditWordType::Aw, word_x: 455, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 922.0, word_type: CreditWordType::Oh, word_x: 523, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 926.0, word_type: CreditWordType::Aw, word_x: 593, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 930.0, word_type: CreditWordType::Aw, word_x: 616, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 934.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 939.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 940.0, word_type: CreditWordType::Aw, word_x: 214, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 942.0, word_type: CreditWordType::Aw, word_x: 297, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 944.0, word_type: CreditWordType::Aw, word_x: 348, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 948.0, word_type: CreditWordType::Ee, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 952.0, word_type: CreditWordType::Aw, word_x: 455, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 954.0, word_type: CreditWordType::Oh, word_x: 523, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 958.0, word_type: CreditWordType::Aw, word_x: 593, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 960.0, word_type: CreditWordType::Aw, word_x: 616, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 966.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 967.0, word_type: CreditWordType::Aa, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 971.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 972.0, word_type: CreditWordType::Aw, word_x: 214, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 974.0, word_type: CreditWordType::Aw, word_x: 297, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 976.0, word_type: CreditWordType::Aw, word_x: 348, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 980.0, word_type: CreditWordType::Ee, word_x: 400, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 984.0, word_type: CreditWordType::Aw, word_x: 455, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 986.0, word_type: CreditWordType::Oh, word_x: 523, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 990.0, word_type: CreditWordType::Aw, word_x: 593, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 994.0, word_type: CreditWordType::Aw, word_x: 616, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 998.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 1001.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 1002.0, word_type: CreditWordType::Ee, word_x: 150, brain_type: CreditBrainType::FastOn },
    CreditsTiming { frame: 1004.0, word_type: CreditWordType::Oh, word_x: 220, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 1006.0, word_type: CreditWordType::Aw, word_x: 307, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 1008.0, word_type: CreditWordType::Aw, word_x: 390, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 1012.0, word_type: CreditWordType::Ee, word_x: 452, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 1016.0, word_type: CreditWordType::Aw, word_x: 512, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 1018.0, word_type: CreditWordType::Aw, word_x: 573, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 1022.0, word_type: CreditWordType::Aw, word_x: 630, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 1024.0, word_type: CreditWordType::Aw, word_x: 656, brain_type: CreditBrainType::NextWord },
    CreditsTiming { frame: 1028.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::FlyOff },
    CreditsTiming { frame: 1029.0, word_type: CreditWordType::Aa, word_x: 0, brain_type: CreditBrainType::Off },
    CreditsTiming { frame: 1033.0, word_type: CreditWordType::Off, word_x: 0, brain_type: CreditBrainType::Off },
];

///  C++ gCreditsTimingCount
pub const CREDITS_TIMING_COUNT: usize = 268;

/// 制作人员界面（对应 C++ CreditScreen）
pub struct CreditScreen {
    pub close_button: Option<*mut GameButton>,
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub credits_phase: CreditsPhase,
    pub credits_phase_counter: i32,
    pub credits_reanim_id: ReanimationID,
    pub fog_particle_id: ParticleSystemID,
    pub blink_countdown: i32,
    pub main_menu_button: Option<*mut DialogButton>,
    pub replay_button: Option<*mut DialogButton>,
    pub overlay_widget: Option<*mut Widget>,
    pub draw_brain: bool,
    pub brain_pos_x: f32,
    pub brain_pos_y: f32,
    pub update_count: i32,
    pub draw_count: i32,
    pub dont_sync: bool,
    pub credits_paused: bool,
    pub original_music_volume: f64,
    pub preloaded: bool,
    pub last_draw_count: i32,
}

impl CreditScreen {
    pub fn new() -> Self {
        CreditScreen {
            close_button: None,
            app: None,
            credits_phase: CreditsPhase::Main1,
            credits_phase_counter: 0,
            credits_reanim_id: REANIMATIONID_NULL,
            fog_particle_id: PARTICLESYSTEMID_NULL,
            blink_countdown: 0,
            main_menu_button: None,
            replay_button: None,
            overlay_widget: None,
            draw_brain: false,
            brain_pos_x: 0.0,
            brain_pos_y: 0.0,
            update_count: 0,
            draw_count: 0,
            dont_sync: false,
            credits_paused: false,
            original_music_volume: 0.0,
            preloaded: false,
            last_draw_count: 0,
        }
    }

    pub fn update(&mut self) {
        // 对应 C++ Update：片尾阶段推进与 reanim 同步
        if !self.credits_paused {
            let menu_over = self.main_menu_button.map_or(false, |p| unsafe { (&*p).is_over });
            let replay_over = self.replay_button.map_or(false, |p| unsafe { (&*p).is_over });
            if !menu_over && !replay_over {
                // 对应 C++: SetCursor(CURSOR_POINTER)
                if let Some(app) = self.app {
                    unsafe {
                        (*app).base.set_cursor(crate::lawn::game_enums::CURSOR_POINTER);
                    }
                }
            }
        }
        // C++（CreditScreen.cpp:1131）：
        //     if (mCreditsPaused || (!mApp->IsInDemoMode() && mDrawCount == 0)) return;
        let a_in_demo = self.app.map_or(false, |app| unsafe { (*app).base.is_in_demo_mode() });
        let a_draw_count = self.app.map_or(0, |app| unsafe { (*app).base.m_draw_count });
        if self.credits_paused || (!a_in_demo && a_draw_count == 0) {
            return;
        }

        self.update_count += 1;
        if self.update_count == 1 {
            // C++ 中 PreLoadCredits() + PlayReanim(1) + 播放片尾音乐
            let _ = self.play_reanim(1);
            if let Some(app) = self.app {
                unsafe {
                    if let Some(music) = (*app).music.as_mut() {
                        music.make_sure_music_is_playing(MusicTune::CreditsZombiesOnYourLawn);
                    }
                }
            }
        } else if self.dont_sync || self.credits_phase == CreditsPhase::End {
            self.update_movie();
        } else if self.update_count > 1 {
            // [TRANSLATION_NOTE]: C++ 中按 reanim 定义时长与计时器差值调用
            // JumpToFrame(phase+1, 0) 推进阶段或补帧 UpdateMovie()；Rust 侧
            // reanim 轨道计数/计时器未接入，简化直接推进阶段
            if self.credits_phase == CreditsPhase::Main1 {
                self.jump_to_frame(CreditsPhase::Main2, 0.0);
            } else if self.credits_phase == CreditsPhase::Main2 {
                self.jump_to_frame(CreditsPhase::Main3, 0.0);
            } else if self.credits_phase == CreditsPhase::Main3 {
                self.jump_to_frame(CreditsPhase::End, 0.0);
            }
        }

        self.last_draw_count = self.draw_count;
    }
    pub fn draw(&self, g: &mut Graphics) {
        // 对应 C++ Draw：按阶段绘制片尾场景
        // [TRANSLATION_NOTE]: C++ 中按 credits_phase 绘制对应 reanim 动画与
        // 文字（IMAGE_* 资源）；Rust 侧图片资源未接入，暂略
        if let Some(app) = self.app {
            unsafe {
                if let Some(r) = (*app).reanimation_get(self.credits_reanim_id) {
                    r.draw(g);
                }
            }
        }
    }
    pub fn key_char(&mut self, c: char) {
        // 对应 C++ KeyChar：调试键跳帧
        if self.credits_paused {
            return;
        }
        let debug_enabled = self.app.map_or(false, |app| unsafe { (*app).m_debug_keys_enabled });
        if !debug_enabled {
            return;
        }
        match c {
            '1' => self.jump_to_frame(CreditsPhase::Main1, 0.0),
            '2' => self.jump_to_frame(CreditsPhase::Main1, 128.0),
            '3' => self.jump_to_frame(CreditsPhase::Main1, 144.0),
            '4' => self.jump_to_frame(CreditsPhase::Main1, 272.0),
            '5' => self.jump_to_frame(CreditsPhase::Main1, 304.0),
            '6' => self.jump_to_frame(CreditsPhase::Main1, 340.0),
            '7' => self.jump_to_frame(CreditsPhase::Main1, 368.0),
            'q' => self.jump_to_frame(CreditsPhase::Main2, 0.0),
            'w' => self.jump_to_frame(CreditsPhase::Main2, 124.0),
            'e' => self.jump_to_frame(CreditsPhase::Main2, 188.0),
            'r' => self.jump_to_frame(CreditsPhase::Main2, 248.0),
            't' => self.jump_to_frame(CreditsPhase::Main2, 320.0),
            'a' => self.jump_to_frame(CreditsPhase::Main3, 0.0),
            's' => self.jump_to_frame(CreditsPhase::Main3, 124.0),
            'd' => self.jump_to_frame(CreditsPhase::Main3, 216.0),
            'f' => self.jump_to_frame(CreditsPhase::Main3, 240.0),
            'g' => self.jump_to_frame(CreditsPhase::Main3, 324.0),
            'n' => { self.dont_sync = !self.dont_sync; }
            _ => {}
        }
    }
    pub fn key_down(&mut self, key: KeyCode) {
        // 对应 C++ KeyDown：空格/回车/ESC 暂停片尾
        if key == crate::framework::key_codes::KEYCODE_SPACE
            || key == crate::framework::key_codes::KEYCODE_RETURN
            || key == crate::framework::key_codes::KEYCODE_ESCAPE
        {
            self.pause_credits();
        }
    }
    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // C++ 中为空实现
    }
    pub fn button_press(&mut self, _id: i32) {
        // [TRANSLATION_NOTE]: C++ 中 PlaySample(SOUND_GRAVEBUTTON)
    }
    pub fn button_depress(&mut self, the_id: i32) {
        // 对应 C++ ButtonDepress
        const CREDITS_BUTTON_REPLAY: i32 = 0;
        const CREDITS_BUTTON_MAIN_MENU: i32 = 1;
        let Some(app) = self.app else { return };
        unsafe {
            if the_id == CREDITS_BUTTON_MAIN_MENU {
                (*app).kill_credit_screen();
                (*app).do_back_to_main();
            } else if the_id == CREDITS_BUTTON_REPLAY {
                (*app).kill_credit_screen();
                (*app).show_credit_screen();
            }
        }
    }
    pub fn play_reanim(&mut self, index: i32) -> Option<*mut Reanimation> {
        // 对应 C++ PlayReanim：按阶段创建片尾动画并分配渲染组
        let Some(app) = self.app else { return None };
        unsafe {
            // C++ 中先销毁旧动画
            if let Some(r) = (*app).reanimation_get_mut(self.credits_reanim_id) {
                r.reanimation_die();
            }

            let (a_reanim, prefix_assignments): (*mut Reanimation, Vec<(&str, i32)>) = match index {
                1 => {
                    let r = (*app).add_reanimation(0.0, 0.0, 0, ReanimationType::CreditsMain as i32)?;
                    (r, vec![("Background", 1), ("attacher__Zombie", 2), ("Words", 3), ("SpotFront", 3)])
                }
                2 => {
                    let r = (*app).add_reanimation(0.0, 0.0, 0, ReanimationType::CreditsMain2 as i32)?;
                    (r, vec![("Background", 1), ("attacher__Zombie", 2), ("Words", 3), ("SpotFront", 3), ("attacher__undead", 2)])
                }
                3 => {
                    let r = (*app).add_reanimation(0.0, 0.0, 0, ReanimationType::CreditsMain3 as i32)?;
                    (r, vec![("Background", 1), ("attacher__Zombie", 2), ("attacher__DiscoLights", 2), ("Words", 3), ("attacher__cattail", 3), ("SpotFront", 3), ("attacher__undead", 2)])
                }
                _ => return None, // C++ 中 PVZP_ASSERT(false)
            };

            for (prefix, group) in prefix_assignments {
                (*a_reanim).assign_render_group_to_prefix(prefix, group);
            }
            (*a_reanim).m_is_attachment = true;
            (*a_reanim).m_loop_type = crate::todlib::reanimator::ReanimLoopType::PlayOnceAndHold;
            self.credits_reanim_id = (*app).reanimation_get_id(a_reanim);
            Some(a_reanim)
        }
    }
    pub fn jump_to_frame(&mut self, the_phase: CreditsPhase, the_frame: f32) {
        // 对应 C++ JumpToFrame：跳转片尾指定帧并计算音乐偏移
        if let Some(btn) = self.main_menu_button {
            unsafe { (&mut *btn).visible = false; }
        }
        if let Some(btn) = self.replay_button {
            unsafe { (&mut *btn).visible = false; }
        }
        self.credits_phase_counter = 0;
        if let Some(app) = self.app {
            unsafe {
                if let Some(es) = (*app).effect_system.as_mut() {
                    es.effect_system_free_all();
                }
            }
        }

        // C++ 中 PlayReanim(3) 或 PlayReanim(phase+1) 返回动画
        let _reanim = self.play_reanim(if the_phase == CreditsPhase::End { 3 } else { (the_phase as i32) + 1 });

        // 对应 C++ CreditScreen::JumpToFrame（CreditScreen.cpp:1537）:
        // aFrameFactor = 1.0f / (aReanim->mDefinition->mTracks.tracks->mTransforms.count - 1)
        // 即「第一轨道的变换数 - 1」的倒数（原实现以 1/384 近似，且注释误作「轨道数」）
        let a_frame_factor = match _reanim {
            Some(a_reanim) => unsafe {
                let a_frame_count = (*a_reanim)
                    .m_definition
                    .and_then(|d| (*d).m_tracks.first().map(|t| t.m_transforms.len() as f32))
                    .map_or(384.0, |n| n - 1.0);
                1.0 / a_frame_count
            },
            None => 1.0 / 384.0,
        };
        let mut a_music_offset = the_frame * 12142.0;
        let mut a_jump_milliseconds = the_frame * 1000.0 / 7.0;
        if the_phase == CreditsPhase::Main1 {
            if the_frame >= 368.0 { a_music_offset = 12142.0 * (the_frame - 368.0) + 4634474.0; }
            else if the_frame >= 340.0 { a_music_offset = 12142.0 * (the_frame - 340.0) + 4280738.0; }
            else if the_frame >= 304.0 { a_music_offset = 12142.0 * (the_frame - 304.0) + 3825710.0; }
            else if the_frame >= 272.0 { a_music_offset = 12142.0 * (the_frame - 272.0) + 3421764.0; }
            else if the_frame >= 144.0 { a_music_offset = 12142.0 * (the_frame - 144.0) + 1805688.0; }
            else if the_frame >= 128.0 { a_music_offset = 12142.0 * (the_frame - 128.0) + 1603662.0; }
        } else if the_phase == CreditsPhase::Main2 {
            if the_frame >= 320.0 { a_music_offset = 12142.0 * (the_frame - 320.0) + 9069118.0; a_jump_milliseconds += 57142.0; }
            else if the_frame >= 248.0 { a_music_offset = 12142.0 * (the_frame - 248.0) + 8159850.0; a_jump_milliseconds += 57142.0; }
            else if the_frame >= 188.0 { a_music_offset = 12142.0 * (the_frame - 188.0) + 7401454.0; a_jump_milliseconds += 57142.0; }
            else if the_frame >= 124.0 { a_music_offset = 12142.0 * (the_frame - 124.0) + 6593548.0; a_jump_milliseconds += 57142.0; }
            else { a_music_offset = 12142.0 * the_frame + 5026370.0; a_jump_milliseconds += 57142.0; }
        } else if the_phase == CreditsPhase::Main3 {
            if the_frame >= 240.0 { a_music_offset = 12142.0 * (the_frame - 240.0) + 12897822.0; a_jump_milliseconds += 112000.0; }
            else if the_frame >= 216.0 { a_music_offset = 12142.0 * (the_frame - 216.0) + 12594510.0; a_jump_milliseconds += 112000.0; }
            else if the_frame >= 124.0 { a_music_offset = 12142.0 * (the_frame - 124.0) + 11434414.0; a_jump_milliseconds += 112000.0; }
            else { a_music_offset = 12142.0 * the_frame + 9864866.0; a_jump_milliseconds += 112000.0; }
        } else if the_phase == CreditsPhase::End {
            a_music_offset = 14047138.0;
            a_jump_milliseconds += 159142.0;
        }

        // [TRANSLATION_NOTE]: C++ 中 mMusic->PlayFromOffset(MUSIC_FILE_CREDITS_ZOMBIES_ON_YOUR_LAWN,
        // aMusicOffset - 900, 1.0f)；Rust 侧 music 无 PlayFromOffset 等价接入
        let _ = (a_music_offset, a_jump_milliseconds);

        // C++ 中设置动画时间（aFrameFactor * theFrame 或 1.0）
        self.credits_phase = the_phase;
    }
    pub fn draw_fog_effect(&self, g: &mut Graphics, time: f32) {
        // 对应 C++ DrawFogEffect：雾效滚动
        // [TRANSLATION_NOTE]: C++ 中按 reanim 定义轨道时间与 IMAGE_FOG 图片逐格
        // 绘制雾色循环；Rust 侧 reanim 轨道计数/雾图片未接入，暂略
        let _ = (g, time);
    }
    pub fn update_blink(&mut self) {
        // 对应 C++ UpdateBlink：向日葵眨眼动画定时重创建
        self.blink_countdown -= 1;
        if self.blink_countdown > 0 {
            return;
        }

        self.blink_countdown = 700;
        // 对应 C++ CreditScreen::UpdateBlink（CreditScreen.cpp:1101-1121）
        if let Some(app) = self.app {
            unsafe {
                // 先取裸指针（Copy），避免与后续 add_reanimation 的可变借用冲突
                let a_credits_reanim_ptr: Option<*mut crate::todlib::reanimator::Reanimation> =
                    (*app)
                        .reanimation_get(self.credits_reanim_id)
                        .map(|r| r as *const _ as *mut _);
                let a_sunflower_reanim = match a_credits_reanim_ptr {
                    Some(r) => self.find_sub_reanim(r, ReanimationType::Sunflower),
                    None => None,
                };
                if let Some(a_sunflower) = a_sunflower_reanim {
                    // C++: MAIN3 阶段且 mAnimTime 超过 aFrameFactor*200 时不创建眨眼动画
                    if self.credits_phase == CreditsPhase::Main3 {
                        if let Some(a_credits_ptr) = a_credits_reanim_ptr {
                            let a_frame_count = (*a_credits_ptr)
                                .m_definition
                                .and_then(|d| {
                                    (*d).m_tracks.first().map(|t| t.m_transforms.len() as f32)
                                })
                                .map_or(0.0, |n| n - 1.0);
                            let a_frame_factor = 1.0 / a_frame_count;
                            if (*a_credits_ptr).m_anim_time > a_frame_factor * 200.0 {
                                return;
                            }
                        }
                    }

                    let a_blink_reanim =
                        (*app).add_reanimation(0.0, 0.0, 0, ReanimationType::Sunflower as i32);
                    if let Some(a_blink) = a_blink_reanim {
                        (*a_blink).set_frames_for_layer("anim_blink");
                        (*a_blink).m_anim_rate = 15.0;
                        (*a_blink).m_loop_type =
                            crate::todlib::reanimator::ReanimLoopType::PlayOnceFullLastFrame;
                        (*a_blink).attach_to_another_reanimation(&mut *a_sunflower, "anim_idle");
                    }
                }
            }
        }
    }
    pub fn draw_final_credits(&self, g: &mut Graphics) {
        // 对应 C++ DrawFinalCredits：最终名单滚动
        // [TRANSLATION_NOTE]: C++ 中 CREDIT_SCREEN_ANIM_RATE = 0.3f
        const CREDIT_SCREEN_ANIM_RATE: f32 = 0.3;
        let a_content_height = draw_credits_content(g, 0, false);
        let a_total_cycle = a_content_height + crate::lawn::game_enums::BOARD_HEIGHT;
        let a_scroll_offset = ((self.credits_phase_counter as f32 * CREDIT_SCREEN_ANIM_RATE) as i32) % a_total_cycle;
        draw_credits_content(g, crate::lawn::game_enums::BOARD_HEIGHT - a_scroll_offset, true);
    }
    pub fn draw_overlay(&self, g: &mut Graphics) {
        // 对应 C++ DrawOverlay：END 阶段黑色淡出
        if self.credits_phase == CreditsPhase::End {
            let a_fade_alpha = crate::todlib::tod_common::tod_animate_curve(
                50, 100, self.credits_phase_counter, 255, 0,
                crate::lawn::game_enums::TodCurves::Linear,
            );
            if a_fade_alpha > 0 {
                g.set_color(&crate::framework::color::Color::from_rgb(0, 0, 0));
                g.fill_rect_xywh(0, 0, crate::lawn::game_enums::BOARD_WIDTH, crate::lawn::game_enums::BOARD_HEIGHT);
                let _ = a_fade_alpha;
            }
        }
    }
    pub fn update_movie(&mut self) {
        // 对应 C++ UpdateMovie：片尾动画推进与阶段切换
        self.update_blink();

        let mut loop_count = 0i32;
        if let Some(app) = self.app {
            unsafe {
                if let Some(r) = (*app).reanimation_get(self.credits_reanim_id) {
                    loop_count = r.m_loop_count;
                    // C++ 中 aCreditsReanim->Update() + mEffectSystem->Update() +
                    // mPoolEffect->PoolEffectUpdate()；Rust 侧 effect_system 由外部更新
                    let _ = r;
                }
                // 对应 C++ CreditScreen.cpp:1207: TurnOffTongues(aCreditsReanim, 0)
                if let Some(r) = (*app).reanimation_get(self.credits_reanim_id) {
                    let a_reanim_ptr = r as *const _ as *mut crate::todlib::reanimator::Reanimation;
                    self.turn_off_tongues(a_reanim_ptr, 0);
                }
            }
        }

        if self.credits_phase == CreditsPhase::Main1 && loop_count > 0 {
            let _ = self.play_reanim(2);
            self.credits_phase = CreditsPhase::Main2;
        } else if self.credits_phase == CreditsPhase::Main2 && loop_count > 0 {
            let _ = self.play_reanim(3);
            self.credits_phase = CreditsPhase::Main3;
        } else if self.credits_phase == CreditsPhase::Main3 && loop_count > 0 {
            self.credits_phase = CreditsPhase::End;
        } else if self.credits_phase == CreditsPhase::End {
            self.credits_phase_counter += 1;
            if self.credits_phase_counter == 50 {
                if let Some(btn) = self.main_menu_button {
                    unsafe { (&mut *btn).visible = true; }
                }
                if let Some(btn) = self.replay_button {
                    unsafe { (&mut *btn).visible = true; }
                }
            }
        }

        // [TRANSLATION_NOTE]: C++ 中按 ShouldTriggerTimedEvent 触发各阶段事件
        //（嘴巴/肢体动画等）；Rust 侧 reanim 事件系统未接入
    }
    pub fn pause_credits(&mut self) {
        // 对应 C++ PauseCredits：停止音效/音乐并弹出暂停菜单
        if self.credits_paused {
            return;
        }
        if let Some(app) = self.app {
            unsafe {
                if let Some(ss) = (*app).sound_system.as_ref() {
                    ss.stop_foley(FoleyType::Scream);
                }
                // [TRANSLATION_NOTE]: C++ 中 PlaySample(SOUND_PAUSE)
                if let Some(music) = (*app).music.as_mut() {
                    music.game_music_pause(true);
                }
            }
        }
        self.credits_paused = true;
        // [TRANSLATION_NOTE]: C++ 中 LawnMessageBox(DIALOG_MESSAGE, ...) 暂停菜单与恢复流程
        // 未接入，此处以 do_dialog 近似提示
        if let Some(app) = self.app {
            unsafe {
                let _ = (*app).do_dialog(
                    crate::lawn::game_enums::Dialogs::Message as i32,
                    true,
                    "[CREDITS_PAUSE_HEADER]",
                    "[CREDITS_PAUSE_BODY]",
                    "[DIALOG_BUTTON_RESUME]",
                    crate::framework::widget::dialog::BUTTONS_FOOTER,
                );
            }
        }
    }
    pub fn pre_load_credits(&mut self) {
        // 对应 C++ PreLoadCredits：加载片尾背景资源组与 reanim 定义
        self.preloaded = true;
        let resource_names = [
            "DelayLoad_Background1",
            "DelayLoad_Background2",
            "DelayLoad_Background3",
            "DelayLoad_Background4",
            "DelayLoad_Background5",
            "DelayLoad_Background6",
        ];
        if let Some(app) = self.app {
            unsafe {
                if let Some(rm) = (*app).base.resource_manager.as_mut() {
                    for name in &resource_names {
                        let _ = (**rm).load_resources(name);
                    }
                }
                // C++ 中 ReanimatorEnsureDefinitionLoaded + ReanimationPreload
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CreditsMain);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CreditsMain2);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CreditsMain3);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::ZombieCreditsDance);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CreditsBigbrain);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CreditsFlowerPetals);
                crate::todlib::reanim_loader::reanimator_ensure_definition_loaded(ReanimationType::CreditsInfantry);
            }
        }
    }


    /// [TRANSLATION_NOTE]: C++ CreditScreen::AddedToManager
    pub fn added_to_manager(&mut self, the_widget_manager: *mut WidgetManager) {
        // C++: Widget::AddedToManager + AddWidget(mMainMenuButton/mReplayButton/mOverlayWidget)
        if !the_widget_manager.is_null() {
            unsafe {
                if let Some(btn) = self.main_menu_button {
                    (&mut *the_widget_manager).add_widget((&mut *btn).as_widget_ptr());
                }
                if let Some(btn) = self.replay_button {
                    (&mut *the_widget_manager).add_widget((&mut *btn).as_widget_ptr());
                }
            }
        }
    }

    /// [TRANSLATION_NOTE]: C++ CreditScreen::RemovedFromManager
    pub fn removed_from_manager(&mut self, the_widget_manager: *mut WidgetManager) {
        if !the_widget_manager.is_null() {
            unsafe {
                if let Some(btn) = self.main_menu_button {
                    (&mut *the_widget_manager).remove_widget((&mut *btn).as_widget_ptr());
                }
                if let Some(btn) = self.replay_button {
                    (&mut *the_widget_manager).remove_widget((&mut *btn).as_widget_ptr());
                }
            }
        }
    }

    /// [TRANSLATION_NOTE]: C++ CreditScreen::FindSubReanim (CreditScreen.cpp:1072)
    pub fn find_sub_reanim(&self, the_reanim: *mut crate::todlib::reanimator::Reanimation, the_reanim_type: ReanimationType) -> Option<*mut crate::todlib::reanimator::Reanimation> {
        if let Some(reanim_ref) = unsafe { the_reanim.as_ref() } {
            if reanim_ref.reanim_type == the_reanim_type {
                return Some(the_reanim);
            }
            // C++: theReanim->mDefinition->mTracks.count
            let a_track_count = reanim_ref.m_definition.map_or(0, |def| unsafe { (*def).m_tracks.len() });
            for _i in 0..a_track_count {
                // [TRANSLATION_NOTE]: C++ recurses via FindReanimAttachment(mTrackInstances[i].mAttachmentID);
                // attachment system is a stub in Rust, recursion skipped.
            }
        }
        None
    }

    /// [TRANSLATION_NOTE]: C++ CreditScreen::GetTiming (CreditScreen.cpp:520)
    pub fn get_timing(
        &self,
        the_before_timing: &mut *const CreditsTiming,
        the_after_timing: &mut *const CreditsTiming,
        the_fraction: &mut f32,
    ) {
        let app = match self.app {
            Some(p) => p,
            None => return,
        };
        let reanim = unsafe { (*app).reanimation_get(self.credits_reanim_id) };
        // C++: mDefinition->mTracks.tracks->mTransforms.count * mAnimTime - 1.0f
        let mut a_frame_count = reanim.map_or(0.0, |r| {
            let a_transform_count = r
                .m_definition
                .map_or(0, |def| unsafe { (*def).m_tracks.first().map_or(0, |t| t.m_transforms.len()) })
                as f32;
            a_transform_count * r.m_anim_time - 1.0f32
        });
        if self.credits_phase == CreditsPhase::Main1 {
            a_frame_count += 2.0;
        } else if self.credits_phase == CreditsPhase::Main2 {
            a_frame_count += 400.0;
        } else if self.credits_phase == CreditsPhase::Main3 {
            a_frame_count += 785.0;
        } else {
            *the_before_timing = std::ptr::null();
            *the_after_timing = std::ptr::null();
            *the_fraction = 0.0;
            return;
        }

        if a_frame_count < CREDITS_TIMING[0].frame {
            *the_before_timing = std::ptr::null();
            *the_after_timing = &CREDITS_TIMING[0];
            *the_fraction = a_frame_count / CREDITS_TIMING[0].frame;
        } else {
            let mut a_found = false;
            for i in 0..(CREDITS_TIMING_COUNT - 1) {
                let a_timing1 = &CREDITS_TIMING[i];
                let a_timing2 = &CREDITS_TIMING[i + 1];
                // PVZP_ASSERT(aTiming1->mFrame < aTiming2->mFrame)
                debug_assert!(a_timing1.frame < a_timing2.frame);

                if a_timing2.frame > a_frame_count {
                    *the_before_timing = a_timing1;
                    *the_after_timing = a_timing2;
                    *the_fraction = (a_frame_count - a_timing1.frame) / (a_timing2.frame - a_timing1.frame);
                    a_found = true;
                    break;
                }
            }
            if !a_found {
                *the_before_timing = &CREDITS_TIMING[CREDITS_TIMING_COUNT - 1];
                *the_after_timing = std::ptr::null();
                *the_fraction = 0.0;
            }
        }
    }

    /// [TRANSLATION_NOTE]: C++ CreditScreen::TurnOffTongues (CreditScreen.cpp:1501)
    pub fn turn_off_tongues(&self, the_reanim: *mut crate::todlib::reanimator::Reanimation, a_parent_track: i32) {
        let Some(reanim_ref) = (unsafe { the_reanim.as_ref() }) else { return };
        let a_track_count = reanim_ref.m_definition.map_or(0, |def| unsafe { (*def).m_tracks.len() });
        let a_is_credits_dance = reanim_ref.reanim_type == ReanimationType::ZombieCreditsDance;
        unsafe {
            for a_track_index in 0..a_track_count {
                let a_track_instance = &mut *((reanim_ref.m_track_instances.as_ptr() as *mut crate::todlib::definition::ReanimatorTrackInstance).add(a_track_index));
                // C++: reanimType == REANIM_ZOMBIE_CREDITS_DANCE && aParentTrack % 4 != 1 &&
                //      strcasecmp(trackName, "anim_tongue") == 0
                if a_is_credits_dance && a_parent_track % 4 != 1 {
                    let a_track_name = reanim_ref
                        .m_definition
                        .map_or(String::new(), |def| unsafe { (&*def).m_tracks[a_track_index].m_name.clone() });
                    if a_track_name.eq_ignore_ascii_case("anim_tongue") {
                        a_track_instance.m_render_group = crate::todlib::reanimator::RENDER_GROUP_HIDDEN;
                    }
                }
                // [TRANSLATION_NOTE]: C++ recurses via FindReanimAttachment(...); attachment stub, skipped.
            }
        }
    }
}


/// 制作人员叠加 Widget（对应 C++ CreditsOverlay）
pub struct CreditsOverlay {
    pub parent: Option<*mut CreditScreen>,
}

impl CreditsOverlay {
    pub fn new() -> Self { CreditsOverlay { parent: None } }
    pub fn draw(&self, g: &mut Graphics) {
        // 对应 C++ CreditsOverlay::Draw
        // [TRANSLATION_NOTE]: C++ 中绘制附加叠加效果（脑/灯光等）；Rust 侧图片资源未接入
        if let Some(p) = self.parent {
            unsafe { (*p).draw_overlay(g); }
        }
    }
}

// --- 自由函数 ---
pub fn draw_disco(g: &mut Graphics, center_x: f32, center_y: f32, time: f32) {
        // 对应 C++ DrawDisco：3D 加速时的迪斯科灯光三角带
        // [TRANSLATION_NOTE]: C++ 中按 cos/sin 计算三角顶点并 DrawTriangles；
        // Rust 侧 Graphics 无三角带绘制接口，暂略
        let _ = (g, center_x, center_y, time);
    }
pub fn draw_reanim_to_preload(_g: &mut Graphics, the_reanim_type: ReanimationType) {
        // 对应 C++ DrawReanimToPreload：创建指定动画并绘制（预加载用途）
        // [TRANSLATION_NOTE]: C++ 中 CREDIT_SCREEN_ANIM_RATE = 0.3f
        let mut a_reanim = crate::todlib::reanimator::Reanimation::new();
        a_reanim.m_anim_rate = 0.3;
        a_reanim.reanimation_initialize_type(0.0, 0.0, the_reanim_type);
        a_reanim.draw(_g);
    }

/// 绘制制作人员名单内容（对应 C++ DrawCreditsContent）
pub fn draw_credits_content(g: &mut Graphics, y_pos: i32, do_draw: bool) -> i32 {
    let line_height = 20;
    let mut a_y = y_pos;
    if do_draw && a_y > -line_height && a_y < BOARD_HEIGHT + line_height {
        g.set_color(&Color::WHITE);
        g.draw_string("[CREDITS_GAMENAME]", BOARD_WIDTH / 2, a_y);
    }
    a_y += line_height + 20;
    a_y
}

/// 绘制预加载画面（对应 C++ DrawToPreload）
pub fn draw_to_preload(g: &mut Graphics) {
    g.set_color(&Color::BLACK);
    g.fill_rect_xywh(0, 0, BOARD_WIDTH, BOARD_HEIGHT);
    g.set_color(&Color::WHITE);
    g.draw_string("Loading...", BOARD_WIDTH / 2 - 30, BOARD_HEIGHT / 2);
}

/// WidgetManager 包装（对应 C++ CreditScreen : Widget）
pub struct CreditScreenImpl {
    pub screen: *mut CreditScreen,
}

impl CreditScreenImpl {
    pub fn new(screen: *mut CreditScreen) -> Self {
        CreditScreenImpl { screen }
    }
}

impl WidgetImpl for CreditScreenImpl {
    fn update(&mut self, _widget: &mut Widget) {
        unsafe { (*self.screen).update(); }
    }
    fn draw(&mut self, _widget: &Widget, g: &mut Graphics) {
        unsafe { (*self.screen).draw(g); }
    }
    fn draw_overlay(&mut self, _widget: &Widget, g: &mut Graphics) {
        unsafe { (*self.screen).draw_overlay(g); }
    }
    fn key_char(&mut self, _widget: &mut Widget, c: u8) {
        unsafe { (*self.screen).key_char(c as char); }
    }
    fn key_down(&mut self, _widget: &mut Widget, key: KeyCode, _wm: &mut WidgetManager) {
        unsafe { (*self.screen).key_down(key); }
    }
    fn mouse_up_btn(&mut self, _widget: &mut Widget, x: i32, y: i32, _btn: i32, click: i32) {
        unsafe { (*self.screen).mouse_up(x, y, click); }
    }
}

