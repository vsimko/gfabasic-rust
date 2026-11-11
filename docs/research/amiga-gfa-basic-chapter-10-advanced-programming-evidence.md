---
type: "Research Note"
title: "Amiga GFA BASIC Chapter 10 — advanced programming evidence"
---

# Amiga GFA BASIC Chapter 10 — advanced programming evidence

## Scope and version boundary

Primary source: supplied *GFA-BASIC 3.5 Interpreter* PDF, Chapter 10, “Advanced programming”, printed pages 10-2–10-16 (PDF pages 296–310). The manual explicitly states that it was originally printed for version 3.0 and that Chapter 13 is an addendum describing features provided in version 3.5. Chapter 10 is therefore **3.0 base-manual evidence** unless a separate 3.5 source confirms a particular feature.

Printed page labels below are the citations; PDF viewer page numbers are secondary locators only.

## Chapter 10 inventory

| Area | Construct / source spelling | Source-described behavior and constraints | Printed pages |
|---|---|---|---:|
| CLI startup and arguments | CLI invocation, `_dosCmd$` | GFA BASIC can be started from CLI with a program name, optionally prefixed by a dash to load without executing. Additional command characters are exposed through `_dosCmd$`; the text also describes passing a Workbench parameter file. | 10-2 |
| Process creation | `EXEC command$,in,out` | Starts a CLI process. The input/output channel arguments select standard channels; -1 retains the current channel. Examples distinguish a process that blocks until completion from a parallel process. | 10-3 |
| Library bases | `_DosBase`, `_IntBase`, `_GfxBase`, `_LayersBase` | Predefined addresses for DOS, Intuition, Graphics and Layers library bases. The manual says these libraries are already resident and can be called directly; Chapter 11 documents their routines. | 10-4 |
| Hardware sprites | `SPRITE` | Defines sprite bitmap data, releases a sprite, moves it, and globally enables/disables sprites. The source describes eight sprite numbers (0–7), sprite 0's mouse assignment, 16-pixel width, bitmap row layout and paired color registers. | 10-5–10-6 |
| Joystick input | `STICK(p)`, `STRIG(p)` | Reads joystick position and fire-button state for ports 0 and 1. Port 0 is integrated with Intuition mouse handling. The text notes that the compatibility `STICK` command has no effect. | 10-7–10-8 |
| Speech output | `SAY`, `TRANSLATE$()` | `SAY` articulates Amiga phoneme codes; `TRANSLATE$()` converts English text to phoneme codes. Requires `narrator.device` and `translator.library`; a parameter array controls voice characteristics. The source warns that translation is often incorrect for languages other than English. | 10-9 |
| FD procedure definitions | FD files, `MERGE`, `RCALL` | Explains converting supplied Amiga GFA procedure-definition files into listings that can be included with `MERGE`. The example maps library bases, register arguments and library offsets into an `RCALL` wrapper. | 10-10–10-12 |
| Blitter objects | BOBs (blitter objects) | Describes BOB display/use and compatibility with Amiga BASIC OBJECT commands. Explicit exceptions: `COLLISION ON/OFF/STOP` are unimplemented, and `COLLISION(-1)` does not return the window number. | 10-13 |
| Object control | `OBJECT.AX`, `OBJECT.AY`, `OBJECT.CLIP`, `OBJECT.CLOSE`, `OBJECT.ON/OFF` | Sets acceleration and clipping; deletes objects; enables/disables selected or all active BOBs. Object numbers are documented as 0–16. | 10-14–10-15 |
| Object appearance and motion | `OBJECT.PLANES`, `OBJECT.PRIORITY`, `OBJECT.SHAPE`, `OBJECT.START/STOP`, `OBJECT.VX/VY`, `OBJECT.X/Y` | Controls bitplane masks, draw priority, object shape/copy, movement, velocity and position. Inquiry functions return position, acceleration and velocity. | 10-15–10-16 |
| Collision handler | `ON COLLISION GOSUB` | Names a procedure invoked after a collision. This handler is distinct from the explicitly unimplemented collision-control commands listed in the BOB introduction. | 10-16 |

## 3.5 addendum reconciliation

The repository's 3.5 addendum inventory does not currently establish Chapter 10's CLI, sprite, joystick, speech, FD-file or BOB/OBJECT features as 3.5 additions. This is an evidence gap, not proof that those features are absent from the 3.5 interpreter. Do not promote a Chapter 10 row to 3.5 support based solely on its presence in this base chapter.

## 3.52 runtime evidence

The tracked native 3.52 regression batch currently contains `fact-zero`, `arithmetic`, `for-next` and `strings`. None exercises Chapter 10 CLI/process integration, sprites, joystick input, speech, FD wrappers or BOB/OBJECT behavior. No retained 3.52 runtime observation for these Chapter 10 areas is currently recorded in the repository.

## Classification and follow-up

- Treat CLI startup, process launch, library bases and FD wrappers as Amiga platform integration, not portable language-core semantics.
- Treat `SPRITE`, joystick input, speech devices/libraries and BOBs as Amiga-specific runtime facilities.
- Preserve the source's explicit collision exceptions; do not summarize BOB compatibility as complete Amiga BASIC OBJECT compatibility.
- Reconcile each item against explicit 3.5 source material or a controlled 3.5 interpreter observation before assigning target-version status.
- No Chapter 10 row should affect the final 3.5 denominator or implementation numerator solely because it appears in this base chapter.

## Related repository evidence

- [Existing Chapter 10 topic summary](amiga-gfa-basic-3.0-advanced-programming-evidence.md)
- [Amiga 3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md)
- [Amiga 3.5 declaration registry](../../api/registry/amiga/special-and-platform-commands.toml)
- [Native 3.52 regression batch](../../tests/interpreter-probes/amiga-3.52/regression-suite.LST)
- [Issue #42 — reference and conformance corpus](https://github.com/vsimko/gfabasic-rust/issues/42)
- [Issue #307 — evidence-based compatibility metrics](https://github.com/vsimko/gfabasic-rust/issues/307)
