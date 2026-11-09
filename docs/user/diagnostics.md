---
type: "User Guide"
title: "Understanding compiler diagnostics"
---

# Understanding compiler diagnostics

- **Audience:** Users of the planned GFA BASIC compiler.
- **Status:** Proposed user guidance; the source-map diagnostic API and human renderer are implemented in the compiler library, but no GFA BASIC frontend or CLI command currently emits them. Stable diagnostic-code allocation, machine-readable output, and runtime reporting remain unimplemented.
- **Design reference:** [Diagnostics and compatibility reporting](../architecture/DIAGNOSTICS.md).

This page explains the planned user-facing diagnostic contract and records the implemented low-level renderer. The current executable does not emit compiler diagnostics because the GFA BASIC frontend and CLI diagnostic integration do not exist yet.

## What a diagnostic tells you

A diagnostic identifies a problem or a compatibility limitation. It should include a severity, a category, a stable code, a short explanation, and a source location when one is available.

| Severity | Meaning | Expected action |
|---|---|---|
| `error` | The compiler cannot safely complete the requested operation. | Correct the source or configuration, or select a supported feature/profile/target. |
| `warning` | The operation can continue, but a known risk or limitation needs review. | Review the explanation before relying on the output. |
| `note` | Additional context for another diagnostic. | Read it with the related diagnostic. |

A diagnostic category identifies the failing area. Planned categories include source loading, lexical analysis, syntax, semantics, unsupported features, API profile conflicts, target limitations, configuration, toolchain, and runtime.

## Source locations

When the compiler can map a problem to source text, the diagnostic should show the source file, line and column, a short excerpt, and a marker for the relevant range. For included or composed source, the location should refer back to the original source file.

The current source map reports one-based Unicode-scalar columns, not byte offsets or terminal-cell positions. The renderer uses terminal-cell widths only to align the visible caret under the excerpt; tabs and other control characters are escaped before display. Mapping through composed sources remains future work.

## Feature support outcomes

Support is evaluated for one exact dialect/version, API profile set, and output target.

| Outcome | Meaning |
|---|---|
| **Supported** | The documented behavior is implemented and the relevant tests pass for the stated scope. |
| **Partially supported** | A defined subset is implemented and tested; other forms or edge cases are excluded. |
| **Unsupported** | The behavior is known, but no compatible implementation is available. |
| **Unverified** | Evidence is not sufficient to establish the required behavior. |
| **Unsafe / disabled** | The behavior is recognized but deliberately disabled because of its safety boundary. |
| **Target-limited** | The behavior is understood, but the selected output target cannot provide it. |
| **Ambiguous** | Multiple declarations or profiles match and no evidence-backed resolution rule exists. |

These outcomes are not interchangeable with research maturity. A command listed in a research note or API registry is not necessarily implemented or supported.

## Unsupported and ambiguous behavior

The compiler must not silently ignore source text, approximate an unsupported operation, or choose arbitrarily between conflicting API declarations. If compilation requires behavior that is unsupported, unverified, target-limited, or ambiguous, the planned behavior is to stop with an error.

A diagnostic should state the limitation and identify the selected dialect/version, profiles, and target when known. It should provide a manual next step only when that advice is reliable.

## Migration guidance

A diagnostic may point to a manual page, compatibility entry, or possible manual migration path. A suggestion is not a guarantee that another construct has equivalent behavior.

- Check the cited source and exact dialect/version.
- Review differences in types, ranges, side effects, resource handling, timing, and error behavior.
- Treat unverified replacements as candidates, not drop-in equivalents.
- Review source changes and add regression/conformance tests before relying on a migration.
- Do not assume that a successful compile proves behavioral equivalence.

Automatic source rewriting is not part of the current project baseline.

## Current project status

The source-map diagnostic model and deterministic human renderer are implemented and unit-tested in the compiler library. Lexer/parser/semantic stages do not yet emit diagnostics; stable diagnostic-code allocation, machine-readable schema/CLI output, and runtime error reporting remain unimplemented. The architecture reference defines the broader proposed contract, not a claim that frontend diagnostics are available.
