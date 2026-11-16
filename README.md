# gfabasic-rust

A GFA BASIC compiler and runtime in Rust. The initial source dialect is Commodore Amiga GFA BASIC interpreter 3.5; the planned native target is Linux x86-64.

**[Project website](https://vsimko.github.io/gfabasic-rust/)** · [Documentation](docs/index.md) · [Roadmap](ROADMAP.md) · [Open issues](https://github.com/vsimko/gfabasic-rust/issues)

## Current status

The repository contains an experimental, evidence-gated frontend prototype for a narrow `FACT`, `VARIAT`, `COMBIN`, and `PRINT` subset. It is not a general GFA BASIC compiler and does not claim compatibility with the original interpreter. See the [documentation index](docs/index.md) for architecture, implementation status, language research, testing, and contributor guidance.

## Current scope and limitations

The Rust workspace contains compiler, runtime, and Linux platform crates. The compiler has a narrow experimental frontend and temporary native bootstrap; the general language grammar, complete semantic model, LLVM backend, runtime, and platform adapter remain incomplete. The current prototype does not establish general GFA BASIC support or original-interpreter conformance.

## Experimental prototype

```sh
cargo run -p gfabasic-compiler -- build tests/corpus/amiga/3.5/combinatorics/fact-zero.lst -o ./fact
./fact
```

The prototype uses a temporary C/`cc` bootstrap. General language semantics, the LLVM backend, and the full runtime remain unimplemented.

## Contributing and licensing

Start with [contributor documentation](docs/contributor/index.md) and follow the [documentation policy](docs/DOCUMENTATION.md). The project source and original documentation are MIT-licensed. Historical manuals, ROMs, disk images, and sample programs are not distributed by this repository; consult the documentation policy for source and copyright rules.
