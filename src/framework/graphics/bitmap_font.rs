// PvZ Portable Rust — 自包含位图字体加载器
//
// main.pak 中字体为位图字体：
//   - data/<Name>.txt   —— 字体描述（Define CharList0 / WidthList0 / RectList0 / OffsetList0 + LayerSet* 命令）
//   - data/_<Name>.png  —— 字形图集（标准 PNG，含全部字符的方形网格）
//
// 现有 image_font.rs 的 DescParser 命令名（LAYERIMAG 等）与真实描述文件（LayerSetImage 等）不一致，
// 此处提供自包含解析器 + 直接以 draw_image_src 渲染字形，稳定且不依赖 ResourceManager。

#![allow(dead_code)]

use std::collections::HashMap;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::graphics::image::Image;
use crate::framework::rect::Rect;
use crate::framework::color::Color;
use crate::framework::graphics::font::Font;

/// 单个字形的绘制信息
#[derive(Debug, Clone, Copy)]
pub struct GlyphInfo {
    /// 图集中的源矩形
    pub src_rect: Rect,
    /// 绘制偏移（相对基准点）
    pub offset_x: i32,
    pub offset_y: i32,
    /// 前进宽度（含字距）
    pub advance: i32,
}

/// 位图字体
pub struct BitmapFont {
    pub name: String,
    /// 字形图集图像（由 reanim_loader 缓存，生命周期 'static）
    pub atlas: *mut Image,
    pub ascent: i32,
    pub height: i32,
    pub point_size: i32,
    /// 字符 → 字形信息
    pub glyphs: HashMap<char, GlyphInfo>,
    /// 未定义字符的默认宽度
    pub default_width: i32,
}

// ============================================================
// 描述文件解析
// ============================================================

/// 解析后的字体描述数据
struct FontDesc {
    chars: Vec<char>,
    widths: Vec<i32>,
    rects: Vec<Rect>,
    offsets: Vec<(i32, i32)>,
    ascent: i32,
    height: i32,
    point_size: i32,
}

/// 简易 token 解析器：把描述文本按字符/数字/括号/命令切分
struct Tokenizer<'a> {
    bytes: &'a [u8],
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    /// 命令/名称（字母数字下划线）
    Word(String),
    /// 单引号字符字面量（已解码为 char）
    Char(char),
    /// 整数（可能为负）
    Int(i32),
    LParen,
    RParen,
    Semicolon,
    Comma,
}

impl<'a> Tokenizer<'a> {
    fn new(s: &'a str) -> Self {
        Tokenizer { bytes: s.as_bytes(), pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_ws();
        let c = self.peek()?;
        match c {
            b'(' => { self.pos += 1; Some(Token::LParen) }
            b')' => { self.pos += 1; Some(Token::RParen) }
            b';' => { self.pos += 1; Some(Token::Semicolon) }
            b',' => { self.pos += 1; Some(Token::Comma) }
            b'\'' => {
                self.pos += 1;
                // 字符字面量，可能含转义
                let mut ch = None;
                if let Some(c1) = self.peek() {
                    self.pos += 1;
                    if c1 == b'\\' {
                        // 转义
                        if let Some(c2) = self.peek() {
                            self.pos += 1;
                            ch = Some(match c2 {
                                b'\\' => '\\',
                                b'\'' => '\'',
                                b'"' => '"',
                                b'n' => '\n',
                                b't' => '\t',
                                _ => c2 as char,
                            });
                        }
                    } else {
                        // 可能是 UTF-8 多字节字符
                        ch = Some(self.decode_utf8(c1));
                    }
                }
                // 跳到结尾引号
                while let Some(c2) = self.peek() {
                    if c2 == b'\'' {
                        self.pos += 1;
                        break;
                    }
                    self.pos += 1;
                }
                Some(Token::Char(ch.unwrap_or('\u{FFFD}')))
            }
            b'-' | b'0'..=b'9' => {
                let start = self.pos;
                if c == b'-' {
                    self.pos += 1;
                }
                while let Some(d) = self.peek() {
                    if d.is_ascii_digit() {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                let txt = std::str::from_utf8(&self.bytes[start..self.pos]).unwrap_or("0");
                txt.parse::<i32>().ok().map(Token::Int)
            }
            _ if c.is_ascii_alphabetic() || c == b'_' => {
                let start = self.pos;
                while let Some(a) = self.peek() {
                    if a.is_ascii_alphanumeric() || a == b'_' {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                Some(Token::Word(String::from_utf8_lossy(&self.bytes[start..self.pos]).to_string()))
            }
            _ => {
                // 其它字节：跳过（如未知符号）
                self.pos += 1;
                self.next_token()
            }
        }
    }

    /// 解码单个 UTF-8 字符（c 为首字节）
    fn decode_utf8(&mut self, first: u8) -> char {
        let mut buf = vec![first];
        let need = if first >= 0xF0 { 4 } else if first >= 0xE0 { 3 } else if first >= 0xC0 { 2 } else { 1 };
        for _ in 1..need {
            if let Some(b) = self.peek() {
                buf.push(b);
                self.pos += 1;
            }
        }
        String::from_utf8_lossy(&buf).chars().next().unwrap_or('\u{FFFD}')
    }
}

/// 解析 Define 块：`Define <Name> ( ... )`，把括号内内容按 token 收集
struct DescParser {
    chars: Vec<char>,
    widths: Vec<i32>,
    rects: Vec<Rect>,
    offsets: Vec<(i32, i32)>,
    ascent: i32,
    height: i32,
    point_size: i32,
    defined: bool,
}

impl DescParser {
    fn new() -> Self {
        DescParser {
            chars: Vec::new(),
            widths: Vec::new(),
            rects: Vec::new(),
            offsets: Vec::new(),
            ascent: 0,
            height: 0,
            point_size: 12,
            defined: false,
        }
    }

    fn parse(&mut self, text: &str) {
        let mut tz = Tokenizer::new(text);
        while let Some(tok) = tz.next_token() {
            match tok {
                Token::Word(w) if w.eq_ignore_ascii_case("Define") => {
                    if let Some(Token::Word(name)) = tz.next_token() {
                        self.parse_define(&mut tz, &name);
                    }
                }
                Token::Word(w) if w.eq_ignore_ascii_case("LayerSetAscent") => {
                    // LayerSetAscent Main 14;
                    self.skip_args(&mut tz, 1);
                    if let Some(Token::Int(v)) = tz.next_token() {
                        self.ascent = v;
                    }
                }
                Token::Word(w) if w.eq_ignore_ascii_case("LayerSetHeight") => {
                    self.skip_args(&mut tz, 1);
                    if let Some(Token::Int(v)) = tz.next_token() {
                        self.height = v;
                    }
                }
                Token::Word(w) if w.eq_ignore_ascii_case("LayerSetPointSize") => {
                    self.skip_args(&mut tz, 1);
                    if let Some(Token::Int(v)) = tz.next_token() {
                        self.point_size = v;
                    }
                }
                Token::Word(w) if w.eq_ignore_ascii_case("SetDefaultPointSize") => {
                    if let Some(Token::Int(v)) = tz.next_token() {
                        self.point_size = v;
                    }
                }
                _ => {}
            }
        }
    }

    /// 跳过 n 个参数 token（用于 LayerSetXxx Main 这种）
    fn skip_args(&self, tz: &mut Tokenizer, n: usize) {
        let mut count = 0;
        while count < n {
            match tz.next_token() {
                Some(_) => count += 1,
                None => break,
            }
        }
    }

    /// 解析 `Define <name> ( ... )` 括号块
    fn parse_define(&mut self, tz: &mut Tokenizer, name: &str) {
        // 找到 '('
        loop {
            match tz.next_token() {
                Some(Token::LParen) => break,
                Some(_) => continue,
                None => return,
            }
        }

        // 收集括号内顶层 token
        let mut items: Vec<Vec<Token>> = Vec::new();
        let mut cur: Vec<Token> = Vec::new();
        let mut depth = 0;
        loop {
            match tz.next_token() {
                None => break,
                Some(Token::LParen) => { depth += 1; cur.push(Token::LParen); }
                Some(Token::RParen) => {
                    if depth == 0 {
                        break; // Define 块结束
                    }
                    depth -= 1;
                    cur.push(Token::RParen);
                }
                Some(Token::Comma) if depth == 0 => {
                    if !cur.is_empty() {
                        items.push(std::mem::take(&mut cur));
                    }
                }
                Some(t) => cur.push(t),
            }
        }
        if !cur.is_empty() {
            items.push(cur);
        }

        let upper = name.to_uppercase();
        match upper.as_str() {
            "CHARLIST0" => {
                for item in items {
                    if let Some(Token::Char(c)) = item.first() {
                        self.chars.push(*c);
                    }
                }
            }
            "WIDTHLIST0" => {
                for item in items {
                    if let Some(Token::Int(v)) = item.first() {
                        self.widths.push(*v);
                    }
                }
            }
            "RECTLIST0" => {
                for item in items {
                    // 形状: (x, y, w, h)
                    let mut nums = Vec::new();
                    for t in &item {
                        if let Token::Int(v) = t {
                            nums.push(*v);
                        }
                    }
                    if nums.len() == 4 {
                        self.rects.push(Rect::new(nums[0], nums[1], nums[2], nums[3]));
                    }
                }
            }
            "OFFSETLIST0" => {
                for item in items {
                    let mut nums = Vec::new();
                    for t in &item {
                        if let Token::Int(v) = t {
                            nums.push(*v);
                        }
                    }
                    if nums.len() == 2 {
                        self.offsets.push((nums[0], nums[1]));
                    }
                }
            }
            _ => {}
        }
        self.defined = true;
    }

    fn build(self) -> FontDesc {
        FontDesc {
            chars: self.chars,
            widths: self.widths,
            rects: self.rects,
            offsets: self.offsets,
            ascent: self.ascent,
            height: self.height,
            point_size: self.point_size,
        }
    }
}

/// 解析字体描述文本
fn parse_font_desc(text: &str) -> FontDesc {
    let mut p = DescParser::new();
    p.parse(text);
    p.build()
}

// ============================================================
// 全局字体缓存与加载
// ============================================================

/// 字体缓存（按名称索引）
static mut FONT_CACHE: Vec<(String, Option<Box<BitmapFont>>)> = Vec::new();

/// 加载位图字体（带缓存；失败返回 None）
/// 描述：data/<name>.txt，图集：data/_<name>.png
pub fn load_bitmap_font(name: &str) -> Option<*mut BitmapFont> {
    unsafe {
        for (k, v) in FONT_CACHE.iter_mut() {
            if k == name {
                return v.as_mut().map(|b| &mut **b as *mut BitmapFont);
            }
        }
    }

    let desc_path = format!("data/{}.txt", name);
    let desc_text = crate::framework::paklib::with_pak_interface(|pak| pak.load_file(&desc_path))?;
    let desc_text = String::from_utf8_lossy(&desc_text).into_owned();

    let atlas_path = format!("data/_{}.png", name);
    let atlas = crate::todlib::reanim_loader::load_image_by_path(&atlas_path)?;

    let desc = parse_font_desc(&desc_text);

    let mut glyphs = HashMap::new();
    for (i, ch) in desc.chars.iter().enumerate() {
        let width = desc.widths.get(i).copied().unwrap_or(0);
        let src_rect = desc.rects.get(i).copied().unwrap_or(Rect::ZERO);
        let (ox, oy) = desc.offsets.get(i).copied().unwrap_or((0, 0));
        glyphs.insert(*ch, GlyphInfo { src_rect, offset_x: ox, offset_y: oy, advance: width });
    }

    let font = BitmapFont {
        name: name.to_string(),
        atlas,
        ascent: desc.ascent,
        height: desc.height,
        point_size: desc.point_size,
        glyphs,
        default_width: desc.widths.first().copied().unwrap_or(12),
    };

    unsafe {
        FONT_CACHE.push((name.to_string(), Some(Box::new(font))));
        let (_, v) = FONT_CACHE.last_mut().unwrap();
        v.as_mut().map(|b| &mut **b as *mut BitmapFont)
    }
}

/// 获取已加载的字体（未加载返回 None）
pub fn get_bitmap_font(name: &str) -> Option<*mut BitmapFont> {
    unsafe {
        for (k, v) in FONT_CACHE.iter_mut() {
            if k == name {
                return v.as_mut().map(|b| &mut **b as *mut BitmapFont);
            }
        }
    }
    None
}

// ============================================================
// 全局字体（对应 C++ Sexy::FONT_* 全局 _Font* 指针）
// ============================================================

/// 全局 Font 对象缓存（Box::into_raw 持有，进程生命周期内不释放，同 C++ 全局字体）
static mut FONT_OBJ_CACHE: Vec<(String, *mut Font)> = Vec::new();

/// 惰性获取全局字体（对应 C++ ResourceManager::GetFontThrow 后赋给全局指针）
/// 已缓存直接返回；未缓存则从资源包加载位图字库并构造 Font。
pub fn font_global(name: &str) -> *mut Font {
    unsafe {
        for (k, v) in FONT_OBJ_CACHE.iter() {
            if k == name {
                return *v;
            }
        }
    }
    let bmp = load_bitmap_font(name);
    let font = bmp.map(|bmp_ptr| {
        let mut f = Font::new(name, 0);
        f.set_bitmap(bmp_ptr);
        f
    });
    let ptr = match font {
        Some(f) => Box::into_raw(Box::new(f)),
        None => std::ptr::null_mut(),
    };
    if !ptr.is_null() {
        unsafe { FONT_OBJ_CACHE.push((name.to_string(), ptr)); }
    }
    ptr
}

/// 全局字体常量（对应 C++ Resources.cpp 中 Sexy::FONT_* 全局指针）
pub static mut FONT_PICO129: *mut Font = std::ptr::null_mut();
pub static mut FONT_CONTINUUMBOLD14: *mut Font = std::ptr::null_mut();
pub static mut FONT_HOUSEOFTERROR16: *mut Font = std::ptr::null_mut();
pub static mut FONT_DWARVENTODCRAFT12: *mut Font = std::ptr::null_mut();
pub static mut FONT_BRIANNETOD16: *mut Font = std::ptr::null_mut();
pub static mut FONT_BRIANNETOD12: *mut Font = std::ptr::null_mut();
pub static mut FONT_BRIANNETOD32: *mut Font = std::ptr::null_mut();
pub static mut FONT_TINYBOLD: *mut Font = std::ptr::null_mut();

/// 初始化全局字体（对应 C++ LoadResources 中 GetFontThrow 批量赋值）
pub fn init_global_fonts() {
    unsafe {
        FONT_PICO129 = font_global("FONT_PICO129");
        FONT_CONTINUUMBOLD14 = font_global("FONT_CONTINUUMBOLD14");
        FONT_HOUSEOFTERROR16 = font_global("FONT_HOUSEOFTERROR16");
        FONT_DWARVENTODCRAFT12 = font_global("FONT_DWARVENTODCRAFT12");
        FONT_BRIANNETOD16 = font_global("FONT_BRIANNETOD16");
        FONT_BRIANNETOD12 = font_global("FONT_BRIANNETOD12");
        FONT_BRIANNETOD32 = font_global("FONT_BRIANNETOD32");
        FONT_TINYBOLD = font_global("FONT_TINYBOLD");
    }
}

// ============================================================
// 绘制
// ============================================================

impl BitmapFont {
    /// 计算字符串宽度
    pub fn string_width(&self, text: &str) -> i32 {
        let mut w = 0;
        for ch in text.chars() {
            w += match self.glyphs.get(&ch) {
                Some(g) => g.advance,
                None => self.default_width,
            };
        }
        w
    }

    /// 绘制文本（对应 C++ ImageFont::DrawString，逐字形 blit）
    /// 返回字符串宽度
    pub fn draw_text(&self, g: &mut Graphics, x: i32, y: i32, text: &str, color: &Color) -> i32 {
        if self.atlas.is_null() {
            return 0;
        }
        let atlas = unsafe { &*self.atlas };
        let prev_color = g.color;
        let prev_colorize = g.colorize_images;
        g.set_color(color);
        g.set_colorize_images(true);

        let mut cur_x = x;
        for ch in text.chars() {
            let info = match self.glyphs.get(&ch) {
                Some(gi) => *gi,
                None => GlyphInfo {
                    src_rect: Rect::ZERO,
                    offset_x: 0,
                    offset_y: 0,
                    advance: self.default_width,
                },
            };
            if info.src_rect.width > 0 && info.src_rect.height > 0 {
                // 绘制位置：x + offset，y 用 ascent 对齐基线
                let draw_x = cur_x + info.offset_x;
                let draw_y = y + info.offset_y;
                g.draw_image_src(atlas, draw_x, draw_y, &info.src_rect);
            }
            cur_x += info.advance;
        }

        g.set_colorize_images(prev_colorize);
        g.set_color(&prev_color);
        cur_x - x
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_font_desc_sample() {
        // 真实 continuumbold14.txt 格式的代表性子集
        let sample = r#"Define CharList0
 ( ' ', '!', '0', '1', 'A', 'B' );
Define WidthList0
 ( 11,  11,  11,  13,  11,   9 );
Define RectList0
 ( (   0,   0,  0,  0), (   0,   0, 27, 27), (  27,   0, 27, 27), (  54,   0, 27, 27), (  81,   0, 27, 27), ( 108,   0, 27, 27) );
Define OffsetList0
 ( (0, 0), (-11, -4), (-8, -4), (-8, -4), (-7, -4), (-8, -4) );
CreateLayer Main;
LayerSetImage Main 'ContinuumBold14';
LayerSetAscent Main 14;
LayerSetHeight Main 18;
LayerSetPointSize Main 12;
SetDefaultPointSize 12;
"#;
        let desc = parse_font_desc(sample);
        assert_eq!(desc.chars, vec![' ', '!', '0', '1', 'A', 'B']);
        assert_eq!(desc.widths, vec![11, 11, 11, 13, 11, 9]);
        assert_eq!(desc.rects.len(), 6);
        assert_eq!(desc.rects[1], Rect::new(0, 0, 27, 27));
        assert_eq!(desc.rects[3], Rect::new(54, 0, 27, 27));
        assert_eq!(desc.offsets.len(), 6);
        assert_eq!(desc.offsets[1], (-11, -4));
        assert_eq!(desc.ascent, 14);
        assert_eq!(desc.height, 18);
        assert_eq!(desc.point_size, 12);
    }
}
