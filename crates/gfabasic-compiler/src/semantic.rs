//! Symbol resolution and semantic analysis boundary.
//!
//! This module currently provides a deliberately narrow symbol index for the
//! evidence-backed FACT/VARIAT/COMBIN/PRINT prototype. It does not define
//! general Amiga GFA BASIC declaration, scope, or type rules.

use std::collections::BTreeMap;

use crate::frontend::{ParsedDocument, Statement, TokenKind};

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
}

/// A reference resolved to one declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedReference {
    /// Identity of the referenced declaration.
    pub symbol: SymbolId,
    /// Source span of the reference identifier.
    pub span: crate::source::SourceSpan,
}

/// Symbol declarations and resolved references for one parsed document.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SymbolTable {
    /// Symbols in source declaration order.
    pub symbols: Vec<Symbol>,
    /// References in source order.
    pub references: Vec<ResolvedReference>,
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
            Statement::AssignFact { name, span, .. }
            | Statement::AssignVariat { name, span, .. }
            | Statement::AssignCombin { name, span, .. } => {
                let declaration = document.syntax.tokens.iter().find_map(|token| {
                    if token.span.start != span.start {
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
            | Statement::MatCopy { span, .. } => errors.push(SemanticError {
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
                        }),
                        None => errors.push(SemanticError {
                            code: "GFA-SEM-0002".to_owned(),
                            message: format!("variable '{name}' is not assigned before PRINT"),
                            span: token.span,
                        }),
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(table)
    } else {
        Err(errors)
    }
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
    fn reports_matrix_semantics_as_not_implemented() {
        let source = include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-one-identity.bas");
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
    fn diagnoses_reassignment_without_creating_a_second_symbol() {
        let source_text = "x=FACT(1)\nx=FACT(2)\nPRINT x\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("duplicate.bas", source_text);
        let document = parse_document(source_id, source_text).expect("syntax is valid");

        let errors = resolve_prototype_symbols(&document).expect_err("reassignment is unsupported");
        assert_eq!(errors[0].code, "GFA-SEM-0001");
        assert_eq!(errors[0].span.start, source_text.find('x').unwrap() + 10);
    }
}
