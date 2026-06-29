#!/usr/bin/env python3
"""
通用 C++ ↔ Rust API 差异比对工具（支持 JSON/YAML 输入输出）

特点：
- 自动处理 PascalCase（C++）↔ snake_case（Rust）命名转换
- 支持类方法匹配（C++ 无类前缀 vs Rust parent::method 格式）
- 正确处理空参数列表（避免将 `fn X()` 计为 1 个参数）

用法:
    # 默认 JSON 输入输出
    python3 compare_manifests.py

    # 输出 YAML 格式
    python3 compare_manifests.py --yaml

    # 自定义输入文件
    python3 compare_manifests.py --cpp cpp_funcs.yaml --rust rust_funcs.json --yaml -o diff.yaml
"""

import json
import sys
import os
import re
import argparse
from collections import defaultdict

# 已知的空实现存根函数（在 GL 模式下运行，不会调用软件渲染路径）
# 这些函数由用户确认保持空实现，不计入 mismatch
_STUB_FUNCTIONS = {
    'blt_matrix_helper', 'blt_triangles_tex_helper', 'normal_draw_line',
    'normal_draw_line_aa', 'sw_draw_triangle', 'tod_blt_matrix',
    'tod_sand_image_if_needed', 'tod_sw_tri_add_all_draw_tri_funcs',
    'ref',
}

try:
    import yaml
except ImportError:
    yaml = None

# ---------------------------------------------------------------------------
# 命名转换工具
# ---------------------------------------------------------------------------

_re_camel = re.compile(r'(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])')


def pascal_to_snake(name: str) -> str:
    """将 PascalCase 或 CamelCase 转换为 snake_case"""
    s = _re_camel.sub('_', name)
    # 处理数字后的边界（如 Check3D → check_3d, PolyFill3D → poly_fill_3d）
    s = re.sub(r'(?<=[a-zA-Z])(?=[0-9])', '_', s)
    s = re.sub(r'(?<=[0-9])(?=[a-z])', '_', s)
    s = s.lower()
    # 合并连续下划线
    s = re.sub(r'_+', '_', s)
    return s.strip('_')


def normalize_signature(sig: str) -> str:
    """去除 fn 前缀和 -> 返回类型，提取纯参数部分"""
    s = sig.strip()
    if s.startswith('fn '):
        s = s[3:]
    # 去掉函数名（第一个 '(' 之前的部分）
    if '(' in s:
        s = s[s.index('('):]
    return s


def count_rust_params(sig: str) -> int:
    """计算 Rust 签名的参数个数（正确处理空参数列表）"""
    s = sig.strip()
    if '(' not in s:
        return 0
    # 定位参数区域
    start = s.index('(')
    end = s.rindex(')')
    inside = s[start + 1:end].strip()
    if not inside:
        return 0
    # 去掉 self 参数（Rust 方法中的 &self, &mut self, self: &Self 等）
    parts = []
    for p in inside.split(','):
        p = p.strip()
        # 跳过 self 参数（支持 self, self mut:, &self, &mut self, mut self 等多种 Rust 格式）
        p_stripped = p.rsplit(':', 1)[0].strip() if ':' in p else p
        # 匹配: self, self mut, mut self, &self, &mut self
        if re.match(r'^(&?(mut\s+)?)?self(\s+mut)?$', p_stripped):
            continue
        parts.append(p)
    if not parts:
        return 0
    return len(parts)


_self_param_re = re.compile(r'^(&?(mut\s+)?)?self(\s+mut)?$')


# ---------------------------------------------------------------------------
# 主比对逻辑
# ---------------------------------------------------------------------------

def load_manifest(filepath):
    """根据扩展名自动加载 JSON 或 YAML"""
    if not os.path.exists(filepath):
        raise FileNotFoundError(f"File not found: {filepath}")
    ext = os.path.splitext(filepath)[1].lower()
    if ext in ('.yaml', '.yml'):
        if yaml is None:
            raise ImportError("PyYAML is required to read YAML files. Install with: pip install pyyaml")
        with open(filepath, 'r', encoding='utf-8') as f:
            return yaml.safe_load(f)
    else:
        with open(filepath, 'r', encoding='utf-8') as f:
            return json.load(f)


def extract_cpp_params(signature):
    """从 C++ 签名中提取参数个数（正确处理嵌套括号，如 const TriVertex (*)[3]）"""
    if '(' not in signature:
        return 0
    start = signature.index('(')
    depth = 0
    inside_chars = []
    for ch in signature[start + 1:]:
        if ch == '(':
            depth += 1
        elif ch == ')':
            if depth == 0:
                break
            depth -= 1
        inside_chars.append(ch)
    inside = ''.join(inside_chars).strip()
    if not inside:
        return 0
    return inside.count(',') + 1


def normalize_and_index(entries, is_rust=False):
    """
    构建规范化索引：
    - key: 标准化后的 snake_case 名称
    - value: 原始条目信息列表（可能有多个同名不同类）
    对 Rust 方法额外生成 `parent::method` 格式的规范化键。
    """
    index = defaultdict(list)

    for e in entries:
        name = e.get('name', '')
        parent = e.get('parent', '') or ''
        kind = e.get('kind', 'free')
        sig = e.get('signature', '')

        if is_rust:
            params = count_rust_params(sig)
            # 生成多个索引键以便匹配
            snake_key = pascal_to_snake(name).lower()
            # 去掉所有下划线的紧凑版本
            compact_key = snake_key.replace('_', '')
            
            def add_rust_entry(index, key, info):
                """将 Rust 条目注册到索引"""
                index[key].append(info)
            
            # Rust 键：方法的 parent::method
            if kind == 'method' and parent:
                entry_info = {
                    'signature': sig,
                    'params': params,
                    'file': e.get('file', ''),
                    'line': e.get('line', 0),
                    'original_key': snake_key,
                    'parent': parent,
                    'rust_name': name,
                }
                # 用 snake_case 名注册（匹配 C++ pascal_to_snake 转换）
                add_rust_entry(index, snake_key, entry_info)
                # 用原名小写注册（兼容直接小写匹配）
                if name.lower() != snake_key:
                    add_rust_entry(index, name.lower(), entry_info)
                # 用 parent::method 形式注册
                rust_key = f"{parent.lower()}::{snake_key}"
                add_rust_entry(index, rust_key, {**entry_info, 'original_key': rust_key})
                # 用紧凑 key 注册
                if compact_key != snake_key:
                    add_rust_entry(index, compact_key, entry_info)
            else:
                entry_info = {
                    'signature': sig,
                    'params': params,
                    'file': e.get('file', ''),
                    'line': e.get('line', 0),
                    'original_key': snake_key,
                    'parent': '',
                    'rust_name': name,
                }
                # 用 snake_case 名注册
                add_rust_entry(index, snake_key, entry_info)
                # 用原名小写注册
                if name.lower() != snake_key:
                    add_rust_entry(index, name.lower(), entry_info)
                # 用紧凑 key 注册
                if compact_key != snake_key:
                    add_rust_entry(index, compact_key, entry_info)
        else:
            params = extract_cpp_params(sig)
            snake_key = pascal_to_snake(name).lower()
            index[snake_key].append({
                'signature': sig,
                'params': params,
                'file': e.get('file', ''),
                'line': e.get('line', 0),
                'original_key': name,
            })
            # 也加原名小写
            if name.lower() != snake_key:
                index[name.lower()].append({
                    'signature': sig,
                    'params': params,
                    'file': e.get('file', ''),
                    'line': e.get('line', 0),
                    'original_key': name,
                })

    return index


def _find_best_rust_match(cpp_name, cpp_params, rust_index):
    """在 Rust 条目中查找与 C++ 函数匹配的最佳候选。

    匹配规则：
    1. 先按 snake_case 名称匹配
    2. 尝试去掉常见前缀（mix_, tod_ 等）后匹配
    3. 如果 C++ 名称包含类前缀（如 Board::CountSunFlowers），也匹配
    """
    candidates = []

    # 生成要搜索的键
    base_key = pascal_to_snake(cpp_name).lower()
    search_keys = [base_key]

    # 如果有类前缀，也尝试只看函数名
    if '::' in cpp_name:
        parts = cpp_name.split('::')
        search_keys.append(pascal_to_snake(parts[-1]).lower())

    # 去掉所有下划线的紧凑版本（处理单词拆分差异，如 count_sun_flowers vs count_sunflowers）
    compact_key = base_key.replace('_', '')
    if compact_key != base_key:
        search_keys.append(compact_key)

    # 原名小写（保留原始下划线）
    raw_lower = cpp_name.lower()
    if raw_lower not in search_keys:
        search_keys.append(raw_lower)
        # 原名小写的紧凑版本
        raw_compact = raw_lower.replace('_', '')
        if raw_compact != raw_lower and raw_compact not in search_keys:
            search_keys.append(raw_compact)

    # 尝试去掉前几个词（类名前缀），如 pool_effect_initialize → initialize
    parts = base_key.split('_')
    if len(parts) > 2:
        for i in range(1, len(parts)):
            sub_key = '_'.join(parts[i:])
            if sub_key not in search_keys:
                search_keys.append(sub_key)
            sub_c = sub_key.replace('_', '')
            if sub_c not in search_keys:
                search_keys.append(sub_c)

    # 去掉常见前缀再尝试
    # Mix_ 前缀：C++ SDL_mixer API 函数，Rust 侧通常去掉前辍
    for prefix in ('mix_', 'mix__'):
        if base_key.startswith(prefix):
            search_keys.append(base_key[len(prefix):])
    # Mix_ADLMIDI_ → adlmidi_, Mix_OPNMIDI_ → opnmidi_, Mix_GME_ → gme_
    for old_p, new_p in [('mix_adlmidi_', 'adlmidi_'), ('mix_opnmidi_', 'opnmidi_'),
                          ('mix_gme_', 'gme_')]:
        if base_key.startswith(old_p):
            search_keys.append(base_key.replace(old_p, new_p, 1))

    for key in search_keys:
        if key in rust_index:
            candidates.extend(rust_index[key])

    if not candidates:
        return None

    # 从所有候选中选择参数数量最接近（优先精确匹配）的
    best = None
    best_diff = 999
    for c in candidates:
        diff = abs(c['params'] - cpp_params)
        if diff < best_diff or (diff == best_diff and c['params'] == cpp_params):
            best = c
            best_diff = diff

    return best


def _should_treat_as_extra(rust_entry, matched_set):
    """判断 Rust 条目是否应为 extra 条目"""
    key = rust_entry.get('original_key', '').lower()
    name = rust_entry.get('rust_name', '').lower()
    # 检查是否已经被匹配
    return key not in matched_set and name not in matched_set


def compare(cpp_data, rust_data):
    cpp_index = normalize_and_index(cpp_data, is_rust=False)
    rust_index = normalize_and_index(rust_data, is_rust=True)

    # 扁平化 C++ 条目
    cpp_items = {}
    for entries in cpp_index.values():
        for e in entries:
            k = e['original_key']
            if k not in cpp_items:
                cpp_items[k] = e

    missing = []
    extra = []
    mismatch = []
    matched = []
    matched_rust_keys = set()

    # 遍历 C++ 条目，在 Rust 中查找匹配
    for cpp_key, cpp_info in sorted(cpp_items.items()):
        cpp_params = cpp_info['params']
        rust_match = _find_best_rust_match(cpp_key, cpp_params, rust_index)

        if rust_match:
            rust_key = rust_match.get('original_key', '')
            rust_name = rust_match.get('rust_name', '')
            matched.append({
                'cpp_name': cpp_key,
                'rust_name': rust_key or rust_name,
                'cpp_params': cpp_params,
                'rust_params': rust_match['params'],
            })
            matched_rust_keys.add(rust_key.lower())
            matched_rust_keys.add(rust_name.lower())
            matched_rust_keys.add(pascal_to_snake(rust_name).lower())

            # 参数数不同时记录但不作为 strict mismatch
            # （允许空存根函数、不同类重载、Rust 惯用差异等）
            if cpp_params != rust_match['params']:
                # 已知的软件渲染空存根函数不计入 mismatch
                cpp_snake = pascal_to_snake(cpp_key).lower()
                if cpp_snake not in _STUB_FUNCTIONS:
                    mismatch.append({
                        'name': cpp_key,
                        'cpp_params': cpp_params,
                        'rust_params': rust_match['params'],
                        'cpp_sig': cpp_info['signature'],
                        'rust_sig': rust_match['signature'],
                    })
        else:
            missing.append({'name': cpp_key, 'cpp': cpp_info})

    # Extra: Rust 中有但 C++ 中没有（且未被匹配的）
    # 先收集所有唯一的 Rust 条目（按 file+line 去重）
    seen_extra = set()
    extra_candidates = {}
    for entries in rust_index.values():
        for e in entries:
            uid = (e.get('file', ''), e.get('line', 0))
            if uid not in seen_extra:
                seen_extra.add(uid)
                rust_name = e.get('rust_name', '')
                rust_key = e.get('original_key', '').lower() or rust_name.lower()
                # 检查是否已被匹配
                is_matched = False
                for mk in matched_rust_keys:
                    if (rust_key == mk or rust_name.lower() == mk or
                        pascal_to_snake(rust_name).lower() == mk):
                        is_matched = True
                        break
                if not is_matched:
                    key = rust_key
                    extra_candidates[key] = e  # last wins

    for name, info in sorted(extra_candidates.items()):
        extra.append({'name': name, 'rust': info})

    report = {
        'missing': missing,
        'extra': extra,
        'mismatch': mismatch,
        'matched_count': len(matched),
        'summary': {
            'cpp_total': len(set(e.get('name', '') for e in cpp_data)),
            'rust_total': len(rust_data),
            'missing_count': len(missing),
            'extra_count': len(extra),
            'mismatch_count': len(mismatch),
        }
    }
    return report


def main():
    parser = argparse.ArgumentParser(description='Compare C++ and Rust API manifests')
    parser.add_argument('--cpp', default='functions_manifest.json',
                        help='Path to C++ manifest')
    parser.add_argument('--rust', default='rust_functions_manifest.json',
                        help='Path to Rust manifest')
    parser.add_argument('-o', '--output',
                        help='Output file path')
    parser.add_argument('-y', '--yaml', action='store_true',
                        help='Force output in YAML format')
    args = parser.parse_args()

    if args.output is None:
        args.output = 'diff_report.yaml' if args.yaml else 'diff_report.json'

    if args.yaml:
        output_format = 'yaml'
    else:
        ext = os.path.splitext(args.output)[1].lower()
        output_format = 'yaml' if ext in ('.yaml', '.yml') else 'json'

    try:
        cpp_data = load_manifest(args.cpp)
        rust_data = load_manifest(args.rust)
    except Exception as e:
        print(f"Error loading manifests: {e}", file=sys.stderr)
        sys.exit(1)

    report = compare(cpp_data, rust_data)

    try:
        if output_format == 'yaml':
            if yaml is None:
                raise ImportError("PyYAML required for YAML output")
            with open(args.output, 'w', encoding='utf-8') as f:
                yaml.dump(report, f, allow_unicode=True, default_flow_style=False, sort_keys=False)
        else:
            with open(args.output, 'w', encoding='utf-8') as f:
                json.dump(report, f, indent=2, ensure_ascii=False)
    except Exception as e:
        print(f"Error writing output: {e}", file=sys.stderr)
        sys.exit(1)

    s = report['summary']
    print(f"Summary:")
    print(f"  C++ total unique names: {s['cpp_total']}")
    print(f"  Rust total entries: {s['rust_total']}")
    print(f"  Matched: {report['matched_count']}")
    print(f"  Missing (C++ only): {s['missing_count']}")
    print(f"  Extra (Rust only): {s['extra_count']}")
    print(f"  Mismatch (parameter count): {s['mismatch_count']}")
    print(f"Full report saved to {args.output}")


if __name__ == '__main__':
    main()
