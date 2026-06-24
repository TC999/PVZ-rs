// PvZ Portable Rust 翻译 — DataArray（数据数组容器）
// 对应 C++ Sexy.TodLib/DataArray.h
//
// DataArray 是游戏中核心的容器类，使用 ID 索引对象池。
// 每个 ID 编码了索引和密钥，用于检测悬空引用。

#![allow(dead_code)]

pub fn data_array_index(the_id: u32) -> u32 { the_id & DATA_ARRAY_INDEX_MASK }
pub fn data_array_key(the_id: u32) -> u32 { the_id & DATA_ARRAY_KEY_MASK }

const DATA_ARRAY_INDEX_MASK: u32 = 65535;
const DATA_ARRAY_KEY_MASK: u32 = !0xFFFF;
const DATA_ARRAY_KEY_SHIFT: u32 = 16;
const DATA_ARRAY_MAX_SIZE: u32 = 65536;
const DATA_ARRAY_KEY_FIRST: u32 = 1;

/// DataArray 容器
pub struct DataArray<T> {
    /// 槽位：None = 空闲，Some = 已分配
    slots: Vec<Option<Slot<T>>>,
    /// 空闲链表头（索引）
    free_head: u32,
    /// 当前已分配数
    count: u32,
    /// 下一个密钥
    next_key: u32,
    /// 名称
    name: String,
}

struct Slot<T> {
    /// 对象本身
    item: T,
    /// 完整 ID（包含密钥和索引）
    id: u32,
}

impl<T> DataArray<T> {
    pub fn new() -> Self {
        DataArray {
            slots: Vec::new(),
            free_head: 0,
            count: 0,
            next_key: DATA_ARRAY_KEY_FIRST,
            name: String::new(),
        }
    }

    /// 初始化（对应 C++ DataArrayInitialize）
    pub fn initialize(&mut self, max_size: u32, name: &str) {
        self.slots = Vec::with_capacity(max_size as usize);
        for i in 0..max_size {
            self.slots.push(None);
        }
        self.free_head = 0;
        self.count = 0;
        self.next_key = 1001;
        self.name = name.to_string();
    }

    /// 释放所有资源（对应 C++ DataArrayDispose）
    pub fn dispose(&mut self) {
        self.slots.clear();
        self.free_head = 0;
        self.count = 0;
        self.name = String::new();
    }

    /// 分配新对象（对应 C++ DataArrayAlloc）
    pub fn alloc(&mut self) -> *mut T
    where
        T: Default,
    {
        assert!(self.count < self.slots.len() as u32, "Data array full: {}", self.name);

        // 从空闲链表中找到可用的索引
        let mut index = self.free_head;
        while (index as usize) < self.slots.len() && self.slots[index as usize].is_some() {
            index += 1;
        }
        // 如果中间没有空闲槽，使用末尾
        if (index as usize) >= self.slots.len() || index >= self.slots.len() as u32 {
            index = self.count; // 追加
        }

        let id = (self.next_key << DATA_ARRAY_KEY_SHIFT) | index;
        if self.next_key >= 65535 {
            self.next_key = 1;
        } else {
            self.next_key += 1;
        }

        let item: T = Default::default();
        self.slots[index as usize] = Some(Slot { item, id });
        self.count += 1;

        // 更新空闲链表头
        if self.free_head == index {
            self.free_head = index + 1;
        }

        &mut self.slots[index as usize].as_mut().unwrap().item as *mut T
    }

    /// 释放对象（对应 C++ DataArrayFree）
    pub fn free(&mut self, item: *mut T) {
        for i in 0..self.slots.len() {
            if let Some(ref mut slot) = self.slots[i] {
                let slot_item_ptr = &mut slot.item as *mut T;
                if slot_item_ptr == item {
                    self.slots[i] = None;
                    self.count -= 1;
                    if i < self.free_head as usize {
                        self.free_head = i as u32;
                    }
                    return;
                }
            }
        }
        panic!("DataArrayFree: item not found in {}", self.name);
    }

    /// 释放所有对象
    pub fn free_all(&mut self) {
        for slot in self.slots.iter_mut() {
            *slot = None;
        }
        self.free_head = 0;
        self.count = 0;
    }

    /// 获取对象 ID
    pub fn get_id(&self, item: *const T) -> u32 {
        for slot in self.slots.iter() {
            if let Some(s) = slot {
                if &s.item as *const T == item {
                    return s.id;
                }
            }
        }
        0
    }

    /// 尝试通过 ID 获取（安全版本）
    pub fn try_to_get(&self, id: u32) -> Option<&T> {
        let index = (id & DATA_ARRAY_INDEX_MASK) as usize;
        if index >= self.slots.len() {
            return None;
        }
        match &self.slots[index] {
            Some(slot) if slot.id == id => Some(&slot.item),
            _ => None,
        }
    }

    /// 尝试通过 ID 获取可变引用
    pub fn try_to_get_mut(&mut self, id: u32) -> Option<&mut T> {
        let index = (id & DATA_ARRAY_INDEX_MASK) as usize;
        if index >= self.slots.len() {
            return None;
        }
        match &mut self.slots[index] {
            Some(slot) if slot.id == id => Some(&mut slot.item),
            _ => None,
        }
    }

    /// 获取对象（带断言）
    pub fn get(&self, id: u32) -> &T {
        self.try_to_get(id).unwrap_or_else(|| panic!("DataArrayGet(0x{:x}) failed for {}", id, self.name))
    }

    /// 获取可变对象
    pub fn get_mut(&mut self, id: u32) -> &mut T {
        let index = (id & DATA_ARRAY_INDEX_MASK) as usize;
        &mut self.slots[index].as_mut().unwrap().item
    }

    /// 迭代下一个活动对象
    pub fn iterate_next(&self, current: *const T) -> *mut T {
        let start = if current.is_null() {
            0
        } else {
            let mut i = 0;
            while i < self.slots.len() {
                if let Some(slot) = &self.slots[i] {
                    if &slot.item as *const T == current {
                        break;
                    }
                }
                i += 1;
            }
            i + 1
        };

        for i in start..self.slots.len() {
            if let Some(slot) = &self.slots[i] {
                return &slot.item as *const T as *mut T;
            }
        }
        std::ptr::null_mut()
    }

    pub fn len(&self) -> u32 { self.count }
    pub fn is_empty(&self) -> bool { self.count == 0 }
    pub fn max_size(&self) -> u32 { self.slots.len() as u32 }
}

impl<T> Drop for DataArray<T> {
    fn drop(&mut self) {
        self.dispose();
    }
}

impl<T> Default for DataArray<T> {
    fn default() -> Self {
        Self::new()
    }
}
