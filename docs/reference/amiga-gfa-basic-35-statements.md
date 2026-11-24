---
type: "Reference"
title: "Amiga GFA BASIC 3.5 statements — evidenced forms"
---

# Amiga GFA BASIC 3.5 statements — evidenced forms

## Scope and evidence boundary

This page records a **limited set of statement forms directly witnessed in Amiga GFA BASIC 3.5 material**. It is a partial contribution to [issue #21](https://github.com/vsimko/gfabasic-rust/issues/21), not a complete grammar or a claim of full 3.5 compatibility.

Primary source: `AM-INTERPRETER-35`, *GFA-BASIC 3.5 Interpreter*. The manual states that its base chapters describe version 3.0 and that the 3.5 addendum describes additional 3.5 features. The forms below are taken from the addendum at printed pp. 13-34–13-36.

Secondary source: `AM-BOOK-DE-1990`, *Das große GFA-BASIC-Buch zum Amiga* (DATA BECKER, 1990). Its main language chapters describe earlier interpreter versions. Chapter 18 states on printed p. 451 that it describes commands newly added in Version 3.5. The DATA-pointer example on printed p. 480 identifies itself as a demonstration of new GFA-BASIC V3.5 commands. The examples below establish only the exact forms shown in those contexts. They do not promote all Chapter 7 rules from the base manual to 3.5.

A page-image-checked example establishes that the shown form is used in that 3.5 context. It does not establish every overload, edge case, error condition, or interpreter/compiler mode. The entries below are documentary evidence, not native-interpreter conformance results or Rust implementation claims.

## Control flow and procedures

| Construct | Form directly witnessed | What the source establishes | Source |
|---|---|---|---|
| Counted loop | `FOR i=1 TO 15 ... NEXT i` | Ascending `TO` form with an explicit named `NEXT`. | `AM-BOOK-DE-1990`, printed p. 471 |
| Counted loop, negative step | `FOR j=i TO 1 STEP -1 ... NEXT j` | The example uses a negative step in a descending loop. | `AM-BOOK-DE-1990`, printed p. 480 |
| Counted loop, DOWNTO | `FOR j%=i% DOWNTO 0 ... NEXT j%` | The 3.5 addendum's DATA-pointer example uses the `DOWNTO` spelling. | `AM-INTERPRETER-35`, printed p. 13-35 |
| WHILE loop | `WHILE _DATA ... WEND` | The companion-book example loops while the DATA pointer is nonzero. | `AM-BOOK-DE-1990`, printed p. 480 |
| Conditional DO loop | `DO WHILE _DATA ... LOOP` | The 3.5 addendum uses the DATA-pointer availability predicate at the DO boundary. | `AM-INTERPRETER-35`, printed pp. 13-34–13-35 |
| Procedure | `PROCEDURE drawp ... RETURN` | A parameterless procedure is defined and terminated with `RETURN`. | `AM-BOOK-DE-1990`, printed p. 471 |
| Procedure call | `@drawp` | The example invokes the procedure with the `@` prefix. | `AM-BOOK-DE-1990`, printed p. 471 |
| Program termination | `END` | The vector example terminates with `END`. | `AM-BOOK-DE-1990`, printed p. 471 |

The procedure example does not establish parameter passing, recursion, nested procedures, or the full set of procedure aliases. Those require separate evidence.

## DATA and file operations

| Construct | Form directly witnessed | What the source establishes | Source |
|---|---|---|---|
| DATA-pointer query | `_DATA` | Returns the current DATA-pointer position; the addendum states that `_DATA` is zero when the next `READ` would run out of data. | `AM-INTERPRETER-35`, printed p. 13-34; `AM-BOOK-DE-1990`, printed p. 480 |
| DATA-pointer restore | `_DATA=dp%(j%)` | Restores a previously saved DATA-pointer value. The companion-book text warns to use a previously read position. | `AM-INTERPRETER-35`, printed p. 13-34; `AM-BOOK-DE-1990`, printed p. 480 |
| DATA read | `READ a` | Reads one value into the shown scalar variable in the DATA-pointer example. | `AM-INTERPRETER-35`, printed p. 13-35; `AM-BOOK-DE-1990`, printed p. 480 |
| DATA reset | `RESTORE` | The companion-book example resets the DATA pointer to the beginning before reading. | `AM-BOOK-DE-1990`, printed p. 480 |
| File open, output | `OPEN "O",#1,"ram:matrizen"` | The matrix example opens a numbered channel in output mode. | `AM-BOOK-DE-1990`, printed p. 458 |
| File open, input | `OPEN "I",#1,"ram:matrizen"` | The same example opens a numbered channel in input mode. | `AM-BOOK-DE-1990`, printed p. 458 |
| File close | `CLOSE #1` | The example closes the numbered channel. | `AM-BOOK-DE-1990`, printed p. 458 |

The file examples do not establish append, update, random-access, error, or resource-lifetime behavior. The DATA examples do not establish every DATA item type, READ target form, or arbitrary pointer assignment.

## Other witnessed statements

| Construct | Form directly witnessed | Boundary | Source |
|---|---|---|---|
| Base selection | `OPTION BASE 0` | Only the shown setting is witnessed here. | `AM-BOOK-DE-1990`, printed p. 458 |
| Declaration | `DIM p(anzahl,dimension)` | A two-dimensional numeric field declaration in the vector example. | `AM-BOOK-DE-1990`, printed p. 471 |
| Delay | `PAUSE 50` | The example uses this delay inside the counted loop; timing units and limits are not specified by this example. | `AM-BOOK-DE-1990`, printed p. 471 |
| Amiga screen position | `LOCATE 1,12` | The example positions screen output before displaying a matrix. | `AM-BOOK-DE-1990`, printed p. 471 |
| Amiga graphics | `COLOR 3`, `DRAW TO ...` | These are Amiga display/graphics operations, not evidence for language-core control-flow rules. | `AM-BOOK-DE-1990`, printed p. 471 |

## GURU error handling in the 3.5 addendum

The addendum documents the following exact forms in its GURU-recovery example:

- `GURU 1` enables the GURU check in the shown form.
- `ONERROR GOSUB ER` selects the error-handling procedure.
- `PROCEDURE ER ... RETURN` defines the handler.

The addendum also states a specific restriction: in compiled version 3.5 programs, `RESUME NEXT` and `RESUME LABEL` cannot be executed when a GURU caused the `ONERROR GOSUB` call. This is a restriction in that stated context; it is not a general statement about every `RESUME` form or interpreter execution.

Source: `AM-INTERPRETER-35`, printed p. 13-36.

## Not established by this page

The following Chapter 7 families are described in the manual's 3.0 base material, but the evidence listed on this page does not establish their 3.5 availability or full 3.5 semantics: `IF/ELSE/ENDIF`, `ELSEIF`, `SELECT/CASE`, `GOTO`, `GOSUB`, `LOCAL`, `FUNCTION/ENDFUNC`, `DEFFN`, `ON BREAK`, `ON ERROR`, `EVERY`, `AFTER`, `EXIT IF`, and the remaining `DO/LOOP` condition placements.

This is an evidence gap, not evidence that these constructs were removed or are incompatible. The page-cited 3.0 inventory is maintained separately in [Chapter 7 program-handling evidence](../research/amiga-gfa-basic-chapter-7-program-handling-evidence.md). The narrow 3.5 example-use audits are [the companion-book statement audit](../research/amiga-gfa-basic-35-example-used-statements-audit.md) and [the interpreter-addendum statement audit](../research/amiga-gfa-basic-35-base-statement-use-audit.md).

## Implementation and test status

This page does not claim that the listed forms are implemented by the Rust compiler or conformant with the original interpreter. Parser and semantic support are tracked separately in [issue #47](https://github.com/vsimko/gfabasic-rust/issues/47) and [issue #48](https://github.com/vsimko/gfabasic-rust/issues/48). The evidence-gated corpus is tracked in [issue #42](https://github.com/vsimko/gfabasic-rust/issues/42); its current fixtures do not yet provide an executable corpus runner or coverage for every statement listed here.

## Related sources and work

- [Amiga GFA BASIC 3.5 primary source notes](../research/amiga-gfa-basic-35-primary-reference.md)
- [Amiga GFA BASIC Chapter 7 program-handling evidence](../research/amiga-gfa-basic-chapter-7-program-handling-evidence.md)
- [Issue #21 — statements and control flow](https://github.com/vsimko/gfabasic-rust/issues/21)
