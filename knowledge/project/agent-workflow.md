---
type: Agent Workflow
title: Repository discovery and agent workflow
description: Required steps for an AI agent to reconstruct project context and complete issue-driven work.
tags: [agents, workflow, github, issues]
sources:
  - resource: ../../AGENTS.md
    title: Repository agent instructions
  - resource: ../../docs/contributor/README.md
    title: Contributor documentation
---

# Goal

An agent must be able to enter the repository with no chat history and reconstruct the project state from version-controlled sources.

# Discovery procedure

1. Read the root [README](../../README.md).
2. Read [AGENTS.md](../../AGENTS.md) for mandatory instructions and human-blocker policy.
3. Read the [project roadmap](../../ROADMAP.md) to understand current maturity and delivery order.
4. Read the [compatibility policy](../compatibility/baseline.md) and [evidence policy](../compatibility/evidence.md).
5. Inspect open GitHub [issues](https://github.com/vsimko/gfabasic-rust/issues) and [pull requests](https://github.com/vsimko/gfabasic-rust/pulls).
6. Check issues labelled `needs-human`. Do not bypass decisions, credentials, approvals, or permissions that require a human.
7. Select a concrete issue. Read its acceptance criteria, comments, linked issues, and canonical documents before changing code.
8. Implement the scoped change, add or update tests, and update the issue with evidence and blockers.
9. Open a pull request linked to the issue. Report only checks that actually ran.

# Source of truth

GitHub issues are the source of truth for task scope, acceptance criteria, blockers, and implementation discussion. Source code and tests are the source of truth for implemented behavior. The roadmap is a navigation summary, not a substitute for issue details or test results.

Do not use chat history, commit messages, or private agent notes as authoritative project state.

# Detailed instructions

- [Repository-wide agent instructions](../../AGENTS.md)
- [Contributor guide](../../docs/contributor/README.md)
- [Development setup](../../docs/contributor/development-setup.md)
