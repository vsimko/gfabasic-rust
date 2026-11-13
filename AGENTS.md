# AI agent and human-blocker policy

This file is the repository-wide instruction entry point for AI coding agents. Read it before taking project-management or implementation actions. The canonical documentation bundle is [`docs/index.md`](docs/index.md); follow its OKF v0.2 conventions for all documentation changes. The project Agent Skills are indexed in [`.agents/skills/README.md`](.agents/skills/README.md).

## Agent skill routing

Before performing a matching task, read and follow its skill. Skills provide task procedures; the linked documentation remains the canonical source of project facts and policy.

- For Amiga GFA BASIC reference-interpreter probes, .LST fixtures, or native Amiberry Merge + Run debugging, use [`amiga-gfa-basic-regression-testing`](.agents/skills/amiga-gfa-basic-regression-testing/SKILL.md) and [the probe-writing guide](docs/testing/amiga-gfa-basic-probes.md).
- For API registry evidence reviews, semantic verification, or agent review packages/results, use [`api-evidence-verification`](.agents/skills/api-evidence-verification/SKILL.md) and [AI-assisted API verification](docs/architecture/AI-VERIFICATION.md).
- For historical Amiga GFA BASIC manual/book research, OCR/page-image review, command/API extraction, or version applicability, use [`amiga-gfa-basic-documentation-research`](.agents/skills/amiga-gfa-basic-documentation-research/SKILL.md) and [the research index](docs/research/index.md).
- For adding or editing conformance-corpus fixtures, sidecars, or manifest entries, use [`corpus-fixture-authoring`](.agents/skills/corpus-fixture-authoring/SKILL.md) and [the corpus contract](tests/corpus/README.md).

If more than one task type applies, use all relevant skills. Do not treat a skill as a replacement for reading the linked source-of-truth documentation.

## Human actions are tracked as issues

Any action that requires a human decision, approval, credential, permission, external account access, physical action, or other manual intervention **and blocks project progress** MUST be represented by an open GitHub issue.

- Apply the repository label `needs-human` to every such issue. This label is reserved for work that cannot proceed until a human acts.
- If the blocker prevents dependent work across a workstream or the project, also describe its impact explicitly in the issue body.
- The issue MUST state: **Human action required**, why the action is needed, the exact decision/action requested, who can perform it (if known), what work is blocked, and what becomes unblocked afterward.
- Link dependent issues and pull requests where practical. Keep the issue open until the human action is completed and verified.
- Do not silently close, downgrade, or remove the blocker label while the required human action remains outstanding.
- Do not make owner-only decisions on the owner's behalf (for example, selecting the project's software license), fabricate credentials/approvals, or bypass access controls.

## Agent workflow

1. Before starting work, inspect open issues and identify issues labelled `needs-human`.
2. Treat those issues as explicit blockers; do not attempt the decision/action yourself.
3. Continue all independent, unblocked work. A human blocker must not unnecessarily stop unrelated project work.
4. If an agent discovers a new human blocker, create an issue immediately, include the required details above, and apply `needs-human`.
5. If the `needs-human` label does not exist or cannot be applied, report that limitation clearly to the maintainer. Do not substitute an unrelated label or claim the blocker is correctly labelled.
6. When the human action is completed, record the outcome in the issue, remove `needs-human` only when appropriate, and resume dependent work.

## Meaning of the label

`needs-human` means that an explicit human action is required before the issue (or its dependent work) can progress. It is distinct from ordinary technical complexity, agent work in progress, and external tasks that can be completed autonomously.

The issue tracker is the source of truth for blockers; do not keep human-blocking requirements only in chat, comments, or private agent notes.

## Repository consistency validation

Read `docs/architecture/VALIDATION.md` before changing rules or website status claims. Keep checks declarative and evidence-based. Do not infer implementation support from prose, issue titles, or commit messages. Avoid new dependencies without a demonstrated need. Update fixtures, tests, and documentation when validation behavior changes.
