// 游戏音乐管理器 — 对应 C++ src/Lawn/System/Music.h / Music.cpp

#![allow(dead_code)]

use crate::lawn::lawn_app::LawnApp;
use crate::framework::sound::music_interface::MusicInterface;

/// 音乐曲目（对应 C++ MusicTune）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MusicTune {
    None = -1,
    DayGrasswalk = 1,          // 白天草地关卡
    NightMoongrains,            // 黑夜草地关卡
    PoolWateryGraves,           // 白天泳池关卡
    FogRigormormist,            // 黑夜泳池关卡
    RoofGrazeTheRoof,           // 屋顶关卡
    ChooseYourSeeds,            // 选卡界面/小游戏界面
    TitleCrazyDaveMainTheme,    // 主菜单
    ZenGarden,                  // 禅境花园
    PuzzleCerebrawl,            // 解谜模式
    MinigameLoonboon,           // 小游戏
    Conveyer,                   // 传送带关卡
    FinalBossBrainiacManiac,    // 僵王博士关卡
    CreditsZombiesOnYourLawn,   // MV
    NumMusicTunes,
}

/// 音乐文件（对应 C++ MusicFile）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MusicFile {
    None = -1,
    MainMusic = 1,
    Drums,
    Hihats,
    CreditsZombiesOnYourLawn,
    NumMusicFiles,
}

/// 音乐爆发状态（对应 C++ MusicBurstState）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MusicBurstState {
    Off,
    Starting,
    On,
    Finishing,
}

/// 音乐鼓点状态（对应 C++ MusicDrumsState）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MusicDrumsState {
    Off,
    OnQueued,
    On,
    OffQueued,
    Fading,
}

/// 音乐文件数据（对应 C++ MusicFileData）
pub struct MusicFileData {
    pub file_data: Option<*mut u32>,
}

/// 游戏音乐管理器（对应 C++ Music）
pub struct Music {
    pub app: Option<*mut LawnApp>,
    pub music_interface: Option<*mut dyn MusicInterface>,
    pub cur_music_tune: MusicTune,
    pub cur_music_file_main: MusicFile,
    pub cur_music_file_drums: MusicFile,
    pub cur_music_file_hihats: MusicFile,
    pub burst_override: i32,
    pub base_bpm: f32,
    pub base_mod_speed: f32,
    pub music_burst_state: MusicBurstState,
    pub burst_state_counter: i32,
    pub music_drums_state: MusicDrumsState,
    pub queued_drum_track_packed_order: i32,
    pub drums_state_counter: i32,
    pub pause_offset: i32,
    pub pause_offset_drums: i32,
    pub paused: bool,
    pub music_disabled: bool,
    pub fade_out_counter: i32,
    pub fade_out_duration: i32,
}

impl Music {
    pub fn new() -> Self {
        Music {
            app: None,
            music_interface: None,
            cur_music_tune: MusicTune::None,
            cur_music_file_main: MusicFile::None,
            cur_music_file_drums: MusicFile::None,
            cur_music_file_hihats: MusicFile::None,
            burst_override: 0,
            base_bpm: 0.0,
            base_mod_speed: 0.0,
            music_burst_state: MusicBurstState::Off,
            burst_state_counter: 0,
            music_drums_state: MusicDrumsState::Off,
            queued_drum_track_packed_order: 0,
            drums_state_counter: 0,
            pause_offset: 0,
            pause_offset_drums: 0,
            paused: false,
            music_disabled: false,
            fade_out_counter: 0,
            fade_out_duration: 0,
        }
    }

    pub fn new_with_app(app: *mut LawnApp) -> Self {
        Music {
            app: Some(app),
            music_interface: None,
            cur_music_tune: MusicTune::None,
            cur_music_file_main: MusicFile::None,
            cur_music_file_drums: MusicFile::None,
            cur_music_file_hihats: MusicFile::None,
            burst_override: 0,
            base_bpm: 0.0,
            base_mod_speed: 0.0,
            music_burst_state: MusicBurstState::Off,
            burst_state_counter: 0,
            music_drums_state: MusicDrumsState::Off,
            queued_drum_track_packed_order: 0,
            drums_state_counter: 0,
            pause_offset: 0,
            pause_offset_drums: 0,
            paused: false,
            music_disabled: false,
            fade_out_counter: 0,
            fade_out_duration: 0,
        }
    }

    // --- 方法存根（待从 Music.cpp 翻译具体实现） ---

    pub fn music_init(&mut self) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn music_dispose(&mut self) {
        // 对应 C++ 内联空实现
    }

    pub fn music_update(&mut self) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn stop_all_music(&mut self) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn play_music(&mut self, tune: MusicTune, offset: i32, drums_offset: i32) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn get_music_handle(&self, file: MusicFile) -> Option<*mut std::ffi::c_void> {
        // TODO: 从 Music.cpp 翻译
        None
    }

    pub fn start_game_music(&mut self) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn load_song(&mut self, file: MusicFile, file_name: &str) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn music_resync(&mut self) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn update_music_burst(&mut self) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn start_burst(&mut self) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn game_music_pause(&mut self, pause: bool) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn play_from_offset(&mut self, file: MusicFile, offset: i32, volume: f64) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn music_resync_channel(&mut self, file_to_match: MusicFile, file_to_sync: MusicFile) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn tod_load_music(&mut self, file: MusicFile, file_name: &str) -> bool {
        // TODO: 从 Music.cpp 翻译
        false
    }

    pub fn music_title_screen_init(&mut self) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn make_sure_music_is_playing(&mut self, tune: MusicTune) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn fade_out(&mut self, fade_out_duration: i32) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn setup_volume_for_tune(&mut self, tune: MusicTune, drums_volume: f32, hihats_volume: f32) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn get_music_order(&self, file: MusicFile) -> u32 {
        // TODO: 从 Music.cpp 翻译
        0
    }

    pub fn music_credit_screen_init(&mut self) {
        // TODO: 从 Music.cpp 翻译
    }

    pub fn get_num_loading_tasks(&self) -> i32 {
        // TODO: 从 Music.cpp 翻译
        0
    }
}

impl Default for Music {
    fn default() -> Self {
        Music::new()
    }
}
