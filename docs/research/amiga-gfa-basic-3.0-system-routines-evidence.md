---
type: "Research Note"
title: "GFA BASIC 3.0 Amiga — operating-system routine evidence"
---

# GFA BASIC 3.0 Amiga — operating-system routine evidence

Source: *GFA-BASIC 3.0 Amiga*, Chapter 11, “System routines”, printed pages 11-2–11-92 (Interpreter PDF pages 312–402). The chapter introduces direct operating-system calls and says its descriptions are reduced to the most important aspects. It notes that the interpreter intercepts invalid input but leaves error handling to the operating system (GURU). (11-2)

## Documented library groups

The chapter is organized by Amiga library, with the following printed-page ranges from the contents:

| Library group | Printed pages |
|---|---:|
| Exec-Library | 11-3–11-23 |
| Graphics-Library | 11-24–11-53 |
| Workbench-Library | 11-54–11-57 |
| Intuition-Library | 11-58–11-75 |
| DiskFont-Library | 11-76–11-77 |
| Layers-Library | 11-78–11-84 |
| DOS-Library | 11-85–11-92 |

(Contents, PDF page 9)

## Representative routine families evidenced in the chapter

- **Exec:** task, device, memory and list management; the first entry pages include `AbortIO`, `AddDevice`, `AddFreeList` and `AddHead`. (11-3)
- **Graphics:** a separate library group for graphics and character routines, distinct from GFA BASIC's Chapter 8 drawing commands. (11-24–11-53)
- **Workbench:** Workbench integration routines are listed as a separate group. (11-54–11-57)
- **Intuition:** GUI/windowing system routines are grouped separately from the BASIC screen/window command layer. (11-58–11-75)
- **DiskFont:** disk-font routines occupy their own short group. (11-76–11-77)
- **Layers:** layer creation, movement, sizing, locking and synchronization. Examples include `FattenLayerInfo`, `InitLayers`, `LockLayer`, `MoveLayer`, `ScrollLayer`, `SizeLayer`, `UnlockLayer` and `WhichLayer`. (11-79–11-84)
- **DOS:** file/process and directory operations. Examples include `Close`, `CreateDir`, `CurrentDir`, `DateStamp`, `Delay`, `DeleteFile`, `Examine`, `Execute`, `LoadSeg`, `Lock`, `Open`, `Read`, `Rename`, `Seek`, `SetComment`, `SetProtection`, `UnLock` and `Write`. (11-85–11-92)

The examples are representative, not a complete transcription of each library's routine catalogue. The chapter's own table of operating-system functions in Appendix 12 begins at 12-19 and should be used as an index, not as a substitute for each routine's argument/return description.

## Calling convention and risk boundary

Chapter 10 explains that the library-base variables are available to GFA BASIC and that FD files can generate wrappers which marshal parameters into register slots before `RCALL` (10-4, 10-10–10-12). Chapter 11 documents OS routines and their parameters, but does not establish that arbitrary calls are safe: the introductory note explicitly mentions OS-level error handling and GURU failures (11-2).

This is Amiga-specific API evidence from the 3.0 base manual. It does not define a host-independent runtime API, Linux compatibility, or what the Rust project has implemented. Exact routine signatures must be transcribed from the relevant routine entries before use in an implementation.
