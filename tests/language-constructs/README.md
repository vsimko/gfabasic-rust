# Language-construct manifests

This directory is a version-scoped catalogue of independently testable GFA BASIC constructs. Each construct has one stable kebab-case directory containing a `manifest.yaml` and, when applicable, the small `.LST` probes referenced by its per-version entries.

- [Manifest schema](schema-v1.json) is the normative structural contract.
- [Authoring and validation contract](../../docs/testing/language-construct-manifests.md) explains field semantics and the boundary with the existing conformance corpus.
- [`fact-function`](fact-function/manifest.yaml) demonstrates an expression/function.
- [`mat-one-statement`](mat-one-statement/manifest.yaml) demonstrates a statement.

These manifests are not the existing `tests/corpus/manifest.yaml`, and do not replace corpus sidecars. Manifest expectations and documentary evidence are not native execution results.
