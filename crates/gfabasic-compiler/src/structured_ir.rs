//! Structured numeric IR for counted loops and user-defined subprograms.
//!
//! This remains a deliberately bounded bootstrap IR: values are signed 64-bit
//! integers, parameters are passed by value, and strings/VAR parameters are not
//! represented. The frontend retains the source AST and this module lowers its
//! executable structured subset into explicit blocks and calls.

use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::{BinaryOperator, Expression, Program, Statement};
use crate::source::SourceSpan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValueExpr {
    I64(i64),
    Variable(String),
    Negate(Box<ValueExpr>),
    Binary {
        operator: BinaryOperator,
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<ValueExpr>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Assign { variable: String, value: ValueExpr, span: SourceSpan },
    Print { variables: Vec<String>, span: SourceSpan },
    For {
        variable: String,
        start: ValueExpr,
        end: ValueExpr,
        step: ValueExpr,
        body: Vec<Instruction>,
        span: SourceSpan,
    },
    CallProcedure { name: String, arguments: Vec<ValueExpr>, span: SourceSpan },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subprogram {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Instruction>,
    pub return_value: Option<ValueExpr>,
    pub span: SourceSpan,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Module {
    pub globals: BTreeSet<String>,
    pub main: Vec<Instruction>,
    pub procedures: Vec<Subprogram>,
    pub functions: Vec<Subprogram>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweringError {
    pub message: String,
    pub span: SourceSpan,
}

fn convert_expression(expression: &Expression) -> ValueExpr {
    match expression {
        Expression::Integer(value) => ValueExpr::I64(*value),
        Expression::Variable(name) => ValueExpr::Variable(name.clone()),
        Expression::Negate(value) => ValueExpr::Negate(Box::new(convert_expression(value))),
        Expression::Binary { operator, left, right } => ValueExpr::Binary {
            operator: *operator,
            left: Box::new(convert_expression(left)),
            right: Box::new(convert_expression(right)),
        },
        Expression::FunctionCall { name, arguments } => ValueExpr::FunctionCall {
            name: name.clone(),
            arguments: arguments.iter().map(convert_expression).collect(),
        },
    }
}

fn intrinsic_value(id: &str, args: &[i64]) -> Result<Option<i64>, String> {
    if args.iter().any(|value| !(0..=20).contains(value)) {
        return Ok(None);
    }
    let resolved = crate::semantic::resolve_api_handler(id).map_err(|error| error.to_string())?;
    match (resolved, args) {
        (gfabasic_runtime::handlers::ResolvedHandler::IntegerToUnsigned(handler), [one]) => {
            Ok(handler(u32::try_from(*one).map_err(|e| e.to_string())?)
                .and_then(|value| i64::try_from(value).ok()))
        }
        (gfabasic_runtime::handlers::ResolvedHandler::TwoIntegersToUnsigned(handler), [one, two]) => {
            Ok(handler(
                u32::try_from(*one).map_err(|e| e.to_string())?,
                u32::try_from(*two).map_err(|e| e.to_string())?,
            ).and_then(|value| i64::try_from(value).ok()))
        }
        _ => Err(format!("runtime handler signature mismatch for {id}")),
    }
}

fn lower_statements(statements: &[Statement], errors: &mut Vec<LoweringError>) -> Vec<Instruction> {
    let mut result = Vec::new();
    for statement in statements {
        match statement {
            Statement::AssignInteger { name, value, span } => result.push(Instruction::Assign {
                variable: name.clone(), value: ValueExpr::I64(*value), span: *span,
            }),
            Statement::AssignExpression { name, expression, span } => result.push(Instruction::Assign {
                variable: name.clone(), value: convert_expression(expression), span: *span,
            }),
            Statement::AssignFact { name, argument, span } => {
                match intrinsic_value("gfa.language.math.fact", &[*argument]) {
                    Ok(Some(value)) => result.push(Instruction::Assign { variable: name.clone(), value: ValueExpr::I64(value), span: *span }),
                    Ok(None) => errors.push(LoweringError { message: "FACT argument is outside the currently implemented prototype range (0..=20)".into(), span: *span }),
                    Err(message) => errors.push(LoweringError { message, span: *span }),
                }
            }
            Statement::AssignVariat { name, n, k, span } => {
                match intrinsic_value("gfa.language.math.variat", &[*n, *k]) {
                    Ok(Some(value)) => result.push(Instruction::Assign { variable: name.clone(), value: ValueExpr::I64(value), span: *span }),
                    Ok(None) => errors.push(LoweringError { message: "VARIAT arguments are outside the currently implemented prototype range (0 <= k <= n <= 20)".into(), span: *span }),
                    Err(message) => errors.push(LoweringError { message, span: *span }),
                }
            }
            Statement::AssignCombin { name, n, k, span } => {
                match intrinsic_value("gfa.language.math.combin", &[*n, *k]) {
                    Ok(Some(value)) => result.push(Instruction::Assign { variable: name.clone(), value: ValueExpr::I64(value), span: *span }),
                    Ok(None) => errors.push(LoweringError { message: "COMBIN arguments are outside the currently implemented prototype range (0 <= k <= n <= 20)".into(), span: *span }),
                    Err(message) => errors.push(LoweringError { message, span: *span }),
                }
            }
            Statement::PrintVariables { names, span } => result.push(Instruction::Print { variables: names.clone(), span: *span }),
            Statement::ForLoop { variable, start, end, step, down_to, body, span } => {
                if matches!(step, Some(Expression::Integer(0))) {
                    errors.push(LoweringError { message: "FOR STEP must not be zero".into(), span: *span });
                }
                let step = if *down_to {
                    ValueExpr::I64(-1)
                } else {
                    step.as_ref().map(convert_expression).unwrap_or(ValueExpr::I64(1))
                };
                result.push(Instruction::For {
                    variable: variable.clone(),
                    start: convert_expression(start),
                    end: convert_expression(end),
                    step,
                    body: lower_statements(body, errors),
                    span: *span,
                });
            }
            Statement::ProcedureCall { name, arguments, span } => result.push(Instruction::CallProcedure {
                name: name.clone(),
                arguments: arguments.iter().map(convert_expression).collect(),
                span: *span,
            }),
            Statement::ProcedureDefinition { .. } | Statement::FunctionDefinition { .. } | Statement::DefFnDefinition { .. } => {
                errors.push(LoweringError { message: "subprogram declarations are only valid at program scope".into(), span: statement_span(statement) });
            }
            _ => errors.push(LoweringError { message: "this parsed statement is not part of the current numeric executable subset".into(), span: statement_span(statement) }),
        }
    }
    result
}

fn statement_span(statement: &Statement) -> SourceSpan {
    match statement {
        Statement::AssignInteger { span, .. }
        | Statement::AssignFact { span, .. }
        | Statement::AssignVariat { span, .. }
        | Statement::AssignCombin { span, .. }
        | Statement::PrintVariables { span, .. }
        | Statement::AssignExpression { span, .. }
        | Statement::ForLoop { span, .. }
        | Statement::ProcedureDefinition { span, .. }
        | Statement::ProcedureCall { span, .. }
        | Statement::FunctionDefinition { span, .. }
        | Statement::DefFnDefinition { span, .. }
        | Statement::DimMatrix { span, .. }
        | Statement::MatOne { span, .. }
        | Statement::MatSet { span, .. }
        | Statement::MatClr { span, .. }
        | Statement::MatPrint { span, .. }
        | Statement::MatBase { span, .. }
        | Statement::MatTrans { span, .. }
        | Statement::MatNorm { span, .. }
        | Statement::MatArithmetic { span, .. }
        | Statement::MatMulMatrix { span, .. }
        | Statement::MatMulScalar { span, .. }
        | Statement::MatMulScale { span, .. }
        | Statement::MatAnalysis { span, .. }
        | Statement::MatInv { span, .. }
        | Statement::MatCopy { span, .. }
        | Statement::MatRead { span, .. }
        | Statement::MatInput { span, .. } => *span,
    }
}

fn insert_subprogram(
    name: &str,
    parameters: &[String],
    span: SourceSpan,
    signatures: &mut BTreeMap<String, (bool, usize)>,
    errors: &mut Vec<LoweringError>,
) {
    let mut seen = BTreeSet::new();
    for parameter in parameters {
        if !seen.insert(parameter) {
            errors.push(LoweringError { message: format!("duplicate parameter '{parameter}' in subprogram '{name}'"), span });
        }
    }
    if signatures.insert(name.to_owned(), (false, parameters.len())).is_some() {
        errors.push(LoweringError { message: format!("duplicate procedure/function name '{name}'"), span });
    }
}

pub fn lower(program: &Program) -> Result<Module, Vec<LoweringError>> {
    let mut module = Module::default();
    let mut errors = Vec::new();
    let mut signatures = BTreeMap::<String, (bool, usize)>::new();

    for statement in &program.statements {
        match statement {
            Statement::ProcedureDefinition { name, parameters, span, .. } => {
                insert_subprogram(name, parameters, *span, &mut signatures, &mut errors);
            }
            Statement::FunctionDefinition { name, parameters, span, .. }
            | Statement::DefFnDefinition { name, parameters, span, .. } => {
                let mut seen = BTreeSet::new();
                for parameter in parameters {
                    if !seen.insert(parameter) {
                        errors.push(LoweringError { message: format!("duplicate parameter '{parameter}' in function '{name}'"), span: *span });
                    }
                }
                if signatures.insert(name.clone(), (true, parameters.len())).is_some() {
                    errors.push(LoweringError { message: format!("duplicate procedure/function name '{name}'"), span: *span });
                }
            }
            _ => {}
        }
    }

    for statement in &program.statements {
        match statement {
            Statement::ProcedureDefinition { name, parameters, body, span } => {
                module.procedures.push(Subprogram {
                    name: name.clone(), parameters: parameters.clone(),
                    body: lower_statements(body, &mut errors), return_value: None, span: *span,
                });
            }
            Statement::FunctionDefinition { name, parameters, body, return_value, span } => {
                module.functions.push(Subprogram {
                    name: name.clone(), parameters: parameters.clone(),
                    body: lower_statements(body, &mut errors),
                    return_value: Some(convert_expression(return_value)), span: *span,
                });
            }
            Statement::DefFnDefinition { name, parameters, expression, span } => {
                module.functions.push(Subprogram {
                    name: name.clone(), parameters: parameters.clone(), body: Vec::new(),
                    return_value: Some(convert_expression(expression)), span: *span,
                });
            }
            _ => {}
        }
    }
    let main_statements: Vec<_> = program.statements.iter().filter(|statement| !matches!(
        statement, Statement::ProcedureDefinition { .. } | Statement::FunctionDefinition { .. } | Statement::DefFnDefinition { .. }
    )).cloned().collect();
    module.main = lower_statements(&main_statements, &mut errors);

    validate_calls(&module, &signatures, &mut errors);
    collect_module_variables(&module, &mut module.globals);

    if errors.is_empty() { Ok(module) } else { Err(errors) }
}

fn validate_expression(expression: &ValueExpr, signatures: &BTreeMap<String, (bool, usize)>, span: SourceSpan, errors: &mut Vec<LoweringError>) {
    match expression {
        ValueExpr::FunctionCall { name, arguments } => {
            match signatures.get(name) {
                Some((true, arity)) if *arity == arguments.len() => {}
                Some((true, arity)) => errors.push(LoweringError { message: format!("function '{name}' expects {arity} argument(s), got {}", arguments.len()), span }),
                Some((false, _)) => errors.push(LoweringError { message: format!("'{name}' is a procedure and cannot be used as a function"), span }),
                None => errors.push(LoweringError { message: format!("unknown user-defined function '{name}'"), span }),
            }
            for argument in arguments { validate_expression(argument, signatures, span, errors); }
        }
        ValueExpr::Negate(value) => validate_expression(value, signatures, span, errors),
        ValueExpr::Binary { left, right, .. } => {
            validate_expression(left, signatures, span, errors);
            validate_expression(right, signatures, span, errors);
        }
        ValueExpr::I64(_) | ValueExpr::Variable(_) => {}
    }
}

fn validate_instructions(instructions: &[Instruction], signatures: &BTreeMap<String, (bool, usize)>, errors: &mut Vec<LoweringError>) {
    for instruction in instructions {
        match instruction {
            Instruction::Assign { value, span, .. } => validate_expression(value, signatures, *span, errors),
            Instruction::Print { .. } => {}
            Instruction::CallProcedure { name, arguments, span } => {
                match signatures.get(name) {
                    Some((false, arity)) if *arity == arguments.len() => {}
                    Some((false, arity)) => errors.push(LoweringError { message: format!("procedure '{name}' expects {arity} argument(s), got {}", arguments.len()), span: *span }),
                    Some((true, _)) => errors.push(LoweringError { message: format!("'{name}' is a function and cannot be called with GOSUB"), span: *span }),
                    None => errors.push(LoweringError { message: format!("unknown procedure '{name}'"), span: *span }),
                }
                for argument in arguments { validate_expression(argument, signatures, *span, errors); }
            }
            Instruction::For { start, end, step, body, span, .. } => {
                validate_expression(start, signatures, *span, errors);
                validate_expression(end, signatures, *span, errors);
                validate_expression(step, signatures, *span, errors);
                validate_instructions(body, signatures, errors);
            }
        }
    }
}

fn validate_calls(module: &Module, signatures: &BTreeMap<String, (bool, usize)>, errors: &mut Vec<LoweringError>) {
    validate_instructions(&module.main, signatures, errors);
    for procedure in &module.procedures { validate_instructions(&procedure.body, signatures, errors); }
    for function in &module.functions {
        validate_instructions(&function.body, signatures, errors);
        if let Some(value) = &function.return_value { validate_expression(value, signatures, function.span, errors); }
    }
}

fn collect_expression_variables(expression: &ValueExpr, variables: &mut BTreeSet<String>) {
    match expression {
        ValueExpr::Variable(name) => { variables.insert(name.clone()); }
        ValueExpr::Negate(value) => collect_expression_variables(value, variables),
        ValueExpr::Binary { left, right, .. } => {
            collect_expression_variables(left, variables);
            collect_expression_variables(right, variables);
        }
        ValueExpr::FunctionCall { arguments, .. } => for argument in arguments { collect_expression_variables(argument, variables); },
        ValueExpr::I64(_) => {}
    }
}

fn collect_instruction_variables(instructions: &[Instruction], variables: &mut BTreeSet<String>) {
    for instruction in instructions {
        match instruction {
            Instruction::Assign { variable, value, .. } => {
                variables.insert(variable.clone());
                collect_expression_variables(value, variables);
            }
            Instruction::Print { variables: names, .. } => variables.extend(names.iter().cloned()),
            Instruction::CallProcedure { arguments, .. } => for argument in arguments { collect_expression_variables(argument, variables); },
            Instruction::For { variable, start, end, step, body, .. } => {
                variables.insert(variable.clone());
                collect_expression_variables(start, variables);
                collect_expression_variables(end, variables);
                collect_expression_variables(step, variables);
                collect_instruction_variables(body, variables);
            }
        }
    }
}

fn collect_module_variables(module: &Module, variables: &mut BTreeSet<String>) {
    collect_instruction_variables(&module.main, variables);
    for procedure in &module.procedures {
        variables.extend(procedure.parameters.iter().cloned());
        collect_instruction_variables(&procedure.body, variables);
    }
    for function in &module.functions {
        variables.extend(function.parameters.iter().cloned());
        collect_instruction_variables(&function.body, variables);
        if let Some(value) = &function.return_value { collect_expression_variables(value, variables); }
    }
}

fn c_variable(name: &str) -> String { format!("v_{name}") }
fn c_function(name: &str) -> String { format!("fn_{name}") }
fn c_procedure(name: &str) -> String { format!("proc_{name}") }

fn emit_expression(expression: &ValueExpr) -> String {
    match expression {
        ValueExpr::I64(value) => value.to_string(),
        ValueExpr::Variable(name) => c_variable(name),
        ValueExpr::Negate(value) => format!("gfa_neg({})", emit_expression(value)),
        ValueExpr::Binary { operator, left, right } => {
            let function = match operator { BinaryOperator::Add => "gfa_add", BinaryOperator::Subtract => "gfa_sub", BinaryOperator::Multiply => "gfa_mul" };
            format!("{function}({}, {})", emit_expression(left), emit_expression(right))
        }
        ValueExpr::FunctionCall { name, arguments } => {
            let args = arguments.iter().map(emit_expression).collect::<Vec<_>>().join(", ");
            format!("{}({args})", c_function(name))
        }
    }
}

fn parameters_signature(parameters: &[String]) -> String {
    if parameters.is_empty() { "void".to_owned() }
    else { parameters.iter().map(|name| format!("long long {}", c_variable(name))).collect::<Vec<_>>().join(", ") }
}

fn emit_instructions(instructions: &[Instruction], output: &mut String, loop_id: &mut usize) {
    for instruction in instructions {
        match instruction {
            Instruction::Assign { variable, value, .. } => {
                output.push_str(&format!("    {} = {};\n", c_variable(variable), emit_expression(value)));
            }
            Instruction::Print { variables, .. } => {
                let format = vec!["%lld"; variables.len()].join(" ");
                let args = variables.iter().map(|name| c_variable(name)).collect::<Vec<_>>().join(", ");
                output.push_str(&format!("    (void)printf(\"{format}\\n\", {args});\n"));
            }
            Instruction::CallProcedure { name, arguments, .. } => {
                let args = arguments.iter().map(emit_expression).collect::<Vec<_>>().join(", ");
                output.push_str(&format!("    {}({args});\n", c_procedure(name)));
            }
            Instruction::For { variable, start, end, step, body, .. } => {
                let id = *loop_id;
                *loop_id += 1;
                let end_name = format!("_gfa_for_end_{id}");
                let step_name = format!("_gfa_for_step_{id}");
                output.push_str("    {\n");
                output.push_str(&format!("        long long {end_name} = {};\n", emit_expression(end)));
                output.push_str(&format!("        long long {step_name} = {};\n", emit_expression(step)));
                output.push_str(&format!("        if ({step_name} == 0) {{ fputs(\"FOR STEP must not be zero\\n\", stderr); exit(EXIT_FAILURE); }}\n"));
                output.push_str(&format!("        {} = {};\n", c_variable(variable), emit_expression(start)));
                output.push_str("        do {\n");
                let mut nested = String::new();
                emit_instructions(body, &mut nested, loop_id);
                for line in nested.lines() { output.push_str("    "); output.push_str(line); output.push('\n'); }
                output.push_str(&format!("            if (__builtin_add_overflow({}, {step_name}, &{})) {{ fputs(\"FOR counter overflow\\n\", stderr); exit(EXIT_FAILURE); }}\n", c_variable(variable), c_variable(variable)));
                output.push_str(&format!("        }} while ({step_name} > 0 ? {} <= {end_name} : {} >= {end_name});\n", c_variable(variable), c_variable(variable)));
                output.push_str("    }\n");
            }
        }
    }
}

pub fn emit_c(module: &Module) -> String {
    let mut output = String::from(
        "#include <stdio.h>\n#include <stdlib.h>\n\n\
static long long gfa_add(long long a, long long b) { long long r; if (__builtin_add_overflow(a,b,&r)) { fputs(\"integer overflow\\n\", stderr); exit(EXIT_FAILURE); } return r; }\n\
static long long gfa_sub(long long a, long long b) { long long r; if (__builtin_sub_overflow(a,b,&r)) { fputs(\"integer overflow\\n\", stderr); exit(EXIT_FAILURE); } return r; }\n\
static long long gfa_mul(long long a, long long b) { long long r; if (__builtin_mul_overflow(a,b,&r)) { fputs(\"integer overflow\\n\", stderr); exit(EXIT_FAILURE); } return r; }\n\
static long long gfa_neg(long long a) { long long r; if (__builtin_sub_overflow(0,a,&r)) { fputs(\"integer overflow\\n\", stderr); exit(EXIT_FAILURE); } return r; }\n\n"
    );
    for name in &module.globals { output.push_str(&format!("long long {} = 0;\n", c_variable(name))); }
    output.push('\n');
    for procedure in &module.procedures {
        output.push_str(&format!("static void {}({});\n", c_procedure(&procedure.name), parameters_signature(&procedure.parameters)));
    }
    for function in &module.functions {
        output.push_str(&format!("static long long {}({});\n", c_function(&function.name), parameters_signature(&function.parameters)));
    }
    output.push('\n');
    let mut loop_id = 0;
    for procedure in &module.procedures {
        output.push_str(&format!("static void {}({}) {{\n", c_procedure(&procedure.name), parameters_signature(&procedure.parameters)));
        emit_instructions(&procedure.body, &mut output, &mut loop_id);
        output.push_str("    return;\n}\n\n");
    }
    for function in &module.functions {
        output.push_str(&format!("static long long {}({}) {{\n", c_function(&function.name), parameters_signature(&function.parameters)));
        emit_instructions(&function.body, &mut output, &mut loop_id);
        if let Some(value) = &function.return_value { output.push_str(&format!("    return {};\n", emit_expression(value))); }
        else { output.push_str("    return 0;\n"); }
        output.push_str("}\n\n");
    }
    output.push_str("int main(void) {\n");
    emit_instructions(&module.main, &mut output, &mut loop_id);
    output.push_str("    return 0;\n}\n");
    output
}

#[cfg(test)]
mod tests {
    use super::{emit_c, lower};
    use crate::frontend::parse_document;
    use crate::source::SourceMap;

    #[test]
    fn lowers_nested_for_and_numeric_subprograms() {
        let source = "x=0\nFOR i=1 TO 3\n x=x+i\nNEXT i\nPROCEDURE add(v)\n x=x+v\nRETURN\nFUNCTION twice(v)\n RETURN v*2\nENDFUNC\nDEFFN square(v)=v*v\nGOSUB add(4)\nx=FN twice(x)+FN square(2)\nPRINT x\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("structured.bas", source);
        let document = parse_document(id, source).expect("structured syntax parses");
        let module = lower(&document.program).expect("calls resolve and lower");
        assert_eq!(module.procedures.len(), 1);
        assert_eq!(module.functions.len(), 2);
        let c = emit_c(&module);
        assert!(c.contains("do {"));
        assert!(c.contains("proc_add"));
        assert!(c.contains("fn_twice"));
        assert!(c.contains("fn_square"));
        assert!(c.contains("gfa_mul"));
    }

    #[test]
    fn rejects_unknown_calls_and_wrong_arity() {
        let source = "GOSUB missing(1)\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("unknown-call.bas", source);
        let document = parse_document(id, source).unwrap();
        let errors = lower(&document.program).expect_err("unknown procedure must fail");
        assert!(errors.iter().any(|error| error.message.contains("unknown procedure")));
    }

    #[test]
    fn rejects_zero_literal_step() {
        let source = "FOR i=1 TO 3 STEP 0\n x=1\nNEXT i\nPRINT x\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("zero-step.bas", source);
        let document = parse_document(id, source).unwrap();
        let errors = lower(&document.program).expect_err("zero step must fail");
        assert!(errors.iter().any(|error| error.message.contains("STEP must not be zero")));
    }
}
