# First compiler vertical slice — proposed boundary

**Status:** partially implemented as an experimental FACT/PRINT frontend and native bootstrap prototype; not a complete compiler feature or conformance claim.

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

The manual does not define an exact whitespace contract for the rendered numeric output in this example. The end-to-end test asserts the ordered numeric values while treating runs of output whitespace as separators; it does not assert a stricter formatting guarantee.

No further base-manual grammar review is a prerequisite for this specific source slice. Interpreter execution would strengthen confidence but remains separately blocked by lawful access to the original 3.5 interpreter under #66.

## Pipeline work required

- [x] Frontend: narrow lexer/parser, AST, and source-span diagnostics for the demonstrated FACT/PRINT shape.
- [ ] Semantics: explicit variable binding and FACT argument subset validation.
- [ ] IR: a small typed representation for the value, intrinsic call, and output.
- [x] Bootstrap backend: temporary C/`cc` bridge emits a native host executable; this is not the planned LLVM backend.
- [x] Tests: frontend positive/negative tests and an end-to-end executable test; general IR/conformance tests remain pending.

The frontend currently parses the narrow FACT/PRINT subset and emits source-located lexical/syntax/unsupported diagnostics. A temporary C/`cc` bridge produces a native host executable for the fixture. General semantic analysis, typed project IR, the LLVM backend, and runtime remain unimplemented. The bootstrap path is not a claim of original-interpreter conformance.

## Acceptance

The vertical slice is complete only when a clean Linux x86-64 runner can compile the fixture, execute the generated program independently of the compiler process, assert the numeric result and exit status, and reject an unsupported construct with a stable diagnostic. CI must run the end-to-end test.
