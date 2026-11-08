# ADR 0015: Listing-first compiler input

- Status: Accepted
- Date: 2026-11-08
- Deciders: Project owner

## Context

The current compiler frontend is a textual source frontend. Research into native tokenized `.GFA` files (including the separate `gfalist` utility) is useful for future compatibility work, but the native container, token encoding, version markers, and preservation rules are not yet specified or verified. Implementing that path now would add a second input representation before the compiler's language pipeline is mature.

## Decision

The implementation phase is **listing-first**:

1. The only compiler source input considered by the current implementation is textual `.LST` source.
2. `.LST` bytes are decoded according to the verified Amiga GFA BASIC 3.5 text encoding, BOM, and line-ending contract, then passed to the text lexer/parser.
3. The lexer, parser, AST/CST, semantic analysis, formatter, LSP, corpus fixtures, and compiler CLI are designed and tested against textual listing source only.
4. No tokenized `.GFA` container reader, header parser, opcode/token decoder, binary-preservation layer, or automatic `.GFA` detection is to be implemented or assumed in this phase.
5. Native `.GFA` support is deferred until the majority of the compiler pipeline for `.LST` works. It will require a separate design decision, verified format evidence, legally usable fixtures, and its own tests. It may later decode into the shared textual/frontend representation, but that architecture is not pre-decided here.

## Consequences

- Issue #120 is limited to text listing decoding and verified source composition.
- Issues #46, #47, #48, #49, #111, #113, #208, and #209 must not acquire binary-input requirements.
- Conformance and formatter fixtures used for implementation are textual `.LST` files. A fixture's historical origin does not justify making the compiler read native `.GFA`.
- Research records about `.GFA`, tokenized formats, and `gfalist` may remain in documentation, clearly marked as deferred research and not implementation scope.
- Documentation and CLI help must not imply that native `.GFA` input is currently supported or planned for the first compiler milestone.

## Revisit criteria

Revisit this decision only after the project owner considers the majority of the textual `.LST` compiler pipeline functional. A follow-up decision must define the exact native format/version scope and evidence requirements before implementation begins.
