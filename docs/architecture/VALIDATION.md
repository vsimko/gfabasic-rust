---
type: "Architecture"
title: "Repository consistency validation"
---

# Repository consistency validation

See [ADR 0013](adr/0013-repository-consistency-validation.md).

The initial validation tooling uses Python 3 standard library only and declarative JSON rule files. It has no package installation step, network access, dynamic evaluation, shell execution, or executable code in rule files.

Rules are stored separately from the validator. Each rule has a stable unique ID, description, enabled flag, severity, check type, check parameters, and evidence reference. Unknown rule fields and unsupported check types fail clearly. Repository paths must not escape the repository root.

Supported initial checks:

- `file_exists`: verifies that a repository-relative path names a file.
- `text_contains`: verifies that a file contains an exact UTF-8 text fragment.

Run the checks from the repository root:

```sh
python3 scripts/validate_repository.py
python3 -m unittest discover -s tests/tooling
```

A rule must identify its authoritative source. Do not infer implementation status from keywords, issue titles, commits, or unverified prose. A passing check validates only its stated condition; it does not prove general compiler or dialect compatibility.

Keep checks deterministic and offline. Use warnings for exploratory checks and errors only for reliable invariants. Disable a rule explicitly and document why rather than deleting it to hide a discrepancy. Add fixtures and tests for every new check type.

The validator implementation is tracked in [#249](https://github.com/vsimko/gfabasic-rust/issues/249). The website data generator is tracked in [#250](https://github.com/vsimko/gfabasic-rust/issues/250).

## Generated website data

`validation/project.json` is the explicit, versioned project-status model. Each status claim names repository evidence; the generator checks that evidence files exist and reads workspace package names from `Cargo.toml`. It writes deterministic output to `site/data/project.json`.

Run the complete local workflow from the repository root:

```sh
python3 scripts/generate_project_data.py
python3 scripts/validate_repository.py
python3 -m unittest discover -s tests/tooling
```

The Pages workflow runs these steps before uploading the site artifact. Edit the model when a project status changes, update its evidence references, and regenerate the site data. Do not hand-edit generated JSON or infer statuses from unstructured issue/commit text.


## OKF v0.2 documentation bundle

The only canonical human-readable documentation bundle is `docs/`. Its root `index.md` declares OKF version `0.2`; all other non-reserved Markdown files are concepts with YAML frontmatter and a non-empty `type`. Nested `index.md` files provide directory navigation and have no frontmatter. Do not create a parallel `knowledge/` tree or hand-maintain website copies.

Run the repository-owned validator and website generator:

```sh
python -m pip install PyYAML==6.0.2 markdown-it-py==3.0.0
python scripts/validate_okf.py
python scripts/generate_docs_site.py
python -m unittest discover -s scripts/tests -p "test_*.py"
```

The validator is stricter than OKF's minimum conformance floor where the project needs reliable navigation: it checks local links and anchors, complete directory indexes, and reachability. The generator uses safe Markdown rendering (raw HTML disabled), deterministic paths and ordering, and writes build-only pages under `site/docs/`. PR previews and the protected `main` deployment build those pages from `docs/` before publishing.
