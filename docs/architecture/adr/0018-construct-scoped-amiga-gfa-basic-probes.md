---
type: "Architecture Decision Record"
title: "Construct-scoped Amiga GFA-BASIC probes"
---

# ADR 0018: Construct-scoped Amiga GFA-BASIC probes

- **Status:** Accepted
- **Date:** 2026-11-17
- **Tracking:** [Manifest contract #570](https://github.com/vsimko/gfabasic-rust/issues/570), [change selection #571](https://github.com/vsimko/gfabasic-rust/issues/571), [parallel CI #572](https://github.com/vsimko/gfabasic-rust/issues/572)
- **Related:** [ADR 0017](0017-multi-version-amiga-gfa-basic-conformance.md)

## Context

The current native workflows execute version-level regression suites. A change to one language construct can therefore run unrelated probes, and the result is not naturally organized around the construct being investigated.

The project needs a durable catalogue that relates one language construct to its independently authored probe, evidence, expected behavior, and observed result for each exact interpreter version. CI should execute only the changed construct and its applicable interpreter versions. Different versions must execute independently and in parallel.

## Decision

Use one directory per language construct under `tests/language-constructs/<construct-id>/`.

Each construct directory uses:
- `manifest.yaml` as its machine-readable metadata source;
- one or more independently authored `.LST` probe files;
- an optional `README.md` for rationale or unresolved details that do not fit the manifest.

Use YAML for this new construct catalogue. Do not introduce TOML for construct manifests. This decision does not replace the existing TOML API registry. The corpus manifest and sidecars follow the separate migration tracked in [#574](https://github.com/vsimko/gfabasic-rust/issues/574); construct manifests remain a distinct contract.

The manifest contract must keep these dimensions distinct:
- construct identity and classification;
- documentary/source evidence and its exact platform/version scope;
- expected behavior for each exact reference interpreter;
- probe file or version-specific probe variant;
- observed execution outcome and artifact reference;
- Rust implementation and conformance status.

The exact interpreter identifiers remain `3.041U`, `3.5`, and `3.52`. Never transfer an expectation or result between versions. Use explicit unknown/unavailable states; do not infer support from a manifest, source sighting, parser acceptance, or successful host-side validation.

CI change detection maps changed construct directories to only the versions declared by their manifests. Each selected construct/version pair is a separate parallel job. Unchanged constructs are not run. A shared schema or selector change can validate all manifests without automatically scheduling every native interpreter run. Existing whole-suite workflows remain available as baseline and diagnostic paths during migration.

Native execution results are retained as workflow artifacts and summarized per construct/version. They are not automatically committed into the construct directory. A green host job means only that the configured harness and result protocol completed; it does not establish broader language conformance.

## Alternatives considered

1. **Keep only version-wide suites.** Rejected as the primary change-triggered workflow because a focused construct change needlessly executes unrelated tests.
2. **One manifest for all constructs.** Rejected because per-construct directories make probes, evidence, and ownership boundaries easier to inspect and change independently.
3. **TOML for new manifests.** Rejected for this catalogue; YAML is the selected authoring format. The existing API registry TOML contract remains unchanged; corpus-format migration is governed separately by #574.
4. **Run every construct on every version for every change.** Rejected because it defeats change-scoped execution and increases cost without adding relevant coverage.

## Consequences

- The schema, example manifests, and validator are defined under tracking issue #570.
- Deterministic change detection and construct/version selection are implemented in [PR #593](https://github.com/vsimko/gfabasic-rust/pull/593) for issue #571.
- Dynamic parallel native execution and result aggregation are defined under #572.
- The regression-testing and workflow-maintenance skills must follow this construct-scoped model.
- The existing `tests/interpreter-probes/` suites and native workflows remain supported until a separately reviewed migration is complete.
- Exact-version results continue to use `PASS`, `FAIL`, `DIFFERENT`, and `UNTESTED` as defined by ADR 0017.
