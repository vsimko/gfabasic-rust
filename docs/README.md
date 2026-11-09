# Documentation index

The [OKF knowledge bundle](../knowledge/index.md) is the agent-oriented progressive-disclosure entry point. It uses Google's Open Knowledge Format (OKF) v0.2: Markdown concepts with YAML frontmatter, directory indexes, and standard links.

## Start here

| Audience | Start with | Purpose |
|---|---|---|
| New visitors and AI agents | [Project README](../README.md), then [OKF knowledge bundle](../knowledge/index.md) and [AGENTS.md](../AGENTS.md) | Project purpose, current status, agent workflow, and concept discovery |
| Contributors | [Contributor documentation](contributor/README.md) | Contribution constraints and development workflow |
| Architects and implementers | [Architecture overview](architecture/README.md) | Architecture baseline, boundaries, open questions, and decisions |
| Language/API researchers | [Reference notes](reference/README.md) and [research index](research/README.md) | Version-scoped language/API research and historical evidence |

## Canonical sources

The OKF bundle provides concise typed concepts and navigation. Detailed implementation status is in [ROADMAP.md](../ROADMAP.md) and GitHub issues. Detailed architecture, research, contributor, and user documents remain under this directory. Source code and tests define implemented behavior.

| Area | Canonical location |
|---|---|
| OKF knowledge bundle | [`knowledge/index.md`](../knowledge/index.md) |
| Agent instructions | [`AGENTS.md`](../AGENTS.md) |
| Project roadmap | [`ROADMAP.md`](../ROADMAP.md) |
| Compatibility evidence | [`architecture/COMPATIBILITY-EVIDENCE.md`](architecture/COMPATIBILITY-EVIDENCE.md) |
| Documentation policy | [`DOCUMENTATION.md`](DOCUMENTATION.md) |
| Architecture | [`architecture/README.md`](architecture/README.md) |
| Language/API research | [`reference/README.md`](reference/README.md) |
| Research procedures | [`research/README.md`](research/README.md) |
| Source provenance | [`reference-sources.md`](reference-sources.md) |
| Conformance corpus | [`tests/corpus/README.md`](../tests/corpus/README.md) |

## Migration status

The `knowledge/` directory is the OKF v0.2 bundle. The existing `docs/` tree is a legacy Markdown documentation set and is not yet a conformant OKF bundle because its concept files do not all have YAML frontmatter. Migrate detailed documents in small, reviewed batches. Do not claim the entire repository is OKF-conformant until every non-reserved Markdown file in the bundle has valid frontmatter with a non-empty `type`.
