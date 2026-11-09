---
type: "Architecture"
title: "Implementation dashboard"
---

# Implementation dashboard

## Purpose

The project website provides two views: a public project introduction and a maintainer-facing implementation dashboard. The dashboard summarizes current repository evidence. It does not replace GitHub Issues, the project roadmap, source research, tests, or CI.

The architecture decision is [ADR 0014](adr/0014-implementation-dashboard.md). Implementation work is tracked in [issues #304–#310](https://github.com/vsimko/gfabasic-rust/issues).

## Canonical data sources

| Dashboard information | Canonical source |
|---|---|
| Task scope, acceptance criteria, blockers | GitHub Issues |
| Delivery phases and sequencing | ROADMAP.md |
| API declarations and declared support/evidence metadata | API registry |
| Language research and provenance | Research inventories and reference-source register |
| Architecture decisions | ADRs |
| Implemented and verified behavior | Source code and automated tests |
| CI health | GitHub Actions workflow definitions and run results |
| Recent delivery activity | Git commits and pull requests |

Do not copy facts into a manually maintained dashboard-only inventory when they already exist in one of these sources. If a future generator needs a compact intermediate manifest, treat it as generated output and validate it against its canonical inputs.

## Component status vocabulary

- **Implemented** — code exists and relevant automated checks pass.
- **Partial** — only a defined subset is implemented; the supported boundary must be stated.
- **Prototype** — experimental implementation that is not a general supported capability.
- **Research** — evidence gathering or source analysis; it is not implementation.
- **Planned** — no implementation claim is made.
- **Blocked** — an external decision, source, artifact, or dependency prevents progress.

Every component status must include a short rationale and a link to evidence. Link a GitHub issue when the work is tracked. Do not infer status only from issue closure: an architecture or documentation issue can be complete while the corresponding product feature remains unimplemented.

## Independent compatibility dimensions

Keep these dimensions separate:

1. **Evidence** — documented, primary-reference cross-checked, runtime-observed, inferred, disputed, or unknown.
2. **Implementation** — implemented, partial, not implemented, or unsafe-disabled.
3. **Platform support** — supported, partial, adapter-backed, emulated, unsupported, unsafe-disabled, or unknown.
4. **Runtime observation** — whether behavior has been observed in the reference interpreter or generated runtime.
5. **Conformance** — whether an executable test verifies behavior against a trusted reference or an explicitly accepted rule.

Each count must show its numerator, denominator, and definition. Do not count inferred or disputed evidence as verified. Do not count research as implementation. Do not calculate a single overall implementation or compatibility percentage.

## Generation and validation contract

The dashboard is static. Generate its JSON during the site build or a dedicated repository workflow. The browser must not need a GitHub token or make privileged GitHub API requests.

The generator must:

- use deterministic ordering and stable identifiers;
- record source links and source revision/update time;
- validate its schema and required fields;
- distinguish missing data from zero;
- reject conflicting or malformed source data;
- avoid fabricated timestamps, delivery dates, or status transitions;
- expose a clear stale/unavailable state when live workflow or activity data cannot be refreshed;
- support offline rendering from checked-in generated data where the project workflow requires it.

The exact schema and generator implementation are tracked by [#305](https://github.com/vsimko/gfabasic-rust/issues/305). CI, activity, and metric-specific contracts are tracked by [#307](https://github.com/vsimko/gfabasic-rust/issues/307), [#308](https://github.com/vsimko/gfabasic-rust/issues/308), and [#309](https://github.com/vsimko/gfabasic-rust/issues/309).

## Presentation rules

- Keep the dashboard near the top of the website, after the introductory hero.
- Show current maturity and the most important component states first.
- Link every displayed number or status to its source.
- Make roadmap dependencies and blockers visible without inventing target dates.
- Show only workflows that exist. Mark a missing check as not configured, not as passing.
- Show recent commits and pull requests as activity, not as proof that a feature is complete.
- Preserve the existing API catalogue and public project explanation.
- Use semantic HTML, keyboard-accessible controls, responsive layouts, and explicit loading/error/empty states.

## Maintenance

Update the canonical source first, then regenerate dashboard data. Update ROADMAP.md when phase order, sequencing, or project maturity changes. Update GitHub Issues when task scope or acceptance criteria change. Update an ADR when the architecture decision changes. Never edit generated JSON by hand unless the generation policy explicitly defines it as an editable source.
