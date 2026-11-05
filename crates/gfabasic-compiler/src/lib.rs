//! Compiler implementation boundaries.
//!
//! The source map, strict UTF-8 transport loader, structured diagnostics, and
//! minimal help/version CLI are implemented building blocks. The frontend
//! currently recognizes only the evidence-backed Amiga GFA BASIC 3.5 FACT/PRINT
//! vertical-slice subset. Semantic analysis, IR, backend, and runtime support
//! remain unimplemented; this is not yet a working compiler.

pub mod backend;
pub mod cli;
pub mod diagnostics;
pub mod frontend;
pub mod ir;
pub mod semantic;
pub mod source;
