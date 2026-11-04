# Documentation index

This index is the entry point for project documentation. It separates user guidance, contributor guidance, architecture decisions, language/API references, and research material.

> **Project status:** The repository has an accepted architecture and a bootstrapped three-crate Rust workspace. Compiler, runtime, and CLI behavior are not implemented. Documentation checks and fast Rust workspace checks are available; platform integration, conformance, fuzz, security, and release workflows are not. A module placeholder does not mean that the corresponding feature is available.

## Start here

| Audience | Start with | Purpose |
|---|---|---|
| New visitors and prospective users | [Project README](../README.md) | Project purpose, current maturity, planned direction, and limitations |
| Users | [User documentation](user/README.md) | Current support status and the location of future usage guides |
| Contributors | [Contributor documentation](contributor/README.md) | Current contribution constraints and links to architecture and test policy |
| Architects and implementers | [Architecture overview](architecture/README.md) | Architecture baseline, boundaries, open questions, and decision records |
| Language/API researchers | [Reference notes](reference/README.md) | Platform- and version-scoped language and API research |
| Source reviewers | [Reference source register](reference-sources.md) | Provenance, applicability, evidence status, and citation conventions |

## Documentation map

| Area | Canonical location | Audience and content |
|---|---|---|
| Project overview | [`README.md`](../README.md) | All readers; concise project status and direction |
| Documentation policy | [`DOCUMENTATION.md`](DOCUMENTATION.md) | Authors and reviewers; format, writing, evidence, copyright, and validation rules |
| User entry point | [`user/README.md`](user/README.md) | Users; current support status and future user-guide structure |
| Contributor entry point | [`contributor/README.md`](contributor/README.md) | Contributors; current development constraints and contribution workflow |
| Architecture | [`architecture/README.md`](architecture/README.md) | Implementers; system boundaries and architecture document index |
| Architecture decisions | [`architecture/adr/TEMPLATE.md`](architecture/adr/TEMPLATE.md) | Maintainers; structure for numbered, reviewable decisions |
| Language and API research | [`reference/README.md`](reference/README.md) | Researchers; source-backed, dialect/version-specific notes |
| Source provenance | [`reference-sources.md`](reference-sources.md) | Researchers and reviewers; source identity, access, licensing, and evidence |
| Research procedures and corpora | [`research/README.md`](research/README.md) | Researchers; experiment procedures and analysis of historical material |
| Reusable page templates | [`templates/README.md`](templates/README.md) | Documentation authors; starting structures for guides and reference pages |

## Repository layout

The current documentation tree is:

```text
docs/
├── README.md
├── DOCUMENTATION.md
├── reference-sources.md
├── architecture/
│   ├── README.md
│   └── adr/
├── contributor/
│   └── README.md
├── reference/
│   └── README.md
├── research/
│   └── README.md
├── templates/
│   ├── README.md
│   ├── GUIDE.md
│   └── REFERENCE.md
└── user/
    └── README.md
```

The architecture, reference, and research directories contain the detailed material. The templates directory has its own index. User and contributor directories currently provide entry points only. Add a detailed page when its content can be stated accurately and reviewed; do not create empty placeholder pages or copy the same definition into multiple locations.

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

## Planned user and contributor pages

Create these pages only when the implementation or workflow provides verified instructions:

| Planned page | Location | Prerequisite |
|---|---|---|
| Installation and prerequisites | `user/installation.md` | A distributable compiler build and declared host baseline |
| First compilation and CLI usage | `user/quick-start.md` | A working CLI and a redistributable example |
| Language support and limitations | `user/language-support.md` | A maintained compatibility matrix and verified implementation status |
| Troubleshooting | `user/troubleshooting.md` | Known user-visible failure modes and diagnostic behavior |
| Development environment | [`contributor/development-setup.md`](contributor/development-setup.md) | Pinned Rust toolchain, workspace build/test commands, and troubleshooting |
| Build and test commands | `contributor/build-and-test.md` | Executable local checks and corresponding CI jobs |
| Contribution workflow | `contributor/contribution-workflow.md` | Agreed review, branch, and release practices |

Until those prerequisites exist, the README and the entry-point pages must state the limitation instead of inventing commands.

## Ownership and review

Project maintainers own the documentation structure and resolve conflicts between documents. The author of a change is responsible for checking its claims, citations, links, and copyright status. Reviewers should check:

1. The intended audience and canonical location.
2. Consistency with accepted architecture decisions and current implementation.
3. Dialect, platform, version, source edition, and evidence status for language/API claims.
4. Whether examples and commands are independently authored and actually verified.
5. Relative links, heading structure, and GitHub rendering.
6. Copyright and redistribution constraints.

The repository currently has no `CODEOWNERS` file. Until formal path ownership is introduced, documentation changes follow the normal repository pull-request review process and require maintainer review. See [the documentation policy](DOCUMENTATION.md) and [contributor entry point](contributor/README.md).

## Templates

- [Guide template](templates/GUIDE.md) — procedures and task-oriented user or contributor guidance.
- [Reference template](templates/REFERENCE.md) — stable technical facts, declarations, and source-backed language/API entries.

Templates are authoring aids. Remove their instructions and unused sections from a published page. A template does not establish that a feature exists or is supported.
