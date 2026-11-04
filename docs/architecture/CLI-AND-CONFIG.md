# CLI, configuration, and build artifacts

- Status: **Accepted by [ADR 0005](adr/0005-cli-and-project-configuration.md).** The CLI, project loader, and artifact writer are not implemented.
- Tracking: [issue #9](https://github.com/vsimko/gfabasic-rust/issues/9).

## Command model

The executable name is \`gfabasic\`. The initial command surface is deliberately small:

| Command | Purpose | Output |
|---|---|---|
| \`gfabasic build [SOURCE]\` | Validate and compile a source entry point | Native executable and requested optional artifacts |
| \`gfabasic check [SOURCE]\` | Load, parse, and semantically validate without emitting a native executable | Diagnostics and optional compatibility report |
| \`gfabasic version [--verbose]\` | Report compiler version and build identity | Human-readable version/build metadata |

If SOURCE is omitted, the command uses the entry point in the discovered or explicitly supplied project file. If SOURCE is supplied, its parent directory is the starting point for project-file discovery. Multi-file source composition must follow the separately verified MERGE/include semantics; the CLI must not invent a project module/import model.

There is no implicit \`run\` command in the initial interface. Running generated programs is a separate runtime/deployment concern.

## Project file

Use TOML in a project file named \`gfabasic.toml\`. The project-file schema has its own integer \`schema-version\`, independent of the compiler product version and runtime ABI.

Illustrative schema (the dialect identifier and source version are intentionally placeholders):

\`\`\`toml
schema-version = 1

[project]
name = "example"
entry = "main.gfa"

[source]
dialect = "<explicit-dialect-id>"
version = "<explicit-dialect-version>"

[api]
profiles = ["amiga"]

[target]
triple = "x86_64-unknown-linux-gnu"

[build]
output-dir = "build"
artifact-name = "example"
emit = ["executable"]
\`\`\`

Initial fields:

- \`schema-version\`: required configuration schema version.
- \`project.name\`: optional human-readable project name.
- \`project.entry\`: source entry point, relative to the project root.
- \`source.dialect\` and \`source.version\`: required explicit source dialect identity.
- \`api.profiles\`: optional list of API profile identifiers; the default is an empty list.
- \`target.triple\`: optional LLVM target triple; the accepted initial default is \`x86_64-unknown-linux-gnu\`.
- \`build.output-dir\`: output directory, relative to the project root; default \`build\`.
- \`build.artifact-name\`: optional output basename; default is derived from the entry-point filename.
- \`build.emit\`: requested artifact kinds; default \`["executable"]\`.

The example is a policy-level schema, not an implemented parser or frozen serialization library contract. The exact TOML field types and schema validator belong to the implementation work under #12 and #55.

## Project discovery and paths

1. \`--project PATH\` selects the project file explicitly and disables automatic discovery.
2. Otherwise, start from the source file's parent directory when SOURCE is supplied, or the current working directory when it is omitted.
3. Search that directory and its parents for the nearest \`gfabasic.toml\`; stop at the filesystem root.
4. If no project file is found, use single-file mode. The source dialect and version must then be supplied explicitly on the CLI.
5. Resolve project-relative source and output paths against the directory containing \`gfabasic.toml\`, not the process working directory.
6. A missing, unreadable, malformed, or unsupported-version project file is an actionable configuration error.

A future \`--no-project\` option may explicitly disable discovery; it must not be combined with \`--project\`.

## CLI options and precedence

Initial options:

| Option | Meaning |
|---|---|
| \`--project PATH\` | Use the specified project file; no upward discovery |
| \`--dialect ID\` | Override source dialect identifier |
| \`--dialect-version VERSION\` | Override source dialect version |
| \`--api-profile ID\` | Add an API profile; repeatable |
| \`--no-api-profiles\` | Clear profiles from the project file |
| \`--target TRIPLE\` | Override the output target |
| \`--out-dir PATH\` | Override the output directory |
| \`--emit KIND\` | Request an optional artifact kind; repeatable |
| \`--diagnostic-format human\|json\` | Select human or machine-readable diagnostics |
| \`--report-json PATH\` | Write a machine-readable compatibility/support report |
| \`--verbose\`, \`--quiet\` | Adjust non-diagnostic progress output |
| \`--help\` | Show command and option help |
| \`--version\`, \`--version --verbose\` | Show product version; verbose mode includes build identity |

Precedence is:

1. Explicit command-line option.
2. Project-file value.
3. Documented default, only where a default is safe.

The dialect identifier and version are mandatory. The compiler must not infer them from file extensions, source contents, API profile, or output target. API profiles default to none. The target defaults to the accepted initial target unless explicitly overridden.

A repeated \`--api-profile\` adds profiles to the selected set; \`--no-api-profiles\` clears the project-file set. Duplicate identifiers are normalized to one entry. Conflicting declarations across selected profiles are semantic diagnostics, not load-order overrides.

Unknown options, unknown project keys, invalid values, incompatible schema versions, and contradictory settings must fail with actionable diagnostics. Do not silently ignore configuration.

## Build artifact model

The default output directory is \`build/\` under the project root. Place target-specific artifacts under \`build/<target-triple>/\` so builds for different targets do not overwrite each other.

The initial artifact kind is \`executable\`. Optional kinds may include object files, LLVM IR, debug information, and a compatibility report. Each kind must be implemented and documented before it is advertised by \`--emit\`.

Every successful build should produce a manifest containing compiler product version, source revision, selected dialect/version, API profiles, target triple, relevant schema versions, runtime ABI identity when defined, and toolchain/dependency versions. Do not embed machine-local absolute paths or nondeterministic timestamps into outputs by default.

The compiler must write outputs atomically where practical. A failed build must not leave a partially written executable presented as successful. Diagnostics go to stderr; machine-readable reports are written to the requested path or stdout only when explicitly requested.

## Exit status

| Code | Meaning |
|---:|---|
| 0 | Command completed successfully |
| 1 | Source diagnostics prevent the requested operation |
| 2 | CLI usage, project configuration, or schema validation error |
| 3 | Compiler backend, LLVM, or target/toolchain failure |
| 4 | File-system or environment failure |

The same exit code must not be used for success and failure. Human-readable diagnostic wording may evolve; the exit-status categories are stable public CLI behavior once implemented.

## Determinism and safety

- The same source, project configuration, compiler version, and declared toolchain inputs must produce deterministic diagnostics and semantically equivalent artifacts.
- Use locked dependencies and record relevant toolchain/native dependency versions.
- Normalize timestamps and paths where supported; reproducible binary identity is governed by the release policy.
- Do not execute arbitrary commands from project configuration.
- Do not require network access or an LLM to compile source.
- Do not let project configuration silently enable unverified or unsupported APIs.
- Keep machine-readable diagnostics/report schemas independently versioned.

## Deferred implementation

The CLI executable, TOML parser/schema validator, project discovery, source composition, diagnostic renderer, compatibility report, native output, and artifact manifest are not implemented. This specification accepts the interface model; it does not claim any command is currently usable.
