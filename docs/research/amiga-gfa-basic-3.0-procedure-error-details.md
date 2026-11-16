---
type: "Research Note"
title: "GFA BASIC 3.0 Amiga — procedures, functions and error-flow evidence"
---

# GFA BASIC 3.0 Amiga — procedures, functions and error-flow evidence

Source: *GFA BASIC 3.5 Interpreter*, Chapter 7, printed pp. 7-21–7-34. This is the 3.0 base-manual program-handling chapter.

## Procedures and subroutines

- `PROCEDURE name[(parameters)]` declares a procedure; `RETURN` returns from it. The index places the main procedure section at p. 7-21 and `LOCAL` at p. 7-23. (7-21–7-23)
- The chapter distinguishes ordinary subroutines from `FUNCTION`/`DEFFN` user functions. The `@` notation is used in documented function/procedure calls; `FN` is also indexed as a function-call form. (7-24–7-27)
- The manual describes local variables and procedure/function parameters, but exact typing, by-reference/value semantics, recursion limits and lifetime require a separate signature-level transcription. Do not infer these from the names alone.

## Error and break handling

| Form | Source page | Candidate semantics |
|---|---:|---|
| `ON BREAK CONT` | 7-28 | Continue/handle break behavior as described by the interpreter |
| `ON BREAK GOSUB proc` | 7-28 | Route break event to a procedure |
| `ON ERROR GOSUB proc` | 7-29 | Route runtime error to a procedure |
| `ERROR n` | 7-31 | Raise/set an error condition |
| `ERR` | 7-31 | Exposes error number |
| `RESUME NEXT` | 7-29 | Resume execution after the failing statement |

The 3.5 addendum's `GURU` facility is separate explicit 3.5 evidence; it adds GURU-check control and a GURU-number function, and warns that recovery after a GURU-induced error is restricted in compiled programs (printed pp. 13-35–13-36). Do not treat this as proof that all 3.0 error handling details are unchanged.

## Timer event commands

`EVERY` and `AFTER` are indexed on printed p. 7-32. The chapter describes scheduled procedure execution, with EVERY repeating and AFTER one-shot timing. Exact time units, cancellation, nesting/reentrancy and interaction with ON BREAK/ON ERROR should be checked against the full entry before implementation. (7-32–7-34)

## Version and implementation boundary

These forms are 3.0 source candidates. The page references and names support inventory discovery, but exact syntax/semantics still need individual review and 3.5 confirmation. The Amiga interpreter's event/timer behavior is not automatically portable to the Linux runtime.
