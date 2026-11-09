---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — GURU declaration audit"
---

# Amiga GFA BASIC 3.5 — GURU declaration audit

Primary source: *GFA BASIC 3.5 Interpreter*, Chapter 13 addendum, printed pp. 13-35–13-36. This records what the addendum says; it is not a runtime-conformance claim.

## 1. GURU statement

- **Form:** `GURU x`.
- **Documented switch:** `x=0` disables GURU checking; `x<>0` enables it.
- **Documented effect:** when a GURU Meditation/system crash occurs, GFA BASIC transfers to the `ONERROR GOSUB` routine or generates error message `#109` (“GURU Meditation!”).
- **Version/source:** Amiga 3.5 addendum, printed p. 13-36.
- **Unspecified:** accepted numeric types, conversion, whether all nonzero values are equivalent, state lifetime, and behavior when no ONERROR handler is installed beyond the stated #109 alternative.

## 2. GURU function

- **Form:** `GURU`, used as an expression; the example is `PRINT HEX$(GURU)`.
- **Documented purpose:** returns the GURU number, allowing the handler to inspect the cause.
- **Example context:** `GURU 1`, followed by `ONERROR GOSUB ER`; the handler prints `HEX$(GURU)`.
- **Version/source:** Amiga 3.5 addendum, printed p. 13-36.
- **Unspecified:** return type, value outside a GURU handler, reset timing, and whether the value is stable after nested error handling.

## 3. Error mapping and recovery constraints

- The addendum identifies error messages `#110`–`#118` as the “normal” GURUs `$2`–`$A`.
- In compiled programs (version 3.5), `RESUME NEXT` and `RESUME LABEL` cannot be executed when an `ONERROR GOSUB` call was caused by a GURU.
- The manual advises terminating the program as soon as possible after `#109`; it characterizes GURUs `$2`–`$A` as less dangerous and normally not problematic.
- GURU checking works only if the system can continue program execution. The addendum explicitly attributes GURUs to system problems, not to GFA BASIC itself.
- **Unspecified:** complete GURU number catalogue, exact mapping for every possible number, interpreter-mode recovery behavior, and the OS conditions under which execution can continue.

## Compatibility boundary

Both the statement and function are explicitly documented as 3.5 additions. The compiled-program restriction is specifically stated for version 3.5 and must not be generalized to interpreted programs. The source does not define a portable mechanism for recovering from arbitrary Amiga system crashes. No original-interpreter runtime observations are available; all claims above are manual-derived.

## Implementation implications

1. Keep the GURU-check enable state separate from ordinary BASIC error-handler state.
2. Preserve the numeric GURU code for retrieval by the `GURU` expression while handling the corresponding error.
3. Do not permit compiled-mode `RESUME NEXT` or `RESUME LABEL` after a GURU-triggered handler invocation.
4. Avoid inventing a GURU-number table beyond the documented #110–#118 / $2–$A range.
5. Add runtime compatibility tests only when a legally obtained, version-identified Amiga GFA BASIC 3.5 interpreter is available.
