# Documentation policy

## Decision and scope

**Decision:** Use Google's [Open Knowledge Format (OKF) v0.2](https://github.com/GoogleCloudPlatform/open-knowledge-format) for the project's agent-oriented knowledge base. The canonical bundle is [`knowledge/`](../knowledge/index.md).

OKF represents knowledge as UTF-8 Markdown concept documents with YAML frontmatter. A bundle uses directory hierarchy, standard Markdown links, and optional `index.md` and `log.md` files for progressive disclosure and change history. Each concept has a non-empty `type` field. The root `index.md` declares `okf_version: "0.2"`.

The repository also contains operational and user-facing documentation under `docs/`, root-level project instructions, and machine-readable registries. These are not automatically an OKF bundle merely because they use Markdown. The `knowledge/` bundle provides the canonical agent-facing map and typed concepts. Migrate existing long-form documents into the OKF bundle in reviewed increments; do not bulk-wrap files with empty metadata or duplicate their content.

## OKF authoring rules

- Follow the canonical [OKF v0.2 specification](https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md).
- Every concept document must start with YAML frontmatter containing a non-empty `type`.
- Use `title`, `description`, and `tags` when they improve discovery.
- Record `sources` for claims derived from identifiable materials. Use a resource path or URL and a clear title. Do not invent provenance, authors, verification, or freshness.
- Use `generated` and `verified` only when the actor and event are known. Preserve the distinction between machine-generated and human-reviewed content.
- Use `status` and `stale_after` only when their meaning and lifecycle are maintained.
- Keep one concept per document. Use standard Markdown links between related concepts.
- Use `index.md` in the root and in directories where it helps progressive disclosure. Index files contain no frontmatter except the root index's `okf_version`.
- Use `log.md` for a chronological change log only when it is maintained.
- Preserve unknown frontmatter keys when editing a concept. Do not reject unknown `type` values.
- Prefer short, direct English. Use structural headings, lists, tables, and code blocks.
- Keep canonical technical details in one place. Link to source code, tests, GitHub issues, and detailed documents rather than copying them.

## Bundle entry path

Agents must start at [`knowledge/index.md`](../knowledge/index.md), then follow the relevant area index and concept links. The root [README](../README.md) and [AGENTS.md](../AGENTS.md) point to this bundle and retain repository-operational instructions.

## Relationship to other documentation

- `knowledge/` is the OKF knowledge bundle and the preferred agent discovery surface.
- `docs/` contains detailed technical documents, research notes, contributor guidance, and existing indexes. It is being migrated incrementally; files without OKF frontmatter are not yet OKF concepts.
- `AGENTS.md` remains the GitHub coding-agent instruction file. It links to the knowledge bundle but is not itself an OKF concept.
- TOML, JSON, YAML configuration, API registries, and test fixtures remain in their domain-specific formats. Do not replace them with explanatory Markdown.

## Validation

The OKF v0.2 minimum conformance conditions are: every non-reserved Markdown document in the bundle has parseable YAML frontmatter; every frontmatter block has a non-empty `type`; and reserved `index.md` and `log.md` files follow the specification.

Validate the bundle in CI or with a deterministic local tool before claiming full conformance. Also review internal links, source provenance, concept boundaries, and whether indexes support progressive discovery. A valid frontmatter block does not prove that a claim is true or current.

## General writing and evidence rules

- Use UTF-8 Markdown and relative links for local material.
- Write technical documentation in clear, direct English.
- State the dialect, platform, and version for every GFA BASIC language/API claim.
- Cite source title, edition/version, platform, and printed page or section.
- Separate observed facts, documented claims, inference, assumptions, and unresolved questions.
- Do not claim implementation support because a manual describes a feature or because a registry entry exists.
- Do not copy substantial copyrighted manual text, source listings, scans, or OCR dumps.

## Copyright and source material

This project does not distribute historical GFA BASIC manuals or books, ROMs, disk images, or historical sample programs. Record bibliographic metadata, concise citations, original analysis, and independently authored tests. The project MIT license does not grant rights to third-party materials.
