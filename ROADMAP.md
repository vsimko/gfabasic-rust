# Project roadmap

This roadmap is the project-level view of delivery order for `gfabasic-rust`. GitHub Issues remain the source of truth for task scope, acceptance criteria, blockers, and implementation discussion.

**Snapshot:** 2026-11-06  
**Current maturity:** architecture and research baseline; Rust workspace, source-map/loading, structured diagnostics, a narrow Amiga 3.5 FACT/PRINT frontend with the exact addendum special-name lexemes `_DATA` and `_0`…`_9` retained by the lossless lexer, an initial lexer evidence inventory separating verified 3.5 rules from inherited 3.0/V3.03 candidates, and a temporary C/`cc` native bootstrap prototype, plus the initial source-preserving formatter library and CLI for the supported subset. The general semantic model, typed IR, planned LLVM backend, project configuration, runtime behavior, and full GFA BASIC conformance remain unimplemented.

## Status rules

- **Implemented** means code exists and the relevant automated checks pass.
- **Research** means evidence gathering or source analysis; it is not compiler support.
- **Blocked** means an external decision, source, or artifact is required.
- **Planned** means no implementation claim is made.
- A closed architecture/documentation issue does not imply that the corresponding compiler feature exists.

## Current project state

| Area | Current state | Evidence / tracking |
|---|---|---|
| Rust workspace | Three crates exist: compiler, generated-program runtime, Linux platform adapter; compiler includes source-map/loading, diagnostics, a narrow FACT/PRINT frontend and temporary native bootstrap; general semantics/IR/LLVM/configuration/runtime/platform behavior remains unimplemented | [Cargo workspace](Cargo.toml), [workspace design](docs/architecture/WORKSPACE.md) |
| Formatter and source tooling | Initial library formatter and gfabasic fmt CLI for the supported FACT/VARIAT/COMBIN/PRINT subset; preserves token spelling, rejects unsupported syntax, supports stdout/check/output/in-place modes with atomic file replacement. Comment trivia is preserved losslessly; string/DATA payloads, broader grammar, LSP integration, and fixture-wide checks remain open | [#208](https://github.com/vsimko/gfabasic-rust/issues/208), [#209](https://github.com/vsimko/gfabasic-rust/issues/209) |
| Compiler frontend, semantic analysis, IR | Narrow FACT/PRINT parser subset; lossless lexer additionally recognizes the explicitly documented Amiga 3.5 addendum names `_DATA` and `_0`…`_9` without claiming general identifier support; general grammar, semantic analysis, and typed IR are not implemented | [#46–#49](https://github.com/vsimko/gfabasic-rust/issues?q=is%3Aissue+is%3Aopen+label%3Acompiler), [#120](https://github.com/vsimko/gfabasic-rust/issues/120), [#121](https://github.com/vsimko/gfabasic-rust/issues/121) |
| LLVM backend and native executable | Temporary C/`cc` bridge emits one narrow native host executable; planned LLVM backend remains unimplemented | [#50](https://github.com/vsimko/gfabasic-rust/issues/50) |
| CLI and configuration | `gfabasic` supports help/version and a restricted `build` command for the FACT/PRINT prototype; general configuration and compilation integration remain open | [#9](https://github.com/vsimko/gfabasic-rust/issues/9), [#111](https://github.com/vsimko/gfabasic-rust/issues/111) |
| Runtime and platform integration | Not implemented; SDL3, Amiga and Atari ST runtime layers remain planned | [#51–#54](https://github.com/vsimko/gfabasic-rust/issues?q=is%3Aissue+is%3Aopen) |
| API registry | Schema and metadata validation are complete; runtime handler resolution remains unimplemented | [#115](https://github.com/vsimko/gfabasic-rust/issues/115) |
| Dialect and API research | Amiga GFA BASIC interpreter 3.5 is the accepted normative source dialect; 3.0/3.03 material is historical research requiring 3.5 confirmation, and Atari ST V2.xx/V3.0 remains research-only. No dialect is implemented or conformant | [ADR 0009](docs/architecture/adr/0009-initial-source-dialect-amiga-gfa-basic-3.5.md), [#3](https://github.com/vsimko/gfabasic-rust/issues/3), [#63–#66](https://github.com/vsimko/gfabasic-rust/issues?q=is%3Aissue+is%3Aopen) |
| Tests | Existing source-loader and registry checks remain; the FACT/PRINT frontend and native-executable path have focused positive/negative and end-to-end tests. General conformance runner and fuzz targets remain unimplemented | [#42](https://github.com/vsimko/gfabasic-rust/issues/42), [#43](https://github.com/vsimko/gfabasic-rust/issues/43), [#116](https://github.com/vsimko/gfabasic-rust/issues/116), [#118](https://github.com/vsimko/gfabasic-rust/issues/118) |
| Documentation | README, documentation index, architecture, source register, research notes, compatibility matrix, and policies exist; user quick start is correctly withheld until a usable source-compilation command and distributable build are available and tested | [#16](https://github.com/vsimko/gfabasic-rust/issues/16) |
| CI and supply chain | Documentation and Rust workspace checks, RustSec audits, source-loader fuzzing, and source-loading benchmark workflows are implemented. The dependency-license policy is approved; pull-request CI generates and validates an SPDX 2.3 SBOM for the full workspace. Upload to an actual published GitHub Release remains unverified under [#39](https://github.com/vsimko/gfabasic-rust/issues/39) | [#27](https://github.com/vsimko/gfabasic-rust/issues/27), [#39](https://github.com/vsimko/gfabasic-rust/issues/39), [#118](https://github.com/vsimko/gfabasic-rust/issues/118) |
| Releases | A narrow workflow generates and validates the workspace SBOM on pull requests and is configured to attach it to a published release. No compiler build, package, installation, or full release workflow/artifact exists; the release-event upload remains untested | [#39](https://github.com/vsimko/gfabasic-rust/issues/39), [#40](https://github.com/vsimko/gfabasic-rust/issues/40), [#62](https://github.com/vsimko/gfabasic-rust/issues/62) |

## Delivery phases

### Phase 0 — Maintain a trustworthy project baseline

Keep the architecture, documentation, and research evidence internally consistent. Do not add speculative usage instructions.

- [x] Architecture and ADR structure — [#1](https://github.com/vsimko/gfabasic-rust/issues/1)
- [x] Goals and compatibility principles — [#2](https://github.com/vsimko/gfabasic-rust/issues/2)
- [x] Target model — [#4](https://github.com/vsimko/gfabasic-rust/issues/4)
- [x] Compiler pipeline and IR architecture — [#6](https://github.com/vsimko/gfabasic-rust/issues/6)
- [x] CLI/configuration architecture — [#9](https://github.com/vsimko/gfabasic-rust/issues/9)
- [x] Workspace architecture — [#11](https://github.com/vsimko/gfabasic-rust/issues/11)
- [x] API registry architecture — [#12](https://github.com/vsimko/gfabasic-rust/issues/12)
- [x] Documentation policy; “OKF” ambiguity explicitly unresolved — [#14](https://github.com/vsimko/gfabasic-rust/issues/14)
- [x] Reconcile the project README and roadmap with actual buildable capabilities — [#16](https://github.com/vsimko/gfabasic-rust/issues/16)
- [x] Verify contributor setup instructions against a clean supported Linux environment — [#112](https://github.com/vsimko/gfabasic-rust/issues/112)

### Phase 1 — Establish an evidence-backed first language subset

Choose a precise dialect/version and document only source-verified syntax and semantics. This phase is the prerequisite for parser and runtime behavior.

- [x] Select Amiga GFA BASIC interpreter 3.5 as the first normative dialect; keep Atari ST V2.xx/V3.0 research-only — [#3](https://github.com/vsimko/gfabasic-rust/issues/3), [ADR 0009](docs/architecture/adr/0009-initial-source-dialect-amiga-gfa-basic-3.5.md)
- [ ] Complete the Amiga 3.5 feature-level compatibility policy and evidence review — [#19](https://github.com/vsimko/gfabasic-rust/issues/19), [#20](https://github.com/vsimko/gfabasic-rust/issues/20)
- [ ] Complete the language feature inventory and compatibility matrix — [#19](https://github.com/vsimko/gfabasic-rust/issues/19)
- [ ] Verify Amiga 3.5 lexical rules, syntax, types, operators and expressions — [#20](https://github.com/vsimko/gfabasic-rust/issues/20)
- [ ] Verify statements, control flow and built-in functions — [#21](https://github.com/vsimko/gfabasic-rust/issues/21), [#22](https://github.com/vsimko/gfabasic-rust/issues/22)
- [ ] Verify file, disk, data and resource operations — [#23](https://github.com/vsimko/gfabasic-rust/issues/23)
- [ ] Resolve Amiga/Atari platform API evidence separately — [#24](https://github.com/vsimko/gfabasic-rust/issues/24), [#25](https://github.com/vsimko/gfabasic-rust/issues/25), [#63–#66](https://github.com/vsimko/gfabasic-rust/issues?q=is%3Aissue+is%3Aopen)
- [ ] Build a licensed, source-traceable conformance corpus — [#42](https://github.com/vsimko/gfabasic-rust/issues/42)

### Phase 2 — Build a minimal vertical compiler slice

Implement the smallest useful source-to-native-executable path for the accepted subset. Keep frontend and IR independent of LLVM and SDL3.

- [x] Generic source map and strict UTF-8 transport loader — [#45](https://github.com/vsimko/gfabasic-rust/issues/45)
- [ ] Dialect-aware source decoding and composition — [#120](https://github.com/vsimko/gfabasic-rust/issues/120)
- [ ] Source-map integration with compiler diagnostics — [#121](https://github.com/vsimko/gfabasic-rust/issues/121)
- [ ] Dialect-aware lexer — [#46](https://github.com/vsimko/gfabasic-rust/issues/46)
  - Initial FACT/VARIAT/COMBIN/PRINT slice now exposes lossless horizontal-whitespace trivia and line-boundary token spans. [Lexer evidence inventory](docs/research/amiga-gfa-basic-35-lexer-evidence.md) records the two supplied books' relevant rules and version boundaries; only the Chapter 14 special names are explicitly verified as 3.5 lexical forms. Full Amiga 3.5 lexical coverage, comments, literals, general identifiers, and operators remain open and evidence-gated. A narrow additional 3.5 addendum increment recognizes `_DATA` and `_0`…`_9` from Chapter 14 p. 14-6; this does not define general underscore identifiers or parser semantics. Regression coverage also checks EOF/delimiter boundaries and rejects longer underscore-prefixed lookalikes without accepting a valid prefix.
- [ ] Parser and AST — [#47](https://github.com/vsimko/gfabasic-rust/issues/47)
  - Initial supported-subset parser now returns the AST together with its lossless token/trivia representation; general grammar, richer recovery, and semantic integration remain open.
- [ ] Symbol resolution and semantic analysis — [#48](https://github.com/vsimko/gfabasic-rust/issues/48)
  - Prototype-only symbol identities and PRINT references are implemented; full declaration, scope, type, suffix, procedure, and API-profile rules remain evidence-gated.
- [ ] Formatter library and CLI — [#208](https://github.com/vsimko/gfabasic-rust/issues/208)
  - Initial library formatter and CLI are implemented for the current lossless frontend subset; comment/literal preservation, broader grammar, corpus-wide checks, and LSP integration remain open.
- [ ] Typed, target-independent IR — [#49](https://github.com/vsimko/gfabasic-rust/issues/49)
- [ ] LLVM backend and Linux x86-64 output — [#50](https://github.com/vsimko/gfabasic-rust/issues/50)
- [ ] Implement a usable CLI and project configuration — [#111](https://github.com/vsimko/gfabasic-rust/issues/111)
- [ ] Implement the GFA BASIC core runtime as required by the accepted subset — [#51](https://github.com/vsimko/gfabasic-rust/issues/51)
- [ ] Integrate the first end-to-end compiler vertical slice — [#113](https://github.com/vsimko/gfabasic-rust/issues/113); the current FACT/PRINT bootstrap is partial progress only

### Phase 3 — Add platform APIs and robustness

Only implement platform behavior backed by accepted references and tests.

- [ ] Linux platform adapter and SDL3 integration — [#52](https://github.com/vsimko/gfabasic-rust/issues/52)
- [ ] Amiga API compatibility runtime — [#53](https://github.com/vsimko/gfabasic-rust/issues/53)
- [ ] Atari ST API compatibility runtime — [#54](https://github.com/vsimko/gfabasic-rust/issues/54)
- [ ] Resolve API registry declarations to actual runtime handlers — [#115](https://github.com/vsimko/gfabasic-rust/issues/115)
- [ ] Compatibility report and optional migration analysis — [#56](https://github.com/vsimko/gfabasic-rust/issues/56)
- [x] Fuzzing strategy and source-loader smoke/scheduled campaigns — [#43](https://github.com/vsimko/gfabasic-rust/issues/43), [#118](https://github.com/vsimko/gfabasic-rust/issues/118); lexer/parser targets remain deferred until [#46](https://github.com/vsimko/gfabasic-rust/issues/46) and [#47](https://github.com/vsimko/gfabasic-rust/issues/47) expose stable entry points
- [ ] Benchmark strategy and measurements — [#60](https://github.com/vsimko/gfabasic-rust/issues/60)

### Phase 4 — Release readiness

A release must publish an explicit support matrix and known deviations. Do not publish binaries or compatibility claims before the build and test gates exist.

- [x] Approve dependency-license/security/SBOM policy and implement RustSec auditing plus pull-request SBOM generation/validation — [#39](https://github.com/vsimko/gfabasic-rust/issues/39)
- [ ] Verify SPDX 2.3 SBOM attachment on a published GitHub Release — [#39](https://github.com/vsimko/gfabasic-rust/issues/39); the release-event action remains unverified until an actual release is published
- [x] Define release workflow and artifact policy baseline — [#40](https://github.com/vsimko/gfabasic-rust/issues/40)
- [ ] Package compiler, runtime and installation artifacts — [#62](https://github.com/vsimko/gfabasic-rust/issues/62)
- [ ] Implement executable conformance and release test gates — [#116](https://github.com/vsimko/gfabasic-rust/issues/116)
- [ ] Write tested installation and first compiler quick-start documentation — [#126](https://github.com/vsimko/gfabasic-rust/issues/126)

The first end-to-end MVP integration gate is tracked separately in [#113](https://github.com/vsimko/gfabasic-rust/issues/113). Registry/runtime handler resolution is tracked in [#115](https://github.com/vsimko/gfabasic-rust/issues/115).

## Blockers and sequencing rules

1. The initial dialect/version is selected (Amiga GFA BASIC interpreter 3.5); 3.0/3.03 rules remain inherited candidates until confirmed for 3.5, and feature-level source evidence still gates language semantics, parser acceptance criteria, and conformance claims.
2. Source manager, lexer, parser, semantic analysis and typed IR are sequential dependencies for the first compiler path.
3. LLVM backend and runtime integration depend on a stable typed IR and runtime ABI.
4. Platform APIs depend on verified profile declarations; Amiga and Atari ST evidence must not be conflated.
5. A user quick start and installation guide depend on a working, tested CLI and distributable build.
6. The generic source-loader fuzz target is implemented; lexer/parser fuzz targets depend on stable frontend entry points and source contracts.
7. Release packaging depends on successful end-to-end builds and explicit runtime dependency policy.

## Roadmap maintenance

Update this file when phase order, dependency relationships, or project maturity changes. Update the corresponding GitHub Issue when task scope or acceptance criteria change. Never use this roadmap as a substitute for issue-level technical decisions, and never mark a phase complete solely because its planning document exists.
