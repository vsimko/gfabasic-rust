# Compatibility testing and release criteria

- Status: Accepted testing strategy; executable compiler conformance and release-test gates remain open under [issue #116](https://github.com/vsimko/gfabasic-rust/issues/116). The conventions and policy baseline are complete under [issue #10](https://github.com/vsimko/gfabasic-rust/issues/10).

## Conformance corpus

The source corpus is maintained in [tests/corpus/](../../tests/corpus/README.md). It currently contains **ten** independently authored Amiga GFA BASIC 3.5 fixtures marked `verified` from visual review of the cited manual pages: `MAT ONE`, `MAT CLR`, `MAT SET`, `MAT BASE`/`MAT TRANS`, `MAT NORM`, `MAT ADD`, `MAT SUB`, scalar-result `MAT MUL`, `FACT(0)`, and the `FACT`/`VARIAT`/`COMBIN` intrinsic group. The matrix addition and subtraction fixtures cover destination-expression forms, while the multiplication fixture covers the documented equal-length vector scalar product. This status supports only the documented syntax and expected behavior; none of these fixtures has been executed on the original interpreter. The Amiga 3.0 fixtures remain `candidate` and must not be promoted into the 3.5 contract without applicable evidence. The corpus validator checks metadata only. The narrow FACT/PRINT prototype has focused frontend/prototype tests and a native executable smoke test, but there is no general corpus runner or full dialect conformance suite. See [issue #42](https://github.com/vsimko/gfabasic-rust/issues/42) and the [corpus fixture manifest](../../tests/corpus/manifest.toml).

See the [current compatibility matrix](COMPATIBILITY-MATRIX.md) for the explicit statement of which dialects, profiles, and targets are not yet supported.

## Test categories

Use the category that best describes the contract under test. Categories may overlap; for example, a conformance test can be an end-to-end test.

1. **Unit tests** check one function, module, or invariant in isolation. They should avoid external processes and filesystem access unless those are the subject of the test.
2. **Golden/snapshot tests** compare stable, user-visible or compiler-produced output, such as diagnostics or normalized intermediate representations. Snapshot changes require review of the semantic diff; do not update snapshots only to make a failing test pass.
3. **Property-based tests** check general invariants across generated inputs. Record the generator configuration and seed when a failure must be reproduced.
4. **Fuzz tests** explore malformed and adversarial inputs for crashes, hangs, panics, and violated invariants. Keep fuzzing strategy and resource budgets under [issue #43](https://github.com/vsimko/gfabasic-rust/issues/43); do not make long fuzz campaigns part of every fast pull-request check.
5. **Integration tests** exercise boundaries between compiler stages, crates, runtime components, or platform adapters.
6. **Conformance tests** compare behavior with an applicable manual rule or a named trusted reference implementation. Each test must identify the exact dialect/version and API profile, expected behavior, and source page/section or reproducible observation.
7. **End-to-end tests** exercise the complete source-to-executable path and, where applicable, run the generated program. State the target and runtime prerequisites explicitly.
8. **Regression tests** preserve a minimal reproducer for each fixed defect. Link the test to the issue or change that introduced it.

The test layer (for example, lexer, semantic analysis, IR, backend, or runtime) and the test category are separate labels. Do not infer conformance from a unit, snapshot, or end-to-end test unless its expected behavior has an independent source of truth.

## Fixture layout and naming

Keep GFA BASIC source fixtures in [tests/corpus/](../../tests/corpus/README.md). They have their own dialect/version metadata and evidence policy; do not mix them with implementation-only fixtures.

When Cargo crates and test harnesses exist, keep crate-specific fixtures next to the owning crate, for example:

```text
crates/<crate>/tests/
  <suite>_tests.rs
  fixtures/
    <suite>/<case-id>/
      input.txt
      expected.stdout
      expected.stderr
      expected.exit
```

- Use stable lowercase kebab-case case IDs and descriptive Rust test names.
- Use a case directory when a test needs multiple input or expected-output files.
- Keep expected stdout, stderr, exit status, and diagnostic snapshots separate. Omit files that are not part of the contract.
- Put shared fixtures in a common location only when multiple crates genuinely test the same contract. Otherwise, keep ownership with the nearest crate.
- Do not store machine-specific absolute paths, developer home-directory data, generated binaries, secrets, or copyrighted source material without redistribution permission.
- Every fixture must state its purpose and any non-obvious provenance. Conformance fixtures must also follow the metadata contract in [tests/corpus/README.md](../../tests/corpus/README.md).

## Determinism and isolation

Tests must not depend on ambient developer-machine state.

- Inject or freeze clocks when behavior depends on time. Do not use wall-clock time as an implicit expected value.
- Seed pseudo-random generators explicitly. On failure, report the seed and enough generator settings to reproduce the case.
- Set locale, timezone, encoding, and relevant environment variables explicitly when they can affect results.
- Use unique temporary directories and clean them up automatically. Never rely on a fixed path under the repository or a developer's home directory.
- Avoid network access in ordinary unit and integration tests. Any test that requires an external service must be explicitly classified and isolated.
- Do not share mutable files, ports, process-global state, or fixed resource names between parallel tests.
- Mark a test as serial only when isolation is not practical, and document the shared resource that requires serialization.
- Record required operating systems, architectures, system libraries, devices, and runtime capabilities. Skip unsupported environments with an explicit reason; do not silently treat a skipped test as passed.

## Failure reporting

A failing test should provide enough information to reproduce and diagnose the failure without inspecting a developer's local machine.

Where applicable, report:

- test name, fixture ID, and failing compiler/runtime stage;
- source dialect/version, API profile, and target;
- expected and actual exit status, stdout, stderr, or structured diagnostic;
- random seed and generator settings;
- relevant toolchain versions and explicitly configured environment;
- a reproducible command or test invocation.

Normalize temporary paths and other unstable values before comparing snapshots. Do not remove meaningful source locations, diagnostic codes, or semantic differences merely to make output stable.

## Test layers

1. Source loading and diagnostics.
2. Lexer and parser unit tests.
3. AST and semantic analysis tests.
4. Typed IR verification and lowering tests.
5. LLVM code-generation tests.
6. Runtime core unit tests.
7. API profile and platform integration tests.
8. End-to-end source-to-executable tests.
9. Manual-derived conformance tests for verified rules.
10. Regression tests for every fixed defect.

A fixture does not become authoritative merely because it passes. OCR-derived claims require the page-image review specified by the source record; interpreter observations must name the tested release and remain distinct from manual-backed facts.

## Release criteria

A release must state compiler version, supported dialect/version subset, target, API profile statuses, known deviations, and required runtime dependencies. Do not claim full compatibility based only on unit-test coverage or successful compilation.

Tracking: [testing strategy #10 (completed)](https://github.com/vsimko/gfabasic-rust/issues/10), [executable conformance/release gates #116](https://github.com/vsimko/gfabasic-rust/issues/116), [test taxonomy #41](https://github.com/vsimko/gfabasic-rust/issues/41), [conformance corpus #42](https://github.com/vsimko/gfabasic-rust/issues/42), and [fuzzing strategy #43](https://github.com/vsimko/gfabasic-rust/issues/43).
