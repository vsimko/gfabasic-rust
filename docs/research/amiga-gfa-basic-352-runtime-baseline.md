# Amiga GFA BASIC 3.52 runtime baseline

Status: **observed for the single currently available private binary; not normative 3.5 evidence**  
Tracking: [issue #66](https://github.com/vsimko/gfabasic-rust/issues/66), [lexer issue #46](https://github.com/vsimko/gfabasic-rust/issues/46)  
Runtime test: [workflow run #170](https://github.com/vsimko/gfabasic-rust/actions/runs/42571882635)  
Implementation: [PR #346](https://github.com/vsimko/gfabasic-rust/pull/346)

## Version and asset boundary

The project currently has one private Amiga GFA BASIC executable available for automated runtime tests. The workflow and test issue identify it as **GFA BASIC 3.52**. Record the tested executable identity from the provisioned asset and its verification metadata; do not infer that a separately labelled archive or catalogue entry proves the version of every component on its media.

The executable and Kickstart ROM are private runtime-only assets. They must not be committed to this repository or uploaded as workflow artifacts. The workflow verifies the executable's SHA-256, byte size, and Amiga HUNK header before use. Public test artifacts contain diagnostics and results, not the commercial executable or ROM.

There is no 3.5 executable available in the current test setup. Consequently, this record is **not** an observation of the normative Amiga GFA BASIC interpreter 3.5 target selected by ADR 0009.

## Reproducible probe

The CI probe creates a textual listing named `factzero.LST`:

```basic
x = FACT(0)
y = FACT(1)
z = FACT(2)
OPEN "O",#1,"DF0:FACT.OUT"
PRINT #1,x,y,z
CLOSE #1
PRINT x, y, z
```

The test then:

1. Boots the Amiga environment with the private interpreter and licensed Kickstart runtime asset.
2. Uses the interpreter's native editor Merge flow (F2) to load `factzero.LST`.
3. Uses the native Shift+F10 Run shortcut to execute the merged program.
4. Extracts `DF0:FACT.OUT` from the test floppy after emulator shutdown.
5. Asserts that the file contains `1 1 2`.

## Observed result

Run #170 passed. Its recorded result flags were:

- `EDITOR_MERGE_LST_CONTENT_VISIBLE=yes`
- `FACT_PROGRAM_OUTPUT=1 1 2`

The diagnostic artifact is [amiga-gfa-basic-cli-stdin-probe](https://github.com/vsimko/gfabasic-rust/actions/runs/42571882635/artifacts/13653826742).

This establishes that the tested 3.52 binary, in the recorded emulator setup, can merge this textual listing through its native editor workflow, execute these FACT assignments and file-output statements, and produce the asserted output. It does not establish the full syntax or semantics of any construct beyond this exact probe.

## Interpretation rules

- Treat every result in this record as **3.52-only runtime evidence**.
- Do not promote it into the Amiga 3.5 lexical contract or use it to resolve 3.5 uncertainty.
- Do not infer general identifier, comment, string, numeric-literal, operator, encoding, or line-ending rules from this small program.
- Keep source-backed manual statements, runtime observations, and implementation decisions separate.
- When another binary becomes available, record its exact component/version identity independently and rerun probes without overwriting this 3.52 record.

## Remaining blocker for 3.5

Issue #66 remains open. The next step for normative runtime validation is lawful access to the exact Amiga GFA BASIC **3.5 interpreter** media/build and a legally usable Kickstart/OS setup. Once obtained, record provenance and component identity, then run the same probe plus isolated lexical probes for unresolved cases in the [3.5 lexer evidence inventory](amiga-gfa-basic-35-lexer-evidence.md).

No copyrighted interpreter binary, disk image, or ROM is included in this repository.
