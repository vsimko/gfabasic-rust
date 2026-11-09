#!/usr/bin/env python3
"""Build a Markdown index from the string and Ghidra exports."""
import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
strings = json.loads((out / "strings.json").read_text(encoding="utf-8"))
functions = json.loads((out / "functions.json").read_text(encoding="utf-8"))

report = [
    "# Amiga GFA-BASIC binary analysis",
    "",
    "Reverse-engineering evidence for GFA BASIC 3.52 only; not a verified 3.5 language specification.",
    "Strings and function labels are candidates. Command syntax requires parser/dispatch analysis and manual verification.",
    "",
    f"- Printable strings: {len(strings)}",
    f"- Analyzed functions: {len(functions)}",
    "- Architecture: Motorola 68000, big-endian",
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
