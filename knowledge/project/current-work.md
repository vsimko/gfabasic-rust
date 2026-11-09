---
type: Project Status Guide
title: Find current project work
description: How to determine current implementation status, active priorities, and blockers.
tags: [project, status, roadmap, issues]
sources:
  - resource: ../../ROADMAP.md
    title: Project roadmap
  - resource: https://github.com/vsimko/gfabasic-rust/issues
    title: GitHub issue tracker
---

# Current status

The [roadmap](../../ROADMAP.md) provides the project-level snapshot, delivery phases, and implementation maturity. It is a summary and can lag behind individual task discussion.

# Find active work

1. Open [all open issues](https://github.com/vsimko/gfabasic-rust/issues).
2. Review labels and dependencies. Check `needs-human` issues first.
3. Read the full issue body and comments.
4. Check linked pull requests and their CI status.
5. Confirm implementation claims against source code and tests.

# Status rules

- **Implemented** means code exists and relevant checks pass.
- **Research** means evidence gathering or source analysis; it is not implementation support.
- **Blocked** means a required external decision, source, or artifact is missing.
- **Planned** means implementation is not present.

A closed documentation or architecture issue does not prove that a compiler feature is implemented.
