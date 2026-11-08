# Amiga GFA-BASIC book — 515-page extraction audit

Source: `Das Grosse Buch zu GFA-BASIC 3.5 Amiga.pdf` (515 PDF pages; 511 pages contain extractable text; 4 pages are blank or contain no OCR text).

## Source identity and scope

The title advertises GFA-BASIC 3.5, but the book itself says its command descriptions cover the GFA V3.03 interpreter (introductory chapter, printed p. 13; PDF p. 13) and describes approximately 360 commands and functions. Chapter 18 explicitly introduces additions in version 3.5 (printed p. 451; PDF p. 451). Therefore, material outside the explicitly versioned 3.5 sections is recorded as historical/base-interpreter evidence, not silently treated as a 3.5 specification.

The book's organization is problem-oriented: chapters 1–19, appendices A–E, command index (printed pp. 493–497; PDF pp. 493–497), then subject index (printed pp. 499 onward). OCR is imperfect; spelling and syntax need image-level verification before treating any token as canonical.

## Page coverage

The full PDF was passed through text extraction and page segmentation. All 515 page positions were accounted for; 511 contain text and 4 are empty/non-extractable. The table maps the printed contents to the PDF page ranges. Ranges are approximate where OCR distorted contents-page pagination.

| PDF pages | Printed pages / section | Extraction focus |
|---|---|---|
| 1–8 | Front matter | Cover, publication metadata, introductory matter |
| 9–12 | Contents | Full chapter/subchapter map |
| 13–16 | 1. Zu diesem Buch | Interpreter version, notation and command-description conventions |
| 17–18 | 2. Der Amiga | Amiga platform context |
| 19–28 | 3. Das GFA-BASIC | Editor, menu, RUN-only interpreter |
| 29–74 | 4. Basis-BASIC | BASIC fundamentals, data types, variables, arrays, loops and comparisons |
| 75–144 | 5. Ein-/Ausgabebefehle | Input/output, files, DOS library, ports, printer, sound/speech |
| 145–192 | 6. Programmstruktur | Loops, branches, declarations, subprograms, external program calls |
| 193–200 | 7. Textoperationen | String manipulation, analysis and formatting |
| 201–224 | 8. Arithmetik-Befehle | Operators, math, numeric/trigonometric functions, comparisons, bit operations, random values |
| 225–278 | 9. Grafik | Graphics definitions, object graphics, drawing, screen memory, animation |
| 279–290 | 10. Datenumwandlung | Number systems and conversion |
| 291–330 | 11. Feld-, Speicher- und Zeigeroperationen | Arrays, memory management, pointers, Amiga Exec library |
| 331–362 | 12. Programmkontrolle | Start/end, erase, time, errors, system information, multitasking, debugging |
| 363–374 | 13. Interaktionen (Programm/Benutzer) | Program/user interaction |
| 375–388 | 14. Window- und Screen-Programmierung | Window and screen commands |
| 389–394 | 15. Menüprogrammierung mit BASIC-Befehlen | Menu programming |
| 395–404 | 16. Ereignis-Überwachung mit BASIC-Befehlen | Event monitoring |
| 405–450 | 17. Der GFA-Compiler | Compiler use, shell, linker errors, compiler BASIC |
| 451–476 | 18. Vektor- und Matrizenberechnungen | New version 3.5 vector/matrix facilities |
| 477–482 | 19. Mehr Bedienkomfort | Math additions and editor commands |
| 483 | Appendix A | ASCII table |
| 484–488 | Appendix B | GFA-BASIC error messages |
| 489–492 | Appendix C | DOS error messages |
| 493–497 | Appendix D | Alphabetical command index |
| 498 | Appendix E | Source note |
| 499–515 | Subject index | Subject index and cross-references |

## Version evidence

- PDF p. 13: introduction identifies the described command/function set as GFA V3.03 interpreter.
- PDF p. 16: explicitly refers to the version V3.0 in a behavioral note.
- PDF p. 451: chapter 18 says the following pages describe commands newly added in version 3.5, and says the old commands were not fundamentally changed.
- PDF pp. 477–482: chapter 19 contains additional convenience features; these should be checked individually for version scope rather than assumed to be 3.5.
- The command index on PDF pp. 493–497 mixes the whole book's vocabulary; it is not a 3.5-only list.

## Extraction status and caveats

- Page segmentation covered PDF page positions 1–515.
- Searchable text exists for 511 pages; OCR/text extraction is absent on 4.
- The extracted text is a discovery aid, not a faithful facsimile. OCR damages symbols, punctuation, abbreviations, and identifiers (especially Amiga API names).
- The printed command index is a useful inventory seed but must be reconciled against the command's actual description and, where needed, page images.
- No command is marked implemented solely because it appears in this book.
