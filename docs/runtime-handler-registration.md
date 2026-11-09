# Runtime handler registration

Runtime handler IDs are stable API declaration IDs, not Rust symbol names.
Language-core IDs use `gfa.language.<category>.<name>`; Amiga-profile IDs
use `gfa.amiga.<category>.<name>`. IDs are case-sensitive and must match the
registry declaration exactly.

Each registration carries an explicit API profile and a typed Rust function
signature. Resolution requires exactly one registration with the expected
profile and signature. Missing, duplicate, stale, wrong-namespace, wrong-profile,
and wrong-signature mappings are errors; there is no generic fallback handler.

Only declarations whose implementation status is `implemented` or `partial`
may be resolved. Planned, research-only, and not-implemented declarations are
rejected even if a function with the same ID happens to be registered.

The runtime crate exposes this validation contract in
`gfabasic_runtime::handlers`. It is intentionally independent of SDL3.
Registry evidence, Linux target support, and conformance remain separate
requirements: adding a Rust registration does not promote any declaration to
supported or conformant. The current Amiga 3.5 catalogue marks the initial
combinatorics declarations as not implemented, so this change does not register
them as callable runtime features.
