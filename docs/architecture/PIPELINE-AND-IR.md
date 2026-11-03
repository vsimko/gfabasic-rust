# Compiler pipeline and intermediate representation

- Status: Proposed architecture; detailed IR and interfaces remain open under issue #6.

## Pipeline

1. Load source and preserve file identity.
2. Lex using an explicit dialect/version.
3. Parse into an AST with source spans.
4. Resolve declarations and names.
5. Perform type checking and semantic analysis.
6. Lower valid programs into project-owned typed IR.
7. Apply target/API-specific lowering where required.
8. Generate and verify LLVM IR.
9. Generate native objects and link the selected runtime.
10. Emit diagnostics and build artifacts.

## Responsibilities

The frontend owns language rules and semantic diagnostics. LLVM does not define GFA BASIC semantics. Project-owned IR must remain independent of SDL3, Linux, and CPU-specific assumptions. LLVM IR is a backend representation, not the canonical semantic model.

## IR design requirements

Decide how to represent explicit integer widths/signedness, floating-point values/conversions, strings, arrays, aggregates, memory operations, variables, scopes/lifetimes, branches, loops, labels, GOTO/GOSUB/RETURN, procedures/calls, runtime/profile API calls, and source provenance.

Define verification invariants and permitted transformations. Each stage must return structured diagnostics with source provenance. Do not enter code generation when semantic invariants fail.

Select LLVM version and Rust binding only after evaluating compatibility, maintenance, platform support, and clean-build requirements. Record the selection in an ADR. Verify LLVM modules before object emission.

Tracking: [issue #6](https://github.com/vsimko/gfabasic-rust/issues/6).
