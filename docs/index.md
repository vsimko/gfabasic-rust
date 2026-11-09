---
okf_version: "0.2"
---

# GFA BASIC project knowledge

This bundle is the canonical human- and agent-readable documentation source for the gfabasic-rust project. Markdown concept bodies use OKF v0.2 YAML frontmatter; the machine-readable API registry remains separate.

## Start here

- [Project overview](../README.md) - product scope, current implementation status, and limitations
- [Documentation policy](DOCUMENTATION.md) - OKF conventions, provenance, writing, validation, and copyright rules
- [Architecture](architecture/index.md) - system boundaries, design, implementation status, and decisions
- [Contributor guidance](contributor/index.md) - development setup and contribution constraints
- [Language and API reference](reference/index.md) - dialect- and platform-scoped reference material
- [Research](research/index.md) - evidence gathering and analysis of historical GFA BASIC sources
- [Testing](testing/index.md) - test strategy and original-interpreter reference procedures
- [User guidance](user/index.md) - supported commands, diagnostics, and current limitations
- [Templates](templates/index.md) - reusable documentation structures
- [Reference source register](reference-sources.md) - source identity, provenance, applicability, and evidence status

## Documentation principles

- Keep this `docs/` tree as the only canonical documentation bundle.
- Treat this index and nested indexes as navigation, not concept documents; reserved index files have no concept frontmatter.
- Keep technical claims scoped to the documented dialect, platform, and version.
- Distinguish verified source evidence from inference, proposal, and implementation status.
- Keep API registry data in its structured source files; do not duplicate its schema in prose.
- Publish website documentation from this source rather than maintaining duplicate website content.

See the [Google OKF v0.2 specification](https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md).
