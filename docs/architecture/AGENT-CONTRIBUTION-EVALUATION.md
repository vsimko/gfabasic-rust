---
type: "Architecture"
title: "AI-agent contribution evaluation pilot"
---

# AI-agent contribution evaluation pilot

Tracking: [issue #578](https://github.com/vsimko/gfabasic-rust/issues/578).

## Purpose and limits

This is a small retrospective baseline, not a performance ranking or a claim that agent use improves throughput. The sample contains three completed PRs from one short project-work period. Their selection as agent-assisted comes from the contemporaneous project collaboration context; GitHub author/committer metadata identifies the repository owner and cannot independently prove agent authorship.

Do not compare people or agents, publish leaderboards, use PR count/lines/speed as success measures, or set numeric targets from this sample.

## Rubric and data sources

| Signal | Operational definition | Source and limitation |
|---|---|---|
| First-head CI | Conclusion of checks associated with the first submitted PR head; retain failed and skipped jobs separately | GitHub Actions runs for the recorded head SHA |
| Fix-up pushes | Additional head updates after initial submission and before the first green required gate | Not reliably exposed by the available PR metadata. Commit count is not a substitute for push count; report as unavailable |
| Substantive review findings | Formal review or inline thread that requests a code, test, or design change | GitHub review submissions and review threads; empty results do not include informal discussion |
| Follow-up defects | Linked issue or PR reporting a regression or corrective work after merge | Issue/PR tracker; attribution requires evidence, not temporal proximity |
| DoD/evidence quality | Whether the PR records scope, actual validation, skipped checks, risks, and human blockers | PR description compared with the repository PR template and AGENTS.md |
| Scope drift/duplicate work | Reviewer judgement that changed files or work duplicate or exceed the stated issue | PR diff and linked issue; qualitative, not a numeric score |
| Human blockers | Explicit needs-human work remaining at merge | Linked issues and labels |

## Sample selection

Inclusion: completed PRs #581, #582, and #584 from the same short collaboration period, where the project-session context establishes AI assistance. Exclusion: unrelated PRs, open PRs, and work for which agent involvement cannot be established from the collaboration context. This is a convenience sample, not a representative sample of the repository's historical contributions.

| PR | Scope | First submitted head | Recorded CI observations | Reviews |
|---|---|---|---|---|
| [#581](https://github.com/vsimko/gfabasic-rust/pull/581) | Agent DoD and PR lifecycle | `2252822701c6be693276d6a8777c13599554cb9b` | Rust checks, documentation checks, and stable PR gate passed. The first preview smoke test failed because the deployed preview returned HTTP 404 after deployment; a later preview run succeeded. Benchmark compilation passed; measurement job was skipped for this PR. Runs: [preview failure](https://github.com/vsimko/gfabasic-rust/actions/runs/44176081545), [later preview](https://github.com/vsimko/gfabasic-rust/actions/runs/44177849222), [gate](https://github.com/vsimko/gfabasic-rust/actions/runs/44176081520). | No formal review submissions or inline threads returned |
| [#582](https://github.com/vsimko/gfabasic-rust/pull/582) | Deterministic source-map property test and Rust-only mutation workflow | `4f900d84dbc17fe1d0c2c04ace9bd814f2f829a0` | Rust checks, documentation checks, workflow/skill validation, and stable PR gate passed. Mutation testing was intentionally not part of PR CI; the killed-mutant experiment remained unverified at this baseline. Runs: [Rust](https://github.com/vsimko/gfabasic-rust/actions/runs/44182379005), [docs](https://github.com/vsimko/gfabasic-rust/actions/runs/44182379039), [workflow/skills](https://github.com/vsimko/gfabasic-rust/actions/runs/44182379146), [gate](https://github.com/vsimko/gfabasic-rust/actions/runs/44182379156). | No formal review submissions or inline threads returned |
| [#584](https://github.com/vsimko/gfabasic-rust/pull/584) | AI-agent security policy and workflow hardening | `efde37b3e6ee811d09a82d31b1d17f9692e22aa8` | All 11 recorded PR workflow runs completed successfully, including Rust, docs, workflow/skill validation, security audit, SBOM, Amiga reference matrix, CLI probe, preview, benchmarks, and stable PR gate. [Stable gate](https://github.com/vsimko/gfabasic-rust/actions/runs/44188927691). | No formal review submissions or inline threads returned |

## Baseline findings

- **Sample size:** 3 PRs, one contributor account, one short period, and related agent/process work. Generalization is not justified.
- **First-head CI:** two sampled PRs have recorded successful initial validation paths. #581 had one preview smoke-test failure caused by an HTTP 404 on the deployed preview URL; a later preview run succeeded. The evidence does not establish a code defect.
- **Fix-up pushes:** unavailable. The PRs contain 3, 23, and 34 commits respectively, but these are commit totals, not counts of post-submission pushes or fix-ups.
- **Substantive reviews:** no formal reviews or inline review threads were returned for these three PRs. This is not evidence that no informal review occurred.
- **Follow-up work:** issue [#585](https://github.com/vsimko/gfabasic-rust/issues/585) records stale private-asset paths discovered by workflows after #584 merged. The observed mismatch is in private-asset repository contents versus configured paths; the available evidence does not attribute it to the security changes in #584.
- **Human blockers:** #584 explicitly retained #583 (account-level security verification) and #472 (required branch-protection check). These are not agent implementation failures; they are external maintainer actions.
- **Mutation evidence:** #582's scheduled mutation workflow had not yet supplied a verified killed-mutant result at the time of this sample. The PR must not be credited with a demonstrated mutation score.

## Actionable improvement

At merge time, update the PR's validation section from “pending CI” to the exact observed outcomes, including failed-then-recovered checks and intentionally skipped jobs. #582's PR description still says validation is pending despite the successful CI runs, whereas #584 records the final outcomes and limitations. This is a documentation/evidence-quality improvement; it does not require a new workflow or dependency.

## Recommendation on recurring measurement

Do not add a dashboard, telemetry, scheduled evaluation workflow, or numeric target. The sample is too small and attribution/fix-up-push data are incomplete. Revisit only after a larger, deliberately selected sample across distinct task types and contributors can be assembled with consistent attribution and review data. For now, apply the PR-description improvement and keep the rubric available for occasional manual use.
