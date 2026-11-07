# Amiga GFA BASIC 3.5 formatter

## Status and scope

The formatter is a first-class library API in `gfabasic-compiler`, exposed as
`formatter::format_source`. The CLI delegates to the same implementation.
It currently formats only the evidence-gated prototype grammar:

- assignments of `FACT(n)`, `VARIAT(n,k)`, and `COMBIN(n,k)` to `x`, `y`,
  or `z`;
- `PRINT` statements containing the supported variables;
- the independently verified matrix fixture forms: `DIM a(rows,columns)`,
  `MAT ONE a()`, `MAT SET a()=value`, `MAT CLR a()`, `MAT PRINT a()`,
  `MAT BASE 0|1`, `MAT TRANS a()[=b()]`, `MAT NORM a(),0|1`, `MAT ADD`/`MAT SUB`, and the documented
  `MAT MUL` matrix-result, scalar-result, and in-place scalar forms.

The matrix parser slice intentionally supports only matrix names `a` and `b`,
two integer dimensions, and a non-negative integer scalar for `MAT SET`.
`MAT BASE` accepts only the documented offsets 0 and 1. `MAT TRANS` supports
the documented in-place and destination/source forms. `MAT NORM` accepts only
row mode 0 and column mode 1. `MAT ADD` and `MAT SUB` accept the documented
result expressions and in-place matrix/scalar forms. `MAT MUL` accepts
matrix-result products, scalar-result products with two or three matrix
operands, and in-place scalar multiplication. `MAT DET`, `MAT QDET`, and
`MAT RANG` accept a scalar destination, matrix reference with optional row
and column offsets, and an optional square-submatrix order. Arithmetic operators are
spaced on both sides by the formatter. The parser still limits matrix names
to `a` and `b`; scalar operands retain their source identifier spelling.
It does not implement
matrix evaluation or general array semantics. The executable
compiler and semantic-analysis stages explicitly reject these matrix AST nodes;
this increment is for parsing and formatting only.

This is not a general GFA BASIC formatter and does not establish full
interpreter compatibility. Syntax outside the implemented parser subset is
diagnosed; the formatter does not attempt best-effort rewriting.

## Canonical prototype style

The current deterministic style policy is a project decision for this narrow
subset. It is not a claim about the historical GFA editor's appearance.

| Element | Canonical output |
|---|---|
| Assignment | One space on each side of `=` |
| Intrinsic arguments | No space immediately inside parentheses |
| Comma-separated items | One space after each comma; none before |
| Statements | One statement per physical output line |
| Blank lines | Collapsed; empty lines are not retained |
| Line endings | LF (`\n`) |
| Final line | Exactly one final newline for non-empty output |
| Identifiers and built-ins | Preserve original spelling exactly |
| Integer literals | Preserve original spelling, including leading zeroes |

Example:

Input:

```basic
  x=FACT(0001)
y = VARIAT(5,2)
PRINT   x,y,z
```

Canonical output:

```basic
x = FACT(0001)
y = VARIAT(5, 2)
PRINT x, y, z
```

Formatting is idempotent: applying the formatter to canonical output produces
byte-identical output. The formatter compares parsed program meaning before
and after formatting in its test corpus.

## CLI

```sh
# Write canonical source to stdout
gfabasic fmt program.bas

# Return exit status 1 when formatting would change the file
gfabasic fmt --check program.bas

# Write to a separate destination
gfabasic fmt program.bas -o formatted.bas

# Replace the source explicitly
gfabasic fmt --in-place program.bas
```

`--check` does not write files. Normal formatting writes to stdout unless
`-o` or `--in-place` is selected. File destinations are replaced through a
sibling temporary file and rename. Parse/format errors occur before any write,
so invalid or unsupported input is not overwritten.

The CLI currently reads UTF-8 source and emits LF output. This is an explicit
transport and formatting policy for the prototype, not a statement about the
historical Amiga source encoding or line-ending convention.

## Deliberately unsupported syntax

The formatter currently rejects comments, string literals, `DATA` statements,
and all other syntax outside the prototype grammar. It must not discard or
guess at these constructs.

The supplied Amiga interpreter manual's base chapters describe `REM`,
apostrophe remarks, and inline `!` comments on pages labelled GFA-BASIC 3.0.
The German 1990 book provides supplementary Amiga V3.03 descriptions. Those
are inherited candidates, not proof of Amiga GFA BASIC interpreter 3.5
behavior. The current 3.5 lexer evidence inventory explicitly leaves comment
rules unresolved, including comment context around strings and `DATA`.
Therefore these forms must remain unsupported until a 3.5-specific source or a
reproducible observation confirms their lexical behavior.

The same evidence gate applies to string escaping, DATA payload boundaries,
general identifier syntax, and operators. Do not broaden the formatter by
importing Atari ST rules or by treating the base 3.0 text as automatically
normative for 3.5.

## Tests and CI

Formatter unit tests cover canonical whitespace, all currently supported
statement forms, identifier/intrinsic/literal spelling preservation,
idempotence, semantic preservation, unsupported syntax rejection, and blank
line policy. The verified Amiga 3.5 combinatorics and matrix fixtures are checked by CI
with `gfabasic fmt --check`.

As the parser gains verified grammar constructs, add independently authored
golden fixtures and semantic/idempotence tests before enabling formatting for
those constructs. See the [3.5 lexer evidence inventory](../research/amiga-gfa-basic-35-lexer-evidence.md)
and [corpus policy](../../tests/corpus/README.md).
