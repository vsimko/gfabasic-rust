---
type: "Architecture Decision Record"
title: "Multi-version Amiga GFA-BASIC conformance baseline"
---

# ADR 0017: Multi-version Amiga GFA-BASIC conformance baseline

- **Status:** Accepted
- **Date:** 2026-11-16
- **Tracking:** [Issue #557](https://github.com/vsimko/gfabasic-rust/issues/557)

## Context

The project uses original Amiga GFA-BASIC interpreters as behavioral references. Interpreter releases must not be assumed to behave identically. A concrete manually reported example is the abbreviation `M O a()`: GFA-BASIC 3.5 expands it as `MAT SET a()`, while GFA-BASIC 3.52 expands it as `MAT ONE a()`. This observation motivates independent version checks; it does not establish the behavior of every release or related command.

Using only one interpreter would miss historical differences. Treating all versions as interchangeable would also risk misclassifying a documented or observed version distinction as a defect.

## Decision

Adopt a three-version reference strategy:

| Interpreter | Role |
|---|---|
| GFA-BASIC 3.52 | Primary reference baseline for current development and default expected behavior |
| GFA-BASIC 3.5 | Compatibility comparison |
| GFA-BASIC 3.5U | Retired historical profile; no new routine test runs; retain old results as historical evidence only |
| GFA-BASIC 3.0 | Historical compatibility comparison |

Run the same applicable probes against the active matrix versions (3.52, 3.5, and 3.0) when the licensed interpreter asset and native harness are available. Preserve separate results for each exact version. Never infer a version's behavior from another version. The previously inventoried 3.5U package is not an active matrix target; its old probe results remain historical and must not be reported as current 3.5 coverage.

A confirmed difference is a compatibility distinction, not automatically a failure. Expected behavior must be scoped to the tested version and supported by the relevant manual or direct interpreter observation.

## Result vocabulary

Record one result per probe and interpreter version:

- **PASS** — observed behavior matches the version-specific expected result.
- **FAIL** — observed behavior contradicts the version-specific expected result.
- **DIFFERENT** — a verified cross-version behavior difference is present and recorded; this is not itself a failure.
- **UNTESTED** — the version could not be tested or evidence is insufficient. Missing assets or unavailable native runs must not be reported as PASS or FAIL.

Keep manual-documented behavior, directly observed behavior, and assumptions distinct. Identify the running interpreter from the editor menu, not from disk or archive names.

## Alternatives considered

1. **Use only 3.52.** Simpler and gives a consistent modern baseline, but loses visibility into 3.0 and 3.5 compatibility.
2. **Treat all versions as equivalent.** Rejected because the observed matrix-command abbreviation difference disproves that assumption.
3. **Use all versions without a primary baseline.** Rejected because it leaves current development without a clear default reference.

## Consequences

- New conformance probes use 3.52 as the default baseline.
- Applicable probes should run on 3.5 and 3.0 as comparison coverage.
- 3.5U is excluded from new runs and current matrix reporting unless the maintainer explicitly reactivates it.
- Reports and fixtures must retain exact version identity and per-version outcomes.
- A missing interpreter asset or unavailable workflow is an explicit coverage gap.
- Native workflows, private licensed assets, and result aggregation must be expanded incrementally; this ADR does not claim that all three active versions are currently available in CI.
- The status of private assets and runnable workflows must be checked from the current workflow configuration and latest issue/CI evidence, not inferred from the media inventory alone.

## Evidence and limitations

The `M O a()` difference is currently a maintainer-reported observation. Confirm it with preserved screenshots or native-run evidence for both exact interpreter versions before treating it as a fully reproducible conformance record. Consult the matching primary manual for version-specific expected semantics.
