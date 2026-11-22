//! Experimental, deliberately narrow native-code prototype.
//!
//! This module consumes verified project IR and emits temporary C for the
//! host `cc` bootstrap. It is not the production LLVM backend. Output spacing
//! and UTF-8 transport are prototype choices, not verified historical
//! interpreter behavior.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::diagnostics::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::frontend::{Expression, ForDirection, Statement, parse_document};
use crate::ir::{Instruction, Module, ValueId};
use crate::semantic::analyze_prototype;
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

    let document = parse_document(source_id, &source).map_err(|diagnostics| {
        diagnostics
            .iter()
            .map(|diagnostic| diagnostic.render(&sources))
            .collect::<Vec<_>>()
            .join("\n")
    })?;
    let c_source = lower_program(document, &sources)?;

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
    document: crate::frontend::ParsedDocument,
    sources: &SourceMap,
) -> Result<String, String> {
    if contains_structured(&document.program.statements) {
        let module = crate::ir::lower_structured(&document.program).map_err(|errors| {
            errors.iter().map(|error| match error.span {
                Some(span) => semantic_error(sources, span, error.message.clone()),
                None => error.message.clone(),
            }).collect::<Vec<_>>().join("\\n")
        })?;
        return emit_c(&module);
    }
    let model = analyze_prototype(document).map_err(|errors| {
        errors
            .iter()
            .map(|error| {
                Diagnostic::new(
                    &error.code,
                    DiagnosticSeverity::Error,
                    DiagnosticCategory::Unsupported,
                    error.message.clone(),
                )
                .with_primary_span(error.span)
                .render(sources)
            })
            .collect::<Vec<_>>()
            .join("\n")
    })?;
    let module = crate::ir::lower(&model).map_err(|errors| {
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

fn contains_structured(statements: &[Statement]) -> bool {
    statements.iter().any(|statement| match statement {
        Statement::AssignExpression { .. } | Statement::PrintExpressions { .. }
        | Statement::For { .. } | Statement::Procedure { .. } | Statement::Function { .. }
        | Statement::Gosub { .. } => true,
        _ => false,
    })
}

fn canonical(name: &str) -> String { name.to_ascii_uppercase() }

fn collect_expression_variables(expression: &Expression, variables: &mut BTreeSet<String>) {
    match expression {
        Expression::Integer(_, _) => {}
        Expression::Variable(name, _) => { variables.insert(canonical(name)); }
        Expression::Negate(value, _) => collect_expression_variables(value, variables),
        Expression::Add(left, right, _) | Expression::Subtract(left, right, _)
        | Expression::Multiply(left, right, _) | Expression::Divide(left, right, _) => {
            collect_expression_variables(left, variables);
            collect_expression_variables(right, variables);
        }
        Expression::Call { arguments, .. } => for argument in arguments { collect_expression_variables(argument, variables); },
    }
}

fn collect_statement_variables(statements: &[Statement], variables: &mut BTreeSet<String>) {
    for statement in statements {
        match statement {
            Statement::AssignInteger { name, .. } | Statement::AssignFact { name, .. }
            | Statement::AssignVariat { name, .. } | Statement::AssignCombin { name, .. } => { variables.insert(canonical(name)); }
            Statement::AssignExpression { name, value, .. } => {
                variables.insert(canonical(name));
                collect_expression_variables(value, variables);
            }
            Statement::PrintVariables { names, .. } => for name in names { variables.insert(canonical(name)); },
            Statement::PrintExpressions { values, .. } => for value in values { collect_expression_variables(value, variables); },
            Statement::For { variable, start, end, step, body, .. } => {
                variables.insert(canonical(variable));
                collect_expression_variables(start, variables);
                collect_expression_variables(end, variables);
                if let Some(step) = step { collect_expression_variables(step, variables); }
                collect_statement_variables(body, variables);
            }
            Statement::Procedure { parameters, body, .. } | Statement::Function { parameters, body, .. } => {
                for parameter in parameters { variables.insert(canonical(parameter)); }
                collect_statement_variables(body, variables);
                if let Statement::Function { result, .. } = statement { collect_expression_variables(result, variables); }
            }
            Statement::Gosub { arguments, .. } => for argument in arguments { collect_expression_variables(argument, variables); },
            _ => {}
        }
    }
}

fn variable_reference(name: &str, parameters: &BTreeMap<String, String>, globals: &BTreeMap<String, String>) -> Result<String, String> {
    let key = canonical(name);
    parameters.get(&key).or_else(|| globals.get(&key)).cloned()
        .ok_or_else(|| format!("variable '{name}' has no allocated storage"))
}

fn emit_expression(
    expression: &Expression,
    parameters: &BTreeMap<String, String>,
    globals: &BTreeMap<String, String>,
    functions: &BTreeMap<String, String>,
) -> Result<String, String> {
    Ok(match expression {
        Expression::Integer(value, _) => value.to_string(),
        Expression::Variable(name, _) => variable_reference(name, parameters, globals)?,
        Expression::Negate(value, _) => format!("(-{})", emit_expression(value, parameters, globals, functions)?),
        Expression::Add(left, right, _) => format!("({} + {})", emit_expression(left, parameters, globals, functions)?, emit_expression(right, parameters, globals, functions)?),
        Expression::Subtract(left, right, _) => format!("({} - {})", emit_expression(left, parameters, globals, functions)?, emit_expression(right, parameters, globals, functions)?),
        Expression::Multiply(left, right, _) => format!("({} * {})", emit_expression(left, parameters, globals, functions)?, emit_expression(right, parameters, globals, functions)?),
        Expression::Divide(left, right, _) => format!("({} / {})", emit_expression(left, parameters, globals, functions)?, emit_expression(right, parameters, globals, functions)?),
        Expression::Call { name, arguments, .. } => {
            let key = canonical(name);
            let target = match key.as_str() {
                "FACT" => "gfa_builtin_fact".to_owned(),
                "VARIAT" => "gfa_builtin_variat".to_owned(),
                "COMBIN" => "gfa_builtin_combin".to_owned(),
                _ => functions.get(&key).cloned().ok_or_else(|| format!("function '{name}' has no generated symbol"))?,
            };
            let args = arguments.iter().map(|arg| emit_expression(arg, parameters, globals, functions)).collect::<Result<Vec<_>,_>>()?;
            format!("{target}({})", args.join(", "))
        }
    })
}

fn emit_statement(
    statement: &Statement,
    indent: usize,
    parameters: &BTreeMap<String, String>,
    globals: &BTreeMap<String, String>,
    functions: &BTreeMap<String, String>,
    procedures: &BTreeMap<String, String>,
    loop_id: &mut usize,
) -> Result<String, String> {
    let pad = "    ".repeat(indent);
    let mut output = String::new();
    match statement {
        Statement::AssignInteger { name, value, .. } => {
            output.push_str(&format!("{pad}{} = {value};\n", variable_reference(name, parameters, globals)?));
        }
        Statement::AssignFact { name, argument, .. } => {
            output.push_str(&format!("{pad}{} = gfa_builtin_fact({argument});\n", variable_reference(name, parameters, globals)?));
        }
        Statement::AssignVariat { name, n, k, .. } => {
            output.push_str(&format!("{pad}{} = gfa_builtin_variat({n}, {k});\n", variable_reference(name, parameters, globals)?));
        }
        Statement::AssignCombin { name, n, k, .. } => {
            output.push_str(&format!("{pad}{} = gfa_builtin_combin({n}, {k});\n", variable_reference(name, parameters, globals)?));
        }
        Statement::AssignExpression { name, value, .. } => {
            let value = emit_expression(value, parameters, globals, functions)?;
            output.push_str(&format!("{pad}{} = {value};\n", variable_reference(name, parameters, globals)?));
        }
        Statement::PrintVariables { names, .. } => {
            let args = names.iter().map(|name| variable_reference(name, parameters, globals)).collect::<Result<Vec<_>,_>>()?;
            output.push_str(&format!("{pad}(void)printf(\"{}\\n\", {});\n", vec!["%lld"; args.len()].join(" "), args.join(", ")));
        }
        Statement::PrintExpressions { values, .. } => {
            let args = values.iter().map(|value| emit_expression(value, parameters, globals, functions)).collect::<Result<Vec<_>,_>>()?;
            output.push_str(&format!("{pad}(void)printf(\"{}\\n\", {});\n", vec!["%lld"; args.len()].join(" "), args.join(", ")));
        }
        Statement::For { variable, start, end, step, direction, body, .. } => {
            let id = *loop_id;
            *loop_id += 1;
            let start_name = format!("gfa_for_start_{id}");
            let end_name = format!("gfa_for_end_{id}");
            let step_name = format!("gfa_for_step_{id}");
            let start_expr = emit_expression(start, parameters, globals, functions)?;
            let end_expr = emit_expression(end, parameters, globals, functions)?;
            let step_expr = if let Some(step) = step {
                emit_expression(step, parameters, globals, functions)?
            } else if *direction == ForDirection::DownTo { "-1".to_owned() } else { "1".to_owned() };
            let counter = variable_reference(variable, parameters, globals)?;
            output.push_str(&format!("{pad}{{\n{pad}    long long {start_name} = {start_expr};\n{pad}    long long {end_name} = {end_expr};\n{pad}    long long {step_name} = {step_expr};\n{pad}    if ({step_name} == 0) {{ (void)fputs(\"GFA BASIC FOR STEP cannot be zero\\n\", stderr); abort(); }}\n{pad}    {counter} = {start_name};\n{pad}    do {{\n"));
            for child in body {
                output.push_str(&emit_statement(child, indent + 2, parameters, globals, functions, procedures, loop_id)?);
            }
            output.push_str(&format!("{pad}        {counter} += {step_name};\n{pad}    }} while (({step_name} > 0 && {counter} <= {end_name}) || ({step_name} < 0 && {counter} >= {end_name}));\n{pad}}}\n"));
        }
        Statement::Gosub { name, arguments, .. } => {
            let target = procedures.get(&canonical(name)).ok_or_else(|| format!("procedure '{name}' has no generated symbol"))?;
            let args = arguments.iter().map(|arg| emit_expression(arg, parameters, globals, functions)).collect::<Result<Vec<_>,_>>()?;
            output.push_str(&format!("{pad}{target}({});\n", args.join(", ")));
        }
        Statement::Procedure { .. } | Statement::Function { .. } => {}
        _ => return Err("matrix statements are not supported in structured executable code".to_owned()),
    }
    Ok(output)
}

fn emit_structured_c(program: &crate::frontend::Program) -> Result<String, String> {
    let mut variables = BTreeSet::new();
    collect_statement_variables(&program.statements, &mut variables);
    let globals: BTreeMap<String, String> = variables.iter().enumerate().map(|(i, name)| (name.clone(), format!("gfa_var_{i}"))).collect();

    let mut functions = BTreeMap::new();
    let mut procedures = BTreeMap::new();
    let mut function_declarations = Vec::new();
    let mut procedure_declarations = Vec::new();
    for statement in &program.statements {
        match statement {
            Statement::Function { name, parameters, .. } => {
                let symbol = format!("gfa_function_{}", functions.len());
                functions.insert(canonical(name), symbol.clone());
                function_declarations.push((symbol, parameters.clone()));
            }
            Statement::Procedure { name, parameters, .. } => {
                let symbol = format!("gfa_procedure_{}", procedures.len());
                procedures.insert(canonical(name), symbol.clone());
                procedure_declarations.push((symbol, parameters.clone()));
            }
            _ => {}
        }
    }

    let mut c = String::from("#include <stdio.h>\n#include <stdlib.h>\n\n");
    c.push_str("long long gfa_builtin_fact(long long n) { if (n < 0 || n > 20) abort(); long long r = 1; for (long long i = 2; i <= n; ++i) r *= i; return r; }\n");
    c.push_str("long long gfa_builtin_variat(long long n, long long k) { if (n < 0 || n > 20 || k < 0 || k > n) abort(); long long r = 1; for (long long i = 0; i < k; ++i) r *= n - i; return r; }\n");
    c.push_str("long long gfa_builtin_combin(long long n, long long k) { if (n < 0 || n > 20 || k < 0 || k > n) abort(); if (k > n-k) k = n-k; long long r = 1; for (long long i = 1; i <= k; ++i) r = r * (n-k+i) / i; return r; }\n\n");
    for name in globals.values() { c.push_str(&format!("long long {name};\n")); }
    c.push('\n');

    for (symbol, parameters) in &function_declarations {
        let args = (0..parameters.len()).map(|i| format!("long long p{i}")).collect::<Vec<_>>().join(", ");
        c.push_str(&format!("long long {symbol}({});\n", if args.is_empty() { "void" } else { &args }));
    }
    for (symbol, parameters) in &procedure_declarations {
        let args = (0..parameters.len()).map(|i| format!("long long p{i}")).collect::<Vec<_>>().join(", ");
        c.push_str(&format!("void {symbol}({});\n", if args.is_empty() { "void" } else { &args }));
    }
    c.push('\n');

    let mut loop_id = 0;
    for (index, statement) in program.statements.iter().enumerate() {
        match statement {
            Statement::Function { name, parameters, body, result, .. } => {
                let symbol = functions.get(&canonical(name)).unwrap();
                let params: BTreeMap<String, String> = parameters.iter().enumerate().map(|(i,p)| (canonical(p), format!("p{i}"))).collect();
                let args = (0..parameters.len()).map(|i| format!("long long p{i}")).collect::<Vec<_>>().join(", ");
                c.push_str(&format!("long long {symbol}({}) {{\n", if args.is_empty() { "void" } else { &args }));
                for i in 0..parameters.len() { c.push_str(&format!("    (void)p{i};\n")); }
                for child in body { c.push_str(&emit_statement(child, 1, &params, &globals, &functions, &procedures, &mut loop_id)?); }
                c.push_str(&format!("    return {};\n}}\n\n", emit_expression(result, &params, &globals, &functions)?));
            }
            Statement::Procedure { name, parameters, body, .. } => {
                let symbol = procedures.get(&canonical(name)).unwrap();
                let params: BTreeMap<String, String> = parameters.iter().enumerate().map(|(i,p)| (canonical(p), format!("p{i}"))).collect();
                let args = (0..parameters.len()).map(|i| format!("long long p{i}")).collect::<Vec<_>>().join(", ");
                c.push_str(&format!("void {symbol}({}) {{\n", if args.is_empty() { "void" } else { &args }));
                for i in 0..parameters.len() { c.push_str(&format!("    (void)p{i};\n")); }
                for child in body { c.push_str(&emit_statement(child, 1, &params, &globals, &functions, &procedures, &mut loop_id)?); }
                c.push_str("}\n\n");
            }
            _ => { let _ = index; }
        }
    }

    c.push_str("int main(void) {\n");
    for statement in &program.statements {
        if !matches!(statement, Statement::Procedure { .. } | Statement::Function { .. }) {
            c.push_str(&emit_statement(statement, 1, &BTreeMap::new(), &globals, &functions, &procedures, &mut loop_id)?);
        }
    }
    c.push_str("    return 0;\n}\n");
    Ok(c)
}

fn emit_c(module: &Module) -> Result<String, String> {
    if let Some(program) = &module.structured_program {
        return emit_structured_c(program);
    }
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
    use crate::frontend::parse_document;
    use crate::source::SourceMap;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEST: AtomicUsize = AtomicUsize::new(0);
    const FACT_FIXTURE: &str =
        include_str!("../../../tests/corpus/amiga/3.5/combinatorics/fact-zero.lst");

    #[test]
    fn compiles_and_runs_for_next_procedures_and_functions() {
        let cc_version = Command::new("cc").arg("--version").output().unwrap();
        assert!(cc_version.status.success());
        let source = "x=0\nFOR i=1 TO 5\nx=x+i\nNEXT i\nPROCEDURE Add(a,b)\nx=x+a+b\nRETURN\nFUNCTION Square(n)\nRETURN n*n\nENDFUNC\nGOSUB Add(2,3)\nx=x+@Square(4)\nPRINT x\n";
        let sequence = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!("gfabasic-structured-test-{}-{sequence}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let input = directory.join("structured.LST");
        let output = directory.join("structured");
        fs::write(&input, source).unwrap();
        compile_file(&input, &output).unwrap();
        let run = Command::new(&output).output().unwrap();
        assert!(run.status.success(), "{}", String::from_utf8_lossy(&run.stderr));
        assert_eq!(String::from_utf8(run.stdout).unwrap().trim(), "36");
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn compiles_nested_for_next_and_downto_loops() {
        let source = "x=0\nFOR i=1 TO 3\nFOR j=2 DOWNTO 1\nx=x+1\nNEXT j\nNEXT i\nPRINT x\n";
        let sequence = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!("gfabasic-nested-for-test-{}-{sequence}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let input = directory.join("nested.LST");
        let output = directory.join("nested");
        fs::write(&input, source).unwrap();
        compile_file(&input, &output).unwrap();
        let run = Command::new(&output).output().unwrap();
        assert!(run.status.success(), "{}", String::from_utf8_lossy(&run.stderr));
        assert_eq!(String::from_utf8(run.stdout).unwrap().trim(), "6");
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn lowers_the_resolved_semantic_model_for_the_verified_fact_fixture() {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("fact-zero.bas", FACT_FIXTURE);
        let document = parse_document(source_id, FACT_FIXTURE).unwrap();
        let generated = lower_program(document, &sources).unwrap();

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
            let document = parse_document(source_id, source).unwrap();
            assert!(
                lower_program(document, &sources).is_err(),
                "accepted: {source}"
            );
        }
    }

    #[test]
    fn evaluates_documented_amiga_35_combinatorics_intrinsics() {
        let source_text = "x=FACT(5)\ny=VARIAT(5,2)\nz=COMBIN(5,2)\nPRINT x,y,z\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("combinatorics.bas", source_text);
        let document = parse_document(source_id, source_text).unwrap();
        let generated = lower_program(document, &sources).unwrap();

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
