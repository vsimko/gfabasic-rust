---
type: "Architecture Decision Record"
title: "ADR 0016: OKF v0.2 documentation bundle"
---

# ADR 0016: OKF v0.2 documentation bundle

- Status: Accepted
- Date: 2026-11-09
- Supersedes: The unresolved-OKF statement in ADR 0001

## Context

Issue #14 requires a single, machine-validatable documentation source with metadata, discoverable navigation, generated website pages, and CI/deployment integration. The issue identifies Google's [Open Knowledge Format specification v0.2](https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md). The earlier ADR 0001 treated OKF as unidentified; that statement is now obsolete. The project must preserve useful Markdown content and links, avoid a second source tree, and keep domain-specific machine-readable API declarations separate.

## Decision

1. Adopt Google Open Knowledge Format v0.2 for the canonical human-readable documentation bundle rooted at `docs/`.
2. Keep UTF-8 Markdown as the editable concept format. Every non-reserved Markdown concept has parseable YAML frontmatter and a non-empty string `type`. Use `title` for display and add optional metadata only when it is accurate and useful.
3. Make `docs/index.md` the sole bundle root index; its frontmatter declares `okf_version: "0.2"`. Use nested reserved `index.md` files for progressive-disclosure navigation. Reserved index files do not carry ordinary concept frontmatter. A reserved `log.md`, if introduced, is a newest-first chronological update log.
4. Preserve existing concept paths and content where practical. Do not create a parallel `knowledge/` tree, duplicate concept text in website sources, or scrape prose to generate API declarations. The structured API registry remains authoritative for API metadata.
5. Enforce the OKF conformance floor and repository navigation invariants with `scripts/validate_okf.py`: metadata parsing and shapes, reserved-file structure, local links and anchors, complete directory indexes, duplicate canonical paths, and reachability from the root index.
6. Generate the static documentation website from the canonical concepts using `scripts/generate_docs_site.py`. Disable raw HTML, rewrite internal links to generated pages, use concept metadata for titles, use index files for navigation order, and derive a short page summary from metadata or the concept body.
7. Treat concepts with `status: draft` as non-publishable. Publish `status: deprecated` concepts with a visible warning; an omitted status follows OKF's default of `stable`.
8. Keep generated HTML build-only under `site/docs/`. Run validation and generation in PR checks, PR previews, and main-branch production deployment. Documentation-only changes must trigger production deployment; previews are cleaned up when their PR closes. Production and preview smoke tests verify the generated documentation.

## Alternatives considered

- Markdown without a documented metadata/navigation contract: rejected because it does not provide the requested machine-readable bundle and metadata validation.
- A parallel `knowledge/` directory: rejected because it duplicates concepts and creates competing sources of truth.
- A separate CMS or hosted authoring service: rejected because contributors must be able to review and edit source offline in normal Git workflows.
- Generating API declarations from prose: rejected because the API registry has a separate schema and validation contract.

## Consequences

- The existing documentation remains readable as Markdown in GitHub and can be published as static HTML without requiring a hosted authoring service.
- New concepts must include frontmatter and be linked from their directory index; every concept must be reachable from `docs/index.md`.
- The validator is intentionally stricter than OKF's permissive minimum where needed for reliable repository navigation. Unknown extension fields remain accepted.
- Generated pages are reproducible build output and must never be edited directly.
- ADR 0001 remains historically accurate for its original decision date, but its claim that OKF is unidentified is superseded by this ADR.
