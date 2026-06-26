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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameMode {
    Adventure = 0,
    SurvivalNormalStage1,
    SurvivalNormalStage2,
    SurvivalNormalStage3,
    SurvivalNormalStage4,
    SurvivalNormalStage5,
    SurvivalHardStage1,
    SurvivalHardStage2,
    SurvivalHardStage3,
    SurvivalHardStage4,
    SurvivalHardStage5,
    SurvivalEndlessStage1,
    SurvivalEndlessStage2,
    SurvivalEndlessStage3,
    SurvivalEndlessStage4,
    SurvivalEndlessStage5,
    ChallengeWarAndPeas,
    ChallengeWallnutBowling,
    ChallengeSlotMachine,
    ChallengeRainingSeeds,
    ChallengeBeghouled,
    ChallengeInvisighoul,
    ChallengeSeeingStars,
    ChallengeZombiquarium,
    ChallengeBeghouledTwist,
    ChallengeLittleTrouble,
    ChallengePortalCombat,
    ChallengeColumns,
    ChallengeBobsledBonanza,
    ChallengeZombieNimble,
    ChallengeWhackAZombie,
    ChallengeLastStand,
    ChallengeWallnutBowling2,
    ChallengePogoParty,
    ChallengeDrZomboss,
    ChallengeScaryPotter,
    ChallengePuzzleMode,
    ChallengeZenGarden,
    ChallengeTreeOfWisdom,
    ChallengeIceLevel,
    ChallengeSunnyDay,
    ChallengeResistance,
    ChallengeZomBotany,
    ChallengeTimeAttack,
    ChallengeMovingTarget,
    ChallengeHeavyWeapons,
    /// 对应 C++ GAMEMODE_UPSELL
    Upsell,
    /// 对应 C++ GAMEMODE_INTRO
    Intro,
    MaxGameModes,
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
    ExplodeONut,
    GiantWallnut,
    Sprout,
    Leftpeater,
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
    Squirrel = 4,
    FromGui = 5,
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
    BeghouledMatching = 2,
    SlotMachineRolling = 3,
    SlotMachineSpinning = 4,
    PortalCombatRelocating = 5,
    WhackAZombieGoingDown = 6,
    WhackAZombieHitting = 7,
    PogoPartyBouncing = 8,
    ZombieNimbleBouncing = 9,
    ZombiquariumFishing = 10,
    ZombiquariumLeaderboard = 11,
    ZombiquariumFeeding = 12,
    ZombiquariumBungees = 13,
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
    NotHere = 0,
    Waiting = 1,
    Talking = 2,
    GivingPresent = 3,
    WaitingToLeave = 4,
    Leaving = 5,
    Gone = 6,
}

// ============================================================
// 新增：ChosenSeedState — 种子选择状态
// 对应 C++ ConstEnums.h ChosenSeedState
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChosenSeedState {
    Unselected = 0,
    Selected = 1,
    Confirmed = 2,
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
    UpToPool,
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
    NeedsWater,
    NeedsHighGround,
    NeedsLowGround,
}

/// PlantPriority — 植物优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantPriority {
    EatingOrder = 0,
    DiggingOrder,
    BungeeOrder,
    CatapultOrder,
    ZenToolOrder,
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
}

/// MessageStyle — 消息样式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MessageStyle {
    Off = 0,
    TutorialLevel1,
    TutorialLevel1Stay,
    TutorialLevel2,
    TutorialLater,
    HintBig,
    HintSmall,
    HintLong,
    HintStay,
    HintFast,
    HintMedium,
    HintSlow,
    ZombieMessage,
    PlantMessage,
    CoinMessage,
    SunMessage,
    Achievement,
    ChallengeMessage,
    LevelName,
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
    NumReanims,
}

/// 种子相关常量
pub const NUM_SEEDS_IN_CHOOSER: i32 = 49; // 对应 C++ SEED_IMITATER + 1，可选种子数量

/// 僵尸类型计数常量
pub const NUM_ZOMBIE_TYPES: i32 = 34;
pub const NUM_SEED_TYPES: usize = 77;
pub const NUM_MOWER_TYPES: usize = 4;
pub const NUM_CACHED_ZOMBIE_TYPES: usize = 37;
