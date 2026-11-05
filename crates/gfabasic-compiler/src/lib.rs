//! Compiler implementation boundaries.
//!
//! The compiler currently contains source-map/loading primitives, structured
//! diagnostics, and an experimental, explicitly limited FACT/PRINT bootstrap
//! prototype. The general dialect-aware frontend, semantic analysis, typed IR,
//! and planned LLVM backend remain unimplemented.

pub mod backend;
pub mod cli;
pub mod diagnostics;
pub mod frontend;
pub mod ir;
pub mod prototype;
pub mod semantic;
pub mod source;
