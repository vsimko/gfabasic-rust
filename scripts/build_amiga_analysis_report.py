"""Build a Markdown index from the string and Ghidra exports."""
import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
strings = json.loads((out / "strings.json").read_text(encoding="utf-8"))
functions = json.loads((out / "functions.json").read_text(encoding="utf-8"))
stats = json.loads((out / "analysis_stats.json").read_text(encoding="utf-8"))
string_xrefs = [
    json.loads(line)
    for line in (out / "string_xrefs.jsonl").read_text(encoding="utf-8").splitlines()
    if line.strip()
]

report = [
    "# Amiga GFA-BASIC binary analysis",
    "",
    "Reverse-engineering evidence for GFA BASIC 3.52 only; not a verified 3.5 language specification.",
    "Strings and function labels are candidates. Command syntax requires parser/dispatch analysis and manual verification.",
    "",
    f"- Printable strings: {len(strings)}",
    f"- Analyzed functions: {len(functions)}",
    "- Architecture: Motorola 68000, big-endian",
    f"- Instructions exported: {stats['instructions']}",
    f"- References exported: {stats['references']}",
    f"- String cross-references: {stats.get('string_xrefs', 0)}",
    f"- Instructions outside known functions: {stats.get('unclaimed_instructions', 0)}",
    f"- Strings with cross-references: {stats.get('strings_with_xrefs', 0)}",
    f"- Executable memory blocks: {stats['executable_blocks']}",
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
    "## String cross-references",
    "",
    "`string_xrefs.jsonl` maps each extracted file-offset string to Ghidra references targeting its imported address. It records source addresses and containing functions where Ghidra identified one. This is a useful bridge from user-visible diagnostics/keywords to code, but a string reference alone does not prove that the string is a BASIC token or that the referencing function implements it.",
    "",
    "| String | File offset | Referencing function(s) | Source address(es) |",
    "|---|---:|---|---|",
]
for item in string_xrefs:
    if not item["references"]:
        continue
    names = sorted(set(
        ref["function_name"] or "(no containing function)"
        for ref in item["references"]
    ))
    sources = ", ".join(f'0x{ref["from"]:X}' for ref in item["references"])
    text = item["text"].replace("|", r"\|").replace("\n", " ")
    report.append(
        f'| {text} | 0x{item["file_offset"]:X} | {", ".join(names)} | {sources} |'
    )

report += [
    "",
    "## Instruction and reference indexes",
    "",
    "`instructions.jsonl` contains decoded instructions from executable memory blocks; `references.jsonl` records Ghidra references originating at those instructions. Addresses are Ghidra address-space offsets, not guaranteed file offsets.",
    "",
    "## Instructions outside known functions",
    "",
    "unclaimed_instructions.jsonl contains decoded executable instructions for which Ghidra did not assign a containing function. These are candidates for missed function boundaries, not proof of a single contiguous routine.",
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
