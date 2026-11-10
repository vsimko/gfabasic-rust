# Amiga GFA BASIC test output

## Confirmed local observation

The repository maintainer tested the following program with the Amiga GFA BASIC interpreter on Kickstart 1.3:

```basic
OPEN "O",#1,"test-result.txt"
PRINT #1,"GFATEST:hello:PASS"
PRINT #1,"GFATEST:fact-zero:PASS"
CLOSE #1
QUIT
```

The program creates a plain text file containing:

```text
GFATEST:hello:PASS
GFATEST:fact-zero:PASS
```

This is a successful local observation of GFA BASIC's native sequential-file output syntax. It avoids the separate GFA output window used by ordinary `PRINT` and avoids the `==>` prefix observed when writing to `Output()` with the AmigaDOS `Write()` routine.

## Evidence limitations

- The Amiga machine was reported to use Kickstart 1.3.
- The exact GFA BASIC interpreter release/build used for this observation was not recorded.
- This is therefore an operational observation, not yet a version-qualified Amiga GFA BASIC 3.5 conformance result.
- The example demonstrates file output, not redirection of the interpreter's CLI stdout.
- Do not promote corpus fixtures to `verified` based on this I/O experiment. Fixture behavior still needs its own applicable manual evidence or named interpreter observation.

## Proposed corpus result convention

Use one ASCII record per test, with stable fixture IDs and explicit outcomes:

```text
GFATEST:<fixture-id>:PASS
GFATEST:<fixture-id>:FAIL
```

Keep diagnostic details on separate records only after defining their escaping and parsing rules. The batch runner should close the result file before terminating so the host can safely read it.
