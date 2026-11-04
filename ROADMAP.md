# Project roadmap

This roadmap is the project-level view of delivery order for `gfabasic-rust`. GitHub Issues remain the source of truth for task scope, acceptance criteria, blockers, and implementation discussion.

**Snapshot:** 2026-11-04  
**Current maturity:** architecture and research baseline; Rust workspace bootstrap, source-map and strict UTF-8 transport primitives, and a minimal help/version CLI. The language frontend, backend, project configuration, runtime behavior, and GFA BASIC conformance are not implemented.

## Status rules

- **Implemented** means code exists and the relevant automated checks pass.
- **Research** means evidence gathering or source analysis; it is not compiler support.
- **Blocked** means an external decision, source, or artifact is required.
- **Planned** means no implementation claim is made.
- A closed architecture/documentation issue does not imply that the corresponding compiler feature exists.

## Current project state

| Area | Current state | Evidence / tracking |
|---|---|---|
| Rust workspace | Three crates exist: compiler, generated-program runtime, Linux platform adapter; compiler includes source-map and strict UTF-8 loading primitives plus a help/version CLI, while frontend/backend/project configuration/runtime/platform behavior remains unimplemented | [Cargo workspace](Cargo.toml), [workspace design](docs/architecture/WORKSPACE.md) |
| Compiler frontend, semantic analysis, IR | Not implemented beyond source-map/loading primitives | [#46–#49](https://github.com/vsimko/gfabasic-rust/issues?q=is%3Aissue+is%3Aopen+label%3Acompiler), [#120](https://github.com/vsimko/gfabasic-rust/issues/120), [#121](https://github.com/vsimko/gfabasic-rust/issues/121) |
| LLVM backend and native executable | Not implemented | [#50](https://github.com/vsimko/gfabasic-rust/issues/50) |
| CLI and configuration | Minimal `gfabasic` executable supports help/version; build/check explicitly reject compilation. Project configuration and compilation integration remain open | [#9](https://github.com/vsimko/gfabasic-rust/issues/9), [#111](https://github.com/vsimko/gfabasic-rust/issues/111) |
| Runtime and platform integration | Not implemented; SDL3, Amiga and Atari ST runtime layers remain planned | [#51–#54](https://github.com/vsimko/gfabasic-rust/issues?q=is%3Aissue+is%3Aopen) |
| API registry | Schema and metadata validation are complete; runtime handler resolution remains unimplemented | [#115](https://github.com/vsimko/gfabasic-rust/issues/115) |
| Dialect and API research | Atari ST and Amiga research drafts exist; no dialect is accepted or implemented | [#3](https://github.com/vsimko/gfabasic-rust/issues/3), [#63–#66](https://github.com/vsimko/gfabasic-rust/issues?q=is%3Aissue+is%3Aopen) |
| Tests | Test strategy and registry metadata checks exist; no compiler conformance runner, fuzz harness, or executable end-to-end tests | [#42](https://github.com/vsimko/gfabasic-rust/issues/42), [#43](https://github.com/vsimko/gfabasic-rust/issues/43), [#116](https://github.com/vsimko/gfabasic-rust/issues/116), [#118](https://github.com/vsimko/gfabasic-rust/issues/118) |
| Documentation | README, documentation index, architecture, source register, research notes, compatibility matrix, and policies exist; user quick start is correctly withheld until an end-to-end compiler and distributable artifact exist | [#16](https://github.com/vsimko/gfabasic-rust/issues/16) |
| CI and supply chain | Documentation and fast workspace checks are documented; dependency/license policy and reproducible SBOM remain open | [#39](https://github.com/vsimko/gfabasic-rust/issues/39) |
| Releases | Release policy work is tracked, but no compiler artifacts can currently be produced | [#40](https://github.com/vsimko/gfabasic-rust/issues/40), [#62](https://github.com/vsimko/gfabasic-rust/issues/62) |

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

- [ ] Select the first dialect/version and define its compatibility policy — [#3](https://github.com/vsimko/gfabasic-rust/issues/3)
- [ ] Complete the language feature inventory and compatibility matrix — [#19](https://github.com/vsimko/gfabasic-rust/issues/19)
- [ ] Verify lexical rules, syntax, types, operators and expressions — [#20](https://github.com/vsimko/gfabasic-rust/issues/20)
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
- [ ] Parser and AST — [#47](https://github.com/vsimko/gfabasic-rust/issues/47)
- [ ] Symbol resolution and semantic analysis — [#48](https://github.com/vsimko/gfabasic-rust/issues/48)
- [ ] Typed, target-independent IR — [#49](https://github.com/vsimko/gfabasic-rust/issues/49)
- [ ] LLVM backend and Linux x86-64 output — [#50](https://github.com/vsimko/gfabasic-rust/issues/50)
- [ ] Implement a usable CLI and project configuration — [#111](https://github.com/vsimko/gfabasic-rust/issues/111)
- [ ] Implement the GFA BASIC core runtime as required by the accepted subset — [#51](https://github.com/vsimko/gfabasic-rust/issues/51)
- [ ] Integrate the first end-to-end compiler vertical slice — [#113](https://github.com/vsimko/gfabasic-rust/issues/113)

### Phase 3 — Add platform APIs and robustness

Only implement platform behavior backed by accepted references and tests.

- [ ] Linux platform adapter and SDL3 integration — [#52](https://github.com/vsimko/gfabasic-rust/issues/52)
- [ ] Amiga API compatibility runtime — [#53](https://github.com/vsimko/gfabasic-rust/issues/53)
- [ ] Atari ST API compatibility runtime — [#54](https://github.com/vsimko/gfabasic-rust/issues/54)
- [ ] Resolve API registry declarations to actual runtime handlers — [#115](https://github.com/vsimko/gfabasic-rust/issues/115)
- [ ] Compatibility report and optional migration analysis — [#56](https://github.com/vsimko/gfabasic-rust/issues/56)
- [ ] Fuzzing strategy and robustness checks — [#43](https://github.com/vsimko/gfabasic-rust/issues/43), [#118](https://github.com/vsimko/gfabasic-rust/issues/118) (blocked on dependency-license approval in #39)
- [ ] Benchmark strategy and measurements — [#60](https://github.com/vsimko/gfabasic-rust/issues/60)

### Phase 4 — Release readiness

A release must publish an explicit support matrix and known deviations. Do not publish binaries or compatibility claims before the build and test gates exist.

- [ ] Complete dependency/license/security and SBOM policy — [#39](https://github.com/vsimko/gfabasic-rust/issues/39)
- [x] Define release workflow and artifact policy baseline — [#40](https://github.com/vsimko/gfabasic-rust/issues/40)
- [ ] Package compiler, runtime and installation artifacts — [#62](https://github.com/vsimko/gfabasic-rust/issues/62)
- [ ] Implement executable conformance and release test gates — [#116](https://github.com/vsimko/gfabasic-rust/issues/116)
- [ ] Write tested installation and first compiler quick-start documentation — [#126](https://github.com/vsimko/gfabasic-rust/issues/126)

The first end-to-end MVP integration gate is tracked separately in [#113](https://github.com/vsimko/gfabasic-rust/issues/113). Registry/runtime handler resolution is tracked in [#115](https://github.com/vsimko/gfabasic-rust/issues/115).

## Blockers and sequencing rules

1. Dialect/version selection and source evidence block language semantics, parser acceptance criteria, and conformance claims.
2. Source manager, lexer, parser, semantic analysis and typed IR are sequential dependencies for the first compiler path.
3. LLVM backend and runtime integration depend on a stable typed IR and runtime ABI.
4. Platform APIs depend on verified profile declarations; Amiga and Atari ST evidence must not be conflated.
5. A user quick start and installation guide depend on a working, tested CLI and distributable build.
6. Fuzzing can prepare strategy now, but meaningful lexer/parser fuzz targets depend on frontend implementation.
7. Release packaging depends on successful end-to-end builds and explicit runtime dependency policy.

## Roadmap maintenance

Update this file when phase order, dependency relationships, or project maturity changes. Update the corresponding GitHub Issue when task scope or acceptance criteria change. Never use this roadmap as a substitute for issue-level technical decisions, and never mark a phase complete solely because its planning document exists.
