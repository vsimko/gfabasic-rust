# API registry

- Status: Proposed; schema and validation rules remain open under issue #12.

Use one canonical machine-readable registry for declarations whose names, signatures, profile membership, runtime mapping, documentation, and tests must remain consistent. Potential entries include language statements/functions, common runtime functions, Amiga APIs, and Atari ST APIs. Do not populate it with unverified declarations.

Evaluate a schema with stable identifier/name, construct kind/syntax, parameter and return types, applicable dialect/version range, profile membership, source reference/page, evidence status, compiler support status, runtime mapping/target capability, documentation anchor, conformance test identifiers, and deprecation/conflict metadata.

Validation must detect malformed entries, duplicate identifiers, conflicting signatures, invalid profile membership, missing references, missing runtime mappings for claimed supported operations, and missing tests for conformance claims.

Generate reference documentation from the registry where practical. Explanatory documentation must not create a second conflicting declaration catalogue. Treat registry input as data: do not execute embedded code or fetch arbitrary URLs during compilation.

Tracking: [issue #12](https://github.com/vsimko/gfabasic-rust/issues/12).
