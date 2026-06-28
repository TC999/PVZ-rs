#!/usr/bin/env python3
"""
通用 C++ ↔ Rust API 差异比对工具（支持 JSON/YAML 输入输出）
用法:
    # 默认 JSON 输入输出
    python3 compare_manifests.py

    # 指定输入文件（支持 .json / .yaml / .yml）
    python3 compare_manifests.py --cpp manifest_cpp.yaml --rust rust_api.yaml

    # 输出 YAML 格式（自动命名 diff_report.yaml）
    python3 compare_manifests.py --yaml

    # 自定义输出文件名（格式由 --yaml 决定）
    python3 compare_manifests.py --yaml -o my_diff.yaml
"""

import json
import sys
import os
import re
import argparse
from collections import defaultdict

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
    else:
        with open(filepath, 'r', encoding='utf-8') as f:
            return json.load(f)

def clean_cpp_name(entry):
    return entry.get('name', '')

def extract_cpp_params(signature):
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
    parser.add_argument('-o', '--output', help='Output file path (default: diff_report.json or diff_report.yaml if --yaml)')
    parser.add_argument('-y', '--yaml', action='store_true',
                        help='Force output in YAML format (default: JSON unless output filename ends with .yaml/.yml)')
    args = parser.parse_args()

    # 决定输出文件名（若未指定）
    if args.output is None:
        args.output = 'diff_report.yaml' if args.yaml else 'diff_report.json'

    # 决定输出格式
    if args.yaml:
        output_format = 'yaml'
    else:
        ext = os.path.splitext(args.output)[1].lower()
        if ext in ('.yaml', '.yml'):
            output_format = 'yaml'
        else:
            output_format = 'json'

    # 加载数据
    try:
        cpp_data = load_manifest(args.cpp)
        rust_data = load_manifest(args.rust)
    except Exception as e:
        print(f"Error loading manifests: {e}", file=sys.stderr)
        sys.exit(1)

    report = compare(cpp_data, rust_data)

    # 写入输出
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