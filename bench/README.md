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
cargo +1.99.1 bench --manifest-path bench/Cargo.toml --target-dir bench/target --bench source_loading --locked -- --noplot
```

Criterion 0.8.2 is dual-licensed Apache-2.0 OR MIT, within the approved
allowlist. The committed `bench/Cargo.lock` pins the benchmark dependency
graph and is checked by the RustSec workflow. Criterion uses 20 samples, a one-second
warm-up, and two seconds of measurement per benchmark. Reports are written
under `bench/target/criterion`.

The main-branch/weekly/manual GitHub Actions workflow records commit, Rust
toolchain, kernel, architecture, CPU model, logical CPU count, memory, target,
workload revision, benchmark lockfile SHA-256, and whether the Criterion
baseline cache was hit. It caches
Criterion's `base` data only for matching OS/architecture, CPU model, kernel,
toolchain, lockfile, and workload version; other hosts start a fresh baseline.
It uploads Criterion reports and metadata as a 30-day artifact. It does not run
timed benchmarks on pull requests and does not enforce a performance threshold.

The first pre-cache sample was recorded at main commit `b61f7d3`: 2.864 µs
for 4 KiB, 43.816 µs for 64 KiB, 986.582 µs for 1 MiB, and 138.733 µs for
128 × 1 KiB sources (median estimates). It is observational only; the first
cache-enabled main run seeds the repeatable matching-host-profile comparison baseline.

GitHub-hosted runners are not a controlled benchmark machine. Compare
artifacts only when host and toolchain metadata match. Treat these results as
observational until a stable, controlled runner is selected.
