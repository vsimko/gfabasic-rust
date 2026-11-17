---
name: corpus-fixture-authoring
description: Add or revise independently authored GFA BASIC conformance-corpus fixtures, TOML sidecars, manifest entries, and API crosswalks.
---

# GFA BASIC corpus fixture authoring

Use this skill when creating or changing source fixtures in `tests/corpus/`. The corpus is an evidence-gated source corpus, not a general executable conformance suite.

## Required context

1. Read `AGENTS.md`.
2. Read [the corpus contract](../../../tests/corpus/README.md).
3. Read [compatibility testing and release criteria](../../../docs/architecture/TESTING.md).
4. Read the relevant dialect/version reference and source-register entry.
5. Inspect nearby fixtures and sidecars for the same dialect, version, area, and test kind.

## Procedure

### 1. Establish the behavior under test

- Define one focused language/API behavior and its expected result.
- Identify exact dialect, platform, release, and API profile(s). Never infer applicability from another BASIC dialect or release.
- Locate the primary source and printed page/section, or a reproducible observation on a named trusted interpreter.
- Check existing fixtures, issue links, registry declarations, and crosswalks to avoid duplication.
- Classify the test as positive, negative, or semantic. Negative tests require an explicit source rule or trusted compatibility observation; absence from an inventory is not proof of invalidity.

### 2. Create an independent minimal fixture

- Use the existing corpus layout: `<dialect>/<version>/<area>/<case>.lst` plus a matching `.toml` sidecar.
- Choose a stable lowercase kebab-case ID and use it consistently in the corpus manifest and sidecar.
- Keep the program focused, self-contained, deterministic, and free of machine-specific paths, network access, locale assumptions, and external assets.
- Author original source. Do not copy historical listings or substantial manual examples.
- Include only setup necessary to isolate the behavior; avoid unrelated commands that obscure the tested contract.
- Do not make expected output by invoking the implementation under test. Expected behavior must have an independent source of truth.

### 3. Write complete metadata

Follow the current corpus sidecar schema and neighboring examples. Record at least:

- stable `id`;
- exact `dialect` and `version`;
- `api_profiles` (empty for language-core fixtures);
- `kind`: `positive`, `negative`, or `semantic`;
- `status`: normally `candidate` until evidence review is complete;
- `source_key`, printed `source_pages`, and `evidence_status`;
- `expectation`, including expected parse/compile/runtime outcome and stdout/stderr/exit where applicable;
- concise `notes` for unresolved syntax, OCR, edition, or runtime questions.

Do not use `verified` merely because the fixture parses, formats, or passes metadata validation. A fixture can be marked verified only when its syntax and expected behavior have been checked against the cited page image or named trusted interpreter, within the stated scope.

### 4. Register and cross-link

- Add the fixture path (without `.lst` or `.toml`) to `tests/corpus/manifest.toml`.
- Update any relevant declaration-to-fixture crosswalk or research index.
- Add registry `support.tests` links only when the fixture actually substantiates implementation/conformance claims under the registry policy.
- Keep source-evidence links and executable test links distinct.

### 5. Validate

Run from the repository root:

```sh
python scripts/validate_corpus.py
```

The corpus validator checks metadata and paths; it does not compile or execute fixtures. A formatter acceptance check is not interpreter conformance.

## Boundary with construct-scoped native probes

The new native interpreter catalogue is separate from this corpus:
- native probes live under `tests/language-constructs/<construct-id>/` and use YAML `manifest.yaml` files;
- the existing corpus remains under `tests/corpus/` with TOML sidecars and `tests/corpus/manifest.toml`;
- do not migrate corpus files or metadata to YAML as part of the construct-catalogue work;
- use [ADR 0018](../../../docs/architecture/adr/0018-construct-scoped-amiga-gfa-basic-probes.md) for the native probe architecture.

## Review checklist

- [ ] Fixture is original, minimal, deterministic, and self-contained.
- [ ] Platform, dialect, version, and API profile are explicit.
- [ ] Source and printed-page/section locators support the expectation.
- [ ] Sidecar and manifest IDs/paths agree.
- [ ] Status reflects actual evidence quality.
- [ ] Registry crosswalk does not imply unsupported conformance.
- [ ] Validator and relevant tests pass.
- [ ] No copyrighted listings or external assets were added.
