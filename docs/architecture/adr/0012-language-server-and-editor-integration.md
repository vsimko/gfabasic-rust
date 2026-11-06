# ADR 0012: Language server and editor integration

- Status: Proposed
- Date: 2026-11-06
- Decision makers: Project maintainers

## Context

The project needs editor tooling for the selected source dialect, Commodore Amiga GFA BASIC interpreter 3.5. Diagnostics, formatting, navigation, completion, and rename must use the same dialect-aware frontend and semantic model as compilation. A second lexer/parser or text-based rename implementation would drift from compiler behavior and could corrupt source spelling.

The compiler currently supports only a narrow evidence-gated syntax subset. Most general identifier, comment, string, DATA, and statement rules remain unresolved. An editor server must not turn unknown syntax into accepted language behavior or advertise features that cannot be implemented correctly.

## Decision

1. Implement the Language Server Protocol adapter in a separate `gfabasic-lsp` workspace crate. The crate boundary is justified by its editor-protocol and JSON-RPC dependencies; those dependencies must not enter the compiler, runtime, or platform crates.
2. Use standard JSON-RPC over stdio for editor-launched sessions. Keep protocol handling independent of VS Code, IntelliJ, and other editor-specific APIs.
3. Reuse compiler-owned source loading, lossless syntax, parser, symbol index, semantic analysis, diagnostics, and formatter. The LSP crate must not implement a second lexer, parser, formatter, or language-semantic model.
4. Start with full-document synchronization. Incremental synchronization may be advertised only after document updates can be applied safely and tested against the compiler's source model.
5. Track each open document's URI, text, and monotonically increasing client version. Diagnostics and workspace edits must be associated with the version from which they were computed; never apply an edit to a newer document version.
6. Use UTF-16 code units for LSP line/character positions at the protocol boundary. Convert to and from the compiler's UTF-8 byte spans with tested handling of supplementary-plane characters, CRLF, line boundaries, and invalid/out-of-range positions.
7. Implement capabilities in dependency order:
   - initialize/shutdown/exit and document open/change/close;
   - diagnostics from the shared frontend;
   - document and range formatting through the formatter library;
   - definition and references through resolved symbol identity;
   - completion from visible symbols and verified built-ins;
   - rename through resolved references and versioned workspace edits;
   - code actions and build requests only after the relevant compiler/CLI contracts exist.
8. Advertise only implemented and tested capabilities. Return explicit unsupported or unknown results when compiler evidence or semantic information is insufficient.
9. Never implement rename as textual replacement. Edit only references proven to resolve to the selected symbol; preserve the exact original identifier bytes and do not transliterate, case-fold, or Unicode-normalize names.
10. Keep the server independent of LLVM, SDL3, target runtime implementations, and platform API behavior. Build requests delegate to the existing compiler/CLI pipeline rather than duplicating compilation.

## Alternatives considered

### Implement LSP inside the compiler crate

Rejected because protocol dependencies and editor-facing lifecycle code would become compiler dependencies, weakening the compiler's reuse and build boundary.

### Implement a separate frontend in the LSP server

Rejected because syntax acceptance, diagnostics, formatting, and symbol binding would diverge from the compiler and could silently apply different dialect assumptions.

### Use incremental synchronization immediately

Deferred because the current frontend does not yet define a safe incremental reparse/update contract. Full-document synchronization is simpler and keeps source versions coherent.

### Implement rename with text search

Rejected because identical spellings can refer to different declarations, and unresolved or ambiguous references cannot be safely rewritten.

## Consequences

- The LSP server can be built and tested without LLVM, SDL3, or editor-specific SDKs.
- The compiler remains the single source of language semantics and source spans.
- UTF-8/UTF-16 conversion and document-version checks become protocol correctness requirements.
- Early server milestones can provide lifecycle and diagnostics for the existing supported subset without claiming general Amiga 3.5 support.
- Formatting, navigation, completion, and rename remain unavailable until their compiler prerequisites exist and are tested.
- The exact LSP/JSON-RPC Rust dependency and version are not selected by this ADR; dependency review and lockfile/CI validation are required before implementation.

## Initial implementation status (2026-11-06)

The `gfabasic-lsp` crate provides JSON-RPC framing over stdio, initialize/shutdown/exit, full-document open/change/close notifications, per-document version checks, and tested UTF-16/UTF-8 position conversion. It advertises only full-document synchronization; ranged changes are rejected rather than partially applied. Push diagnostics are now produced by the compiler's shared `parse_document` frontend and are tagged with the document version. Closing a document clears its diagnostics. Formatting, navigation, completion, and rename are not implemented or advertised. The protocol JSON parser is intentionally internal to this bootstrap and must receive adversarial/fuzz coverage before the server is treated as production-ready. This implementation note does not change the proposed status of the ADR.

## Evidence and references

- [Issue #209: GFA BASIC language server](https://github.com/vsimko/gfabasic-rust/issues/209).
- [Issue #208: Amiga GFA BASIC 3.5 formatter](https://github.com/vsimko/gfabasic-rust/issues/208).
- [ADR 0007: compiler pipeline and typed IR boundaries](0007-compiler-pipeline-and-ir.md).
- [Rust workspace and dependency policy](../WORKSPACE.md).
