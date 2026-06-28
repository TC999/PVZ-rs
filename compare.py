#!/usr/bin/env python3
"""
通用 C++ ↔ Rust API 差异比对工具
用法:
    # 默认：读取 functions_manifest.json 和 rust_functions_manifest.json，输出 diff_report.json
    python3 compare_manifests.py

    # 指定输入文件（支持 .json / .yaml / .yml）
    python3 compare_manifests.py --cpp manifest_cpp.yaml --rust rust_api.yaml

    # 输出 YAML 格式
    python3 compare_manifests.py --yaml -o diff_report.yaml

    # 完整示例
    python3 compare_manifests.py --cpp cpp_funcs.yaml --rust rust_funcs.yaml --yaml
"""

import json
import sys
import os
import re
import argparse
from collections import defaultdict

# 尝试导入 yaml，若没有则报错（仅在需要时）
try:
    import yaml
except ImportError:
    yaml = None

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
    else:  # 默认 JSON
        with open(filepath, 'r', encoding='utf-8') as f:
            return json.load(f)

def clean_cpp_name(entry):
    # 如果后续需要更精细的类名处理，可在此扩展
    return entry.get('name', '')

def extract_cpp_params(signature):
    """从 C++ 签名中提取参数个数（粗略）"""
    if '(' not in signature:
        return 0
    inside = signature.split('(')[1].split(')')[0]
    if not inside.strip():
        return 0
    return inside.count(',') + 1

def get_rust_key(entry):
    if entry.get('kind') == 'method' and entry.get('parent'):
        return f"{entry['parent']}::{entry['name']}"
    return entry.get('name', '')

def compare(cpp_data, rust_data):
    cpp_dict = {}
    for e in cpp_data:
        key = clean_cpp_name(e)
        if key not in cpp_dict:
            cpp_dict[key] = {
                'signature': e.get('signature', ''),
                'params': extract_cpp_params(e.get('signature', '')),
                'file': e.get('file', ''),
                'line': e.get('line', 0)
            }

    rust_dict = {}
    for e in rust_data:
        key = get_rust_key(e)
        if key not in rust_dict:
            rust_dict[key] = {
                'signature': e.get('signature', ''),
                'params': e.get('signature', '').count(',') + 1 if '(' in e.get('signature', '') else 0,
                'file': e.get('file', ''),
                'line': e.get('line', 0)
            }

    missing = []
    extra = []
    mismatch = []

    cpp_keys = set(cpp_dict.keys())
    rust_keys = set(rust_dict.keys())

    for key in cpp_keys - rust_keys:
        missing.append({'name': key, 'cpp': cpp_dict[key]})

    for key in rust_keys - cpp_keys:
        extra.append({'name': key, 'rust': rust_dict[key]})

    for key in cpp_keys & rust_keys:
        if cpp_dict[key]['params'] != rust_dict[key]['params']:
            mismatch.append({
                'name': key,
                'cpp_params': cpp_dict[key]['params'],
                'rust_params': rust_dict[key]['params'],
                'cpp_sig': cpp_dict[key]['signature'],
                'rust_sig': rust_dict[key]['signature']
            })

    report = {
        'missing': missing,
        'extra': extra,
        'mismatch': mismatch,
        'summary': {
            'cpp_total': len(cpp_keys),
            'rust_total': len(rust_keys),
            'missing_count': len(missing),
            'extra_count': len(extra),
            'mismatch_count': len(mismatch)
        }
    }
    return report

def main():
    parser = argparse.ArgumentParser(description='Compare C++ and Rust API manifests')
    parser.add_argument('--cpp', default='functions_manifest.json',
                        help='Path to C++ manifest (JSON or YAML)')
    parser.add_argument('--rust', default='rust_functions_manifest.json',
                        help='Path to Rust manifest (JSON or YAML)')
    parser.add_argument('-o', '--output', default='diff_report.json',
                        help='Output file path (extension determines format if not using --yaml)')
    parser.add_argument('-y', '--yaml', action='store_true',
                        help='Force output in YAML format (default: JSON)')
    args = parser.parse_args()

    try:
        cpp_data = load_manifest(args.cpp)
        rust_data = load_manifest(args.rust)
    except Exception as e:
        print(f"Error loading manifests: {e}", file=sys.stderr)
        sys.exit(1)

    report = compare(cpp_data, rust_data)

    # 决定输出格式
    output_format = 'yaml' if args.yaml else 'json'
    # 如果输出文件名以 .yaml/.yml 结尾且未明确指定 --yaml，也可自动切换，但这里优先显式参数
    if args.yaml:
        output_format = 'yaml'
    else:
        ext = os.path.splitext(args.output)[1].lower()
        if ext in ('.yaml', '.yml'):
            output_format = 'yaml'
        else:
            output_format = 'json'

    try:
        if output_format == 'yaml':
            if yaml is None:
                raise ImportError("PyYAML is required to write YAML. Install with: pip install pyyaml")
            with open(args.output, 'w', encoding='utf-8') as f:
                yaml.dump(report, f, allow_unicode=True, default_flow_style=False, sort_keys=False)
        else:
            with open(args.output, 'w', encoding='utf-8') as f:
                json.dump(report, f, indent=2, ensure_ascii=False)
    except Exception as e:
        print(f"Error writing output: {e}", file=sys.stderr)
        sys.exit(1)

    # 打印摘要
    s = report['summary']
    print(f"Summary:")
    print(f"  C++ functions: {s['cpp_total']}")
    print(f"  Rust functions: {s['rust_total']}")
    print(f"  Missing (C++ only): {s['missing_count']}")
    print(f"  Extra (Rust only): {s['extra_count']}")
    print(f"  Mismatch (parameter count): {s['mismatch_count']}")
    print(f"Full report saved to {args.output}")

if __name__ == '__main__':
    main()