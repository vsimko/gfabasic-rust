# ADR 0014: Repository implementation dashboard

- Status: Accepted
- Date: 2026-11-07
- Decision makers: Project maintainers

## Context

The project website currently combines a public introduction, prototype description, API catalogue, and a coarse roadmap. Maintainers also need a concise visual view of implementation progress, evidence coverage, delivery dependencies, blockers, automated checks, and recent repository activity. A single percentage would hide important distinctions: research is not implementation, a prototype is not a supported feature, and documented API declarations do not imply runtime behavior or conformance.

The project already keeps its canonical information in repository files, validated registries, tests, GitHub Issues, and GitHub Actions. The website is a static GitHub Pages site and should remain useful without a runtime service, database, privileged token, or client-side API calls.

## Decision

1. Extend the existing static website with a maintainer-facing implementation dashboard while preserving its public project presentation.
2. Generate dashboard data at build time as deterministic, validated static JSON. Do not require a server, database, runtime secret, or browser-side GitHub API access.
3. Keep source-of-truth boundaries explicit:
   - GitHub Issues own task scope, acceptance criteria, blockers, and implementation discussion.
   - ROADMAP.md owns project delivery phases and sequencing.
   - The API registry owns API declaration metadata and declared evidence/support states.
   - Source inventories and ADRs own documented research provenance and accepted architectural decisions.
   - Tests and conformance results own executable verification claims.
   - GitHub Actions owns CI workflow and run status.
   - Git history and pull requests own repository activity.
4. Use component statuses Implemented, Partial, Prototype, Research, Planned, and Blocked. Each component entry must include a rationale, evidence link, related issue links where applicable, and an update timestamp or source revision.
5. Keep these dimensions separate: source evidence, implementation state, Linux/platform support, runtime observation, and conformance verification. Every metric must define its numerator, denominator, and inclusion rules.
6. Never calculate or display one aggregate implementation or compatibility percentage. Research, inferred evidence, and planning documents must not count as implementation or conformance.
7. Display only facts available from canonical sources. Missing, stale, conflicting, or unavailable data must be marked unknown/stale or cause generation validation to fail; it must not be inferred silently.
8. Render roadmap phases, dependencies, blockers, CI health, and recent activity as focused views that link to their underlying records. Do not invent delivery dates or completion claims.
9. Keep the implementation compatible with the existing HTML/CSS/JavaScript site and static hosting. Add a framework or hosted service only through a separate ADR.

## Alternatives considered

### Use a single progress percentage

Rejected because it collapses research, partial prototypes, implementation, and conformance into a misleading number.

### Query GitHub from the browser at runtime

Rejected because it introduces rate limits, availability and CORS concerns, and possible token exposure. Build-time generation is more deterministic and works with static hosting.

### Maintain dashboard facts manually in JavaScript

Rejected because it creates a second source of truth and makes status drift likely. Data must be derived from canonical repository sources and validated.

### Replace the existing website with a dashboard application

Rejected because the site is also the public project introduction and does not need a separate application framework.

## Consequences

- The dashboard remains deployable as static GitHub Pages content.
- A small generation and validation pipeline is required.
- Data provenance and freshness become part of the dashboard contract.
- Metrics may be unavailable until the relevant canonical source provides reliable data; unknown is preferable to an invented value.
- The public-facing site and the maintainer dashboard share one deployment but keep distinct information hierarchies.
- Implementation is split into issues [#304](https://github.com/vsimko/gfabasic-rust/issues/304) through [#310](https://github.com/vsimko/gfabasic-rust/issues/310).

## Evidence and references

- [Issue #304: dashboard overview](https://github.com/vsimko/gfabasic-rust/issues/304)
- [Issue #305: generated progress data model](https://github.com/vsimko/gfabasic-rust/issues/305)
- [Issue #306: roadmap, dependencies, and blockers](https://github.com/vsimko/gfabasic-rust/issues/306)
- [Issue #307: compatibility metrics](https://github.com/vsimko/gfabasic-rust/issues/307)
- [Issue #308: CI and supply-chain health](https://github.com/vsimko/gfabasic-rust/issues/308)
- [Issue #309: repository activity](https://github.com/vsimko/gfabasic-rust/issues/309)
- [Issue #310: dashboard documentation](https://github.com/vsimko/gfabasic-rust/issues/310)
- [Project roadmap](../../../ROADMAP.md)
- [ADR 0013: repository consistency validation](0013-repository-consistency-validation.md)

## Supersedes / superseded by

None.
