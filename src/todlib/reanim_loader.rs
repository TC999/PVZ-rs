// PvZ Portable Rust — Reanim 定义加载器
//
// main.pak 中的 reanim 资源是 Tod 工具链处理过的 XML 片段（开头可能残缺），
// 无编译 .dat 文件，因此运行时直接解析 XML 构建 ReanimatorDefinition。
// 对应 C++ Reanimator.cpp 的 gLawnReanimationArray / ReanimatorEnsureDefinitionLoaded
// 与 Definition.cpp 的 DefMap(Reanimator) 解析逻辑。

use crate::lawn::game_enums::ReanimationType;
use crate::todlib::definition::{ReanimatorDefinition, ReanimatorTrackDefinition, ReanimatorTransform};

/// ReanimationType → 资源文件路径（对应 C++ gLawnReanimationArray 的 mReanimFileName）
pub fn get_reanim_file_path(reanim_type: ReanimationType) -> Option<&'static str> {
    let path = match reanim_type {
        ReanimationType::LoadbarSprout => "reanim/LoadBar_sprout.reanim",
        ReanimationType::LoadbarZombiehead => "reanim/LoadBar_Zombiehead.reanim",
        ReanimationType::Sodroll => "reanim/SodRoll.reanim",
        ReanimationType::FinalWave => "reanim/FinalWave.reanim",
        ReanimationType::Peashooter => "reanim/PeaShooterSingle.reanim",
        ReanimationType::Wallnut => "reanim/Wallnut.reanim",
        ReanimationType::Lilypad => "reanim/Lilypad.reanim",
        ReanimationType::Sunflower => "reanim/SunFlower.reanim",
        ReanimationType::Lawnmower => "reanim/LawnMower.reanim",
        ReanimationType::Readysetplant => "reanim/StartReadySetPlant.reanim",
        ReanimationType::Cherrybomb => "reanim/CherryBomb.reanim",
        ReanimationType::Squash => "reanim/Squash.reanim",
        ReanimationType::Doomshroom => "reanim/DoomShroom.reanim",
        ReanimationType::Snowpea => "reanim/SnowPea.reanim",
        ReanimationType::Repeater => "reanim/PeaShooter.reanim",
        ReanimationType::Sunshroom => "reanim/SunShroom.reanim",
        ReanimationType::Tallnut => "reanim/Tallnut.reanim",
        ReanimationType::Fumeshroom => "reanim/Fumeshroom.reanim",
        ReanimationType::Puffshroom => "reanim/Puffshroom.reanim",
        ReanimationType::Hypnoshroom => "reanim/Hypnoshroom.reanim",
        ReanimationType::Chomper => "reanim/Chomper.reanim",
        ReanimationType::Zombie => "reanim/Zombie.reanim",
        ReanimationType::Sun => "reanim/Sun.reanim",
        ReanimationType::Potatomine => "reanim/PotatoMine.reanim",
        ReanimationType::Spikeweed => "reanim/Caltrop.reanim",
        ReanimationType::Spikerock => "reanim/SpikeRock.reanim",
        ReanimationType::Threepeater => "reanim/ThreePeater.reanim",
        ReanimationType::Marigold => "reanim/Marigold.reanim",
        ReanimationType::Iceshroom => "reanim/IceShroom.reanim",
        ReanimationType::ZombieFootball => "reanim/Zombie_football.reanim",
        ReanimationType::ZombieNewspaper => "reanim/Zombie_paper.reanim",
        ReanimationType::ZombieZamboni => "reanim/Zombie_zamboni.reanim",
        ReanimationType::Splash => "reanim/splash.reanim",
        ReanimationType::Jalapeno => "reanim/Jalapeno.reanim",
        ReanimationType::JalapenoFire => "reanim/fire.reanim",
        ReanimationType::CoinSilver => "reanim/Coin_silver.reanim",
        ReanimationType::ZombieCharred => "reanim/Zombie_charred.reanim",
        ReanimationType::ZombieCharredImp => "reanim/Zombie_charred_imp.reanim",
        ReanimationType::ZombieCharredDigger => "reanim/Zombie_charred_digger.reanim",
        ReanimationType::ZombieCharredZamboni => "reanim/Zombie_charred_zamboni.reanim",
        ReanimationType::ZombieCharredCatapult => "reanim/Zombie_charred_catapult.reanim",
        ReanimationType::ZombieCharredGargantuar => "reanim/Zombie_charred_gargantuar.reanim",
        ReanimationType::Scareyshroom => "reanim/ScaredyShroom.reanim",
        ReanimationType::Pumpkin => "reanim/Pumpkin.reanim",
        ReanimationType::Plantern => "reanim/Plantern.reanim",
        ReanimationType::Torchwood => "reanim/Torchwood.reanim",
        ReanimationType::Splitpea => "reanim/SplitPea.reanim",
        ReanimationType::Seashroom => "reanim/SeaShroom.reanim",
        ReanimationType::Blover => "reanim/Blover.reanim",
        ReanimationType::FlowerPot => "reanim/Pot.reanim",
        ReanimationType::Cactus => "reanim/Cactus.reanim",
        ReanimationType::Dancer => "reanim/Zombie_disco.reanim",
        ReanimationType::Tanglekelp => "reanim/Tanglekelp.reanim",
        ReanimationType::Starfruit => "reanim/Starfruit.reanim",
        ReanimationType::Polevaulter => "reanim/Zombie_polevaulter.reanim",
        ReanimationType::Balloon => "reanim/Zombie_balloon.reanim",
        ReanimationType::Gargantuar => "reanim/Zombie_gargantuar.reanim",
        ReanimationType::Imp => "reanim/Zombie_imp.reanim",
        ReanimationType::Digger => "reanim/Zombie_digger.reanim",
        ReanimationType::DiggerDirt => "reanim/Digger_rising_dirt.reanim",
        ReanimationType::ZombieDolphinrider => "reanim/Zombie_dolphinrider.reanim",
        ReanimationType::Pogo => "reanim/Zombie_pogo.reanim",
        ReanimationType::BackupDancer => "reanim/Zombie_backup.reanim",
        ReanimationType::Bobsled => "reanim/Zombie_bobsled.reanim",
        ReanimationType::Jackinthebox => "reanim/Zombie_jackbox.reanim",
        ReanimationType::Snorkel => "reanim/Zombie_snorkle.reanim",
        ReanimationType::Bungee => "reanim/Zombie_bungi.reanim",
        ReanimationType::Catapult => "reanim/Zombie_catapult.reanim",
        ReanimationType::Ladder => "reanim/Zombie_ladder.reanim",
        ReanimationType::Puff => "reanim/Puff.reanim",
        ReanimationType::Sleeping => "reanim/Z.reanim",
        ReanimationType::GraveBuster => "reanim/Gravebuster.reanim",
        ReanimationType::ZombiesWon => "reanim/ZombiesWon.reanim",
        ReanimationType::Magnetshroom => "reanim/Magnetshroom.reanim",
        ReanimationType::Boss => "reanim/Zombie_boss.reanim",
        ReanimationType::Cabbagepult => "reanim/Cabbagepult.reanim",
        ReanimationType::Kernelpult => "reanim/Cornpult.reanim",
        ReanimationType::Melonpult => "reanim/Melonpult.reanim",
        ReanimationType::CoffeeBean => "reanim/Coffeebean.reanim",
        ReanimationType::Umbrellaleaf => "reanim/Umbrellaleaf.reanim",
        ReanimationType::Gatlingpea => "reanim/GatlingPea.reanim",
        ReanimationType::Cattail => "reanim/Cattail.reanim",
        ReanimationType::Gloomshroom => "reanim/GloomShroom.reanim",
        ReanimationType::BossIceball => "reanim/Zombie_boss_iceball.reanim",
        ReanimationType::BossFireball => "reanim/Zombie_boss_fireball.reanim",
        ReanimationType::Cobcannon => "reanim/CobCannon.reanim",
        ReanimationType::Garlic => "reanim/Garlic.reanim",
        ReanimationType::GoldMagnet => "reanim/GoldMagnet.reanim",
        ReanimationType::WinterMelon => "reanim/WinterMelon.reanim",
        ReanimationType::TwinSunflower => "reanim/TwinSunflower.reanim",
        ReanimationType::PoolCleaner => "reanim/PoolCleaner.reanim",
        ReanimationType::RoofCleaner => "reanim/RoofCleaner.reanim",
        ReanimationType::FirePea => "reanim/FirePea.reanim",
        ReanimationType::Imitater => "reanim/Imitater.reanim",
        ReanimationType::Yeti => "reanim/Zombie_yeti.reanim",
        ReanimationType::BossDriver => "reanim/Zombie_Boss_driver.reanim",
        ReanimationType::LawnMoweredZombie => "reanim/LawnMoweredZombie.reanim",
        ReanimationType::CrazyDave => "reanim/CrazyDave.reanim",
        ReanimationType::TextFadeOn => "reanim/TextFadeOn.reanim",
        ReanimationType::Hammer => "reanim/Hammer.reanim",
        ReanimationType::SlotMachineHandle => "reanim/SlotMachine.reanim",
        ReanimationType::CreditsFootball => "reanim/Credits_Football.reanim",
        ReanimationType::CreditsJackbox => "reanim/Credits_Jackbox.reanim",
        ReanimationType::SelectorScreen => "reanim/SelectorScreen.reanim",
        ReanimationType::PortalCircle => "reanim/Portal_Circle.reanim",
        ReanimationType::PortalSquare => "reanim/Portal_Square.reanim",
        ReanimationType::ZengardenSprout => "reanim/ZenGarden_sprout.reanim",
        ReanimationType::ZengardenWateringcan => "reanim/ZenGarden_wateringcan.reanim",
        ReanimationType::ZengardenFertilizer => "reanim/ZenGarden_fertilizer.reanim",
        ReanimationType::ZengardenBugspray => "reanim/ZenGarden_bugspray.reanim",
        ReanimationType::ZengardenPhonograph => "reanim/ZenGarden_phonograph.reanim",
        ReanimationType::Diamond => "reanim/Diamond.reanim",
        ReanimationType::ZombieHand => "reanim/Zombie_hand.reanim",
        ReanimationType::Stinky => "reanim/Stinky.reanim",
        ReanimationType::Rake => "reanim/Rake.reanim",
        ReanimationType::RainCircle => "reanim/Rain_circle.reanim",
        ReanimationType::RainSplash => "reanim/Rain_splash.reanim",
        ReanimationType::ZombieSurprise => "reanim/Zombie_surprise.reanim",
        ReanimationType::CoinGold => "reanim/Coin_gold.reanim",
        ReanimationType::Treeofwisdom => "reanim/TreeOfWisdom.reanim",
        ReanimationType::TreeofwisdomClouds => "reanim/TreeOfWisdomClouds.reanim",
        ReanimationType::TreeofwisdomTreefood => "reanim/TreeFood.reanim",
        ReanimationType::CreditsMain => "reanim/Credits_Main.reanim",
        ReanimationType::CreditsMain2 => "reanim/Credits_Main2.reanim",
        ReanimationType::CreditsMain3 => "reanim/Credits_Main3.reanim",
        ReanimationType::ZombieCreditsDance => "reanim/Zombie_credits_dance.reanim",
        ReanimationType::CreditsStage => "reanim/Credits_stage.reanim",
        ReanimationType::CreditsBigbrain => "reanim/Credits_BigBrain.reanim",
        ReanimationType::CreditsFlowerPetals => "reanim/Credits_Flower_petals.reanim",
        ReanimationType::CreditsInfantry => "reanim/Credits_Infantry.reanim",
        ReanimationType::CreditsThroat => "reanim/Credits_Throat.reanim",
        ReanimationType::CreditsCrazydave => "reanim/Credits_CrazyDave.reanim",
        ReanimationType::CreditsBossdance => "reanim/Credits_Bossdance.reanim",
        ReanimationType::ZombieCreditsScreenDoor => "reanim/Zombie_Credits_Screendoor.reanim",
        ReanimationType::ZombieCreditsConehead => "reanim/Zombie_Credits_Conehead.reanim",
        ReanimationType::CreditsZombiearmy1 => "reanim/Credits_ZombieArmy1.reanim",
        ReanimationType::CreditsZombiearmy2 => "reanim/Credits_ZombieArmy2.reanim",
        ReanimationType::CreditsTombstones => "reanim/Credits_Tombstones.reanim",
        ReanimationType::CreditsSolarpower => "reanim/Credits_SolarPower.reanim",
        ReanimationType::CreditsAnyhour => "reanim/Credits_Anyhour.reanim",
        ReanimationType::CreditsWearetheundead => "reanim/Credits_WeAreTheUndead.reanim",
        ReanimationType::CreditsDiscolights => "reanim/Credits_DiscoLights.reanim",
        ReanimationType::Flag => "reanim/Zombie_FlagPole.reanim",
        _ => return None,
    };
    Some(path)
}

/// 全局图片名表（ReanimatorTransform.mImage 索引 ← 图片名）
/// 对应 C++ DefMap 解析时把 DT_IMAGE 字段由名字解析为 Image*（此处用索引暂存名字）
static mut REANIM_IMAGE_NAMES: Vec<String> = Vec::new();

/// 把图片名登记到全局表，返回索引（-1 无图）
pub fn resolve_reanim_image_name(name: &str) -> i32 {
    let name = name.trim().to_string();
    if name.is_empty() {
        return -1;
    }
    unsafe {
        if let Some(idx) = REANIM_IMAGE_NAMES.iter().position(|n| *n == name) {
            return idx as i32;
        }
        REANIM_IMAGE_NAMES.push(name);
        (REANIM_IMAGE_NAMES.len() - 1) as i32
    }
}

/// 按索引取图片名（-1 返回 None）
pub fn get_reanim_image_name(idx: i32) -> Option<&'static str> {
    if idx < 0 {
        return None;
    }
    unsafe {
        REANIM_IMAGE_NAMES
            .get(idx as usize)
            .map(|s| s.as_str() as *const str)
            .map(|p| unsafe { &*p })
    }
}

use crate::todlib::xml_parser::XmlNode;

/// 缺失字段哨兵（对应 C++ DEFAULT_FIELD_PLACEHOLDER = -10000.0f）
const FIELD_PLACEHOLDER: f32 = -10000.0;

/// 解析单个 <t> 变换元素（对应 C++ ReanimatorTransform 的 DefMap 字段）
fn parse_transform_node(node: &XmlNode) -> ReanimatorTransform {
    let mut t = ReanimatorTransform {
        m_trans_x: FIELD_PLACEHOLDER,
        m_trans_y: FIELD_PLACEHOLDER,
        m_skew_x: FIELD_PLACEHOLDER,
        m_skew_y: FIELD_PLACEHOLDER,
        m_scale_x: FIELD_PLACEHOLDER,
        m_scale_y: FIELD_PLACEHOLDER,
        m_frame: FIELD_PLACEHOLDER,
        m_alpha: FIELD_PLACEHOLDER,
        m_image: -1,
        m_visible: true,
        m_font: -1,
        m_text: -1,
        m_color: crate::framework::color::Color::WHITE,
        m_extra_int: 0,
        m_extra_float: 0.0,
    };
    for child in &node.children {
        let v = child.text.trim();
        match child.name.as_str() {
            "x" => t.m_trans_x = v.parse().unwrap_or(0.0),
            "y" => t.m_trans_y = v.parse().unwrap_or(0.0),
            "kx" => t.m_skew_x = v.parse().unwrap_or(0.0),
            "ky" => t.m_skew_y = v.parse().unwrap_or(0.0),
            "sx" => t.m_scale_x = v.parse().unwrap_or(1.0),
            "sy" => t.m_scale_y = v.parse().unwrap_or(1.0),
            "f" => t.m_frame = v.parse().unwrap_or(0.0),
            "a" => t.m_alpha = v.parse().unwrap_or(1.0),
            "i" => t.m_image = resolve_reanim_image_name(v),
            "font" => t.m_font = -1, // [TRANSLATION_NOTE]: 字体解析未接入
            "text" => {
                // [TRANSLATION_NOTE]: 文本字段暂存 extra_int 标记
            }
            _ => {}
        }
    }
    t
}

/// 用前一帧填充缺失字段（对应 C++ ReanimationFillInMissingData）
fn fill_in_missing_data(transforms: &mut [ReanimatorTransform]) {
    let (mut px, mut py, mut pkx, mut pky) = (0.0, 0.0, 0.0, 0.0);
    let (mut psx, mut psy) = (1.0, 1.0);
    let (mut pf, mut pa) = (0.0, 1.0);
    let mut pimg = -1;
    for t in transforms.iter_mut() {
        if t.m_trans_x == FIELD_PLACEHOLDER { t.m_trans_x = px; } else { px = t.m_trans_x; }
        if t.m_trans_y == FIELD_PLACEHOLDER { t.m_trans_y = py; } else { py = t.m_trans_y; }
        if t.m_skew_x == FIELD_PLACEHOLDER { t.m_skew_x = pkx; } else { pkx = t.m_skew_x; }
        if t.m_skew_y == FIELD_PLACEHOLDER { t.m_skew_y = pky; } else { pky = t.m_skew_y; }
        if t.m_scale_x == FIELD_PLACEHOLDER { t.m_scale_x = psx; } else { psx = t.m_scale_x; }
        if t.m_scale_y == FIELD_PLACEHOLDER { t.m_scale_y = psy; } else { psy = t.m_scale_y; }
        if t.m_frame == FIELD_PLACEHOLDER { t.m_frame = pf; } else { pf = t.m_frame; }
        if t.m_alpha == FIELD_PLACEHOLDER { t.m_alpha = pa; } else { pa = t.m_alpha; }
        if t.m_image == -1 { t.m_image = pimg; } else { pimg = t.m_image; }
        t.m_visible = t.m_image != -1;
    }
}

/// 解析 reanim XML 片段（对应 C++ DefinitionLoadXML + ReanimationLoadDefinition）
pub fn parse_reanim_xml(xml: &str) -> Option<ReanimatorDefinition> {
    let nodes = crate::todlib::xml_parser::parse_fragment(xml);
    let mut def = ReanimatorDefinition {
        m_fps: 12.0,
        m_tracks: Vec::new(),
    };
    for node in &nodes {
        match node.name.as_str() {
            "track" => {
                let mut track = ReanimatorTrackDefinition {
                    m_name: node.child_text("name"),
                    m_parent_name: String::new(), // [TRANSLATION_NOTE]: XML 中无 parent 字段
                    m_transforms: Vec::new(),
                    m_shader: String::new(),
                };
                for child in &node.children {
                    if child.name == "t" {
                        track.m_transforms.push(parse_transform_node(child));
                    }
                }
                fill_in_missing_data(&mut track.m_transforms);
                def.m_tracks.push(track);
            }
            "fps" => {
                if let Ok(v) = node.text.trim().parse::<f32>() {
                    def.m_fps = v;
                }
            }
            _ => {}
        }
    }
    if def.m_tracks.is_empty() {
        None
    } else {
        Some(def)
    }
}

use crate::framework::paklib::with_pak_interface;

/// 定义缓存（按 ReanimationType 索引），对应 C++ gReanimatorDefArray
static mut REANIM_DEF_CACHE: Vec<Option<Box<ReanimatorDefinition>>> = Vec::new();

/// 确保定义已加载（对应 C++ ReanimatorEnsureDefinitionLoaded）
pub fn reanimator_ensure_definition_loaded(reanim_type: ReanimationType) {
    unsafe {
        if REANIM_DEF_CACHE.len() == 0 {
            // 预留全部槽位（NumReanims）
            REANIM_DEF_CACHE.resize(144, None);
        }
        let idx = reanim_type as usize;
        if idx >= REANIM_DEF_CACHE.len() {
            return;
        }
        if REANIM_DEF_CACHE[idx].is_some() {
            return;
        }
        REANIM_DEF_CACHE[idx] = load_reanim_definition(reanim_type);
    }
}

fn load_reanim_definition(reanim_type: ReanimationType) -> Option<Box<ReanimatorDefinition>> {
    let path = get_reanim_file_path(reanim_type)?;
    let data = with_pak_interface(|pak| pak.load_file(path))?;
    let xml = String::from_utf8_lossy(&data);
    parse_reanim_xml(&xml).map(Box::new)
}

/// 获取已加载的定义（未加载先加载；失败返回 None）
pub fn reanimator_get_definition(reanim_type: ReanimationType) -> Option<*mut ReanimatorDefinition> {
    reanimator_ensure_definition_loaded(reanim_type);
    unsafe {
        REANIM_DEF_CACHE
            .get(reanim_type as usize)
            .and_then(|slot| slot.as_ref())
            .map(|b| &**b as *const ReanimatorDefinition as *mut ReanimatorDefinition)
    }
}

// ============================================================
// reanim 图片加载（对应 C++ DefinitionLoadImage + gDefLoadResPaths）
// ============================================================

use crate::framework::graphics::image::Image;

/// 图片缓存（按图片名索引）
static mut REANIM_IMAGE_CACHE: Vec<Option<Box<Image>>> = Vec::new();

/// 图片名 → 候选文件路径（对应 C++ gDefLoadResPaths 前缀/目录组合）
/// 例：IMAGE_REANIM_ZOMBIE_INNERARM_HAND → reanim/ZOMBIE_INNERARM_HAND.png、images/...png
fn image_path_candidates(name: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(rest) = name.strip_prefix("IMAGE_REANIM_") {
        out.push(format!("reanim/{}.png", rest));
        out.push(format!("images/{}.png", rest));
    } else if let Some(rest) = name.strip_prefix("IMAGE_") {
        out.push(format!("{}.png", rest));
        out.push(format!("particles/{}.png", rest));
        out.push(format!("images/{}.png", rest));
    } else {
        out.push(format!("{}.png", name));
    }
    out
}

/// 从 pak 读字节并用 image crate 解码为标准 PNG → Image（RGBA 像素）
fn load_reanim_image(name: &str) -> Option<Box<Image>> {
    let candidates = image_path_candidates(name);
    for path in candidates {
        // pak 查询用大写归一化，故此处大小写不敏感
        let bytes = with_pak_interface(|pak| pak.load_file(&path))?;
        if let Some(img) = decode_png_bytes(&bytes) {
            return Some(img);
        }
    }
    None
}

/// 解码 PNG 字节为 Image（pixels 存 RGBA8，对应 Graphics 的 blt 读取）
fn decode_png_bytes(bytes: &[u8]) -> Option<Box<Image>> {
    let dyn_img = image::load_from_memory(bytes).ok()?;
    let rgba = dyn_img.to_rgba8();
    let w = rgba.width() as i32;
    let h = rgba.height() as i32;
    let mut img = Image::new(w, h);
    img.pixels = rgba.into_raw();
    Some(Box::new(img))
}

/// 按图片名索引获取图片（未缓存则加载；失败返回 None）
pub fn reanimator_get_image(idx: i32) -> Option<*mut Image> {
    if idx < 0 {
        return None;
    }
    unsafe {
        if REANIM_IMAGE_CACHE.len() <= idx as usize {
            while REANIM_IMAGE_CACHE.len() <= idx as usize {
                REANIM_IMAGE_CACHE.push(None);
            }
        }
        if REANIM_IMAGE_CACHE[idx as usize].is_none() {
            if let Some(name) = get_reanim_image_name(idx) {
                REANIM_IMAGE_CACHE[idx as usize] = load_reanim_image(name);
            }
        }
        REANIM_IMAGE_CACHE[idx as usize]
            .as_mut()
            .map(|b| &mut **b as *mut Image)
    }
}

/// 按资源路径加载图片的缓存（供背景图/UI 等非 reanim 图片使用）
static mut PATH_IMAGE_CACHE: Vec<(String, Option<Box<Image>>)> = Vec::new();

/// 按 pak 路径加载图片（带缓存；失败返回 None）
/// 例：load_image_by_path("images/background1.jpg")
pub fn load_image_by_path(path: &str) -> Option<*mut Image> {
    unsafe {
        for (k, v) in PATH_IMAGE_CACHE.iter_mut() {
            if k == path {
                return v.as_mut().map(|b| &mut **b as *mut Image);
            }
        }
    }
    let img = load_image_from_path_internal(path);
    unsafe {
        PATH_IMAGE_CACHE.push((path.to_string(), img));
        let (_, v) = PATH_IMAGE_CACHE.last_mut().unwrap();
        v.as_mut().map(|b| &mut **b as *mut Image)
    }
}

fn load_image_from_path_internal(path: &str) -> Option<Box<Image>> {
    let bytes = with_pak_interface(|pak| pak.load_file(path))?;
    decode_png_bytes(&bytes)
}