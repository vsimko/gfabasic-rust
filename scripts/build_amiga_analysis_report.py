#!/usr/bin/env python3
"""Build a Markdown index from the string and Ghidra exports."""
import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
strings = json.loads((out / "strings.json").read_text(encoding="utf-8"))
functions = json.loads((out / "functions.json").read_text(encoding="utf-8"))
stats = json.loads((out / "analysis_stats.json").read_text(encoding="utf-8"))

report = [
    "# Amiga GFA-BASIC binary analysis",
    "",
    "Reverse-engineering evidence for GFA BASIC 3.52 only; not a verified 3.5 language specification.",
    "Strings and function labels are candidates. Command syntax requires parser/dispatch analysis and manual verification.",
    "",
    f"- Printable strings: {len(strings)}",
    f"- Analyzed functions: {len(functions)}",
    "- Architecture: Motorola 68000, big-endian",
    f"- Instructions exported: {stats["instructions"]}",
    f"- References exported: {stats["references"]}",
    f"- Executable memory blocks: {stats["executable_blocks"]}",
    "",
    "## Printable strings (candidate evidence)",
    "",
    "| File offset | Length | Text |",
    "|---:|---:|---|",
]
for item in strings:
    text = item["text"].replace("|", r"\|").replace("\n", " ")
    report.append(f'| 0x{item["offset"]:X} | {item["length"]} | {text} |')

report += [
    "",
    "## Instruction and reference indexes",
    "",
    "`instructions.jsonl` contains decoded instructions from executable memory blocks; `references.jsonl` records Ghidra references originating at those instructions. Addresses are Ghidra address-space offsets, not guaranteed file offsets.",
    "",
    "## Function inventory",
    "",
    "| Entry | Name | Size (bytes) | Decompilation |",
    "|---:|---|---:|---|",
]
for item in functions:
    report.append(
        f'| 0x{item["entry"]:X} | `{item["name"]}` | {item["size"]} | `{item["file"]}` |'
    )

(out / "REPORT.md").write_text("\n".join(report) + "\n", encoding="utf-8")
