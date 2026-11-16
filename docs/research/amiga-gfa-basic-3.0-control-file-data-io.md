---
type: "Research Note"
title: "Amiga GFA BASIC 3.0 control, file and data I/O evidence"
---

# Amiga GFA BASIC 3.0 control, file and data I/O evidence

> **Version boundary:** This is a page-reviewed extraction from the 3.0 base body of the supplied *GFA BASIC 3.5 Interpreter* scan. The source identifies the base language as GFA BASIC 3.0 and treats the 3.5 update separately. These declarations are inherited 3.0 candidates only; this document does not assert unchanged availability or behavior in interpreter 3.5.

Primary source: `AM-INTERPRETER-35`, *GFA BASIC 3.5 Interpreter*, Chapters 5–7, printed pages as cited below.

## Block 1 — remaining program-control and system commands

### Break and error handling

| Form | Source-described behavior | Printed page |
|---|---|---|
| `ON BREAK` | Enables the normal program-break response to the Control + left Shift + Alt key combination. | 7-28 |
| `ON BREAK CONT` | Suppresses the normal break response. | 7-28 |
| `ON BREAK GOSUB proc` | Installs a procedure called when the key combination is pressed. | 7-28 |
| `ON ERROR` | Restores normal error reporting and program break behavior. | 7-29 |
| `ON ERROR GOSUB proc` | Transfers control to an error-handling procedure when an error occurs. The handler must reinstall this command if it is to handle subsequent errors. | 7-29 |
| `RESUME` | Resumes after the error-handling procedure; only meaningful in that handler. | 7-29–7-30 |
| `RESUME NEXT` | Continues with the command following the command that caused the error. | 7-29–7-30 |
| `RESUME label` | Continues at a named label. | 7-29–7-30 |
| `ERROR x` | Raises the error with number x; useful for testing error handlers. | 7-31 |
| `ERR` | Contains the number of the most recently encountered error. | 7-31 |
| `ERR$(x)` | Returns the interpreter error message for error number x. | 7-31 |

The manual's example demonstrates `ON ERROR GOSUB` followed by `ERROR 5`, then uses `ERR` in the handler and `RESUME NEXT` to continue. The error number/message table is elsewhere in the manual; this extraction does not reproduce or normalize that table.

### Timed procedure calls

- `EVERY ticks GOSUB proc` schedules the parameterless procedure repeatedly at the specified interval.
- `AFTER ticks GOSUB proc` schedules a one-time procedure call after the interval.
- Time is expressed in 1/50-second ticks; the manual describes practical resolution as approximately one tenth of a second, somewhat higher during mouse movement.
- `EVERY STOP` / `EVERY CONT` suspend and resume the recurring schedule.
- `AFTER STOP` / `AFTER CONT` suspend and resume the one-shot schedule.
- The manual cautions that timing checks depend on command execution and can be delayed by slow commands.

Evidence: printed pp. 7-32–7-34. The sample uses `EVERY 4 GOSUB lines` and `AFTER 600 GOSUB text`. The exact scheduling/re-entrancy behavior beyond these descriptions is not specified here.

### Labels, comments and jumps

- `GOTO label` transfers execution to a programmer-defined label. A label may contain letters, digits, underscores and periods, may begin with a digit, and ends with a colon; the colon is omitted from the GOTO operand (p. 7-36).
- The source explicitly says GOTO cannot branch into or out of procedures/functions. It warns that extensive GOTO use quickly makes programs difficult to read.
- `REM` and apostrophe comments ignore the remainder of a line. The source also documents an exclamation mark as an inline-comment introducer after a command, except on DATA instructions (p. 7-35).
- The source says alternate comment characters typed in place of apostrophe are replaced by apostrophe by the editor. This is an editor behavior, not evidence of distinct runtime comment tokens.

### Delay, termination and program lifecycle

| Command | Source-described behavior | Printed page |
|---|---|---|
| `PAUSE x` | Halts execution for x/50 seconds. | 7-37 |
| `DELAY x` | Delays execution for x seconds; the manual notes OS multitasking and practical resolution limitations. | 7-37 |
| `END` | Ends program execution and displays a program-end alert before returning to the editor. | 7-38 |
| `EDIT` | Ends execution and returns directly to the editor. | 7-38 |
| `STOP` | Displays STOP/CONT choices; CONT resumes, STOP enters direct mode where variables can be inspected/changed and execution continued with CONT. | 7-38 |
| `NEW` | Erases the current program; can also be executed from a program. | 7-39 |
| `LOAD f$` | Loads a GFA BASIC program; default extension is .GFA. | 7-40 |
| `SAVE f$` | Saves program data; default extension is .GFA. | 7-41 |
| `PSAVE f$` | Saves a list-protected program that is run when loaded but cannot subsequently be listed. | 7-41 |
| `LIST [f$]` | Lists the current program; with a path, saves the program as ASCII text, default extension .LST. | 7-42 |
| `LLIST` | Sends the listing to the printer. | 7-42 |
| `CHAIN f$` | Loads and executes another GFA BASIC program; default extension is .GFA. | 7-43 |
| `RUN [f$]` | Executes the current program or loads and executes the named program. | 7-44 |
| `SYSTEM [n]`, `QUIT [n]` | Ends the program and exits GFA BASIC, passing an exit value to the caller; zero conventionally means no error. | 7-45 |
| `TRON`, `TRON #n`, `TROFF` | Enables/disables command tracing to the screen or an open channel. | 7-46–7-47 |
| `TRON proc` | Installs a procedure to run before each command; `TRACES` contains the next command. | 7-48 |
| `$` | A compiler-control command treated as REM by the interpreter; details are deferred to the compiler handbook. | 7-49 |

The printed pages 7-50–7-55 cover `INLINE`, `C:` and `RCALL`/system-call mechanisms; these are memory/native-code integration, not ordinary control flow. They are not expanded in this block. Exact behavior and version applicability require separate review before compiler/runtime implementation.

## Block 2 — disk and file organization

### Paths and directory operations

The manual describes a hierarchical file specification consisting of a drive/device, directory path and file name (printed p. 6-2). The following declarations are documented:

| Form | Source-described behavior | Printed page |
|---|---|---|
| `CHDIR name$` | Changes the current directory of the current drive. | 6-4 |
| `DIR$(n)` | Returns the current path for a drive; the manual says n is a dummy parameter. | 6-4 |
| `DFREE(n)` | Returns free disk space for a drive. | 6-4 |
| `DIR p$ [TO name$]` | Displays a directory in a window; optional destination redirects the output. | 6-5 |
| `FILES p$ [TO name$]` | Directory listing variant that displays file names and file sizes. | 6-5 |
| `EXIST(name$)` | Returns true/false according to whether the named file exists. | 6-6 |
| `NAME old$ AS new$` | Renames a file; source says both names must be on the same disk. | 6-10 |
| `RENAME old$ AS new$` | Alias for NAME in the source. | 6-10 |
| `KILL name$` | Deletes the named file. | 6-10 |

The source's directory examples use Amiga device/path syntax. Do not silently substitute host filesystem path rules. The manual does not define a Linux path mapping.

### Opening, closing and file metadata

`OPEN mode$,[#]n,name$ [num]` opens a channel to a file or peripheral (printed p. 6-7). The documented modes are:

- `O`: output (create/write),
- `I`: input (read),
- `A`: append,
- `U`: update,
- `R`: random access.

The channel number is an integer in the documented range 0–99. The optional `#` marker is described as optional for file-channel commands (cross-reference printed p. 1-6). The optional record-length argument is associated with random-access files; the random-access chapter describes records and fields separately.

| Form | Source-described behavior | Printed page |
|---|---|---|
| `LOF(#n)` | Returns file length for an opened channel. | 6-8 |
| `LOC(#n)` | Returns current file position. | 6-8 |
| `EOF(#n)` | Tests end-of-file for an opened channel. | 6-8 |
| `CLOSE [#n]` | Closes one channel or, without a number, all open channels. | 6-8 |
| `TOUCH [#]n` | Updates the file's date/time information. | 6-8 |

### Binary memory/file transfer and file positioning

| Form | Source-described behavior | Printed page |
|---|---|---|
| `BLOAD name$ [,adr]` | Loads a binary file into memory, optionally at an address. | 6-11 |
| `BSAVE name$,adr,num` | Saves a memory region to a binary file. | 6-11 |
| `BGET #n,adr,num` | Reads bytes from an opened channel into memory. | 6-11 |
| `BPUT #n,adr,num` | Writes bytes from memory to an opened channel. | 6-11 |
| `SEEK #n,pos` | Sets the file data pointer to a position. | 6-18 |
| `RELSEEK #n,num` | Moves the file data pointer relatively. | 6-18 |

The manual's examples and descriptions identify memory addresses as integer expressions; elsewhere it warns that addresses require at least a four-byte variable. Exact byte-order, error handling and host portability are not established by this extraction.

## Block 3 — sequential I/O, DATA and random-access records

### Sequential text and byte I/O

| Form | Source-described behavior | Printed page |
|---|---|---|
| `INPUT #n,var1[,var2,...]` | Reads values from a data channel into numeric/string variables. | 6-14 |
| `LINE INPUT #n,a$[,b$,...]` | Reads a line/string from a data channel. | 6-14 |
| `PRINT #n,expression` | Writes numeric/string output to a data channel. | 6-15 |
| `PRINT #n, USING form$,expression` | Writes formatted output. | 6-15 |
| `WRITE #n,expression` | Writes data in a form intended for later INPUT. | 6-15 |
| `INPUT$(num[,#n])` | Reads a specified number of characters from keyboard or an open channel. | 6-13 |
| `INP(#n)` | Reads one byte from an open data channel. | 6-12 |
| `OUT #n,a[,b,c,...]` | Writes byte values to an open data channel. | 6-12 |

The source describes `PRINT` and `WRITE` as distinct output forms: WRITE is intended to create data that can be read back with INPUT. The exact text encoding, newline convention and numeric serialization details are not established here.

### String-array persistence

- `STORE #i,x$()[,n]` writes a string array to a text file, with line-feed separation; the optional n limits the number of elements written (printed pp. 6-16–6-17).
- `RECALL #i,x$()[,n][TO z],x` reads strings from a file into a string array. The manual's syntax and examples use a maximum-count argument and a returned count variable; retain the original printed syntax as a candidate until OCR/page-image review resolves punctuation and optional-argument grouping.
- The example writes 500 array entries, erases/re-dimensions the array, reads the file into another array, and prints the number read (pp. 6-16–6-17).

### Random-access files

The manual introduces random-access files as fixed-length records composed of fields (printed pp. 6-3 and 6-18–6-21).

| Form | Source-described behavior | Printed page |
|---|---|---|
| `FIELD #n,num AS set$[,num AS set$,...]` | Defines named string fields of specified byte lengths within a record. | 6-20 |
| `FIELD #n,num AT(x)[,...]` | Defines fields at explicit offsets. | 6-20 |
| `GET #n[,set]` | Reads a record from a random-access file. | 6-21 |
| `PUT #n[,set]` | Writes a record to a random-access file. | 6-21 |
| `RECORD #n,set` | Selects/sets the record number used by GET/PUT. | 6-21 |

The source's worked example opens a random-access file with mode `R`, assigns field widths totaling 64 bytes, and uses GET/PUT with record numbers. A field is a string variable bound to a segment of a record; it is not an ordinary independent string buffer. The lifetime, aliasing, validation of record size and behavior for malformed/short files require further detailed review.

### DATA statements and in-program data pointers

Chapter 5 documents `DATA`, `READ` and `RESTORE` (printed pp. 5-13–5-14):

- `DATA const[,const1,...]` declares numeric or string constants in the program.
- `READ var[,var1,...]` assigns the next DATA values to numeric or string variables.
- `RESTORE [name]` resets the DATA read position, optionally to a user-defined DATA label/name.
- The example reads values in a loop, then restores to a named DATA section and reads string values.

The 3.5 addendum separately documents two commands for DATA pointer operation. It uses an integer array to save and restore `_DATA` pointer values; the example saves positions, traverses DATA, then restores positions to reread entries (addendum printed pp. 14-3–14-4; PDF pages 464–465). The exact declaration spellings are OCR-sensitive in the supplied scan; the addendum visibly uses `_DATA` and the example includes assignment/read operations. Do not invent a token name from OCR where the glyph is unclear; verify the page image before registry entry.

## Evidence and compatibility notes

- This file is a partial page-reviewed extraction, not a complete statement or file API catalogue.
- All Chapter 5–7 base-manual features above are 3.0 candidates. Their presence in the combined scan does not itself prove they were retained unchanged in interpreter 3.5.
- The 3.5 addendum's DATA-pointer feature is 3.5-specific evidence, but its OCR-sensitive command spelling must be image-verified before adding a normative declaration.
- Amiga device names, CLI behavior, native program files, and DOS semantics must not be equated with host operating-system behavior.
- The original interpreter has not been executed as part of this extraction. No runtime observation or conformance result is claimed.
- For implementation, keep source syntax, interpreter semantics, host filesystem policy, and Linux runtime support as separate status dimensions.
