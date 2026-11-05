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

fn lower_program(program: &Program, sources: &SourceMap) -> Result<String, String> {
    let mut variables = BTreeMap::<String, u32>::new();
    let mut statements = Vec::new();
    let mut saw_print = false;

    for statement in &program.statements {
        match statement {
            Statement::AssignFact {
                name,
                argument,
                span,
            } => {
                if variables.contains_key(name) {
                    return Err(semantic_error(
                        sources,
                        *span,
                        format!("reassignment of '{name}' is outside the current prototype subset"),
                    ));
                }
                let value = match argument {
                    0 => 1,
                    1 => 1,
                    2 => 2,
                    _ => {
                        return Err(semantic_error(
                            sources,
                            *span,
                            format!(
                                "FACT({argument}) is outside the current verified prototype subset"
                            ),
                        ));
                    }
                };
                variables.insert(name.clone(), value);
                statements.push(format!("    long long {name} = {value};\n"));
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
            "x=FACT(3)\nPRINT x\n",
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
