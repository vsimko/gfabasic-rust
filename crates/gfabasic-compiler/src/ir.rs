//! Project-owned typed IR for the evidence-backed combinatorics prototype.
//!
//! This is intentionally smaller than the planned general-purpose IR. It
//! models only i64 constants and ordered PRINT operations; it does not imply
//! that GFA BASIC variables are universally i64.

use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::{Program, Statement};
use crate::source::SourceSpan;

/// Stable identity of a value within one IR module.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ValueId(pub u32);

/// Types currently representable by this deliberately narrow IR.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueType {
    I64,
}

/// A verified, source-provenanced instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    ConstI64 {
        result: ValueId,
        value: i64,
        span: SourceSpan,
    },
    Print {
        values: Vec<ValueId>,
        span: SourceSpan,
    },
}

/// One straight-line function body. Control flow and general calls are not
/// represented by this prototype IR.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Module {
    pub instructions: Vec<Instruction>,
}

/// Lowering or verification failure with source provenance where available.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IrError {
    pub message: String,
    pub span: Option<SourceSpan>,
}

impl IrError {
    fn at(message: impl Into<String>, span: SourceSpan) -> Self {
        Self {
            message: message.into(),
            span: Some(span),
        }
    }

    fn global(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
        }
    }
}

fn factorial(n: i64) -> Option<i64> {
    if !(0..=20).contains(&n) {
        return None;
    }
    (1..=n).try_fold(1_i64, |acc, x| acc.checked_mul(x))
}

fn variat(n: i64, k: i64) -> Option<i64> {
    if !(0..=20).contains(&n) || !(0..=n).contains(&k) {
        return None;
    }
    ((n - k + 1)..=n).try_fold(1_i64, |acc, x| acc.checked_mul(x))
}

fn combin(n: i64, k: i64) -> Option<i64> {
    if !(0..=20).contains(&n) || !(0..=n).contains(&k) {
        return None;
    }
    let k = k.min(n - k);
    let mut value = 1_i64;
    for i in 1..=k {
        value = value.checked_mul(n - k + i)?;
        value /= i;
    }
    Some(value)
}

/// Lower the current frontend AST to typed, constant-valued IR.
///
/// Intrinsics are evaluated here only for the prototype's documented literal
/// subset. Their bounds are implementation limits, not dialect limits.
pub fn lower(program: &Program) -> Result<Module, Vec<IrError>> {
    let mut module = Module::default();
    let mut variables = BTreeMap::<String, ValueId>::new();
    let mut next_value = 0_u32;
    let mut errors = Vec::new();
    let mut saw_print = false;

    for statement in &program.statements {
        match statement {
            Statement::AssignFact {
                name,
                argument,
                span,
            } => {
                let value = factorial(*argument);
                lower_assignment(
                    name,
                    value,
                    *span,
                    &mut variables,
                    &mut module,
                    &mut next_value,
                    &mut errors,
                );
            }
            Statement::AssignVariat { name, n, k, span } => {
                let value = variat(*n, *k);
                lower_assignment(
                    name,
                    value,
                    *span,
                    &mut variables,
                    &mut module,
                    &mut next_value,
                    &mut errors,
                );
            }
            Statement::AssignCombin { name, n, k, span } => {
                let value = combin(*n, *k);
                lower_assignment(
                    name,
                    value,
                    *span,
                    &mut variables,
                    &mut module,
                    &mut next_value,
                    &mut errors,
                );
            }
            Statement::DimMatrix { span, .. }
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
            | Statement::MatCopy { span, .. }
            | Statement::MatRead { span, .. }
            | Statement::MatInput { span, .. } => {
                errors.push(IrError::at(
                    "matrix statements are supported by the parser and formatter but not by the executable compiler prototype",
                    *span,
                ));
            }
            Statement::PrintVariables { names, span } => {
                let mut values = Vec::with_capacity(names.len());
                for name in names {
                    match variables.get(name) {
                        Some(value) => values.push(*value),
                        None => errors.push(IrError::at(
                            format!("variable '{name}' is not assigned before PRINT"),
                            *span,
                        )),
                    }
                }
                if values.len() == names.len() {
                    module.instructions.push(Instruction::Print {
                        values,
                        span: *span,
                    });
                    saw_print = true;
                }
            }
        }
    }

    if !saw_print {
        errors.push(IrError::global(
            "prototype requires at least one valid PRINT statement",
        ));
    }
    if errors.is_empty() {
        verify(&module)?;
        Ok(module)
    } else {
        Err(errors)
    }
}

fn lower_assignment(
    name: &str,
    value: Option<i64>,
    span: SourceSpan,
    variables: &mut BTreeMap<String, ValueId>,
    module: &mut Module,
    next_value: &mut u32,
    errors: &mut Vec<IrError>,
) {
    if variables.contains_key(name) {
        errors.push(IrError::at(
            format!("reassignment of '{name}' is outside the current prototype subset"),
            span,
        ));
        return;
    }
    let Some(value) = value else {
        errors.push(IrError::at(
            "intrinsic arguments are outside the supported prototype range (0 <= k <= n <= 20)",
            span,
        ));
        return;
    };
    let id = ValueId(*next_value);
    *next_value += 1;
    variables.insert(name.to_owned(), id);
    module.instructions.push(Instruction::ConstI64 {
        result: id,
        value,
        span,
    });
}

/// Verify unique definitions and reject uses that precede their definitions.
pub fn verify(module: &Module) -> Result<(), Vec<IrError>> {
    let mut definitions = BTreeSet::new();
    let mut errors = Vec::new();
    for instruction in &module.instructions {
        match instruction {
            Instruction::ConstI64 { result, span, .. } => {
                if !definitions.insert(*result) {
                    errors.push(IrError::at(
                        format!("value %{} is defined more than once", result.0),
                        *span,
                    ));
                }
            }
            Instruction::Print { values, span } => {
                for value in values {
                    if !definitions.contains(value) {
                        errors.push(IrError::at(
                            format!("PRINT uses undefined i64 value %{}", value.0),
                            *span,
                        ));
                    }
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Return the explicit type of a value defined in this module.
pub fn value_type(module: &Module, id: ValueId) -> Option<ValueType> {
    module
        .instructions
        .iter()
        .find_map(|instruction| match instruction {
            Instruction::ConstI64 { result, .. } if *result == id => Some(ValueType::I64),
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use super::{Instruction, ValueId, ValueType, lower, value_type, verify};
    use crate::frontend::parse;
    use crate::source::SourceMap;

    #[test]
    fn lowers_all_three_intrinsics_to_typed_constants_and_print() {
        let source = "x=FACT(5)\ny=VARIAT(5,2)\nz=COMBIN(5,2)\nPRINT x,y,z\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("combinatorics.bas", source);
        let ast = parse(id, source).unwrap();
        let ir = lower(&ast).unwrap();

        assert_eq!(ir.instructions.len(), 4);
        assert!(matches!(
            ir.instructions[0],
            Instruction::ConstI64 {
                result: ValueId(0),
                value: 120,
                ..
            }
        ));
        assert!(matches!(
            ir.instructions[1],
            Instruction::ConstI64 {
                result: ValueId(1),
                value: 20,
                ..
            }
        ));
        assert!(matches!(
            ir.instructions[2],
            Instruction::ConstI64 {
                result: ValueId(2),
                value: 10,
                ..
            }
        ));
        assert!(matches!(
            &ir.instructions[3],
            Instruction::Print { values, .. }
                if values == &[ValueId(0), ValueId(1), ValueId(2)]
        ));
        assert_eq!(value_type(&ir, ValueId(0)), Some(ValueType::I64));
        verify(&ir).unwrap();
    }

    #[test]
    fn rejects_matrix_statements_from_the_executable_build_subset() {
        let source = include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-one-identity.bas");
        let mut sources = SourceMap::new();
        let id = sources.add_source("matrix.bas", source);
        let ast = parse(id, source).unwrap();

        let errors = lower(&ast).expect_err("matrix execution is not implemented");
        assert!(
            errors
                .iter()
                .any(|error| error.message.contains("matrix statements")),
            "expected an explicit matrix support diagnostic: {errors:?}"
        );
    }

    #[test]
    fn handles_zero_and_boundary_combinatorics_values() {
        let source = "x=FACT(0)\ny=VARIAT(20,0)\nz=COMBIN(20,10)\nPRINT x,y,z\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("boundary.bas", source);
        let ast = parse(id, source).unwrap();
        let ir = lower(&ast).unwrap();

        assert!(matches!(
            ir.instructions[0],
            Instruction::ConstI64 { value: 1, .. }
        ));
        assert!(matches!(
            ir.instructions[1],
            Instruction::ConstI64 { value: 1, .. }
        ));
        assert!(matches!(
            ir.instructions[2],
            Instruction::ConstI64 { value: 184756, .. }
        ));
    }

    #[test]
    fn rejects_undefined_variables_and_reassignment() {
        for source in ["PRINT x\n", "x=FACT(1)\nx=FACT(2)\nPRINT x\n"] {
            let mut sources = SourceMap::new();
            let id = sources.add_source("invalid.bas", source);
            let ast = parse(id, source).unwrap();
            assert!(lower(&ast).is_err(), "accepted: {source}");
        }
    }

    #[test]
    fn verifier_rejects_use_before_definition_and_duplicate_ids() {
        let source = "PRINT x\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("ir-invalid.bas", source);
        let span = crate::source::SourceSpan::new(id, 0, source.len());
        let undefined = super::Module {
            instructions: vec![Instruction::Print {
                values: vec![ValueId(9)],
                span,
            }],
        };
        assert!(verify(&undefined).is_err());

        let duplicate = super::Module {
            instructions: vec![
                Instruction::ConstI64 {
                    result: ValueId(1),
                    value: 1,
                    span,
                },
                Instruction::ConstI64 {
                    result: ValueId(1),
                    value: 2,
                    span,
                },
            ],
        };
        assert!(verify(&duplicate).is_err());
    }
}
