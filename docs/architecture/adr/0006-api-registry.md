---
type: "Architecture Decision Record"
title: "ADR 0006: API declaration registry"
---

# ADR 0006: API declaration registry

- Status: Accepted
- Date: 2026-11-04

## Context

Compiler parsing/semantic analysis, API profile selection, runtime mapping, reference documentation, and conformance tests need a consistent declaration identity and metadata model. Duplicating spellings, signatures, version availability, and profile membership across Rust code and documentation would create drift. Research entries also need to be representable without implying implementation or support.

## Decision

1. Use a checked-in declarative registry as the canonical source for structured language/API declaration metadata.
2. Use TOML as the canonical editable format, with a versioned JSON Schema for the parsed data model.
3. Store files under api/registry/language-core/, api/registry/amiga/, and api/registry/atari-st/, grouped by logical module. Store schema definitions under api/schema/.
4. Give every declaration a stable globally unique, lowercase dot-separated ID: gfa.language.*, gfa.amiga.*, or gfa.atari-st.*. IDs are independent of spelling and version.
5. Keep source dialect/version, API profile, output target, evidence state, implementation state, support classification, and conformance state as separate fields.
6. Require structured syntax, signature, semantic contract, source provenance, implementation mapping, target support, tests, and documentation links. Research-only records may explicitly defer implementation/tests but cannot claim support.
7. Rust code implements behavior behind stable registry IDs; it must not independently redefine source spelling, syntax availability, signatures, or profile membership.
8. Tests remain independent fixtures. The registry links to tests but must not generate expected results from the implementation under test.
9. Generate reference metadata from the registry where useful. Generated sections must be reproducible, clearly marked, and must not overwrite authored explanation or source citations.
10. Validate duplicate IDs/names, overlapping availability, profile conflicts, schema validity, handler mappings, evidence, test IDs, and documentation anchors. Support claims require primary evidence and passing conformance tests.

The normative detail is maintained in [API-REGISTRY.md](../API-REGISTRY.md). The registry validator and generated-reference checks are implementation work under #55.

## Alternatives considered

- YAML or JSON as the canonical editable format: rejected in favor of TOML, which is familiar in Rust tooling and supports comments while retaining structured arrays/tables.
- Rust source as the only source of truth: rejected because it duplicates reference metadata and makes research-only declarations difficult to represent.
- Documentation pages as the canonical source: rejected because they are not a robust machine-readable input for compiler validation and code mapping.
- One record per source spelling without version/profile metadata: rejected because historical dialects and profiles may differ or conflict.
- A single overloaded status field: rejected because source evidence, implementation, support, and conformance are independent dimensions.
- Generating conformance expectations from registry/runtime implementation: rejected because it would make tests circular.

## Consequences

- CLI, parser, runtime, documentation generation, and conformance tooling must consume the same stable declaration IDs.
- The registry schema and compiler/runtime versions evolve independently.
- The validator must permit explicit research-only entries while preventing unsupported claims.
- Changes to canonical TOML records require schema validation and review of generated docs/tests.
- The registry model is accepted; no schema, registry file, validator, generator, or declaration is implemented by this ADR.

## References

- [API registry specification](../API-REGISTRY.md)
- [API profile model](../API-PROFILES.md)
- [CLI and project configuration](../CLI-AND-CONFIG.md)
- [Versioning policy](../VERSIONING.md)
- [Issue #12](https://github.com/vsimko/gfabasic-rust/issues/12)
- [Issue #55](https://github.com/vsimko/gfabasic-rust/issues/55)
