# GFA BASIC 3.0 base-manual function extraction: variables, arrays, and conversions

## Evidence boundary

This extraction records forms and semantics printed in Chapter 2 of the supplied *GFA-BASIC 3.5 Interpreter* PDF. The chapter identifies its base-language text as **GFA-BASIC 3.0 Amiga**. These are therefore 3.0 source candidates, not proof that every function is available or behaves identically in every 3.5 interpreter. Page references are the manual's printed labels.

## Arrays and variable descriptors

### DIM?
- **Form:** `DIM?(x())`.
- **Argument:** array variable.
- **Meaning:** returns the number of elements in the array. The source examples produce 11 for `x(10)` and 12 for `y%(2,3)` under the default zero-based convention.
- **Source:** Chapter 2, printed pp. 2-3, 2-9.
- **Boundary:** this is element count, not dimension count; array creation and index rules belong to DIM and OPTION BASE statements.

### ARRPTR and VARPTR
- **Forms:** `ARRPTR(y)`, `VARPTR(x)`; shorthand `*y` for ARRPTR and `V:x` for VARPTR.
- **Meaning:** VARPTR returns the address of a variable's value (for strings, string data); ARRPTR returns the address of the variable/array descriptor. For arrays, `ARRPTR(x%())` addresses the descriptor while `VARPTR(x%(5))` addresses an element.
- **Source:** Chapter 2, printed pp. 2-2–2-3, 2-33.
- **Caution:** the source explains memory layout but does not establish portable pointer lifetime or safety guarantees.

## Character and numeric text conversion

### ASC and CHR$
- **Forms:** `ASC(a$)`, `CHR$(x)`.
- **Arguments:** string expression; integer expression.
- **Meaning:** ASC returns the first character's code, or zero for an empty string. CHR$ returns the character for the low byte (equivalent to `x AND 255`).
- **Source:** printed p. 2-13.

### BIN$, OCT$, HEX$
- **Forms:** `BIN$(x[,y])`, `OCT$(x[,y])`, `HEX$(x[,y])`.
- **Arguments:** integer and optional output width.
- **Meaning:** convert to base 2, 8, or 16 text. The manual specifies widths 1–32 for BIN$ and 0–11 for OCT$; HEX$'s optional width is described as the number of hexadecimal places, without an explicit bound on this page.
- **Source:** printed p. 2-14.

### STR$
- **Forms:** `STR$(x)`, `STR$(x,y)`, `STR$(x,y,z)`.
- **Arguments:** numeric expressions.
- **Meaning:** number-to-text conversion. The second argument selects a rightmost output width, conceptually `RIGHT$(SPACE$(n)+STR$(x),n)`. The third form selects z characters starting at position y.
- **Source:** printed p. 2-15.
- **Examples:** `STR$(123.4567,6,2)` -> `123.46`; `STR$(PI,5,3)` -> `3.142`; `STR$(PI,2,2)` -> `14`.

### VAL and VAL?
- **Forms:** `VAL(a$)`, `VAL?(a$)`.
- **Argument:** string expression.
- **Meaning:** VAL parses a numeric prefix and stops at the first unparseable character; if no number begins the string it returns zero. Prefixes `&H`, `&X`, and `&O` select hexadecimal, binary, and octal. VAL? reports the number of consumed characters and returns zero if none form a number.
- **Source:** printed p. 2-16.
- **Caution:** exact malformed-input/error behavior beyond the stated stopping rule is not specified here.

### CINT and CFLOAT
- **Forms:** `CINT(x)`, `CFLOAT(y)`.
- **Meaning:** CINT converts floating point to a rounded integer; CFLOAT converts an integer to floating point.
- **Source:** printed p. 2-17.
- **Applicability:** the manual explicitly says these are only meaningful to the compiler. They are not evidence of interpreter built-ins and are classified compiler-only.

### CVI, CVL, MKI$, MKL$, MKS$, MKD$
- **Forms:** `CVI(a$)`, `CVL(a$)`, `MKI$(x)`, `MKL$(x)`, `MKS$(x)`, `MKD$(x)`.
- **Meaning:** CVI/CVL convert strings from internal binary representation to numbers; MK* creates strings containing internal numeric representation rather than ASCII digits.
- **Source:** printed pp. 2-18–2-19.
- **Disputed spelling:** p. 2-18's signature list names CVI/CVL, while its prose names CVF and a following application list names CVS. The signature block lists MKI$/MKL$/MKS$/MKD$, while p. 2-19 prose additionally names MKF$. Preserve unresolved CVx and MKx slots; do not promote CVF/CVS or MKF$ to confirmed separate identifiers without corroborating edition or runtime evidence.

## Memory, environment, and storage queries

### FRE
- **Forms:** `FRE()`, `FRE(x)`.
- **Meaning:** reports available memory. The extracted passage does not fully specify the argument domain or return units.
- **Source:** printed p. 2-29. Signature/topic are source-backed; detailed semantics need page-level review.

### TYPE
- **Form:** `TYPE(x)`.
- **Meaning:** returns a type code for the supplied variable/expression.
- **Source:** printed p. 2-30. Exact code mapping requires focused review before registry declaration.

### PEEK, DPEEK, LPEEK
- **Forms:** `PEEK(x)`, `DPEEK(x)`, `LPEEK(x)`.
- **Argument:** address expression.
- **Meaning:** reads one, two, or four bytes beginning at address x, respectively.
- **Source:** printed p. 2-36.
- **Caution:** byte order, address validity, and fault behavior are not fully specified in the extracted description.

### BYTE, CARD, WORD, INT, LONG, FLOAT, SINGLE, DOUBLE, CHAR
- **Forms:** `BYTE(...)`/`BYTE{...}`, `CARD(...)`/`CARD{...}`, `WORD(...)`/`WORD{...}`, and brace forms `INT{...}`, `LONG{...}`, `FLOAT{...}`, `SINGLE{...}`, `DOUBLE{...}`, `CHAR{...}`.
- **Meaning:** typed conversion/casting forms for scalar data types. Brace notation is a conversion form, not an array subscript.
- **Source:** printed pp. 2-37–2-38; parenthesized forms are cross-listed on p. 4-26.
- **Caution:** overflow, truncation, rounding, and invalid-range behavior are not established for every conversion by this extraction.

### MALLOC and MFREE
- **Forms:** `MALLOC(x)`, `MFREE(y)`.
- **Meaning:** allocate and release memory blocks. Exact return semantics and command/function boundary require inspection of the full section.
- **Source:** printed p. 2-41.
- **Status:** retain as candidates; do not create fully typed declarations from this abbreviated evidence alone.

## Exclusions and neighbouring constructs

- DIM, OPTION BASE, ERASE, INSERT, DELETE, and ARRAYFILL are statements/commands, not functions.
- CINT and CFLOAT are explicitly compiler-only in this manual.
- V: and * are alternate address forms associated with VARPTR and ARRPTR, not additional identifiers.
- Chapter 2 is 3.0 base text inside the supplied 3.5 interpreter manual; it does not alone establish 3.5 availability.

## Registry follow-up

Before promoting candidates to `api/registry/language-core/`, verify 3.5 availability and capture exact type codes, conversion ranges, allocation return values, and memory-error behavior from the cited pages or native probes. Keep disputed CVx/MKx spellings unresolved until corroborated.
