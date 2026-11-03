# Documentation policy

## Source format

Use UTF-8 Markdown for human-readable documentation. Keep documentation in the repository so that it remains readable in GitHub and can be reviewed with normal Git diffs.

Issue #14 refers to an “OKF format”. That name is ambiguous in the available project context. Do not label Markdown as OKF and do not claim compliance with an unidentified specification. Resolve the exact requested format, version, authoritative specification, tooling, and licensing requirements before making such a claim.

Machine-readable API declarations, when designed, must be kept in a validated data format and treated as a separate source of truth from explanatory prose.

## Writing rules

Write technical documentation in clear, direct English. Use ASD-STE100 Simplified Technical English principles as a style guide where applicable:

- Use short sentences with one main idea.
- Prefer active voice and direct instructions.
- Use consistent terms. Define technical terms before use.
- Expand abbreviations at first use.
- Use numbered steps for procedures.
- State prerequisites, expected results, and failure conditions.
- Do not use vague claims such as “fully compatible”, “complete”, or “works everywhere”.
- Separate verified facts from assumptions and proposals.
- Do not copy substantial text from copyrighted manuals.

This project has not yet established a licensed ASD-STE100 dictionary or automated checker. Do not claim formal ASD-STE100 certification. Human review is required.

## Structure and links

- Use one H1 heading per document.
- Use sequential heading levels.
- Use relative links for files in this repository.
- Use descriptive link text.
- Mark code blocks with a language identifier when known.
- Include source citations for GFA BASIC behavior. Cite the document, edition/version, platform, and page/section.
- Mark unresolved behavior explicitly. Do not fill gaps with rules from another BASIC dialect.

## Validation

At minimum, review Markdown rendering, local links, headings, table readability, spelling, and code examples. Automated checks may be added after the CI and tooling architecture is established.

## Copyright and source material

Do not commit scans, substantial manual extracts, or copied examples unless redistribution rights are confirmed. Prefer original explanations with short citations to source pages. Keep bibliographic metadata and provenance in [reference-sources.md](reference-sources.md).
