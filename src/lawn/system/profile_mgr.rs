// PvZ Portable Rust 翻译 — ProfileMgr（玩家档案管理器）
// 对应 C++ src/Lawn/System/ProfileMgr.h / ProfileMgr.cpp

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::lawn::system::player_info::PlayerInfo;
use crate::lawn::system::data_sync::{DataSync, DataReader, DataWriter};

/// 不区分大小写的字符串比较函数（对应 C++ StringLessNoCase）
fn case_insensitive_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    a.to_lowercase().cmp(&b.to_lowercase())
}

/// 档案管理器（对应 C++ ProfileMgr）
/// 管理多个 PlayerInfo 档案的增删改查
pub struct ProfileMgr {
    /// 档案映射表（不区分大小写的键 -> PlayerInfo）
    m_profile_map: BTreeMap<String, PlayerInfo>,
    /// 下一个可用的档案 ID
    m_next_profile_id: u32,
    /// 下一个可用使用序号
    m_next_profile_use_seq: u32,
}

impl ProfileMgr {
    pub fn new() -> Self {
        ProfileMgr {
            m_profile_map: BTreeMap::new(),
            m_next_profile_id: 1,
            m_next_profile_use_seq: 1,
        }
    }

    /// 清空所有档案
    pub fn clear(&mut self) {
        self.m_profile_map.clear();
        self.m_next_profile_id = 1;
        self.m_next_profile_use_seq = 1;
    }

    /// 获取档案数量
    pub fn get_num_profiles(&self) -> usize {
        self.m_profile_map.len()
    }

    /// 获取指定名称的档案
    /// 返回可变引用，使用序号递增
    pub fn get_profile(&mut self, name: &str) -> Option<&mut PlayerInfo> {
        // 使用不区分大小写的查找
        let key = self.find_key(name)?;
        let profile = self.m_profile_map.get_mut(&key)?;
        profile.m_use_seq = self.m_next_profile_use_seq;
        self.m_next_profile_use_seq += 1;
        Some(profile)
    }

    /// 添加新档案
    pub fn add_profile(&mut self, name: &str) -> Option<&mut PlayerInfo> {
        // 检查是否已存在（不区分大小写）
        if self.find_key(name).is_some() {
            return None;
        }

        let mut profile = PlayerInfo::new();
        profile.name = name.to_string();
        profile.m_id = self.m_next_profile_id;
        profile.m_use_seq = self.m_next_profile_use_seq;

        self.m_next_profile_id += 1;
        self.m_next_profile_use_seq += 1;

        self.m_profile_map.insert(name.to_string(), profile);
        self.delete_old_profiles();

        self.m_profile_map.get_mut(name)
    }

    /// 获取任意档案（第一个）
    pub fn get_any_profile(&mut self) -> Option<&mut PlayerInfo> {
        if self.m_profile_map.is_empty() {
            return None;
        }

        // 获取第一个档案（BTreeMap 迭代顺序为键的字母序）
        let first_key = self.m_profile_map.keys().next().cloned()?;
        let profile = self.m_profile_map.get_mut(&first_key)?;
        profile.m_use_seq = self.m_next_profile_use_seq;
        self.m_next_profile_use_seq += 1;
        Some(profile)
    }

    /// 删除指定名称的档案
    pub fn delete_profile(&mut self, name: &str) -> bool {
        let key = match self.find_key(name) {
            Some(k) => k,
            None => return false,
        };
        self.m_profile_map.remove(&key);
        true
    }

    /// 重命名档案
    pub fn rename_profile(&mut self, old_name: &str, new_name: &str) -> bool {
        let old_key = match self.find_key(old_name) {
            Some(k) => k,
            None => return false,
        };

        // 如果新名称已存在（不区分大小写），返回 false
        if old_name != new_name && self.find_key(new_name).is_some() {
            return false;
        }

        if old_name.to_lowercase() == new_name.to_lowercase() {
            // 仅修改大小写：直接更新键
            if let Some(profile) = self.m_profile_map.remove(&old_key) {
                let mut p = profile;
                p.name = new_name.to_string();
                self.m_profile_map.insert(new_name.to_string(), p);
                return true;
            }
        } else {
            // 不同名称：插入新键，删除旧键
            if let Some(mut profile) = self.m_profile_map.remove(&old_key) {
                profile.name = new_name.to_string();
                self.m_profile_map.insert(new_name.to_string(), profile);
                return true;
            }
        }

        false
    }

    /// 获取档案映射表的可变引用
    pub fn get_profile_map(&mut self) -> &mut BTreeMap<String, PlayerInfo> {
        &mut self.m_profile_map
    }

    /// 加载档案（从持久化存储）
    /// 对应 C++ Load() — 从 userdata/users.dat 读取 DataSync 数据
    pub fn load(&mut self) {
        let a_file_name = "userdata/users.dat";
        match DataReader::open_file(Path::new(a_file_name)) {
            Some(reader) => {
                let mut a_sync = DataSync::from_reader(reader);
                self.sync_state(&mut a_sync);
            }
            None => {}
        }
    }

    /// 保存档案（到持久化存储）
    /// 对应 C++ Save() — 将 DataSync 数据写入 userdata/users.dat
    pub fn save(&mut self) {
        let mut a_sync = DataSync::from_writer(DataWriter::open_memory(0x20));
        self.sync_state(&mut a_sync);

        let _ = fs::create_dir_all("userdata");
        let a_file_name = "userdata/users.dat";
        if let Some(writer) = a_sync.get_writer_mut() {
            writer.write_to_file(Path::new(a_file_name));
        }
    }

    /// 同步档案状态（对应 C++ SyncState：版本 + 档案数 + 逐个 SyncSummary）
    pub fn sync_state(&mut self, the_sync: &mut DataSync) {
        let is_reader = the_sync.get_reader().is_some();

        // 版本号（对应 C++ gProfileVersion = 14）
        let mut a_version: u32 = 14;
        the_sync.sync_u32(&mut a_version);
        the_sync.set_version(a_version as i32);
        if a_version != 14 {
            return;
        }

        if is_reader {
            self.m_profile_map.clear();
            let mut a_max_profile_id = 0u32;
            let mut a_max_use_seq = 0u32;
            let mut a_profile_count = the_sync.get_reader_mut().map(|r| r.read_u16()).unwrap_or(0);
            while a_profile_count > 0 {
                let mut a_profile = PlayerInfo::new();
                a_profile.sync_summary(the_sync);
                if a_profile.m_id > a_max_profile_id {
                    a_max_profile_id = a_profile.m_id;
                }
                if a_profile.m_use_seq > a_max_use_seq {
                    a_max_use_seq = a_profile.m_use_seq;
                }
                let name = a_profile.name.clone();
                self.m_profile_map.insert(name, a_profile);
                a_profile_count -= 1;
            }
            self.m_next_profile_id = a_max_profile_id + 1;
            self.m_next_profile_use_seq = a_max_use_seq + 1;
        } else {
            if let Some(writer) = the_sync.get_writer_mut() {
                writer.write_u16(self.m_profile_map.len() as u16);
            }
            // 逐个档案写入 SyncSummary（BTreeMap 按键序，与 C++ map 一致）
            for (_name, profile) in self.m_profile_map.iter_mut() {
                profile.sync_summary(the_sync);
            }
        }
    }

    // ======== 内部方法 ========

    /// 不区分大小写查找键
    fn find_key(&self, name: &str) -> Option<String> {
        let lower = name.to_lowercase();
        self.m_profile_map
            .keys()
            .find(|k| k.to_lowercase() == lower)
            .cloned()
    }

    /// 删除最旧的档案（m_use_seq 最小）
    fn delete_oldest_profile(&mut self) {
        let oldest_key = self
            .m_profile_map
            .iter()
            .min_by_key(|(_, p)| p.m_use_seq)
            .map(|(k, _)| k.clone());

        if let Some(key) = oldest_key {
            self.m_profile_map.remove(&key);
        }
    }

    /// 清理过期档案（保持最多 200 个）
    fn delete_old_profiles(&mut self) {
        while self.m_profile_map.len() > 200 {
            self.delete_oldest_profile();
        }
    }
}

impl Default for ProfileMgr {
    fn default() -> Self {
        Self::new()
    }
}
