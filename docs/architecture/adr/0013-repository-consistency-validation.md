# ADR 0013: Repository consistency validation

- Status: Accepted
- Date: 2026-11-06
- Decision makers: Project maintainers

## Context

The repository contains implementation code, architecture documents, research inventories, a roadmap, CI workflows, and a GitHub Pages site. These sources can drift apart. Validation must be cheap, deterministic, understandable, and must not become a second source of project truth.

## Decision

The initial tooling uses Python 3 standard library only, JSON rule files, a small explicit validator with a fixed set of check types, and standard-library unittest tests. No package installation is required. No YAML parser, Pydantic, pytest, Node.js, dynamic evaluation, shell execution from rules, arbitrary imports, or embedded executable code is permitted.

Each rule has a stable unique ID, description, enabled flag, severity, check type, check-specific parameters, and an evidence/source-of-truth reference. The rule file is the editable inventory; Python defines the finite set of safe check types. Tuning an existing rule must not require editing the validator.

Validation distinguishes evidence from inference. A passing consistency check is not proof that a language feature or compiler component is implemented. Unknown status must remain explicitly unknown.

The rule format and maintenance process are documented in [VALIDATION.md](../VALIDATION.md). New check types require a concrete use case, tests, and documentation.

## Alternatives considered

- YAML: more comfortable for some edits, but requires a third-party parser; JSON is available in the standard library.
- Pydantic/jsonschema: richer schema validation, but adds dependency and installation overhead for a small schema.
- pytest: richer test experience, but unittest is sufficient and dependency-free.
- General-purpose rule engine/plugins: unnecessary abstraction and execution complexity.
- Python-coded rules: flexible but makes routine edits require code changes and permits arbitrary execution.

## Consequences

- CI needs only Python 3; no pip installation or dependency cache.
- Rule edits are ordinary JSON changes, reviewed separately from validator code.
- The fixed check catalog intentionally limits expressiveness; new capabilities require code, tests, and docs.
- The generator and validator may share a project model later, but descriptive text must not silently become authoritative status.
- Checks remain deterministic, offline, and limited to explicitly selected repository files.

## Evidence and references

- Python standard library: json, argparse, pathlib, unittest.
- Existing repository architecture and evidence policy in [../README.md](../README.md).
- Follow-up validator: [#249](https://github.com/vsimko/gfabasic-rust/issues/249).
- Project data generator: [#250](https://github.com/vsimko/gfabasic-rust/issues/250).

## Supersedes / superseded by

None.
