//! Experimental, deliberately narrow native-code prototype.
//!
//! This module consumes verified project IR and emits temporary C for the
//! host `cc` bootstrap. It is not the production LLVM backend. Output spacing
//! and UTF-8 transport are prototype choices, not verified historical
//! interpreter behavior.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::diagnostics::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::frontend::parse;
use crate::ir::{Instruction, Module, ValueId};
use crate::source::{SourceMap, SourceSpan};

static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);

/// Compile the current evidence-backed Amiga GFA BASIC 3.5 subset to a native
/// host executable using the temporary C/`cc` bootstrap bridge.
pub fn compile_file(input: &Path, output: &Path) -> Result<(), String> {
    if !crate::source::is_text_listing_path(input) {
        return Err(format!(
            "compiler accepts textual .LST source only: {}",
            input.display()
        ));
    }

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

fn lower_program(
    program: &crate::frontend::Program,
    sources: &SourceMap,
) -> Result<String, String> {
    let module = crate::ir::lower(program).map_err(|errors| {
        errors
            .iter()
            .map(|error| match error.span {
                Some(span) => semantic_error(sources, span, error.message.clone()),
                None => error.message.clone(),
            })
            .collect::<Vec<_>>()
            .join("\n")
    })?;
    emit_c(&module)
}

fn emit_c(module: &Module) -> Result<String, String> {
    let mut names = BTreeMap::<ValueId, String>::new();
    let mut statements = Vec::new();

    for instruction in &module.instructions {
        match instruction {
            Instruction::ConstI64 { result, value, .. } => {
                let name = format!("v{}", result.0);
                names.insert(*result, name.clone());
                statements.push(format!("    long long {name} = {value};\n"));
            }
            Instruction::Print { values, .. } => {
                let arguments = values
                    .iter()
                    .map(|value| {
                        names.get(value).cloned().ok_or_else(|| {
                            format!("verified IR references missing value %{}", value.0)
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let placeholders = vec!["%lld"; arguments.len()].join(" ");
                statements.push(format!(
                    "    (void)printf(\"{placeholders}\\n\", {});\n",
                    arguments.join(", ")
                ));
            }
        }
    }

    if !statements
        .iter()
        .any(|statement| statement.contains("printf("))
    {
        return Err("verified IR contains no PRINT instruction".to_owned());
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
    use std::path::Path;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEST: AtomicUsize = AtomicUsize::new(0);
    const FACT_FIXTURE: &str =
        include_str!("../../../tests/corpus/amiga/3.5/combinatorics/fact-zero.lst");

    #[test]
    fn lowers_the_frontend_ast_for_the_verified_fact_fixture() {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("fact-zero.bas", FACT_FIXTURE);
        let program = parse(source_id, FACT_FIXTURE).unwrap();
        let generated = lower_program(&program, &sources).unwrap();

        assert!(generated.contains("long long v0 = 1;"));
        assert!(generated.contains("long long v1 = 1;"));
        assert!(generated.contains("long long v2 = 2;"));
        assert!(generated.contains("printf(\"%lld %lld %lld\\n\", v0, v1, v2)"));
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

        assert!(generated.contains("long long v0 = 120;"));
        assert!(generated.contains("long long v1 = 20;"));
        assert!(generated.contains("long long v2 = 10;"));
        assert!(generated.contains("printf(\"%lld %lld %lld\\n\", v0, v1, v2)"));
    }

    #[test]
    fn rejects_non_listing_inputs_before_attempting_to_read_them() {
        for input in ["missing.GFA", "missing.BAS", "missing.txt", "missing"] {
            let error = compile_file(Path::new(input), Path::new("unused-output"))
                .expect_err("only textual .LST input is supported");
            assert!(
                error.contains("compiler accepts textual .LST source only"),
                "unexpected error for {input}: {error}"
            );
        }
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
        let input = directory.join("combinatorics.LST");
        let output = directory.join("combinatorics");
        fs::write(
            &input,
            include_str!("../../../tests/corpus/amiga/3.5/combinatorics/three-intrinsics.lst"),
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
        let input = directory.join("fact.LST");
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
