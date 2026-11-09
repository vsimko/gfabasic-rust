#@category GFA_BASIC
# Export function inventory, executable instructions, references, and string xrefs.

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
memory = program.getMemory()
reference_manager = program.getReferenceManager()
function_manager = program.getFunctionManager()
decomp = DecompInterface()
if not decomp.openProgram(program):
    raise RuntimeError("Ghidra decompiler could not open the imported program")

functions = []
blocks = []
instruction_count = 0
reference_count = 0
for block in memory.getBlocks():
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
unclaimed_path = os.path.join(out_dir, "unclaimed_instructions.jsonl")
references_path = os.path.join(out_dir, "references.jsonl")
unclaimed_count = 0
with open(instructions_path, "w") as instruction_file, open(unclaimed_path, "w") as unclaimed_file, open(references_path, "w") as reference_file:
    for block in memory.getBlocks():
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
            }) + "\n")
            instruction_count += 1
            if function_manager.getFunctionContaining(address) is None:
                unclaimed_file.write(json.dumps({
                    "address": address.getOffset(),
                    "mnemonic": instruction.getMnemonicString(),
                    "operands": operands,
                    "length": instruction.getLength(),
                }) + "\n")
                unclaimed_count += 1
            for ref in reference_manager.getReferencesFrom(address):
                reference_file.write(json.dumps({
                    "from": address.getOffset(),
                    "to": ref.getToAddress().getOffset(),
                    "type": ref.getReferenceType().getName(),
                    "operand": ref.getOperandIndex(),
                }) + "\n")
                reference_count += 1

# The Amiga Hunk header is 40 bytes for this verified input. Ghidra's imported
# address 0 therefore corresponds to file offset 0x28. Resolve string xrefs
# against Ghidra addresses rather than guessing from nearby decompiler labels.
string_xrefs_path = os.path.join(out_dir, "string_xrefs.jsonl")
strings_path = os.path.join(out_dir, "strings.json")
with open(strings_path, "r") as strings_file:
    strings = json.load(strings_file)
address_space = program.getAddressFactory().getDefaultAddressSpace()
string_xref_count = 0
strings_with_xrefs = 0
with open(string_xrefs_path, "w") as xref_file:
    for item in strings:
        ghidra_offset = item["offset"] - 40
        record = {
            "file_offset": item["offset"],
            "address": ghidra_offset,
            "text": item["text"],
            "length": item["length"],
            "references": [],
        }
        if ghidra_offset >= 0:
            address = address_space.getAddress(ghidra_offset)
            if memory.contains(address):
                for ref in reference_manager.getReferencesTo(address):
                    source = ref.getFromAddress()
                    function = function_manager.getFunctionContaining(source)
                    record["references"].append({
                        "from": source.getOffset(),
                        "function_entry": function.getEntryPoint().getOffset() if function else None,
                        "function_name": function.getName() if function else None,
                        "type": ref.getReferenceType().getName(),
                    })
        string_xref_count += len(record["references"])
        if record["references"]:
            strings_with_xrefs += 1
        xref_file.write(json.dumps(record, ensure_ascii=True) + "\n")

c_path = os.path.join(out_dir, "decompiled.c")
with open(c_path, "w") as c_file:
    iterator = function_manager.getFunctions(True)
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
        "string_xrefs": string_xref_count,
        "unclaimed_instructions": unclaimed_count,
        "strings_with_xrefs": strings_with_xrefs,
        "executable_blocks": sum(1 for block in blocks if block["execute"]),
    }, handle, indent=2)
print("Exported %d functions, %d instructions, %d references, %d string xrefs, and %d unclaimed instructions" %
      (len(functions), instruction_count, reference_count, string_xref_count, unclaimed_count))
