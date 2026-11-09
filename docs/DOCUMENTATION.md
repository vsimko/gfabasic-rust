---
type: Documentation Policy
title: Documentation policy
tags: [documentation, okf, writing, provenance]
---

# Documentation policy

## Decision and scope

**Decision (2026-11-09):** Use **Google Open Knowledge Format (OKF) v0.2** as the canonical structure and metadata convention for the human-readable documentation bundle rooted at `docs/`. The authoritative specification is [Google's Open Knowledge Format v0.2 specification](https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md).

OKF v0.2 is identifiable and specified. It is a deliberately minimal, human- and agent-readable knowledge-bundle format: a directory hierarchy of UTF-8 Markdown concept files with YAML frontmatter. Each non-reserved concept file requires a non-empty `type` field. `index.md` and `log.md` are reserved filenames with their own structures; a bundle-root `index.md` may declare `okf_version: "0.2"`. The specification defines optional provenance, trust, lifecycle, and attestation metadata while allowing producer-defined types and extension fields.

The repository's issue [#14](https://github.com/vsimko/gfabasic-rust/issues/14) explicitly selects Google OKF v0.2 and defines the migration, validation, website-generation, and CI requirements. The earlier statement that OKF could not be identified was incorrect and is superseded by this decision.

Markdown remains the human-readable body syntax, but Markdown alone is not the complete bundle convention: OKF adds required concept frontmatter, reserved index/log semantics, and defined metadata conventions. Do not claim that the repository is fully OKF-conformant until the complete bundle has been migrated and validated against the v0.2 conformance requirements.

OKF does not define a fixed taxonomy, schema registry, required toolchain, or website generator. It does not replace domain-specific machine-readable schemas such as the project's API registry. Keep API declarations and configuration data in their existing structured source formats; document or link to them from OKF concepts rather than duplicating their schemas as prose.

This policy applies to the canonical documentation bundle in `docs/`. Keep its editable source in Git. The website is a generated publication of selected canonical concepts, not a second authoring location. Do not create a parallel top-level `knowledge/` tree or hand-maintained duplicate website content.

## Source format

- Encode text as UTF-8.
- Use Markdown files with the `.md` extension.
- Use GitHub-flavoured Markdown only for features supported by GitHub's renderer. Do not require a hosted documentation service or custom renderer to read essential project information.
- Use relative links for files in this repository.
- Keep generated documentation out of the canonical source unless a later decision defines a reproducible generation process.
- Keep source files readable in a plain-text editor.


## Generated website data

The website may consume generated JSON summaries for its static dashboard. Generated data is not a canonical source of project status. Every generated value must be traceable to its canonical source, and the generation process must be deterministic and validated. Keep generated output separate from editable source data. Do not hand-edit generated files, scrape explanatory Markdown as a data source, or add credentials to browser code. Missing or stale values must remain explicit; never replace unknown values with zero or a guessed status. See [the implementation dashboard guide](architecture/IMPLEMENTATION-DASHBOARD.md) and [ADR 0014](architecture/adr/0014-implementation-dashboard.md).

## Writing rules

Write technical documentation in clear, direct English. Use ASD-STE100 Simplified Technical English principles as a style guide where applicable:

- Use short sentences with one main idea.
- Prefer active voice and direct instructions.
- Use consistent terms. Define technical terms before use.
- Expand abbreviations at first use.
- Use numbered steps for procedures.
- State prerequisites, expected results, and failure conditions.
- Use the same term for the same concept throughout the repository.
- Avoid vague claims such as “fully compatible”, “complete”, or “works everywhere”.
- Separate verified facts from assumptions and proposals.
- Do not copy substantial text from copyrighted manuals.

This project has not established a licensed ASD-STE100 dictionary or an automated checker. Do not claim formal ASD-STE100 certification or full compliance. Human review is required. The rules above are project writing guidance, not a substitute for the complete standard.

## Document structure and navigation

- Use one H1 heading per document. The H1 must identify the document topic.
- Use heading levels in sequence. Do not select a heading level only to change its visual size.
- Give each document a clear audience and purpose.
- Link to a canonical definition instead of duplicating it in multiple documents.
- Use descriptive link text. Avoid phrases such as “click here”.
- Use tables for compact, genuinely tabular information. Use lists for procedures and unrelated items.
- Mark code blocks with a language identifier when known.
- State the dialect, platform, and version when documenting GFA BASIC behavior.
- Cite the source document, edition/version, platform, and printed page or section for each documented language/API rule.
- Mark unresolved behavior explicitly. Do not fill evidence gaps with rules from another BASIC dialect.

## Examples and technical claims

- Prefer small, independently authored examples.
- Identify the dialect/version and required API profile for each conformance example.
- Distinguish syntax demonstrated by an example from semantics established by a primary reference.
- Do not claim that a documented feature is implemented. State its implementation and test status separately.
- Do not include copyrighted source code or substantial manual extracts unless redistribution rights are confirmed.

## Validation

For every documentation change, review:

1. Markdown rendering on GitHub.
2. Local links and referenced file paths.
3. Heading hierarchy and document navigation.
4. Table readability and code-block language identifiers.
5. Spelling, terminology, and clarity.
6. Source citations, dialect/version scope, and evidence status for GFA BASIC claims.
7. Copyright and redistribution constraints for external material.

The GitHub Actions workflow runs structural Markdown linting and offline local-link/anchor validation. Heading hierarchy, table readability, source correctness, spelling/terminology, dialect/version scope, copyright review, and semantic/API-reference accuracy remain human-review responsibilities. Do not claim that the automated tools certify ASD-STE100 compliance or GFA BASIC correctness.

## Spelling and terminology

The current CI does not run an automatic spell-checker or automatic correction tool. Markdown linting is structural and does not rewrite source files. Spelling, terminology, and GFA BASIC identifiers remain subject to human review.

This is intentional until the project has a reviewed terminology dictionary. If a spell-checker is introduced, configure explicit terms for dialect names, keywords, built-in identifiers, API names, file extensions, and platform terminology. The checker must report candidates only; it must never auto-correct source text or silently rewrite GFA BASIC identifiers. Review false positives and dictionary changes as code.

## Automated checks

The repository runs the following documentation checks in GitHub Actions for documentation-related pull requests and pushes to `main`:

- `markdownlint-cli2` (pinned to 0.24.0) checks selected structural Markdown rules: heading levels, heading style, trailing whitespace, tabs, spacing around headings/lists/fenced code, fenced-code language identifiers, and final newlines.
- `lychee` (pinned to 0.24.2) checks local Markdown file links and anchors in offline mode. It does not test whether external websites are reachable.

Run the same checks locally from the repository root:

```sh
npx --yes markdownlint-cli2@0.24.0 "**/*.md" "#node_modules"
```

Install lychee 0.24.2 and run:

```sh
lychee --offline --include-fragments anchor-only --root-dir "$PWD" "**/*.md"
```

These tools do not certify ASD-STE100 compliance, validate the correctness of GFA BASIC semantics, or check external URL availability. Human review remains required. The accepted machine-readable API registry is validated by the API-registry job; generated-reference consistency and runtime-handler verification remain limited by the fact that the registry generator and compiler/runtime are not yet implemented.

## Copyright and source material

**Distribution policy:** This project does not distribute historical GFA BASIC manuals or books, ROMs, disk images, or historical sample programs. Treat these items as external research sources only. Do not commit them to the repository, include them in release archives, or make them prerequisites for building, testing, or using the project.

Research may refer to materials that are lawfully accessible to the researcher. Record bibliographic metadata, provenance, platform/version applicability, and concise page/section citations in [reference-sources.md](reference-sources.md). Write original explanations and independently authored conformance fixtures. Do not copy substantial manual text, source listings, scans, OCR dumps, ROM contents, or disk images.

The project MIT license covers only project-owned source code and original documentation. It does not grant rights to third-party reference materials. Any future proposal to distribute such material would require a separate explicit project-owner decision and a documented rights basis before inclusion.
