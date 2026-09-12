// PvZ Portable Rust 翻译 — 游戏常量枚举
// 对应 C++ src/ConstEnums.h 和 src/GameConstants.h

#![allow(dead_code, non_camel_case_types)]

// ============================================================
// 游戏常量
// ============================================================
pub const PI: f64 = 3.141592653589793;
pub const BOARD_WIDTH: i32 = 800;
pub const BOARD_HEIGHT: i32 = 600;
pub const BOARD_OFFSET: i32 = 220;
pub const BOARD_EDGE: i32 = -100;
pub const LAWN_XMIN: i32 = 40;
pub const LAWN_YMIN: i32 = 80;
pub const SEEDBANK_MAX: i32 = 10;
pub const SEED_PACKET_WIDTH: i32 = 50;
pub const SEED_PACKET_HEIGHT: i32 = 70;
pub const CONVEYOR_SPEED: i32 = 4; // 对应 C++ SeedPacket.cpp:35 CONVEYOR_SPEED
pub const NUM_LEVELS: i32 = 50; // 5 areas * 10 levels
pub const FLAG_RAISE_TIME: i32 = 100;
pub const ZOMBIE_COUNTDOWN_FIRST_WAVE: i32 = 1800;
pub const ZOMBIE_COUNTDOWN: i32 = 2500;
pub const ZOMBIE_COUNTDOWN_RANGE: i32 = 600;
pub const SUN_COUNTDOWN: i32 = 425;
pub const SUN_COUNTDOWN_RANGE: i32 = 275;

// ============================================================
// 布局相关常量
// ============================================================
pub const WIDE_BOARD_WIDTH: i32 = 800;
pub const BOARD_IMAGE_WIDTH_OFFSET: i32 = 1180;
pub const BOARD_ICE_START: i32 = 800;
pub const HIGH_GROUND_HEIGHT: i32 = 30;
pub const SEED_BANK_OFFSET_X: i32 = 0;
pub const SEED_BANK_OFFSET_X_END: i32 = 10;
pub const SEED_CHOOSER_OFFSET_Y: i32 = 516;
pub const IMITATER_DIALOG_WIDTH: i32 = 500;
pub const IMITATER_DIALOG_HEIGHT: i32 = 600;

// ============================================================
// 关卡与关卡流程常量
// ============================================================
pub const ADVENTURE_AREAS: i32 = 5;
pub const LEVELS_PER_AREA: i32 = 10;
pub const FINAL_LEVEL: i32 = 50; // == NUM_LEVELS
pub const LAST_STAND_FLAGS: i32 = 5;
pub const ZOMBIE_COUNTDOWN_BEFORE_FLAG: i32 = 4500;
pub const ZOMBIE_COUNTDOWN_BEFORE_REPICK: i32 = 5499;
pub const ZOMBIE_COUNTDOWN_MIN: i32 = 400;
pub const FOG_BLOW_RETURN_TIME: i32 = 2000;
pub const SUN_COUNTDOWN_MAX: i32 = 950;
pub const SURVIVAL_NORMAL_FLAGS: i32 = 5;
pub const SURVIVAL_HARD_FLAGS: i32 = 10;

// ============================================================
// 商店界面布局常量
// ============================================================
pub const STORESCREEN_ITEMOFFSET_1_X: i32 = 422;
pub const STORESCREEN_ITEMOFFSET_1_Y: i32 = 206;
pub const STORESCREEN_ITEMOFFSET_2_X: i32 = 372;
pub const STORESCREEN_ITEMOFFSET_2_Y: i32 = 310;
pub const STORESCREEN_ITEMSIZE: i32 = 74;
pub const STORESCREEN_COINBANK_X: i32 = 650;
pub const STORESCREEN_COINBANK_Y: i32 = 559;
pub const STORESCREEN_PAGESTRING_X: i32 = 470;
pub const STORESCREEN_PAGESTRING_Y: i32 = 500;

// ============================================================
// 枚举
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AdviceType {
    None = -1,
    ClickOnSun = 0,
    ClickedOnSun = 1,
    ClickedOnCoin = 2,
    SeedRefresh = 3,
    CantAffordPlant = 4,
    PlantGravebustersOnGraves = 5,
    PlantLilypadOnWater = 6,
    PlantTanglekelpOnWater = 7,
    PlantSeashroomOnWater = 8,
    PlantPotatoeMineOnLily = 9,
    PlantWrongArtType = 10,
    PlantNeedPot = 11,
    PlantNotOnGrave = 12,
    PlantNotOnCrater = 13,
    CantPlantThere = 14,
    PlantNotOnWater = 15,
    PlantingNeedsGround = 16,
    BeghouledDragToMatch3 = 17,
    BeghouledMatch3 = 18,
    BeghouledMatch4 = 19,
    BeghouledSaveSun = 20,
    BeghouledUseCrater1 = 21,
    BeghouledUseCrater2 = 22,
    PlantNotPassedLine = 23,
    PlantOnlyOnRepeaters = 24,
    PlantOnlyOnMelonpult = 25,
    PlantOnlyOnSunflower = 26,
    PlantOnlyOnSpikeweed = 27,
    PlantOnlyOnKernelpult = 28,
    PlantOnlyOnMagnetshroom = 29,
    PlantOnlyOnFumeshroom = 30,
    PlantOnlyOnLilypad = 31,
    PlantNeedsRepeater = 32,
    PlantNeedsMelonpult = 33,
    PlantNeedsSunflower = 34,
    PlantNeedsSpikeweed = 35,
    PlantNeedsKernelpult = 36,
    PlantNeedsMagnetshroom = 37,
    PlantNeedsFumeshroom = 38,
    PlantNeedsLilypad = 39,
    SlotMachinePull = 40,
    HugeWave = 41,
    ShovelRefresh = 42,
    PortalRelocating = 43,
    SlotMachineCollectSun = 44,
    DestroyPotsToFinishLevel = 45,
    UseShovelOnPots = 46,
    AlmostThere = 47,
    ZombiquariumClickTrophy = 48,
    ZombiquariumCollectSun = 49,
    ZombiquariumClickToFeed,
    ZombiquariumBuySnorkel,
    IZombiePlantsNotReal,
    IZombieNotPassedLine,
    IZombieLeftOfLine,
    SlotMachineSpinAgain,
    IZombieEatAllBrains,
    PeashooterDied,
    StinkySleeping,
    BeghouledNoMoves,
    PlantSunflower5,
    PlantingNeedSleeping,
    ClickToContinue,
    SurviveFlags,
    UnlockedMode,
    NeedWheelbarrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BackgroundType {
    Day = 0,
    Night = 1,
    Pool = 2,
    Fog = 3,
    Roof = 4,
    Boss = 5,
    MushroomGarden = 6,
    Greenhouse = 7,
    Zombiquarium = 8,
    TreeOfWisdom = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BoardResult {
    None = 0,
    Won = 1,
    Lost = 2,
    Restart = 3,
    Quit = 4,
    QuitApp = 5,
    Cheat = 6,
}

// [TRANSLATION_NOTE]: GameMode 枚举数值已显式对齐 C++ ConstEnums.h 的 GameMode 枚举（0-73）。
// 修复：原先从 GAMEMODE_CHALLENGE_WAR_AND_PEAS_2(32) 起全部错位，导致存档文件名
// (userdata/game{id}_{mode}.v4) 与 C++ 互不兼容。现已补齐 24 个缺失模式并把
// ScaryPotter / I_Zombie 系列拆分到各关卡。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameMode {
    Adventure = 0,
    SurvivalNormalStage1 = 1,
    SurvivalNormalStage2 = 2,
    SurvivalNormalStage3 = 3,
    SurvivalNormalStage4 = 4,
    SurvivalNormalStage5 = 5,
    SurvivalHardStage1 = 6,
    SurvivalHardStage2 = 7,
    SurvivalHardStage3 = 8,
    SurvivalHardStage4 = 9,
    SurvivalHardStage5 = 10,
    SurvivalEndlessStage1 = 11,
    SurvivalEndlessStage2 = 12,
    SurvivalEndlessStage3 = 13,
    SurvivalEndlessStage4 = 14,
    SurvivalEndlessStage5 = 15,
    ChallengeWarAndPeas = 16,
    ChallengeWallnutBowling = 17,
    ChallengeSlotMachine = 18,
    ChallengeRainingSeeds = 19,
    ChallengeBeghouled = 20,
    ChallengeInvisighoul = 21,
    ChallengeSeeingStars = 22,
    ChallengeZombiquarium = 23,
    ChallengeBeghouledTwist = 24,
    ChallengeLittleTrouble = 25,
    ChallengePortalCombat = 26,
    ChallengeColumns = 27,
    ChallengeBobsledBonanza = 28,
    /// 对应 C++ GAMEMODE_CHALLENGE_SPEED
    ChallengeZombieNimble = 29,
    ChallengeWhackAZombie = 30,
    ChallengeLastStand = 31,
    ChallengeWarAndPeas2 = 32,
    ChallengeWallnutBowling2 = 33,
    ChallengePogoParty = 34,
    /// 对应 C++ GAMEMODE_CHALLENGE_FINAL_BOSS
    ChallengeFinalBoss = 35,
    ChallengeArtChallengeWallnut = 36,
    ChallengeSunnyDay = 37,
    /// 对应 C++ GAMEMODE_CHALLENGE_RESODDED
    ChallengeResodded = 38,
    ChallengeBigTime = 39,
    ChallengeArtChallengeSunflower = 40,
    ChallengeAirRaid = 41,
    ChallengeIceLevel = 42,
    ChallengeZenGarden = 43,
    ChallengeHighGravity = 44,
    ChallengeGraveDanger = 45,
    ChallengeShovel = 46,
    ChallengeStormyNight = 47,
    ChallengeBungeeBlitz = 48,
    ChallengeSquirrel = 49,
    ChallengeTreeOfWisdom = 50,
    ScaryPotter1 = 51,
    ScaryPotter2 = 52,
    ScaryPotter3 = 53,
    ScaryPotter4 = 54,
    ScaryPotter5 = 55,
    ScaryPotter6 = 56,
    ScaryPotter7 = 57,
    ScaryPotter8 = 58,
    ScaryPotter9 = 59,
    ScaryPotterEndless = 60,
    PuzzleIZombie1 = 61,
    PuzzleIZombie2 = 62,
    PuzzleIZombie3 = 63,
    PuzzleIZombie4 = 64,
    PuzzleIZombie5 = 65,
    PuzzleIZombie6 = 66,
    PuzzleIZombie7 = 67,
    PuzzleIZombie8 = 68,
    PuzzleIZombie9 = 69,
    PuzzleIZombieEndless = 70,
    /// 对应 C++ GAMEMODE_UPSELL
    Upsell = 71,
    /// 对应 C++ GAMEMODE_INTRO
    Intro = 72,
    MaxGameModes = 73,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SeedType {
    Peashooter = 0,
    Sunflower = 1,
    Cherrybomb = 2,
    Wallnut = 3,
    PotatoMine = 4,
    Snowpea = 5,
    Chomper = 6,
    Repeater = 7,
    Puffshroom = 8,
    Sunshroom = 9,
    Fumeshroom = 10,
    Gravebuster = 11,
    Hypnoshroom = 12,
    Scaredyshroom = 13,
    Iceshroom = 14,
    Doomshroom = 15,
    Lilypad = 16,
    Squash = 17,
    Threepeater = 18,
    Tanglekelp = 19,
    Jalapeno = 20,
    Spikeweed = 21,
    Torchwood = 22,
    Tallnut = 23,
    Seashroom = 24,
    Plantern = 25,
    Cactus = 26,
    Blover = 27,
    Splitpea = 28,
    Starfruit = 29,
    Pumpkinshell = 30,
    Magnetshroom = 31,
    Cabbagepult = 32,
    Flowerpot = 33,
    Kernelpult = 34,
    InstantCoffee = 35,
    Garlic = 36,
    Umbrella = 37,
    Marigold = 38,
    Melonpult = 39,
    Gatlingpea = 40,
    Twinsunflower = 41,
    Gloomshroom = 42,
    Cattail = 43,
    Wintermelon = 44,
    GoldMagnet = 45,
    Spikerock = 46,
    Cobcannon = 47,
    Imitater = 48,
    ExplodeONut = 49,
    GiantWallnut = 50,
    Sprout = 51,
    Leftpeater = 52,
    /// 对应 C++ SEED_BEGHOULED_BUTTON_SHUFFLE
    BeghouledButtonShuffle = 53,
    /// 对应 C++ SEED_BEGHOULED_BUTTON_CRATER
    BeghouledButtonCrater = 54,
    /// 对应 C++ SEED_SLOT_MACHINE_SUN
    SlotMachineSun = 55,
    /// 对应 C++ SEED_SLOT_MACHINE_DIAMOND
    SlotMachineDiamond = 56,
    /// 对应 C++ SEED_ZOMBIQUARIUM_SNORKLE
    ZombiquariumSnorkle = 57,
    /// 对应 C++ SEED_ZOMBIQUARIUM_TROPHY
    ZombiquariumTrophy = 58,
    /// 对应 C++ SEED_ZOMBIE_NORMAL
    ZombieNormal = 59,
    /// 对应 C++ SEED_ZOMBIE_TRAFFIC_CONE
    ZombieTrafficCone = 60,
    /// 对应 C++ SEED_ZOMBIE_POLEVAULTER
    ZombiePolevaulter = 61,
    /// 对应 C++ SEED_ZOMBIE_PAIL
    ZombiePail = 62,
    /// 对应 C++ SEED_ZOMBIE_LADDER
    ZombieLadder = 63,
    /// 对应 C++ SEED_ZOMBIE_DIGGER
    ZombieDigger = 64,
    /// 对应 C++ SEED_ZOMBIE_BUNGEE
    ZombieBungee = 65,
    /// 对应 C++ SEED_ZOMBIE_FOOTBALL
    ZombieFootball = 66,
    /// 对应 C++ SEED_ZOMBIE_BALLOON
    ZombieBalloon = 67,
    /// 对应 C++ SEED_ZOMBIE_SCREEN_DOOR
    ZombieScreenDoor = 68,
    /// 对应 C++ SEED_ZOMBONI
    Zomboni = 69,
    /// 对应 C++ SEED_ZOMBIE_POGO
    ZombiePogo = 70,
    /// 对应 C++ SEED_ZOMBIE_DANCER
    ZombieDancer = 71,
    /// 对应 C++ SEED_ZOMBIE_GARGANTUAR
    ZombieGargantuar = 72,
    /// 对应 C++ SEED_ZOMBIE_IMP
    ZombieImp = 73,
    None = -1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombieType {
    Invalid = -1,
    Normal = 0,
    Flag,
    TrafficCone,
    Polevaulter,
    Pail,
    Newspaper,
    Door,
    Football,
    Dancer,
    BackupDancer,
    DuckyTube,
    Snorkel,
    Zamboni,
    Bobsled,
    DolphinRider,
    JackInTheBox,
    Balloon,
    Digger,
    Pogo,
    Yeti,
    Bungee,
    Ladder,
    Catapult,
    Gargantuar,
    Imp,
    Boss,
    PeaHead,
    WallnutHead,
    JalapenoHead,
    GatlingHead,
    SquashHead,
    TallnutHead,
    RedeEyeGargantuar,
    /// 对应 C++ ZOMBIE_CACHED_POLEVAULTER_WITH_POLE（= NUM_ZOMBIE_TYPES 33 + 1），
    /// 仅用于 ReanimatorCache 缓存带杆撑杆跳僵尸帧，不作为实际出场僵尸类型
    CachedPolevaulterWithPole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CoinType {
    None = 0,
    Silver,
    Gold,
    Diamond,
    Sun,
    SmallSun,
    LargeSun,
    FinalSeedPacket,
    Trophy,
    Shovel,
    Almanac,
    Carkeys,
    Vase,
    WateringCan,
    Taco,
    Note,
    UsableSeedPacket,
    PresentPlant,
    AwardMoneyBag,
    AwardPresent,
    AwardBagDiamond,
    AwardSilverSunflower,
    AwardGoldSunflower,
    Chocolate,
    AwardChocolate,
    PresentMinigames,
    PresentPuzzleMode,
    PresentSurvivalMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombiePhase {
    Normal = 0,
    Dying,
    Burned,
    Mowered,
    BungeeDiving,
    BungeeDivingScreaming,
    BungeeAtBottom,
    BungeeGrabbing,
    BungeeRising,
    BungeeHitOuchy,
    BungeeCutscene,
    PolevaulterPreVault,
    PolevaulterInVault,
    PolevaulterPostVault,
    RisingFromGrave,
    JackInTheBoxRunning,
    JackInTheBoxPopping,
    BobsledSliding,
    BobsledBoarding,
    BobsledCrashing,
    PogoBouncing,
    PogoHighBounce1,
    PogoHighBounce2,
    PogoHighBounce3,
    PogoHighBounce4,
    PogoHighBounce5,
    PogoHighBounce6,
    PogoForwardBounce2,
    PogoForwardBounce7,
    NewspaperReading,
    NewspaperMaddening,
    NewspaperMad,
    DiggerTunneling,
    DiggerRising,
    DiggerTunnelingPauseWithoutAxe,
    DiggerRiseWithoutAxe,
    DiggerStunned,
    DiggerWalking,
    DiggerWalkingWithoutAxe,
    DiggerCutscene,
    DancerDancingIn,
    DancerSnappingFingers,
    DancerSnappingFingersWithLight,
    DancerSnappingFingersHold,
    DancerDancingLeft,
    DancerWalkToRaise,
    DancerRaiseLeft1,
    DancerRaiseRight1,
    DancerRaiseLeft2,
    DancerRaiseRight2,
    DancerRising,
    DolphinWalking,
    DolphinIntoPool,
    DolphinRiding,
    DolphinInJump,
    DolphinWalkingInPool,
    DolphinWalkingWithoutDolphin,
    SnorkelWalking,
    SnorkelIntoPool,
    SnorkelWalkingInPool,
    SnorkelUpToEat,
    SnorkelEatingInPool,
    SnorkelDownFromEat,
    ZombiquariumAccel,
    ZombiquariumDrift,
    ZombiquariumBackAndForth,
    ZombiquariumBite,
    CatapultLaunching,
    CatapultReloading,
    GargantuarThrowing,
    GargantuarSmashing,
    ImpGettingThrown,
    ImpLanding,
    BalloonFlying,
    BalloonPopping,
    BalloonWalking,
    LadderCarrying,
    LadderPlacing,
    BossEnter,
    BossIdle,
    BossSpawning,
    BossStomping,
    BossBungeesEnter,
    BossBungeesDrop,
    BossBungeesLeave,
    BossDropRv,
    BossHeadEnter,
    BossHeadIdleBeforeSpit,
    BossHeadIdleAfterSpit,
    BossHeadSpit,
    BossHeadLeave,
    YetiRunning,
    SquashPreLaunch,
    SquashRising,
    SquashFalling,
    SquashDoneFalling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DrawVariation {
    Normal = 0,
    Imitater,
    MarigoldWhite,
    MarigoldMagenta,
    MarigoldOrange,
    MarigoldPink,
    MarigoldLightBlue,
    MarigoldRed,
    MarigoldBlue,
    MarigoldViolet,
    MarigoldLavender,
    MarigoldYellow,
    MarigoldLightGreen,
    ZenGarden,
    ZenGardenWater,
    SproutNoFlower,
    ImitaterLess,
    Aquarium,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CursorType {
    Normal = 0,
    PlantFromBank,
    PlantFromUsableCoin,
    PlantFromGlove,
    PlantFromDuplicator,
    PlantFromWheelBarrow,
    Shovel,
    Hammer,
    CobcannonTarget,
    WateringCan,
    Fertilizer,
    BugSpray,
    Phonograph,
    Chocolate,
    Glove,
    MoneySign,
    Wheelbarrow,
    TreeFood,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StoreItem {
    PlantGatlingpea = 0,
    PlantTwinsunflower,
    PlantGloomshroom,
    PlantCattail,
    PlantWintermelon,
    PlantGoldMagnet,
    PlantSpikerock,
    PlantCobcannon,
    PlantImitater,
    BonusLawnMower,
    PottedMarigold1,
    PottedMarigold2,
    PottedMarigold3,
    GoldWateringcan,
    Fertilizer,
    BugSpray,
    Phonograph,
    GardeningGlove,
    MushroomGarden,
    WheelBarrow,
    StinkyTheSnail,
    PacketUpgrade,
    PoolCleaner,
    RoofCleaner,
    Rake,
    AquariumGarden,
    Chocolate,
    TreeOfWisdom,
    TreeFood,
    Firstaid,
    Pvz,
    Invalid = -1,
}

/// RenderLayer — 渲染层
pub const RENDER_LAYER_GROUND: i32 = 0;
pub const RENDER_LAYER_GRAVE_STONE: i32 = 301000;
pub const RENDER_LAYER_PLANT: i32 = 302000;
pub const RENDER_LAYER_ZOMBIE: i32 = 303000;
pub const RENDER_LAYER_BOSS: i32 = 304000;
pub const RENDER_LAYER_PROJECTILE: i32 = 305000;
pub const RENDER_LAYER_LAWN_MOWER: i32 = 306000;
pub const RENDER_LAYER_PARTICLE: i32 = 307000;
pub const RENDER_LAYER_TOP: i32 = 400000;
pub const RENDER_LAYER_FOG: i32 = 500000;
pub const RENDER_LAYER_UI_BOTTOM: i32 = 100000;
pub const RENDER_LAYER_COIN_BANK: i32 = 600000;
pub const RENDER_LAYER_UI_TOP: i32 = 700000;
pub const RENDER_LAYER_SCREEN_FADE: i32 = 900000;
pub const RENDER_LAYER_ABOVE_UI: i32 = 800000;

// TodCurves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TodCurves {
    Constant = 0,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInOutWeak,
    FastInOut,
    FastInOutWeak,
    WeakFastInOut,
    Bounce,
    BounceFastMiddle,
    BounceSlowMiddle,
    SinWave,
    EaseSinWave,
}

// ============================================================
// 新增：TutorialState — 教程状态
// 对应 C++ ConstEnums.h TutorialState
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TutorialState {
    Off = 0,
    Level1PickUpPeashooter = 1,
    Level1PickUpSunflower = 2,
    Level1PlantPeashooter = 3,
    Level1PlantSunflower = 4,
    Level1RefreshPeashooter = 5,
    Level1RefreshSunflower = 6,
    Level1Completed = 7,
    MoreSunflowers = 8,
    MorePeashooters = 9,
    ZombieAppears = 10,
    ZombieDying = 11,
    ClickOnSun = 12,
    ZombieHead = 13,
    FirstWin = 14,
    SecondWin = 15,
    ThirdWin = 16,
    FirstLost = 17,
    SecondLost = 18,
    ThirdLost = 19,
    ZombieAppears2 = 20,
    Conveyor = 21,
    SlotMachine = 22,
    Zombiquarium = 23,
    Beghouled = 24,
    Twist = 25,
    PortalCombat = 26,
    ZombieNimble = 27,
    WhackAZombie = 28,
    LastStand = 29,
    BobsledBonanza = 30,
    /// 对应 C++ TUTORIAL_ZOMBIQUARIUM_BUY_SNORKEL
    ZombiquariumBuySnorkel = 31,
    /// 对应 C++ TUTORIAL_ZOMBIQUARIUM_BOUGHT_SNORKEL
    ZombiquariumBoughtSnorkel = 32,
    /// 对应 C++ TUTORIAL_ZOMBIQUARIUM_CLICK_TROPHY
    ZombiquariumClickTrophy = 33,
    /// 对应 C++ TUTORIAL_WHACK_A_ZOMBIE_BEFORE_PICK_SEED
    WhackAZombieBeforePickSeed = 34,
    /// 对应 C++ TUTORIAL_WHACK_A_ZOMBIE_PICK_SEED
    WhackAZombiePickSeed = 35,
    /// 对应 C++ TUTORIAL_WHACK_A_ZOMBIE_COMPLETED
    WhackAZombieCompleted = 36,
    /// 对应 C++ TUTORIAL_SHOVEL_PICKUP
    ShovelPickup = 37,
    /// 对应 C++ TUTORIAL_SHOVEL_DIG
    ShovelDig = 38,
    /// 对应 C++ TUTORIAL_SHOVEL_KEEP_DIGGING
    ShovelKeepDigging = 39,
    /// 对应 C++ TUTORIAL_LEVEL_2_PICK_UP_SUNFLOWER
    Level2PickUpSunflower = 40,
    /// 对应 C++ TUTORIAL_LEVEL_2_PLANT_SUNFLOWER
    Level2PlantSunflower = 41,
    /// 对应 C++ TUTORIAL_LEVEL_2_REFRESH_SUNFLOWER
    Level2RefreshSunflower = 42,
    /// 对应 C++ TUTORIAL_LEVEL_2_COMPLETED
    Level2Completed = 43,
    /// 对应 C++ TUTORIAL_MORESUN_PICK_UP_SUNFLOWER
    MoreSunPickUpSunflower = 44,
    /// 对应 C++ TUTORIAL_MORESUN_PLANT_SUNFLOWER
    MoreSunPlantSunflower = 45,
    /// 对应 C++ TUTORIAL_MORESUN_REFRESH_SUNFLOWER
    MoreSunRefreshSunflower = 46,
    /// 对应 C++ TUTORIAL_MORESUN_COMPLETED
    MoreSunCompleted = 47,
    /// 对应 C++ TUTORIAL_SLOT_MACHINE_PULL
    SlotMachinePullTut = 48,
    /// 对应 C++ TUTORIAL_SLOT_MACHINE_COMPLETED
    SlotMachineCompleted = 49,
    /// 对应 C++ TUTORIAL_SHOVEL_COMPLETED
    ShovelCompleted = 50,
    /// 对应 C++ TUTORIAL_ZEN_GARDEN_PICKUP_WATER
    ZenGardenPickupWater = 51,
    /// 对应 C++ TUTORIAL_ZEN_GARDEN_WATER_PLANT
    ZenGardenWaterPlant = 52,
    /// 对应 C++ TUTORIAL_ZEN_GARDEN_KEEP_WATERING
    ZenGardenKeepWatering = 53,
    /// 对应 C++ TUTORIAL_ZEN_GARDEN_VISIT_STORE
    ZenGardenVisitStore = 54,
    /// 对应 C++ TUTORIAL_ZEN_GARDEN_FERTILIZE_PLANTS
    ZenGardenFertilizePlants = 55,
    /// 对应 C++ TUTORIAL_ZEN_GARDEN_COMPLETED
    ZenGardenCompleted = 56,
}

// ============================================================
// 新增：ZombieHeight — 僵尸高度状态
// 对应 C++ ConstEnums.h ZombieHeight
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombieHeight {
    Normal = 0,
    InToPool,
    OutOfPool,
    DraggedUnder,
    UpToHighGround,
    DownOffHighGround,
    UpLadder,
    Falling,
    InToChimney,
    GettingBungeeDropped,
    Zombiquarium,
}

// ============================================================
// 新增：ShieldType — 僵尸盾牌类型
// 对应 C++ ConstEnums.h ShieldType
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ShieldType {
    None = 0,
    Door = 1,
    Newspaper = 2,
    Ladder = 3,
}

// ============================================================
// 新增：CoinMotion — 硬币运动模式
// 对应 C++ ConstEnums.h CoinMotion
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CoinMotion {
    FromSky = 0,
    FromSkySlow = 1,
    FromPlant = 2,
    Coin = 3,
    LawnmowerCoin = 4,
    FromPresent = 5,
    FromBoss = 6,
}

// ============================================================
// 新增：ChallengeState — 挑战模式子状态
// 对应 C++ ConstEnums.h ChallengeState
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChallengeState {
    Normal = 0,
    BeghouledMoving = 1,
    BeghouledFalling = 2,
    BeghouledNoMatches = 3,
    SlotMachineRolling = 4,
    StormFlash1 = 5,
    StormFlash2 = 6,
    StormFlash3 = 7,
    ZenFading = 8,
    ScaryPotterMaletting = 9,
    LastStandOnslaught = 10,
    TreeJustGrew = 11,
    TreeGiveWisdom = 12,
    TreeWaitingToBabble = 13,
    TreeBabbling = 14,
}

// ============================================================
// 新增：RenderObjectType — 渲染对象类型
// 对应 C++ ConstEnums.h RenderObjectType
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RenderObjectType {
    Coin = 0,
    Projectile = 1,
    Plant = 2,
    Zombie = 3,
    LawnMower = 4,
    Particle = 5,
    GridItem = 6,
    /// 以下为 C++ ConstEnums.h 中额外定义的值
    ZombieShadow = 7,
    ZombieBungeeTarget = 8,
    PlantOverlay = 9,
    PlantMagnetItems = 10,
    CursorPreview = 11,
    Reanimation = 12,
    Ice = 13,
    TopUi = 14,
    Fog = 15,
    Storm = 16,
    BottomUi = 17,
    Backdrop = 18,
    DoorMask = 19,
    CoinBank = 20,
    ProjectileShadow = 21,
    Mower = 22,
    ScreenFade = 23,
    BossPart = 24,
    GridItemOverlay = 25,
}

// ============================================================
// 新增：CrazyDaveState — 疯狂戴夫对话状态
// 对应 C++ ConstEnums.h CrazyDaveState
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CrazyDaveState {
    // [TRANSLATION_NOTE]: 数值与语义已对齐 C++ ConstEnums.h CrazyDaveState
    Off = 0,
    Entering = 1,
    Leaving = 2,
    Idling = 3,
    Talking = 4,
    HandingTalking = 5,
    HandingIdling = 6,
}

// ============================================================
// 新增：ChosenSeedState — 种子选择状态
// 对应 C++ ConstEnums.h ChosenSeedState
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChosenSeedState {
    FlyingToBank = 0,
    InBank = 1,
    FlyingToChooser = 2,
    InChooser = 3,
    Hidden = 4,
}

// ============================================================
// 新增：HelmType — 僵尸头盔类型
// 同时定义于 zombie.rs，这里复用以供 game_enums 使用者直接访问
// 注：如有重复定义冲突，请移除 zombie.rs 中的 HelmType 定义
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HelmType {
    None = 0,
    TrafficCone = 1,
    Pail = 2,
    FootballHelmet = 3,
    Digger = 4,
    Redeyes = 5,
    Headband = 6,
    Bobsled = 7,
    Wallnut = 8,
    Tallnut = 9,
}

// ============================================================
// 新增：PlantSubClass — 植物子类
// 同时定义于 plant.rs，这里复用以供 game_enums 使用者直接访问
// 注：如有重复定义冲突，请移除 plant.rs 中的 PlantSubClass 定义
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantSubClass {
    Normal = 0,
    Shooter = 1,
}

// ============================================================
// 新增：PlantWeapon — 植物武器类型
// 同时定义于 plant.rs，这里复用以供 game_enums 使用者直接访问
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantWeapon {
    Primary = 0,
    Secondary = 1,
}

// ============================================================
// 新增：PlantOnBungeeState — 植物被蹦极绑架状态
// 同时定义于 plant.rs，这里复用以供 game_enums 使用者直接访问
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantOnBungeeState {
    NotOnBungee = 0,
    GettingGrabbedByBungee = 1,
    RisingWithBungee = 2,
}

// ============================================================
// 新增：PlantState — 植物动画/行为状态
// 同时定义于 plant.rs，这里复用以供 game_enums 使用者直接访问
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantState {
    NotReady = 0,
    Ready,
    DoingSpecial,
    SquashLook,
    SquashPreLaunch,
    SquashRising,
    SquashFalling,
    SquashDoneFalling,
    GravebusterLanding,
    GravebusterEating,
    ChomperBiting,
    ChomperBitingGotOne,
    ChomperBitingMissed,
    ChomperDigesting,
    ChomperSwallowing,
    PotatoRising,
    PotatoArmed,
    PotatoMashed,
    SpikeweedAttacking,
    SpikeweedAttacking2,
    ScaredyshroomLowering,
    ScaredyshroomScared,
    ScaredyshroomRaising,
    SunshroomSmall,
    SunshroomGrowing,
    SunshroomBig,
    MagnetshroomSucking,
    MagnetshroomCharging,
    BowlingUp,
    BowlingDown,
    CactusLow,
    CactusRising,
    CactusHigh,
    CactusLowering,
    TanglekelpGrabbing,
    CobcannonArming,
    CobcannonLoading,
    CobcannonReady,
    CobcannonFiring,
    KernelpultButter,
    UmbrellaTriggered,
    UmbrellaReflecting,
    ImitaterMorphing,
    ZenGardenWatered,
    ZenGardenNeedy,
    ZenGardenHappy,
    MarigoldEnding,
    FlowerpotInvulnerable,
    LilypadInvulnerable,
}

// ============================================================
// 新增：MagnetItemType — 磁力菇吸取物类型
// 同时定义于 plant.rs，这里复用以供 game_enums 使用者直接访问
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MagnetItemType {
    None = 0,
    Pail1 = 1,
    Pail2 = 2,
    Pail3 = 3,
    FootballHelmet1 = 4,
    FootballHelmet2 = 5,
    FootballHelmet3 = 6,
    Door1 = 7,
    Door2 = 8,
    Door3 = 9,
    Pogo1 = 10,
    Pogo2 = 11,
    Pogo3 = 12,
    JackInTheBox = 13,
    Ladder1 = 14,
    Ladder2 = 15,
    Ladder3 = 16,
    LadderPlaced = 17,
    SilverCoin = 18,
    GoldCoin = 19,
    Diamond = 20,
    PickAxe = 21,
}

// ============================================================
// PlantLayer — 植物渲染层级
// 对应 C++ Plant.h PLANT_LAYER
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantLayer {
    Below = -1,
    Main,
    Reanim,
    ReanimHead,
    ReanimBlink,
    OnTop,
}

// ============================================================
// PlantOrder — 植物在同一格内的绘制顺序
// 对应 C++ Plant.h PLANT_ORDER
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantOrder {
    Lilypad,
    Normal,
    Pumpkin,
    Flyer,
    Cherrybomb,
}

// ============================================================
// DamageFlags — 伤害标志位（bitmask）
// 对应 C++ ConstEnums.h DamageFlags
// ============================================================
pub const DAMAGE_FLAGS_NORMAL: u32 = 0;
pub const DAMAGE_FLAGS_IGNORE_SHIELD: u32 = 1;
pub const DAMAGE_FLAGS_IGNORE_HELM: u32 = 2;
pub const DAMAGE_FLAGS_IGNORE_FLYING: u32 = 4;
pub const DAMAGE_FLAGS_IGNORE_VOODOO: u32 = 8;
pub const DAMAGE_FLAGS_IGNORE_ALL: u32 = 15;

// ============================================================
// DamageRangeFlags — 伤害范围标志位（bitmask）
// ============================================================
pub const DAMAGE_RANGE_NONE: u32 = 0;
pub const DAMAGE_RANGE_NORMAL: u32 = 1;
pub const DAMAGE_RANGE_WIDE: u32 = 2;
pub const DAMAGE_RANGE_ALL: u32 = 3;

// ============================================================
// ID 类型
// ============================================================
pub type AttachmentID = i32;
pub type CoinID = u32;
pub type ParticleID = u32;
pub type ParticleEmitterID = u32;
pub type ParticleSystemID = u32;
pub type PlantID = u32;
pub type ReanimationID = u32;
pub type ZombieID = u32;

pub const ATTACHMENTID_NULL: AttachmentID = 0;
pub const COINID_NULL: CoinID = 0;
pub const PARTICLEID_NULL: ParticleID = 0;
pub const PARTICLEEMITTERID_NULL: ParticleEmitterID = 0;
pub const PARTICLESYSTEMID_NULL: ParticleSystemID = 0;
pub const PLANTID_NULL: PlantID = 0;
pub const REANIMATIONID_NULL: ReanimationID = 0;
pub const ZOMBIEID_NULL: ZombieID = 0;

/// 绘制字符串对齐方式（对应 C++ DrawStringJustification）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DrawStringJustification {
    DS_ALIGN_LEFT = 0,
    DS_ALIGN_RIGHT = 1,
    DS_ALIGN_CENTER = 2,
    DS_ALIGN_LEFT_VERTICAL_MIDDLE = 3,
    DS_ALIGN_RIGHT_VERTICAL_MIDDLE = 4,
    DS_ALIGN_CENTER_VERTICAL_MIDDLE = 5,
}

// ============================================================
// 以下枚举对应 C++ ConstEnums.h
// 命名风格：PascalCase，与现有译名保持一致
// ============================================================

/// AlmanacPage — 图鉴页面
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AlmanacPage {
    Index = 0,
    Plants,
    Zombies,
}

/// AwardType — 奖励类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AwardType {
    ForLevel = 0,
    CreditsZombieNote,
    HelpZombieNote,
    AchievementOnly,
    PreCreditsZombieNote,
}

/// BossPart — 僵尸王部位
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BossPart {
    BackLeg = 0,
    FrontLeg,
    Main,
    BackArm,
    Fireball,
}

/// ChallengePage — 挑战页面
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChallengePage {
    Survival = 0,
    Challenge,
    Limbo,
    Puzzle,
    MaxPages,
}

/// DebugTextMode — 调试文字模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DebugTextMode {
    None = 0,
    ZombieSpawn,
    Music,
    Memory,
    Collision,
}

/// EffectType — 效果类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EffectType {
    Particle = 0,
    Trail,
    Reanim,
    Attachment,
    Other,
}

/// EmitterType — 粒子发射器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EmitterType {
    Circle = 0,
    Box,
    BoxPath,
    CirclePath,
    CircleEvenSpacing,
}

/// GameScenes — 游戏场景
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameScenes {
    Loading = 0,
    Menu,
    LevelIntro,
    Playing,
    ZombiesWon,
    ZombiesLost,
    Averages,
    PostGame,
}

/// GardenType — 花园类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GardenType {
    Main = 0,
    Mushroom,
    Wheelbarrow,
    Aquarium,
}

/// GridSquareType — 网格方块类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GridSquareType {
    None = 0,
    Grass,
    Dirt,
    Pool,
    HighGround,
}

/// LawnMowerState — 割草机状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LawnMowerState {
    RollingIn = 0,
    Ready,
    Triggered,
    Squished,
}

/// LawnMowerType — 割草机类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LawnMowerType {
    Lawn = 0,
    Pool,
    Roof,
    SuperMower,
    NumTypes,
}

/// MowerHeight — 割草机高度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MowerHeight {
    Land = 0,
    DownToPool,
    InPool,
    UpToLand,
}

/// NotRecommend — 不推荐提示
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NotRecommend {
    Nocturnal = 0,
    NeedsPool,
    NeedsGraves,
    NeedsFog,
    NeedsRoof,
    OnRoof,
    ForChallenge,
    AtNight,
}

/// PlantPriority — 植物优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantPriority {
    EatingOrder = 0,
    DiggingOrder,
    BungeeOrder,
    CatapultOrder,
    /// 对应 C++ TOPPLANT_ANY（Bungee/Catapult/Any 同一分支）
    Any,
    ZenToolOrder,
    /// 对应 C++ TOPPLANT_ONLY_NORMAL_POSITION
    OnlyNormalPosition,
    /// 对应 C++ TOPPLANT_ONLY_FLYING
    OnlyFlying,
    TopPlantOnly,
    OnlyPumpkin,
    OnlyUnderPlant,
    OnlyAbovePlant,
    OnlyVulnerable,
}

/// PlantingReason — 种植原因/结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantingReason {
    Ok = 0,
    NotHere,
    OnlyOnGraves,
    OnlyInPool,
    OnlyOnGround,
    OnlyOnHighGround,
    OnlyOnLawnMower,
    NeedsPot,
    NotOnGrave,
    NotOnCrater,
    NotOnWater,
    OnlyOnLilypad,
    OnlyOnFlowerpot,
    NotPassedLine,
    /// 对应 C++ PLANTING_NOT_ON_ART
    NotOnArt,
    /// 对应 C++ PLANTING_NEEDS_SLEEPING
    NeedsSleeping,
    /// 对应 C++ PLANTING_NEEDS_UPGRADE
    NeedsUpgrade,
    /// 对应 C++ PLANTING_NEEDS_GROUND
    NeedsGround,
}

/// PlantRowType — 植物行类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantRowType {
    Dirt = 0,
    Normal,
    Pool,
    HighGround,
}

/// PottedPlantAge — 盆栽植物年龄
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PottedPlantAge {
    Sprout = 0,
    Small,
    Medium,
    Full,
}

/// PottedPlantNeed — 盆栽植物需求
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PottedPlantNeed {
    None = 0,
    Water,
    Fertilizer,
    Bugspray,
    Phonograph,
}

/// ProjectileMotion — 抛射物运动方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProjectileMotion {
    Straight = 0,
    Lobbed,
    Threepeater,
    Bee,
    BeeBackwards,
    Floating,
    Following,
    Homing,
    Slow,
    Boomerang,
}

/// ProjectileType — 抛射物类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProjectileType {
    Pea = 0,
    Snowpea,
    Cabbage,
    Melon,
    Puff,
    Wintermelon,
    CobCannon,
    Butter,
    Kernel,
    PeaIce,
    PeaFire,
    PeaAcid,
    PeaElectric,
    PeaDark,
    PeaShadow,
}

/// ReanimLoopType — 动画循环类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReanimLoopType {
    Loop = 0,
    LoopFullLastFrame,
    PlayOnce,
    PlayOnceAndHold,
    PlayOnceFullLastFrame,
    PlayOnceAndHoldFullLastFrame,
}

/// ScaryPotType — 恐怖罐子类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ScaryPotType {
    None = 0,
    Seed,
    Zombie,
    Sun,
}

/// SeedChooserState — 选卡界面状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SeedChooserState {
    Normal = 0,
    ViewLawn,
}

/// StorePages — 商店页面
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StorePages {
    SlotUpgrades = 0,
    PlantUpgrades,
    Zen1,
    Zen2,
    NumPages,
}

/// TrialType — 试用类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TrialType {
    None = 0,
    StageLocked,
}

/// UnlockingState — 解锁状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UnlockingState {
    Off = 0,
    Shaking,
    Fading,
}

/// GameObjectType — 游戏对象类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameObjectType {
    None = 0,
    Plant,
    Projectile,
    Coin,
    SeedPacket,
    Zombie,
    LawnMower,
    GridItem,
    Particle,
    Attachment,
    Reanimation,
    Effect,
    Trail,
    CursorPreview,
    CoinBank,
    ScreenFade,
    TopUi,
    ScrollWidget,
    Dialog,
    StoreScreen,
    ChallengeScreen,
    TitleScreen,
    AlmanacScreen,
    /// 对应 C++ OBJECT_TYPE_TREE_OF_WISDOM
    TreeOfWisdom,
    /// 对应 C++ OBJECT_TYPE_SLOT_MACHINE_HANDLE
    SlotMachineHandle,
    /// 对应 C++ OBJECT_TYPE_SHOVEL
    Shovel,
    /// 对应 C++ OBJECT_TYPE_WATERING_CAN
    WateringCan,
    /// 对应 C++ OBJECT_TYPE_FERTILIZER
    Fertilizer,
    /// 对应 C++ OBJECT_TYPE_BUG_SPRAY
    BugSpray,
    /// 对应 C++ OBJECT_TYPE_PHONOGRAPH
    Phonograph,
    /// 对应 C++ OBJECT_TYPE_CHOCOLATE
    Chocolate,
    /// 对应 C++ OBJECT_TYPE_GLOVE
    Glove,
    /// 对应 C++ OBJECT_TYPE_MONEY_SIGN
    MoneySign,
    /// 对应 C++ OBJECT_TYPE_WHEELBARROW
    Wheelbarrow,
    /// 对应 C++ OBJECT_TYPE_TREE_FOOD
    TreeFood,
    /// 对应 C++ OBJECT_TYPE_NEXT_GARDEN
    NextGarden,
    /// 对应 C++ OBJECT_TYPE_MENU_BUTTON
    MenuButton,
    /// 对应 C++ OBJECT_TYPE_STORE_BUTTON
    StoreButton,
    /// 对应 C++ OBJECT_TYPE_SCARY_POT
    ScaryPot,
    /// 对应 C++ OBJECT_TYPE_STINKY
    Stinky,
}

/// GridItemType — 网格物品类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GridItemType {
    None = 0,
    Gravestone,
    Crater,
    Ladder,
    PortalCircle,
    PortalSquare,
    ScaryPot,
    Squirrel,
    ZenGardenPlant,
    GraveStone2,
    GraveStone3,
    GraveStone4,
    GraveStone5,
}

/// GridItemState — 网格物品状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GridItemState {
    Normal = 0,
    GravestoneSpecial,
    PortalClosed,
    ScaryPotQuestion,
    ScaryPotLeaf,
    ScaryPotShaking,
    ScaryPotOpen,
    CraterOld,
    LadderDown,
    LadderUp,
    LadderCarried,
    PortalOpen,
    GravestoneAppearing,
    GravestoneSinking,
    GravestoneSunk,
    GravestoneRising,
    GravestoneDestroying,
    SquirrelComing,
    SquirrelEating,
    SquirrelLeaving,
    SquirrelTaken,
    ZenGardenPlantSprout,
    ZenGardenPlantGrowing,
    ZenGardenPlantMature,
    ZenGardenPlantFertilized,
    ZenGardenPlantWatered,
    ZenGardenPlantBugsprayed,
    ZenGardenPlantPhonograph,
    ZenGardenPlantHappy,
    ZenGardenPlantDead,
    /// 耙子正在吸引僵尸（对应 C++ GRIDITEM_STATE_RAKE_ATTRACTING）
    RakeAttracting,
    /// 耙子等待触发（对应 C++ GRIDITEM_STATE_RAKE_WAITING）
    RakeWaiting,
    /// 耙子已触发（对应 C++ GRIDITEM_STATE_RAKE_TRIGGERED）
    RakeTriggered,
    /// 脑子被碾碎（对应 C++ GRIDITEM_STATE_BRAIN_SQUISHED）
    BrainSquished,
    /// 恐怖罐子中为僵尸（对应 C++ GRIDITEM_STATE_SCARY_POT_ZOMBIE）
    ScaryPotZombie,
    /// 松鼠等待（对应 C++ GRIDITEM_STATE_SQUIRREL_WAITING）
    SquirrelWaiting,
    /// 松鼠偷看（对应 C++ GRIDITEM_STATE_SQUIRREL_PEEKING）
    SquirrelPeeking,
    /// 松鼠向上跑（对应 C++ GRIDITEM_STATE_SQUIRREL_RUNNING_UP）
    SquirrelRunningUp,
    /// 松鼠向下跑（对应 C++ GRIDITEM_STATE_SQUIRREL_RUNNING_DOWN）
    SquirrelRunningDown,
    /// 松鼠向左跑（对应 C++ GRIDITEM_STATE_SQUIRREL_RUNNING_LEFT）
    SquirrelRunningLeft,
    /// 松鼠向右跑（对应 C++ GRIDITEM_STATE_SQUIRREL_RUNNING_RIGHT）
    SquirrelRunningRight,
    /// 松鼠被捕获（对应 C++ GRIDITEM_STATE_SQUIRREL_CAUGHT）
    SquirrelCaught,
    /// 松鼠僵尸（对应 C++ GRIDITEM_STATE_SQUIRREL_ZOMBIE）
    SquirrelZombie,
    /// 智慧树肥料（对应 C++ GRIDITEM_STATE_ZEN_TOOL_FERTILIZER）
    ZenToolFertilizer,
    /// 臭鼬向左走（对应 C++ GRIDITEM_STINKY_WALKING_LEFT）
    StinkyWalkingLeft,
    /// 臭鼬向左转（对应 C++ GRIDITEM_STINKY_TURNING_LEFT）
    StinkyTurningLeft,
    /// 臭鼬向右走（对应 C++ GRIDITEM_STINKY_WALKING_RIGHT）
    StinkyWalkingRight,
    /// 臭鼬向右转（对应 C++ GRIDITEM_STINKY_TURNING_RIGHT）
    StinkyTurningRight,
    /// 臭鼬睡觉（对应 C++ GRIDITEM_STINKY_SLEEPING）
    StinkySleeping,
    /// 臭鼬入睡（对应 C++ GRIDITEM_STINKY_FALLING_ASLEEP）
    StinkyFallingAsleep,
    /// 臭鼬醒来（对应 C++ GRIDITEM_STINKY_WAKING_UP）
    StinkyWakingUp,
    /// [TRANSLATION_NOTE]: 以下 4 个为追加变体（对应 C++ GRIDITEM_STATE_ZEN_TOOL_*，C++ 数值为 14-18，
    /// 因 Rust 侧 GridItemState 整体未对齐 C++ 数值，此处追加到末尾保持既有变体不变）
    ZenToolWateringCan,
    ZenToolBugSpray,
    ZenToolPhonograph,
    ZenToolGoldWateringCan,
}

/// MessageStyle — 消息样式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MessageStyle {
    // [TRANSLATION_NOTE]: 数值与命名已对齐 C++ ConstEnums.h MessageStyle（19 值）
    Off = 0,
    TutorialLevel1,
    TutorialLevel1Stay,
    TutorialLevel2,
    TutorialLater,
    TutorialLaterStay,
    HintLong,
    HintFast,
    HintStay,
    HintTallFast,
    HintTallUnlockMessage,
    HintTallLong,
    BigMiddle,
    BigMiddleFast,
    HouseName,
    HugeWave,
    SlotMachine,
    ZenGardenLong,
    Achievement,
}

/// Dialogs — 对话框类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Dialogs {
    NewGame = 0,
    Options,
    NewOptions,
    Almanac,
    Store,
    PreGameNag,
    LoadGame,
    ConfirmUpdateCheck,
    CheckingUpdates,
    RegisterError,
    ColordepthExp,
    OpenUrlWait,
    OpenUrlFail,
    Quit,
    HighScores,
    Nag,
    Info,
    GameOver,
    LevelComplete,
    Paused,
    NoMoreMoney,
    Bonus,
    ConfirmBackToMain,
    ConfirmRestart,
    ThanksForRegistering,
    NotEnoughMoney,
    Upgraded,
    NoUpgrade,
    ChooserWarning,
    UserDialog,
    CreateUser,
    ConfirmDeleteUser,
    RenameUser,
    CreateUserError,
    RenameUserError,
    Cheat,
    CheatError,
    Continue,
    GetReady,
    RestartConfirm,
    ConfirmPurchase,
    ConfirmSell,
    TimesUp,
    VirtualHelp,
    JumpAhead,
    CrazyDave,
    StorePurchase,
    ZenSell,
    Message,
    Imitater,
    PurchasePacketSlot,
    NumDialogs,
}

/// 数值到 Dialogs 枚举的转换（对应 C++ int↔Dialogs 隐式转换）
impl std::convert::TryFrom<i32> for Dialogs {
    type Error = ();
    fn try_from(v: i32) -> std::result::Result<Dialogs, ()> {
        if v >= 0 && v < Dialogs::NumDialogs as i32 {
            // Dialogs 是 #[repr(i32)] 连续枚举（无 gaps），直接 transmute 安全
            Ok(unsafe { std::mem::transmute::<i32, Dialogs>(v) })
        } else {
            Err(())
        }
    }
}

/// ParticleEffect — 粒子效果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleEffect {
    None = -1,
    Melonsplash,
    Wintermelon,
    Fumecloud,
    Popcornsplash,
    Powie,
    Jackexplode,
    ZombieHead,
    ZombieArm,
    ZombieTrafficCone,
    ZombiePail,
    ZombieHelmet,
    ZombieFlag,
    ZombieDoor,
    ZombieNewspaper,
    ZombieHeadlight,
    Pow,
    ZombiePogo,
    ZombieNewspaperHead,
    ZombieBalloonHead,
    SodRoll,
    GraveStoneRise,
    Planting,
    PlantingPool,
    ZombieRise,
    GraveBuster,
    GraveBusterDie,
    PoolSplash,
    IceSparkle,
    SeedPacket,
    TallNutBlock,
    Doom,
    DiggerRise,
    DiggerTunnel,
    DancerRise,
    PoolSparkly,
    WallnutEatSmall,
    WallnutEatLarge,
    PeaSplat,
    ButterSplat,
    CabbageSplat,
    PuffSplat,
    StarSplat,
    IceTrap,
    SnowpeaSplat,
    SnowpeaPuff,
    SnowpeaTrail,
    LanternShine,
    SeedPacketPickup,
    PotatoMine,
    PotatoMineRise,
    PuffshroomTrail,
    PuffshroomMuzzle,
    SeedPacketFlash,
    WhackAZombieRise,
    ZombieLadder,
    UmbrellaReflect,
    SeedPacketPick,
    IceTrapZombie,
    IceTrapRelease,
    ZamboniSmoke,
    Gloomcloud,
    ZombiePogoHead,
    ZamboniTire,
    ZamboniExplosion,
    ZamboniExplosion2,
    CatapultExplosion,
    MowerCloud,
    BossIceBall,
    Blastmark,
    CoinPickupArrow,
    PresentPickup,
    ImitaterMorph,
    MoweredZombieHead,
    MoweredZombieArm,
    ZombieHeadPool,
    ZombieBossFireball,
    FireballDeath,
    IceballDeath,
    IceballTrail,
    FireballTrail,
    BossExplosion,
    ScreenFlash,
    TrophySparkle,
    PortalCircle,
    PortalSquare,
    PottedPlantGlow,
    PottedWaterPlantGlow,
    PottedZenGlow,
    MindControl,
    VaseShatter,
    VaseShatterLeaf,
    VaseShatterZombie,
    AwardPickupArrow,
    ZombieSeaweed,
    ZombieMustache,
    ZombieSunglass,
    ZombiePinata,
    DustSquash,
    DustFoot,
    ZombieDaisies,
    CreditStrobe,
    CreditsRayswipe,
    CreditsZombieheadwipe,
    Starburst,
    CreditsFog,
    PresentPickUpArrow,
    NumParticles,
}

/// ReanimationType — 动画类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ReanimationType {
    None = u32::MAX,
    LoadbarSprout = 0,
    LoadbarZombiehead,
    Sodroll,
    FinalWave,
    Peashooter,
    Wallnut,
    Lilypad,
    Sunflower,
    Lawnmower,
    Readysetplant,
    Cherrybomb,
    Squash,
    Doomshroom,
    Snowpea,
    Repeater,
    Sunshroom,
    Tallnut,
    Fumeshroom,
    Puffshroom,
    Hypnoshroom,
    Chomper,
    Zombie,
    Sun,
    Potatomine,
    Spikeweed,
    Spikerock,
    Threepeater,
    Marigold,
    Iceshroom,
    ZombieFootball,
    ZombieNewspaper,
    ZombieZamboni,
    Splash,
    Jalapeno,
    JalapenoFire,
    CoinSilver,
    ZombieCharred,
    ZombieCharredImp,
    ZombieCharredDigger,
    ZombieCharredZamboni,
    ZombieCharredCatapult,
    ZombieCharredGargantuar,
    Scareyshroom,
    Pumpkin,
    Plantern,
    Torchwood,
    Splitpea,
    Seashroom,
    Blover,
    FlowerPot,
    Cactus,
    Dancer,
    Tanglekelp,
    Starfruit,
    Polevaulter,
    Balloon,
    Gargantuar,
    Imp,
    Digger,
    DiggerDirt,
    ZombieDolphinrider,
    Pogo,
    BackupDancer,
    Bobsled,
    Jackinthebox,
    Snorkel,
    Bungee,
    Catapult,
    Ladder,
    Puff,
    Sleeping,
    GraveBuster,
    ZombiesWon,
    Magnetshroom,
    Boss,
    Cabbagepult,
    Kernelpult,
    Melonpult,
    CoffeeBean,
    Umbrellaleaf,
    Gatlingpea,
    Cattail,
    Gloomshroom,
    BossIceball,
    BossFireball,
    Cobcannon,
    Garlic,
    GoldMagnet,
    WinterMelon,
    TwinSunflower,
    PoolCleaner,
    RoofCleaner,
    FirePea,
    Imitater,
    Yeti,
    BossDriver,
    LawnMoweredZombie,
    CrazyDave,
    TextFadeOn,
    Hammer,
    SlotMachineHandle,
    CreditsFootball,
    CreditsJackbox,
    SelectorScreen,
    PortalCircle,
    PortalSquare,
    ZengardenSprout,
    ZengardenWateringcan,
    ZengardenFertilizer,
    ZengardenBugspray,
    ZengardenPhonograph,
    Diamond,
    ZombieHand,
    Stinky,
    Rake,
    RainCircle,
    RainSplash,
    ZombieSurprise,
    CoinGold,
    Treeofwisdom,
    TreeofwisdomClouds,
    TreeofwisdomTreefood,
    CreditsMain,
    CreditsMain2,
    CreditsMain3,
    ZombieCreditsDance,
    CreditsStage,
    CreditsBigbrain,
    CreditsFlowerPetals,
    CreditsInfantry,
    CreditsThroat,
    CreditsCrazydave,
    CreditsBossdance,
    ZombieCreditsScreenDoor,
    ZombieCreditsConehead,
    CreditsZombiearmy1,
    CreditsZombiearmy2,
    CreditsTombstones,
    CreditsSolarpower,
    CreditsAnyhour,
    CreditsWearetheundead,
    CreditsDiscolights,
    Flag,
    ZombatarHead,
    NumReanims,
}

/// 种子相关常量
pub const NUM_SEEDS_IN_CHOOSER: i32 = 49; // 对应 C++ SEED_IMITATER + 1，可选种子数量

/// 僵尸类型计数常量
pub const NUM_ZOMBIE_TYPES: i32 = 33;
/// 提示类型计数常量（对应 C++ ConstEnums.h AdviceType 结束符 NUM_ADVICE_TYPES）
pub const NUM_ADVICE_TYPES: i32 = 66;
/// 对应 C++ SeedType::NUM_SEED_TYPES（= SEED_LEFTPEATER 52 + 1）。其后 53..73 的
/// SEED_BEGHOULED_BUTTON_* / SEED_ZOMBIE_* / Zombiquarium 等为值域扩展项，不计入本常量。
pub const NUM_SEED_TYPES: usize = 53;
pub const NUM_MOWER_TYPES: usize = 4;
/// 对应 C++ ZombieType::NUM_CACHED_ZOMBIE_TYPES（= ZOMBIE_CACHED_POLEVAULTER_WITH_POLE 34 + 1）
pub const NUM_CACHED_ZOMBIE_TYPES: usize = 35;
