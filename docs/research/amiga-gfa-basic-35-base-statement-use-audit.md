---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — inherited statement evidence from the interpreter addendum"
---

# Amiga GFA BASIC 3.5 — inherited statement evidence from the interpreter addendum

## Scope

This audit records base-manual statement forms that are directly used or explicitly referenced in the version-specific Chapter 13 addendum of `AM-INTERPRETER-35`. It confirms only the form and context shown; grouped variants not demonstrated remain `documented-in-3.0-only`.

## Direct 3.5 evidence

| Base construct | 3.5 evidence pages | Evidence | Boundary |
|---|---|---|---|
| `PRINT` | 13-2, 13-34–13-36 | Plain `PRINT` appears in matrix headings, separators, the data-pointer example, and GURU output. | `PRINT AT`, `WRITE`, and `LOCATE` are not demonstrated. |
| `FOR / NEXT / DOWNTO` | 13-35 | The data-pointer example uses `FOR j%=i% DOWNTO 0 ... NEXT j%`; other examples use ascending `TO`. | The editor replacement `ENDFOR` is not demonstrated. |
| `DO WHILE ... LOOP` | 13-35 | The data-pointer example uses `DO WHILE DATA ... LOOP`. | Other beginning-/end-tested WHILE/UNTIL placements remain unconfirmed. |
| `DATA / READ` | 13-35 | The example declares `DATA` records and consumes a value with `READ a`. | The `DATA` availability predicate in `DO WHILE DATA` is documented only as part of this loop example; it is not added to the 117-name function inventory. |
| `INC / DEC` | 13-35 | The example uses `INC i%` and `DEC i%`. | Only the shown integer-variable form is confirmed. |
| `ONERROR GOSUB` | 13-36 | The GURU description explains transfer to an `ONERROR GOSUB` routine; the example uses `ONERROR GOSUB ER`. | Confirmation is limited to GURU recovery; the separate `ON ERROR` reset-to-default form is not demonstrated. |
| `RESUME NEXT / RESUME LABEL` | 13-36 | The addendum explicitly states that compiled 3.5 programs cannot execute these forms after a GURU-triggered `ONERROR GOSUB`. | This is a documented restriction in that context, not a general statement about every RESUME form or interpreter mode. Plain `RESUME` remains unconfirmed. |

## Grouped variants deliberately left unconfirmed

- `PRINT AT`, `WRITE`, and `LOCATE` remain base-only; the addendum uses plain `PRINT`.
- `ENDFOR` remains base-only; the addendum uses `NEXT`.
- Other `DO / LOOP` WHILE/UNTIL placements remain base-only; only `DO WHILE ... LOOP` is shown.
- `ON ERROR` reset-to-default remains base-only; the addendum references `ONERROR GOSUB` in the GURU recovery context.
- Plain `RESUME` remains base-only; the addendum specifically documents `RESUME NEXT` and `RESUME LABEL` as restricted in compiled 3.5 GURU recovery.

## Evidence boundary

These are documentary confirmations, not claims of complete syntax coverage, runtime conformance, or Rust implementation. Original 3.0 source pages remain attached to each candidate; the 3.5 evidence source and printed pages are stored separately in `amiga-gfa-basic-candidates.toml`.

See also `amiga-gfa-basic-candidate-report.md` and `amiga-gfa-basic-35-base-function-use-audit.md`.
