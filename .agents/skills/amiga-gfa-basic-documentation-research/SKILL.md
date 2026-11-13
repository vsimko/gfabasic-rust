---
name: amiga-gfa-basic-documentation-research
description: Research Amiga GFA BASIC language and API behavior from historical manuals, books, OCR, and interpreter observations. Use when extracting commands, functions, syntax, semantics, version applicability, source provenance, or documenting evidence gaps.
---

# Amiga GFA BASIC documentation research

Use this skill for source-led research into Amiga GFA BASIC. The output is evidence and original analysis, not an automatic language specification or implementation claim.

## Required context

1. Read `AGENTS.md`.
2. Read [research policy and source boundaries](../../../docs/research/README.md).
3. Read [the reference source register](../../../docs/reference-sources.md).
4. Read the relevant source/topic index, command catalogue, version-specific reference, and existing research notes before starting new extraction.
5. For documentation conventions, follow [the documentation policy](../../../docs/DOCUMENTATION.md).

## Procedure

### 1. Define the research question

- State the construct/API family, target platform, exact dialect/version, and question to resolve.
- Identify which component is being described (editor, interpreter, compiler, linker, or runtime library). Do not conflate similarly labelled products.
- Search existing research and registry entries first; extend a canonical note instead of creating duplicate inventories.
- Record what evidence would resolve the question and what remains outside scope.

### 2. Establish source provenance

For every source, record the source-register key, title, author/publisher where known, edition/date, platform, claimed version scope, and lawful access/provenance. Use printed page numbers and section headings, not PDF viewer indices alone.

Prioritize evidence appropriately:

1. Applicable primary manual/addendum page image.
2. Reproducible observation on a named original reference interpreter.
3. Independent contemporaneous technical source, with its secondary status explicit.
4. OCR, indexes, archive catalogues, FAQs, and source-code sightings as discovery aids or limited supporting evidence.

OCR text does not prove punctuation, signatures, tables, or semantics when the page image has not been checked. Do not infer unseen page content from OCR.

### 3. Extract and reconcile evidence

- Preserve exact source terminology and statement spelling when quoting short identifiers.
- Capture syntax forms, context, argument order, optional/repeated parameters, types, return values, side effects, errors, resource lifecycle, and examples where the source supports them.
- Record the printed page/section for each claim.
- Separate direct source statements, runtime observations, interpretation/inference, and unknowns.
- Compare overlapping sources and record disagreements explicitly. Do not silently reconcile conflicting values or choose the more convenient source.
- Keep Amiga and Atari ST evidence separate.
- Keep 3.0, 3.03, 3.5, 3.5U, 3.51, and 3.52 labels distinct. Do not infer interpreter version from an archive label or compiler version.
- Mark candidate/version applicability as uncertain until supported by an applicable source or named runtime observation.

### 4. Write the research artifact

- Update the existing canonical research note, inventory, TOML manifest, or source register when possible.
- Use the repository's documentation frontmatter, naming, navigation, and citation conventions.
- Include a concise scope statement, source/edition/version, printed-page locators, evidence status, disagreements, limitations, and next verification step.
- Keep machine-readable facts in their canonical structured registry/manifest. Do not create a parallel data source by scraping prose.
- If the claim is ready for an API declaration, link the declaration and follow the `api-evidence-verification` skill.

### 5. Review and validate

- Re-open cited page images for OCR-sensitive details and confirm each locator.
- Check that every claim is supported at the stated level and that version/platform scope is precise.
- Search for counter-evidence and relevant existing notes.
- Check local links, anchors, and navigation indexes.
- Run applicable validators, for example:

```sh
python scripts/validate_api_registry.py
python scripts/validate_corpus.py
python scripts/validate_okf.py
```

Run only validators relevant to the files changed; consult their documented commands and current workflow.

## Boundaries

- Never commit or redistribute historical manuals, books, scans, OCR dumps, ROMs, disk images, or copied historical sample programs.
- Retain bibliographic metadata, concise citations, lawful provenance, original analysis, and independently authored fixtures only.
- Do not claim that documentation proves implementation, Linux support, or interpreter execution.
- Do not claim completeness from an index, catalogue, partial chapter extraction, or OCR-only review.
- When evidence is missing or conflicting, state that explicitly instead of filling the gap from general BASIC knowledge.
