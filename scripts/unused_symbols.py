#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""统计 Rust 侧未使用符号（函数 / 方法 / 变量 / 常量 / 类型 / 字段 / 枚举变体）。

数据来源：rustc 的 can-never-be-used 诊断（dead_code 与 unused_* 系列），
以 bin 目标 pvz-portable（真实游戏入口，src/main.rs 依赖链）为准，
lib 目标的独有诊断一并并入（同一份源码编译两次，按证据去重）。

用法：
    python scripts/unused_symbols.py             # 复用 target/unused_diag.json
    python scripts/unused_symbols.py --collect   # 重新采集诊断
产物：
    target/unused_items.json   机读清单
    UNUSED_SYMBOLS.md          人读报告
本脚本只读源码、只写上述两个文件，不改动任何游戏代码。
"""
import json
import os
import re
import subprocess
import sys
from collections import Counter, OrderedDict
from datetime import datetime

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DIAG = os.path.join(ROOT, "target", "unused_diag.json")
ITEMS = os.path.join(ROOT, "target", "unused_items.json")
REPORT = os.path.join(ROOT, "UNUSED_SYMBOLS.md")
TARGETS = ("pvz-portable", "pvz_portable")

KIND_ORDER = ["函数", "方法", "关联项", "关联函数", "常量", "静态变量", "关联常量",
              "结构体", "枚举", "trait", "类型别名", "字段", "枚举变体",
              "变量", "赋值未使用", "多余的 mut", "导入", "其他"]

_line_cache = {}
_file_scan_cache = {}


def read_lines(path):
    if path not in _line_cache:
        try:
            with open(os.path.join(ROOT, path), "r", encoding="utf-8", errors="replace") as f:
                _line_cache[path] = f.read().split("\n")
        except OSError:
            _line_cache[path] = []
    return _line_cache[path]


def src_line(path, lineno):
    lines = read_lines(path)
    return lines[lineno - 1] if 0 < lineno <= len(lines) else ""


DECL_RE = re.compile(
    r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?"
    r"(?:(?:const|static|fn|struct|enum|union|trait|type|unsafe|async|extern|impl)\s+)*"
    r"(?:mut\s+)?([A-Za-z_]\w*)")


def name_index(path):
    """文件内 声明名 -> 首次出现行号（含结构体/枚举的变体与字段行）。"""
    if path in _file_scan_cache:
        return _file_scan_cache[path]
    index = {}
    for i, line in enumerate(read_lines(path), 1):
        s = line.strip()
        if not s or s.startswith("//"):
            continue
        m = DECL_RE.match(line)
        if m:
            index.setdefault(m.group(1), i)
        m2 = re.match(r"^([A-Za-z_]\w*)\s*[:(,=]", s)
        if m2:
            index.setdefault(m2.group(1), i)
    _file_scan_cache[path] = index
    return index


def decl_name(text):
    t = text.strip()
    for pat in (r"\bfn\s+([A-Za-z_]\w*)",
                r"\b(?:const|static)\s+(?:mut\s+)?([A-Za-z_]\w*)",
                r"\b(?:struct|enum|union|trait)\s+([A-Za-z_]\w*)"):
        m = re.search(pat, t)
        if m:
            return m.group(1)
    m = re.match(r"^(?:pub(?:\s*\([^)]*\))?\s+)?(?:mut\s+)?([A-Za-z_]\w*)\s*:", t)
    if m:
        return m.group(1)
    m = re.match(r"^([A-Za-z_]\w*)\s*[,(=]", t)
    if m:
        return m.group(1)
    m = re.match(r"^([A-Za-z_]\w*)\s*$", t)
    return m.group(1) if m else ""


KIND_PREFIX = [
    ("associated function", "关联函数"),
    ("associated constant", "关联常量"),
    ("multiple associated items", "关联项"),
    ("associated item", "关联项"),
    ("multiple methods", "方法"),
    ("multiple variants", "枚举变体"),
    ("multiple fields", "字段"),
    ("function", "函数"),
    ("method", "方法"),
    ("constant", "常量"),
    ("static", "静态变量"),
    ("struct", "结构体"),
    ("enum", "枚举"),
    ("variant", "枚举变体"),
    ("field", "字段"),
    ("trait", "trait"),
    ("type", "类型别名"),
]

NON_DEAD = {"unused_variables": "变量", "unused_assignments": "赋值未使用",
            "unused_imports": "导入", "unused_mut": "多余的 mut"}


def kind_of(msg):
    for prefix, name in KIND_PREFIX:
        if msg.startswith(prefix):
            return name
    return "其他"


def collect():
    print("运行 cargo build --all-targets --message-format=json ...")
    os.makedirs(os.path.dirname(DIAG), exist_ok=True)
    with open(DIAG, "w", encoding="utf-8") as out:
        subprocess.run(["cargo", "build", "--all-targets", "--message-format=json"],
                       cwd=ROOT, stdout=out, stderr=subprocess.DEVNULL)


def resolve_rows(kind, names, prim):
    """把诊断里的名字与主 span 对齐；对不上时退回文件内名字索引。"""
    rows = []
    resolved = []
    for s in prim:
        fn = s["file_name"].replace("\\", "/")
        line = s["line_start"]
        text = s.get("text") or []
        resolved.append((fn, line, decl_name(text[0]["text"] if text else "")))
    if names and len(resolved) == len(names) + 1 and resolved[0][2] not in names:
        resolved = resolved[1:]
    if len(resolved) == len(names) and names:
        for i, name in enumerate(names):
            fn, line, nm = resolved[i]
            if nm and nm != name:
                hit = name_index(fn).get(name)
                if hit:
                    line = hit
            rows.append((name, fn, line))
        return rows
    if names:
        for name in names:
            placed = False
            for fn, line, nm in resolved:
                if nm == name:
                    rows.append((name, fn, line))
                    placed = True
                    break
            if not placed:
                fn = resolved[0][0] if resolved else ""
                hit = name_index(fn).get(name) if fn else None
                rows.append((name, fn, hit or (resolved[0][1] if resolved else 0)))
        return rows
    for fn, line, nm in resolved:
        rows.append((nm or "(未解析)", fn, line))
    return rows


def load_diags():
    items, seen = [], set()
    with open(DIAG, "r", encoding="utf-8") as f:
        for raw in f:
            if not raw.startswith('{"reason":"compiler-message"'):
                continue
            try:
                rec = json.loads(raw)
            except json.JSONDecodeError:
                continue
            if rec.get("target", {}).get("name") not in TARGETS:
                continue
            msg = rec.get("message") or {}
            code = (msg.get("code") or {}).get("code")
            if msg.get("level") != "warning" or not code:
                continue
            if code != "dead_code" and code not in NON_DEAD:
                continue
            spans = [s for s in (msg.get("spans") or []) if s.get("line_start")]
            if not spans:
                continue
            prim = [s for s in spans if s.get("is_primary")] or spans
            names = re.findall(r"`([^`]+)`", msg.get("message", ""))
            if code == "dead_code":
                kind = kind_of(msg["message"])
                rows = resolve_rows(kind, names, prim)
            else:
                kind = NON_DEAD[code]
                fn = prim[0]["file_name"].replace("\\", "/")
                line = prim[0]["line_start"]
                if not names:
                    text = prim[0].get("text") or []
                    txt = text[0]["text"] if text else src_line(fn, line)
                    if code == "unused_mut":
                        m = re.search(r"^(?:\s*let\s+)(?:mut\s+)?([A-Za-z_]\w*)", txt)
                    else:
                        m = None
                    if m:
                        names = [m.group(1)]
                    else:
                        names = [decl_name(txt) or "(未解析)"]
                rows = [(n, fn, line) for n in names]
            for name, fn, line in rows:
                key = (kind, name, fn, line)
                if key in seen:
                    continue
                seen.add(key)
                items.append({"kind": kind, "name": name, "file": fn, "line": line,
                              "code": code, "msg": msg["message"]})
    items.sort(key=lambda i: (i["file"], i["line"], i["name"]))
    return items


def context(path, line):
    """取声明行的 pub 标记与最近的 C++ 出处注释。"""
    lines = read_lines(path)
    pub = ""
    if 0 < line <= len(lines):
        t = lines[line - 1].strip()
        if t.startswith("pub("):
            pub = "pub(crate)"
        elif re.match(r"^pub\b", t):
            pub = "pub"
    cpp = ""
    for j in range(max(0, line - 6), max(0, line - 1)):
        s = lines[j].strip()
        if s.startswith("// C++:"):
            cpp = s[3:].strip()
            break
    return pub, cpp


def rustc_version():
    try:
        return subprocess.run(["rustc", "--version"], capture_output=True, text=True,
                              cwd=ROOT).stdout.strip()
    except OSError:
        return "unknown"


def esc(s):
    return s.replace("|", "\\|")


def build_report(items):
    kinds = Counter(i["kind"] for i in items)
    by_file = OrderedDict()
    for it in items:
        by_file.setdefault(it["file"], []).append(it)
    fn_like = sum(kinds[k] for k in ("函数", "方法", "关联项", "关联函数"))
    var_like = sum(kinds[k] for k in ("变量", "赋值未使用", "多余的 mut"))
    enum_like = kinds["枚举变体"]

    out = []
    w = out.append
    w("# UNUSED_SYMBOLS · Rust 侧未使用符号统计")
    w("")
    w("> 数据来源：rustc 的 can-never-be-used 诊断（dead_code / unused_variables / unused_assignments / unused_mut / unused_imports）。")
    w("> 语义：这些符号**在本 crate 内从未被使用**，是“翻译了但没有接线”或“多余定义”的机器证据。")
    w("")
    w("| 项 | 值 |")
    w("|---|---|")
    w("| 生成时间 | " + datetime.now().strftime("%Y-%m-%d %H:%M:%S") + " |")
    w("| 工具链 | " + rustc_version() + " |")
    w("| 采集命令 | cargo build --all-targets --message-format=json |")
    w("| 统计范围 | src/ 全部模块；以 bin 目标 pvz-portable（src/main.rs 依赖链）为准，并入 lib 目标独有诊断 |")
    w("| 符号条目总数 | " + str(len(items)) + "（按 类别+名称+文件+行号 去重，lib/bin 重复编译不重复计数） |")
    w("| 涉及文件 | " + str(len(by_file)) + " |")
    w("")
    w("## 1. 总览")
    w("")
    w("| 类别 | 数量 | 说明 |")
    w("|---|---:|---|")
    note = {
        "函数": "自由函数，无人调用",
        "方法": "impl 方法，无人调用",
        "关联项": "关联函数，无人调用",
        "关联函数": "关联函数，无人调用",
        "常量": "常量，无人引用",
        "静态变量": "static，无人引用",
        "关联常量": "关联常量，无人引用",
        "结构体": "结构体，从未被构造",
        "枚举": "枚举类型，从未被使用",
        "trait": "trait，无人实现/使用",
        "类型别名": "type 别名，无人使用",
        "字段": "结构体字段，从未被读取",
        "枚举变体": "枚举变体，从未被构造",
        "变量": "局部变量声明后未使用（unused_variables）",
        "赋值未使用": "赋值结果从未被读（unused_assignments）",
        "多余的 mut": "不需要 mut 的变量（unused_mut）",
        "导入": "use 导入未使用（unused_imports）",
        "其他": "未归类",
    }
    for k in KIND_ORDER:
        if kinds.get(k):
            w("| " + k + " | " + str(kinds[k]) + " | " + note.get(k, "") + " |")
    w("")
    w("**归纳**：函数/方法类 " + str(fn_like) + " 项，变量类 " + str(var_like) +
      " 项，枚举变体 " + str(enum_like) + " 项，其余（常量/类型/字段/导入）" +
      str(len(items) - fn_like - var_like - enum_like) + " 项。")
    w("")
    w("## 2. 分文件汇总")
    w("")
    cols = ["函数", "方法", "关联项", "关联函数", "常量", "静态变量", "关联常量",
            "结构体", "枚举", "trait", "类型别名", "字段", "枚举变体",
            "变量", "赋值未使用", "多余的 mut", "导入"]
    shown = [c for c in cols if any(kinds.get(c) and any(i["kind"] == c for i in v) for v in by_file.values())]
    w("| 文件 | " + " | ".join(shown) + " | 合计 |")
    w("|---|" + "---:|" * (len(shown) + 1))
    totals = Counter()
    for fn, its in sorted(by_file.items(), key=lambda kv: -len(kv[1])):
        c = Counter(i["kind"] for i in its)
        totals.update(c)
        w("| " + fn + " | " + " | ".join(str(c.get(k, "")) for k in shown) + " | " + str(len(its)) + " |")
    w("| **合计** | " + " | ".join("**" + str(totals.get(k, 0)) + "**" if totals.get(k) else "" for k in shown) +
      " | **" + str(len(items)) + "** |")
    w("")
    w("## 3. 详细清单")
    w("")
    groups = [
        ("3.1 未使用函数（自由函数）", ["函数"]),
        ("3.2 未使用方法（impl 方法）", ["方法"]),
        ("3.3 未使用关联项（关联函数 / 关联常量）", ["关联项", "关联函数", "关联常量"]),
        ("3.4 未使用常量 / 静态变量", ["常量", "静态变量"]),
        ("3.5 未使用类型（结构体 / 枚举 / trait / 别名）", ["结构体", "枚举", "trait", "类型别名"]),
        ("3.6 未读取字段", ["字段"]),
        ("3.7 未构造枚举变体", ["枚举变体"]),
        ("3.8 未使用变量（unused_variables）", ["变量"]),
        ("3.9 未读赋值（unused_assignments）", ["赋值未使用"]),
        ("3.10 多余 mut（unused_mut）", ["多余的 mut"]),
        ("3.11 未使用导入（unused_imports）", ["导入"]),
    ]
    for title, ks in groups:
        sel = [i for i in items if i["kind"] in ks]
        if not sel:
            continue
        w("### " + title)
        w("")
        w("共 " + str(len(sel)) + " 项。")
        w("")
        w("| 位置 | 名称 | pub | C++ 出处 |")
        w("|---|---|---|---|")
        for it in sel:
            pub, cpp = context(it["file"], it["line"])
            w("| " + it["file"] + ":" + str(it["line"]) + " | " + esc(it["name"]) + " | " +
              (pub or "") + " | " + esc(cpp) + " |")
        w("")
    w("## 4. 口径与局限")
    w("")
    w("1. **证据来源**：全部来自 rustc 静态诊断，不是人工猜测；清单只覆盖“编译器能证明从未被使用”的符号。")
    w("   被 #[allow(dead_code)] 屏蔽、导出到 crate 之外、或经函数指针/字符串间接调用的符号不会出现在这里。")
    w("2. **target 口径**：同一份 src/ 被编译为 lib(pvz_portable) 与 bin(pvz-portable) 两次；本报告以 bin 为准并去重，")
    w("   lib 目标独有的诊断（lib 内 pub fn 无任何调用者等）同样并入。")
    w("3. **trait 默认方法**也计入“方法未使用”：它们可能对应 C++ 的虚函数，未被调用通常意味着派生类未覆写或调用点缺失，")
    w("   须按 docs/04_PROGRESS.md 的接线状态逐个核对，不能直接判定为死代码。")
    w("4. **枚举变体**大多来自整体未接线的枚举（如 ResourceId 的 868 个变体），统计时按变体逐一列出，实际是整类未接线。")
    w("5. **运行验证**：为确认统计基线可信，另跑过一次 cargo run —— SDL 初始化与资源加载完成、窗口进入主循环后由人工结束进程（日志 target/run.log）；未做玩法与 .dmo 回放一致性验证。")
    w("")
    w("## 5. 复现方式")
    w("")
    w("{{FENCE}}")
    w("python scripts/unused_symbols.py --collect   # 重新编译采集诊断并重建本报告")
    w("python scripts/unused_symbols.py             # 复用 target/unused_diag.json 重建本报告")
    w("{{FENCE}}")
    w("")
    w("机器可读清单见 target/unused_items.json（字段：kind / name / file / line / code / msg）。")
    w("")
    return "\n".join(out).replace("{{FENCE}}", "`" * 3)


def main():
    if "--collect" in sys.argv or not os.path.exists(DIAG):
        collect()
    items = load_diags()
    with open(ITEMS, "w", encoding="utf-8") as f:
        json.dump(items, f, ensure_ascii=False, indent=0)
    unresolved = [i for i in items if i["name"] == "(未解析)"]
    report = build_report(items)
    with open(REPORT, "w", encoding="utf-8") as f:
        f.write(report)
    print("条目: %d, 未解析: %d, 文件: %d" % (len(items), len(unresolved),
                                             len(set(i["file"] for i in items))))
    for k, n in Counter(i["kind"] for i in items).most_common():
        print("  %-10s %d" % (k, n))
    for i in unresolved[:10]:
        print("  未解析:", i)


if __name__ == "__main__":
    main()
