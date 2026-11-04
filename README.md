# gfabasic-rust

A planned general-purpose GFA BASIC compiler and runtime implemented in Rust.

> **Project status:** Architecture and workspace bootstrap, with a minimal CLI executable. `gfabasic --help` and `gfabasic --version` are implemented; source compilation, project configuration, and runtime behavior are not. No GFA BASIC dialect or platform API is currently claimed as implemented or conformant.

## Project direction

The current planning baseline is to compile GFA BASIC source to native executables, with LLVM as the planned code-generation infrastructure. The first planned output target is Linux x86-64 (ELF, GNU ABI). SDL3 is planned as an implementation detail of the Linux platform integration layer.

The language core, platform API profiles, and output target are separate concepts. Amiga and Atari ST API profiles are planned as independently selectable profiles. Their behavior must be verified against the applicable GFA BASIC manuals before it is specified as supported.

Compilation must be deterministic and must not require an LLM. An LLM may assist external migration or analysis workflows, but it is not part of the compiler execution path.

## Documentation

Start with the [documentation index](docs/README.md). The [project roadmap](ROADMAP.md) describes delivery phases, dependencies, and current maturity. The documentation separates user guidance, contributor guidance, architecture decisions, language/API research, and reusable page templates.

Key documents:

- [Project roadmap](ROADMAP.md)
- [Architecture overview](docs/architecture/README.md)
- [Project goals and compatibility](docs/architecture/GOALS.md)
- [Dialect and version policy](docs/architecture/DIALECTS.md)
- [Documentation policy](docs/DOCUMENTATION.md)
- [GFA BASIC reference sources](docs/reference-sources.md)
- [Language reference notes](docs/reference/README.md)
- [Conformance test corpus](tests/corpus/README.md)
- [Frontend fuzzing strategy](docs/architecture/FUZZING.md)
- [Open project work](https://github.com/vsimko/gfabasic-rust/issues)

## Current scope and limitations

The Rust workspace contains three architectural crates: compiler, generated-program runtime, and Linux platform adapter. The compiler crate includes tested source-map and strict UTF-8 file-loading primitives plus a minimal `gfabasic` CLI. The CLI currently supports help and version output only; `build` and `check` explicitly report that compilation is unavailable. The lexer, parser, semantic analysis, IR, backend, project configuration, runtime behavior, and Linux platform adapter remain unimplemented. The pinned Rust toolchain and fast workspace checks are defined in CI. No GFA BASIC dialect or API profile is implemented. Planned targets, API profiles, and language features are not implementation or compatibility claims.

See the roadmap and architecture documents for decisions, assumptions, dependencies, and unresolved questions.

## License

The project source code and original project documentation are licensed under the [MIT License](LICENSE). Copyright remains with the respective contributors; the license grants the permissions stated in the license text.

The project does not distribute historical GFA BASIC manuals or books, ROMs, disk images, or historical sample programs. They may be consulted as external research sources where lawfully accessible, but they must not be copied into this repository or bundled with project releases. Research outputs must use bibliographic metadata, concise citations, original explanations, and independently authored test fixtures. The MIT license applies only to project-owned code and documentation; it does not grant rights to third-party materials.

## Contributing

Start with the [contributor documentation](docs/contributor/README.md), then read the [architecture overview](docs/architecture/README.md) and check the [roadmap](ROADMAP.md) and open issues before proposing implementation work.
