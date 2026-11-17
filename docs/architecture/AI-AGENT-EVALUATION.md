---
type: "Architecture"
title: "AI agent contribution evaluation pilot"
---

# AI agent contribution evaluation pilot

## Purpose and guardrails

This document records a small retrospective baseline for agent-assisted contributions in this repository. It is intended to improve the contribution process, not to rank agents or people.

The pilot follows issue [#578](https://github.com/vsimko/gfabasic-rust/issues/578):

- No leaderboard, person/agent ranking, or productivity score.
- Raw PR count, lines changed, and elapsed time are not success measures.
- No numeric targets are set from this sample.
- No dashboard, scheduled workflow, telemetry service, or new dependency is introduced.
- Facts from GitHub/CI are separated from interpretation and from information known only through this project conversation.

## Compact rubric

| Dimension | Operational definition | Evidence source | Interpretation |
|---|---|---|---|
| First-head CI reliability | Result of the required PR gate and relevant required workflows for the first PR head submitted for review | PR creation time, first head SHA, GitHub Actions runs | Record the first result and final result separately. A later green run does not erase the initial failure. |
| Rework before green | Number of commits added after the PR was opened, plus the reason for each observed corrective change | PR commit timestamps and CI failure logs | This is only a rework proxy. GitHub data available to this pilot does not reliably expose push count; commits are not equivalent to pushes. |
| Review findings | Substantive human review comments that required code, test, or design changes | PR reviews and review comments | If no human review is recorded, mark this dimension not assessable; do not interpret silence as approval or quality. |
| Escaped regressions and follow-ups | Linked issue/PR or post-merge workflow failure attributable to the change | Issue/PR cross-links and Actions runs on the merge commit | Record the concrete failure and mitigation; do not assign a speculative severity score. |
| Acceptance and evidence quality | Whether issue criteria are complete, commands/results are recorded, and unresolved validation is explicit | Issue body, PR body, workflow artifacts, test results | An implementation PR can merge while the parent issue remains open if an acceptance item is still pending. |
| Scope discipline and hand-off | Unrequested changes, corrections after user clarification, duplicate effort, and human-only blockers | Conversation/task provenance, changed files, needs-human issues | Separate avoidable agent rework from settings or decisions that only a maintainer can make. |

## Sample selection and provenance

The sample contains four consecutive PRs from the 2026-11-17 workstream:

- [#581](https://github.com/vsimko/gfabasic-rust/pull/581) — contributor process and PR template.
- [#582](https://github.com/vsimko/gfabasic-rust/pull/582) — Rust source-map properties and scheduled mutation workflow.
- [#584](https://github.com/vsimko/gfabasic-rust/pull/584) — AI-agent security policy and workflow hardening.
- [#586](https://github.com/vsimko/gfabasic-rust/pull/586) — follow-up restricting privileged private-asset workflows to manual dispatch.

This is a purposive sample across process documentation, Rust testing, and workflow/security changes. It is not a random or statistically representative sample of all repository contributions.

Agent involvement is known from the task provenance in this project conversation. GitHub identifies the repository owner as the PR author; that metadata alone does not establish agent authorship. No other PRs are included because their agent attribution cannot be established from the available evidence.

For each PR, the first submitted head is the latest commit whose committer timestamp is at or before the PR creation time. The count of commits after PR creation is reported as a rework proxy, not as a push count. CI results refer to the first observed Actions runs for that head and the final required PR gate.

## Baseline observations

| PR | First submitted head and first CI result | Commits after PR opened | Human review evidence | Follow-up / acceptance status |
|---|---|---:|---|---|
| #581 | Head 2252822. Stable PR gate passed. The first preview smoke test returned HTTP 404; a later preview run passed. [Gate](https://github.com/vsimko/gfabasic-rust/actions/runs/44176081520), [first preview](https://github.com/vsimko/gfabasic-rust/actions/runs/44176081545), [successful preview retry](https://github.com/vsimko/gfabasic-rust/actions/runs/44177849222). | 0 (3 total) | No human review records or substantive review comments. | Issue #577 closed after merge. No follow-up defect was recorded in this sample. |
| #582 | Head bdb4c7b. Documentation validation failed because the testing index lacked a direct entry for the new pilot document; the stable PR gate consequently failed. [Docs run](https://github.com/vsimko/gfabasic-rust/actions/runs/44178248935), [gate run](https://github.com/vsimko/gfabasic-rust/actions/runs/44178268722). Final gate passed after fixes. [Final gate](https://github.com/vsimko/gfabasic-rust/actions/runs/44182379156). | 20 (23 total) | No human review records or substantive review comments. | The user corrected an out-of-scope mutation experiment that briefly targeted a Python validator; the final mutation target is Rust-only. The temporary Rust mutant's CI attempt stopped at rustfmt before tests executed. Issue #579 remains open until the first scheduled mutation report is reviewed. |
| #584 | Head f7b1bbe. Documentation validation failed because the architecture index lacked a direct entry for the new policy. The first PR-gate attempt also hit a GitHub API HTTP 500. [Docs run](https://github.com/vsimko/gfabasic-rust/actions/runs/44187844410), [gate run](https://github.com/vsimko/gfabasic-rust/actions/runs/44187846554). Final gate passed. [Final gate](https://github.com/vsimko/gfabasic-rust/actions/runs/44188927691). | 13 (34 total) | No human review records or substantive review comments. | After merge, the private-media inventory and reference-analysis push workflows failed with HTTP 404 because configured private-asset paths were stale. [Inventory run](https://github.com/vsimko/gfabasic-rust/actions/runs/44189649278), [analysis run](https://github.com/vsimko/gfabasic-rust/actions/runs/44189649086). Issue #585 records the repair; PR #586 makes both workflows manual-only until then. Issue #580 remains open for account-level verification in #583. |
| #586 | Head b8db523. Documentation, Rust, workflow validation, security audit, SBOM, benchmark validation, preview, and stable PR gate all passed. [Stable gate](https://github.com/vsimko/gfabasic-rust/actions/runs/44189963968). | 0 (3 total) | No human review records or substantive review comments. | Follow-up mitigation merged. The underlying private asset paths remain unresolved in #585; no claim is made that those workflows are operational. |

### Findings

**Positive signals**

- All four PRs were ultimately merged with a successful stable PR gate.
- The final PR #582 and #584 runs passed their relevant documentation, workflow, Rust, and integration checks.
- Human-only platform decisions were not silently guessed: #580 remains labelled needs-human and links to #583; branch protection remains separately tracked in #472.
- The post-merge private-asset failures were recorded in #585 and the risky automatic triggers were removed in #586 rather than repeatedly dispatching the broken workflows.

**Quality gaps and rework**

- Two PRs (#582 and #584) failed their first documentation validation because a new architecture/testing document was not added to the required direct index. This is a repeatable pre-PR check that should be run before opening a PR.
- The #582 workstream used 20 commits after opening the PR; #584 used 13. These are commit counts, not push counts, and they include several one-file corrective commits. They indicate avoidable review churn, not a measured productivity score.
- The #582 mutation experiment briefly targeted a Python implementation despite the Rust-only intent, and the manual Rust mutant was not actually executed by the test step. The scheduled workflow is in place, but issue #579 correctly remains open pending its first completed report.
- The #584 workflow review did not fully account for automatic push-triggered side effects. Two privileged workflows ran after merge and failed against stale private asset paths. This is an escaped workflow-configuration problem, mitigated by #586 and tracked in #585.
- None of the four PRs has a recorded human review. Therefore this sample cannot measure the quality of independent review findings or establish that the merged work received substantive peer review.

**Human blockers versus avoidable rework**

- Secret scope, GitHub App installation permissions, collaborator access, Actions account settings, and external agent runtime details are not exposed by the repository integration. These are correctly tracked as human actions in #583.
- Requiring the exact PR gate in branch protection is tracked separately in #472.
- Missing documentation index entries, excessive one-file commits, and incomplete trigger/side-effect inspection are repository/agent-process improvements, not human-only blockers.

## Actionable improvements

1. **Workflow-change checklist:** the PR template now asks contributors changing workflows to review event triggers, effective permissions, secrets, fork/Dependabot behavior, checkout credentials, artifacts, and post-merge side effects. This is the follow-up change in [PR #589](https://github.com/vsimko/gfabasic-rust/pull/587).
2. **Batch related edits:** prepare related file changes together and commit one coherent logical unit (or a small number of reviewable units), rather than creating one commit per file or per small correction.
3. **Run documentation preflight before opening a PR:** validate OKF direct-index entries, links, and Markdown locally when adding a document. The repository already documents these commands in the contributor guide.
4. **Audit event side effects before merge:** for workflow changes, inspect not only PR execution but also push-to-main, manual dispatch, reusable workflow calls, release, artifact deletion, and publication behavior.
5. **Do not close an issue merely because its implementation PR merged:** leave it open when a scheduled test, mutation report, external verification, or human decision remains outstanding. Issue #579 is the current example.

## Recommendation

This pilot does not justify recurring measurement, a dashboard, scheduled reporting, or numeric targets. The sample is small, same-day, one-workstream, and has no recorded independent reviews. Keep this as a one-off baseline. Revisit only after a broader set of agent-assisted tasks across different task types has reliable attribution and independent review evidence.

No recurring automation or new dependency is recommended.
