# Compatibility evidence and agent workflow

- Status: **Accepted working policy**
- Scope: GFA BASIC compiler compatibility research, implementation, and debugging.
- Current reference implementation: Commodore Amiga GFA BASIC interpreter 3.52.

## Purpose

Build a useful compiler as early as possible. The compiler should accept and implement the largest practical set of programs and language features that the available GFA BASIC 3.52 interpreter can interpret.

The 3.52 interpreter is the current behavioral reference. This is a project working assumption. It does not claim that releases 3.5 and 3.52 are historically identical.

A direct comparison with an independently identified interpreter 3.5 release is a later validation phase. It must not block current compiler development.

## Evidence priority

Use evidence in this order:

1. **Observed interpreter behavior.** A reproducible result from the 3.52 interpreter is the primary compatibility oracle.
2. **Historical programs.** A program shown to run in the reference interpreter is a valuable compatibility fixture. Record its provenance and use it only when redistribution rights permit.
3. **Books and manuals.** Use them to discover features, understand intended use, and investigate failures. A book can describe an older release, omit details, or contain an error. It does not override a contrary observation from the reference interpreter.
4. **Binary analysis.** Disassembly or decompilation can help explain internal behavior. Record the exact binary identity and the address, symbol, or relevant context. Treat conclusions as analysis evidence, not as a substitute for runtime tests.
5. **Unverified assumptions.** Keep assumptions explicit. Do not silently turn them into language rules.

Evidence sources can disagree. Preserve the disagreement and its provenance. Do not silently reconcile it.

## Feature records

For each important language feature, record as much of this information as is available:

- Stable feature name or identifier.
- Exact dialect, platform, and interpreter version.
- Syntax and a small independently authored example.
- Observed behavior and test environment.
- Manual or book title, edition, printed page, and section.
- Binary identity and disassembly context, when used.
- Evidence status: observed, documented, inferred, disputed, or unknown.
- Compiler implementation status and known deviations.
- Regression or conformance test paths.
- Related GitHub issue or pull request.

Do not claim implementation support because a manual describes a feature or because a registry entry exists. Keep evidence status separate from implementation status.

## Debugging and compatibility investigation

When the compiler rejects a program, crashes, or produces unexpected behavior:

1. Preserve the failing source and reduce it to a minimal reproducer.
2. Run the same reproducer in the GFA BASIC 3.52 interpreter using the existing Amiberry workflow where applicable.
3. Record the interpreter result, environment, and any output or observable side effect.
4. Compare the interpreter result with the compiler result. Identify whether the difference concerns lexical acceptance, parsing, semantics, runtime behavior, generated output, or the test harness.
5. Consult relevant manuals and books. Check their exact edition, platform, and version. Record any disagreement with observed behavior.
6. Inspect existing binary-analysis notes or perform focused analysis when it can explain the difference. Record the exact binary identity and the limits of the conclusion.
7. Fix the implementation or document a justified limitation. Add a regression test for every reproducible defect.
8. Link the test and evidence from the relevant GitHub issue. Update the canonical feature record instead of copying the same rule into several documents.

Never hide a crash by skipping an unknown construct or assigning it semantics from another BASIC dialect.

## Test and source handling

- Prefer small, independently authored fixtures.
- Keep reference-interpreter tests distinct from documentation-derived tests and compiler-only unit tests.
- Record which interpreter version actually ran each test. Do not relabel a 3.52 result as a verified 3.5 result.
- Reuse the repository's existing Amiberry workflows and private runtime assets. Do not create a parallel emulator workflow without a specific technical need.
- Do not commit commercial manuals, books, ROMs, disk images, or historical program listings. Keep bibliographic metadata, concise citations, original analysis, and independently authored fixtures.
- Do not publish private runtime assets or credentials in logs or artifacts.

## Version policy

For current development, use GFA BASIC 3.52 as the practical behavior baseline and maximize compatibility with its interpreter.

If an independently identified GFA BASIC 3.5 interpreter becomes available, run the same regression suite against both versions. Record which features and programs behave identically and which differ. Keep the results version-scoped.

## GitHub workflow for agents

GitHub Issues are the source of truth for work, acceptance criteria, blockers, and discussion.

1. Start with the root [README](../../README.md), then read [AGENTS.md](../../AGENTS.md).
2. Read the [documentation index](../README.md) and this policy.
3. Inspect open issues and pull requests before starting work. Check `needs-human` issues and do not bypass human blockers.
4. Select one issue. Read its full description, comments, linked issues, and relevant architecture/research documents.
5. Implement only the agreed scope. Keep the issue updated with evidence, decisions, and blockers.
6. Add or update tests and documentation with the implementation.
7. Open a pull request linked to the issue. Report only checks that actually ran.
8. Update the roadmap or status dashboard only when the canonical implementation state changes.

Do not treat chat history, commit messages, generated dashboards, or an agent's private notes as the source of truth. If project context is missing, reconstruct it from the README, AGENTS.md, documentation indexes, GitHub issues, and linked canonical documents.

## Relationship to the documentation format

The canonical documentation format and writing rules are defined in [DOCUMENTATION.md](../DOCUMENTATION.md). This policy uses plain UTF-8 Markdown and relative links. Do not claim conformance to an external “Open Knowledge Format” specification unless the project identifies the exact publisher, specification, version, and required profile.
