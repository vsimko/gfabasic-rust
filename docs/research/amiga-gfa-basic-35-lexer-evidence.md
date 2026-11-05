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

### 2.1 Special names from the Chapter 14 addendum

The interpreter manual's Chapter 14 opening list identifies the DATA-pointer operations and special variables as new version 3.5 features. The detailed descriptions and examples establish:

| Spelling | Lexical interpretation | Evidence |
|---|---|---|
| _DATA | Exact special identifier naming the current DATA pointer position | AM-INTERPRETER-35, printed addendum pp. 14-5–14-6 |
| _DATA= | The _DATA identifier followed by assignment operator =; it sets the DATA pointer to a previously saved position | AM-INTERPRETER-35, printed addendum pp. 14-5–14-6 |
| _0 … _9 | Ten exact special variable names; the manual describes them as longword variables | AM-INTERPRETER-35, printed addendum p. 14-6 |

The 3.5 evidence authorizes these exact spellings. It does **not** authorize a general rule that arbitrary identifiers may start with underscore. The lexer must continue to reject unverified forms such as _foo, _10, _DATA2, and case variants unless a 3.5 source or interpreter observation establishes them.

The current narrow lexer recognizes these spellings, preserving their source text and spans. This is a small verified 3.5 increment, not general identifier coverage.

## 3. Inherited candidates from the 3.0 base manual

The following rules are described in the base chapters of AM-INTERPRETER-35, but those pages are documented as GFA-BASIC 3.0. Treat each as **inherited candidate / confirmation needed for 3.5**, not as already verified 3.5 behavior.

### 3.1 Variable and function names

- Base manual: variable names start with an ASCII letter A–Z, followed by letters, digits 0–9, dots (.) or underscores (_) (printed pp. 2-1–2-2).
- The base manual lists type postfixes: Boolean !, Byte | (OCR may render it as capital I), Word &, Integer %, Float #, String $. The 1990 German book also describes these postfixes and says a real-variable # is optional (printed pp. 56–57).
- The German 1990 book says names may be arbitrarily long and comprise A–Z/a–z/0–9, underscore, and dot; variable and function names must begin with a letter (printed p. 56).
- Preserve original spelling. The 3.0 editor uppercases command names and formats variable names, but this is editor behavior, not evidence that a compiler may rewrite source spelling.
- Arrays use parentheses after the name, for example x%(5); array declarations and indexing therefore require parentheses to remain separate tokens from identifiers.

**Implementation consequence:** a future identifier token needs the full spelling, base-name span, and any suffix as part of its source representation. Do not split suffix punctuation into an unrelated operator without a grammar-aware distinction. Do not normalize case in the lossless stream.

**Unresolved:** exact Amiga 3.5 identifier repertoire, maximum length, case sensitivity, non-ASCII handling, Unicode/code-page behavior, and whether # is a mandatory or optional Float suffix.

### 3.2 Labels

- Base manual: a label is made from letters, digits, underscores, and dots; unlike a variable name it may start with a digit; it ends in colon; the colon is omitted when the label is referenced by GOTO (printed p. 7-36).
- German 1990 book: labels may begin with a digit and end in a colon; it also warns that labels which collide with command names can be reinterpreted by the interpreter/editor (printed pp. 56–57).

**Unresolved / source discrepancy:** the German 1990 book uses broader wording (“arbitrary characters”) than the base manual's explicit character set. Neither establishes the exact 3.5 rule. Labels need a distinct lexical/parse context; a colon cannot simply be treated as a generic statement separator.

### 3.3 Comments and remark syntax

- Base manual documents a full-line REM comment and a remark character at the beginning of a line; it also documents an inline comment introduced by ! after a command (printed p. 7-35).
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
- The base manual's DATA section says DATA values may be numeric or string constants; a string containing a comma must be enclosed in quotation marks (printed p. 5-15).
- German 1990 book says text DATA normally need not be quoted; commas delimit successive DATA items, so text containing commas must be quoted. It also says DATA rows may contain arbitrary text when no READ operation uses them (printed pp. 168–169).

**Unresolved:** string escape rules, doubled quotes, embedded quote handling, control characters, exact empty-string syntax, character encoding, and whether unquoted DATA text is consumed by the interpreter with precisely the 1990 book's described rules in 3.5.

**Implementation consequence:** preserve raw literal payload and delimiters. DATA requires a context-sensitive mode; it cannot be lexed as an ordinary comma-separated expression list with comments stripped globally.

### 3.6 Numeric literals

- Base manual's DATA section explicitly permits numeric DATA constants in hexadecimal, octal, and binary forms (printed p. 5-15).
- Base manual's VAL description identifies &H (hex), &X (binary), and &O (octal) prefixes (printed p. 2-15).
- The German 1990 book describes &H hexadecimal notation and says a leading dollar form such as $1AF7 is also accepted/converted by GFA-BASIC (printed p. 34); it also uses &X and &H in examples (printed p. 233).

**Unresolved:** the complete 3.5 numeric-literal grammar, leading/trailing decimal point rules, exponent syntax and sign rules, digit separators, suffix interaction, overflow behavior, and acceptance of the alternate $ hex prefix. OCR samples such as &A, &HA, and %1010 are ambiguous and must not be promoted without page-image verification.

### 3.7 Operators and punctuation candidates

The base operator chapter (printed pp. 3-2–3-16) lists the following operator families:

- Parentheses: ( and ).
- Prefixes: unary + and -.
- Power: the OCR of the hierarchy is imperfect; the source's examples and original page image must be checked before fixing the exact glyph/associativity.
- Multiplicative: * and /.
- Integer arithmetic: DIV and MOD.
- Additive: + and -; string concatenation also uses +.
- Equality and comparisons: =, approximate numeric equality ==, <>, <, >, <=, >=; the manual also accepts =< and => as alternatives to <= and >=.
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

The German 1990 book also says command names are displayed in uppercase by the editor and that the reader should distinguish fixed command names from programmer-defined names (printed pp. 56–58).

**Unresolved:** whether all abbreviation forms are accepted by the 3.5 interpreter when reading source files, exact abbreviation disambiguation, keyword boundary rules, case sensitivity outside the editor, and the complete 3.5 reserved-word/special-variable inventory. A lexer should initially emit identifier/name candidates and let a dialect-specific classification/parser layer resolve keywords by context; do not make all dictionary words globally reserved based only on the editor.

## 5. Evidence matrix and implementation gate

| Area | Current evidence | Status for Amiga 3.5 |
|---|---|---|
| Exact _DATA, _DATA=, _0…_9 | Chapter 14 3.5 addendum, pp. 14-5–14-6 | **Verified 3.5** |
| General identifier grammar and suffixes | Base manual pp. 2-1–2-3; German V3.03 pp. 56–57 | Inherited candidate |
| Labels | Base manual p. 7-36; German V3.03 pp. 56–57 | Inherited candidate; wording discrepancy |
| REM/apostrophe/! comments | Base manual p. 7-35; German V3.03 p. 166 | Inherited candidate |
| One command per physical line | Base editor p. 1-8 | Editor rule; source grammar confirmation needed |
| String delimiters and escapes | Base manual pp. 1-4, 5-7; DATA p. 5-15; German V3.03 pp. 168–169 | Delimiter candidate; escaping unresolved |
| Radix-prefixed numeric constants | Base manual pp. 2-15, 5-15; German V3.03 pp. 34, 233 | Inherited candidate; complete grammar unresolved |
| Operator inventory/precedence | Base manual pp. 3-2–3-16 | Inherited candidate; power glyph and 3.5 confirmation pending |
| Command abbreviations | Base manual editor pp. 1-7–1-8 and appendix pp. 12-8 onward | Editor behavior; interpreter acceptance unresolved |
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

- [ ] Inspect page images for base manual pp. 2-1–2-3, 3-2–3-16, 5-15, 7-35–7-36 and German 1990 pp. 34, 56–58, 166–170; resolve OCR glyphs and capture exact evidence notes.
- [ ] Search the 3.5 addendum and all supplied 3.5-specific pages for any explicit confirmation or change to base lexical rules.
- [ ] Obtain a lawful, reproducible Amiga GFA BASIC 3.5 interpreter probe environment or recorded observations for unresolved cases.
- [ ] Determine source encoding and line-ending behavior independently of the Rust transport loader.
- [ ] Build an evidence-linked lexical test matrix before expanding the production lexer beyond the exact Chapter 14 special names and existing FACT/PRINT vertical slice.
- [ ] Update issue #46 with the source-backed rule matrix and remaining acceptance gaps.

## Source rights

This document contains concise original summaries and bibliographic/page references only. Do not commit, redistribute, or reproduce either supplied PDF, substantial source passages, page images, or historical program listings.
