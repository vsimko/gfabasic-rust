//! Compiler implementation boundaries.
//!
//! The compiler contains source-map/loading primitives, structured
//! diagnostics, an evidence-gated Amiga GFA BASIC 3.5 combinatorics frontend,
//! an initial narrow typed IR, and a temporary native-code bootstrap bridge.
//! General language semantics and the planned LLVM backend remain unimplemented.

pub mod backend;
pub mod cli;
pub mod diagnostics;
pub mod frontend;
pub mod formatter;
pub mod ir;
pub mod prototype;
pub mod semantic;
pub mod source;
