# gfabasic-rust

A planned general-purpose GFA BASIC compiler and runtime implemented in Rust.

> **Project status:** Planning and architecture definition. The repository does not yet contain a compiler implementation. No GFA BASIC dialect or platform API is currently claimed as implemented or conformant.

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

This repository is at the planning stage. There is no Rust workspace, compiler, runtime, or CLI. A documentation-only CI workflow validates Markdown style and local links; compiler/runtime build and test workflows are not yet available. Planned targets, API profiles, and language features are not implementation or compatibility claims.

See the architecture documents for decisions, assumptions, and unresolved questions.

## Contributing

Start with the [contributor documentation](docs/contributor/README.md), then read the [architecture overview](docs/architecture/README.md) and check the open issues before proposing implementation work. New language claims must cite an applicable GFA BASIC reference source and identify its edition/version and platform.
