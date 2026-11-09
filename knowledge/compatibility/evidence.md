---
type: Research Method
title: Compatibility evidence and debugging method
description: How to investigate reference behavior, resolve discrepancies, and add reliable regression tests.
tags: [research, evidence, debugging, tests]
sources:
  - resource: ../../docs/architecture/COMPATIBILITY-EVIDENCE.md
    title: Compatibility evidence and agent workflow
  - resource: ../../docs/research/README.md
    title: Research materials index
---

# Evidence priority

1. **Observed interpreter behavior** is the primary compatibility evidence. Record the exact executable identity, environment, input, and observable result.
2. **Historical programs** are useful compatibility fixtures when provenance and redistribution rights are known.
3. **Books and manuals** help discover features and explain failures. They may describe another release, omit details, or contain errors.
4. **Binary analysis** can explain internal behavior. Record the exact binary identity and relevant address, symbol, or context.
5. **Assumptions** must remain explicit and must not silently become language rules.

When sources disagree, preserve the disagreement and its provenance. Do not silently reconcile it.

# Debugging procedure

1. Preserve the failing program and reduce it to a minimal reproducer.
2. Run the same reproducer in the 3.52 reference interpreter using the existing Amiberry workflow.
3. Record the environment, output, and observable side effects.
4. Compare interpreter and compiler results. Identify the failing stage: lexing, parsing, semantics, runtime, generated output, or test harness.
5. Consult the relevant manuals and books. Record edition, platform, version, printed page, and section.
6. Inspect binary-analysis evidence when it can explain the difference.
7. Fix the implementation or document a justified limitation.
8. Add a regression test and link it from the relevant GitHub issue.

# Evidence and implementation status

Use separate status for evidence, implementation, and tests. A manual description or registry entry does not prove compiler support. Never skip unknown syntax or assign semantics from another BASIC dialect.

# Full policy

See [COMPATIBILITY-EVIDENCE.md](../../docs/architecture/COMPATIBILITY-EVIDENCE.md) and [the research index](../../docs/research/README.md).
