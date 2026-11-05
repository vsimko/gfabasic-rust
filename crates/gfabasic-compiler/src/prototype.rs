//! Experimental, deliberately narrow native-code prototype.
//!
//! This module consumes the evidence-gated FACT/PRINT AST. It is a bootstrap
//! bridge for issue #113, not the production backend: it emits C and invokes
//! the host `cc` command until the project-owned typed IR and LLVM backend
//! exist. Output spacing and UTF-8 transport are prototype choices, not
//! verified historical interpreter behavior.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::diagnostics::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::frontend::{Program, Statement, parse};
use crate::source::{SourceMap, SourceSpan};

static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);

/// Compile the current evidence-backed Amiga GFA BASIC 3.5 subset to a native
/// host executable using the temporary C/`cc` bootstrap bridge.
pub fn compile_file(input: &Path, output: &Path) -> Result<(), String> {
    let mut sources = SourceMap::new();
    let source_id = sources
        .load_utf8(input)
        .map_err(|error| error.to_string())?;
    let source = sources
        .get(source_id)
        .expect("source ID was just inserted")
        .text()
        .to_owned();

    let program = parse(source_id, &source).map_err(|diagnostics| {
        diagnostics
            .iter()
            .map(|diagnostic| diagnostic.render(&sources))
            .collect::<Vec<_>>()
            .join("\n")
    })?;
    let c_source = lower_program(&program, &sources)?;

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

    let result =
        result.map_err(|error| format!("could not start bootstrap C toolchain 'cc': {error}"))?;
    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        return Err(format!(
            "bootstrap C code generation failed: {}",
            stderr.trim()
        ));
    }

    Ok(())
}

fn semantic_error(sources: &SourceMap, span: SourceSpan, message: impl Into<String>) -> String {
    Diagnostic::new(
        "GFA-SEM-0001",
        DiagnosticSeverity::Error,
        DiagnosticCategory::Unsupported,
        message,
    )
    .with_primary_span(span)
    .render(sources)
}

fn factorial_value(n: i64) -> Option<i64> {
    if !(0..=20).contains(&n) {
        return None;
    }
    (1..=n).try_fold(1_i64, |value, factor| value.checked_mul(factor))
}

fn variat_value(n: i64, k: i64) -> Option<i64> {
    if !(0..=20).contains(&n) || !(0..=n).contains(&k) {
        return None;
    }
    ((n - k + 1)..=n).try_fold(1_i64, |value, factor| value.checked_mul(factor))
}

fn combin_value(n: i64, k: i64) -> Option<i64> {
    if !(0..=20).contains(&n) || !(0..=n).contains(&k) {
        return None;
    }
    let k = k.min(n - k);
    let mut value = 1_i64;
    for index in 1..=k {
        value = value.checked_mul(n - k + index)?;
        value /= index;
    }
    Some(value)
}

fn emit_assignment(
    variables: &mut BTreeMap<String, i64>,
    statements: &mut Vec<String>,
    name: &str,
    value: i64,
    span: SourceSpan,
    sources: &SourceMap,
) -> Result<(), String> {
    if variables.contains_key(name) {
        return Err(semantic_error(
            sources,
            span,
            format!("reassignment of '{name}' is outside the current prototype subset"),
        ));
    }
    variables.insert(name.to_owned(), value);
    statements.push(format!("    long long {name} = {value};\n"));
    Ok(())
}

fn lower_program(program: &Program, sources: &SourceMap) -> Result<String, String> {
    let mut variables = BTreeMap::<String, i64>::new();
    let mut statements = Vec::new();
    let mut saw_print = false;

    for statement in &program.statements {
        match statement {
            Statement::AssignFact {
                name,
                argument,
                span,
            } => {
                let value = factorial_value(*argument).ok_or_else(|| {
                    semantic_error(
                        sources,
                        *span,
                        format!(
                            "FACT({argument}) is outside the supported integer-literal range 0..=20"
                        ),
                    )
                })?;
                emit_assignment(&mut variables, &mut statements, name, value, *span, sources)?;
            }
            Statement::AssignVariat { name, n, k, span } => {
                let value = variat_value(*n, *k).ok_or_else(|| {
                    semantic_error(
                        sources,
                        *span,
                        format!("VARIAT({n},{k}) requires 0 <= k <= n <= 20"),
                    )
                })?;
                emit_assignment(&mut variables, &mut statements, name, value, *span, sources)?;
            }
            Statement::AssignCombin { name, n, k, span } => {
                let value = combin_value(*n, *k).ok_or_else(|| {
                    semantic_error(
                        sources,
                        *span,
                        format!("COMBIN({n},{k}) requires 0 <= k <= n <= 20"),
                    )
                })?;
                emit_assignment(&mut variables, &mut statements, name, value, *span, sources)?;
            }
            Statement::PrintVariables { names, span } => {
                for name in names {
                    if !variables.contains_key(name) {
                        return Err(semantic_error(
                            sources,
                            *span,
                            format!("variable '{name}' is not assigned before PRINT"),
                        ));
                    }
                }
                let placeholders = vec!["%lld"; names.len()].join(" ");
                statements.push(format!(
                    "    (void)printf(\"{placeholders}\\n\", {});\n",
                    names.join(", ")
                ));
                saw_print = true;
            }
        }
    }

    if !saw_print {
        return Err("prototype requires at least one PRINT statement".to_owned());
    }

    let mut c_source = String::from("#include <stdio.h>\nint main(void) {\n");
    for statement in statements {
        c_source.push_str(&statement);
    }
    c_source.push_str("    return 0;\n}\n");
    Ok(c_source)
}

#[cfg(test)]
mod tests {
    use super::{compile_file, lower_program};
    use crate::frontend::parse;
    use crate::source::SourceMap;
    use std::fs;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEST: AtomicUsize = AtomicUsize::new(0);
    const FACT_FIXTURE: &str =
        include_str!("../../../tests/corpus/amiga/3.5/combinatorics/fact-zero.bas");

    #[test]
    fn lowers_the_frontend_ast_for_the_verified_fact_fixture() {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("fact-zero.bas", FACT_FIXTURE);
        let program = parse(source_id, FACT_FIXTURE).unwrap();
        let generated = lower_program(&program, &sources).unwrap();

        assert!(generated.contains("long long x = 1;"));
        assert!(generated.contains("long long y = 1;"));
        assert!(generated.contains("long long z = 2;"));
        assert!(generated.contains("printf(\"%lld %lld %lld\\n\", x, y, z)"));
    }

    #[test]
    fn rejects_semantics_outside_the_verified_subset() {
        for source in [
            "x=FACT(21)\nPRINT x\n",
            "x=VARIAT(3,4)\nPRINT x\n",
            "x=COMBIN(3,4)\nPRINT x\n",
            "PRINT x\n",
            "x=FACT(1)\nx=FACT(2)\nPRINT x\n",
        ] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("unsupported.bas", source);
            let program = parse(source_id, source).unwrap();
            assert!(
                lower_program(&program, &sources).is_err(),
                "accepted: {source}"
            );
        }
    }

    #[test]
    fn evaluates_documented_amiga_35_combinatorics_intrinsics() {
        let source_text = "x=FACT(5)\ny=VARIAT(5,2)\nz=COMBIN(5,2)\nPRINT x,y,z\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("combinatorics.bas", source_text);
        let program = parse(source_id, source_text).unwrap();
        let generated = lower_program(&program, &sources).unwrap();

        assert!(generated.contains("long long x = 120;"));
        assert!(generated.contains("long long y = 20;"));
        assert!(generated.contains("long long z = 10;"));
        assert!(generated.contains("printf(\"%lld %lld %lld\\n\", x, y, z)"));
    }

    #[test]
    fn compiles_and_runs_the_amiga_35_combinatorics_fixture() {
        let cc_version = Command::new("cc")
            .arg("--version")
            .output()
            .expect("native bootstrap end-to-end test requires a host C compiler (`cc`)");
        assert!(cc_version.status.success());

        let sequence = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "gfabasic-combinatorics-test-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).unwrap();
        let input = directory.join("combinatorics.bas");
        let output = directory.join("combinatorics");
        fs::write(
            &input,
            include_str!("../../../tests/corpus/amiga/3.5/combinatorics/three-intrinsics.bas"),
        )
        .unwrap();

        compile_file(&input, &output).unwrap();
        let run = Command::new(&output).output().unwrap();
        assert!(run.status.success());
        assert_eq!(
            String::from_utf8(run.stdout)
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>(),
            ["120", "20", "10"]
        );

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn compiles_and_runs_a_native_executable() {
        let cc_version = Command::new("cc")
            .arg("--version")
            .output()
            .expect("native bootstrap end-to-end test requires a host C compiler (`cc`)");
        assert!(
            cc_version.status.success(),
            "native bootstrap end-to-end test requires a working host C compiler (`cc`)"
        );

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
        assert_eq!(
            String::from_utf8(run.stdout)
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>(),
            ["1", "1", "2"]
        );

        fs::remove_dir_all(directory).unwrap();
    }
}
