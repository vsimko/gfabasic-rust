---
type: "Documentation Policy"
title: "Documentation policy"
---

# Documentation policy

## Decision and scope

**Decision (2026-11-09):** Use Google's [Open Knowledge Format (OKF) v0.2](https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md) for the canonical documentation bundle rooted at `docs/`. Markdown remains the human-readable source format; each non-reserved Markdown concept has YAML frontmatter with a non-empty `type`. The root `docs/index.md` declares `okf_version: "0.2"`.

OKF v0.2 is intentionally permissive. Its conformance floor is parseable YAML frontmatter and a non-empty `type` for every concept, plus the prescribed structures for reserved `index.md` and `log.md` files. Unknown types and extension fields remain valid. This repository adds stricter project quality gates (navigation, links, metadata shapes where used, and deterministic website output) without misrepresenting those checks as requirements imposed by OKF itself.

The `docs/` tree is the only canonical human-readable documentation bundle. Do not create a parallel `knowledge/` directory or duplicate source content in website files. Keep the machine-readable API registry and generated API catalogue in their existing structured pipeline; OKF does not replace domain-specific schemas.

The project uses UTF-8 Markdown and GitHub-flavoured Markdown (GFM) features only where GitHub renders them consistently. Source files remain readable and useful without the generated website. The website is a generated publication of selected concepts, never an authoring location.

This policy applies to human-readable project documentation. Machine-readable API declarations and configuration data must use separately defined, validated formats. They must not be maintained by scraping explanatory Markdown.

## OKF v0.2 bundle

- `docs/index.md` is the bundle entry point and declares exactly `okf_version: "0.2"`. It is the only `index.md` allowed to contain frontmatter.
- `index.md` files are reserved directory listings and have no ordinary concept metadata. Keep their body as headings followed by links to direct concepts or subdirectories.
- `log.md` is reserved for a chronological update log with newest-first `## YYYY-MM-DD` headings. Do not use either reserved filename for a concept.
- Every other `.md` file under `docs/` is a concept with parseable YAML frontmatter and a non-empty string `type`. `title` is used for display; optional OKF metadata is added only when supported by evidence.
- Use descriptive, project-owned type names (for example `Architecture`, `Architecture Decision Record`, `Research Note`, `Reference`, `User Guide`, and `Contributor Guide`). Unknown types and additional metadata keys remain valid under OKF.
- Do not add `verified` or source-provenance claims without evidence. A manual citation does not prove interpreter execution; keep dialect/version and verification boundaries in the concept body.
- Use nested `index.md` files to provide progressive disclosure. Each index lists its directory's direct concepts and child directories; it does not copy concept content.
- Preserve useful paths and existing concept bodies. Prefer adding metadata and navigation over gratuitous renames. When concepts overlap, retain one canonical source and link to it rather than duplicating content.
- Keep `README.md` and `AGENTS.md` as short repository entry points that direct readers and agents to `docs/index.md`.

## Source format

- Encode text as UTF-8.
- Use Markdown files with the `.md` extension.
- Use GitHub-flavoured Markdown only for features supported by GitHub's renderer. Do not require a hosted documentation service or custom renderer to read essential project information.
- Use relative links for files in this repository.
- Keep generated documentation out of the canonical source unless a later decision defines a reproducible generation process.
- Keep source files readable in a plain-text editor.


## Generated website data

The website may consume generated JSON summaries for its static dashboard. Selected documentation pages must be rendered from the canonical OKF concepts in `docs/` rather than maintained as a second copy. Concepts with `status: draft` are not published; `status: deprecated` concepts remain available for old links and display a warning; an absent status follows OKF's default of `stable`. Generated data is not a canonical source of project status. Every generated value must be traceable to its canonical source, and the generation process must be deterministic and validated. Keep generated output separate from editable source data. Do not hand-edit generated files, scrape explanatory Markdown as a data source, or add credentials to browser code. Missing or stale values must remain explicit; never replace unknown values with zero or a guessed status. See [the implementation dashboard guide](architecture/IMPLEMENTATION-DASHBOARD.md) and [ADR 0014](architecture/adr/0014-implementation-dashboard.md).

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

Run the OKF validator and generate the website pages locally before review:\n\n```sh\npython -m pip install PyYAML==6.0.2 markdown-it-py==3.0.0\npython scripts/validate_okf.py\npython scripts/generate_docs_site.py\n```\n\nThe validator checks frontmatter, reserved files, supported metadata shapes, local links and anchors, complete directory indexes, and reachability from the root index. The generator writes build-only HTML pages to `site/docs/`; do not edit those generated files.\n\nFor every documentation change, review:

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
