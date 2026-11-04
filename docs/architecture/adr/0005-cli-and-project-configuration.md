# ADR 0005: CLI and project configuration

- Status: Accepted
- Date: 2026-11-04

## Context

The project needs an explicit and reproducible way to select source dialect/version, API profiles, and output target. Configuration discovery and precedence must not silently change language semantics. The CLI must also distinguish source diagnostics from configuration and toolchain failures.

## Decision

1. Name the executable \`gfabasic\` and use the initial commands \`build\`, \`check\`, and \`version\`.
2. Use a TOML project file named \`gfabasic.toml\`, with an independent integer schema version.
3. Discover the nearest project file upward from the source parent directory, or from the current directory when no source is supplied. An explicit \`--project PATH\` disables discovery.
4. In single-file mode without a project file, require explicit dialect identifier and version on the command line.
5. Resolve settings by command line > project file > safe documented default. Dialect and version have no implicit default; API profiles default to none; target defaults to the accepted initial Linux target.
6. Keep dialect/version, API profile set, and output target independent. Never infer dialect or profile from file extension, source text, or target.
7. Use project-relative paths resolved from the project file directory.
8. Use \`build/<target-triple>/\` as the default target-specific output directory. The initial output is a native executable; optional artifacts are exposed only after implementation.
9. Define stable exit-status categories for success, source diagnostics, configuration/usage errors, backend/toolchain failures, and filesystem/environment failures.
10. Write diagnostics to stderr. Write machine-readable reports only when explicitly requested. Do not execute arbitrary commands from project configuration or require network/LLM services.

The detailed policy is maintained in [CLI-AND-CONFIG.md](../CLI-AND-CONFIG.md). Exact schema validation and implementation are separate work.

## Alternatives considered

- JSON or YAML project files: rejected for the initial project configuration in favor of TOML, which is already familiar in Rust tooling and supports comments.
- Implicit dialect inference from extension/content: rejected because it can silently select incorrect historical semantics.
- Implicit API profile from output target: rejected because source API semantics and output target are independent.
- Configuration precedence in which project files override CLI options: rejected because explicit command-line options must remain the highest-precedence override.
- One output directory shared by all targets: rejected because it risks cross-target artifact collisions.
- Running project-defined shell commands during compilation: rejected because configuration must not become an arbitrary code-execution mechanism.

## Consequences

- CLI and configuration must preserve the accepted dialect/profile/target separation from ADR 0004 and the target identity from ADR 0003.
- The project-file schema must be independently versioned and validated.
- Source dialect/version selection is mandatory and explicit.
- The initial target has a documented default, but cross-compilation is not implied.
- The accepted interface is not implemented; the workspace, parser, CLI, diagnostics, and artifact writer remain future work.

## References

- [CLI and configuration specification](../CLI-AND-CONFIG.md)
- [API profile model](../API-PROFILES.md)
- [Target model](../TARGETS.md)
- [Versioning policy](../VERSIONING.md)
- [Issue #9](https://github.com/vsimko/gfabasic-rust/issues/9)
