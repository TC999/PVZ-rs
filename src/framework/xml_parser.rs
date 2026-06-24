// PvZ Portable Rust 翻译 — XMLParser XML 解析器
// 对应 C++ SexyAppFramework/misc/XMLParser.h / XMLParser.cpp
//
// 流式 XML 解析器，逐个读取 XML 元素（Start/End/Element/Comment/Instruction）

#![allow(dead_code)]

use std::collections::HashMap;

/// XML 参数（对应 C++ XMLParam）
#[derive(Debug, Clone)]
pub struct XMLParam {
    pub key: String,
    pub value: String,
}

pub type XMLParamMap = HashMap<String, String>;

/// XML 元素（对应 C++ XMLElement）
#[derive(Debug, Clone)]
pub struct XMLElement {
    pub elem_type: i32,
    pub section: String,
    pub value: String,
    pub instruction: String,
    pub attributes: XMLParamMap,
}

impl XMLElement {
    pub const TYPE_NONE: i32 = 0;
    pub const TYPE_START: i32 = 1;
    pub const TYPE_END: i32 = 2;
    pub const TYPE_ELEMENT: i32 = 3;
    pub const TYPE_INSTRUCTION: i32 = 4;
    pub const TYPE_COMMENT: i32 = 5;

    pub fn new() -> Self {
        XMLElement {
            elem_type: 0,
            section: String::new(),
            value: String::new(),
            instruction: String::new(),
            attributes: HashMap::new(),
        }
    }
}

/// XML 解析器（对应 C++ XMLParser）
///
/// 流式解析器：先 open_file 加载 XML 内容，然后循环调用 next_element
/// 获取每个元素，直到返回 false。
pub struct XMLParser {
    /// 原始文件名（用于错误信息）
    file_name: String,
    /// 错误信息
    error_text: String,
    /// 当前行号
    line_num: i32,
    /// 是否遇到错误
    has_failed: bool,
    /// 整个文件的字节缓存
    buf: Vec<u8>,
    /// 当前解析位置
    pos: usize,
    /// 是否已完成解析（读完或遇到错误）
    done: bool,
    /// 是否在元素内部（用于跳过已解析的开始标签体）
    in_element: bool,
}

impl XMLParser {
    pub fn new() -> Self {
        XMLParser {
            file_name: String::new(),
            error_text: String::new(),
            line_num: 1,
            has_failed: false,
            buf: Vec::new(),
            pos: 0,
            done: false,
            in_element: false,
        }
    }

    /// 打开 XML 文件并读取全部内容到缓冲区
    pub fn open_file(&mut self, file_name: &str) -> bool {
        self.file_name = file_name.to_string();
        self.line_num = 1;
        self.pos = 0;
        self.done = false;
        self.has_failed = false;
        self.error_text.clear();
        self.in_element = false;

        // 尝试从 PAK 或文件系统读取
        let content = crate::framework::paklib::with_pak_interface(|pak| {
            pak.load_file(file_name)
        });

        let data = match content {
            Some(d) => d,
            None => {
                // 兜底：从真实文件系统读取
                match std::fs::read(file_name) {
                    Ok(d) => d,
                    Err(e) => {
                        self.error_text = format!("无法打开 XML 文件 '{}': {}", file_name, e);
                        self.has_failed = true;
                        return false;
                    }
                }
            }
        };

        // 检查是否有 BOM 并跳过
        let start = if data.len() >= 3 && data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF {
            3
        } else {
            0
        };

        self.buf = data[start..].to_vec();
        true
    }

    /// 提供缓冲区（用于测试或文件已在内存中的情况）
    pub fn open_buffer(&mut self, file_name: &str, data: &[u8]) {
        self.file_name = file_name.to_string();
        self.line_num = 1;
        self.pos = 0;
        self.done = false;
        self.has_failed = false;
        self.error_text.clear();
        self.in_element = false;
        self.buf = data.to_vec();
    }

    /// 读取下一个 XML 元素
    ///
    /// 返回 true 表示成功读取了一个元素，false 表示遇到结束或错误。
    pub fn next_element(&mut self, element: &mut XMLElement) -> bool {
        // 重置元素
        element.elem_type = XMLElement::TYPE_NONE;
        element.section.clear();
        element.value.clear();
        element.instruction.clear();
        element.attributes.clear();

        if self.done || self.has_failed || self.pos >= self.buf.len() {
            return false;
        }

        // 跳过空白和注释，直到找到 '<' 或文件结束
        loop {
            if self.pos >= self.buf.len() {
                self.done = true;
                return false;
            }

            let c = self.buf[self.pos] as char;

            // 跳过空白字符
            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                if c == '\n' {
                    self.line_num += 1;
                }
                self.pos += 1;
                continue;
            }

            // 找到 '<'
            if c == '<' {
                break;
            }

            // 遇到非空白非 '<' 字符：错误
            self.error_text = format!(
                "第 {} 行: 期望 '<'，但找到字符 '{}'",
                self.line_num, c
            );
            self.has_failed = true;
            return false;
        }

        // 现在 pos 指向 '<'
        self.pos += 1; // 跳过 '<'
        if self.pos >= self.buf.len() {
            self.error_text = format!("第 {} 行: XML 文件意外结束", self.line_num);
            self.has_failed = true;
            return false;
        }

        let next = self.buf[self.pos] as char;

        match next {
            '/' => {
                // 结束标签 </...>
                self.pos += 1; // 跳过 '/'
                self.parse_end_tag(element)
            }
            '?' => {
                // 处理指令 <?...?>
                self.pos += 1; // 跳过 '?'
                self.parse_instruction(element)
            }
            '!' => {
                // 注释或 DOCTYPE
                self.pos += 1; // 跳过 '!'
                if self.pos < self.buf.len() && self.buf[self.pos] as char == '-' {
                    self.parse_comment(element)
                } else {
                    // 跳过 DOCTYPE 直到 '>'
                    self.skip_doctype()
                }
            }
            _ => {
                // 开始标签或自闭合标签 <name ... /> 或 <name ...>
                self.parse_start_or_element(element, next)
            }
        }
    }

    // ================ 内部解析方法 ================

    /// 解析结束标签：</tagname>
    fn parse_end_tag(&mut self, element: &mut XMLElement) -> bool {
        element.elem_type = XMLElement::TYPE_END;

        // 读取标签名直到 '>'
        while self.pos < self.buf.len() {
            let c = self.buf[self.pos] as char;
            if c == '>' {
                self.pos += 1; // 跳过 '>'
                return true;
            }
            if c == '\n' {
                self.line_num += 1;
            }
            // 跳过空白
            if !c.is_whitespace() {
                element.section.push(c);
            }
            self.pos += 1;
        }

        self.error_text = format!("第 {} 行: 结束标签未闭合", self.line_num);
        self.has_failed = true;
        false
    }

    /// 解析处理指令：<?...?>
    fn parse_instruction(&mut self, element: &mut XMLElement) -> bool {
        element.elem_type = XMLElement::TYPE_INSTRUCTION;

        // 读取直到 '?>'
        while self.pos < self.buf.len() {
            if self.buf[self.pos] as char == '?' && self.pos + 1 < self.buf.len()
                && self.buf[self.pos + 1] as char == '>'
            {
                self.pos += 2; // 跳过 '?>'
                return true;
            }
            if self.buf[self.pos] as char == '\n' {
                self.line_num += 1;
            }
            element.instruction.push(self.buf[self.pos] as char);
            self.pos += 1;
        }

        self.error_text = format!("第 {} 行: 处理指令未闭合", self.line_num);
        self.has_failed = true;
        false
    }

    /// 解析注释：<!--...-->
    fn parse_comment(&mut self, element: &mut XMLElement) -> bool {
        element.elem_type = XMLElement::TYPE_COMMENT;

        // 期望 "--"
        if self.pos + 1 >= self.buf.len()
            || self.buf[self.pos] as char != '-'
            || self.buf[self.pos + 1] as char != '-'
        {
            self.error_text = format!("第 {} 行: 期望注释开始 '<!--'", self.line_num);
            self.has_failed = true;
            return false;
        }
        self.pos += 2; // 跳过 "--"

        // 读取直到 '-->'
        while self.pos + 2 < self.buf.len() {
            if self.buf[self.pos] as char == '-'
                && self.buf[self.pos + 1] as char == '-'
                && self.buf[self.pos + 2] as char == '>'
            {
                self.pos += 3; // 跳过 '-->'
                // C++ 兼容：注释返回 false，跳过不返回元素
                return self.next_element(element);
            }
            if self.buf[self.pos] as char == '\n' {
                self.line_num += 1;
            }
            element.value.push(self.buf[self.pos] as char);
            self.pos += 1;
        }

        self.error_text = format!("第 {} 行: 注释未闭合", self.line_num);
        self.has_failed = true;
        false
    }

    /// 跳过 DOCTYPE 等直到 '>'
    fn skip_doctype(&mut self) -> bool {
        while self.pos < self.buf.len() {
            if self.buf[self.pos] as char == '>' {
                self.pos += 1;
                // 继续读下一个元素
                let mut dummy = XMLElement::new();
                return self.next_element(&mut dummy);
            }
            if self.buf[self.pos] as char == '\n' {
                self.line_num += 1;
            }
            self.pos += 1;
        }

        self.error_text = format!("第 {} 行: DOCTYPE 未闭合", self.line_num);
        self.has_failed = true;
        false
    }

    /// 解析开始标签或自闭合元素：<name ...> 或 <name .../>
    fn parse_start_or_element(&mut self, element: &mut XMLElement, first_char: char) -> bool {
        // 读取标签名
        let mut tag_name = String::new();
        tag_name.push(first_char);
        self.pos += 1; // 跳过 first_char，避免重复读取

        while self.pos < self.buf.len() {
            let c = self.buf[self.pos] as char;
            // 标签名遇到空白、'/'、'>' 结束
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '/' || c == '>' {
                break;
            }
            tag_name.push(c);
            self.pos += 1;
        }

        element.section = tag_name;
        element.elem_type = XMLElement::TYPE_START;

        // 跳过空白，解析属性
        self.skip_whitespace();

        // 解析属性直到 '/>' 或 '>'
        let mut self_closing = false;

        loop {
            if self.pos >= self.buf.len() {
                break;
            }

            let c = self.buf[self.pos] as char;

            if c == '/' && self.pos + 1 < self.buf.len() && self.buf[self.pos + 1] as char == '>' {
                // 自闭合标签 <... />
                self_closing = true;
                self.pos += 2; // 跳过 '/>'
                break;
            }

            if c == '>' {
                self.pos += 1; // 跳过 '>'
                break;
            }

            if c == '\n' {
                self.line_num += 1;
            }

            // 跳过空白
            if c.is_whitespace() {
                self.pos += 1;
                continue;
            }

            // 解析属性
            let (key, value) = match self.parse_attribute() {
                Some(kv) => kv,
                None => return false,
            };
            element.attributes.insert(key, value);
        }

        // 如果是自闭合标签，生成 TYPE_ELEMENT（包含完整信息）
        if self_closing {
            element.elem_type = XMLElement::TYPE_ELEMENT;
        }

        true
    }

    /// 解析一个属性：name="value" 或 name='value'
    fn parse_attribute(&mut self) -> Option<(String, String)> {
        // 读取属性名
        let mut key = String::new();
        while self.pos < self.buf.len() {
            let c = self.buf[self.pos] as char;
            if c == '=' || c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '/' || c == '>' {
                break;
            }
            key.push(c);
            self.pos += 1;
        }

        if key.is_empty() {
            self.error_text = format!("第 {} 行: 空属性名", self.line_num);
            self.has_failed = true;
            return None;
        }

        // 跳过空白直到 '='
        self.skip_whitespace();
        if self.pos >= self.buf.len() || self.buf[self.pos] as char != '=' {
            self.error_text = format!("第 {} 行: 属性 '{}' 后面期望 '='", self.line_num, key);
            self.has_failed = true;
            return None;
        }
        self.pos += 1; // 跳过 '='

        // 跳过空白
        self.skip_whitespace();

        // 读取引号
        if self.pos >= self.buf.len() {
            self.error_text = format!("第 {} 行: 属性 '{}' 的值未找到", self.line_num, key);
            self.has_failed = true;
            return None;
        }

        let quote = self.buf[self.pos] as char;
        if quote != '"' && quote != '\'' {
            self.error_text = format!("第 {} 行: 属性 '{}' 的值必须用引号括起", self.line_num, key);
            self.has_failed = true;
            return None;
        }
        self.pos += 1; // 跳过引号

        // 读取值直到匹配的引号
        let mut value = String::new();
        while self.pos < self.buf.len() {
            let c = self.buf[self.pos] as char;
            if c == quote {
                self.pos += 1; // 跳过引号
                return Some((key, value));
            }
            if c == '\n' {
                self.line_num += 1;
            }
            value.push(c);
            self.pos += 1;
        }

        self.error_text = format!("第 {} 行: 属性 '{}' 的值引号未闭合", self.line_num, key);
        self.has_failed = true;
        None
    }

    /// 跳过空白字符，更新行号
    fn skip_whitespace(&mut self) {
        while self.pos < self.buf.len() {
            let c = self.buf[self.pos] as char;
            if c == ' ' || c == '\t' || c == '\r' {
                self.pos += 1;
            } else if c == '\n' {
                self.line_num += 1;
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    // ================ 查询方法 ================

    pub fn has_failed(&self) -> bool {
        self.has_failed
    }

    pub fn get_error_text(&self) -> &str {
        &self.error_text
    }

    pub fn get_line_num(&self) -> i32 {
        self.line_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_xml() {
        let xml = r#"<Resources>
            <Image id="test_image" path="images/test.png" />
            <Sound id="test_sound" path="sounds/test.ogg" />
        </Resources>"#;

        let mut parser = XMLParser::new();
        parser.open_buffer("test.xml", xml.as_bytes());

        // 第一个元素：<Resources> (START)
        let mut elem = XMLElement::new();
        assert!(parser.next_element(&mut elem));
        assert_eq!(elem.elem_type, XMLElement::TYPE_START);
        assert_eq!(elem.section, "Resources");

        // 第二个元素：<Image ... /> (ELEMENT, 自闭合)
        let mut elem = XMLElement::new();
        assert!(parser.next_element(&mut elem));
        assert_eq!(elem.elem_type, XMLElement::TYPE_ELEMENT);
        assert_eq!(elem.section, "Image");
        assert_eq!(elem.attributes.get("id").map(|s| s.as_str()), Some("test_image"));
        assert_eq!(elem.attributes.get("path").map(|s| s.as_str()), Some("images/test.png"));

        // 第三个元素：<Sound ... /> (ELEMENT)
        let mut elem = XMLElement::new();
        assert!(parser.next_element(&mut elem));
        assert_eq!(elem.elem_type, XMLElement::TYPE_ELEMENT);
        assert_eq!(elem.section, "Sound");

        // 第四个元素：</Resources> (END)
        let mut elem = XMLElement::new();
        assert!(parser.next_element(&mut elem));
        assert_eq!(elem.elem_type, XMLElement::TYPE_END);
        assert_eq!(elem.section, "Resources");

        // 应该没有更多元素了
        let mut elem = XMLElement::new();
        assert!(!parser.next_element(&mut elem));
    }

    #[test]
    fn test_xml_with_comments() {
        let xml = r#"<Root>
            <!-- this is a comment -->
            <Child />
        </Root>"#;

        let mut parser = XMLParser::new();
        parser.open_buffer("test.xml", xml.as_bytes());

        // <Root> (START)
        let mut elem = XMLElement::new();
        assert!(parser.next_element(&mut elem));
        assert_eq!(elem.section, "Root");

        // <Child /> (ELEMENT) — 注释被跳过
        let mut elem = XMLElement::new();
        assert!(parser.next_element(&mut elem));
        assert_eq!(elem.section, "Child");
        assert_eq!(elem.elem_type, XMLElement::TYPE_ELEMENT);

        // </Root> (END)
        let mut elem = XMLElement::new();
        assert!(parser.next_element(&mut elem));
        assert_eq!(elem.section, "Root");
        assert_eq!(elem.elem_type, XMLElement::TYPE_END);
    }

    #[test]
    fn test_xml_attributes_with_apostrophe() {
        let xml = r#"<Item name='hello world' />"#;
        let mut parser = XMLParser::new();
        parser.open_buffer("test.xml", xml.as_bytes());

        let mut elem = XMLElement::new();
        assert!(parser.next_element(&mut elem));
        assert_eq!(elem.attributes.get("name").map(|s| s.as_str()), Some("hello world"));
    }
}
