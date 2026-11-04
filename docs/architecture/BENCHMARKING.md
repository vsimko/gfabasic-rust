# Performance benchmarks and profiling strategy

- Status: **Initial source-loading microbenchmark implemented; compiler/frontend/runtime benchmarks and stable regression comparisons remain pending.**
- Tracking: [issue #60](https://github.com/vsimko/gfabasic-rust/issues/60).
- Purpose: define repeatable measurements that can guide optimization without confusing compiler speed, generated-program speed, and build-system performance.

## 1. Benchmark categories

Keep the following categories separate. A single end-to-end number cannot identify which subsystem changed.

| Category | Measurements | Example workload |
|---|---|---|
| Source loading | File discovery, decoding, source composition, source-map construction | One file; many small files; large source |
| Frontend | Lexing, parsing, AST construction, diagnostics | Synthetic programs with controlled token/statement counts |
| Semantic analysis | Name resolution, declarations, type checks, profile validation | Programs with increasing symbol, scope, and type counts |
| Project IR | Lowering, verification, and optimization passes owned by the project | Programs with controlled control-flow and data-flow complexity |
| LLVM backend | LLVM IR generation, verification, object emission | Fixed, already-validated IR inputs |
| Linking | Runtime selection and native link time | Small and large generated programs |
| End-to-end compilation | Source-to-executable wall time, CPU time, peak memory, output size | Representative redistributable projects and synthetic size tiers |
| Generated-program runtime | Execution time and memory for generated native programs | Numeric, string, array, file, and algorithm workloads |
| API/runtime compatibility | Core runtime and profile-specific API operations | Deterministic non-interactive API workloads; graphics/input measured separately |
| Build infrastructure | Clean build, incremental build, dependency compilation, cache hit/miss behavior | Fixed workspace and toolchain on a controlled runner |

Do not combine compiler execution time with generated-program execution time. Do not use an emulator's boot time as a proxy for the runtime cost of a generated program.

## 2. Workload policy

Use two complementary workload groups:

1. **Microbenchmarks** isolate one operation or compiler phase. They help locate regressions and compare implementations.
2. **Macrobenchmarks** use complete, independently authored programs. They measure behavior closer to real projects and expose interactions among phases.

Benchmark inputs must be synthetic or redistributable. Do not commit proprietary customer code, copyrighted manual listings, ROMs, disk images, or other material without explicit redistribution rights.

Keep a benchmark corpus separate from the conformance corpus:

- A conformance fixture establishes an expected language or runtime behavior.
- A benchmark workload measures cost and scaling.
- A fixture must not become a performance baseline until its syntax, expected result, and dialect/version identity are sufficiently verified.
- Every benchmark workload must still validate its output or result checksum before timing is accepted.

Use size tiers where possible. Record the workload version and input size so that results from different programs are not compared as if they were equivalent.

## 3. Measurement protocol

For each result, record:

- repository commit and benchmark-corpus revision;
- compiler version/build revision and build mode;
- Rust toolchain and LLVM version, once selected;
- host OS/kernel, CPU model, logical/physical core count, RAM, and architecture;
- output target, dialect/version, and API profiles;
- compiler flags, optimization level, and relevant configuration;
- input files, source size, and workload parameters;
- whether caches are cold or warm;
- measurement tool and its version;
- wall time, CPU time, peak resident memory, and artifact size where relevant;
- correctness result and any excluded samples.

Use a controlled machine for comparisons. Avoid comparing measurements from different CPUs, operating systems, toolchains, optimization modes, or cache states without labelling the difference.

For short microbenchmarks, use a statistical harness such as Criterion after the Rust workspace is created. For process-level macrobenchmarks, use a repeatable runner that performs warm-up runs and multiple measured repetitions. Report a distribution (at least median and spread; include p95 when the sample count supports it), not a single fastest run.

Measure cold and warm build paths separately. For generated-program runtime, separate process startup from steady-state work when that distinction matters. Do not include benchmark harness setup or result validation in the timed section unless that overhead is intentionally part of the metric.

## 4. Correctness and determinism

A performance result is valid only if the workload completed with the expected result.

- Fix random seeds and input data.
- Fix locale, timezone, environment variables, and temporary-path behavior where relevant.
- Avoid network access and external services.
- Keep test data and output sizes bounded and documented.
- Verify output, exit status, and side effects outside the timed region where possible.
- Record failures and outliers; do not silently discard slow results.
- Use the same benchmark workload and toolchain settings when comparing revisions.

For graphics and input, separate deterministic off-screen or headless operations from interactive latency. Record display backend, resolution, refresh rate, input device, and frame-synchronization settings for interactive measurements. Do not use unstable desktop timing as a hard CI gate.

## 5. Profiling workflow

Use profiling only after a repeatable benchmark identifies a meaningful regression or optimization opportunity.

Candidate tools for a Linux host include:

- Criterion for Rust microbenchmarks;
- Linux perf for CPU counters and sampled profiles;
- flamegraph-style reports for call-stack analysis;
- system tools for peak memory and process-level timing.

These are candidate tools, not selected dependencies. Choose and pin them after the Rust workspace and CI environment are established. Keep instrumented/profiled runs separate from baseline timing runs because instrumentation can change performance.

Profile one subsystem at a time. Record the hypothesis, workload, profile command, relevant counters, finding, and follow-up change. Do not optimize based only on an unrepeatable local measurement.

## 6. CI and regression policy

There should be no hard performance threshold before a stable baseline exists.

Proposed rollout:

1. Add a workload as each compiler/runtime component becomes executable; source loading is the first implemented workload.
2. Establish baselines on a known runner and publish machine/toolchain metadata with every result.
3. Run a small smoke set on pull requests only after its duration and variance are understood.
4. Run the broader suite on a scheduled or release workflow.
5. Store raw results as build artifacts and retain a comparable summary.
6. Introduce regression alerts only when repeated measurements show a stable noise floor and the team has selected justified thresholds.

Do not fail every pull request because of a small timing fluctuation. A regression gate must use comparable hardware, a known baseline, and a threshold larger than normal run-to-run noise. Revisit thresholds when the runner, compiler, LLVM, or benchmark corpus changes.

## 7. Current source-loading benchmark

A first microbenchmark is implemented in the separate `bench` workspace. It
measures the generic strict UTF-8 byte decoder and source-map construction,
not GFA BASIC syntax, dialect encoding, filesystem I/O, or source composition.
The synthetic workload is deliberately not a language-conformance fixture.

The Criterion 0.8.2 harness measures:
- one source of 4 KiB, 64 KiB, and 1 MiB;
- 128 independent 1 KiB sources in one source map.

Input preparation is outside the timed routine. The timed routine constructs
the source map, decodes the owned byte buffers, builds line indexes, and adds
the sources. The harness uses 20 samples, a one-second warm-up, and two seconds
of measurement per benchmark. It reports throughput and Criterion statistics.

Benchmark-only dependencies live in the independent `bench` workspace and
are excluded from ordinary workspace builds and the release SBOM. The
benchmark workflow compiles the harness on relevant pull requests but runs
measurements only on a weekly schedule or manual dispatch. Results and host
metadata are uploaded as 30-day artifacts. There are no hard performance
thresholds.

The workflow records commit, toolchain, kernel, architecture, CPU model,
logical CPU count, memory, target, and synthetic workload revision. GitHub
hosted runner hardware can vary; compare results only when the recorded host
and toolchain metadata match. Initial results from the scheduled/manual workflow will be observational,
not a controlled-machine regression baseline.

Run locally with:

```sh
cargo +1.99.1 bench --manifest-path bench/Cargo.toml --target-dir bench/target --bench source_loading --locked -- --noplot
```

See [`bench/README.md`](../../bench/README.md) for workload boundaries and
artifact details.

## 8. Remaining benchmark work

- Add frontend benchmarks after lexer/parser entry points exist (#46/#47).
- Add semantic/IR/backend benchmarks only when those stages have real,
  deterministic inputs and outputs.
- Add runtime/API benchmarks after runtime contracts and implementations exist
  (#51–#55).
- Establish comparable baselines on a controlled, stable machine before
  introducing regression alerts or hard gates.

## 9. Initial project status

As of 2026-11-04:

- The repository has a three-crate Rust workspace and a baseline CI workflow.
- The compiler frontend beyond source loading, runtime behavior, and LLVM/SDL3 integration are not implemented.
- A source-loading microbenchmark harness exists; other compiler/runtime benchmark workloads remain unavailable.
- No stable cross-run performance baseline or regression gate exists yet.
- The broader benchmark categories, measurement protocol, correctness requirements, profiling workflow, and regression policy remain design guidance; only source loading currently has an executable benchmark.
- Criterion 0.8.2 is selected for the isolated benchmark workspace; Linux profiling tools remain candidates.
- CI cache performance is not yet measured. Do not infer cache effectiveness from a successful build or cache hit alone.

The next implementation steps are to add deterministic benchmarks for lexer/parser, semantic, IR, backend, and runtime components as those components become executable. Keep timed measurements out of the fast pull-request path until their runtime and variance are understood. Do not create empty targets or claim baselines before there is executable work to measure.

Issue #60 can be closed only after the benchmark harness and representative workloads exist, a baseline can be reproduced on a documented environment, and the project has a repeatable way to compare results. The strategy in this document is necessary groundwork, but it does not satisfy those implementation and baseline criteria by itself.
