//! Compiler implementation boundaries.
//!
//! The source map and strict UTF-8 transport loader, structured diagnostics,
//! and minimal help/version CLI are implemented building blocks. The backend,
//! frontend, IR, and semantic-analysis modules remain architectural
//! placeholders. No module implements or claims support for any GFA BASIC
//! dialect or language feature.

pub mod backend;
pub mod cli;
pub mod diagnostics;
pub mod frontend;
pub mod ir;
pub mod semantic;
pub mod source;
