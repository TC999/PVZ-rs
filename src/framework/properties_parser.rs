// PvZ Portable Rust 翻译 — 属性解析器
// 对应 C++ SexyAppFramework/misc/PropertiesParser.h / PropertiesParser.cpp

use crate::framework::sexy_app_base::SexyAppBase;
use crate::framework::xml_parser::{XMLElement, XMLParser};

/// 解析单个元素文本（对应 C++ PropertiesParser::ParseSingleElement，PropertiesParser.cpp）
/// 元素内容（TYPE_ELEMENT）即文本；遇到嵌套 START 报错
fn parse_single_element(parser: &mut XMLParser) -> Result<String, String> {
    loop {
        let mut a_element = XMLElement::new();
        if !parser.next_element(&mut a_element) {
            return Err("unexpected end of properties".to_string());
        }
        if a_element.elem_type == XMLElement::TYPE_START {
            return Err(format!("Unexpected Section: '{}'", a_element.value));
        } else if a_element.elem_type == XMLElement::TYPE_ELEMENT {
            return Ok(a_element.value);
        } else if a_element.elem_type == XMLElement::TYPE_END {
            return Ok(String::new());
        }
    }
}

/// 解析字符串数组（对应 C++ PropertiesParser::ParseStringArray，PropertiesParser.cpp）
fn parse_string_array(parser: &mut XMLParser) -> Result<Vec<String>, String> {
    let mut a_string_vector: Vec<String> = Vec::new();
    loop {
        let mut a_element = XMLElement::new();
        if !parser.next_element(&mut a_element) {
            return Err("unexpected end of properties".to_string());
        }
        if a_element.elem_type == XMLElement::TYPE_START {
            if a_element.value == "String" {
                a_string_vector.push(parse_single_element(parser)?);
            } else {
                return Err(format!("Invalid Section '{}'", a_element.value));
            }
        } else if a_element.elem_type == XMLElement::TYPE_END {
            return Ok(a_string_vector);
        }
    }
}

/// 解析 <Properties> 节内容（对应 C++ PropertiesParser::ParseProperties，PropertiesParser.cpp）
fn parse_properties(app: &mut SexyAppBase, parser: &mut XMLParser) -> Result<(), String> {
    loop {
        let mut a_element = XMLElement::new();
        if !parser.next_element(&mut a_element) {
            return Err("unexpected end of properties".to_string());
        }
        if a_element.elem_type == XMLElement::TYPE_START {
            match a_element.value.as_str() {
                "String" => {
                    let a_def = parse_single_element(parser)?;
                    let an_id = a_element.attributes.get("id").cloned().unwrap_or_default();
                    app.set_string(&an_id, &a_def);
                }
                "StringArray" => {
                    let a_def = parse_string_array(parser)?;
                    let an_id = a_element.attributes.get("id").cloned().unwrap_or_default();
                    app.set_string_vector(&an_id, a_def);
                }
                "Boolean" => {
                    let a_val = parse_single_element(parser)?;
                    let a_val = a_val.to_uppercase();
                    let a_bool_val;
                    if a_val == "1" || a_val == "YES" || a_val == "ON" || a_val == "TRUE" {
                        a_bool_val = true;
                    } else if a_val == "0" || a_val == "NO" || a_val == "OFF" || a_val == "FALSE" {
                        a_bool_val = false;
                    } else {
                        return Err(format!("Invalid Boolean Value: '{}'", a_val));
                    }
                    let an_id = a_element.attributes.get("id").cloned().unwrap_or_default();
                    app.set_boolean(&an_id, a_bool_val);
                }
                "Integer" => {
                    let a_val = parse_single_element(parser)?;
                    let an_int: i32 = a_val
                        .trim()
                        .parse()
                        .map_err(|_| format!("Invalid Integer Value: '{}'", a_val))?;
                    let an_id = a_element.attributes.get("id").cloned().unwrap_or_default();
                    app.set_integer(&an_id, an_int);
                }
                "Double" => {
                    let a_val = parse_single_element(parser)?;
                    let a_double: f64 = a_val
                        .trim()
                        .parse()
                        .map_err(|_| format!("Invalid Double Value: '{}'", a_val))?;
                    let an_id = a_element.attributes.get("id").cloned().unwrap_or_default();
                    app.set_double(&an_id, a_double);
                }
                _ => {
                    return Err(format!("Invalid Section '{}'", a_element.value));
                }
            }
        } else if a_element.elem_type == XMLElement::TYPE_END {
            return Ok(()); // </Properties>
        }
    }
}

/// 解析属性缓冲区（对应 C++ PropertiesParser::ParsePropertiesBuffer + DoParseProperties）
/// 顶层必须为 <Properties> 节；成功返回 Ok(())，失败返回错误文本
pub fn parse_properties_buffer(app: &mut SexyAppBase, data: &[u8]) -> Result<(), String> {
    let mut parser = XMLParser::new();
    parser.open_buffer("properties", data);

    loop {
        let mut a_element = XMLElement::new();
        if !parser.next_element(&mut a_element) {
            break;
        }
        if a_element.elem_type == XMLElement::TYPE_START {
            if a_element.value == "Properties" {
                if parse_properties(app, &mut parser).is_err() {
                    break;
                }
            } else {
                return Err(format!("Invalid Section '{}'", a_element.value));
            }
        } else if a_element.elem_type == XMLElement::TYPE_ELEMENT {
            return Err(format!("Element Not Expected '{}'", a_element.value));
        }
    }

    if parser.has_failed() {
        return Err(parser.get_error_text().to_string());
    }
    Ok(())
}
