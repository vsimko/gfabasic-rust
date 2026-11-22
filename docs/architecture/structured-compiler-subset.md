---
type: "Architecture Note"
title: "Structured numeric compiler subset"
---

# Structured numeric compiler subset

Status: experimental compiler prototype, tracked by [issue #617](https://github.com/vsimko/gfabasic-rust/issues/617).

## Supported syntax

The current compiler accepts these numeric forms when a structured construct is present in the source:

```basic
FOR i = 1 TO 10
  total = total + i
NEXT i

FOR j = 10 DOWNTO 1
  total = total + j
NEXT j

FOR k = 1 TO 9 STEP 2
  total = total + k
NEXT k

PROCEDURE Add(a,b)
  total = total + a + b
RETURN

FUNCTION Square(n)
  RETURN n * n
ENDFUNC

DEFFN Double(n) = n * 2

GOSUB Add(3,4)
total = total + @Square(5) + FN Double(6)
PRINT total
```

- Expressions: integer literals, variables, unary minus, parentheses, `+`, `-`, `*`, `/`, and user-function calls.
- `FOR` counters may be named variables. `NEXT` may omit the counter name; if supplied, it must match the opening `FOR`.
- `TO` defaults to step +1. `DOWNTO` defaults to step -1 and cannot be combined with `STEP`. Explicit `STEP` may be positive or negative; a statically known zero step is diagnosed and a dynamic zero step aborts at runtime.
- The loop is emitted as a post-test loop, matching the base-manual description that `FOR/NEXT` is non-rejecting and executes at least once. The counter is advanced after each body execution.
- Procedures accept positional value parameters and are called through `GOSUB`. `SUB`, `ENDPROC`, and `ENDSUB` are accepted aliases. Parameters shadow global variables; reference parameters are not supported.
- Multi-line `FUNCTION` returns a numeric expression through `RETURN expression` and ends with `ENDFUNC`. Single-line `DEFFN` definitions are supported and can be called using `FN name(...)` or `@name(...)`. DEFFN self-recursion is rejected; multi-line functions may recurse.
- The numeric bootstrap currently uses signed 64-bit C `long long` values and integer division. This is an implementation subset, not a claim about the full GFA BASIC numeric model.

## Explicitly unsupported

This milestone does not implement `VAR` reference parameters, `LOCAL`, string parameters or string-returning functions, type suffixes, array parameters, nested declarations, `IF/ELSE`, `EXIT IF`, `GOTO`, or the other loop families. Unsupported statements are diagnosed rather than ignored. Function/procedure semantics such as dynamic-local rules, all return paths, and complete recursion behavior are not claimed beyond the tested subset.

## Evidence boundary

The supplied *GFA-BASIC 3.5 Interpreter* manual's Chapter 7 base material explicitly identifies itself as GFA-BASIC 3.0. It documents:

- non-rejecting `FOR/NEXT` loops and their at-least-once behavior (printed p. 7-2);
- `FOR variable=start TO end [STEP step]` and `NEXT` (printed pp. 7-13–7-14);
- `PROCEDURE`, `GOSUB`, value/VAR parameters, and `RETURN` (printed pp. 7-21–7-22);
- multi-line `FUNCTION` and `DEFFN` syntax and calls (printed pp. 7-24–7-27).

These Chapter 7 descriptions are 3.0 base-manual evidence, not proof of exact 3.5 compatibility. The exact 3.5 native smoke run [44094135503](https://github.com/vsimko/gfabasic-rust/actions/runs/44094135503) includes only a simple `FOR/NEXT` sum probe (1 through 4 equals 10). Procedures and functions have not been run against the exact 3.5 interpreter and remain UNTESTED there. Compiler end-to-end tests prove only the behavior of this Rust/C bootstrap implementation.

The structured path retains validated source-level blocks in the prototype IR and emits temporary C. It is not yet a fully lowered typed control-flow graph or the production LLVM backend. See [pipeline and IR architecture](PIPELINE-AND-IR.md).
