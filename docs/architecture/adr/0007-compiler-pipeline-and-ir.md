# ADR 0007: Compiler pipeline and typed IR boundaries

- Status: Accepted
- Date: 2026-11-04
- Decision makers: Project maintainers

## Context

The compiler needs a stable boundary between dialect-specific frontend semantics and target-specific code generation. The initial output target is Linux x86-64 and LLVM is the planned backend, but the frontend and intermediate representation must not depend on Linux, SDL3, or LLVM implementation types.

The dialect/version research is not complete. The compiler therefore needs an architecture that can preserve unresolved source distinctions and prevent an unverified construct from acquiring guessed semantics. The runtime ABI and detailed memory model are separate decisions.

## Decision

1. Use explicit compiler stages: source loading, lexing, parsing, name resolution, semantic analysis, typed project IR lowering, target/API lowering, backend code generation, object generation/linking, and artifact reporting.
2. Keep source syntax and source spans in the AST, and retain a lossless token/trivia stream or CST alongside semantic syntax. Expose them together as one parsed-document result so consumers cannot accidentally pair an AST with a different source stream. Preserve original spelling and trivia through source spans; resolve dialect-specific meaning during semantic analysis. Do not delegate language semantics to LLVM.
3. Resolve names into stable, document-local symbol identities with declaration/reference spans before semantic lowering. Keep unresolved or ambiguous names diagnostic; do not substitute textual matching. Make the project-owned IR typed, target-independent, and explicitly structured around basic blocks and terminators. Represent conversions, calls, memory operations, and source provenance explicitly.
4. Require IR verification before backend code generation. Invalid semantic or IR states must produce diagnostics and must not produce an executable.
5. Keep LLVM IR and LLVM-specific types inside the backend. The frontend and project IR must build and test without LLVM or SDL3.
6. Preserve source provenance through lowering and optimization so backend and runtime diagnostics can be mapped to original source locations.
7. Keep backend selection and target-specific lowering separate from source dialect and API-profile selection.
8. Do not select an LLVM version or Rust binding yet. Before adding one, evaluate LLVM/Rust compatibility, maintenance cadence, unsafe FFI surface, native build reproducibility, platform support, license, and security-update policy. Record the selected versions and binding in a follow-up ADR after a clean CI build and a verified IR-to-object prototype.

The exact GFA BASIC type inventory, source control-flow mapping, pointer semantics, and runtime-call ABI remain governed by the dialect, memory, and runtime decisions. This ADR defines the compiler boundary; it does not guess those semantics.

## Alternatives considered

### Lower source AST directly to LLVM IR

This would couple language semantics to LLVM types and APIs, make frontend tests depend on LLVM, and make another backend difficult to add. Rejected.

### Use LLVM IR as the canonical project IR

LLVM IR is target/code-generation oriented and does not express the project's source provenance, compatibility classifications, or profile-level runtime operations as a stable project-owned contract. Rejected.

### Create a separate IR for every target

This would duplicate semantic lowering and make it harder to compare behavior across targets. Use one target-independent project IR and explicit target/API lowering instead. Rejected.

### Select an LLVM binding during workspace bootstrap

No verified LLVM version, native package source, or clean-build strategy has been evaluated. Selecting a binding now would add an unvalidated native dependency to the workspace. Deferred.

## Consequences

- The frontend and IR can be developed and tested without LLVM or SDL3.
- Lossless token/trivia data remains available to formatting and editor tooling through the parsed-document result; the semantic AST is not required to carry formatting trivia itself.
- The backend consumes a verified semantic model rather than defining language behavior.
- The initial symbol index is explicitly prototype-scoped; it does not establish general Amiga GFA BASIC 3.5 declaration, case, scope, or type rules.
- Additional backends can be added without redesigning source parsing and semantic analysis.
- IR verification and source-provenance preservation become required compiler contracts.
- The LLVM binding and exact release remain open until dependency and CI evaluation is complete.
- Dialect-specific types, GOTO/GOSUB behavior, pointer semantics, and runtime ABI must be resolved by their respective evidence-backed decisions before implementation.

## Evidence and references

- [Compiler pipeline and IR specification](../PIPELINE-AND-IR.md).
- [Dialect/version policy](../DIALECTS.md), [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3).
- [Memory model](../MEMORY.md), [issue #8](https://github.com/vsimko/gfabasic-rust/issues/8).
- [Runtime architecture](../RUNTIME.md), [issue #7](https://github.com/vsimko/gfabasic-rust/issues/7).
- [Workspace boundaries](../WORKSPACE.md), [ADR 0002](0002-initial-rust-workspace.md).
- [LLVM backend implementation](https://github.com/vsimko/gfabasic-rust/issues/50).

## Supersedes / superseded by

None.

## Prototype implementation note (2026-11-05)

The first narrow implementation now exercises the intended boundary end to end for the evidence-backed Amiga 3.5 combinatorics subset: frontend AST → project-owned `ConstI64`/`Print` IR → verifier → temporary C emitter → host `cc` executable. The verifier enforces unique definitions and definition-before-use. This small prototype does not satisfy the complete general IR shape described above: it has no basic blocks, terminators, general expressions, calls, memory operations, or dialect-wide type system. Its `i64` representation and intrinsic bounds are implementation limits only. LLVM remains deferred pending the evaluation specified in this ADR.


## Amiga 3.5 addendum special lexemes (2026-11-05)

The lossless lexer recognizes the exact special names explicitly documented in the selected Amiga GFA BASIC 3.5 interpreter addendum: `_DATA`, `_DATA=`, and the special variables `_0` through `_9`. In the supplied 469-page `GFA_Basic_3.5_Interpreter.pdf`, these appear in Chapter 13 on printed pp. 13-34–13-35 (`_DATA`/`_DATA=` on p. 13-34; `_0`…`_9` on p. 13-35). The separately catalogued combined 3.0 + 3.5 update scan numbers the corresponding material as Chapter 14, printed pp. 14-5–14-6. The lexer represents `_DATA` and each numbered name as an identifier candidate, with the assignment character tokenized separately and all original source spelling retained by byte spans.

This is a deliberately closed lexical exception, not a general rule for underscore-prefixed identifiers and not a case-insensitivity decision. Other spellings such as `_foo`, `_10`, `_DATA2`, and lowercase `_data` remain lexical diagnostics until the Amiga 3.5 source evidence establishes broader rules. The parser and semantic model do not yet claim support for these special variables or DATA-pointer operations; this increment only makes their verified spellings available in the lossless token stream.

Evidence inventory: [Amiga GFA BASIC 3.5 addendum](../../research/amiga-gfa-basic-35-addendum-inventory.md); see also the [lexer evidence matrix](../../research/amiga-gfa-basic-35-lexer-evidence.md). The regression suite checks that each exact documented spelling is recognized at end-of-input and remains delimited from existing punctuation; adjacent unverified suffixes are rejected as a whole rather than accepted as a partial special-name prefix. These are boundary tests, not claims about the broader identifier grammar. This increment does not close issue #46; comments, strings/DATA payloads, general identifiers, encoding, line endings, and the remaining operator grammar are still unresolved.

## Formatter implementation note (2026-11-05)

The first formatter library increment is exposed independently as `gfabasic_compiler::formatter::format_source`. It validates the entire source with `parse_document` before returning output, then copies token spelling from original source spans while canonicalizing only whitespace and line boundaries for the currently supported FACT/VARIAT/COMBIN/PRINT subset. It preserves identifier and literal spelling exactly, uses LF and one final newline, and returns diagnostics for unsupported syntax. Comments, string/DATA payloads, broader identifiers, and richer grammar remain unsupported until lossless lexical and grammar evidence is available. The formatter has no file-writing, CLI, LLVM, runtime, or SDL dependency; CLI atomic-write behavior and LSP integration are follow-up increments under #208 and #209.
