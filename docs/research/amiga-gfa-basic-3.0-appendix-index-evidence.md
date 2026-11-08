# GFA BASIC 3.0 Amiga — appendix and command-index evidence

Source: *GFA-BASIC 3.0 Amiga*, Chapter 12, “Appendices”, printed pages 12-2–12-19 (Interpreter PDF pages 404 onward). This is supporting index/reference material for the 3.0 baseline.

## Appendix contents

The contents page lists:
- Keyboard scan codes (12-2)
- ASCII table (12-3)
- Special ASCII codes (12-4–12-5)
- Fill-in patterns (12-6)
- AmigaDOS error messages (12-7)
- Table of commands (12-8 onward)
- Table of operating-system functions (12-19 onward)

## Fill patterns and error numbers

Appendix 12-6 explains that the numbered fill patterns are used by `DEFFILL`; the first number identifies the style and the second the pattern. This is a cross-reference for Chapter 8's fill API, not an independent command definition. (12-6)

Appendix 12-7 lists AmigaDOS error numbers and descriptions, including insufficient free store (103), task table full (104), object already exists (203), object not found (205), disk write protected (214), seek error (219), disk full (221), and no disk in drive (226). These are AmigaDOS-specific error references. (12-7)

## Indexes and their limits

The command table provides command/function names, abbreviations and manual page references. It is useful for discovering spellings and locating the detailed entry; it should not be treated as a definitive 3.5 inventory because the book explicitly separates the original 3.0 manual from its Chapter 13 addendum.

The operating-system function table is a separate index from the BASIC command table. It points into Chapter 11's library-specific system routine descriptions. The existence of an index entry does not by itself provide complete calling semantics, argument types, register mapping or error behavior; those details belong to the corresponding Chapter 11 entry.

## Version boundary

The title-page/contents note says the manual was originally printed for version 3.0 and that Chapter 13 describes additional features provided in version 3.5. Therefore Chapters 8–12 and their indexes are treated here as 3.0 source evidence. Do not silently relabel their contents as verified 3.5 behavior. Use Chapter 13 when a 3.5 addition is explicitly documented, and retain unresolved version status where the addendum is silent.
