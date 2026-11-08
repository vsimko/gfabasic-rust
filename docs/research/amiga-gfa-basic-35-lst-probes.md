# Amiga GFA BASIC 3.5 interpreter probes from text listings

- Tracking: [issue #66](https://github.com/vsimko/gfabasic-rust/issues/66).
- Related corpus: [issue #42](https://github.com/vsimko/gfabasic-rust/issues/42).
- Purpose: validate syntax and behavior of independently authored textual `.LST` programs with the original interpreter.
- Input under test: ASCII text listing (`.LST`), not the tokenized native `.GFA` program format.

## Why the probe path uses .LST

The supplied interpreter manual describes **Merge (F2)** as accepting data in ASCII format with the `.LST` extension, and **Save,A (Shift+F2)** as saving the current program in ASCII format. This is sufficient to select text listings as the initial probe interchange format.

That statement does not specify the encoding of bytes above ASCII, BOM handling, or line-ending variants. Initial probes therefore use plain ASCII characters only. Do not infer UTF-8, ISO-8859-1, Amiga code-page behavior, or a complete byte-level file contract from the manual's “ASCII” wording.

The `.GFA` native program format is a separate concern. It is not required for testing command syntax or runtime behavior through the editor's text-listing workflow.

## Manual-driven procedure

1. Start the legally usable Amiga environment and record the exact interpreter/editor version shown by the software.
2. Open the editor and use **Merge (F2)** to load one probe `.LST` file.
3. Inspect the merged program in the editor. Record any syntax rejection, rewriting, abbreviation expansion, or other editor-side effect separately from runtime behavior.
4. Run the program using the interpreter's normal execution command.
5. Record the exact visible output, error text, exit/stop behavior, and side effects.
6. Repeat from a clean program state. Do not combine multiple probes in one listing.
7. Save the observation record with interpreter version, environment, listing SHA-256, manual page reference, and actual result.

Use **Save,A (Shift+F2)** only as an optional round-trip check: save the loaded program as ASCII and compare the resulting listing with the input. A difference is an observation about editor serialization, not proof that the original input was invalid.

## Initial probe

Start with [`fact-zero.LST`](../../tests/interpreter-probes/amiga-3.5/fact-zero.LST) and its [observation metadata](../../tests/interpreter-probes/amiga-3.5/fact-zero.toml). It exercises the 3.5 addendum's documented `FACT` example at the zero boundary. The expected result is manual-backed; it is **not** an interpreter observation until the procedure above has actually been run on a version-identified interpreter.

## Observation record

For every run, record the following fields:

- `probe_id`, `dialect`, `version_target`, and `component`
- `actual_component_version` and `environment_record`
- `input_format` and `listing_sha256`
- `loaded_successfully` and `editor_rewrite`
- `execution_result`, `observed_output`, and `observed_errors`
- `repeatability`, `evidence_status`, and `observation_status`

Keep manual expectation, editor acceptance, and runtime observation as separate fields. Never promote a manual-backed fixture to interpreter-observed merely because it parses in the Rust frontend.

## Boundaries

- No copyrighted interpreter, ROM, disk image, manual page image, or historical sample listing belongs in the repository.
- Keep each probe original, minimal, and focused on one behavior.
- Do not use a 3.52 result as evidence for the normative 3.5 target. Record it separately if used for exploratory testing.
- Do not infer invalid syntax from a failed load until environment, version, and editor diagnostics are recorded.
- Do not make `.GFA` token decoding a prerequisite for this work.

