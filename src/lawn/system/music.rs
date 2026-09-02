// 游戏音乐管理器 — 对应 C++ src/Lawn/System/Music.h / Music.cpp

#![allow(dead_code)]

use crate::lawn::lawn_app::LawnApp;
use crate::framework::sound::music_interface::MusicInterface;
use crate::lawn::game_enums::GameMode;

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
        // 对应 C++ MusicInit：预加载 drums 与 credits 音乐
        self.load_song(MusicFile::Drums, "sounds/mainmusic.mo3");
        self.load_song(MusicFile::CreditsZombiesOnYourLawn, "sounds/ZombiesOnYourLawn.ogg");
    }

    pub fn music_dispose(&mut self) {
        // 对应 C++ 内联空实现
    }

    pub fn music_update(&mut self) {
        // 对应 C++ MusicUpdate
        if self.fade_out_counter > 0 {
            self.fade_out_counter -= 1;
            if self.fade_out_counter == 0 {
                self.stop_all_music();
            } else {
                let a_fade_level = crate::todlib::tod_common::tod_animate_curve_float(
                    self.fade_out_duration, 0, self.fade_out_counter, 1.0, 0.0, crate::lawn::game_enums::TodCurves::Linear,
                );
                if let Some(mi) = self.music_interface {
                    unsafe {
                        (*mi).set_song_volume(self.cur_music_file_main as i32, a_fade_level as f64);
                    }
                }
            }
        }
        self.update_music_burst();
    }

    pub fn stop_all_music(&mut self) {
        // 对应 C++ StopAllMusic
        if let Some(mi) = self.music_interface {
            unsafe {
                if self.cur_music_file_main != MusicFile::None {
                    (*mi).stop_music(self.cur_music_file_main as i32);
                }
                if self.cur_music_file_drums != MusicFile::None {
                    (*mi).stop_music(self.cur_music_file_drums as i32);
                }
            }
        }
        self.cur_music_tune = MusicTune::None;
        self.cur_music_file_main = MusicFile::None;
        self.cur_music_file_drums = MusicFile::None;
        self.cur_music_file_hihats = MusicFile::None;
        self.queued_drum_track_packed_order = -1;
        self.music_drums_state = MusicDrumsState::Off;
        self.music_burst_state = MusicBurstState::Off;
        self.pause_offset = 0;
        self.pause_offset_drums = 0;
        self.paused = false;
        self.fade_out_counter = 0;
    }

    pub fn play_music(&mut self, tune: MusicTune, mut offset: i32, mut drums_offset: i32) {
        // 对应 C++ PlayMusic
        if self.music_disabled {
            return;
        }
        self.cur_music_tune = tune;
        self.cur_music_file_main = MusicFile::None;
        self.cur_music_file_drums = MusicFile::None;
        self.cur_music_file_hihats = MusicFile::None;

        match tune {
            MusicTune::DayGrasswalk => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::NightMoongrains => {
                self.cur_music_file_main = MusicFile::MainMusic;
                self.cur_music_file_drums = MusicFile::Drums;
                if offset == -1 {
                    offset = 0x30;
                    drums_offset = 0x5C;
                }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
                self.play_from_offset(self.cur_music_file_drums, drums_offset, 0.0);
            }
            MusicTune::PoolWateryGraves => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0x5E; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::FogRigormormist => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0x7D; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::RoofGrazeTheRoof => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0xB8; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::ChooseYourSeeds => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0x7A; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::TitleCrazyDaveMainTheme => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0x98; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::ZenGarden => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0xDD; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::PuzzleCerebrawl => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0xB1; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::MinigameLoonboon => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0xA6; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::Conveyer => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0xD4; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::FinalBossBrainiacManiac => {
                self.cur_music_file_main = MusicFile::MainMusic;
                if offset == -1 { offset = 0x9E; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            MusicTune::CreditsZombiesOnYourLawn => {
                self.cur_music_file_main = MusicFile::CreditsZombiesOnYourLawn;
                if offset == -1 { offset = 0; }
                self.play_from_offset(self.cur_music_file_main, offset, 1.0);
            }
            _ => {}
        }
    }

    pub fn play_from_offset(&mut self, file: MusicFile, offset: i32, volume: f64) {
        // 对应 C++ PlayFromOffset
        let is_credits = self.cur_music_tune == MusicTune::CreditsZombiesOnYourLawn;
        if is_credits {
            let a_no_loop = file == MusicFile::CreditsZombiesOnYourLawn;
            if let Some(mi) = self.music_interface {
                unsafe {
                    (*mi).play_music(file as i32, offset, a_no_loop);
                }
            }
        } else {
            if let Some(mi) = self.music_interface {
                unsafe {
                    (*mi).play_music(file as i32, offset, false);
                    (*mi).set_song_volume(file as i32, volume);
                }
            }
        }
    }

    pub fn start_burst(&mut self) {
        // 对应 C++ StartBurst
        if self.music_burst_state == MusicBurstState::Off {
            self.music_burst_state = MusicBurstState::Starting;
            self.burst_state_counter = 400;
        }
    }

    pub fn fade_out(&mut self, fade_out_duration: i32) {
        // 对应 C++ FadeOut
        if self.cur_music_tune != MusicTune::None {
            self.fade_out_counter = fade_out_duration;
            self.fade_out_duration = fade_out_duration;
        }
    }

    pub fn make_sure_music_is_playing(&mut self, tune: MusicTune) {
        // 对应 C++ MakeSureMusicIsPlaying
        if self.cur_music_tune != tune {
            self.stop_all_music();
            self.play_music(tune, -1, -1);
        }
    }

    pub fn game_music_pause(&mut self, pause: bool) {
        // 对应 C++ GameMusicPause
        if pause {
            if !self.paused && self.cur_music_tune != MusicTune::None {
                self.pause_offset = self.get_music_order(self.cur_music_file_main) as i32;
                if let Some(mi) = self.music_interface {
                    unsafe {
                        if self.cur_music_file_main != MusicFile::None {
                            (*mi).pause_music(self.cur_music_file_main as i32);
                        }
                        if self.cur_music_file_drums != MusicFile::None {
                            (*mi).pause_music(self.cur_music_file_drums as i32);
                        }
                    }
                }
                self.paused = true;
            }
        } else if self.paused {
            if let Some(mi) = self.music_interface {
                unsafe {
                    if self.cur_music_file_main != MusicFile::None {
                        (*mi).resume_music(self.cur_music_file_main as i32);
                    }
                    if self.cur_music_file_drums != MusicFile::None {
                        (*mi).resume_music(self.cur_music_file_drums as i32);
                    }
                }
            }
            self.paused = false;
        }
    }

    pub fn update_music_burst(&mut self) {
        // 对应 C++ UpdateMusicBurst（Burst/Drums 状态机与音量调度）
        // [TRANSLATION_NOTE]: 依赖 board 僵尸计数与 SDL 轨道跳转（Mix_ModMusicStreamJumpToOrder），
        // Rust 侧以音乐接口音量近似；完整 MO3 轨道调度留待 SDL 音乐后端接入
        let has_board = self.app.map_or(false, |app| unsafe { (*app).board.is_some() });
        if !has_board {
            return;
        }
        let a_burst_scheme = match self.cur_music_tune {
            MusicTune::DayGrasswalk | MusicTune::PoolWateryGraves
            | MusicTune::FogRigormormist | MusicTune::RoofGrazeTheRoof => 1,
            MusicTune::NightMoongrains => 2,
            _ => 0,
        };
        if a_burst_scheme == 0 {
            return;
        }
        if self.burst_state_counter > 0 {
            self.burst_state_counter -= 1;
        }
        if self.drums_state_counter > 0 {
            self.drums_state_counter -= 1;
        }
        // 简化状态推进：保留 Burst 状态机骨架
        match self.music_burst_state {
            MusicBurstState::Off => {
                self.start_burst();
            }
            MusicBurstState::Starting => {
                if self.burst_state_counter == 0 {
                    self.music_burst_state = MusicBurstState::On;
                    self.burst_state_counter = 800;
                }
            }
            MusicBurstState::On => {
                if self.burst_state_counter == 0 {
                    self.music_burst_state = MusicBurstState::Finishing;
                    self.burst_state_counter = 800;
                    self.music_drums_state = MusicDrumsState::OffQueued;
                }
            }
            MusicBurstState::Finishing => {
                if self.burst_state_counter == 0 {
                    self.music_burst_state = MusicBurstState::Off;
                }
            }
        }
    }

    pub fn music_resync(&mut self) {
        // 对应 C++ MusicResync（跳回当前曲目播放位置）
        let a_packed_order = self.get_music_order(self.cur_music_file_main) as i32;
        if self.cur_music_tune != MusicTune::None && a_packed_order > 0 {
            // [TRANSLATION_NOTE]: SDL 轨道跳转依赖后端，暂以重播近似
            self.play_music(self.cur_music_tune, a_packed_order, -1);
        }
    }

    pub fn music_resync_channel(&mut self, file_to_match: MusicFile, file_to_sync: MusicFile) {
        // 对应 C++ MusicResyncChannel
        let a_packed_order = self.get_music_order(file_to_match);
        let _ = (file_to_sync, a_packed_order);
        // [TRANSLATION_NOTE]: SDL 轨道跳转依赖后端，暂不执行
    }

    pub fn get_music_order(&self, file: MusicFile) -> u32 {
        // 对应 C++ GetMusicOrder：依赖 SDL 后端 GetMusicOrder
        // [TRANSLATION_NOTE]: Rust MusicInterface 无轨道顺序查询，返回 0
        let _ = file;
        0
    }

    pub fn setup_volume_for_tune(&mut self, tune: MusicTune, drums_volume: f32, hihats_volume: f32) {
        // 对应 C++ SetupVolumeForTune：按曲目设置 30 轨音量（SDL ModMusic 通道）
        // [TRANSLATION_NOTE]: 依赖 Mix_ModMusicStreamSetChannelVolume 逐轨设置，
        // Rust 侧以主/鼓/镲三通道音量近似
        let _ = (tune, drums_volume, hihats_volume);
    }

    pub fn get_music_handle(&self, file: MusicFile) -> Option<*mut std::ffi::c_void> {
        // 对应 C++ GetMusicHandle：依赖 SDL 音乐映射表
        // [TRANSLATION_NOTE]: Rust MusicInterface 无句柄查询，返回空
        let _ = file;
        None
    }

    pub fn tod_load_music(&mut self, file: MusicFile, file_name: &str) -> bool {
        // 对应 C++ PvzpLoadMusic：从 pak 读取文件并交给 SDL 加载
        // [TRANSLATION_NOTE]: 依赖 pak 读取与 SDLMusicInterface 加载，Rust 侧由 LoadSong 调用音乐接口
        let _ = (file, file_name);
        true
    }

    pub fn load_song(&mut self, file: MusicFile, file_name: &str) {
        // 对应 C++ LoadSong
        if !self.tod_load_music(file, file_name) {
            self.music_disabled = true;
        }
    }

    pub fn music_title_screen_init(&mut self) {
        // 对应 C++ MusicTitleScreenInit
        self.load_song(MusicFile::MainMusic, "sounds/mainmusic.mo3");
        self.make_sure_music_is_playing(MusicTune::TitleCrazyDaveMainTheme);
    }

    pub fn music_credit_screen_init(&mut self) {
        // 对应 C++ MusicCreditScreenInit
        self.load_song(MusicFile::CreditsZombiesOnYourLawn, "sounds/ZombiesOnYourLawn.ogg");
    }

    pub fn get_num_loading_tasks(&self) -> i32 {
        // 对应 C++ MUSIC_LOADING_TASKS
        7000 // MUSIC_LOADING_TASK_WEIGHT(3500) * 2 个加载文件
    }

    pub fn start_game_music(&mut self) {
        // 对应 C++ StartGameMusic：按模式/关卡选择曲目
        let app = match self.app {
            Some(a) => a,
            None => return,
        };
        unsafe {
            let app = &*app;
            if app.game_mode == GameMode::ChallengeZenGarden || app.game_mode == GameMode::ChallengeTreeOfWisdom {
                self.make_sure_music_is_playing(MusicTune::ZenGarden);
            } else if app.is_final_boss_level() {
                self.make_sure_music_is_playing(MusicTune::FinalBossBrainiacManiac);
            } else if app.is_wallnut_bowling_level() || app.is_whack_a_zombie_level()
                || app.is_little_trouble_level() || app.is_bungee_blitz_level()
                || app.game_mode == GameMode::ChallengeZombieNimble
            {
                self.make_sure_music_is_playing(MusicTune::MinigameLoonboon);
            } else if (app.is_adventure_mode() && (app.player_info.as_ref().map_or(0, |p| p.m_level) == 10
                || app.player_info.as_ref().map_or(0, |p| p.m_level) == 20
                || app.player_info.as_ref().map_or(0, |p| p.m_level) == 30))
                || app.game_mode == GameMode::ChallengeColumns
            {
                self.make_sure_music_is_playing(MusicTune::Conveyer);
            } else if app.is_stormy_night_level() {
                self.stop_all_music();
            } else if app.is_scary_potter_level() || app.is_izombie_level() {
                self.make_sure_music_is_playing(MusicTune::PuzzleCerebrawl);
            } else {
                // [TRANSLATION_NOTE]: 阶段背景判定依赖 board，Rust 侧按默认白天曲目
                self.make_sure_music_is_playing(MusicTune::DayGrasswalk);
            }
        }
    }
}

impl Default for Music {
    fn default() -> Self {
        Music::new()
    }
}
