# Amiga GFA BASIC 3.5 lexer evidence inventory

Status: **research baseline; not a complete 3.5 lexical specification**  
Tracking: [issue #46](https://github.com/vsimko/gfabasic-rust/issues/46)  
Normative target: Commodore Amiga GFA BASIC interpreter **3.5 only** (ADR 0009).

This document records original findings from the two supplied PDFs. It separates explicit 3.5 addendum evidence from syntax described only in the 3.0 base manual or the 1990 V3.03 book. A rule in the latter categories is not automatically accepted by the 3.5 lexer.

## 1. Sources and version boundaries

| Source ID | Source | Scope | Use for lexer |
|---|---|---|---|
| AM-INTERPRETER-35 | *GFA BASIC 3.5 Interpreter*, supplied PDF | Included software is 3.5; the base text was originally printed for 3.0, with explicit 3.5 addenda | Primary. Only explicitly marked 3.5 update material is immediately normative; base rules are inherited candidates pending confirmation |
| AM-BOOK-DE-1990 | Bleek, Hecht, Litzkendorf, *Das große GFA-BASIC-Buch zum Amiga*, DATA BECKER, 1990, ISBN 3-89011-399-0 | Cover says “Bis Version 3.5”, but the foreword identifies the main interpreter reference as V3.03 and discusses compiler material separately | Supplementary Amiga evidence, not proof of 3.5 interpreter behavior |

The German 1990 book is not a substitute for the interpreter manual's 3.5 addendum. It is useful for discovering constructs and cross-checking older Amiga syntax. Its OCR must be checked against page images before ambiguous glyphs become requirements. Neither PDF is copied into this repository.

## 2. Explicitly versioned 3.5 lexical evidence

### 2.1 Special names in the 3.5 addendum

The supplied 469-page `GFA_Basic_3.5_Interpreter.pdf` explicitly identifies Chapter 13 as its 3.5 addendum. Its printed pp. 13-34–13-35 document the DATA-pointer operations and special variables. A separately catalogued combined 3.0 + 3.5 update scan places corresponding material in Chapter 14, printed pp. 14-5–14-6; these are alternate pagination witnesses, not conflicting language rules. The detailed descriptions and examples establish:

| Spelling | Lexical interpretation | Evidence |
|---|---|---|
| _DATA | Exact special identifier naming the current DATA pointer position | AM-INTERPRETER-35, printed addendum p. 13-34 (combined update scan: pp. 14-5–14-6) |
| _DATA= | The _DATA identifier followed by assignment operator =; it sets the DATA pointer to a previously saved position | AM-INTERPRETER-35, printed addendum p. 13-34 (combined update scan: pp. 14-5–14-6) |
| _0 … _9 | Ten exact special variable names; the manual describes them as longword variables | AM-INTERPRETER-35, printed addendum p. 13-35 (combined update scan: p. 14-6) |

The 3.5 evidence authorizes these exact spellings. It does **not** authorize a general rule that arbitrary identifiers may start with underscore. The lexer must continue to reject unverified forms such as _foo, _10, _DATA2, and case variants unless a 3.5 source or interpreter observation establishes them.

The current narrow lexer recognizes these spellings, preserving their source text and spans. This is a small verified 3.5 increment, not general identifier coverage.

### 2.2 Page-image verification log

The following page images were inspected from the supplied PDFs, rather than relying only on OCR text:

- **AM-INTERPRETER-35, printed p. 13-34:** the page visibly prints `_DATA` and `_DATA=` as separate operation forms. The prose says `_DATA` specifies the DATA-pointer position and `_DATA=` sets that pointer to a previously determined value. Lexically, `_DATA=` is therefore the `_DATA` identifier followed by the assignment token `=`; it is not one indivisible identifier token.
- **AM-INTERPRETER-35, printed p. 13-35:** the heading visibly reads `_0..._9`; the prose describes ten special variables and says they are longwords like `%`, not floating-point numbers. This confirms the exact ten spellings and their special-variable status, but not a general underscore-initial identifier rule.
- **AM-INTERPRETER-35, printed p. 13-33:** the 3.5 addendum shows the signatures `x=FACT(n)`, `y=VARIAT(n,k)`, and `z=COMBIN(n,k)` with sample variables `x`, `y`, and `z`. These examples demonstrate the three new intrinsic forms, but do not define general identifier syntax or case sensitivity. The page's running header reads `GFA-BASIC 3.5 Addendum`, its heading is numbered `14. Further new commands in Version 3.5`, and its footer remains `13-33`; this explains why the separately catalogued combined scan uses a distinct `14-x` pagination sequence for this material.
- **AM-INTERPRETER-35, printed p. 13-36:** the addendum distinguishes the `GURU x` command from the `GURU` function (used inside `HEX$(GURU)`). This is a context distinction in a 3.5 command/function example, not a general reserved-word list.
- **AM-INTERPRETER-35, printed pp. 2-2–2-3:** the variable-name rule is explicitly labelled GFA-BASIC 3.0. The Byte postfix is visibly a vertical bar `|`, not capital `I`. The type table lists Float `#`, but the prose on p. 2-3 says Float has no postfix; this is an internal source contradiction and must remain unresolved. The prose confirms `$` for String.
- **AM-INTERPRETER-35, printed p. 3-2:** the operator hierarchy image clearly shows `^` as the power sign. This resolves the OCR ambiguity in the glyph, but the hierarchy is still a 3.0 candidate, not a 3.5 conformance rule.
- **AM-INTERPRETER-35, printed pp. 3-13–3-14 and 3-16:** page images confirm `==` as approximate numeric equality, `<=`/`>=` and their alternate spellings `=<`/`=>`, and `<>` as not-equal. These are inherited 3.0 operator spellings only.
- **AM-INTERPRETER-35, printed p. 5-15:** the DATA entry explicitly gives `DATA const [,const1,const2,...]`, says constants may be numeric or string, and says numeric values may be hexadecimal, octal, or binary. It requires a string containing a comma to be enclosed in quotation marks. The same page independently gives the label character set (letters, digits, underscore, period), permits a leading digit, and requires a final colon.
- **AM-INTERPRETER-35, printed p. 5-16:** the DATA example visibly contains `DATA 10,&A,&A,&HA,&012,&X1010,%1010`. Its accompanying prose says HEX values can be read using `$` before the number and BIN values using `%`. Because these descriptions and example spellings do not align cleanly, the exact direct-source numeric prefix grammar remains unresolved.
- **AM-INTERPRETER-35, printed p. 2-16:** `VAL(a$)` converts a string to a number; the text says `&H`, `&X`, and `&O` prefixes select hexadecimal, binary, and octal interpretation. This documents VAL's string conversion behavior, not by itself the direct numeric-literal grammar.
- **AM-INTERPRETER-35, printed pp. 7-35–7-36:** page images confirm `REM x`, apostrophe-prefixed remark lines, and `<command line> ! x`; the manual says inline `!` is not allowed with DATA instructions. The GOTO entry repeats the label character set and states that the colon is omitted in the GOTO reference. These are still base-manual 3.0 rules.
- **AM-BOOK-DE-1990, printed p. 57:** the page gives labels `Data_label:`, `Xyz_label.1:` (with a following `!Kommentar`), and `1:`; it says a label stands alone, may have a comment, and ends in a colon. The same page says variable/function names may be arbitrarily long, use A–Z/a–z/0–9 plus underscore and dot, and must begin with a letter. It lists type endings `%`, a byte marker printed as a vertical stroke-like glyph, `&`, `!`, and `$`, with `#` optional for real variables. These are V3.03 supplementary observations, not 3.5 acceptance rules.
- **AM-BOOK-DE-1990, printed p. 168:** the DATA section describes comma-separated value/text entries; text DATA need not normally be quoted, while a comma inside text would otherwise be interpreted as the next-entry separator. It also says arbitrary text may occur in DATA lines if the program contains no READ statement, in which case the text is ignored like REM. This is a V3.03 description and does not establish 3.5 DATA lexing.
- **AM-BOOK-DE-1990, printed p. 169:** the REM entry lists `REM`, the high comma/apostrophe shorthand, and `!`. It describes `!` as usable inside a command line and says that at the beginning of a line it is converted to a REM line. This is supplementary V3.03 evidence only; it does not resolve the 3.5 comment grammar.

The image review corrects two OCR-derived ambiguities in the earlier inventory: the broad wording about “arbitrary characters” on p. 57 applies to variable/function names, not labels; and the comment reference is printed p. 169 in this supplied book scan (not p. 166). The byte suffix glyph on p. 57 remains visually ambiguous at this scan resolution and is not promoted to a token rule.

The two remaining page images in the supplied manual's printed 3.5 addendum sequence (pp. 13-33 and 13-36) were also checked. They add examples of version-specific command/function forms but no further general lexical contract. This does not amount to a full review of every page in the manual.

Additional German-book page-image checks:

- **AM-BOOK-DE-1990, printed p. 35:** the page says general hexadecimal notation uses a leading `$` (example `$1AF7`), while GFA-BASIC also accepts `&H` (example `&H1AF7`) and converts the dollar spelling to its own syntax. This is a V3.03 book statement, not direct evidence for the 3.5 source lexer.
- **AM-BOOK-DE-1990, printed p. 56:** the book says Amiga GFA-BASIC allows command names as variable names and has no reserved terms apart from reserved variables such as `TIMER` and `DATE$`. This is useful evidence against globally reserving every command word in a future contextual parser, but remains V3.03-only.
- **AM-BOOK-DE-1990, printed p. 58:** the book says a name after `@` at the beginning of a line is a procedure call, while `@` before a name following an equality sign is a function call. It also confirms the editor displays command names in uppercase. The `@` interpretation is context-sensitive and must not be flattened into a universal prefix classification.
- **AM-BOOK-DE-1990, printed p. 167:** the page gives `Befehlszeile ! Kommentartext`, says `!` has the same commenting role as REM, and explicitly says it cannot be used this way on DATA lines because it would be treated as text DATA. At the beginning of a line, `!` is converted to the apostrophe REM abbreviation. This corroborates the separate REM entry on p. 169.
- **AM-BOOK-DE-1990, printed p. 170:** the RESTORE entry uses `RESTORE [Labelname]` and describes resetting the DATA pointer to the first DATA item following that label. This confirms the use of a label reference without the trailing colon in this V3.03 example; it does not add a new label-character rule.

## 3. Inherited candidates from the 3.0 base manual

The following rules are described in the base chapters of AM-INTERPRETER-35, but those pages are documented as GFA-BASIC 3.0. Treat each as **inherited candidate / confirmation needed for 3.5**, not as already verified 3.5 behavior.

### 3.1 Variable and function names

- Base manual: variable names start with an ASCII letter A–Z, followed by letters, digits 0–9, dots (.) or underscores (_) (printed pp. 2-1–2-2).
- The base manual's p. 2-2 type table lists Boolean `!`, Byte `|`, Word `&`, Integer `%`, Float `#`, and String `$`. Its prose on p. 2-3 contradicts the table by saying Float has no postfix. The German 1990 book says the real-variable `#` is optional (printed p. 57). Keep the Float suffix disputed; do not treat the table alone as a settled rule.
- The German 1990 book says names may be arbitrarily long and comprise A–Z/a–z/0–9, underscore, and dot; variable and function names must begin with a letter (printed p. 56).
- Preserve original spelling. The 3.0 editor uppercases command names and formats variable names, but this is editor behavior, not evidence that a compiler may rewrite source spelling.
- Arrays use parentheses after the name, for example x%(5); array declarations and indexing therefore require parentheses to remain separate tokens from identifiers.

**Implementation consequence:** a future identifier token needs the full spelling, base-name span, and any suffix as part of its source representation. Do not split suffix punctuation into an unrelated operator without a grammar-aware distinction. Do not normalize case in the lossless stream.

**Unresolved:** exact Amiga 3.5 identifier repertoire, maximum length, case sensitivity, non-ASCII handling, Unicode/code-page behavior, and the internally contradictory Float `#` postfix rule in the 3.0 base manual.

### 3.2 Labels

- Base manual: a label is made from letters, digits, underscores, and dots; unlike a variable name it may start with a digit; it ends in colon; the colon is omitted when the label is referenced by GOTO (printed pp. 5-15 and 7-36).
- German 1990 book: labels may begin with a digit and end in a colon; it also warns that labels which collide with command names can be reinterpreted by the interpreter/editor (printed pp. 56–57).

**Unresolved / source discrepancy:** the German 1990 book uses broader wording (“arbitrary characters”) than the base manual's explicit character set. Neither establishes the exact 3.5 rule. Labels need a distinct lexical/parse context; a colon cannot simply be treated as a generic statement separator.

### 3.3 Comments and remark syntax

- Base manual documents `REM x`, apostrophe-prefixed remark lines, and an inline comment introduced by `!` after a command (printed p. 7-35). It explicitly says inline `!` is not allowed with DATA instructions.
- German 1990 book describes the apostrophe/high comma as a shorthand for REM and ! as an inline comment introducer (printed p. 166).
- Both sources state that inline ! is not available on DATA lines, where it would be treated as DATA text.
- German 1990 book says the comment marker is ignored by BASIC and that ! comments may follow a command on the same line (printed p. 166).

**Implementation consequence:** comments must be retained as trivia with exact original bytes and spans. Comment recognition must be aware of string and DATA contexts; blindly truncating at the first ! is incorrect. The apostrophe is context-sensitive: it is used for remark syntax and is also a PRINT output separator in the base manual.

**Unresolved:** exact apostrophe code point(s), alternate remark characters accepted by the editor, case/spacing variants of REM, and exact 3.5 behavior for inline comments.

### 3.4 Physical lines and statement boundaries

- The base manual's editor section says each program line can contain only one command and that remarks may follow the command (printed p. 1-8).
- It describes editor-time syntax checking, abbreviation expansion, indentation, removal of excess spaces, and command/variable name case formatting (printed pp. 1-7–1-8).

These statements describe the historical editor. They do not yet establish the full source-file grammar accepted by the 3.5 interpreter/compiler, nor whether direct-mode input, ASCII MERGE files, and saved binary program files use identical rules.

**Implementation consequence:** preserve every physical line boundary and whitespace span. Do not silently join lines or split a line into multiple statements. Keep CR, LF, and CRLF distinctions until the 3.5 file-format/source-encoding contract is established.

### 3.5 String literals and DATA payloads

- The base manual says string constants are enclosed in quotation marks (printed pp. 1-4 and 5-7).
- The base manual's DATA section says DATA values may be numeric or string constants; numeric values may be hexadecimal, octal, or binary; and a string containing a comma must be enclosed in quotation marks (printed p. 5-15).
- German 1990 book says text DATA normally need not be quoted; commas delimit successive DATA items, so text containing commas must be quoted. It also says DATA rows may contain arbitrary text when no READ operation uses them (printed pp. 168–169).

**Unresolved:** string escape rules, doubled quotes, embedded quote handling, control characters, exact empty-string syntax, character encoding, and whether unquoted DATA text is consumed by the interpreter with precisely the 1990 book's described rules in 3.5.

**Implementation consequence:** preserve raw literal payload and delimiters. DATA requires a context-sensitive mode; it cannot be lexed as an ordinary comma-separated expression list with comments stripped globally.

### 3.6 Numeric literals

- Base manual's DATA section explicitly permits numeric DATA constants in hexadecimal, octal, and binary forms (printed p. 5-15).
- Base manual's `VAL(a$)` description says strings beginning with `&H`, `&X`, or `&O` are interpreted as hexadecimal, binary, or octal (printed p. 2-16). This is evidence about VAL string conversion, not sufficient proof of direct numeric-literal syntax.
- The German 1990 book describes a leading dollar form such as `$1AF7` and the GFA-BASIC `&H` form (printed p. 35); it also uses `&X` and `&H` in examples (printed p. 233). This is V3.03 supplementary evidence only.

**Unresolved:** the complete 3.5 numeric-literal grammar, leading/trailing decimal point rules, exponent syntax and sign rules, digit separators, suffix interaction, overflow behavior, and exact direct-source radix prefixes. The 3.0 DATA example on p. 5-16 shows `&A`, `&HA`, `&012`, `&X1010`, and `%1010`, while its prose mentions `$` for HEX and `%` for BIN; the VAL page separately documents `&H`, `&X`, and `&O` inside strings. Do not collapse these distinct contexts into one lexer rule.

### 3.7 Operators and punctuation candidates

The base operator chapter (printed pp. 3-2–3-16) lists the following operator families:

- Parentheses: ( and ).
- Prefixes: unary + and -.
- Power: `^` (printed operator hierarchy and examples, p. 3-2; page image checked).
- Multiplicative: * and /.
- Integer arithmetic: DIV and MOD.
- Additive: + and -; string concatenation also uses +.
- Equality and comparisons: `=`, approximate numeric equality `==`, not-equal `<>`, `<`, `>`, `<=`, `>=`; the manual also accepts `=<` and `=>` as alternatives to `<=` and `>=` (printed pp. 3-13–3-16; page images checked).
- Logical/bitwise: AND, OR, XOR, IMP, EQV, unary NOT.

The manual documents a precedence hierarchy and says logical operators operate on 32-bit integer values. This list is a 3.0 base candidate pending 3.5 confirmation. Preserve exact operator spelling in tokens; do not canonicalize =< into <= or => into >= in the source stream.

Additional punctuation observed in syntax/examples:
- comma separates arguments, list items, and DATA values;
- semicolon separates PRINT/INPUT forms and can affect output newline behavior;
- apostrophe may be a PRINT separator as well as a remark marker;
- colon terminates labels in the documented forms;
- # prefixes file-channel references in examples and is also listed as a variable type postfix;
- @ prefixes user-defined procedure/function calls in the German 1990 book (printed pp. 57–58);
- ! is a Boolean variable postfix and a comment introducer in different contexts;
- & is a Word variable postfix and part of numeric radix prefixes.

These are lexical candidates, not a complete token contract for Amiga 3.5.

## 4. Abbreviations and reserved words

The base interpreter manual's appendix contains a command/abbreviation table. It gives examples such as P for PRINT, G for GOSUB, GO for GOTO, and multi-character abbreviations such as LI INPUT for LINE INPUT. The editor section says abbreviations are expanded to full command names when a line is accepted (printed pp. 1-7–1-8; appendix pp. 12-8 onward).

The German 1990 book says Amiga GFA-BASIC permits command names as variable names and has no reserved terms apart from reserved variables such as `TIMER` and `DATE$` (printed p. 56). It says the editor displays command names in uppercase (p. 58). It also describes contextual `@` procedure/function calls (p. 58). These are V3.03 observations, not 3.5 keyword rules.

**Unresolved:** whether all abbreviation forms are accepted by the 3.5 interpreter when reading source files, exact abbreviation disambiguation, keyword boundary rules, case sensitivity outside the editor, and the complete 3.5 reserved-word/special-variable inventory. A lexer should initially emit identifier/name candidates and let a dialect-specific classification/parser layer resolve keywords by context; do not make all dictionary words globally reserved based only on the editor.

## 5. Evidence matrix and implementation gate

| Area | Current evidence | Status for Amiga 3.5 |
|---|---|---|
| Exact _DATA, _DATA=, _0…_9 | Chapter 13 3.5 addendum, pp. 13-34–13-35 (combined update scan: Chapter 14, pp. 14-5–14-6) | **Verified 3.5** |
| General identifier grammar and suffixes | Base manual pp. 2-2–2-3 (page images checked; Float `#` contradiction); German V3.03 pp. 56–57 | Inherited candidate; Float suffix disputed |
| Labels | Base manual pp. 5-15 and 7-36 (page images checked); German V3.03 p. 57 (page image checked) | Inherited candidate; complete 3.5 status unresolved |
| REM/apostrophe/! comments | Base manual p. 7-35; German V3.03 p. 169 (page image checked) | Inherited candidate; DATA exclusion explicit in base manual |
| One command per physical line | Base editor p. 1-8 | Editor rule; source grammar confirmation needed |
| String delimiters and escapes | Base manual pp. 1-4, 5-7; DATA p. 5-15; German V3.03 p. 168 (page image checked) | Delimiter candidate; escaping unresolved |
| Radix-prefixed numeric constants | Base manual pp. 2-16, 5-15–5-16 (page images checked); German V3.03 pp. 35, 233 (p. 35 page image checked) | Inherited candidate; direct-source prefixes ambiguous |
| Operator inventory/precedence | Base manual pp. 3-2, 3-13–3-16 (selected page images checked) | Inherited candidate; glyphs confirmed, 3.5 status pending |
| Command abbreviations / keyword reservation | Base manual editor pp. 1-7–1-8 and appendix pp. 12-8 onward; German V3.03 p. 56 (page image checked) | Editor expansion; Amiga V3.03 book says command words can be variables; 3.5 status unresolved |
| Encoding/BOM/non-ASCII | No adequate 3.5 evidence extracted | Unknown |
| Line endings and continuation | No adequate 3.5 source-file evidence extracted | Unknown |
| Complete 3.5 reserved words | Addendum inventory only, not full language catalogue | Unknown |

## 6. Lexer architecture constraints derived from evidence

1. Keep the original source as the authority. Every token and trivia item must retain half-open byte spans into the original input.
2. Preserve all whitespace, comments, physical line endings, original identifier spelling, operator spelling, and literal bytes. The formatter must not need to reconstruct discarded text.
3. Recognize only the explicitly verified 3.5 special names at the underscore prefix. Do not generalize underscore-initial identifiers.
4. Treat comments and DATA as context-sensitive; ! is not a global comment delimiter.
5. Treat identifier suffixes and punctuation contextually. Examples include !, |, &, %, #, $, @, and :.
6. Do not normalize case, whitespace, line endings, alternate comparison spellings, or numeric radix spelling during lexing.
7. Return structured lexical diagnostics for invalid or unverified source rather than silently repairing it.
8. Keep physical source decoding separate from tokenization. The generic UTF-8 source loader does not establish the Amiga source code page.
9. Maintain separate evidence-backed positive and negative tests. Each accepted form must cite the 3.5 passage or a reproducible interpreter observation; inherited candidates must remain quarantined until confirmed.
10. Do not infer parser or semantic acceptance from a token example, editor display, compiler command line, or OCR artifact.

## 7. Next extraction and verification steps

- [x] Inspect page images for primary 3.5 addendum pp. 13-33–13-36; base manual pp. 2-2–2-3, 2-16, 3-2, 3-13–3-16, 5-15–5-16, 7-35–7-36; and German 1990 pp. 35, 56–58, 57, 167–170. Keep all 3.0/V3.03 observations provisional.
- [ ] Inspect remaining page images for base manual operator details not covered by the selected hierarchy/comparison pages and German 1990 pp. 166; resolve remaining OCR glyphs.
- [ ] Search the 3.5 addendum and all supplied 3.5-specific pages for any explicit confirmation or change to base lexical rules.
- [ ] Obtain a lawful, reproducible Amiga GFA BASIC 3.5 interpreter probe environment or recorded observations for unresolved cases.
- [ ] Determine source encoding and line-ending behavior independently of the Rust transport loader.
- [ ] Build an evidence-linked lexical test matrix before expanding the production lexer beyond the exact 3.5 addendum special names and existing FACT/PRINT vertical slice.
- [ ] Update issue #46 with the source-backed rule matrix and remaining acceptance gaps.

## Source rights

This document contains concise original summaries and bibliographic/page references only. Do not commit, redistribute, or reproduce either supplied PDF, substantial source passages, page images, or historical program listings.


## 8. Focused source audit (2026-11-06)

A second retrieval pass over the supplied GFA BASIC 3.5 Interpreter PDF checked the version labels and the passages most likely to be mistaken for 3.5 lexical guarantees. The title page identifies the bundled software as 3.5, but explicitly says the handbook was originally printed for 3.0 and that Chapter 13 is the 3.5 addendum. The following candidate rules remain **unpromoted**:

| Candidate | Passage rechecked | Audit result | 3.5 implementation consequence |
|---|---|---|---|
| Editor case handling and command abbreviation | Base manual printed pp. 1-7–1-8; section repeatedly labels editor as GFA-BASIC 3.0 | Editor expands abbreviations and sets command/variable display case when leaving a line; this describes editor behavior in the 3.0 base text, not a 3.5 file-lexer case rule | Do not case-fold identifiers, rewrite keyword case, or accept abbreviations in source files on this evidence |
| One command per line and comment placement | Base manual printed p. 1-8 and pp. 7-34–7-35 | One command per editor line; REM/remark-line forms and inline ! comments are described in the base text; inline ! is excluded after DATA instructions | Keep physical-line, remark, and inline-comment grammar provisional pending explicit 3.5 confirmation |
| Variable-name repertoire and suffixes | Base manual printed pp. 2-2–2-3 | The variable-type discussion is explicitly a 3.0 rule; the Float # table/prose contradiction remains | No general identifier/suffix acceptance expansion; retain the existing narrow 3.5 special-name exception only |
| String delimiters and payload handling | Base manual printed pp. 1-4 and 5-7; DATA p. 5-15 | Quoted strings and comma-separated output/data examples are base-manual syntax; no 3.5 addendum statement resolving escapes was found in this pass | Do not infer quote escaping, comment stripping inside strings, or DATA payload tokenization |
| Numeric prefixes | Base manual printed pp. 2-16 and 5-15–5-16 | OCR/page-image notes already record conflicting $, %, &H, &X, &O, and &A descriptions/examples; no 3.5 addendum correction found | Do not promote a complete direct numeric-literal prefix grammar |
| Comment forms | Base manual printed pp. 7-34–7-35 | REM, remark-line shorthand, and inline ! are base-manual descriptions; ! has a DATA restriction | Do not implement ! as an unconditional lexer comment delimiter |

The 3.5 addendum review continues to support only the exact special spellings _DATA, _0…_9 as version-marked lexical forms (printed pp. 13-34–13-35). _DATA= is _DATA followed by =, not a combined identifier. The addendum examples for FACT/VARIAT/COMBIN and GURU do not establish general identifier, keyword-reservation, case, comment, or literal rules.

### Regression gate after this audit

The narrow special-name tests remain the only positive underscore-prefix contract: accept exact _DATA and each _0…_9 spelling, preserve original bytes/spans, and tokenize _DATA= as identifier plus assignment. Negative boundary cases must reject _foo, _10, _DATA2, and _data as a whole (never accept a valid prefix and leave a suffix). These are exact-name boundary tests only. Do not add broad positive tests for identifiers, comments, strings, numeric prefixes, or operators until a 3.5 source passage or a reproducible 3.5 interpreter observation supports each case. Existing FACT/PRINT prototype tests remain scoped to their cited addendum examples and are not a general grammar conformance suite.

This audit adds no newly verified general lexical rule and therefore intentionally makes no production lexer behavior change.


## 9. Full Chapter 13 text sweep (2026-11-07)

The complete OCR text of the supplied interpreter manual's Chapter 13 (the explicitly versioned 3.5 addendum, printed pp. 13-1 onward) was extracted and searched, rather than sampling only the last four pages. This is a text-level sweep; OCR-derived spellings below are discovery evidence and should be checked against page images before becoming exact-token regression fixtures.

The sweep adds one useful qualification to the earlier abbreviation assessment:

- **Amiga GFA BASIC 3.5 explicitly documents command abbreviations.** Printed p. 13-1 gives `MAT BASE 0` / `MAT BASE 1` and the abbreviation `m b 0/m b 1`. Printed pp. 13-2–13-3 give abbreviations for `MAT CLR`, `MAT SET`, `MAT ONE`, `MAT READ`, `MAT PRINT`, and `MAT INPUT`; later matrix-operation entries also document abbreviations for `MAT CPY`, `MAT XCPY`, and `MAT TRANS`. Therefore it is no longer accurate to describe all command abbreviations as evidenced only by the 3.0 editor appendix.
- These are **specific command abbreviations in 3.5 documentation**, not proof that the editor's complete 3.0 abbreviation table applies to 3.5 source files, nor proof of abbreviation matching, case folding, token boundaries, or ambiguity resolution. Keep generic abbreviation support unimplemented until those details are established.
- The chapter contains many 3.5 command signatures and examples using matrix names, parentheses, commas, brackets, assignment, numeric arguments, and command names. They are syntax examples for the documented matrix commands; they do not independently define the general identifier, literal, punctuation, or operator grammar.
- The chapter's final pages also contain the already inventoried `_DATA` and `_0`…`_9` special names. No additional general lexical contract for identifiers, comments, strings, numeric literal prefixes, source encoding, or line endings was found in the OCR sweep.

**Evidence matrix correction:** command abbreviations now have a narrow verified 3.5 subset (the explicit MAT abbreviations above), while the full abbreviation mechanism and reserved-word policy remain unresolved. This does not justify enabling abbreviations in the production lexer: abbreviation expansion is likely a command/parser/editor concern and needs a verified source-file rule.

The OCR extraction is not a substitute for page-image inspection. Do not use OCR-normalized punctuation or damaged examples (including apparent decimal/comma substitutions) as lexical fixtures without checking the corresponding printed page image.


## 10. Page-image verification of 3.5 matrix abbreviations (2026-11-09)

The supplied 469-page interpreter manual's printed 3.5 addendum pages 13-1–13-5 (PDF pages 431–435) were visually inspected. This verifies the abbreviation spellings discovered in the Chapter 13 text sweep and fixes OCR-sensitive spacing/punctuation. The abbreviations are printed as short command forms with spaces; they are not evidence for a general lexer-level abbreviation rule.

| Printed page | Full command form | Printed abbreviation | Evidence scope |
|---|---|---|---|
| 13-1 | MAT BASE 0 / MAT BASE 1 | m b 0 / m b 1 | Explicit 3.5 command abbreviation |
| 13-2 | MAT CLR a() | m cl a() | Explicit 3.5 command abbreviation |
| 13-2 | MAT SET a()=x | m se a()=x | Explicit 3.5 command abbreviation |
| 13-2 | MAT ONE a() | m o a() | Explicit 3.5 command abbreviation |
| 13-4 | MAT READ a() | m r a() | Explicit 3.5 command abbreviation |
| 13-4 | MAT PRINT [#i,]a()[,g,n] | m p [#i]a()[g,n] or m ? [#i,]a()[g,n] | Explicit 3.5 alternatives; punctuation/optional-argument notation must be preserved as printed |
| 13-5 | MAT INPUT #i,a() | m i #i,a() | Explicit 3.5 command abbreviation |
| 13-5 | MAT CPY a(...)=b(...)[,h,w] | m c a(i,j)=b(k,l),h,w | Explicit 3.5 command abbreviation |
| 13-5 | MAT XCPY a(...)=b(...)[,h,w] | m x a(i,j)=b(k,l),h,w | Explicit 3.5 command abbreviation |
| 13-5 | MAT TRANS a()[=b()] | m t a()=b() | Explicit 3.5 command abbreviation |

The page images confirm the OCR text for these forms on pp. 13-1–13-5. They do not establish whether abbreviations are accepted in merged .LST source, whether matching is case-insensitive, how whitespace is tokenized, how abbreviations interact with identifiers, or how ambiguous prefixes are resolved. The examples are command-specific and remain a parser/editor concern until source-file acceptance is demonstrated. Do not add generic abbreviation expansion to the lexer from this evidence alone.

The image review is limited to pp. 13-1–13-5; later matrix-command abbreviation entries in Chapter 13 still require their own page-image checks before their exact punctuation is promoted to regression fixtures.


## 11. Page-image verification of later matrix abbreviations (2026-11-09)

The next addendum page images were inspected in the supplied interpreter PDF: printed pp. 13-6–13-15 (PDF pp. 436–445). These pages contain additional explicit abbreviation rows. The following spellings are legible in the page images:

| Printed page | Full command form | Printed abbreviation | Evidence scope |
|---|---|---|---|
| 13-6 | MAT COPY a()=b() | m c a()=b() | Explicit 3.5 command abbreviation; full-matrix copy |
| 13-7 | MAT COPY a(i,j)=b() | m c a(i,j)=b() | Explicit 3.5 command abbreviation; destination offset |
| 13-8 | MAT COPY a()=b(i,j) | m c a()=b(i,j) | Explicit 3.5 command abbreviation; source offset |
| 13-9 | MAT COPY a(i,j)=b(k,l) | m c a(i,j)=b(k,l) | Explicit 3.5 command abbreviation; source and destination offsets |
| 13-10 | MAT COPY a()=b(),h,w | m c a()=b() | Explicit 3.5 command abbreviation; optional dimensions are described in the command syntax |
| 13-11 | MAT XCPY a(i,j)=b(k,l)[,h,w] | m x a(i,j)=b(k,l),h,w | Explicit 3.5 command abbreviation; transposing copy |
| 13-15 | MAT TRANS a() | m t a() | Explicit 3.5 command abbreviation; in-place transpose |

Printed pp. 13-12–13-14 were also checked. They continue the MAT CPY/XCPY/TRANS explanations and examples but do not add another distinct abbreviation row. Page 13-15 repeats the in-place MAT TRANS form and explicitly prints its abbreviation.

The command-specific forms above expand the verified 3.5 abbreviation inventory; they do not establish a general abbreviation algorithm, a minimum prefix length, case-insensitive matching, acceptance in .LST files, or identifier-boundary/ambiguity behavior. In particular, the printed abbreviation m c is reused for several MAT COPY forms with different argument patterns. That is evidence for command-specific syntax, not evidence that the lexer can safely expand m c without parser context.

**Correction to section 10:** the statement that later matrix-command abbreviation entries still require page-image checks is now superseded for the entries on printed pp. 13-6–13-11 and 13-15. The image review now covers the contiguous addendum range pp. 13-1–13-15 (PDF pp. 431–445). No production lexer behavior changes.
