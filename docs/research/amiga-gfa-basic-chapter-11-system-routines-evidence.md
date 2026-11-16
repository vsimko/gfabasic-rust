---
type: "Research Note"
title: "Amiga GFA BASIC Chapter 11 — system-routine evidence"
---

# Amiga GFA BASIC Chapter 11 — system-routine evidence

## Scope and version boundary

Primary source: supplied *GFA-BASIC 3.5 Interpreter* PDF, Chapter 11, “System calls”, printed pages 11-2–11-92 (PDF pages 312–402). The manual states that the interpreter permits direct calls to operating-system routines, automatically formats/corrects their syntax and indentation, and reduces descriptions to the most important aspects. It warns that invalid input is intercepted by the interpreter but error handling is left to the operating system (GURU). It also says that system functions with BASIC-command equivalents are performed using the operating-system routine (printed p. 11-2).

The supplied manual identifies Chapters 1–12 as original 3.0 material and Chapter 13 as the 3.5 addendum. Chapter 11 is therefore **3.0 base-manual evidence**, not independent proof that every listed routine is supported by 3.5.

Printed page labels below are primary locators; PDF viewer page numbers are secondary.

## Chapter 11 inventory

| Area | Construct / source spelling | Source-described behavior and constraints | Printed pages |
|---|---|---|---:|
| Exec library | Exec-Library routines: `AllocMem`, `AvailMem`, `FreeMem`, `AllocSignal`, `FindTask`, `SetTaskPri`, `Forbid`, `Permit`, `OpenLibrary`, `CloseLibrary`, message-port and device calls | Low-level task, signal, memory, library, device, interrupt, message-port and linked-list operations. Parameters are generally integer expressions representing system-structure addresses; return values vary by routine. | 11-3–11-23 |
| Graphics library | Graphics-Library routines: `AllocRaster`, `Draw`, `DrawCircle`, `Flood`, `LoadRGB4`, `LoadView`, `Move`, `RectFill`, `SetAPen`, `SetBPen`, `SetDrMd`, `SetFont`, `SetRGB4`, `Text`, `WritePixel` | Amiga raster, viewport, color-map, drawing, text, sprite/gel, blitter and region operations. These are direct OS graphics calls, distinct from GFA BASIC's Chapter 8 graphics statements. | 11-24–11-53 |
| Workbench library | Workbench-Library routines: `BumpRevision`, `CloseWorkBench`, `FindToolTypes`, `FreeDiskObject`, `GetDiskObject`, `GetIcon`, `OpenWorkBench`, `PutDiskObject`, `PutWBObject`, `WBenchToBack`, `WBenchToFront` | Workbench integration, icon/disk-object and tool-type operations. The short group is not a general replacement for the BASIC file or window command layers. | 11-54–11-57 |
| Intuition library | Intuition-Library routines: `ActivateWindow`, `AddGadget`, `BeginRefresh`, `ClearMenuStrip`, `CloseWindow`, `DisplayBeep`, `EndRequest`, `ModifyProp`, `OnMenu`, `OpenScreen`, `OpenWindow`, `RefreshWindowFrame`, `SetWindowTitles`, `WindowToBack`, `WindowToFront` | GUI screens, windows, gadgets, requesters, menus, pointer and event/message handling. Calls use Intuition structure addresses and are distinct from GFA BASIC's higher-level Chapter 9/10 commands. | 11-58–11-75 |
| DiskFont library | DiskFont-Library routines: `AvailFonts`, `OpenFont` | Disk-font discovery and opening font structures. The manual gives this library a separate, short section. | 11-76–11-77 |
| Layers library | Layers-Library routines: `AttemptLockLayerRom`, `CreateUpfrontLayer`, `FattenLayerInfo`, `InitLayers`, `LockLayer`, `MoveLayer`, `ScrollLayer`, `SizeLayer`, `UnlockLayer`, `WhichLayer` | Layer creation, locking, movement, scrolling, sizing and layer-information management. The text includes ROM-specific lock variants; do not collapse them into one generic lock operation. | 11-78–11-84 |
| DOS library | DOS-Library routines: `Close`, `CreateDir`, `CurrentDir`, `DateStamp`, `Delay`, `DeleteFile`, `Execute`, `Examine`, `ExNext`, `Exit`, `Info`, `IoErr`, `LoadSeg`, `Lock`, `Open`, `Read`, `Rename`, `Seek`, `SetComment`, `SetProtection`, `UnLoadSeg`, `UnLock`, `WaitForChar`, `Write` | AmigaDOS file handles, byte I/O, directory traversal, locks, metadata, process/CLI execution and segment loading. The source describes FileHandle, Lock and FileInfoBlock addresses and documents return values per routine. | 11-85–11-92 |

The names above are a representative family inventory, not a complete transcription of every routine entry or signature. The chapter's separate operating-system function table in Appendix 12 (beginning at printed p. 12-19) is an index, not a substitute for the per-routine parameter descriptions.

## 3.5 addendum reconciliation

The 3.5 addendum inventory does not establish the Chapter 11 library catalogue as a new 3.5 feature set. Chapter 13 separately documents the `GURU` statement and `GURU` function (printed pp. 13-36–13-37 in the supplied interpreter manual); those are 3.5 evidence and must not be confused with the Chapter 11 library calls or treated as confirmation of all 3.0 system routines.

No Chapter 11 library routine should be promoted to confirmed 3.5 support solely because it appears in this base chapter. Absence from the addendum is an evidence gap, not proof of runtime absence.

## 3.52 runtime evidence

The repository's tracked native 3.52 regression batch contains `fact-zero`, `arithmetic`, `for-next` and `strings`. These probes do not exercise direct Exec, Graphics, Workbench, Intuition, DiskFont, Layers or DOS library calls. No retained 3.52 runtime observation for this Chapter 11 API surface is currently recorded.

## Classification and safety boundary

- Keep these routines in an Amiga OS API namespace, separate from BASIC language keywords, statements and built-in functions.
- Treat structure pointers, register-level calling conventions, NUL-terminated OS strings and OS-owned resources as ABI concerns; this chapter's abbreviated descriptions are not a complete ABI specification.
- The Chapter 10 `_DosBase`, `_IntBase`, `_GfxBase` and `_LayersBase` variables and FD/`RCALL` wrappers provide related calling context (printed pp. 10-4 and 10-10–10-12), not evidence that arbitrary calls are safe.
- Preserve the manual's explicit GURU/error-handling warning. Do not infer host-independent or Linux behavior from Amiga library calls.
- Verify exact signatures and structure layouts against the relevant Amiga system documentation and controlled interpreter observations before implementation.

## Related repository evidence

- [Existing Chapter 11 system-routine summary](amiga-gfa-basic-3.0-system-routines-evidence.md)
- [Chapter 10 advanced programming evidence](amiga-gfa-basic-chapter-10-advanced-programming-evidence.md)
- [Amiga 3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md)
- [German book command and Amiga OS API extraction](amiga-gfa-basic-german-book-command-api-extraction.md)
- [Native 3.52 regression batch](../../tests/interpreter-probes/amiga-3.52/regression-suite.LST)
- [Issue #42 — reference and conformance corpus](https://github.com/vsimko/gfabasic-rust/issues/42)
- [Issue #307 — evidence-based compatibility metrics](https://github.com/vsimko/gfabasic-rust/issues/307)
