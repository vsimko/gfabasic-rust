//! Experimental, deliberately narrow compiler prototype.
//!
//! This bootstrap slice accepts only the independently authored Amiga GFA
//! BASIC 3.5 FACT/PRINT fixture shape documented by the 3.5 addendum. It is
//! not a general lexer/parser, does not claim interpreter conformance, and
//! does not replace the planned LLVM backend. C is used only as a temporary
//! host toolchain bridge so the end-to-end path can be exercised early.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);

/// Compile the current verified-fixture subset to a native host executable.
///
/// The selected source profile is fixed to Amiga GFA BASIC interpreter 3.5.
/// The current prototype supports only literal FACT(0), FACT(1), FACT(2)
/// assignments and PRINT lists of previously assigned one-letter variables.
pub fn compile_file(input: &Path, output: &Path) -> Result<(), String> {
    let source = fs::read(input)
        .map_err(|error| format!("could not read {}: {error}", input.display()))?;
    let source = String::from_utf8(source)
        .map_err(|error| format!("{} is not valid UTF-8: {error}", input.display()))?;
    let c_source = compile_source(&source)?;

    let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    let temp_path = std::env::temp_dir().join(format!(
        "gfabasic-bootstrap-{}-{sequence}.c",
        std::process::id()
    ));
    fs::write(&temp_path, c_source)
        .map_err(|error| format!("could not write temporary C source: {error}"))?;

    let result = Command::new("cc")
        .arg("-std=c11")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Werror")
        .arg(&temp_path)
        .arg("-o")
        .arg(output)
        .output();

    let _ = fs::remove_file(&temp_path);

    let result = result.map_err(|error| {
        format!("could not start the bootstrap C toolchain 'cc': {error}")
    })?;
    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        return Err(format!("bootstrap C code generation failed: {}", stderr.trim()));
    }

    Ok(())
}

/// Parse and lower the exact source subset used by the 3.5 FACT fixture.
///
/// This intentionally rejects all constructs outside the subset. In
/// particular, it does not infer the semantics of arbitrary expressions,
/// variable names, comments, or PRINT formatting.
fn compile_source(source: &str) -> Result<String, String> {
    let mut variables = BTreeMap::<char, u32>::new();
    let mut print_lines = Vec::<Vec<char>>::new();
    let mut saw_statement = false;

    for (line_index, raw_line) in source.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        saw_statement = true;

        if let Some(rest) = line.strip_prefix("PRINT ") {
            let mut names = Vec::new();
            for item in rest.split(',') {
                let item = item.trim();
                let mut chars = item.chars();
                let Some(name) = chars.next() else {
                    return Err(format!("line {line_number}: empty PRINT item"));
                };
                if !name.is_ascii_lowercase() || chars.next().is_some() {
                    return Err(format!(
                        "line {line_number}: prototype supports one-letter lowercase variables only"
                    ));
                }
                if !variables.contains_key(&name) {
                    return Err(format!(
                        "line {line_number}: variable '{name}' is not assigned before PRINT"
                    ));
                }
                names.push(name);
            }
            if names.is_empty() {
                return Err(format!("line {line_number}: PRINT requires at least one variable"));
            }
            print_lines.push(names);
            continue;
        }

        let Some((lhs, rhs)) = line.split_once('=') else {
            return Err(format!(
                "line {line_number}: unsupported statement; expected a FACT assignment or PRINT"
            ));
        };
        let mut lhs_chars = lhs.trim().chars();
        let Some(name) = lhs_chars.next() else {
            return Err(format!("line {line_number}: missing assignment variable"));
        };
        if !name.is_ascii_lowercase() || lhs_chars.next().is_some() {
            return Err(format!(
                "line {line_number}: prototype supports one-letter lowercase variables only"
            ));
        }
        let rhs = rhs.trim();
        let Some(argument) = rhs.strip_prefix("FACT(").and_then(|value| value.strip_suffix(')')) else {
            return Err(format!(
                "line {line_number}: prototype supports only literal FACT(0), FACT(1), and FACT(2) assignments"
            ));
        };
        let argument = argument.trim().parse::<u32>().map_err(|_| {
            format!("line {line_number}: FACT argument must be a supported integer literal")
        })?;
        let value = match argument {
            0 => 1,
            1 => 1,
            2 => 2,
            _ => {
                return Err(format!(
                    "line {line_number}: FACT({argument}) is outside the current verified prototype subset"
                ));
            }
        };
        variables.insert(name, value);
    }

    if !saw_statement {
        return Err("source is empty".to_owned());
    }
    if print_lines.is_empty() {
        return Err("prototype requires at least one PRINT statement".to_owned());
    }

    let mut c_source = String::from("#include <stdio.h>\nint main(void) {\n");
    for names in print_lines {
        let placeholders = vec!["%lld"; names.len()].join(" ");
        let values = names
            .iter()
            .map(|name| format!("(long long){}", variables[name]))
            .collect::<Vec<_>>()
            .join(", ");
        c_source.push_str(&format!(
            "    (void)printf(\"{placeholders}\\n\", {values});\n"
        ));
    }
    c_source.push_str("    return 0;\n}\n");
    Ok(c_source)
}

#[cfg(test)]
mod tests {
    use super::{compile_file, compile_source};
    use std::fs;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEST: AtomicUsize = AtomicUsize::new(0);

    const FACT_FIXTURE: &str = "x=FACT(0)\ny=FACT(1)\nz=FACT(2)\nPRINT x,y,z\n";

    #[test]
    fn lowers_the_verified_fact_fixture_to_c() {
        let generated = compile_source(FACT_FIXTURE).unwrap();
        assert!(generated.contains("printf(\"%lld %lld %lld\\n\""));
        assert!(generated.contains("(long long)1, (long long)1, (long long)2"));
    }

    #[test]
    fn rejects_constructs_outside_the_explicit_subset() {
        for source in [
            "x=FACT(3)\nPRINT x\n",
            "x=FACT(1+1)\nPRINT x\n",
            "PRINT x\n",
            "x=FACT(1)\n",
            "x=FACT(1)\nPRINT x\nY=FACT(1)\n",
        ] {
            assert!(compile_source(source).is_err(), "accepted: {source}");
        }
    }

    #[test]
    fn compiles_and_runs_a_native_executable_when_cc_is_available() {
        if Command::new("cc").arg("--version").output().is_err() {
            return;
        }

        let sequence = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "gfabasic-prototype-test-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).unwrap();
        let input = directory.join("fact.bas");
        let output = directory.join("fact");
        fs::write(&input, FACT_FIXTURE).unwrap();

        compile_file(&input, &output).unwrap();
        let run = Command::new(&output).output().unwrap();
        assert!(run.status.success());
        assert_eq!(String::from_utf8(run.stdout).unwrap(), "1 1 2\n");

        fs::remove_dir_all(directory).unwrap();
    }
}
