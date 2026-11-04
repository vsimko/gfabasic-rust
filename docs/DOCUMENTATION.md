# Documentation policy

## Decision and scope

**Decision (2026-11-03):** Use UTF-8 Markdown as the canonical source format for human-readable project documentation. Use GitHub-flavoured Markdown (GFM) features only where GitHub renders them consistently. Keep all documentation in this repository.

Issue #14 refers to an “OKF format”, but the issue does not identify an expanded name, publisher, specification, version, or conformance profile. The repository contains no evidence that “OKF” names a specific documentation format. This is an unresolved requirement, not a format that can be implemented or validated as written.

Do not describe Markdown as “OKF”, and do not claim compliance with an unidentified specification. If the requester supplies the intended expansion or a specification, review it in a separate decision before changing the canonical format.

Markdown is the practical choice for the current project because it is plain text, open, readable in GitHub, accessible to contributors without a special authoring tool, and suitable for normal Git diffs and reviews. This decision does not prevent the project from generating other formats from the Markdown source in the future.

This policy applies to human-readable project documentation. Machine-readable API declarations and configuration data must use separately defined, validated formats. They must not be maintained by scraping explanatory Markdown.

## Source format

- Encode text as UTF-8.
- Use Markdown files with the `.md` extension.
- Use GitHub-flavoured Markdown only for features supported by GitHub's renderer. Do not require a hosted documentation service or custom renderer to read essential project information.
- Use relative links for files in this repository.
- Keep generated documentation out of the canonical source unless a later decision defines a reproducible generation process.
- Keep source files readable in a plain-text editor.

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
