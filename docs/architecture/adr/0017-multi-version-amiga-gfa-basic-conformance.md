---
type: "Architecture Decision Record"
title: "Multi-version Amiga GFA-BASIC conformance baseline"
---

# ADR 0017: Multi-version Amiga GFA-BASIC conformance baseline

- **Status:** Accepted
- **Date:** 2026-11-16
- **Tracking:** [Issue #557](https://github.com/vsimko/gfabasic-rust/issues/557)
- **Construct-scoped execution:** [ADR 0018](0018-construct-scoped-amiga-gfa-basic-probes.md)

## Context

The project uses original Amiga GFA-BASIC interpreters as behavioral references. Interpreter releases must not be assumed to behave identically. A concrete manually reported example is the abbreviation `M O a()`: GFA-BASIC 3.5 expands it as `MAT SET a()`, while GFA-BASIC 3.52 expands it as `MAT ONE a()`. This observation motivates independent version checks; it does not establish the behavior of every release or related command.

Using only one interpreter would miss historical differences. Treating all versions as interchangeable would also risk misclassifying a documented or observed version distinction as a defect.

## Decision

Adopt a three-version reference strategy:

| Interpreter | Role |
|---|---|
| GFA-BASIC 3.52 | Active native runtime baseline and default expected behavior |
| GFA-BASIC 3.041U | Available native comparison profile for the 3.0-era dialect; retain the exact version label |
| Amiga GFA-BASIC 3.5 | Normative source-language target under ADR 0009; editor identity and initial three-case native smoke suite verified (arithmetic, FOR/NEXT, strings); broader conformance remains incomplete |

Run the same applicable probes against active native profiles when the licensed interpreter asset and native harness are available. The current profiles are 3.52 and 3.041U. Preserve separate results for each exact version; do not relabel 3.041U as exactly 3.0. Amiga 3.5 remains the normative documented source target. The candidate interpreter identity is visually verified: the running editor menu captured with the right mouse button held displays `GFA-BASIC 3.5 Block`. The matrix's initial native 3.5 Merge + Run smoke suite completed in run [44094135503](https://github.com/vsimko/gfabasic-rust/actions/runs/44094135503). The retained artifact `amiga-gfa-basic-3.5-asset-discovery` (artifact 14333027972) contains the held-right-button menu screenshot visibly identifying `GFA-BASIC 3.5 Block`, Merge/Run screenshots, and raw output `GFATEST:arithmetic:PASS`, `GFATEST:for-next:PASS`, `GFATEST:strings:PASS`. Record these three smoke probes as PASS for this exact candidate; this is not complete language conformance.

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
- Run applicable probes on 3.041U as the available 3.0-era comparison profile.
- Record the initial 3.5 smoke probes as PASS only for the exact run and artifact inspected above; broader 3.5 language-conformance coverage remains incomplete.
- Reports and fixtures must retain exact version identity and per-version outcomes.
- A missing interpreter asset or unavailable workflow is an explicit coverage gap.
- The unified matrix entry point is `.github/workflows/amiga-reference-matrix.yml`. It invokes the 3.52 and 3.041U native workflows and captures the 3.5 editor's held-right-button version menu as identity evidence. The initial 3.5 native smoke suite is wired into the matrix; the three configured cases passed in run 44094135503 and the raw artifact was inspected. This is limited smoke coverage, not complete language conformance. The individual workflows remain available for focused manual runs. A successful matrix run validates only the configured probes and result-collection protocol, not complete language conformance.
- New construct-scoped probes and change-triggered version jobs follow [ADR 0018](0018-construct-scoped-amiga-gfa-basic-probes.md). The existing whole-suite workflows remain available during migration.
- The status of private assets and runnable workflows must be checked from the current workflow configuration and latest issue/CI evidence, not inferred from the media inventory alone.

## Evidence and limitations

The `M O a()` difference is currently a maintainer-reported observation. Confirm it with preserved screenshots or native-run evidence for both exact interpreter versions before treating it as a fully reproducible conformance record. Consult the matching primary manual for version-specific expected semantics.
