# Repository consistency validation

See [ADR 0013](adr/0013-repository-consistency-validation.md).

The initial validation tooling uses Python 3 standard library only and declarative JSON rule files. It has no package installation step, network access, dynamic evaluation, shell execution, or executable code in rule files.

Rules are stored separately from the validator. Each rule has a stable unique ID, description, enabled flag, severity, check type, check parameters, and evidence reference. Unknown rule fields and unsupported check types must fail clearly. Repository paths must not escape the repository root.

A rule must identify its authoritative source. Do not infer implementation status from keywords, issue titles, commits, or unverified prose. A passing check validates only its stated condition; it does not prove general compiler or dialect compatibility.

Keep checks deterministic and offline. Use warnings for exploratory checks and errors only for reliable invariants. Disable a rule explicitly and document why rather than deleting it to hide a discrepancy. Add fixtures and tests for every new check type.

The validator implementation is tracked in [#249](https://github.com/vsimko/gfabasic-rust/issues/249). The website data generator is tracked in [#250](https://github.com/vsimko/gfabasic-rust/issues/250). The command-line invocation will be documented with the validator implementation.
