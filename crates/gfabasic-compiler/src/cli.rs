//! Minimal command-line entry point plus the experimental FACT/PRINT build.
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

const HELP: &str = "gfabasic - GFA BASIC compiler

Usage:
  gfabasic [--help]
  gfabasic [--version] [--verbose]
  gfabasic help
  gfabasic version [--verbose]
  gfabasic fmt <source.bas> [--check] [-o <path> | --in-place]
  gfabasic build <source.bas> -o <output>

Commands currently available:
  help       Show this help
  version    Show compiler version and optional build identity
  fmt        Format the supported Amiga GFA BASIC 3.5 source subset
  build      Compile the narrow Amiga GFA BASIC 3.5 combinatorics/PRINT prototype subset

Formatter options:
  --check       Exit with status 1 when the source is not canonically formatted
  -o <path>     Write formatted output to a separate path (atomically)
  --in-place    Replace the source file atomically; never implied

The formatter currently accepts FACT, VARIAT, COMBIN, and PRINT prototype
syntax only. Unsupported syntax is diagnosed and never written. Identifier
spelling is preserved exactly. The formatter uses UTF-8 transport and LF
output; this is not a claim about historical Amiga source encoding.

The build prototype accepts FACT(n), VARIAT(n,k), and COMBIN(n,k) assignments
with non-negative integer-literal arguments, followed by PRINT variable lists.
The current evaluator restricts n to 0..=20 and variables to x, y, and z.
It uses the host cc toolchain as a temporary bootstrap choice. General
GFA BASIC support and the LLVM backend are not implemented.
";
static NEXT_TEMP_FILE: AtomicUsize = AtomicUsize::new(0);

pub fn run(args: impl IntoIterator<Item = String>) -> u8 {
    run_with_io(args, &mut io::stdout().lock(), &mut io::stderr().lock())
}

fn run_with_io<I, W, E>(args: I, out: &mut W, err: &mut E) -> u8
where I: IntoIterator<Item = String>, W: Write, E: Write {
    let args: Vec<String> = args.into_iter().collect();
    match args.as_slice() {
        [] => { let _ = out.write_all(HELP.as_bytes()); 0 }
        [arg] if arg == "-h" || arg == "--help" || arg == "help" => {
            let _ = out.write_all(HELP.as_bytes()); 0
        }
        [arg] if arg == "--version" || arg == "version" => {
            let _ = writeln!(out, "gfabasic {}", env!("CARGO_PKG_VERSION")); 0
        }
        [arg, verbose] if (arg == "--version" || arg == "version") && verbose == "--verbose" => {
            let _ = writeln!(out, "gfabasic {}", env!("CARGO_PKG_VERSION"));
            let build_id = option_env!("GFA_BASIC_BUILD_ID").unwrap_or("unknown");
            let _ = writeln!(out, "build-id: {build_id}"); 0
        }
        [command, ..] if command == "fmt" => run_fmt(&args[1..], out, err),
        [command, input, flag, output] if command == "build" && flag == "-o" => {
            match crate::prototype::compile_file(Path::new(input), Path::new(output)) {
                Ok(()) => {
                    let _ = writeln!(out, "built {} (Amiga GFA BASIC 3.5 FACT/PRINT prototype)", output);
                    0
                }
                Err(message) => { let _ = writeln!(err, "error: {message}"); 1 }
            }
        }
        [command, ..] if command == "check" => {
            let _ = writeln!(err, "error: 'check' is not implemented; the general compiler pipeline is unavailable");
            2
        }
        [command, ..] if command == "build" => {
            let _ = writeln!(err, "error: expected 'gfabasic build <source.bas> -o <output>'");
            2
        }
        _ => {
            let _ = writeln!(err, "error: invalid command or arguments");
            let _ = err.write_all(b"Run 'gfabasic --help' for usage.\n"); 2
        }
    }
}

fn run_fmt<W: Write, E: Write>(args: &[String], out: &mut W, err: &mut E) -> u8 {
    let mut source_path: Option<PathBuf> = None;
    let mut output_path: Option<PathBuf> = None;
    let mut check = false;
    let mut in_place = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--check" => check = true,
            "--in-place" => in_place = true,
            "-o" | "--output" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    let _ = writeln!(err, "error: -o requires an output path"); return 2;
                };
                if output_path.replace(PathBuf::from(path)).is_some() {
                    let _ = writeln!(err, "error: output path specified more than once"); return 2;
                }
            }
            option if option.starts_with('-') => {
                let _ = writeln!(err, "error: unknown fmt option '{option}'"); return 2;
            }
            path => {
                if source_path.replace(PathBuf::from(path)).is_some() {
                    let _ = writeln!(err, "error: fmt accepts exactly one source path"); return 2;
                }
            }
        }
        index += 1;
    }
    let Some(source_path) = source_path else {
        let _ = writeln!(err, "error: expected 'gfabasic fmt <source.bas> [--check] [-o <path> | --in-place]'");
        return 2;
    };
    if check && (in_place || output_path.is_some()) {
        let _ = writeln!(err, "error: --check cannot be combined with -o or --in-place"); return 2;
    }
    if in_place && output_path.is_some() {
        let _ = writeln!(err, "error: --in-place cannot be combined with -o"); return 2;
    }
    let source = match fs::read_to_string(&source_path) {
        Ok(source) => source,
        Err(error) => {
            let _ = writeln!(err, "error: could not read {}: {error}", source_path.display()); return 1;
        }
    };
    let mut sources = crate::source::SourceMap::new();
    let source_id = sources.add_source(source_path.to_string_lossy().into_owned(), source.as_str());
    let formatted = match crate::formatter::format_source(source_id, &source) {
        Ok(formatted) => formatted,
        Err(diagnostics) => {
            for diagnostic in diagnostics { let _ = writeln!(err, "{}", diagnostic.render(&sources)); }
            return 1;
        }
    };
    if check {
        if formatted == source { return 0; }
        let _ = writeln!(err, "{}: would reformat", source_path.display()); return 1;
    }
    let destination = if in_place { Some(source_path.as_path()) } else { output_path.as_deref() };
    if let Some(destination) = destination {
        match atomic_write(destination, formatted.as_bytes()) {
            Ok(()) => 0,
            Err(error) => {
                let _ = writeln!(err, "error: could not write {} atomically: {error}", destination.display()); 1
            }
        }
    } else if out.write_all(formatted.as_bytes()).is_ok() { 0 } else {
        let _ = writeln!(err, "error: could not write formatted source to stdout"); 1
    }
}

fn atomic_write(path: &Path, contents: &[u8]) -> io::Result<()> {
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    let file_name = path.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "destination has no file name"))?;
    let mut temporary_path = None;
    let mut temporary_file = None;
    for _ in 0..100 {
        let sequence = NEXT_TEMP_FILE.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(".{}.gfabasic-{}-{sequence}.tmp", file_name.to_string_lossy(), std::process::id()));
        match OpenOptions::new().write(true).create_new(true).open(&candidate) {
            Ok(file) => { temporary_path = Some(candidate); temporary_file = Some(file); break; }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    let temporary_path = temporary_path.ok_or_else(|| io::Error::new(io::ErrorKind::AlreadyExists, "could not allocate a unique temporary file"))?;
    let result = (|| {
        let mut file = temporary_file.expect("temporary path and file are allocated together");
        file.write_all(contents)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temporary_path, path)
    })();
    if result.is_err() { let _ = fs::remove_file(&temporary_path); }
    result
}

#[cfg(test)]
mod tests {
    use super::{atomic_write, run_with_io};
    use std::fs;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT_TEST: AtomicUsize = AtomicUsize::new(0);

    fn invoke(args: &[&str]) -> (u8, String, String) {
        let mut out = Vec::new(); let mut err = Vec::new();
        let code = run_with_io(args.iter().map(|arg| (*arg).to_owned()), &mut out, &mut err);
        (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
    }
    fn test_directory() -> std::path::PathBuf {
        let sequence = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!("gfabasic-cli-test-{}-{sequence}", std::process::id()));
        fs::create_dir_all(&directory).unwrap(); directory
    }

    #[test] fn no_arguments_show_help() {
        let (code, out, err) = invoke(&[]);
        assert_eq!(code, 0); assert!(out.contains("Usage:"));
        assert!(out.contains("combinatorics/PRINT prototype")); assert!(err.is_empty());
    }
    #[test] fn help_flag_shows_usage() {
        let (code, out, err) = invoke(&["--help"]);
        assert_eq!(code, 0); assert!(out.starts_with("gfabasic - GFA BASIC compiler"));
        assert!(out.contains("fmt <source.bas>")); assert!(err.is_empty());
    }
    #[test] fn version_prints_package_version() {
        let (code, out, err) = invoke(&["version"]);
        assert_eq!(code, 0); assert_eq!(out, format!("gfabasic {}\n", env!("CARGO_PKG_VERSION"))); assert!(err.is_empty());
    }
    #[test] fn verbose_version_includes_build_identity() {
        let (code, out, err) = invoke(&["--version", "--verbose"]);
        assert_eq!(code, 0); assert!(out.contains(&format!("gfabasic {}\n", env!("CARGO_PKG_VERSION"))));
        assert!(out.contains("build-id: ")); assert!(err.is_empty());
    }
    #[test] fn fmt_writes_canonical_text_to_stdout() {
        let dir = test_directory(); let input = dir.join("source.bas");
        fs::write(&input, "x=FACT(1)\r\nPRINT   x").unwrap();
        let (code, out, err) = invoke(&["fmt", input.to_str().unwrap()]);
        assert_eq!(code, 0, "{err}"); assert_eq!(out, "x = FACT(1)\nPRINT x\n"); assert!(err.is_empty());
        fs::remove_dir_all(dir).unwrap();
    }
    #[test] fn fmt_check_reports_difference_and_does_not_write() {
        let dir = test_directory(); let input = dir.join("source.bas"); let original = "x=FACT(1)\n";
        fs::write(&input, original).unwrap();
        let (code, out, err) = invoke(&["fmt", "--check", input.to_str().unwrap()]);
        assert_eq!(code, 1); assert!(out.is_empty()); assert!(err.contains("would reformat"));
        assert_eq!(fs::read_to_string(&input).unwrap(), original); fs::remove_dir_all(dir).unwrap();
    }
    #[test] fn fmt_check_succeeds_for_canonical_source() {
        let dir = test_directory(); let input = dir.join("source.bas");
        fs::write(&input, "x = FACT(1)\nPRINT x\n").unwrap();
        let (code, out, err) = invoke(&["fmt", input.to_str().unwrap(), "--check"]);
        assert_eq!(code, 0); assert!(out.is_empty()); assert!(err.is_empty()); fs::remove_dir_all(dir).unwrap();
    }
    #[test] fn fmt_in_place_replaces_source_atomically() {
        let dir = test_directory(); let input = dir.join("source.bas");
        fs::write(&input, "x=FACT(1)\n").unwrap();
        let (code, out, err) = invoke(&["fmt", "--in-place", input.to_str().unwrap()]);
        assert_eq!(code, 0, "{err}"); assert!(out.is_empty()); assert!(err.is_empty());
        assert_eq!(fs::read_to_string(&input).unwrap(), "x = FACT(1)\n"); fs::remove_dir_all(dir).unwrap();
    }
    #[test] fn fmt_output_path_is_written_atomically() {
        let dir = test_directory(); let input = dir.join("source.bas"); let output = dir.join("formatted.bas");
        fs::write(&input, "x=FACT(1)\n").unwrap();
        let (code, out, err) = invoke(&["fmt", input.to_str().unwrap(), "-o", output.to_str().unwrap()]);
        assert_eq!(code, 0, "{err}"); assert!(out.is_empty());
        assert_eq!(fs::read_to_string(&output).unwrap(), "x = FACT(1)\n");
        assert_eq!(fs::read_to_string(&input).unwrap(), "x=FACT(1)\n"); fs::remove_dir_all(dir).unwrap();
    }
    #[test] fn invalid_source_is_never_overwritten() {
        let dir = test_directory(); let input = dir.join("source.bas"); let original = "PRINT \"unsupported\"\n";
        fs::write(&input, original).unwrap();
        let (code, out, err) = invoke(&["fmt", "--in-place", input.to_str().unwrap()]);
        assert_eq!(code, 1); assert!(out.is_empty()); assert!(err.contains("error["));
        assert_eq!(fs::read_to_string(&input).unwrap(), original); fs::remove_dir_all(dir).unwrap();
    }
    #[test] fn atomic_write_replaces_existing_file() {
        let dir = test_directory(); let path = dir.join("target.bas"); fs::write(&path, "old").unwrap();
        atomic_write(&path, b"new complete contents").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"new complete contents"); fs::remove_dir_all(dir).unwrap();
    }
    #[test] fn build_requires_source_and_output_arguments() {
        let (code, out, err) = invoke(&["build"]);
        assert_eq!(code, 2); assert!(out.is_empty()); assert!(err.contains("expected 'gfabasic build"));
    }
    #[test] fn build_compiles_the_verified_fact_fixture() {
        let cc = Command::new("cc").arg("--version").output().expect("CLI native build test requires cc");
        assert!(cc.status.success(), "CLI native build test requires a working cc");
        let dir = test_directory(); let input = dir.join("fact.bas"); let output = dir.join("fact");
        fs::write(&input, "x=FACT(0)\ny=FACT(1)\nz=FACT(2)\nPRINT x,y,z\n").unwrap();
        let (code, out, err) = invoke(&["build", input.to_str().unwrap(), "-o", output.to_str().unwrap()]);
        assert_eq!(code, 0, "{err}"); assert!(out.contains("FACT/PRINT prototype"));
        let run = Command::new(&output).output().unwrap(); assert!(run.status.success());
        assert_eq!(String::from_utf8(run.stdout).unwrap().split_whitespace().collect::<Vec<_>>(), ["1", "1", "2"]);
        fs::remove_dir_all(dir).unwrap();
    }
    #[test] fn check_is_not_advertised_as_available() {
        let (code, out, err) = invoke(&["check"]);
        assert_eq!(code, 2); assert!(out.is_empty()); assert!(err.contains("general compiler pipeline is unavailable"));
    }
    #[test] fn unknown_arguments_return_usage_error() {
        let (code, out, err) = invoke(&["--unknown"]);
        assert_eq!(code, 2); assert!(out.is_empty()); assert!(err.contains("invalid command or arguments"));
        assert!(err.contains("gfabasic --help"));
    }
}
