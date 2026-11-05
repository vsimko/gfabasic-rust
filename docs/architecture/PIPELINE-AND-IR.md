# Compiler pipeline and intermediate representation

- Status: **Accepted architecture boundary**; exact dialect semantics remain governed by [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3), and memory semantics by [issue #8](https://github.com/vsimko/gfabasic-rust/issues/8).
- Decision record: [ADR 0007](adr/0007-compiler-pipeline-and-ir.md).

## Pipeline contract

The compiler uses distinct stages. Each stage consumes an explicit input model, produces an explicit output model, and reports structured diagnostics with source provenance. A later stage must not reconstruct language semantics that an earlier stage failed to establish.

| Stage | Input | Output | Responsibility and boundary |
|---|---|---|---|
| Source loading | Input paths, selected source configuration | Source files, stable file IDs, source map | Read bytes, identify files, preserve original locations, and report I/O/encoding errors. Source composition is enabled only for verified dialect rules. |
| Lexing | Source files and explicit dialect/version | Lossless tokens and trivia with source spans | Retain original spelling, horizontal whitespace, and line-boundary spans for recognized forms. Recognize only verified lexical forms; do not normalize invalid input into valid tokens. |
| Parsing | Lossless tokens/trivia and dialect/version | Parsed document: lossless syntax plus AST with source spans | Build syntax structure while retaining the exact token/trivia stream for formatting and editor consumers. Report malformed or unsupported forms; do not resolve names or infer platform APIs. |
| Name resolution | Parsed document and declaration environment | Stable symbol identities, declarations, references, and source spans | Resolve names according to verified dialect rules. The prototype index is limited to first assignments and PRINT references; general scopes and declaration rules remain open. |
| Semantic analysis | Resolved AST, dialect/version, API profiles | Typed/validated AST or semantic model | Apply documented types, conversions, call signatures, control-flow rules, and profile resolution. LLVM is not involved. |
| IR lowering | Valid semantic model | Project-owned typed IR | Make implicit language decisions explicit. Preserve source provenance and reject unsupported semantics. |
| Target/API lowering | Typed IR, target, selected API profiles | Target-ready project IR | Lower only operations whose target/profile mapping is defined. Keep source dialect, API profile, and output target as separate inputs. |
| LLVM backend | Target-ready project IR | Verified LLVM module | Translate IR operations to LLVM IR. Verify the module before object emission. LLVM does not define BASIC semantics. |
| Object generation and linking | Verified module, explicit target, runtime link plan | Native object and executable | Emit target objects and link only the runtime components required by the selected target/profile. The exact linker and deployment policy remain under release/runtime work. |
| Artifact reporting | Diagnostics, target, link result | Build result and artifacts | Report diagnostics, exit status, output paths, target identity, and runtime requirements. |

The frontend must retain a lossless representation alongside semantic syntax. Significant tokens and trivia refer to the original UTF-8 byte ranges; consumers reconstruct exact source text from the source plus these spans rather than from normalized token values. The current FACT/VARIAT/COMBIN/PRINT lexer exposes horizontal spaces and tabs as trivia and line boundaries as tokens. This is an initial representation contract, not complete Amiga 3.5 lexical coverage: comment forms, string/DATA payloads, identifier character rules, encodings, and additional operators remain evidence-gated under [issue #46](https://github.com/vsimko/gfabasic-rust/issues/46). Unsupported characters continue to produce diagnostics. The lexer also recognizes the exact 3.5 addendum special names `_DATA` and `_0` through `_9` as identifier candidates, preserving their spelling and byte spans; this narrow exception does not establish a general underscore-identifier or case-folding rule. The evidence is Chapter 14, printed p. 14-6, inventoried in [the 3.5 addendum notes](../research/amiga-gfa-basic-35-addendum-inventory.md).

## Bootstrap prototype exception

The experimental combinatorics vertical slice under [issue #113](https://github.com/vsimko/gfabasic-rust/issues/113) parses the evidence-backed `FACT`, `VARIAT`, `COMBIN`, and `PRINT` subset, lowers the AST to the project-owned typed IR, verifies the IR, and only then emits temporary C for the host `cc` command. This is a bootstrap bridge, not the production LLVM backend. Its output spacing and UTF-8 transport are prototype choices, not verified historical interpreter behavior. CR, CRLF, and LF line separators in the experimental frontend are transport-level handling only until the original interpreter behavior is observed under #66. Do not generalize this bridge or infer support for other syntax.

## Stage and error rules

- Every source location uses a stable source-file identifier and byte-based source span. Human-readable line/column positions are derived from the source map.
- Source encoding, line endings, comments, token rules, and source composition are selected by the explicit dialect/version. Do not assume that documentation UTF-8 policy defines GFA BASIC source encoding.
- A stage returns structured diagnostics with stable codes and source spans. A parser may recover to report multiple syntax errors, but an invalid compilation must not proceed to code generation.
- Unknown, unsupported, or ambiguous constructs must produce diagnostics. They must not be silently skipped, rewritten, or assigned semantics from another BASIC dialect.
- Diagnostics and source provenance must survive AST transformations, semantic lowering, IR optimization, and backend lowering. A backend diagnostic must be traceable to the originating source span where one exists.
- The compiler must not emit an executable when required semantic or IR verification invariants fail.

## AST-to-IR boundary

The AST preserves source-level structure and dialect-specific distinctions. The semantic model resolves those distinctions using verified rules. The IR represents the result of those decisions; it is not a second parser and must not contain unresolved source-language ambiguity.

The initial project IR has these properties:

1. **Typed values:** every value has an explicit project-owned type. Integer width and signedness, floating-point format, string/array representation, and pointer/address types are explicit where the selected dialect semantics require them. The exact source type inventory remains tied to [dialect research #3](https://github.com/vsimko/gfabasic-rust/issues/3).
2. **Explicit conversions:** numeric conversions, truncation, overflow behavior, and Boolean normalization are represented as explicit operations after semantic analysis. The backend must not invent implicit conversions.
3. **Explicit control flow:** functions contain basic blocks with explicit terminators. Branches, returns, and value merges are represented in the IR graph; there is no reliance on source-order fall-through to define control flow.
4. **Typed calls:** calls identify a resolved function/runtime operation and carry arguments checked against a typed signature. Platform API calls remain distinct from language-core runtime calls.
5. **Explicit memory operations:** loads, stores, address calculations, and allocation operations are visible to verification and later lowering. Pointer width is not inferred from a BASIC integer type. Detailed ownership and raw-memory semantics remain governed by [issue #8](https://github.com/vsimko/gfabasic-rust/issues/8).
6. **Source provenance:** functions, blocks, instructions, and terminators retain source spans or an explicit synthetic-origin marker.
7. **Target independence:** generic IR types and operations do not expose Linux, SDL3, ELF, x86-64, Amiga, or Atari hardware types. Platform-specific behavior is represented by explicit, typed runtime/API operations.
8. **Verification:** an IR verifier checks type consistency, block/terminator structure, call signatures, and provenance invariants before backend code generation.

### Illustrative IR shape

The following is abstract IR notation, not GFA BASIC source syntax and not a promise that a particular dialect has these exact source constructs:

```text
function example() -> i32 {
entry:
    %left:i32  = const.i32 1
    %right:i32 = const.i32 2
    %sum:i32   = add.i32 %left, %right
    return %sum
}
```

A conditional is represented with explicit blocks and a terminator:

```text
entry:
    branch %condition, then_block, else_block

then_block:
    jump merge_block(%then_value)

else_block:
    jump merge_block(%else_value)

merge_block(%result:i32):
    return %result
```

These examples show typed values and explicit control flow only. The mapping of GFA BASIC labels, GOTO, GOSUB, RETURN, procedures, loops, and error handling must follow the selected dialect's verified semantics. Do not infer those mappings from the examples.

## LLVM backend boundary

LLVM is a code-generation backend, not the language's type checker, semantic analyzer, runtime, or API registry.

- Keep LLVM IR and LLVM-specific types inside the backend crate/module.
- The frontend and project IR must build and run their unit tests without LLVM or SDL3 installed.
- Convert project IR to LLVM IR only after semantic and project-IR verification succeeds.
- Run LLVM module verification before object emission and report verifier failures as compiler diagnostics.
- Select an explicit target triple and target CPU/features. Do not use `-march=native` for reproducible output.
- Preserve source locations when emitting debug metadata. The initial debug policy is to make debug information available for development builds and to keep optimized release builds reproducible.
- Optimization passes must preserve the project IR's defined semantics. Target-specific optimization begins only after target selection.
- Object generation belongs to the backend. Runtime selection and final linking belong to the build/link stage; generated code must not call SDL3 directly.

### LLVM version and Rust binding policy

No LLVM version or Rust binding is selected by this ADR. This is an explicit deferral, not an implicit dependency choice.

Evaluate the following before adding a binding:

- exact LLVM release compatibility with the pinned Rust toolchain and the selected Linux CI image;
- maintenance cadence and support for the LLVM release the project intends to pin;
- whether the binding is a high-level wrapper (for example, Inkwell) or a lower-level LLVM C API binding (for example, llvm-sys), including the amount of unsafe FFI and version-specific code the project must own;
- native development-package availability, clean builds, linking mode, and cross-compilation requirements;
- license, security-update process, and ability to reproduce the toolchain in CI.

A high-level wrapper can reduce direct FFI and improve ergonomics, but may lag LLVM releases or constrain supported LLVM versions. A lower-level binding can expose more of the C API but transfers more version-specific and unsafe integration work to this project. Neither is selected until a clean CI build and a small verified IR-to-object prototype demonstrate the maintenance and deployment costs.

Keep the LLVM dependency optional and confined to the backend package. Do not add an unpinned LLVM dependency to the workspace root. Record the selected LLVM release, binding, native package source, and upgrade policy in a later ADR before implementing the backend under [issue #50](https://github.com/vsimko/gfabasic-rust/issues/50).

## Extension points

The project-owned IR is the stable conceptual boundary for future code-generation backends. A new backend may consume verified IR without changing source loading, parsing, name resolution, or language semantics. Target-specific lowering may be extended for another operating system or CPU, but must not leak target assumptions back into the frontend.

## Related work

- [Dialect/version policy](DIALECTS.md), [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3).
- [Memory model](MEMORY.md), [issue #8](https://github.com/vsimko/gfabasic-rust/issues/8).
- [Diagnostics](DIAGNOSTICS.md), [issue #26](https://github.com/vsimko/gfabasic-rust/issues/26).
- [Workspace and dependency policy](WORKSPACE.md), [issue #11](https://github.com/vsimko/gfabasic-rust/issues/11).
- [LLVM backend implementation](https://github.com/vsimko/gfabasic-rust/issues/50).

## Current prototype IR implementation

The compiler crate now contains an initial project-owned IR for the evidence-gated Amiga 3.5 combinatorics vertical slice. Its deliberately limited instruction set is `ConstI64` and ordered `Print`; values have stable per-module IDs and an explicit `I64` type. AST lowering resolves the current literal-only `FACT`, `VARIAT`, and `COMBIN` subset, binds only variables already assigned, rejects reassignment and out-of-range intrinsic arguments, and preserves source spans on emitted instructions. The verifier checks unique value definitions and rejects uses that do not follow a definition.

The verified IR is now the active input to the temporary C bootstrap: the compiler follows AST → typed IR → IR verification → C emission → `cc` executable. The next backend step is to replace the C emitter with LLVM under issue #50. The current IR has no general expressions, runtime calls, control flow, memory model, or dialect-wide type system. Its `i64` type is a prototype implementation type only and must not be generalized to GFA BASIC variables without dialect evidence.
