# Fuzz targets

The fuzz workspace is intentionally independent of the root Cargo workspace.
Its dependencies are used only by fuzz builds and are not part of ordinary
workspace checks or release artifacts.

## Engine and toolchain

The source-loader target uses AFL++ through `afl` and `cargo-afl` 0.18.2.
Both packages declare Apache-2.0 in their published manifests, which is
included in the repository's approved dependency-license allowlist.

The supported CI environment is Linux x86-64 with the pinned stable Rust
1.99.1 toolchain and a C/C++ build toolchain. No nightly toolchain change is
required.

## Build and run locally

```sh
cargo +1.99.1 install cargo-afl --version 0.18.2 --locked
cargo +1.99.1 afl build --manifest-path fuzz/Cargo.toml --target-dir fuzz/target --release --bin source_loader --locked
AFL_NO_UI=1 AFL_SKIP_CPUFREQ=1 AFL_NO_AFFINITY=1 \
  cargo +1.99.1 afl fuzz \
    -i fuzz/corpus/source_loader \
    -o fuzz/out/source_loader \
    -V 30 -t 1000 -m 256 -G 1048576 \
    fuzz/target/release/source_loader
```

Use `-V 600` for a 10-minute campaign. The pull-request workflow uses
30 seconds; the weekly and manually dispatched workflow uses 10 minutes.
The separate `fuzz/Cargo.lock` is committed and audited by the security
workflow. The campaign artifact also includes that lockfile, preserving the
exact dependency resolution used for each run. Update it only when deliberately
changing fuzz dependencies, then review the new dependency licenses and
advisories.

## Target boundary

`source_loader` exercises strict UTF-8 byte decoding and source-map
invariants. It does not establish the source encoding of any GFA BASIC
dialect, parse source text, or test MERGE/include behavior. Its seed corpus is
synthetic transport data, not a language-conformance corpus.

Fuzzer output is generated data and must not be committed. CI packages the raw
output and best-effort minimized inputs into a tarball; AFL++ queue filenames
are sanitized in the archive for cross-platform artifact handling. For a
reproducible failure, preserve the original input, record the target/toolchain
and campaign settings, and add the smallest appropriate regression case to
the deterministic test suite.
