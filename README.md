# gfabasic-rust

A GFA BASIC compiler and runtime implemented in Rust, targeting programs accepted by the Commodore Amiga GFA BASIC 3.52 interpreter and native Linux x86-64 executables.

**[Project website](https://vsimko.github.io/gfabasic-rust/)** · [Documentation](docs/README.md) · [Roadmap](ROADMAP.md) · [Open issues](https://github.com/vsimko/gfabasic-rust/issues)

> **Project status:** The workspace contains an experimental, evidence-gated frontend for a narrow set of GFA BASIC constructs. The current prototype supports selected `FACT`, `VARIAT`, and `COMBIN` assignments followed by `PRINT`, with documented argument and variable limits. Compilation follows AST → typed project IR → IR verification → temporary C/`cc` bootstrap. This is not general GFA BASIC support and does not claim full compatibility with the reference interpreter. General language semantics and the planned LLVM backend remain unimplemented. See the [roadmap](ROADMAP.md) for the current implementation state.

## Project direction

The practical compatibility baseline is the **Commodore Amiga GFA BASIC 3.52 interpreter**. Our goal is to compile the largest practical set of programs and features that this interpreter can interpret. The existing Amiberry workflow is used for repeatable reference tests.

Books and manuals are supporting research sources. They help discover features and investigate failures, but they may describe another release or contain errors. Observed interpreter behavior takes priority when sources disagree. Binary disassembly/decompilation is a separate diagnostic source and must retain its provenance.

A comparison with an independently identified GFA BASIC 3.5 interpreter is a later validation phase. It must not block current compiler development. Do not treat a 3.52 test result as proof of 3.5 behavior.

The sole planned output target remains Linux x86-64 (ELF, GNU ABI), with LLVM as the intended native code-generation infrastructure. SDL3 is the Linux platform integration layer for graphics, windows, and input. See [ADR 0011](docs/architecture/adr/0011-amiga-35-linux-sdl3-product-scope.md) and the [compatibility evidence policy](docs/architecture/COMPATIBILITY-EVIDENCE.md).

Compilation must be deterministic and must not require an LLM. An LLM may assist repository research and development workflows, but it is not part of the compiler execution path.

## Experimental prototype

Build the current narrow fixture subset with:

```sh
cargo run -p gfabasic-compiler -- build tests/corpus/amiga/3.5/combinatorics/fact-zero.lst -o ./fact
./fact
```

The prototype prints the numeric values in order. Its current C bridge separates them with spaces, but that formatting is not asserted as historical interpreter behavior.

The prototype rejects constructs outside its documented subset. It currently uses textual `.LST` input through the current strict UTF-8 transport loader and the host `cc` executable; neither establishes the historical source encoding nor selects the production compiler backend.

## Documentation and agent entry point

When entering the repository without prior context:

1. Read this README.
2. Read [AGENTS.md](AGENTS.md) for mandatory agent and issue-tracking rules.
3. Read the [documentation index](docs/README.md) and [roadmap](ROADMAP.md).
4. Read [compatibility evidence and agent workflow](docs/architecture/COMPATIBILITY-EVIDENCE.md).
5. Inspect open GitHub issues and pull requests. Select work from an issue and follow its linked canonical specification.

Key documents:

- [Project roadmap](ROADMAP.md)
- [Compatibility evidence and agent workflow](docs/architecture/COMPATIBILITY-EVIDENCE.md)
- [Architecture overview](docs/architecture/README.md)
- [Project goals and compatibility](docs/architecture/GOALS.md)
- [Dialect and version policy](docs/architecture/DIALECTS.md)
- [Documentation policy](docs/DOCUMENTATION.md)
- [GFA BASIC reference sources](docs/reference-sources.md)
- [Language reference notes](docs/reference/README.md)
- [Research procedures and historical evidence](docs/research/README.md)
- [Conformance test corpus](tests/corpus/README.md)
- [Frontend fuzzing strategy](docs/architecture/FUZZING.md)
- [Open project work](https://github.com/vsimko/gfabasic-rust/issues)

## Current scope and limitations

The Rust workspace contains compiler, generated-program runtime, Linux platform, and language-server crates. The compiler includes source-map and strict UTF-8 file-loading primitives, structured diagnostics, a narrow evidence-gated frontend subset, and a temporary native-executable bootstrap path. The general lexer/parser grammar, complete semantic model and general-purpose IR, LLVM backend, project configuration, core runtime, and Linux platform adapter remain unimplemented. The pinned Rust toolchain and fast workspace checks are defined in CI.

The 3.52 compatibility baseline is a development oracle, not a claim that the compiler already implements the interpreter. Every feature must have separate evidence, implementation, and test status. See the roadmap and architecture documents for current maturity and unresolved questions.

## License

The project source code and original project documentation are licensed under the [MIT License](LICENSE). Copyright remains with the respective contributors; the license grants the permissions stated in the license text.

The project does not distribute historical GFA BASIC manuals or books, ROMs, disk images, or historical sample programs. They may be consulted as external research sources where lawfully accessible, but they must not be copied into this repository or bundled with project releases. Research outputs must use bibliographic metadata, concise citations, original explanations, and independently authored test fixtures. The MIT license applies only to project-owned code and documentation; it does not grant rights to third-party materials.

## Contributing

Start with the [contributor documentation](docs/contributor/README.md), then read the [architecture overview](docs/architecture/README.md) and check the [roadmap](ROADMAP.md) and open issues before proposing implementation work.
