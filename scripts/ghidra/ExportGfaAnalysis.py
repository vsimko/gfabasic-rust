#@category GFA_BASIC
# Export function inventory, memory blocks, and Ghidra C-like decompilation.

import json
import os
from ghidra.app.decompiler import DecompInterface
from ghidra.util.task import ConsoleTaskMonitor

args = getScriptArgs()
if len(args) != 1:
    raise RuntimeError("Expected output directory argument")
out_dir = args[0]
if not os.path.isdir(out_dir):
    os.makedirs(out_dir)

program = getCurrentProgram()
monitor = ConsoleTaskMonitor()
decomp = DecompInterface()
decomp.openProgram(program)
functions = []
blocks = []
for block in program.getMemory().getBlocks():
    blocks.append({
        "name": block.getName(),
        "start": block.getStart().getOffset(),
        "end": block.getEnd().getOffset(),
        "size": block.getSize(),
        "initialized": block.isInitialized(),
        "execute": block.isExecute(),
        "read": block.isRead(),
        "write": block.isWrite(),
    })

c_path = os.path.join(out_dir, "decompiled.c")
c_file = open(c_path, "w")
try:
    iterator = program.getFunctionManager().getFunctions(True)
    while iterator.hasNext():
        func = iterator.next()
        entry = func.getEntryPoint().getOffset()
        size = func.getBody().getNumAddresses()
        safe_name = "".join(ch if ch.isalnum() or ch in "._-" else "_" for ch in func.getName())
        file_name = "function_%08X_%s.c" % (entry, safe_name)
        result = decomp.decompileFunction(func, 90, monitor)
        c_text = ""
        if result is not None and result.decompileCompleted():
            c_text = result.getDecompiledFunction().getC()
        with open(os.path.join(out_dir, file_name), "w") as one:
            one.write(c_text)
        functions.append({
            "entry": entry,
            "name": func.getName(),
            "size": size,
            "file": file_name,
            "decompiled": bool(c_text),
        })
        c_file.write("\n/* 0x%08X %s */\n%s\n" % (entry, func.getName(), c_text))
finally:
    c_file.close()
    decomp.dispose()

functions.sort(key=lambda item: item["entry"])
with open(os.path.join(out_dir, "functions.json"), "w") as handle:
    json.dump(functions, handle, indent=2)
with open(os.path.join(out_dir, "memory_blocks.json"), "w") as handle:
    json.dump(blocks, handle, indent=2)
