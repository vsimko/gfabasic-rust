//! Minimal command-line entry point plus the experimental FACT/PRINT build.

use std::io::{self, Write};
use std::path::Path;

const HELP: &str = "gfabasic - GFA BASIC compiler

Usage:
  gfabasic [--help]
  gfabasic [--version] [--verbose]
  gfabasic help
  gfabasic version [--verbose]
  gfabasic build <source.bas> -o <output>

Commands currently available:
  help       Show this help
  version    Show compiler version and optional build identity
  build      Compile the narrow Amiga GFA BASIC 3.5 combinatorics/PRINT prototype subset

The prototype accepts FACT(n), VARIAT(n,k), and COMBIN(n,k) assignments
with non-negative integer-literal arguments, followed by PRINT variable lists.
The current evaluator restricts n to 0..=20 and variables to x, y, and z.
It uses UTF-8 transport and the host 'cc' toolchain as temporary bootstrap
choices; neither is a claim about historical source encoding or interpreter
output formatting. General GFA BASIC support and the LLVM backend are not
implemented. The check command is unavailable.
";

/// Run the implemented CLI subset and return its process exit code.
pub fn run(args: impl IntoIterator<Item = String>) -> u8 {
    run_with_io(args, &mut io::stdout().lock(), &mut io::stderr().lock())
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
        [arg, verbose] if (arg == "--version" || arg == "version") && verbose == "--verbose" => {
            let _ = writeln!(out, "gfabasic {}", env!("CARGO_PKG_VERSION"));
            let build_id = option_env!("GFA_BASIC_BUILD_ID").unwrap_or("unknown");
            let _ = writeln!(out, "build-id: {build_id}");
            0
        }
        [command, input, flag, output] if command == "build" && flag == "-o" => {
            match crate::prototype::compile_file(Path::new(input), Path::new(output)) {
                Ok(()) => {
                    let _ = writeln!(
                        out,
                        "built {} (Amiga GFA BASIC 3.5 FACT/PRINT prototype)",
                        output
                    );
                    0
                }
                Err(message) => {
                    let _ = writeln!(err, "error: {message}");
                    1
                }
            }
        }
        [command, ..] if command == "check" => {
            let _ = writeln!(
                err,
                "error: 'check' is not implemented; the general compiler pipeline is unavailable"
            );
            2
        }
        [command, ..] if command == "build" => {
            let _ = writeln!(
                err,
                "error: expected 'gfabasic build <source.bas> -o <output>'"
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
    use std::fs;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEST: AtomicUsize = AtomicUsize::new(0);

    fn invoke(args: &[&str]) -> (u8, String, String) {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let code = run_with_io(args.iter().map(|arg| (*arg).to_owned()), &mut out, &mut err);
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
        assert!(out.contains("combinatorics/PRINT prototype"));
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
    fn build_requires_source_and_output_arguments() {
        let (code, out, err) = invoke(&["build"]);
        assert_eq!(code, 2);
        assert!(out.is_empty());
        assert!(err.contains("expected 'gfabasic build"));
    }

    #[test]
    fn build_compiles_the_verified_fact_fixture() {
        let cc_version = Command::new("cc")
            .arg("--version")
            .output()
            .expect("CLI native build test requires a host C compiler (`cc`)");
        assert!(
            cc_version.status.success(),
            "CLI native build test requires a working host C compiler (`cc`)"
        );

        let sequence = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "gfabasic-cli-test-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).unwrap();
        let input = directory.join("fact.bas");
        let output = directory.join("fact");
        fs::write(&input, "x=FACT(0)\ny=FACT(1)\nz=FACT(2)\nPRINT x,y,z\n").unwrap();

        let input = input.to_str().unwrap();
        let output_arg = output.to_str().unwrap();
        let (code, out, err) = invoke(&["build", input, "-o", output_arg]);
        assert_eq!(code, 0, "{err}");
        assert!(out.contains("FACT/PRINT prototype"));
        let run = Command::new(&output).output().unwrap();
        assert!(run.status.success());
        assert_eq!(
            String::from_utf8(run.stdout)
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>(),
            ["1", "1", "2"]
        );

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn check_is_not_advertised_as_available() {
        let (code, out, err) = invoke(&["check"]);
        assert_eq!(code, 2);
        assert!(out.is_empty());
        assert!(err.contains("general compiler pipeline is unavailable"));
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
