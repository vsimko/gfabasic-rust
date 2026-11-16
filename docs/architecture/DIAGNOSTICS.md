---
type: "Architecture"
title: "Diagnostics and compatibility reporting"
---

# Diagnostics and compatibility reporting

- Status: **The source-map diagnostic model and deterministic human renderer are implemented and unit-tested.** The narrow FACT/PRINT frontend emits lexical, syntax, and unsupported-feature diagnostics with source spans; the prototype CLI renders these errors. Stable diagnostic-code allocation, machine-readable schema, general semantic diagnostics, and runtime reporting remain unimplemented.
- Tracking: [issue #26](https://github.com/vsimko/gfabasic-rust/issues/26).
- Related design: [dialect policy](DIALECTS.md), [API profiles](API-PROFILES.md), [API registry](API-REGISTRY.md), [CLI/configuration](CLI-AND-CONFIG.md), [versioning policy](VERSIONING.md), and [source manager](https://github.com/vsimko/gfabasic-rust/issues/45).

This document defines the proposed user-facing contract. The compiler library has a base `Diagnostic` model and deterministic source-map renderer. The narrow FACT/PRINT frontend emits lexical, syntax, and unsupported-feature diagnostics; the prototype build path renders them through the source map. There is no stable code registry or machine-readable schema, and general semantic/runtime stages do not yet emit diagnostics.

## 1. Design requirements

Diagnostics must help a user identify the construct or operation, understand why it cannot be accepted for the selected compilation, and find a safe next step.

- Never silently ignore source text, syntax, statements, expressions, declarations, or runtime operations that the selected compiler cannot handle.
- Distinguish invalid source from valid-but-unsupported source, configuration/profile conflicts, target limitations, toolchain failures, and failures during execution.
- Identify the exact source dialect/version, selected API profiles, output target, and relevant compiler/runtime version where those values are known.
- Preserve source provenance through compiler stages so later diagnostics can point back to the original input.
- Keep diagnostics deterministic for the same source and configuration. Do not include machine-local absolute paths or unstable timestamps by default.
- Do not claim compatibility from a successful parse, a registry entry, or an implementation that lacks the required evidence and tests.

## 2. Diagnostic severity and category

Severity describes the impact on the current operation. Category describes the reason. They are separate fields.

| Severity | Meaning | Default effect |
|---|---|---|
| `error` | The compiler cannot safely continue this compilation stage or cannot produce a result that satisfies the selected contract. | Compilation fails. |
| `warning` | The operation can continue, but the user should review a documented compatibility, portability, deprecation, or behavior risk. | Compilation may continue; the warning must not conceal a known semantic gap. |
| `note` | Supporting information attached to a diagnostic or report. | Informational only. |

A hint or suggested action is supplemental text, not a severity. Do not use a warning where the selected source construct has no valid implementation: unsupported or unverified behavior must be an error rather than a warning that allows a misleading executable.

Use a category that identifies the failing boundary:

| Category | Use |
|---|---|
| `source` | Source loading, file identity, or source decoding failure. |
| `lexical` | A character sequence cannot be tokenized under the selected dialect/version. |
| `syntax` | The token sequence does not form a valid construct in the selected dialect/version. |
| `semantic` | Name resolution, typing, declaration, or other language rule is violated. |
| `unsupported` | The source construct is recognized or identified, but the compiler does not implement the required behavior. |
| `profile` | An API declaration is unavailable, ambiguous, or conflicting for the selected profile set. |
| `target` | The requested semantics are known, but the selected output target cannot provide them. |
| `configuration` | Project settings are invalid, incomplete, or contradictory. |
| `toolchain` | An external compiler, linker, SDK, or build dependency is missing or failed. |
| `runtime` | A generated program encountered a runtime failure. This is a runtime event, not a source-compilation error. |

A diagnostic should have one primary category. Additional context belongs in related information rather than a comma-separated category list.

## 3. Stable diagnostic identifiers

Use a stable, machine-readable code independent of the message wording and severity. Proposed form:

`GFA-<DOMAIN>-<NUMBER>`

Examples of domain names are `SRC`, `LEX`, `PARSE`, `SEM`, `SUPPORT`, `PROFILE`, `TARGET`, `CONFIG`, `TOOLCHAIN`, and `RUNTIME`. The numeric portion is zero-padded and allocated once. This naming scheme is a proposal; reserve and publish actual codes only when implementation begins.

Rules:

- A code identifies one diagnostic condition, not a line of text or a severity.
- Do not reuse a retired code for a different condition.
- Message wording may improve without changing the code's meaning.
- If a condition is split or its meaning changes incompatibly, allocate a new code and document the transition.
- Tests should assert codes and structured fields where those are part of the contract, not rely only on exact prose.

## 4. Source locations and excerpts

A source location should identify the original source unit and a primary range. Where available, show:

- a repository/project-relative or user-supplied source name;
- a one-based line and column for display;
- the end of the primary range;
- a short excerpt from the original source;
- a caret or range marker that identifies the relevant text.

The implemented `SourceMap` stores half-open UTF-8 byte spans and maps valid byte offsets to one-based line and Unicode-scalar columns. It strips a carriage return before a line ending for column calculations; mapping through included or composed sources remains future work. The diagnostic renderer aligns carets using Unicode terminal-cell width for visible text. Tabs and other control characters are escaped in excerpts, and caret width follows that escaped representation. Source decoding and composition semantics remain dialect-dependent and are tracked separately under [issue #120](https://github.com/vsimko/gfabasic-rust/issues/120); strict UTF-8 transport does not claim that every GFA BASIC dialect uses UTF-8.

Excerpts are optional when source text is unavailable or when displaying it would expose unrelated data. Keep excerpts short, preserve the original text, and do not normalize a source fragment in a way that makes its position misleading. For generated or transformed code, prefer the original source location carried through the source map.

## 5. Structured diagnostic content

The implemented Rust `Diagnostic` record contains caller-supplied code, severity, category, message, an optional primary source span, related spans, notes, optional help, and references. Its renderer resolves source locations and excerpts through `SourceMap`. The code is caller-supplied; the project has not yet established a stable diagnostic-code registry. The table below describes the intended user-facing record, not a serialized schema:

| Field | Purpose |
|---|---|
| `code` | Stable diagnostic identifier. |
| `severity` | `error`, `warning`, or `note`. |
| `category` | Primary failure category from section 2. |
| `message` | Short statement of the problem. |
| `primary_location` | Source identity and primary range, when applicable. |
| `excerpt` | Optional short source excerpt. |
| `related` | Zero or more additional locations with an explanation. |
| `notes` | Supporting facts, selected configuration, or evidence context. |
| `help` | A safe, actionable next step; omitted when no reliable suggestion exists. |
| `references` | Relevant project documentation, issue, or source citation identifiers. |

For compatibility-related diagnostics, include the selected dialect/version, enabled API profiles, and target when known. A profile conflict should identify each conflicting declaration and its source/reference. A target limitation should state the required capability and the selected target that lacks it.

The exact serialization format, field names, and schema version remain open under [issue #9](https://github.com/vsimko/gfabasic-rust/issues/9) and [issue #12](https://github.com/vsimko/gfabasic-rust/issues/12). JSON is a possible machine-readable representation, not an implemented or accepted interface. If introduced, it must have its own schema version, be deterministic, omit terminal color codes, and keep diagnostic data separate from human-formatted output.

## 6. User-facing support outcomes

Support outcomes describe whether a particular feature is usable for one exact dialect/version, API profile set, and output target. They do not replace the feature-maturity, evidence, implementation, or target dimensions in [VERSIONING.md](VERSIONING.md) and [API-REGISTRY.md](API-REGISTRY.md).

| Outcome | Definition | Compiler behavior when the source requires it |
|---|---|---|
| **Supported** | The documented behavior is implemented and the relevant conformance tests pass for the exact declared scope. Known limitations are stated. | Accept only within the tested scope. |
| **Partially supported** | A clearly bounded subset is implemented and tested; the remaining forms, versions, or edge cases are identified. | Accept only the implemented subset. Emit an error for a use outside it; do not silently approximate. |
| **Unsupported** | The construct or operation is known, but no compatible implementation is available for the selected scope. | Emit an error. |
| **Unverified** | Available evidence is insufficient to establish the required syntax or semantics. | Do not claim support or guess behavior. Emit an error if compilation requires the unresolved behavior. |
| **Unsafe / disabled** | The behavior is documented or recognized but deliberately disabled because it exposes unsafe memory, hardware, operating-system, or host capabilities. | Emit an error and explain the safety boundary. |
| **Target-limited** | The semantics are sufficiently understood, but the selected output target has no implementation for them. | Emit an error naming the target limitation. |
| **Ambiguous** | More than one selected declaration or profile can match and no evidence-backed resolution rule exists. | Emit an error listing the candidates; never resolve by file order or arbitrary precedence. |

These are report outcomes, not maturity labels. In particular:

- **Research** and **Proposed** describe feature maturity, not compiler support.
- **Experimental** means an implementation exists but is not stable; it must not be reported as fully supported.
- **Stable** is permitted only for the exact documented and tested scope defined in [VERSIONING.md](VERSIONING.md).
- **Verified**, **Inferred**, **Disputed**, and **Unknown** describe evidence quality, not implementation.
- An API registry entry is not proof of support. Its evidence, implementation mapping, target classification, and tests must be considered separately.

A partially supported feature is not permission to execute an unverified fallback. If the compiler cannot prove that a particular use is inside the implemented subset, fail closed with an actionable diagnostic.

## 7. Unsupported syntax, operations, and profile conflicts

The frontend must distinguish a construct that is invalid in the selected dialect/version from one that is valid in the source language but unsupported by this compiler. The distinction depends on accepted dialect grammar and source evidence; do not infer it from another BASIC dialect.

For recognized but unsupported constructs:

1. Identify the construct and source range.
2. State whether the limitation is implementation, evidence, profile, or target related.
3. Name the selected dialect/version/profile/target context.
4. Explain that the compiler will not ignore or emulate the operation.
5. Offer a manual next step only when a reliable reference exists.

For conflicting API declarations, report the source name, each candidate declaration/profile, and the reason resolution is ambiguous. Do not pick one based on declaration order, operating system, or output target. A future explicit disambiguation rule must be source-backed and documented in the API profile/registry contract.

## 8. Runtime failures

Runtime failures occur after compilation and must not be reported as if they were source syntax errors. When runtime reporting exists, use a distinct runtime event with:

- a stable runtime code;
- a concise failure description;
- the original source location when source mapping is available;
- a call/operation context that is safe to expose;
- the runtime and target identity;
- a relevant error value or operating-system result when defined by the runtime contract.

Do not expose arbitrary memory contents, secrets, or unbounded dumps by default. The exact error propagation and runtime ABI remain open under the runtime architecture work.

## 9. Examples

The excerpts below use placeholders such as `⟨source construct⟩`; they are diagnostic-format illustrations, not GFA BASIC syntax examples.

### Unsupported construct

```text
src/main.bas:12:3: error[GFA-SUPPORT-0001]: this construct is not supported for the selected compilation
  12 |   ⟨source construct⟩
     |   ^^^^^^^^^^^^^^^^^^
  = note: selected dialect: ⟨dialect/version⟩
  = note: selected profiles: ⟨profile list⟩
  = note: target: ⟨target triple⟩
  = help: consult the compatibility matrix before replacing this construct
```

The message does not say that the construct is invalid GFA BASIC. It says only that this compiler does not support it in the selected configuration.

### Conflicting API profiles

```text
src/main.bas:18:1: error[GFA-PROFILE-0001]: the selected profiles provide conflicting declarations for ⟨name⟩
  = note: candidate: ⟨declaration ID A⟩ from ⟨profile A⟩
  = note: candidate: ⟨declaration ID B⟩ from ⟨profile B⟩
  = help: resolve the profile/declaration conflict using the documented API profile rules
```

The compiler must not select either declaration implicitly.

## 10. Migration guidance

Compatibility diagnostics may suggest investigation or a manual migration path. They must not promise that a replacement is equivalent unless the replacement's semantics are verified for the same source dialect/version and selected target.

- Prefer a reference to the exact manual page, API declaration, or compatibility-matrix entry.
- Explain known differences in types, ranges, side effects, resource lifecycle, timing, error behavior, or target capabilities.
- Mark a suggestion as a candidate when equivalence has not been established.
- Do not rewrite source automatically by default.
- Do not present a successful compile after a source rewrite as proof of behavioral equivalence.
- Require a user review and relevant regression/conformance tests for manual migrations.

Automatic fixes, if ever introduced, must be narrowly scoped, semantics-preserving for the declared source scope, reviewable as a diff, and independently tested. Otherwise provide no fix.

## 11. Validation and acceptance

When the implementation exists, tests should cover at least:

- stable code and category for representative source, syntax, semantic, unsupported, profile, target, configuration, and toolchain failures;
- correct primary and related source locations, including multi-file/source-map cases;
- no silent acceptance or skipping of unsupported constructs;
- deterministic human and machine-readable output;
- separation of compile-time diagnostics from runtime failures;
- support-outcome reporting for exact dialect/version/profile/target combinations;
- migration hints that do not promise unverified equivalence.

The CLI must document exit-status behavior separately; this document does not assign numeric exit codes. The source-map diagnostic model, human renderer, and unit tests exist. Frontend stages do not yet emit diagnostics; there is no stable code registry, machine-readable schema, CLI integration, or executable conformance/diagnostic test runner.

Tracking: [issue #26](https://github.com/vsimko/gfabasic-rust/issues/26).
