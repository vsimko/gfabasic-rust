# GFA BASIC reference sources

This register identifies candidate sources for language and API research. A listed source is not automatically authoritative for every dialect or version. No feature should be promoted to “verified” until the relevant edition, platform, and page/section have been checked.

## Evidence status

- **Verified**: the exact source passage and applicability have been checked.
- **Inferred**: the conclusion is plausible but not explicit in the source.
- **Disputed**: sources or observed implementations conflict.
- **Unknown**: evidence has not been checked.

## Candidate Atari ST sources

| ID | Source | Applicability | Evidence status | Notes |
|---|---|---|---|---|
| ST-MANUAL-36 | *GFA BASIC 3 - User Manual*, Version 3 interpreter user manual | Atari ST GFA BASIC 3; exact edition and publication metadata still to be confirmed | Candidate primary manual; not yet page-verified | A publicly accessible scan is indexed by AtariMania: [PDF record](https://www.atarimania.com/st/files/GFA_BASIC_Version_3_Interpreter_User_Manual.pdf). Check title page, edition, version, and relevant pages before citing normative rules. |
| ST-REF-35 | *GFA-BASIC 3.5 command reference* | Atari ST GFA-BASIC 3.5 | Secondary/primary status to be confirmed | [Available PDF](https://info-coach.fr/atari/documents/general/gfabasic.pdf). The indexed introduction identifies it as a command reference for GFA-BASIC 3.5. Verify source provenance and page references. |
| ST-COMPENDIUM | GFA-Basic Compendium / hypertext documentation | Atari ST; exact versions per topic | Secondary reference | [GFA-Basic FAQ and resource site](https://www.gfabasic.net/htm/gfa_faq.htm). Useful for finding references and historical context; do not substitute it for the relevant manual when establishing syntax or semantics. |

## Candidate Amiga sources

No Amiga manual has yet been identified and verified in this initial register. The Atari ST manual must not be treated as sufficient evidence for Amiga-specific commands or behavior.

## Historical/version context

The GFA-Basic FAQ reports separate last-known releases for Atari ST/TT and Amiga, including Atari ST/TT 3.6 and Amiga 3.52. This is secondary historical context, not a language specification: [GFA-Basic FAQ](https://www.gfabasic.net/htm/gfa_faq.htm).

The FAQ also discusses differences among the Atari ST, Amiga, DOS, and Windows versions. These differences reinforce the need to keep dialect/version identity explicit. Confirm every detail against the relevant original manual.

## Citation convention

For every language or API rule, record:

`[source ID], [platform/dialect], [version/edition], p. [page] or §[section]`

Also record a short original explanation, evidence status, and any uncertainty. Do not reproduce substantial copyrighted source text.

## Research tasks

- Confirm the title-page metadata and pagination of each Atari ST manual.
- Identify the exact Amiga GFA BASIC 3.52 manual and its provenance.
- Determine which sources apply to interpreter behavior, compiler behavior, and runtime/API behavior.
- Record discrepancies between manual editions and observed interpreter/compiler behavior.
- Review copyright and redistribution status before adding source files to this repository.
