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

## Corpus result protocol

Use one ASCII record per fixture, with the stable fixture ID from the corpus manifest and one terminal outcome:

```text
GFATEST:<fixture-id>:PASS
GFATEST:<fixture-id>:FAIL
```

Protocol rules for the host-side collector:

1. The result file is the batch's result channel; do not parse the interpreter's visible output window or CLI transcript as test results.
2. Emit exactly one terminal record for every fixture that the batch claims to have run. A missing record means **incomplete**, never PASS.
3. Fixture IDs must be unique within a batch and must match IDs requested by the host. Unknown IDs, duplicate terminal records, malformed records, and unknown outcomes make the batch invalid.
4. Keep each record on one line and restrict fixture IDs to the manifest's ASCII identifier syntax. Do not add free-form diagnostics to this protocol until an escaping/encoding rule is specified.
5. Close the result file before terminating the interpreter. The host reads it only after the emulator run has completed and the file is available.
6. Preserve the raw result file with the run metadata. A parsed PASS is an execution observation only for the exact interpreter build, ROM/OS, and emulator configuration recorded for that run; it does not by itself change the fixture's manual-evidence status or prove another interpreter version.

The host should compare the returned IDs with the requested batch, not infer success from process exit alone. A run with no result file, a truncated file, or an incomplete set of records is inconclusive/failed collection, not a successful test run.

## Batch execution boundary

The intended efficient workflow is to prepare one batch, start the emulator/interpreter once, execute the batch, close the result file, and collect the file once. This document specifies only the result channel and validation contract. It does not claim that the current workflow already supports arbitrary batches, automatic `.LST` tokenization, or per-fixture isolation inside one interpreter session.

