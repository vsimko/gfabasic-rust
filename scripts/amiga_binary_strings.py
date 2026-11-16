#!/usr/bin/env python3
"""Extract printable byte strings with file offsets; discovery evidence only."""
import json
import re
import sys
from pathlib import Path

source, destination = map(Path, sys.argv[1:3])
data = source.read_bytes()
items = []
for match in re.finditer(rb"[ -~]{4,}", data):
    raw = match.group()
    items.append({"offset": match.start(), "text": raw.decode("ascii"), "length": len(raw)})
destination.parent.mkdir(parents=True, exist_ok=True)
destination.write_text(json.dumps(items, indent=2, ensure_ascii=True) + "\n")
print(f"Extracted {len(items)} printable strings")
