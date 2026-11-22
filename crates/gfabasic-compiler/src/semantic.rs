//! Symbol resolution and semantic analysis boundary.
//!
//! This module currently provides a deliberately narrow symbol index for the
//! evidence-backed FACT/VARIAT/COMBIN/PRINT prototype. It does not define
//! general Amiga GFA BASIC declaration, scope, or type rules.

use std::collections::BTreeMap;

use crate::frontend::{Expression, ForDirection, ParsedDocument, Program, Statement, TokenKind};

/// Stable identity of a symbol within one parsed document.
///
/// IDs are assigned in declaration order and are not persisted across edits.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SymbolId(pub u32);

/// One variable symbol declared by an assignment in the current prototype.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbol {
    /// Document-local identity.
    pub id: SymbolId,
    /// Exact source spelling; no case folding or translation is applied.
    pub name: String,
    /// Source span of the declaration identifier.
    pub declaration: crate::source::SourceSpan,
    /// Complete assignment statement that introduces this symbol.
    pub statement_span: crate::source::SourceSpan,
}

/// A reference resolved to one declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedReference {
    /// Identity of the referenced declaration.
    pub symbol: SymbolId,
    /// Source span of the reference identifier.
    pub span: crate::source::SourceSpan,
    /// PRINT statement containing this reference.
    pub statement_span: crate::source::SourceSpan,
}

/// Symbol declarations and resolved references for one parsed document.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SymbolTable {
    /// Symbols in source declaration order.
    pub symbols: Vec<Symbol>,
    /// References in source order.
    pub references: Vec<ResolvedReference>,
}

/// A parsed document paired with its resolved, document-local symbol graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticModel {
    /// Exact parsed document whose source spans are used by the symbol graph.
    pub document: ParsedDocument,
    /// Resolved declarations and source references for the prototype subset.
    pub symbols: SymbolTable,
}

/// Resolve the current supported subset and retain its source document.
pub fn analyze_prototype(document: ParsedDocument) -> Result<SemanticModel, Vec<SemanticError>> {
    let symbols = resolve_prototype_symbols(&document)?;
    Ok(SemanticModel { document, symbols })
}

impl SymbolTable {
    /// Find a symbol by its document-local identity.
    pub fn get(&self, id: SymbolId) -> Option<&Symbol> {
        self.symbols.iter().find(|symbol| symbol.id == id)
    }

    /// Iterate over references bound to the selected symbol.
    pub fn references_to(&self, id: SymbolId) -> impl Iterator<Item = &ResolvedReference> {
        self.references
            .iter()
            .filter(move |reference| reference.symbol == id)
    }
}

/// A source-located error in the current prototype's symbol rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticError {
    /// Stable diagnostic code.
    pub code: String,
    /// Human-readable diagnostic message.
    pub message: String,
    /// Source span responsible for the diagnostic.
    pub span: crate::source::SourceSpan,
}

/// Resolve assignment declarations and PRINT references in the current subset.
///
/// This prototype policy treats the first assignment to a name as its
/// declaration, rejects reassignment, and resolves PRINT names only to earlier
/// assignments. These are implementation limits of the current vertical slice,
/// not claims about the full Amiga GFA BASIC 3.5 declaration or scope rules.
pub fn resolve_prototype_symbols(
    document: &ParsedDocument,
) -> Result<SymbolTable, Vec<SemanticError>> {
    let mut by_name = BTreeMap::<String, SymbolId>::new();
    let mut table = SymbolTable::default();
    let mut errors = Vec::new();

    for statement in &document.program.statements {
        match statement {
            Statement::AssignInteger { name, span, .. }
            | Statement::AssignFact { name, span, .. }
            | Statement::AssignVariat { name, span, .. }
            | Statement::AssignCombin { name, span, .. } => {
                let declaration = document.syntax.tokens.iter().find_map(|token| {
                    if token.span.start < span.start || token.span.end > span.end {
                        return None;
                    }
                    match &token.kind {
                        TokenKind::Identifier(token_name) if token_name == name => Some(token.span),
                        _ => None,
                    }
                });

                let Some(declaration) = declaration else {
                    errors.push(SemanticError {
                        code: "GFA-SEM-0000".to_owned(),
                        message: "assignment declaration token is missing from lossless syntax"
                            .to_owned(),
                        span: *span,
                    });
                    continue;
                };

                if by_name.contains_key(name) {
                    errors.push(SemanticError {
                        code: "GFA-SEM-0001".to_owned(),
                        message: format!(
                            "reassignment of '{name}' is outside the current prototype subset"
                        ),
                        span: declaration,
                    });
                    continue;
                }

                let id = SymbolId(table.symbols.len() as u32);
                by_name.insert(name.clone(), id);
                table.symbols.push(Symbol {
                    id,
                    name: name.clone(),
                    declaration,
                    statement_span: *span,
                });
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
            | Statement::MatInput { span, .. } => errors.push(SemanticError {
                code: "GFA-SEM-0003".to_owned(),
                message: "matrix syntax is parsed for formatting but matrix semantic analysis is not implemented".to_owned(),
                span: *span,
            }),
            Statement::PrintVariables { names, span } => {
                let identifiers: Vec<_> = document
                    .syntax
                    .tokens
                    .iter()
                    .filter(|token| {
                        token.span.start >= span.start
                            && token.span.end <= span.end
                            && matches!(&token.kind, TokenKind::Identifier(_))
                    })
                    .collect();

                for (name, token) in names.iter().zip(identifiers.into_iter().skip(1)) {
                    let TokenKind::Identifier(token_name) = &token.kind else {
                        continue;
                    };
                    if token_name != name {
                        errors.push(SemanticError {
                            code: "GFA-SEM-0000".to_owned(),
                            message: "PRINT identifier does not match the parsed AST".to_owned(),
                            span: token.span,
                        });
                        continue;
                    }

                    match by_name.get(name) {
                        Some(id) => table.references.push(ResolvedReference {
                            symbol: *id,
                            span: token.span,
                            statement_span: *span,
                        }),
                        None => errors.push(SemanticError {
                            code: "GFA-SEM-0002".to_owned(),
                            message: format!("variable '{name}' is not assigned before PRINT"),
                            span: token.span,
                        }),
                    }
                }
            }
            _ => {}
        }
    }

    if errors.is_empty() {
        Ok(table)
    } else {
        Err(errors)
    }
}

/// Validate declarations, parameter lists, calls, and counted-loop invariants for
/// the structured compiler subset. User procedures/functions are resolved
/// document-wide, so forward calls and recursion are legal.
pub fn validate_structured_program(program: &Program) -> Result<(), Vec<SemanticError>> {
    use std::collections::BTreeMap;

    #[derive(Clone)]
    struct Callable { function: bool, arity: usize, span: crate::source::SourceSpan }

    fn error(errors: &mut Vec<SemanticError>, code: &str, message: impl Into<String>, span: crate::source::SourceSpan) {
        errors.push(SemanticError { code: code.to_owned(), message: message.into(), span });
    }
    fn register(
        name: &str, parameters: &[String], function: bool, span: crate::source::SourceSpan,
        declarations: &mut BTreeMap<String, Callable>, errors: &mut Vec<SemanticError>,
    ) {
        let key = name.to_ascii_uppercase();
        if matches!(key.as_str(), "FACT" | "VARIAT" | "COMBIN") {
            error(errors, "GFA-SEM-0013", format!("'{name}' conflicts with a built-in function"), span);
        }
        if let Some(previous) = declarations.get(&key) {
            error(errors, "GFA-SEM-0013", format!("duplicate procedure/function declaration '{name}'"), span);
            let _ = previous.span;
        } else {
            declarations.insert(key, Callable { function, arity: parameters.len(), span });
        }
        let mut seen = std::collections::BTreeSet::new();
        for parameter in parameters {
            if !seen.insert(parameter.to_ascii_uppercase()) {
                error(errors, "GFA-SEM-0014", format!("duplicate parameter '{parameter}' in '{name}'"), span);
            }
        }
    }
    fn scan_expr(
        expr: &Expression, declarations: &BTreeMap<String, Callable>,
        errors: &mut Vec<SemanticError>,
    ) {
        match expr {
            Expression::Integer(_, _) | Expression::Variable(_, _) => {}
            Expression::Negate(value, _) => scan_expr(value, declarations, errors),
            Expression::Add(left, right, _) | Expression::Subtract(left, right, _)
            | Expression::Multiply(left, right, _) | Expression::Divide(left, right, _) => {
                scan_expr(left, declarations, errors);
                scan_expr(right, declarations, errors);
            }
            Expression::Call { name, arguments, span } => {
                for argument in arguments { scan_expr(argument, declarations, errors); }
                let key = name.to_ascii_uppercase();
                let builtin_arity = match key.as_str() { "FACT" => Some(1), "VARIAT" | "COMBIN" => Some(2), _ => None };
                if let Some(arity) = builtin_arity {
                    if arguments.len() != arity {
                        error(errors, "GFA-SEM-0012", format!("{name} expects {arity} argument(s)"), *span);
                    }
                    if !arguments.iter().all(|arg| matches!(arg, Expression::Integer(_, _) | Expression::Negate(_, _))) {
                        error(errors, "GFA-SEM-0016", format!("{name} arguments must be integer literals in this compiler subset"), *span);
                    }
                } else {
                    match declarations.get(&key) {
                        Some(callable) if !callable.function => error(errors, "GFA-SEM-0011", format!("'{name}' is a procedure, not a function"), *span),
                        Some(callable) if callable.arity != arguments.len() => error(errors, "GFA-SEM-0012", format!("function '{name}' expects {} argument(s), got {}", callable.arity, arguments.len()), *span),
                        Some(_) => {}
                        None => error(errors, "GFA-SEM-0010", format!("unknown function '{name}'"), *span),
                    }
                }
            }
        }
    }
    fn scan_statements(
        statements: &[Statement], declarations: &BTreeMap<String, Callable>,
        errors: &mut Vec<SemanticError>,
    ) {
        for statement in statements {
            match statement {
                Statement::AssignExpression { value, .. } => scan_expr(value, declarations, errors),
                Statement::PrintExpressions { values, .. } => for value in values { scan_expr(value, declarations, errors); },
                Statement::For { start, end, step, body, direction, span, .. } => {
                    scan_expr(start, declarations, errors);
                    scan_expr(end, declarations, errors);
                    if let Some(step) = step {
                        scan_expr(step, declarations, errors);
                        if matches!(step, Expression::Integer(0, _) | Expression::Negate(box Expression::Integer(0, _), _)) {
                            error(errors, "GFA-SEM-0015", "FOR STEP must not be zero", *span);
                        }
                    }
                    if *direction == ForDirection::DownTo && step.is_some() {
                        error(errors, "GFA-SEM-0015", "DOWNTO cannot be combined with STEP", *span);
                    }
                    scan_statements(body, declarations, errors);
                }
                Statement::Gosub { name, arguments, span } => {
                    for argument in arguments { scan_expr(argument, declarations, errors); }
                    match declarations.get(&name.to_ascii_uppercase()) {
                        Some(callable) if callable.function => error(errors, "GFA-SEM-0011", format!("'{name}' is a function, not a procedure"), *span),
                        Some(callable) if callable.arity != arguments.len() => error(errors, "GFA-SEM-0012", format!("procedure '{name}' expects {} argument(s), got {}", callable.arity, arguments.len()), *span),
                        Some(_) => {}
                        None => error(errors, "GFA-SEM-0010", format!("unknown procedure '{name}'"), *span),
                    }
                }
                Statement::Procedure { body, .. } | Statement::Function { body, .. } => {
                    scan_statements(body, declarations, errors);
                }
                Statement::Function { result, .. } => scan_expr(result, declarations, errors),
                Statement::AssignFact { .. } | Statement::AssignVariat { .. } | Statement::AssignCombin { .. }
                | Statement::AssignInteger { .. } | Statement::PrintVariables { .. } => {}
                _ => {}
            }
        }
    }

    let mut declarations = BTreeMap::<String, Callable>::new();
    let mut errors = Vec::new();
    for statement in &program.statements {
        match statement {
            Statement::Procedure { name, parameters, span, .. } => register(name, parameters, false, *span, &mut declarations, &mut errors),
            Statement::Function { name, parameters, span, .. } => register(name, parameters, true, *span, &mut declarations, &mut errors),
            _ => {}
        }
    }
    for statement in &program.statements {
        match statement {
            Statement::Procedure { body, .. } | Statement::Function { body, .. } => {
                if body.iter().any(|statement| matches!(statement, Statement::Procedure { .. } | Statement::Function { .. })) {
                    error(&mut errors, "GFA-SEM-0017", "nested procedure/function declarations are not supported", body.iter().find(|statement| matches!(statement, Statement::Procedure { .. } | Statement::Function { .. })).map(|s| match s { Statement::Procedure { span, .. } | Statement::Function { span, .. } => *span, _ => unreachable!() }).unwrap());
                }
            }
            _ => {}
        }
    }
    scan_statements(&program.statements, &declarations, &mut errors);
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// Resolve a built-in API call through the canonical registry and runtime table.
///
/// The declaration ID comes from the API registry; source spelling is never
/// used as a fallback lookup key.
pub fn resolve_api_handler(
    declaration_id: &str,
) -> Result<gfabasic_runtime::handlers::ResolvedHandler, gfabasic_runtime::handlers::RegistryError>
{
    gfabasic_runtime::handlers::resolve_builtin_handler(declaration_id)
}

#[cfg(test)]
mod tests {
    use super::{SymbolId, resolve_prototype_symbols};
    use crate::frontend::parse_document;
    use crate::source::SourceMap;

    #[test]
    fn resolves_print_reference_to_the_exact_assignment_symbol() {
        let source_text = "x=FACT(1)\nPRINT x\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("symbols.bas", source_text);
        let document = parse_document(source_id, source_text).expect("prototype source parses");
        let symbols =
            resolve_prototype_symbols(&document).expect("PRINT should resolve to prior assignment");

        assert_eq!(symbols.symbols.len(), 1);
        assert_eq!(symbols.symbols[0].id, SymbolId(0));
        assert_eq!(symbols.symbols[0].name, "x");
        assert_eq!(symbols.references_to(SymbolId(0)).count(), 1);
    }

    #[test]
    fn resolves_let_assignment_declaration_and_print_reference() {
        let source_text = "LET x=FACT(1)\nPRINT x\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("let-symbols.bas", source_text);
        let document = parse_document(source_id, source_text).expect("LET syntax is valid");
        let symbols = resolve_prototype_symbols(&document)
            .expect("LET assignment should declare x for the following PRINT");

        assert_eq!(symbols.symbols.len(), 1);
        assert_eq!(symbols.symbols[0].name, "x");
        let declaration = symbols.symbols[0].declaration;
        assert_eq!(&source_text[declaration.start..declaration.end], "x");
        assert_eq!(symbols.references_to(SymbolId(0)).count(), 1);
    }

    #[test]
    fn resolves_integer_assignment_as_a_symbol_declaration() {
        let source_text = "LET x=42\nPRINT x\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("integer-symbols.bas", source_text);
        let document = parse_document(source_id, source_text).expect("integer assignment parses");
        let symbols = resolve_prototype_symbols(&document)
            .expect("integer assignment should declare x for PRINT");

        assert_eq!(symbols.symbols.len(), 1);
        assert_eq!(symbols.symbols[0].name, "x");
        let declaration = symbols.symbols[0].declaration;
        assert_eq!(&source_text[declaration.start..declaration.end], "x");
        assert_eq!(symbols.references_to(SymbolId(0)).count(), 1);
    }

    #[test]
    fn reports_matrix_semantics_as_not_implemented() {
        let source = include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-one-identity.lst");
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("matrix.bas", source);
        let document = parse_document(source_id, source).expect("matrix fixture syntax parses");

        let errors = resolve_prototype_symbols(&document)
            .expect_err("matrix semantics are outside the current semantic model");
        assert!(errors.iter().any(|error| error.code == "GFA-SEM-0003"));
    }

    #[test]
    fn diagnoses_print_reference_without_a_prior_assignment() {
        let source_text = "PRINT x\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("unresolved.bas", source_text);
        let document = parse_document(source_id, source_text).expect("syntax is valid");

        let errors = resolve_prototype_symbols(&document).expect_err("x is unresolved");
        assert_eq!(errors[0].code, "GFA-SEM-0002");
        assert_eq!(errors[0].span.start, source_text.find('x').unwrap());
    }

    #[test]
    fn resolves_multiple_symbols_and_references_by_declaration_identity() {
        let source_text = "LET x=FACT(1)\nLET y=VARIAT(5,2)\nPRINT y,x,y\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("multiple-symbols.LST", source_text);
        let document = parse_document(source_id, source_text).expect("supported source parses");
        let symbols =
            resolve_prototype_symbols(&document).expect("all references resolve to declarations");

        assert_eq!(symbols.symbols.len(), 2);
        assert_eq!(symbols.symbols[0].id, SymbolId(0));
        assert_eq!(symbols.symbols[0].name, "x");
        assert_eq!(sources.slice(symbols.symbols[0].declaration), Some("x"));
        assert_eq!(symbols.symbols[1].id, SymbolId(1));
        assert_eq!(symbols.symbols[1].name, "y");
        assert_eq!(sources.slice(symbols.symbols[1].declaration), Some("y"));
        assert_eq!(
            symbols.get(SymbolId(0)).map(|symbol| symbol.name.as_str()),
            Some("x")
        );
        assert_eq!(
            symbols.get(SymbolId(1)).map(|symbol| symbol.name.as_str()),
            Some("y")
        );

        let references: Vec<_> = symbols.references.iter().collect();
        assert_eq!(references.len(), 3);
        let expected = [(SymbolId(1), "y"), (SymbolId(0), "x"), (SymbolId(1), "y")];
        for (reference, (expected_symbol, expected_name)) in references.iter().zip(expected) {
            assert_eq!(reference.symbol, expected_symbol);
            assert_eq!(sources.slice(reference.span), Some(expected_name));
        }

        assert_eq!(symbols.references_to(SymbolId(0)).count(), 1);
        assert_eq!(symbols.references_to(SymbolId(1)).count(), 2);
        assert!(symbols.get(SymbolId(2)).is_none());
    }

    #[test]
    fn diagnoses_each_unresolved_print_reference_with_its_own_span() {
        let source_text = "PRINT x,x\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("multiple-unresolved.bas", source_text);
        let document = parse_document(source_id, source_text).expect("syntax is valid");

        let errors =
            resolve_prototype_symbols(&document).expect_err("both PRINT references are unresolved");

        assert_eq!(errors.len(), 2);
        let expected_starts = [6, 8];
        for (error, expected_start) in errors.iter().zip(expected_starts) {
            assert_eq!(error.code, "GFA-SEM-0002");
            assert_eq!(error.span.start, expected_start);
            assert_eq!(sources.slice(error.span), Some("x"));
        }
    }

    #[test]
    fn diagnoses_reassignment_without_creating_a_second_symbol() {
        let source_text = "x=FACT(1)\nx=FACT(2)\nPRINT x\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("duplicate.bas", source_text);
        let document = parse_document(source_id, source_text).expect("syntax is valid");

        let errors = resolve_prototype_symbols(&document).expect_err("reassignment is unsupported");
        assert_eq!(errors[0].code, "GFA-SEM-0001");
        assert_eq!(errors[0].span.start, source_text.find('x').unwrap() + 10);
    }
    #[test]
    fn api_dispatch_resolves_partial_handlers_by_stable_registry_id() {
        match super::resolve_api_handler("gfa.language.math.fact").unwrap() {
            gfabasic_runtime::handlers::ResolvedHandler::IntegerToUnsigned(handler) => {
                assert_eq!(handler(5), Some(120));
            }
            gfabasic_runtime::handlers::ResolvedHandler::TwoIntegersToUnsigned(_) => {
                panic!("FACT must resolve to its unary signature");
            }
        }

        match super::resolve_api_handler("gfa.language.math.variat").unwrap() {
            gfabasic_runtime::handlers::ResolvedHandler::TwoIntegersToUnsigned(handler) => {
                assert_eq!(handler(5, 2), Some(20));
            }
            gfabasic_runtime::handlers::ResolvedHandler::IntegerToUnsigned(_) => {
                panic!("VARIAT must resolve to its binary signature");
            }
        }
    }

    #[test]
    fn api_dispatch_rejects_declarations_without_runtime_implementation() {
        assert_eq!(
            super::resolve_api_handler("gfa.language.matrix.mat-one").unwrap_err(),
            gfabasic_runtime::handlers::RegistryError::UnsupportedDeclaration(
                "gfa.language.matrix.mat-one".to_owned()
            )
        );
    }

    #[test]
    fn resolves_each_repeated_print_occurrence_to_the_same_symbol() {
        let source_text = "x=FACT(1)\nPRINT x,x,x\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("repeated-references.LST", source_text);
        let document = parse_document(source_id, source_text).expect("syntax is valid");
        let symbols = resolve_prototype_symbols(&document).expect("all PRINT references resolve");

        let references: Vec<_> = symbols.references_to(SymbolId(0)).collect();
        assert_eq!(references.len(), 3);

        let print_start = source_text.find("PRINT").expect("PRINT statement exists");
        let expected_starts = [print_start + 6, print_start + 8, print_start + 10];
        for (reference, expected_start) in references.iter().zip(expected_starts) {
            assert_eq!(reference.symbol, SymbolId(0));
            assert_eq!(reference.span.start, expected_start);
            assert_eq!(&source_text[reference.span.start..reference.span.end], "x");
            assert_eq!(sources.slice(reference.span), Some("x"));
        }
    }
}
