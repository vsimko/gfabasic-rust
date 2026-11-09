---
type: "Architecture Decision Record"
title: "ADR 0001: Documentation and architecture decision records"
---

# ADR 0001: Documentation and architecture decision records

- Status: Accepted
- Date: 2026-11-03

## Context

The project needs a discoverable architecture entry point and a traceable process for decisions. The repository is intended to host its own documentation and must distinguish verified language facts from planning assumptions.

## Decision

- Keep normative architecture documentation under `docs/architecture/`.
- Link to the architecture index from the root README.
- Record consequential decisions in numbered ADR Markdown files.
- Use the status vocabulary defined in the architecture index.
- Keep source provenance for GFA BASIC language and API claims in `docs/reference-sources.md`.
- Treat Markdown as the current human-readable documentation format. The “OKF” requirement remains unresolved until the exact specification is identified.

## Alternatives considered

- A single root-level architecture document: rejected because the planned architecture spans distinct topics and needs independently reviewable decisions.
- An external documentation website: rejected as the canonical source because repository-local documentation must remain usable without a hosted service.
- Unnumbered free-form decision notes: rejected because they do not provide stable references or a clear supersession history.

## Consequences

- Documentation changes are reviewable in Git.
- Decisions can be linked from issues and implementation changes.
- The project must not claim formal OKF compliance or formal ASD-STE100 certification without identifying and validating the applicable standard/tooling.
- Subsequent ADRs may supersede this decision, but must preserve its historical record.
