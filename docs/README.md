# Documentation index

This index is the navigation entry point for project documentation. It separates user guidance, contributor guidance, architecture decisions, language/API references, and research material.

> **Project status:** The repository has an accepted architecture and a four-crate Rust workspace. The compiler contains source-map/loading primitives, structured diagnostics, a narrow evidence-gated frontend, and a formatter for a limited verified syntax subset. A restricted build path uses a temporary C/`cc` bridge. This is not general GFA BASIC support and does not claim full reference-interpreter compatibility. General lexical/grammar coverage, semantic analysis, the planned LLVM backend, project configuration, runtime behavior, and platform integration remain incomplete. See [ROADMAP.md](../ROADMAP.md) for the current implementation state.

## Start here

| Audience | Start with | Purpose |
|---|---|---|
| New visitors and AI agents | [Project README](../README.md), then [AGENTS.md](../AGENTS.md) | Project purpose, current state, navigation, and mandatory agent workflow |
| Contributors | [Contributor documentation](contributor/README.md) | Contribution constraints and development workflow |
| Architects and implementers | [Architecture overview](architecture/README.md) | Architecture baseline, boundaries, open questions, and decisions |
| Language/API researchers | [Reference notes](reference/README.md) and [research index](research/README.md) | Version-scoped language/API research and historical evidence |
| Source reviewers | [Reference source register](reference-sources.md) | Provenance, applicability, evidence status, and citation conventions |

## Required agent discovery path

An agent that enters with no prior conversation context must:

1. Read the root README and [AGENTS.md](../AGENTS.md).
2. Read [ROADMAP.md](../ROADMAP.md) to understand current maturity and delivery order.
3. Read [compatibility evidence and agent workflow](architecture/COMPATIBILITY-EVIDENCE.md).
4. Inspect open GitHub issues and pull requests. Issues are the source of truth for scope, acceptance criteria, blockers, and implementation discussion.
5. Select an issue and follow its links to the canonical architecture, research, and test documents.
6. Keep evidence, implementation status, tests, and issue discussion aligned.

Do not use chat history, commit messages, or generated dashboards as substitutes for canonical project state.

## Documentation map

| Area | Canonical location | Audience and content |
|---|---|---|
| Project overview | [`README.md`](../README.md) | All readers; purpose, compatibility baseline, maturity, and limitations |
| AI agent workflow | [`AGENTS.md`](../AGENTS.md) | AI agents and maintainers; discovery, issue tracking, and human blockers |
| Project roadmap | [`ROADMAP.md`](../ROADMAP.md) | Delivery order, current implementation state, dependencies, and open work |
| Compatibility evidence | [`architecture/COMPATIBILITY-EVIDENCE.md`](architecture/COMPATIBILITY-EVIDENCE.md) | Reference interpreter, evidence priority, feature records, debugging, and tests |
| Documentation policy | [`DOCUMENTATION.md`](DOCUMENTATION.md) | Format, writing, evidence, copyright, and validation rules |
| User entry point | [`user/README.md`](user/README.md) | Current support status and future user-guide structure |
| Contributor entry point | [`contributor/README.md`](contributor/README.md) | Contribution constraints and workflow |
| Architecture | [`architecture/README.md`](architecture/README.md) | System boundaries and architecture document index |
| Implementation dashboard | [`architecture/IMPLEMENTATION-DASHBOARD.md`](architecture/IMPLEMENTATION-DASHBOARD.md); [ADR 0014](architecture/adr/0014-implementation-dashboard.md) | Maintainer dashboard and canonical status sources |
| Language/API research | [`reference/README.md`](reference/README.md) | Version- and platform-scoped language and API notes |
| Research procedures | [`research/README.md`](research/README.md) | Experiments, historical material, and source analysis |
| Source provenance | [`reference-sources.md`](reference-sources.md) | Source identity, access, licensing, and evidence |
| Conformance corpus | [`tests/corpus/README.md`](../tests/corpus/README.md) | Fixture conventions, metadata, and validation |

## High-level documentation layout

```text
docs/
├── README.md
├── DOCUMENTATION.md
├── reference-sources.md
├── architecture/
│   ├── README.md
│   ├── COMPATIBILITY-EVIDENCE.md
│   └── adr/
├── contributor/
├── reference/
├── research/
└── user/
```

Use each directory's README for detailed navigation. Add a detailed page when its content can be stated accurately and reviewed. Do not create empty placeholders or duplicate canonical definitions.

## Navigation conventions

- Keep essential information readable in GitHub without a separate documentation service.
- Give each directory with multiple related pages an index named `README.md`.
- Use relative links for files in this repository.
- Link to the canonical document for a definition. Do not maintain competing copies.
- Keep user instructions separate from contributor procedures and architecture rationale.
- Keep research notes separate from accepted architecture decisions and user-facing support claims.
- Mark proposed, unknown, inferred, disputed, and verified material according to [the documentation policy](DOCUMENTATION.md).
- State the dialect, platform, and version for every GFA BASIC language or API claim.
- Do not document an installation or command as usable until it has been tested in the declared environment.

## Ownership and review

Project maintainers own the documentation structure and resolve conflicts between documents. Authors are responsible for checking claims, citations, links, and copyright status. Reviewers must check evidence, version scope, implementation status, local links, and consistency with the issue tracker.
