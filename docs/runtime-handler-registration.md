---
type: "Architecture Note"
title: "Runtime handler registration"
tags: [runtime, api-registry, architecture]
---

# Runtime handler registration

The TOML files under `api/registry/` are the canonical source of API declaration IDs, implementation status, signatures, and `implementation.handler` mappings. The runtime crate consumes a generated Rust requirement table; regenerate it with:

```sh
python3 scripts/generate_runtime_handler_requirements.py
python3 scripts/generate_runtime_handler_requirements.py --check
```

Language-core IDs use `gfa.language.<category>.<name>`; Amiga IDs use
`gfa.amiga.<category>.<name>`; Atari ST IDs use `gfa.atari-st.<category>.<name>`.
IDs are case-sensitive. Handler IDs are explicit registry values and are not
Rust symbol names.

Each runtime registration carries an API profile and a typed Rust function
signature. Resolution requires exactly one registration with the expected
profile and signature. Missing, duplicate, stale, wrong-namespace, wrong-profile,
and wrong-signature mappings are errors; there is no generic fallback handler.

CI checks that every declaration marked `implemented` or `partial` has a
supported typed signature and exactly one registered runtime handler. It also
rejects handler IDs on non-callable declarations and registrations that are not
referenced by the canonical registry. The runtime crate tests the complete
generated declaration set, including unsupported declarations, without SDL3.

The compiler's semantic API resolver uses the same runtime lookup by stable
declaration ID. Its literal-only IR lowering dispatches FACT, VARIAT, COMBIN,
SUCC, and integer-literal INT through those typed registrations.

The runtime also registers RAND, STRING$, HEX$, RND, and SQR with their exact
argument/result shapes. They are runtime-callable but not compiler-executable
yet: the current IR has no general runtime-call instruction and does not
represent strings or real values. This is a concrete compiler/IR dependency,
not end-to-end compiler support.

All ten registrations remain `partial`. The 3.5 sources directly establish
the limited call forms, but detailed RAND/RND range and seeding, negative INT
rounding, negative SQR inputs, HEX$ negative formatting, STRING$ count limits,
and numeric overflow edges are not all independently established for 3.5.
The runtime helpers expose bounded candidate behavior and return `None` for
inputs outside that subset. Linux target behavior is partial; exact 3.5
interpreter conformance remains `not-tested`. A Rust registration does not
promote evidence or historical conformance.
