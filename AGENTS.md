# AI agent and human-blocker policy

This file is the repository-wide instruction entry point for AI coding agents. Read it before taking project-management or implementation actions.

## Start here when entering the repository

Do not depend on chat history or an inherited task summary. Reconstruct the current project state from canonical sources:

1. Read the root [README](README.md).
2. Read the [OKF knowledge bundle](knowledge/index.md). Follow its directory indexes and concept links to discover project knowledge progressively.
3. Read the [project roadmap](ROADMAP.md) for current implementation status and delivery order.
4. Read the [contributor guide](docs/contributor/README.md) and the detailed documents relevant to the selected task.
5. Inspect open GitHub issues and pull requests. Issues are the source of truth for task scope, acceptance criteria, blockers, and discussion. Check `needs-human` issues before choosing work.
6. Select a concrete issue, read its comments and linked material, and keep implementation, tests, and documentation aligned.

The OKF bundle is the agent-facing knowledge map. Follow its `index.md` files before opening unrelated documents. Do not depend on chat history, commit messages, or private notes as authoritative project state.

## Compatibility baseline

The practical reference implementation for current development is the Commodore Amiga GFA BASIC 3.52 interpreter. The goal is to compile the largest practical set of programs and features that this interpreter can interpret. Treat observed behavior as the primary compatibility evidence. Books and manuals are supporting sources; binary analysis is a separate diagnostic evidence class. A comparison with an independently identified 3.5 interpreter is a later validation phase and must not block current compiler work.

Follow [the compatibility concept](knowledge/compatibility/baseline.md) and [the detailed evidence policy](docs/architecture/COMPATIBILITY-EVIDENCE.md). Do not claim that a 3.52 observation proves 3.5 behavior.

## Human actions are tracked as issues

Any action that requires a human decision, approval, credential, permission, external account access, physical action, or other manual intervention **and blocks project progress** MUST be represented by an open GitHub issue.

- Apply the repository label `needs-human` to every such issue. This label is reserved for work that cannot proceed until a human acts.
- If the blocker prevents dependent work across a workstream or the project, also describe its impact explicitly in the issue body.
- The issue MUST state: **Human action required**, why the action is needed, the exact decision/action requested, who can perform it (if known), what work is blocked, and what becomes unblocked afterward.
- Link dependent issues and pull requests where practical. Keep the issue open until the human action is completed and verified.
- Do not silently close, downgrade, or remove the blocker label while the required human action remains outstanding.
- Do not make owner-only decisions on the owner's behalf, fabricate credentials/approvals, or bypass access controls.

## Agent workflow

1. Before starting work, inspect open issues and identify issues labelled `needs-human`.
2. Treat those as explicit blockers; do not attempt the decision/action yourself.
3. Continue all independent, unblocked work. A human blocker must not unnecessarily stop unrelated project work.
4. If an agent discovers a new human blocker, create an issue immediately, include the required details above, and apply `needs-human`.
5. If the `needs-human` label does not exist or cannot be applied, report that limitation clearly to the maintainer.
6. When the human action is completed, record the outcome in the issue, remove `needs-human` only when appropriate, and resume dependent work.

## Repository consistency validation

Read [VALIDATION.md](docs/architecture/VALIDATION.md) before changing repository rules or website status claims. Keep checks declarative and evidence-based. Do not infer implementation support from prose, issue titles, or commit messages. Avoid new dependencies without a demonstrated need.
