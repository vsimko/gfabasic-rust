# CLI, configuration, and build artifacts

- Status: Proposed outline; interface remains open under issue #9.

The CLI should support a single source file and, after source composition semantics are verified, multi-file projects. Configuration must permit explicit dialect/version, API profile set, and output target selection. The compiler must not infer profile or target in a way that silently overrides explicit user settings.

Decide the project-file format, schema version, discovery rules, explicit config option, precedence between CLI and configuration, and validation behavior. Invalid or conflicting settings must produce actionable diagnostics.

Native executable output is the primary planned artifact. Object files, intermediate representations, debug data, and compatibility reports may be optional outputs.

The same source, configuration, compiler version, and relevant toolchain inputs should produce deterministic diagnostics and semantically equivalent artifacts. Record metadata needed to reproduce a build. Avoid timestamps or machine-local paths unless explicitly requested.

Define stable exit-status categories for success, source diagnostics, configuration errors, and toolchain/backend failures.

Tracking: [issue #9](https://github.com/vsimko/gfabasic-rust/issues/9).
