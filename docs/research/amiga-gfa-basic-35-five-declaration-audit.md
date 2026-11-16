---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — five declaration-level audit blocks"
---

# Amiga GFA BASIC 3.5 — five declaration-level audit blocks

Primary source: supplied *GFA BASIC 3.5 Interpreter*, Chapter 13 addendum, printed pp. 13-33–13-36. The independently supplied combined update scan numbers the same material Chapter 14, printed pp. 14-1–14-7. The entries below are source-backed 3.5 declarations, not runtime-conformance claims.

## 1. FACT

- **Context:** expression function.
- **Source form:** `FACT(n)`; assignment example `x=FACT(n)`.
- **Argument:** integer expression `n`.
- **Result:** arithmetic expression representing factorial `n!`; the source explicitly defines `0! = 1`.
- **Version:** explicitly added in Amiga interpreter 3.5.
- **Source:** printed pp. 13-33–13-34 (469-page interpreter scan); pp. 14-1–14-2 (combined update scan).
- **Unspecified:** accepted range, overflow/precision behavior, negative input handling, exact result numeric type, and error code.

## 2. VARIAT

- **Context:** expression function.
- **Source form:** `VARIAT(n,k)`; assignment example `y=VARIAT(n,k)`.
- **Arguments:** integer expressions `n` and `k`, in that order.
- **Result:** number of ordered selections of `k` elements from `n` without repetition, defined as `n!/(n-k)!`.
- **Version:** explicitly added in Amiga interpreter 3.5.
- **Source:** printed pp. 13-33–13-34; combined update pp. 14-1–14-2.
- **Unspecified:** valid argument domain, behavior for `k>n`, negative values, overflow/precision, result type, and error code.

## 3. COMBIN

- **Context:** expression function.
- **Source form:** `COMBIN(n,k)`; assignment example `z=COMBIN(n,k)`.
- **Arguments:** integer expressions `n` and `k`, in that order.
- **Result:** number of unordered selections of `k` elements from `n` without repetition, defined as `n!/((n-k)!*k!)`.
- **Version:** explicitly added in Amiga interpreter 3.5.
- **Source:** printed p. 13-34; combined update p. 14-2.
- **Unspecified:** valid argument domain, behavior for `k>n`, negative values, overflow/precision, result type, and error code.

## 4. OPENW negative screen-number variant

- **Context:** Amiga window-opening statement variant, not a separate portable language-core feature.
- **Source form:** the addendum specifies the case where the screen number supplied to `OPENW` is negative.
- **Effect:** opens the window on the current top screen.
- **Version:** explicitly added in Amiga interpreter 3.5.
- **Source:** printed p. 13-34; combined update p. 14-3.
- **Unspecified:** exact accepted negative values, whether all negatives are equivalent, interaction with other OPENW flags/IDCMP arguments, and failure/error behavior. The full base OPENW signature remains a separate inherited 3.0 candidate.

## 5. PCOLOR

- **Context:** screen-output color statement.
- **Source form:** `PCOLOR foreground[,background]`.
- **Arguments:** foreground color; optional background color.
- **Effect:** like `COLOR`, but applies to screen output produced by `PRINT`.
- **Version:** explicitly added in Amiga interpreter 3.5.
- **Source:** printed p. 13-34; combined update p. 14-4.
- **Unspecified:** numeric argument range, omitted-background behavior beyond examples, persistence/reset rules, interaction with windows/screens, and error behavior.

## Compatibility and evidence boundary

All five blocks above are **confirmed as documented 3.5 declarations/variants** because they occur in the version-specific addendum. This status confirms source availability only; it does not assert that the Rust implementation supports them or that every edge case is runtime-conformant.

Do not fill the listed gaps from Atari GFA BASIC, another Amiga release, or generic BASIC conventions. Add controlled interpreter observations only when a legally obtained, version-identified 3.5 interpreter is available. The existing addendum inventory remains the broader source of truth for all other 3.5 additions.
