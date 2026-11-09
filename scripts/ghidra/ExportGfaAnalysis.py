#@category GFA_BASIC
# Export function inventory, executable instructions, references, and decompilation.

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
listing = program.getListing()
reference_manager = program.getReferenceManager()
decomp = DecompInterface()
if not decomp.openProgram(program):
    raise RuntimeError("Ghidra decompiler could not open the imported program")

functions = []
blocks = []
instruction_count = 0
reference_count = 0

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

instructions_path = os.path.join(out_dir, "instructions.jsonl")
references_path = os.path.join(out_dir, "references.jsonl")
with open(instructions_path, "w") as instruction_file, open(references_path, "w") as reference_file:
    for block in program.getMemory().getBlocks():
        if not block.isInitialized() or not block.isExecute():
            continue
        iterator = listing.getInstructions(block.getStart(), True)
        while iterator.hasNext():
            instruction = iterator.next()
            if instruction.getAddress().compareTo(block.getEnd()) > 0:
                break
            address = instruction.getAddress()
            operands = [
                instruction.getDefaultOperandRepresentation(index)
                for index in range(instruction.getNumOperands())
            ]
            instruction_file.write(json.dumps({
                "address": address.getOffset(),
                "mnemonic": instruction.getMnemonicString(),
                "operands": operands,
                "length": instruction.getLength(),
            }) + "\\n")
            instruction_count += 1
            for ref in reference_manager.getReferencesFrom(address):
                reference_file.write(json.dumps({
                    "from": address.getOffset(),
                    "to": ref.getToAddress().getOffset(),
                    "type": ref.getReferenceType().getName(),
                    "operand": ref.getOperandIndex(),
                }) + "\\n")
                reference_count += 1

c_path = os.path.join(out_dir, "decompiled.c")
with open(c_path, "w") as c_file:
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
        c_file.write("\\n/* 0x%08X %s */\\n%s\\n" % (entry, func.getName(), c_text))

decomp.dispose()
functions.sort(key=lambda item: item["entry"])
with open(os.path.join(out_dir, "functions.json"), "w") as handle:
    json.dump(functions, handle, indent=2)
with open(os.path.join(out_dir, "memory_blocks.json"), "w") as handle:
    json.dump(blocks, handle, indent=2)
with open(os.path.join(out_dir, "analysis_stats.json"), "w") as handle:
    json.dump({
        "functions": len(functions),
        "instructions": instruction_count,
        "references": reference_count,
        "executable_blocks": sum(1 for block in blocks if block["execute"]),
    }, handle, indent=2)
print("Exported %d functions, %d instructions, and %d references" %
      (len(functions), instruction_count, reference_count))
