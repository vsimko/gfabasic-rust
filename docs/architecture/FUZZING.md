---
type: "Architecture"
title: "Frontend fuzzing and robustness strategy"
---

# Frontend fuzzing and robustness strategy

- Status: Source-loader fuzz target and bounded CI campaign are implemented; lexer/parser targets remain pending.
- Tracking: [strategy #43 (completed)](https://github.com/vsimko/gfabasic-rust/issues/43); [implementation #118](https://github.com/vsimko/gfabasic-rust/issues/118).

## Goals and non-goals

Fuzzing is intended to find implementation defects when the future source frontend receives malformed, unusual, deeply nested, or adversarial input. It complements unit, property-based, regression, and manual-derived conformance tests; it does not establish language conformance by itself.

The fuzz targets must be usable without the project LLVM backend, SDL3, an Amiga/Atari emulator, or network access. They must not execute generated programs or invoke external interpreters as part of an ordinary fuzz iteration.

Fuzzing must not define new GFA BASIC syntax or semantics. A crash-free parser is not proof that it accepts the right language, and a fuzzer-generated input is not a normative language example.

## Target boundaries

Create separate targets when the corresponding frontend boundary exists and can be invoked without unrelated system dependencies. The names below are logical target names, not existing Cargo targets.

| Logical target | Input | Main robustness properties |
|---|---|---|
| Source loading | Bounded bytes passed through the generic strict UTF-8 transport helper; dialect encoding remains separate | No panic on malformed input; bounded allocation; stable source identity and diagnostics |
| Lexer | Source text using the selected dialect/version configuration | No panic or hang; token spans remain within source bounds; repeated runs are deterministic |
| Parser | Source text or a stable token-stream entry point | No panic or hang; malformed input produces controlled diagnostics; nesting cannot cause uncontrolled resource exhaustion |
| Expressions and statements | Source fragments parsed through the ordinary frontend entry point | Robust precedence, delimiter, nesting, and recovery paths without inventing acceptance rules |
| Labels and declarations | Source fragments exercising declaration/name-resolution boundaries once those contracts exist | No unexpected panic; diagnostics retain valid source locations |
| Source composition | Project-local files and include inputs, only after composition semantics are specified | No access outside the test sandbox; deterministic resolution; bounded file count and total input |

Do not expose unstable internal parser functions solely to satisfy a fuzzer. Prefer public or crate-internal frontend entry points that exercise the same path used by normal compilation.

A target must state its selected dialect/version and configuration. Until the first dialect and its lexical/source-encoding rules are accepted, byte-level source-loader behavior remains an open contract.

## Input corpora and dictionaries

Keep fuzzing inputs separate from the manual-derived conformance corpus:

- Fuzzer seed corpora belong under `fuzz/corpus/<target>/` when a harness is introduced.
- Minimized defect reproducers promoted to permanent regression coverage belong under `tests/regression/fuzz/<target>/` (or the owning crate's test-fixture directory once the workspace layout is accepted).
- Generated crash, timeout, and temporary artifact output belongs in an ignored local/build-artifact directory. Do not commit untriaged fuzzer output wholesale.
- Keep the existing `tests/corpus/` for dialect/version-scoped research and conformance fixtures. Do not mix mutation artifacts into it.

Start with small synthetic inputs and only those language fragments whose spelling and role have adequate source evidence. A verified manual excerpt may be used as a seed only when its retention and redistribution are permitted. The existing research-only Amiga 3.0 candidate fixtures are not automatically authoritative seeds or expected-behavior oracles; review their evidence and provenance first.

A mutation dictionary may contain keywords, operators, delimiters, and type suffixes only when they are supported by the selected dialect/version evidence. Record the source for each dictionary set. Do not populate a dictionary from guesses, third-party extensions, or another platform's dialect without explicit cross-checking.

## Required invariants

Every target should check the properties that are meaningful for its boundary:

- Arbitrary malformed input must not cause an unexpected Rust panic, abort, or process crash.
- Processing must terminate within the configured fuzzing resource budget; hangs and pathological slowdowns are defects to triage.
- Memory and recursion growth must be bounded by implementation safeguards and the external job budget. Fuzz harness limits are operational safeguards, not GFA BASIC language limits.
- Repeating the same input with the same configuration must produce deterministic tokens, parse results, or diagnostics, as applicable.
- Any emitted source span must be within the corresponding source buffer and must not overflow when offsets are combined.
- Error recovery must not loop forever or repeatedly emit the same diagnostic without progress.
- If source composition is eventually fuzzed, all filesystem access must remain inside an isolated temporary project root. Symlink and path traversal cases must be included in the adversarial set.

Do not assert that every arbitrary input is valid or that malformed input must receive a particular diagnostic unless that behavior is part of an accepted frontend contract.

## Resource budgets

Use explicit limits so fuzzing is useful in CI and cannot consume an unbounded runner allocation. The following are initial proposed harness budgets, not compiler input limits or language restrictions:

| Mode | Proposed execution budget | Purpose |
|---|---:|---|
| Pull-request smoke | Up to 30 seconds per target, at most 2 minutes total fuzz execution | Catch obvious crashes and hangs quickly |
| Scheduled/manual campaign | Start at 10 minutes per target; increase after measuring runner cost and coverage | Explore deeper input combinations |
| Individual input | Use the fuzzer's per-input timeout where supported; begin with 1 second and tune from measurements | Isolate pathological cases |
| Input size | Begin with a 1 MiB maximum generated input and adjust using real source-size requirements | Prevent oversized mutations from dominating the campaign |

Give the fuzzer process a bounded execution time and configure a separate outer CI timeout that includes tool installation and compilation. The current source-loader workflow uses a 15-minute outer job limit.

Do not use a low recursion limit as a substitute for correct parser design. Deep nesting should be tested deliberately; a stack overflow or uncontrolled memory growth is a defect, while any supported-language nesting limit must be separately specified and tested.

## CI execution policy

For each available harness and stable target interface:

1. Run bounded smoke fuzzing only when a pull request changes a path in the trigger matrix below. The matrix covers the source-loader target's build and runtime dependency closure, its harness and corpus, and the fuzz policy/workflow itself.
2. Keep smoke execution separate from formatting, compilation, and deterministic unit tests so failures identify the responsible stage.
3. Run longer campaigns on a schedule and on manual dispatch. Do not make every pull request run a long campaign.
4. Preserve the fuzzer engine, version, target, seed, command, elapsed time, and relevant toolchain details for each failure.
5. Do not introduce a coverage-percentage gate before a repeatable baseline exists. First collect and review coverage trends.
6. Treat a timeout, crash, sanitizer finding, or unexpected panic as a failed fuzz job. Do not silently retry until green; retain the original failure artifact for triage.

The engine and toolchain are selected for the source-loader target; see Current implementation below. Add further targets only when their ordinary compiler entry points and input contracts are stable.

## Failure triage and regression workflow

For each reproducible failure:

1. Preserve the original failing input and record the target, dialect/version configuration, fuzzer/engine version, seed, command, and failure mode.
2. Minimize the input with the selected engine without changing the failure classification.
3. Determine whether the failure is a compiler defect, an invalid harness assumption, an unresolved language question, or an environment/tooling failure.
4. If it is an implementation defect, create or update a focused issue and add the smallest permitted reproducer to the regression suite.
5. Keep the regression test independent of the fuzzer runtime so it can run in the ordinary deterministic test suite.
6. Link the regression fixture to the issue and document any required evidence. Do not mark a syntax case as invalid merely because it is absent from the current reference inventory.

Do not commit ROMs, disk images, copyrighted sample collections, secrets, host paths, or large raw crash corpora. Follow the source-provenance and redistribution rules in [the reference-source register](../reference-sources.md).

## Current implementation

The first target is the generic source-loader byte boundary. It exercises
`SourceMap::add_utf8_bytes`, which is also used by the filesystem loader.
It checks strict UTF-8 rejection, source-map immutability on decode failure,
source identity, line indexing, and valid line/column boundaries. This is a
transport-level UTF-8 test only: it does **not** assert that Amiga GFA BASIC
source files use UTF-8, and it does not test MERGE/include behavior.

The selected engine is AFL++ through the Rust `afl` and `cargo-afl`
integration, both pinned to version `0.18.2`. Their published package
manifests declare Apache-2.0, which is in the approved dependency allowlist.
The fuzz workspace is separate from the normal Cargo workspace so its
fuzz-only dependencies do not enter ordinary builds. CI uses the project's
pinned stable Rust 1.99.1 toolchain on Linux x86-64; this engine does not
require switching the project toolchain to nightly.

CI runs a 30-second smoke campaign on relevant pull requests and a 10-minute
campaign on the weekly schedule or manual dispatch. Both modes cap individual
inputs at 1 MiB, use a 1-second per-input timeout and a 256 MiB target memory
limit, and run under a 15-minute job timeout. The workflow uploads a portable tarball containing raw AFL++ output, attempted
minimized crash/hang inputs, and target/toolchain/corpus metadata, plus the
committed fuzz-workspace lockfile, for seven days. It records the commit, corpus checksums, campaign limits, and
frontend stages explicitly skipped because their entry points are not ready.
On GitHub-hosted runners it bypasses AFL++'s core-pattern preflight because the
runner routes core dumps through an external handler; crash and hang output
files are still checked and retained. No language dictionary is used until its tokens
are supported by the selected dialect evidence.

Local usage and corpus handling are documented in [`fuzz/README.md`](../../fuzz/README.md).
Fuzzer output is local/generated data; do not commit it. Minimize any crash or
hang and promote a permitted reproducer into the deterministic regression
suite before treating it as fixed.

## Remaining implementation decisions

- Add a lexer target after the selected Amiga GFA BASIC interpreter 3.5 lexical rules and token entry point are sufficiently specified in #46. Treat 3.0/3.03 rules only as inherited candidates until confirmed against 3.5.
- Add a parser target after the parser/AST entry point exists in #47.
- Add source-composition fuzzing only after #120 defines deterministic resolution and filesystem sandbox rules.
- Establish useful coverage baselines before adding coverage-percentage gates.
