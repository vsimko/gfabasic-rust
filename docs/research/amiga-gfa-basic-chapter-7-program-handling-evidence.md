---
type: "Research Note"
title: "Amiga GFA BASIC Chapter 7 — program handling evidence"
---

# Amiga GFA BASIC Chapter 7 — program handling evidence

## Scope and evidence rules

Primary source: `AM-INTERPRETER-35`, *GFA-BASIC 3.5 Interpreter*, Chapter 7, printed pages 7-2–7-55. The manual explicitly says its base chapters describe GFA-BASIC 3.0 and Chapter 13 is the 3.5 addendum. Therefore these Chapter 7 entries are **Amiga 3.0 base-manual evidence**, not proof of 3.5 availability. Reconcile each construct against 3.5 evidence or a controlled 3.5 interpreter observation before adding it to the normative 3.5 registry.

Printed page numbers below are the source locators. PDF viewer page numbers are not interchangeable with them.

Evidence classes are independent:

- **Documented (3.0)**: described in the cited base-manual page.
- **Observed (3.52)**: exercised against a named 3.52 interpreter build; this does not establish 3.5 behavior.
- **Implemented**: implemented in this repository; documentation or a probe alone does not qualify.
- **Conformant**: a passing executable test against an accepted reference for the claimed version.

## Construct inventory

| Construct | Syntax / aliases recorded by the manual | Semantics and constraints | Printed pages |
|---|---|---|---:|
| Loop model | `FOR/NEXT`, `REPEAT/UNTIL`, `WHILE/WEND`, `DO/LOOP` | FOR/NEXT and REPEAT/UNTIL are described as non-rejecting; WHILE/WEND is rejecting. DO/LOOP is unbounded unless exited. Conditions can be placed at the DO and/or LOOP boundary. | 7-2 |
| IF | `IF condition [THEN]`, optional `ELSE`, `ENDIF` | THEN is optional. Numeric nonzero values are true. ELSE runs when the IF condition is false. | 7-4–7-5 |
| ELSE IF | `ELSE IF condition`; compact spelling `ELSEIF` | A true branch executes through the next ELSEIF/ELSE/ENDIF, then control continues after ENDIF. ELSE is the fallback. | 7-6–7-7 |
| ON GOSUB | `ON expression GOSUB proc1,proc2,...` | Selects the 1-based procedure slot; fractional parts are ignored. Out-of-range values call no procedure. Procedures take no parameters; execution resumes after ON GOSUB. | 7-8 |
| SELECT | `SELECT expression`, `CASE value [TO value]`, comma lists, open-ended ranges, `DEFAULT`, `ENDSELECT`, `CONT` | Cases are tested top-to-bottom; range endpoints are inclusive. DEFAULT is the fallback; OTHERWISE is an editor/interpreter synonym for DEFAULT. CONT is listed in the syntax but its full control semantics require a separate check. | 7-9–7-12 |
| FOR | `FOR variable=start TO end [STEP step]`, `NEXT`; `DOWNTO` is documented as an alternative to negative STEP | The counter is left at the value that first exceeds the bound; the manual says the body executes at least once. DOWNTO cannot be combined with STEP. | 7-13–7-14 |
| REPEAT | `REPEAT ... UNTIL condition`; `ENDREPEAT condition` | Post-test loop: body executes at least once unless it contains EXIT IF or GOTO. ENDREPEAT is replaced by UNTIL. | 7-15 |
| WHILE | `WHILE condition ... WEND`; `END WHILE` | Pre-test loop; body repeats while condition is true. END WHILE is replaced by WEND. | 7-16 |
| DO/LOOP | `DO ... LOOP`; `ENDDO` | Endless loop; exit by EXIT IF, GOTO, or program termination. ENDDO is replaced by LOOP. | 7-17 |
| Conditional DO/LOOP | `DO [WHILE condition|UNTIL condition]` and `LOOP [WHILE condition|UNTIL condition]` | Conditions on DO are checked before the body; conditions on LOOP determine whether to repeat. The manual describes nine combinations. | 7-18–7-19 |
| EXIT IF | `EXIT IF condition` | Leaves the current loop when the condition is true. The text also permits use inside IF/ENDIF and SELECT/ENDSELECT; exact interaction should be probed independently. | 7-20 |
| Procedures | `GOSUB proc[(args)]`, `PROCEDURE proc[(vars)]`, `RETURN`; aliases `SUB`, `ENDPROC`, `ENDSUB` | Procedure body is between PROCEDURE and RETURN. Parameters can be values or variable references using VAR. GOSUB disambiguates names that collide with BASIC commands. | 7-21–7-22 |
| LOCAL | `LOCAL var[,var...]` | Restricts variables to a procedure/function scope. Lifetime, recursion interaction, and nested-scope details need separate extraction/probes. | 7-23 |
| Multi-line functions | `FUNCTION name(args)` ... `RETURN value` ... `ENDFUNC`; calls may use `@name(...)` | Functions are procedure-like and return a value. Parameter passing and local-variable behavior are described in conjunction with procedures and LOCAL. | 7-24–7-25 |
| Single-line functions | `DEFFN name[(args)]=expression`; calls use `FN name(...)` or `@name(...)` | Expression may be numeric or string. Parameters are local to the function. DEFFN functions cannot be recursive according to the manual. | 7-26–7-27 |
| Break handling | `ON BREAK`, `ON BREAK CONT`, `ON BREAK GOSUB proc` | Controls the Ctrl+left-Shift+Alt break-key reaction: normal break, disabled reaction, or procedure dispatch. | 7-28 |
| Error handling | `ON ERROR`, `ON ERROR GOSUB proc`, `RESUME [NEXT]`, `RESUME label` | Error handler must re-arm ON ERROR GOSUB to catch subsequent errors. RESUME NEXT continues after the failing command; RESUME label jumps to a label. Fatal errors require RESUME label. | 7-29–7-30 |
| Error values | `ERROR number`, `ERR`, `ERR$(number)` | ERROR raises the selected error; ERR contains the current error number; ERR$ returns its message. The error-number table is outside this chapter. | 7-31 |
| Timed procedure dispatch | `EVERY ticks GOSUB proc`, `EVERY STOP/CONT`, `AFTER ticks GOSUB proc`, `AFTER STOP/CONT` | EVERY repeats and AFTER fires once. Ticks are 1/200 second; stated resolution is about 0.1 s and slow commands can delay dispatch. | 7-32–7-34 |
| Comments | `REM text`, apostrophe comment, trailing `! comment` | Comment text is not syntax-checked. Trailing ! comments are disallowed on DATA instructions. | 7-35 |
| Labels and jumps | `label:`, `GOTO label` | Labels may contain letters, digits, underscores, and periods and may start with a digit. GOTO cannot enter or leave procedures/functions. | 7-36 |
| Delays | `PAUSE expression`, `DELAY expression` | PAUSE uses 1/50-second units; DELAY uses seconds. The text notes a 1/50-second minimum timer resolution and a one-second minimum for DELAY. | 7-37 |
| Program stop/exit | `END`, `EDIT`, `STOP` | END ends execution and presents an alert; EDIT returns directly to the editor; STOP offers STOP/CONT and can enter direct mode while preserving variables. QUIT/SYSTEM exit without returning to the editor. | 7-38 |
| Program/editor commands | `NEW`, `LOAD`, `SAVE/PSAVE`, `LIST/LLIST`, `CHAIN`, `RUN`, `SYSTEM/QUIT`, `TRON/TROFF`, `TRACE`, `INLINE`, `C:`, `MONITOR`, `CALL`, `RCALL` | Chapter 7 continues with editor, program-loading, tracing, and native/system-call commands. These are not all language-core statements; classify editor and Amiga/system integration separately. | 7-39–7-55 |

## Independent 3.52 compatibility evidence

The repository contains a native-interpreter regression batch at `tests/interpreter-probes/amiga-3.52/regression-suite.LST` and its expected fixture IDs at `tests/interpreter-probes/amiga-3.52/expected-ids.txt`.

| Probe ID | Construct exercised | Relevance to Chapter 7 | Evidence boundary |
|---|---|---|---|
| `for-next` | FOR/NEXT from 1 through 4; expected sum 10 | Directly exercises the Chapter 7 loop family (7-13–7-14) | 3.52 only; does not prove 3.5 availability or full FOR semantics |
| `arithmetic` | Arithmetic precedence | Supporting expression behavior, not a Chapter 7 construct | 3.52 only |
| `strings` | String concatenation and LEFT$ | Supporting expression/function behavior, not a Chapter 7 construct | 3.52 only |
| `fact-zero` | FACT boundary values | Not Chapter 7; built-in function probe | 3.52 only |

The batch emits `GFATEST:<id>:PASS|FAIL` records. `scripts/validate_gfatest_results.py` checks that a result file is present, ASCII, well-formed, contains exactly the expected unique IDs, and has no missing results. This protocol validator verifies result completeness, not the truth of a PASS record. A successful native 3.52 run and its captured result artifact must be linked before any probe is described as observed/passing.

Do not merge these 3.52 observations into 3.5 evidence, implementation status, or the 3.5 compatibility numerator. They are a separate, version-labelled compatibility dimension.

## Follow-up and unresolved checks

1. Confirm Chapter 7 base-manual constructs individually against the 3.5 target using addendum evidence or controlled 3.5 execution; do not infer retention from absence in the addendum.
2. Expand the 3.52 probe suite with minimal, independently identifiable cases for conditionals, SELECT/CASE, REPEAT/WHILE/DO variants, procedures/VAR/LOCAL, FUNCTION/DEFFN, ON ERROR/RESUME, and EVERY/AFTER. Keep all results explicitly labelled 3.52.
3. Add interpreter result artifacts and provenance (build identity, emulator configuration, run date, expected IDs, raw result checksum) before promoting a probe to observed.
4. Link each construct to a stable registry ID only after the language/platform classification and version applicability are resolved.
5. Chapter 8 graphics remains a separate Amiga/platform API workstream; its existing 3.0 extraction is in [graphics evidence](amiga-gfa-basic-3.0-graphics-evidence.md), printed pp. 8-2–8-24.

## Related work

- [Issue #21 — statements and control flow](https://github.com/vsimko/gfabasic-rust/issues/21)
- [Issue #42 — reference and conformance corpus](https://github.com/vsimko/gfabasic-rust/issues/42)
- [Issue #66 — original interpreter runtime access](https://github.com/vsimko/gfabasic-rust/issues/66)
- [Issue #307 — evidence-based compatibility metrics](https://github.com/vsimko/gfabasic-rust/issues/307)
