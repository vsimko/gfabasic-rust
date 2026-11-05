# First compiler vertical slice — proposed boundary

**Status:** proposed; research-backed candidate, not an implemented compiler feature.

**Normative source dialect:** Commodore Amiga GFA BASIC interpreter 3.5 (ADR 0009).

## Candidate program

The smallest useful end-to-end candidate currently in the corpus is:

```basic
x=FACT(0)
y=FACT(1)
z=FACT(2)
PRINT x,y,z
```

Fixture: `tests/corpus/amiga/3.5/combinatorics/fact-zero.bas`.

## Evidence

- The supplied 3.5 update explicitly lists `x=FACT(n)`, with `n` an integer expression and `x` an arithmetic expression (printed p. 14-1).
- The same page defines factorial and explicitly states `0! = 1` (printed p. 14-1).
- The following page demonstrates assignment from `FACT(n)` and output using `PRINT x,y,z` (printed p. 14-2).
- The independently authored fixture is marked page-image-verified. It has not been run on the original interpreter.

## Proposed implementation boundary

1. One explicitly selected Amiga GFA BASIC 3.5 dialect configuration.
2. Three numeric variable assignments (`x`, `y`, and `z`).
3. Integer literals `0`, `1`, and `2`.
4. Calls to the built-in `FACT` with integer-expression arguments.
5. A comma-separated `PRINT` list for the assigned numeric values.
6. Deterministic output and process exit status on Linux x86-64.

No SDL3, graphics, Amiga API, file I/O, arrays, matrices, procedures, source composition, or other built-ins are required.

## Evidence sufficiency and remaining uncertainty

The selected program's required syntax is demonstrated directly in the 3.5 addendum, so this narrow slice does not need to promote unrelated 3.0-only grammar:

- Printed p. 14-1 gives the `FACT(n)` signature, specifies an integer-expression argument, defines the factorial result, and explicitly states `0! = 1`.
- Printed p. 14-2 demonstrates assignments to `x`, `y`, and `z` from built-in functions, followed by the comma-separated `PRINT x,y,z` statement. Its example uses multiple numeric literals and one statement per source line.
- The fixture changes the example values and uses only the documented `FACT` function; it is independently authored.

The manual does not define an exact whitespace contract for the rendered numeric output in this example. The end-to-end test should assert the ordered numeric values while treating runs of output whitespace as separators; it must not invent a stricter formatting guarantee.

No further base-manual grammar review is a prerequisite for this specific source slice. Interpreter execution would strengthen confidence but remains separately blocked by lawful access to the original 3.5 interpreter under #66.

## Pipeline work required

- Frontend: lexer, minimal parser, AST, and source-span diagnostics.
- Semantics: variable binding, integer expression typing, and the 3.5 `FACT` intrinsic.
- IR: a small typed representation for the value, intrinsic call, and output.
- Backend/runtime: emit a standalone Linux x86-64 executable with deterministic output.
- Tests: corpus metadata validation, frontend positive/negative tests, IR tests, and an end-to-end executable test.

The current repository only has source-map/loading primitives, structured diagnostics, and help/version CLI support. Frontend, IR, backend, and runtime modules are placeholders. This document does not claim that compilation works.

## Acceptance

The vertical slice is complete only when a clean Linux x86-64 runner can compile the fixture, execute the generated program independently of the compiler process, assert the numeric result and exit status, and reject an unsupported construct with a stable diagnostic. CI must run the end-to-end test.