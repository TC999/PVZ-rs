// PvZ Portable Rust 翻译 — TodList 链表容器
// 对应 C++ Sexy.TodLib/TodList.h
//
// 简单的侵入式链表实现，用于游戏实体的管理和遍历。

#![allow(dead_code)]

/// 侵入式链表节点（对应 C++ TodListNode）
#[derive(Debug)]
pub struct TodListNode<T> {
    pub value: T,
    pub prev: *mut TodListNode<T>,
    pub next: *mut TodListNode<T>,
}

impl<T> TodListNode<T> {
    pub fn new(value: T) -> Self {
        TodListNode { value, prev: std::ptr::null_mut(), next: std::ptr::null_mut() }
    }
}

/// 侵入式链表（对应 C++ TodList）
#[derive(Debug)]
pub struct TodList<T> {
    pub head: *mut TodListNode<T>,
    pub tail: *mut TodListNode<T>,
    pub count: i32,
}

impl<T> TodList<T> {
    pub fn new() -> Self { TodList { head: std::ptr::null_mut(), tail: std::ptr::null_mut(), count: 0 } }

    pub fn is_empty(&self) -> bool { self.head.is_null() }

    /// 在末尾添加节点
    pub fn push_back(&mut self, node: *mut TodListNode<T>) {
        unsafe {
            (*node).prev = self.tail;
            (*node).next = std::ptr::null_mut();
            if !self.tail.is_null() { (*self.tail).next = node; }
            self.tail = node;
            if self.head.is_null() { self.head = node; }
        }
        self.count += 1;
    }

    /// 从链表中移除节点
    pub fn remove(&mut self, node: *mut TodListNode<T>) {
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            if !prev.is_null() { (*prev).next = next; }
            else { self.head = next; }
            if !next.is_null() { (*next).prev = prev; }
            else { self.tail = prev; }
        }
        self.count -= 1;
    }
}

impl<T> Default for TodList<T> { fn default() -> Self { TodList::new() } }
