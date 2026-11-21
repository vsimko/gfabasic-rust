# Language-construct probe catalogue

This directory is the home for native, construct-scoped GFA-BASIC reference-interpreter probes.

Each independently testable language construct will have a directory:

```text
tests/language-constructs/
  <construct-id>/
    manifest.yaml
    probe.LST
    README.md       # optional
```

Use YAML for the new `manifest.yaml` files. Do not change the existing corpus YAML sidecars or manifest as part of this catalogue. The API registry remains TOML; broader corpus-format migration is tracked separately in [#574](https://github.com/vsimko/gfabasic-rust/issues/574).

The manifest schema, required metadata, examples, and validator are being specified in [issue #570](https://github.com/vsimko/gfabasic-rust/issues/570). Until that contract is implemented, this README is an architectural overview, not a schema specification. Change selection is tracked in [#571](https://github.com/vsimko/gfabasic-rust/issues/571), and parallel native construct/version CI in [#572](https://github.com/vsimko/gfabasic-rust/issues/572).

Keep source evidence, version-specific expectations, observed interpreter results, and Rust implementation status separate. Exact interpreter versions are not interchangeable. Missing native execution is `UNTESTED`, never PASS.

See [ADR 0018](../../docs/architecture/adr/0018-construct-scoped-amiga-gfa-basic-probes.md) and the [probe-writing guide](../../docs/testing/amiga-gfa-basic-probes.md).
