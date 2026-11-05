//! Compiler implementation boundaries.
//!
//! The compiler contains source-map/loading primitives, structured
//! diagnostics, and an evidence-gated Amiga GFA BASIC 3.5 FACT/PRINT frontend
//! plus a temporary native-code bootstrap bridge. General language semantics,
//! project-owned typed IR, and the planned LLVM backend remain unimplemented.

pub mod backend;
pub mod cli;
pub mod diagnostics;
pub mod frontend;
pub mod ir;
pub mod prototype;
pub mod semantic;
pub mod source;
