# gfabasic-rust

A planned general-purpose GFA BASIC compiler and runtime implemented in Rust.

> **Project status:** Architecture and workspace bootstrap. The Rust workspace skeleton and baseline CI checks are present, but the compiler, runtime behavior, and CLI are not implemented. No GFA BASIC dialect or platform API is currently claimed as implemented or conformant.

## Project direction

The current planning baseline is to compile GFA BASIC source to native executables, with LLVM as the planned code-generation infrastructure. The first planned output target is Linux x86-64 (ELF, GNU ABI). SDL3 is planned as an implementation detail of the Linux platform integration layer.

The language core, platform API profiles, and output target are separate concepts. Amiga and Atari ST API profiles are planned as independently selectable profiles. Their behavior must be verified against the applicable GFA BASIC manuals before it is specified as supported.

Compilation must be deterministic and must not require an LLM. An LLM may assist external migration or analysis workflows, but it is not part of the compiler execution path.

## Documentation

Start with the [documentation index](docs/README.md). It separates user guidance, contributor guidance, architecture decisions, language/API research, and reusable page templates.

Key documents:

- [Architecture overview](docs/architecture/README.md)
- [Project goals and compatibility](docs/architecture/GOALS.md)
- [Dialect and version policy](docs/architecture/DIALECTS.md)
- [Documentation policy](docs/DOCUMENTATION.md)
- [GFA BASIC reference sources](docs/reference-sources.md)
- [Language reference notes](docs/reference/README.md)
- [Conformance test corpus](tests/corpus/README.md)
- [Open project work](https://github.com/vsimko/gfabasic-rust/issues)

## Current scope and limitations

The Rust workspace contains three architectural crates: compiler, generated-program runtime, and Linux platform adapter. These crates currently contain module placeholders only. The pinned Rust toolchain and fast workspace checks are defined in CI. There is still no compiler, runtime behavior, or CLI, and no GFA BASIC dialect or API profile is implemented. Planned targets, API profiles, and language features are not implementation or compatibility claims.

See the architecture documents for decisions, assumptions, and unresolved questions.

## License

The project source code and original project documentation are licensed under the [MIT License](LICENSE). Copyright remains with the respective contributors; the license grants the permissions stated in the license text.

This license does **not** cover third-party materials such as GFA BASIC manuals, books, ROMs, disk images, sample programs, trademarks, or other externally sourced assets. Their copyright, licensing, and redistribution status must be assessed separately; see [issue #18](https://github.com/vsimko/gfabasic-rust/issues/18) and the [reference source register](docs/reference-sources.md).

## Contributing

Start with the [contributor documentation](docs/contributor/README.md), then read the [architecture overview](docs/architecture/README.md) and check the open issues before proposing implementation work.
