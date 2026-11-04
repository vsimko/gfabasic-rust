# Frontend fuzzing and robustness strategy

- Status: Strategy documented; no compiler frontend, fuzz harness, or fuzzing CI job exists yet.
- Tracking: [strategy #43 (completed)](https://github.com/vsimko/gfabasic-rust/issues/43); [implementation #118](https://github.com/vsimko/gfabasic-rust/issues/118).

## Goals and non-goals

Fuzzing is intended to find implementation defects when the future source frontend receives malformed, unusual, deeply nested, or adversarial input. It complements unit, property-based, regression, and manual-derived conformance tests; it does not establish language conformance by itself.

The fuzz targets must be usable without LLVM, SDL3, an Amiga/Atari emulator, or network access. They must not execute generated programs or invoke external interpreters as part of an ordinary fuzz iteration.

Fuzzing must not define new GFA BASIC syntax or semantics. A crash-free parser is not proof that it accepts the right language, and a fuzzer-generated input is not a normative language example.

## Target boundaries

Create separate targets when the corresponding frontend boundary exists and can be invoked without unrelated system dependencies. The names below are logical target names, not existing Cargo targets.

| Logical target | Input | Main robustness properties |
|---|---|---|
| Source loading | Bounded source bytes or text, after the source-encoding contract is defined | No panic on malformed input; bounded allocation; stable source identity and diagnostics |
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

The CI job should also have an outer timeout of no more than 5 minutes for the smoke mode. If the selected engine cannot enforce an individual-input timeout or a reliable outer job timeout, document the limitation and use process isolation before enabling it in CI.

Do not use a low recursion limit as a substitute for correct parser design. Deep nesting should be tested deliberately; a stack overflow or uncontrolled memory growth is a defect, while any supported-language nesting limit must be separately specified and tested.

## CI execution policy

After a harness and stable target interfaces exist:

1. Run bounded smoke fuzzing on pull requests that change the frontend, fuzz harnesses, fuzz dictionaries, or relevant corpus seeds.
2. Keep smoke execution separate from formatting, compilation, and deterministic unit tests so failures identify the responsible stage.
3. Run longer campaigns on a schedule and on manual dispatch. Do not make every pull request run a long campaign.
4. Preserve the fuzzer engine, version, target, seed, command, elapsed time, and relevant toolchain details for each failure.
5. Do not introduce a coverage-percentage gate before a repeatable baseline exists. First collect and review coverage trends.
6. Treat a timeout, crash, sanitizer finding, or unexpected panic as a failed fuzz job. Do not silently retry until green; retain the original failure artifact for triage.

The exact fuzzer engine and Rust toolchain arrangement remain open. Evaluate maintenance, supported platforms, reproducibility, sanitizer support, corpus minimization, CI integration, and whether the engine requires a separate nightly toolchain. If nightly is required, isolate it to the fuzz job and do not change the project's pinned stable development toolchain solely for fuzzing.

## Failure triage and regression workflow

For each reproducible failure:

1. Preserve the original failing input and record the target, dialect/version configuration, fuzzer/engine version, seed, command, and failure mode.
2. Minimize the input with the selected engine without changing the failure classification.
3. Determine whether the failure is a compiler defect, an invalid harness assumption, an unresolved language question, or an environment/tooling failure.
4. If it is an implementation defect, create or update a focused issue and add the smallest permitted reproducer to the regression suite.
5. Keep the regression test independent of the fuzzer runtime so it can run in the ordinary deterministic test suite.
6. Link the regression fixture to the issue and document any required evidence. Do not mark a syntax case as invalid merely because it is absent from the current reference inventory.

Do not commit ROMs, disk images, copyrighted sample collections, secrets, host paths, or large raw crash corpora. Follow the source-provenance and redistribution rules in [the reference-source register](../reference-sources.md).

## Decisions still required

- Select and pin a fuzzer engine after the Rust workspace and frontend entry points exist.
- Define the source encoding and source-composition contracts before implementing byte-level and include-resolution targets.
- Establish a baseline for input sizes, per-input timeouts, total CI cost, and useful coverage.
- Add the smoke and scheduled/manual workflows only after the harness is reproducible in a clean environment.

Until these prerequisites are met, this document is a test strategy only. It does not claim that fuzz targets, regression harnesses, or fuzzing workflows have been implemented.
