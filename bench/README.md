# Source-loading benchmarks

This is an independent Cargo workspace. Criterion and its transitive
dependencies are benchmark-only: they do not enter the normal workspace,
compiler builds, or release SBOM.

## Workload

The current microbenchmark measures the strict UTF-8 byte decoder and
source-map construction in `SourceMap::add_utf8_bytes`.

- One synthetic source at 4 KiB, 64 KiB, and 1 MiB.
- 128 synthetic 1 KiB sources inserted into one source map.
- Input buffers and filenames are prepared outside the timed routine.
- Timed work includes source-map creation, UTF-8 validation, line indexing,
  and source insertion.
- No filesystem I/O, GFA BASIC syntax, dialect encoding, or MERGE/include
  behavior is measured.

The data is synthetic transport text, not a GFA BASIC conformance fixture.
It is deterministic and contains only valid ASCII bytes and CRLF line
separators.

## Run locally

```sh
cargo +1.99.1 bench --manifest-path bench/Cargo.toml --bench source_loading --locked -- --noplot
```

Criterion uses 20 samples, a one-second warm-up, and two seconds of
measurement per benchmark. Reports are written under `bench/target/criterion`.

The weekly/manual GitHub Actions workflow records commit, Rust toolchain,
kernel, architecture, CPU model, logical CPU count, memory, target, and
workload revision. It uploads Criterion reports and metadata as a 30-day
artifact. It does not run timed benchmarks on pull requests and does not
enforce a performance threshold.

GitHub-hosted runners are not a controlled benchmark machine. Compare
artifacts only when host and toolchain metadata match. Treat these results as
observational until a stable, controlled runner is selected.
