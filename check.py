#!/usr/bin/env python3
import json
import re
import os
from collections import defaultdict

# 加载C++清单
with open('functions_manifest.json', 'r') as f:
    cpp_funcs = [item['name'] for item in json.load(f)]

# 收集Rust函数名
rust_funcs = set()
for root, dirs, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            path = os.path.join(root, file)
            with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                matches = re.findall(r'\bfn\s+(\w+)', content)
                rust_funcs.update(matches)

# 生成多种可能的变体
def variants(name):
    # 1. 原样
    yield name
    # 2. 小写
    yield name.lower()
    # 3. snake_case (标准)
    s = re.sub(r'(?<=[a-z0-9])([A-Z])', r'_\1', name)
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    yield s.lower()
    # 4. 移除所有下划线（可能合并）
    yield s.lower().replace('_', '')
    # 5. 只保留字母，忽略大小写和下划线（最宽松）
    yield re.sub(r'[^a-z]', '', name.lower())
    # 6. 如果名称以大写开头，尝试首字母小写
    yield name[0].lower() + name[1:]

# 构建映射：从Rust函数名到可能的C++原名
rust_to_cpp = defaultdict(set)
for rust_f in rust_funcs:
    # 为每个Rust函数生成可能的C++形式（反向映射）
    for cpp_f in cpp_funcs:
        if rust_f in set(variants(cpp_f)):
            rust_to_cpp[rust_f].add(cpp_f)

# 找出未匹配的C++函数
matched_cpp = set()
for rust_f, cpp_set in rust_to_cpp.items():
    matched_cpp.update(cpp_set)

missing = [f for f in cpp_funcs if f not in matched_cpp]

print(f"Total C++: {len(cpp_funcs)}")
print(f"Matched: {len(matched_cpp)}")
print(f"Still missing: {len(missing)}")

with open('real_missing.json', 'w') as f:
    json.dump(missing, f, indent=2)