# gfabasic-rust

A GFA BASIC compiler and runtime implemented in Rust, targeting Amiga GFA BASIC interpreter 3.5 and native Linux x86-64 executables.

**[Project website](https://vsimko.github.io/gfabasic-rust/)** · [Documentation](docs/README.md) · [Roadmap](ROADMAP.md)

> **Project status:** The workspace contains an experimental, evidence-gated Amiga GFA BASIC 3.5 frontend for literal `FACT`, `VARIAT`, and `COMBIN` assignments followed by `PRINT`. The current prototype bounds intrinsic arguments to `0 <= k <= n <= 20` (or `FACT(n)` with `0 <= n <= 20`) and restricts variables to `x`, `y`, and `z`. Compilation now follows AST → typed project IR → IR verification → temporary C/`cc` bootstrap. This is not general GFA BASIC support and does not claim conformance with the original interpreter. General language semantics and the planned LLVM backend remain unimplemented.

## Project direction

The sole supported source dialect is Commodore Amiga GFA BASIC interpreter 3.5. The sole planned output target is Linux x86-64 (ELF, GNU ABI), with LLVM as the native code-generation infrastructure. SDL3 is the Linux platform integration layer for graphics, windows, and input. Atari ST, other GFA BASIC versions, other output targets, and cross-compilation are out of scope. See [ADR 0011](docs/architecture/adr/0011-amiga-35-linux-sdl3-product-scope.md).

Compilation must be deterministic and must not require an LLM. An LLM may assist external migration or analysis workflows, but it is not part of the compiler execution path.

## Experimental prototype

Build the current narrow fixture subset with:

```sh
cargo run -p gfabasic-compiler -- build tests/corpus/amiga/3.5/combinatorics/fact-zero.lst -o ./fact
./fact
```

The prototype prints the numeric values in order. Its current C bridge separates them with spaces, but that formatting is not asserted as historical interpreter behavior.

The prototype rejects constructs outside its documented subset. It currently uses textual `.LST` input through the current strict UTF-8 transport loader and the host `cc` executable; neither establishes the historical source encoding nor selects the production compiler backend.

## Documentation

Start with the [documentation index](docs/README.md). The [project roadmap](ROADMAP.md) describes delivery phases, dependencies, and current maturity. The documentation separates user guidance, contributor guidance, architecture decisions, language/API research, and reusable page templates.

Key documents:

- [Project roadmap](ROADMAP.md)
- [Architecture overview](docs/architecture/README.md)
- [Project goals and compatibility](docs/architecture/GOALS.md)
- [Product scope decision (ADR 0011)](docs/architecture/adr/0011-amiga-35-linux-sdl3-product-scope.md)
- [Dialect and version policy](docs/architecture/DIALECTS.md)
- [Documentation policy](docs/DOCUMENTATION.md)
- [GFA BASIC reference sources](docs/reference-sources.md)
- [Language reference notes](docs/reference/README.md)
- [Conformance test corpus](tests/corpus/README.md)
- [Frontend fuzzing strategy](docs/architecture/FUZZING.md)
- [Open project work](https://github.com/vsimko/gfabasic-rust/issues)

## Current scope and limitations

The Rust workspace contains three architectural crates: compiler, generated-program runtime, and Linux platform adapter. The compiler crate includes source-map and strict UTF-8 file-loading primitives, structured diagnostics, the evidence-gated FACT/PRINT frontend subset, and a temporary native-executable bootstrap path. The general lexer/parser grammar, complete semantic model and general-purpose IR, LLVM backend, project configuration, core runtime, and Linux platform adapter remain unimplemented. The pinned Rust toolchain and fast workspace checks are defined in CI. No complete Amiga GFA BASIC 3.5 dialect or Amiga API profile is implemented or conformant. The only product target is Amiga GFA BASIC 3.5 to Linux x86-64; this scope decision is not an implementation or compatibility claim.

See the roadmap and architecture documents for decisions, assumptions, dependencies, and unresolved questions.

## License

The project source code and original project documentation are licensed under the [MIT License](LICENSE). Copyright remains with the respective contributors; the license grants the permissions stated in the license text.

The project does not distribute historical GFA BASIC manuals or books, ROMs, disk images, or historical sample programs. They may be consulted as external research sources where lawfully accessible, but they must not be copied into this repository or bundled with project releases. Research outputs must use bibliographic metadata, concise citations, original explanations, and independently authored test fixtures. The MIT license applies only to project-owned code and documentation; it does not grant rights to third-party materials.

## Contributing

Start with the [contributor documentation](docs/contributor/README.md), then read the [architecture overview](docs/architecture/README.md) and check the [roadmap](ROADMAP.md) and open issues before proposing implementation work.
