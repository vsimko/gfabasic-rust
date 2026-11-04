//! Executable entry point for the gfabasic command.

use std::process::ExitCode;

fn main() -> ExitCode {
    ExitCode::from(gfabasic_compiler::cli::run(std::env::args().skip(1)))
}
