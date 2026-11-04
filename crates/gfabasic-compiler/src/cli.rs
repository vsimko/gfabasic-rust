//! Minimal command-line entry point.
//!
//! This first increment intentionally implements only help and version output.
//! Compilation and project loading remain unavailable until their prerequisites
//! are implemented; the CLI must not imply that source compilation works.

use std::io::{self, Write};

const HELP: &str = "gfabasic - GFA BASIC compiler

Usage:
  gfabasic [--help]
  gfabasic [--version] [--verbose]
  gfabasic help
  gfabasic version [--verbose]

Commands currently available:
  help       Show this help
  version    Show compiler version and optional build identity

The build and check commands are not implemented yet.
";

/// Run the implemented CLI subset and return its process exit code.
pub fn run(args: impl IntoIterator<Item = String>) -> u8 {
    run_with_io(
        args,
        &mut io::stdout().lock(),
        &mut io::stderr().lock(),
    )
}

fn run_with_io<I, W, E>(args: I, out: &mut W, err: &mut E) -> u8
where
    I: IntoIterator<Item = String>,
    W: Write,
    E: Write,
{
    let args: Vec<String> = args.into_iter().collect();

    match args.as_slice() {
        [] => {
            let _ = out.write_all(HELP.as_bytes());
            0
        }
        [arg] if arg == "-h" || arg == "--help" || arg == "help" => {
            let _ = out.write_all(HELP.as_bytes());
            0
        }
        [arg] if arg == "--version" || arg == "version" => {
            let _ = writeln!(out, "gfabasic {}", env!("CARGO_PKG_VERSION"));
            0
        }
        [arg, verbose]
            if (arg == "--version" || arg == "version") && verbose == "--verbose" =>
        {
            let _ = writeln!(out, "gfabasic {}", env!("CARGO_PKG_VERSION"));
            let build_id = option_env!("GFA_BASIC_BUILD_ID").unwrap_or("unknown");
            let _ = writeln!(out, "build-id: {build_id}");
            0
        }
        [command, ..] if command == "build" || command == "check" => {
            let _ = writeln!(
                err,
                "error: '{command}' is not implemented yet; no source compilation is available"
            );
            2
        }
        _ => {
            let _ = writeln!(err, "error: invalid command or arguments");
            let _ = err.write_all(b"Run 'gfabasic --help' for usage.\n");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run_with_io;

    fn invoke(args: &[&str]) -> (u8, String, String) {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let code = run_with_io(
            args.iter().map(|arg| (*arg).to_owned()),
            &mut out,
            &mut err,
        );
        (
            code,
            String::from_utf8(out).expect("stdout is UTF-8"),
            String::from_utf8(err).expect("stderr is UTF-8"),
        )
    }

    #[test]
    fn no_arguments_show_help() {
        let (code, out, err) = invoke(&[]);
        assert_eq!(code, 0);
        assert!(out.contains("Usage:"));
        assert!(out.contains("not implemented yet"));
        assert!(err.is_empty());
    }

    #[test]
    fn help_flag_shows_usage() {
        let (code, out, err) = invoke(&["--help"]);
        assert_eq!(code, 0);
        assert!(out.starts_with("gfabasic - GFA BASIC compiler"));
        assert!(err.is_empty());
    }

    #[test]
    fn version_prints_package_version() {
        let (code, out, err) = invoke(&["version"]);
        assert_eq!(code, 0);
        assert_eq!(out, format!("gfabasic {}\n", env!("CARGO_PKG_VERSION")));
        assert!(err.is_empty());
    }

    #[test]
    fn verbose_version_includes_build_identity() {
        let (code, out, err) = invoke(&["--version", "--verbose"]);
        assert_eq!(code, 0);
        assert!(out.contains(&format!("gfabasic {}\n", env!("CARGO_PKG_VERSION"))));
        assert!(out.contains("build-id: "));
        assert!(err.is_empty());
    }

    #[test]
    fn compilation_commands_fail_explicitly_until_implemented() {
        for command in ["build", "check"] {
            let (code, out, err) = invoke(&[command]);
            assert_eq!(code, 2);
            assert!(out.is_empty());
            assert!(err.contains("is not implemented yet"));
        }
    }

    #[test]
    fn unknown_arguments_return_usage_error() {
        let (code, out, err) = invoke(&["--unknown"]);
        assert_eq!(code, 2);
        assert!(out.is_empty());
        assert!(err.contains("invalid command or arguments"));
        assert!(err.contains("gfabasic --help"));
    }
}
