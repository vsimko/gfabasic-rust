# ADR 0012: Shared language tooling through Language Server Protocol

- Status: Accepted
- Date: 2026-11-05
- Decision makers: Project maintainer
- Tracking: [LSP server #209](https://github.com/vsimko/gfabasic-rust/issues/209), [VS Code integration #210](https://github.com/vsimko/gfabasic-rust/issues/210), [IntelliJ integration #211](https://github.com/vsimko/gfabasic-rust/issues/211)

## Context

The project is intended to provide more than a command-line compiler. Users should be able to inspect, format, refactor, validate, and build Amiga GFA BASIC 3.5 programs from common editors, initially Visual Studio Code and IntelliJ IDEA.

Implementing editor features independently in each editor would duplicate parsing, dialect rules, symbol resolution, diagnostics, and formatting. A text-only implementation would also make rename and navigation unsafe, especially for legacy programs whose identifiers may be Slovak, English, or mixed-language.

The compiler frontend is not yet general: it currently supports only a narrow evidence-backed prototype. Editor integration must therefore be designed around the future shared frontend and must not imply that unsupported language constructs are understood.

## Decision

1. Use the **Language Server Protocol (LSP)** as the common editor integration boundary for diagnostics, completion, navigation, references, rename, and formatting.
2. Implement one Rust language server, logically isolated from editor clients and backed by the same compiler frontend, semantic model, symbol index, diagnostics, and formatter used by the CLI.
3. Treat the server as an adapter, not a second language implementation. It must not contain an independent lexer, parser, type system, or compiler.
4. Preserve a lossless source representation (CST or equivalent token/trivia stream) alongside the semantic AST. Formatting and editor edits must retain comments, literal payloads, source spans, and original identifier spelling.
5. Implement rename using resolved symbol identity and semantic references, never global text replacement. Preserve identifier spelling unless the user explicitly requests a rename.
6. Keep compiler source spans byte-based and convert positions at the protocol boundary. LSP positions use UTF-16 code units; conversions must be tested for non-ASCII source text and line endings.
7. Start with stdio transport and full-document synchronization. Incremental synchronization may be advertised only after versioned edits and reparsing are reliable.
8. The server may expose build as an LSP execute-command or equivalent editor action, but compilation must delegate to the shared compiler/build pipeline. The server must not duplicate code generation or runtime behavior.
9. Keep editor-specific packaging and UI outside the server. VS Code and IntelliJ clients are thin integrations that expose only capabilities actually advertised by the server.
10. Keep LLVM, SDL3, generated-program runtime, and platform API implementations out of the language server dependency boundary.
11. Defer debugging and Debug Adapter Protocol (DAP) integration until a separate decision defines debugger semantics and runtime instrumentation.
12. The normative language target remains Amiga GFA BASIC interpreter 3.5. No LSP capability may accept or claim semantics for unverified syntax or another dialect/version.

## Alternatives considered

- **Separate parser and language logic in each editor:** rejected because it duplicates semantics and causes inconsistent diagnostics, formatting, and refactoring.
- **TextMate/regex-only editor support:** useful for basic syntax highlighting, but insufficient for semantic diagnostics, symbol navigation, or safe rename. It may be used as a lightweight highlighting layer only.
- **Compiler CLI invoked for every editor feature:** rejected as the primary design because document synchronization, interactive navigation, completion, and versioned workspace edits require a long-lived protocol service. The CLI remains the build and batch interface.
- **Native IntelliJ implementation as the primary language engine:** rejected because it would create a second frontend. A thin LSP client integration is preferred; native IDE APIs may be added only as adapters where LSP is insufficient.
- **Implement DAP at the same time:** deferred because debugging needs a working execution/runtime model and a separate protocol contract.

## Consequences

- The frontend must expose lossless syntax, stable source maps, semantic symbol identities, and reusable diagnostics before the corresponding LSP capabilities are enabled.
- The formatter remains a library independent of the CLI and LSP process; both interfaces call the same implementation.
- Rename, definition, references, and completion depend on the symbol-resolution work, not on syntax highlighting.
- The language server can be tested with protocol-level integration tests without launching either editor.
- VS Code and IntelliJ release cycles can proceed independently of the compiler, provided their LSP contract remains stable.
- LSP support is a planned capability, not an implementation claim. Capability negotiation must accurately reflect implemented and tested behavior.
- The first useful editor milestone is diagnostics plus formatting for the subset supported by the shared frontend; richer refactoring follows semantic coverage.
- DAP, debugger UI, and runtime instrumentation remain separate future work.

## Evidence and references

- [LSP server implementation #209](https://github.com/vsimko/gfabasic-rust/issues/209)
- [VS Code integration #210](https://github.com/vsimko/gfabasic-rust/issues/210)
- [IntelliJ integration #211](https://github.com/vsimko/gfabasic-rust/issues/211)
- [Lossless lexer requirements #46](https://github.com/vsimko/gfabasic-rust/issues/46)
- [Lossless parser/CST requirements #47](https://github.com/vsimko/gfabasic-rust/issues/47)
- [Semantic analysis and symbol resolution #48](https://github.com/vsimko/gfabasic-rust/issues/48)
- [Formatter #208](https://github.com/vsimko/gfabasic-rust/issues/208)
- [Compiler pipeline and IR](../PIPELINE-AND-IR.md)
- [Selected source dialect ADR 0009](0009-initial-source-dialect-amiga-gfa-basic-3.5.md)

## Supersedes / superseded by

None.
