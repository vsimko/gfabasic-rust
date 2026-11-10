---
type: "Research"
title: "Amiga GFA BASIC language population policy"
---

# Amiga GFA BASIC language population policy

This policy records the maintainer's counting decisions for the language-wide implementation progress metric tracked by [issue #307](https://github.com/vsimko/gfabasic-rust/issues/307). The machine-readable counterpart is [amiga-gfa-basic-language-population-policy.toml](amiga-gfa-basic-language-population-policy.toml).

## Denominator: distinct built-in language constructs

Count distinct built-in constructs in these categories:

- Built-in functions.
- Statements and commands, including Amiga-dialect commands.
- Predefined and special variables.
- Distinct operators.
- Built-in data types that have their own declarations or commands.
- Predefined named constants.
- Reserved language keywords and control constructs.

Apply these normalization rules:

- Count a built-in function once, regardless of argument/signature variants.
- Count a command once, regardless of optional parameters or alternate syntactic forms.
- Count an overloaded operator once, regardless of operand types.
- Merge synonyms and alternate spellings with identical semantics into one canonical construct and retain aliases.
- Count a compound control-flow construct once (for example, `IF…ELSE…ENDIF`, `FOR…NEXT`, or `DO…LOOP`); do not count its constituent keywords separately.
- Count each named predefined constant, special variable, distinct operator, and eligible built-in data type separately.
- Do not count variable type suffixes or numeric/string literals as separate constructs.

## Included and excluded population

The supplied Amiga GFA BASIC 3.5 interpreter documentation contains a 3.0 base manual as well as version-specific 3.5 material. Known, individually identified constructs documented in the 3.0 base manual are included in the denominator even when their continued availability in 3.5 has not yet been independently verified. Preserve their original source version and mark their 3.5 status as unverified; do not relabel 3.0 evidence as explicit 3.5 evidence.

Known constructs with unresolved 3.5 availability remain in the denominator and are visibly marked unverified. Entries whose identity or classification is unknown are excluded until resolved. Topic-family placeholders and reference/index headings are not declarations and must be decomposed into individually identified constructs before counting.

Direct 3.5 documentation takes precedence over conflicting statements in the 3.0 base manual. A successful GFA BASIC 3.52 runtime test is accepted as compatibility evidence for 3.5, but the exact tested runtime version must remain recorded separately.

Exclude:

- Editor-only commands and editor features.
- Direct Amiga OS library calls.
- Compiler-only constructs.
- User-defined procedures, functions, and variables.
- Numeric and string literals.

Amiga-specific commands that belong to the interpreter language dialect remain included.

## Numerator: fully implemented constructs

Count a declaration in the numerator only when its implementation status is exactly `implemented`, source implementation exists, and a confirming test passes. `partial`, `prototype`, `planned`, `research`, `blocked`, unknown, or other states do not count.

## Independent evidence dimensions

Keep these dimensions independent on each declaration:

- Source evidence and its provenance/version.
- Implementation status.
- Target/platform support.
- Runtime observation, including exact runtime version.
- Conformance status.

An unverified but known declaration contributes to the denominator, never to the numerator. The dashboard must not present a final language-wide N/M until the inventory is declaration-level, deduplicated, and complete under this policy. Until then, only explicitly labelled subset metrics may be shown.
