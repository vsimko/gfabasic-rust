# gfabasic-rust

A planned general-purpose GFA BASIC compiler and runtime implemented in Rust.

> **Project status:** Planning and architecture definition. The repository does not yet contain a compiler implementation. No GFA BASIC dialect or platform API is currently claimed as implemented or conformant.

## Project direction

The current planning baseline is to compile GFA BASIC source to native executables, with LLVM as the planned code-generation infrastructure. The first planned output target is Linux x86-64 (ELF, GNU ABI). SDL3 is planned as an implementation detail of the Linux platform integration layer.

The language core, platform API profiles, and output target are separate concepts. Amiga and Atari ST API profiles are planned as independently selectable profiles. Their behavior must be verified against the applicable GFA BASIC manuals before it is specified as supported.

Compilation must be deterministic and must not require an LLM. An LLM may assist external migration or analysis workflows, but it is not part of the compiler execution path.

## Documentation

The documentation is maintained in this repository.

- [Architecture](docs/architecture/README.md)
- [Project goals and compatibility](docs/architecture/GOALS.md)
- [Dialect and version policy](docs/architecture/DIALECTS.md)
- [Documentation policy](docs/DOCUMENTATION.md)
- [GFA BASIC reference sources](docs/reference-sources.md)
- [Open project work](https://github.com/vsimko/gfabasic-rust/issues)

## Current scope and limitations

This repository is at the planning stage. There is no Rust workspace, compiler, runtime, CLI, or CI workflow yet. Planned targets, API profiles, and language features are not implementation or compatibility claims.

See the architecture documents for decisions, assumptions, and unresolved questions.

## Contributing

See [the architecture overview](docs/architecture/README.md) and the open issues before proposing implementation work. New language claims must cite an applicable GFA BASIC reference source and identify its edition/version and platform.
