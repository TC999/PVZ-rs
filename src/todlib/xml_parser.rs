// PvZ Portable Rust — 宽容 XML 元素解析器
//
// main.pak 中的 reanim 文件是经 Tod 工具链处理过的 XML 片段：
//   - 文件开头可能残留残缺字符（`>`、`</track>`、`t>` 等）
//   - 轨道定义为 <track><name>..</name><t>..</t>...</track>
// 本解析器不要求完整 XML 文档，只做元素级容错解析：
//   - 跳过开头残缺文本与孤立闭合标签
//   - 不配对/未闭合的元素照常返回
//   - 返回片段根下的所有子元素

#[derive(Debug, Clone)]
pub struct XmlNode {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<XmlNode>,
    pub text: String,
}

impl XmlNode {
    /// 查找第一个指定名字的子元素
    pub fn find_child(&self, name: &str) -> Option<&XmlNode> {
        self.children.iter().find(|c| c.name == name)
    }

    /// 取第一个指定名字子元素的去空白文本
    pub fn child_text(&self, name: &str) -> String {
        self.find_child(name)
            .map(|c| c.text.trim().to_string())
            .unwrap_or_default()
    }
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn peek(&self, off: usize) -> Option<char> {
        self.chars.get(self.pos + off).copied()
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek(0) {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// 读取标签名/属性名（字母数字 _ : - 和 .）
    fn read_name(&mut self) -> String {
        let mut name = String::new();
        while let Some(c) = self.peek(0) {
            if c.is_alphanumeric() || c == '_' || c == ':' || c == '-' || c == '.' {
                name.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        name
    }

    /// 解析一个元素（调用时当前位置必须是 '<'）
    /// 返回 None 表示遇到声明/注释/孤立闭合标签
    fn parse_element(&mut self) -> Option<XmlNode> {
        if self.peek(0) != Some('<') {
            return None;
        }
        self.pos += 1; // 消费 '<'
        match self.peek(0) {
            Some('?') | Some('!') => {
                // XML 声明 / 注释 / DOCTYPE：跳过到 '>'
                while self.pos < self.chars.len() && self.peek(0) != Some('>') {
                    self.pos += 1;
                }
                if self.pos < self.chars.len() {
                    self.pos += 1;
                }
                return None;
            }
            Some('/') => {
                // 孤立的闭合标签（`</track>` 残片）：消费到 '>' 后忽略
                while self.pos < self.chars.len() && self.peek(0) != Some('>') {
                    self.pos += 1;
                }
                if self.pos < self.chars.len() {
                    self.pos += 1;
                }
                return None;
            }
            _ => {}
        }

        let name = self.read_name();
        if name.is_empty() {
            return None;
        }

        let mut node = XmlNode {
            name,
            attrs: Vec::new(),
            children: Vec::new(),
            text: String::new(),
        };

        // 读取属性与标签结束
        loop {
            self.skip_ws();
            match self.peek(0) {
                Some('>') => {
                    self.pos += 1;
                    break;
                }
                Some('/') if self.peek(1) == Some('>') => {
                    self.pos += 2;
                    return Some(node); // 自闭合 <t/>
                }
                _ => {
                    let attr_name = self.read_name();
                    if attr_name.is_empty() {
                        // 无法读属性名：跳过一个字符避免死循环
                        self.pos += 1;
                        continue;
                    }
                    self.skip_ws();
                    let mut value = String::new();
                    if self.peek(0) == Some('=') {
                        self.pos += 1;
                        self.skip_ws();
                        if let Some(q) = self.peek(0) {
                            if q == '"' || q == '\'' {
                                self.pos += 1;
                                while self.pos < self.chars.len() && self.peek(0) != Some(q) {
                                    value.push(self.peek(0).unwrap());
                                    self.pos += 1;
                                }
                                if self.pos < self.chars.len() {
                                    self.pos += 1;
                                }
                            }
                        }
                    }
                    node.attrs.push((attr_name, value));
                }
            }
        }

        // 解析子内容直到闭合标签或 EOF
        loop {
            let text_start = self.pos;
            while self.pos < self.chars.len() && self.peek(0) != Some('<') {
                self.pos += 1;
            }
            if self.pos > text_start {
                node.text.push_str(&self.chars[text_start..self.pos].iter().collect::<String>());
            }
            if self.pos >= self.chars.len() {
                return Some(node); // 未闭合元素，照常返回
            }

            if self.peek(1) == Some('/') {
                // 闭合标签（含残缺的任意闭合标签）：消费到 '>' 后返回当前元素
                self.pos += 2; // 消费 '</'
                while self.pos < self.chars.len() && self.peek(0) != Some('>') {
                    self.pos += 1;
                }
                if self.pos < self.chars.len() {
                    self.pos += 1;
                }
                return Some(node);
            }

            if let Some(child) = self.parse_element() {
                node.children.push(child);
            }
            // parse_element 返回 None 时（声明/孤立闭合），继续循环
        }
    }
}

/// 解析 XML 片段，返回片段根下的所有子元素
/// 开头的残缺文本/孤立闭合标签会被自动跳过
pub fn parse_fragment(input: &str) -> Vec<XmlNode> {
    let mut p = Parser {
        chars: input.chars().collect(),
        pos: 0,
    };
    let mut roots = Vec::new();
    while p.pos < p.chars.len() {
        // 跳过 '<' 之前的残缺文本
        while p.pos < p.chars.len() && p.peek(0) != Some('<') {
            p.pos += 1;
        }
        if p.pos >= p.chars.len() {
            break;
        }
        match p.parse_element() {
            Some(node) => roots.push(node),
            None => {
                // 声明/孤立闭合标签已被 parse_element 消费，继续
            }
        }
    }
    roots
}