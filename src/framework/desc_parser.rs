// PvZ Portable Rust 翻译 — DescParser 描述文件解析器
// 对应 C++ SexyAppFramework/misc/DescParser.h / DescParser.cpp
//
// DescParser 是 PopCap 框架中的描述文件解析器，用于解析字体定义文件
// 等文本格式。支持引号字符串、嵌套列表（括号）、宏定义、注释行（#）等。
//
// FontData 继承自 DescParser，所以此模块同时定义了 DataElement 等基础类型。

use std::collections::HashMap;
use std::fs;

// ============================================================
// DataElement 层次结构
// ============================================================

/// 数据元素（对应 C++ DataElement 基类）
/// 使用枚举替代 C++ 的虚继承体系
#[derive(Debug, Clone)]
pub enum DataElement {
    /// 单值元素（对应 C++ SingleDataElement）
    Single(String),
    /// 列表元素（对应 C++ ListDataElement）
    List(Vec<DataElement>),
}

impl DataElement {
    /// 是否为列表
    pub fn is_list(&self) -> bool {
        matches!(self, DataElement::List(_))
    }

    /// 获取字符串值（仅对 Single 有效）
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DataElement::Single(s) => Some(s.as_str()),
            DataElement::List(_) => None,
        }
    }

    /// 获取列表引用（仅对 List 有效）
    pub fn as_list(&self) -> Option<&Vec<DataElement>> {
        match self {
            DataElement::List(v) => Some(v),
            DataElement::Single(_) => None,
        }
    }

    /// 获取可变列表引用
    pub fn as_list_mut(&mut self) -> Option<&mut Vec<DataElement>> {
        match self {
            DataElement::List(v) => Some(v),
            DataElement::Single(_) => None,
        }
    }
}

/// 数据类型别名（对应 C++ DataElementMap）
pub type DataElementMap = HashMap<String, DataElement>;

// ============================================================
// DescParser trait
// ============================================================

/// 命令分隔符标志
pub const CMDSEP_SEMICOLON: i32 = 1;
pub const CMDSEP_NO_INDENT: i32 = 2;

/// 描述文件解析器（对应 C++ DescParser 基类）
/// 使用 trait 替代 C++ 虚继承，派生类需实现 handle_command
pub trait DescParser {
    /// 获取当前错误信息
    fn error_message(&self) -> &str;
    /// 获取可变错误信息
    fn error_message_mut(&mut self) -> &mut String;
    /// 获取命令分隔符标志
    fn cmd_sep(&self) -> i32;
    /// 设置命令分隔符标志
    fn set_cmd_sep(&mut self, sep: i32);
    /// 获取当前行号
    fn current_line_num(&self) -> i32;
    /// 设置当前行号
    fn set_current_line_num(&mut self, num: i32);
    /// 获取当前行内容
    fn current_line(&self) -> &str;
    /// 获取可变当前行
    fn current_line_mut(&mut self) -> &mut String;
    /// 获取定义映射
    fn define_map(&self) -> &DataElementMap;
    /// 获取可变定义映射
    fn define_map_mut(&mut self) -> &mut DataElementMap;

    // ============================================================
    // 需要子类实现的虚方法
    // ============================================================

    /// 处理命令（对应 C++ HandleCommand 纯虚函数）
    /// 子类必须实现此方法
    fn handle_command(&mut self, params: &DataElement) -> bool;

    // ============================================================
    // 默认实现方法
    // ============================================================

    /// 报告错误（对应 C++ Error）
    fn error(&mut self, err: &str) -> bool {
        self.error_message_mut().push_str(err);
        false
    }

    /// 解引用（对应 C++ Dereference）
    fn dereference(&self, s: &str) -> Option<&DataElement> {
        let upper = s.to_uppercase();
        self.define_map().get(&upper)
    }

    /// 判断是否为立即数/直接量（对应 C++ IsImmediate）
    fn is_immediate(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let first = s.as_bytes()[0];
        (first >= b'0' && first <= b'9') || first == b'-' || first == b'+' || first == b'\'' || first == b'"'
    }

    /// 去除引号（对应 C++ Unquote）
    fn unquote(s: &str) -> String {
        if s.is_empty() {
            return s.to_string();
        }
        let first = s.as_bytes()[0];
        if first != b'\'' && first != b'"' {
            return s.to_string();
        }
        let quote_char = first;
        let mut result = String::new();
        let mut last_was_quote = false;

        for &b in s.as_bytes() {
            if b == quote_char {
                if last_was_quote {
                    result.push(quote_char as char);
                }
                last_was_quote = true;
            } else {
                result.push(b as char);
                last_was_quote = false;
            }
        }

        result
    }

    /// 获取值列表（解析引用和嵌套）（对应 C++ GetValues）
    fn get_values(&mut self, source: &DataElement, values: &mut Vec<DataElement>) -> bool {
        values.clear();
        match source {
            DataElement::List(list) => {
                for elem in list {
                    match elem {
                        DataElement::List(child_list) => {
                            let mut child_values = Vec::new();
                            if !self.get_values(elem, &mut child_values) {
                                return false;
                            }
                            values.push(DataElement::List(child_values));
                        }
                        DataElement::Single(s) => {
                            if !s.is_empty() {
                                if s.as_bytes()[0] == b'\'' || s.as_bytes()[0] == b'"' {
                                    values.push(DataElement::Single(Self::unquote(s)));
                                } else if Self::is_immediate(s) {
                                    values.push(DataElement::Single(s.clone()));
                                } else {
                                    let upper = s.to_uppercase();
                                    if let Some(def) = self.define_map().get(&upper) {
                                        values.push(def.clone());
                                    } else {
                                        self.error_message_mut().push_str(
                                            &format!("Unable to Dereference \"{}\"", s));
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
                true
            }
            DataElement::Single(_) => true,
        }
    }

    /// 将数据元素转为字符串（对应 C++ DataElementToString）
    fn data_element_to_string(&self, elem: &DataElement) -> String {
        match elem {
            DataElement::List(list) => {
                let parts: Vec<String> = list.iter()
                    .map(|e| self.data_element_to_string(e))
                    .collect();
                format!("({})", parts.join(", "))
            }
            DataElement::Single(s) => s.clone(),
        }
    }

    /// 从 DataElement 读取字符串值（对应 C++ DataToString）
    fn data_to_string(&self, source: &DataElement, output: &mut String) -> bool {
        output.clear();
        match source {
            DataElement::Single(s) => {
                if let Some(def) = self.dereference(s) {
                    match def {
                        DataElement::Single(def_str) => {
                            *output = Self::unquote(def_str);
                            true
                        }
                        DataElement::List(_) => false,
                    }
                } else {
                    *output = Self::unquote(s);
                    true
                }
            }
            DataElement::List(_) => false,
        }
    }

    /// 从 DataElement 读取整数值（对应 C++ DataToInt）
    fn data_to_int(&self, source: &DataElement, val: &mut i32) -> bool {
        *val = 0;
        let mut temp = String::new();
        if !self.data_to_string(source, &mut temp) {
            return false;
        }
        temp.trim().parse::<i32>().map(|v| { *val = v; true }).unwrap_or(false)
    }

    /// 从 DataElement 读取字符串向量（对应 C++ DataToStringVector）
    fn data_to_string_vector(&mut self, source: &DataElement, vec: &mut Vec<String>) -> bool {
        vec.clear();
        let values = match source {
            DataElement::List(_) => {
                let mut vals = Vec::new();
                if !self.get_values(source, &mut vals) {
                    return false;
                }
                vals
            }
            DataElement::Single(s) => {
                let def_name = s.to_uppercase();
                match self.define_map().get(&def_name) {
                    Some(DataElement::List(list)) => list.clone(),
                    Some(DataElement::Single(_)) => return false,
                    None => {
                        self.error_message_mut().push_str(
                            &format!("Unable to Dereference \"{}\"", s));
                        return false;
                    }
                }
            }
        };

        for elem in &values {
            match elem {
                DataElement::Single(s) => vec.push(s.clone()),
                DataElement::List(_) => {
                    vec.clear();
                    return false;
                }
            }
        }
        true
    }

    /// 从 DataElement 获取列表值（对应 C++ DataToList）
    fn data_to_list(&mut self, source: &DataElement, values: &mut Vec<DataElement>) -> bool {
        values.clear();
        match source {
            DataElement::List(_) => self.get_values(source, values),
            DataElement::Single(s) => {
                let upper = s.to_uppercase();
                match self.define_map().get(&upper) {
                    Some(DataElement::List(list)) => {
                        *values = list.clone();
                        true
                    }
                    _ => false,
                }
            }
        }
    }

    /// 从 DataElement 读取整数向量（对应 C++ DataToIntVector）
    fn data_to_int_vector(&mut self, source: &DataElement, vec: &mut Vec<i32>) -> bool {
        vec.clear();
        let mut str_vec = Vec::new();
        if !self.data_to_string_vector(source, &mut str_vec) {
            return false;
        }
        for s in &str_vec {
            match s.trim().parse::<i32>() {
                Ok(v) => vec.push(v),
                Err(_) => return false,
            }
        }
        true
    }

    /// 从 DataElement 读取浮点数向量（对应 C++ DataToDoubleVector）
    fn data_to_double_vector(&mut self, source: &DataElement, vec: &mut Vec<f64>) -> bool {
        vec.clear();
        let mut str_vec = Vec::new();
        if !self.data_to_string_vector(source, &mut str_vec) {
            return false;
        }
        for s in &str_vec {
            match s.trim().parse::<f64>() {
                Ok(v) => vec.push(v),
                Err(_) => return false,
            }
        }
        true
    }

    /// 将字符串解析为列表（对应 C++ ParseToList）
    /// 注意：此方法不访问 self，只使用静态辅助函数
    fn parse_to_list(s: &str, list: &mut Vec<DataElement>, expect_list_end: bool, string_pos: &mut Option<usize>) -> bool {
        let mut in_single_quotes = false;
        let mut in_double_quotes = false;
        let mut escaped = false;

        let bytes = s.as_bytes();
        let start_pos = string_pos.unwrap_or(0);
        let mut pos = start_pos;

        // 当前正在构建的单值元素的索引（在 list 中）
        let mut cur_single_idx: Option<usize> = None;

        while pos < bytes.len() {
            let mut add_single_char = false;
            let c = bytes[pos];
            pos += 1;

            let is_separator = c == b' ' || c == b'\t' || c == b'\n' || c == b',';

            if escaped {
                add_single_char = true;
                escaped = false;
            } else {
                if c == b'\'' && !in_double_quotes {
                    in_single_quotes = !in_single_quotes;
                } else if c == b'"' && !in_single_quotes {
                    in_double_quotes = !in_double_quotes;
                }

                if c == b'\\' {
                    escaped = true;
                } else if !in_single_quotes && !in_double_quotes {
                    if c == b')' {
                        if expect_list_end {
                            *string_pos = Some(pos);
                            return true;
                        } else {
                            return false; // "Unexpected List End"
                        }
                    } else if c == b'(' {
                        if cur_single_idx.is_some() {
                            return false; // "Unexpected List Start"
                        } else {
                            let mut child_list = Vec::new();
                            if !Self::parse_to_list(s, &mut child_list, true, &mut Some(pos)) {
                                return false;
                            }
                            list.push(DataElement::List(child_list));
                            // pos was updated by parse_to_list via the Some(pos) reference
                            // Re-read the actual pos from the closure
                        }
                    } else if is_separator {
                        cur_single_idx = None;
                    } else {
                        add_single_char = true;
                    }
                } else {
                    add_single_char = true;
                }
            }

            if add_single_char {
                if let Some(idx) = cur_single_idx {
                    // 追加到现有元素
                    if let Some(DataElement::Single(s_val)) = list.get_mut(idx) {
                        s_val.push(c as char);
                    }
                } else {
                    // 创建新元素
                    let new_idx = list.len();
                    list.push(DataElement::Single(String::new()));
                    if let Some(DataElement::Single(s_val)) = list.get_mut(new_idx) {
                        s_val.push(c as char);
                    }
                    cur_single_idx = Some(new_idx);
                }
            }
        }

        if in_single_quotes || in_double_quotes {
            return false; // Unterminated quotes
        }

        if expect_list_end {
            return false; // Unterminated list
        }

        *string_pos = Some(pos);
        true
    }

    /// 解析描述符文件中的单行（对应 C++ ParseDescriptorLine）
    fn parse_descriptor_line(&mut self, line: &str) -> bool {
        let mut params = Vec::new();
        if !Self::parse_to_list(line, &mut params, false, &mut None) {
            return false;
        }
        if !params.is_empty() {
            match &params[0] {
                DataElement::List(_) => return false, // "Missing Command"
                DataElement::Single(_) => {
                    let params_list = DataElement::List(params);
                    return self.handle_command(&params_list);
                }
            }
        }
        true
    }

    /// 加载描述文件（对应 C++ LoadDescriptor）
    /// 注意：此方法需要外部提供文件内容，不直接依赖 SexyAppBase
    fn load_descriptor(&mut self, file_content: &str) -> bool {
        self.set_current_line_num(0);
        self.error_message_mut().clear();
        let mut has_errors = false;

        let mut line_count = 0i32;
        let bytes = file_content.as_bytes();
        let mut index = 0usize;
        let mut buff_char: Option<u8> = None;

        while index < bytes.len() || buff_char.is_some() {
            let mut skip_line = false;
            let mut at_line_start = true;
            let mut in_single_quotes = false;
            let mut in_double_quotes = false;
            let mut escaped = false;
            let mut is_indented = false;

            loop {
                let a_char: u8;
                if let Some(bc) = buff_char.take() {
                    a_char = bc;
                } else {
                    if index >= bytes.len() {
                        break;
                    }
                    a_char = bytes[index];
                    index += 1;
                }

                if a_char != b'\r' {
                    if a_char == b'\n' {
                        line_count += 1;
                    }

                    if (a_char == b' ' || a_char == b'\t') && at_line_start {
                        is_indented = true;
                    }

                    if !at_line_start || (a_char != b' ' && a_char != b'\t' && a_char != b'\n') {
                        if at_line_start {
                            if (self.cmd_sep() & CMDSEP_NO_INDENT) != 0 && !is_indented && !self.current_line().is_empty() {
                                buff_char = Some(a_char);
                                break;
                            }

                            if a_char == b'#' {
                                skip_line = true;
                            }

                            at_line_start = false;
                        }

                        if a_char == b'\n' {
                            is_indented = false;
                            at_line_start = true;
                        }

                        if a_char == b'\n' && skip_line {
                            skip_line = false;
                        } else if !skip_line {
                            if a_char == b'\\' && (in_single_quotes || in_double_quotes) && !escaped {
                                escaped = true;
                            } else {
                                if a_char == b'\'' && !in_double_quotes && !escaped {
                                    in_single_quotes = !in_single_quotes;
                                }
                                if a_char == b'"' && !in_single_quotes && !escaped {
                                    in_double_quotes = !in_double_quotes;
                                }
                                if a_char == b';' && (self.cmd_sep() & CMDSEP_SEMICOLON) != 0 && !in_single_quotes && !in_double_quotes {
                                    break;
                                }
                                if escaped {
                                    self.current_line_mut().push('\\');
                                    escaped = false;
                                }
                                if self.current_line().is_empty() {
                                    self.set_current_line_num(line_count + 1);
                                }
                                self.current_line_mut().push(a_char as char);
                            }
                        }
                    }
                }
            }

            if !self.current_line().is_empty() {
                let line = self.current_line().to_string();
                if !self.parse_descriptor_line(&line) {
                    has_errors = true;
                    break;
                }
                self.current_line_mut().clear();
            }
        }

        self.current_line_mut().clear();
        self.set_current_line_num(0);
        !has_errors
    }
}
