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
    // Rust 所有权：本列表经 add_*_value 分配的节点（Box::into_raw），remove 或析构时释放
    // （对应 C++ TodList 的节点池 mAllocator 生命周期语义）
    owned: Vec<*mut TodListNode<T>>,
}

impl<T> TodList<T> {
    pub fn new() -> Self {
        TodList { head: std::ptr::null_mut(), tail: std::ptr::null_mut(), count: 0, owned: Vec::new() }
    }

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

    /// 在头部添加节点（对应 C++ TodList::AddHead）
    pub fn add_head(&mut self, node: *mut TodListNode<T>) {
        unsafe {
            (*node).prev = std::ptr::null_mut();
            (*node).next = self.head;
            if !self.head.is_null() { (*self.head).prev = node; }
            self.head = node;
            if self.tail.is_null() { self.tail = node; }
        }
        self.count += 1;
    }

    /// 分配并添加到头部（对应 C++ AddHead；节点所有权由本列表持有）
    pub fn add_head_value(&mut self, value: T) -> *mut TodListNode<T> {
        let node = Box::into_raw(Box::new(TodListNode::new(value)));
        self.owned.push(node);
        self.add_head(node);
        node
    }

    /// 分配并添加到末尾（对应 C++ AddTail；节点所有权由本列表持有）
    pub fn push_back_value(&mut self, value: T) -> *mut TodListNode<T> {
        let node = Box::into_raw(Box::new(TodListNode::new(value)));
        self.owned.push(node);
        self.push_back(node);
        node
    }

    /// 从头部移除并返回其值（对应 C++ RemoveHead；释放节点）
    pub fn remove_head_value(&mut self) -> Option<T> {
        let node = self.head;
        if node.is_null() { return None; }
        unsafe {
            let value = std::ptr::read(&(*node).value);
            self.remove(node);
            Some(value)
        }
    }

    /// 按值查找节点（对应 C++ TodList::Find）
    pub fn find(&self, value: T) -> Option<*mut TodListNode<T>>
    where
        T: PartialEq,
    {
        let mut node = self.head;
        while !node.is_null() {
            unsafe {
                if (*node).value == value {
                    return Some(node);
                }
                node = (*node).next;
            }
        }
        None
    }

    /// 从链表中移除节点（若由本列表分配则同时释放该节点）
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
        if let Some(idx) = self.owned.iter().position(|n| *n == node) {
            self.owned.swap_remove(idx);
            unsafe {
                let _ = Box::from_raw(node);
            }
        }
    }
}

impl<T> Drop for TodList<T> {
    fn drop(&mut self) {
        // 释放所有由本列表拥有的节点
        for node in self.owned.drain(..) {
            unsafe { let _ = Box::from_raw(node); }
        }
    }
}

impl<T> Default for TodList<T> { fn default() -> Self { TodList::new() } }
