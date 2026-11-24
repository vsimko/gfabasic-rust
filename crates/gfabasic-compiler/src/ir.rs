//! Project-owned typed IR for the evidence-backed literal-intrinsic prototype.
//!
//! This first CFG slice models one straight-line entry block, i64 constants,
//! ordered PRINT operations, and an explicit synthetic return terminator. It
//! does not imply that GFA BASIC variables are universally i64 or that general
//! branching, loops, calls, and runtime operations are implemented.

use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::Statement;
use crate::semantic::{SemanticModel, SymbolId, SymbolTable};
use crate::source::SourceSpan;

/// Stable identity of a value within one IR module.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ValueId(pub u32);

/// Stable identity of a basic block within one IR module.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BlockId(pub u32);

/// Origin of an IR structure, distinguishing source-backed and generated nodes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceOrigin {
    Source(SourceSpan),
    Synthetic,
}

/// Explicit control-flow exit for the current straight-line IR subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Terminator {
    Return { origin: SourceOrigin },
}

/// One explicitly identified basic block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicBlock {
    pub id: BlockId,
    pub origin: SourceOrigin,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Terminator>,
}

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

/// One function body with an explicit entry block.
///
/// This increment deliberately supports a single straight-line block only;
/// branch terminators and multi-block CFG verification are future work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    pub entry: BlockId,
    pub blocks: Vec<BasicBlock>,
}

impl Module {
    fn single_block() -> Self {
        let entry = BlockId(0);
        Self {
            entry,
            blocks: vec![BasicBlock {
                id: entry,
                origin: SourceOrigin::Synthetic,
                instructions: Vec::new(),
                terminator: None,
            }],
        }
    }

    fn entry_block_mut(&mut self) -> &mut BasicBlock {
        let entry = self.entry;
        self.blocks
            .iter_mut()
            .find(|block| block.id == entry)
            .expect("single-block lowering initializes its entry")
    }
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

fn unary_intrinsic(declaration_id: &str, argument: i64) -> Result<Option<i64>, String> {
    // Preserve the compiler prototype's existing literal bound. This is not
    // promoted to a historical GFA BASIC language limit.
    if !(0..=20).contains(&argument) {
        return Ok(None);
    }
    let argument = u32::try_from(argument)
        .map_err(|_| format!("integer argument for {declaration_id} is outside the runtime ABI"))?;
    match crate::semantic::resolve_api_handler(declaration_id).map_err(|error| error.to_string())? {
        gfabasic_runtime::handlers::ResolvedHandler::IntegerToUnsigned(handler) => {
            Ok(handler(argument).and_then(|value| i64::try_from(value).ok()))
        }
        _ => Err(format!(
            "runtime handler signature mismatch for {declaration_id}"
        )),
    }
}

fn integer_intrinsic(declaration_id: &str, argument: i64) -> Result<Option<i64>, String> {
    match crate::semantic::resolve_api_handler(declaration_id).map_err(|error| error.to_string())? {
        gfabasic_runtime::handlers::ResolvedHandler::IntegerToInteger(handler) => {
            Ok(handler(argument))
        }
        _ => Err(format!(
            "runtime handler signature mismatch for {declaration_id}"
        )),
    }
}

fn real_to_integer_intrinsic(declaration_id: &str, argument: i64) -> Result<Option<i64>, String> {
    match crate::semantic::resolve_api_handler(declaration_id).map_err(|error| error.to_string())? {
        gfabasic_runtime::handlers::ResolvedHandler::RealToInteger(handler) => {
            Ok(handler(argument as f64))
        }
        _ => Err(format!(
            "runtime handler signature mismatch for {declaration_id}"
        )),
    }
}

fn binary_intrinsic(declaration_id: &str, left: i64, right: i64) -> Result<Option<i64>, String> {
    // Preserve the compiler prototype's existing literal bound. This is not
    // promoted to a historical GFA BASIC language limit.
    if !(0..=20).contains(&left) || !(0..=left).contains(&right) {
        return Ok(None);
    }
    let left = u32::try_from(left)
        .map_err(|_| format!("integer argument for {declaration_id} is outside the runtime ABI"))?;
    let right = u32::try_from(right)
        .map_err(|_| format!("integer argument for {declaration_id} is outside the runtime ABI"))?;
    match crate::semantic::resolve_api_handler(declaration_id).map_err(|error| error.to_string())? {
        gfabasic_runtime::handlers::ResolvedHandler::TwoIntegersToUnsigned(handler) => {
            Ok(handler(left, right).and_then(|value| i64::try_from(value).ok()))
        }
        _ => Err(format!(
            "runtime handler signature mismatch for {declaration_id}"
        )),
    }
}

/// Mutable state shared by the prototype's straight-line lowering operations.
struct LoweringState<'a> {
    symbols: &'a SymbolTable,
    variables: BTreeMap<SymbolId, ValueId>,
    module: Module,
    next_value: u32,
    errors: Vec<IrError>,
}

impl<'a> LoweringState<'a> {
    fn new(symbols: &'a SymbolTable) -> Self {
        Self {
            symbols,
            variables: BTreeMap::new(),
            module: Module::single_block(),
            next_value: 0,
            errors: Vec::new(),
        }
    }

    fn lower_assignment(
        &mut self,
        statement_span: SourceSpan,
        value: Option<i64>,
        span: SourceSpan,
    ) {
        let Some(symbol_id) = self
            .symbols
            .symbols
            .iter()
            .find(|symbol| symbol.statement_span == statement_span)
            .map(|symbol| symbol.id)
        else {
            self.errors.push(IrError::at(
                "assignment has no declaration in the resolved semantic model",
                span,
            ));
            return;
        };
        if self.variables.contains_key(&symbol_id) {
            self.errors.push(IrError::at(
                format!("symbol {:?} has more than one IR definition", symbol_id),
                span,
            ));
            return;
        }
        let Some(value) = value else {
            self.errors.push(IrError::at(
                "intrinsic argument is outside the supported literal subset or the runtime result is unavailable",
                span,
            ));
            return;
        };
        let id = ValueId(self.next_value);
        self.next_value += 1;
        self.variables.insert(symbol_id, id);
        self.module
            .entry_block_mut()
            .instructions
            .push(Instruction::ConstI64 {
                result: id,
                value,
                span,
            });
    }
}

/// Lower a resolved semantic model to typed, constant-valued IR.
///
/// Intrinsics are evaluated here only for the prototype's documented literal
/// subset. Their bounds are implementation limits, not dialect limits.
pub fn lower(model: &SemanticModel) -> Result<Module, Vec<IrError>> {
    let mut state = LoweringState::new(&model.symbols);
    let mut saw_print = false;

    for statement in &model.document.program.statements {
        match statement {
            Statement::AssignInteger { value, span, .. } => {
                state.lower_assignment(*span, Some(*value), *span);
            }
            Statement::AssignFact { argument, span, .. } => {
                let value = match unary_intrinsic("gfa.language.math.fact", *argument) {
                    Ok(value) => value,
                    Err(message) => {
                        state.errors.push(IrError::at(message, *span));
                        continue;
                    }
                };
                state.lower_assignment(*span, value, *span);
            }
            Statement::AssignSucc { argument, span, .. } => {
                let value = match integer_intrinsic("gfa.language.numeric.succ", *argument) {
                    Ok(value) => value,
                    Err(message) => {
                        state.errors.push(IrError::at(message, *span));
                        continue;
                    }
                };
                state.lower_assignment(*span, value, *span);
            }
            Statement::AssignInt { argument, span, .. } => {
                let value = match real_to_integer_intrinsic("gfa.language.numeric.int", *argument) {
                    Ok(value) => value,
                    Err(message) => {
                        state.errors.push(IrError::at(message, *span));
                        continue;
                    }
                };
                state.lower_assignment(*span, value, *span);
            }
            Statement::AssignVariat { n, k, span, .. } => {
                let value = match binary_intrinsic("gfa.language.math.variat", *n, *k) {
                    Ok(value) => value,
                    Err(message) => {
                        state.errors.push(IrError::at(message, *span));
                        continue;
                    }
                };
                state.lower_assignment(*span, value, *span);
            }
            Statement::AssignCombin { n, k, span, .. } => {
                let value = match binary_intrinsic("gfa.language.math.combin", *n, *k) {
                    Ok(value) => value,
                    Err(message) => {
                        state.errors.push(IrError::at(message, *span));
                        continue;
                    }
                };
                state.lower_assignment(*span, value, *span);
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
            | Statement::MatInv { span, .. }
            | Statement::MatCopy { span, .. }
            | Statement::MatRead { span, .. }
            | Statement::MatInput { span, .. } => {
                state.errors.push(IrError::at(
                    "matrix statements are supported by the parser and formatter but not by the executable compiler prototype",
                    *span,
                ));
            }
            Statement::PrintVariables { names, span } => {
                let references: Vec<_> = model
                    .symbols
                    .references
                    .iter()
                    .filter(|reference| reference.statement_span == *span)
                    .collect();
                let mut values = Vec::with_capacity(references.len());
                for reference in references {
                    match state.variables.get(&reference.symbol) {
                        Some(value) => values.push(*value),
                        None => state.errors.push(IrError::at(
                            format!(
                                "resolved symbol {:?} has no IR definition",
                                reference.symbol
                            ),
                            reference.span,
                        )),
                    }
                }
                if values.len() == names.len() {
                    state
                        .module
                        .entry_block_mut()
                        .instructions
                        .push(Instruction::Print {
                            values,
                            span: *span,
                        });
                    saw_print = true;
                }
            }
        }
    }

    if !saw_print {
        state.errors.push(IrError::global(
            "prototype requires at least one valid PRINT statement",
        ));
    }
    if state.errors.is_empty() {
        state.module.entry_block_mut().terminator = Some(Terminator::Return {
            origin: SourceOrigin::Synthetic,
        });
        verify(&state.module)?;
        Ok(state.module)
    } else {
        Err(state.errors)
    }
}

/// Verify the current single-block CFG, its explicit exit, definitions, and uses.
pub fn verify(module: &Module) -> Result<(), Vec<IrError>> {
    let mut errors = Vec::new();
    if module.blocks.len() != 1 {
        errors.push(IrError::global(
            "prototype IR currently requires exactly one straight-line basic block",
        ));
    }
    let Some(block) = module.blocks.iter().find(|block| block.id == module.entry) else {
        errors.push(IrError::global("IR entry block does not exist"));
        return Err(errors);
    };
    if block.terminator.is_none() {
        errors.push(IrError::global("IR basic block is missing a terminator"));
    }
    let mut definitions = BTreeSet::new();
    for instruction in &block.instructions {
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
        .blocks
        .iter()
        .flat_map(|block| block.instructions.iter())
        .find_map(|instruction| match instruction {
            Instruction::ConstI64 { result, .. } if *result == id => Some(ValueType::I64),
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use super::{
        BasicBlock, BlockId, Instruction, Module, SourceOrigin, Terminator, ValueId, ValueType,
        lower, value_type, verify,
    };
    use crate::frontend::parse_document;
    use crate::semantic::analyze_prototype;
    use crate::source::{SourceMap, SourceSpan};

    fn semantic_model(
        source_id: crate::source::SourceId,
        source: &str,
    ) -> crate::semantic::SemanticModel {
        let document = parse_document(source_id, source).expect("supported syntax parses");
        analyze_prototype(document).expect("prototype semantic analysis succeeds")
    }

    #[test]
    fn lowers_all_three_intrinsics_to_typed_constants_and_print() {
        let source = "x=FACT(5)\ny=VARIAT(5,2)\nz=COMBIN(5,2)\nPRINT z,x,y\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("combinatorics.bas", source);
        let model = semantic_model(id, source);
        let ir = lower(&model).unwrap();

        assert_eq!(ir.blocks[0].instructions.len(), 4);
        assert!(matches!(
            ir.blocks[0].instructions[0],
            Instruction::ConstI64 {
                result: ValueId(0),
                value: 120,
                ..
            }
        ));
        assert!(matches!(
            ir.blocks[0].instructions[1],
            Instruction::ConstI64 {
                result: ValueId(1),
                value: 20,
                ..
            }
        ));
        assert!(matches!(
            ir.blocks[0].instructions[2],
            Instruction::ConstI64 {
                result: ValueId(2),
                value: 10,
                ..
            }
        ));
        assert!(matches!(
            &ir.blocks[0].instructions[3],
            Instruction::Print { values, .. }
                if values == &[ValueId(2), ValueId(0), ValueId(1)]
        ));
        assert_eq!(value_type(&ir, ValueId(0)), Some(ValueType::I64));
        assert_eq!(ir.entry, BlockId(0));
        assert_eq!(ir.blocks[0].origin, SourceOrigin::Synthetic);
        assert_eq!(
            ir.blocks[0].terminator,
            Some(Terminator::Return {
                origin: SourceOrigin::Synthetic,
            })
        );
        verify(&ir).unwrap();
    }

    #[test]
    fn lowers_integer_literal_assignments_to_constant_values() {
        let source = "x=42\nLET y=7\nPRINT x,y\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("integer-assignments.bas", source);
        let model = semantic_model(source_id, source);
        let module = lower(&model).expect("integer assignments lower to constants");

        assert!(matches!(
            module.blocks[0].instructions[0],
            Instruction::ConstI64 { value: 42, .. }
        ));
        assert!(matches!(
            module.blocks[0].instructions[1],
            Instruction::ConstI64 { value: 7, .. }
        ));
        assert!(matches!(
            &module.blocks[0].instructions[2],
            Instruction::Print { values, .. } if values.len() == 2
        ));
    }

    #[test]
    fn rejects_matrix_statements_from_the_executable_build_subset() {
        let source = include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-one-identity.lst");
        let mut sources = SourceMap::new();
        let id = sources.add_source("matrix.bas", source);
        let document = parse_document(id, source).unwrap();
        let errors = analyze_prototype(document).expect_err("matrix semantics are not implemented");
        assert!(
            errors
                .iter()
                .any(|error| error.message.contains("matrix semantic analysis")),
            "expected an explicit matrix support diagnostic: {errors:?}"
        );
    }

    #[test]
    fn handles_zero_and_boundary_combinatorics_values() {
        let source = "x=FACT(0)\ny=VARIAT(20,0)\nz=COMBIN(20,10)\nPRINT x,y,z\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("boundary.bas", source);
        let model = semantic_model(id, source);
        let ir = lower(&model).unwrap();

        assert!(matches!(
            ir.blocks[0].instructions[0],
            Instruction::ConstI64 { value: 1, .. }
        ));
        assert!(matches!(
            ir.blocks[0].instructions[1],
            Instruction::ConstI64 { value: 1, .. }
        ));
        assert!(matches!(
            ir.blocks[0].instructions[2],
            Instruction::ConstI64 { value: 184756, .. }
        ));
    }

    #[test]
    fn rejects_undefined_variables_and_reassignment() {
        for source in ["PRINT x\n", "x=FACT(1)\nx=FACT(2)\nPRINT x\n"] {
            let mut sources = SourceMap::new();
            let id = sources.add_source("invalid.bas", source);
            let document = parse_document(id, source).unwrap();
            assert!(analyze_prototype(document).is_err(), "accepted: {source}");
        }
    }

    #[test]
    fn verifier_rejects_use_before_definition_and_duplicate_ids() {
        let source = "PRINT x\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("ir-invalid.bas", source);
        let span = crate::source::SourceSpan::new(id, 0, source.len());
        let undefined = Module {
            entry: BlockId(0),
            blocks: vec![BasicBlock {
                id: BlockId(0),
                origin: SourceOrigin::Synthetic,
                instructions: vec![Instruction::Print {
                    values: vec![ValueId(9)],
                    span,
                }],
                terminator: Some(Terminator::Return {
                    origin: SourceOrigin::Synthetic,
                }),
            }],
        };
        assert!(verify(&undefined).is_err());

        let duplicate = Module {
            entry: BlockId(0),
            blocks: vec![BasicBlock {
                id: BlockId(0),
                origin: SourceOrigin::Synthetic,
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
                terminator: Some(Terminator::Return {
                    origin: SourceOrigin::Synthetic,
                }),
            }],
        };
        assert!(verify(&duplicate).is_err());

        let missing_terminator = Module {
            entry: BlockId(0),
            blocks: vec![BasicBlock {
                id: BlockId(0),
                origin: SourceOrigin::Synthetic,
                instructions: vec![Instruction::ConstI64 {
                    result: ValueId(0),
                    value: 1,
                    span,
                }],
                terminator: None,
            }],
        };
        assert!(
            verify(&missing_terminator)
                .unwrap_err()
                .iter()
                .any(|error| error.message.contains("missing a terminator"))
        );

        let missing_entry = Module {
            entry: BlockId(1),
            blocks: vec![BasicBlock {
                id: BlockId(0),
                origin: SourceOrigin::Synthetic,
                instructions: Vec::new(),
                terminator: Some(Terminator::Return {
                    origin: SourceOrigin::Synthetic,
                }),
            }],
        };
        assert!(
            verify(&missing_entry)
                .unwrap_err()
                .iter()
                .any(|error| error.message.contains("entry block does not exist"))
        );
    }

    #[test]
    fn lowers_succ_and_integer_literal_int_through_typed_registry() {
        let source = "x=SUCC(2)\ny=INT(5)\nPRINT x,y\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("inherited-intrinsics.bas", source);
        let model = semantic_model(id, source);
        let ir = lower(&model).expect("supported integer intrinsic forms lower");
        assert_eq!(ir.blocks[0].instructions.len(), 3);
        assert!(matches!(
            ir.blocks[0].instructions[0],
            Instruction::ConstI64 {
                result: ValueId(0),
                value: 3,
                ..
            }
        ));
        assert!(matches!(
            ir.blocks[0].instructions[1],
            Instruction::ConstI64 {
                result: ValueId(1),
                value: 5,
                ..
            }
        ));
        verify(&ir).expect("literal intrinsic IR is valid");
    }
    #[test]
    fn keeps_the_original_source_span_on_intrinsic_lowering_errors() {
        let source = "x=SUCC(9223372036854775807)\n";
        let mut sources = SourceMap::new();
        let id = sources.add_source("intrinsic-overflow.bas", source);
        let model = semantic_model(id, source);
        let errors =
            lower(&model).expect_err("successor overflow is outside the implemented subset");

        let expected_span = Some(SourceSpan::new(id, 0, source.trim_end().len()));
        assert!(errors.iter().any(|error| error.span == expected_span));
    }
}
