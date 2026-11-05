//! Source-preserving formatter for the currently supported frontend subset.
//!
//! The formatter does not parse comments, strings, or other unimplemented
//! constructs. It refuses input the shared parser cannot represent rather than
//! deleting or rewriting unknown source text.

use crate::diagnostics::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::frontend::{ParsedDocument, Statement, parse_document};
use crate::source::{SourceId, SourceSpan};

fn statement_span(statement: &Statement) -> SourceSpan {
    match statement {
        Statement::AssignFact { span, .. }
        | Statement::AssignVariat { span, .. }
        | Statement::AssignCombin { span, .. }
        | Statement::PrintVariables { span, .. } => *span,
    }
}

fn formatting_error(span: SourceSpan, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(
        "GFA-FMT-0001",
        DiagnosticSeverity::Error,
        DiagnosticCategory::Unsupported,
        message,
    )
    .with_primary_span(span)
}

/// Format a successfully parsed document using the prototype canonical style.
///
/// Significant token text is copied verbatim from the original source, so
/// identifier spelling and numeric literal spelling are not normalized.
/// Assignments have no surrounding operator/parenthesis/comma spaces; PRINT
/// uses one space after its keyword. Line endings are emitted as LF.
pub fn format_document(
    document: &ParsedDocument,
    source: &str,
) -> Result<String, Diagnostic> {
    let mut output = String::new();
    let has_final_line_ending = source.ends_with('\n') || source.ends_with('\r');

    for (index, statement) in document.program.statements.iter().enumerate() {
        let span = statement_span(statement);
        let tokens: Vec<_> = document
            .syntax
            .tokens
            .iter()
            .filter(|token| {
                token.span.start >= span.start
                    && token.span.end <= span.end
                    && !token.span.is_empty()
            })
            .collect();

        if tokens.is_empty() {
            return Err(formatting_error(
                span,
                "parsed statement has no significant source tokens",
            ));
        }

        match statement {
            Statement::PrintVariables { .. } => {
                let keyword = source.get(tokens[0].span.start..tokens[0].span.end).ok_or_else(
                    || formatting_error(tokens[0].span, "token span is outside the supplied source"),
                )?;
                output.push_str(keyword);
                output.push(' ');
                for token in tokens.iter().skip(1) {
                    let text = source.get(token.span.start..token.span.end).ok_or_else(|| {
                        formatting_error(token.span, "token span is outside the supplied source")
                    })?;
                    output.push_str(text);
                }
            }
            Statement::AssignFact { .. }
            | Statement::AssignVariat { .. }
            | Statement::AssignCombin { .. } => {
                for token in &tokens {
                    let text = source.get(token.span.start..token.span.end).ok_or_else(|| {
                        formatting_error(token.span, "token span is outside the supplied source")
                    })?;
                    output.push_str(text);
                }
            }
        }

        if index + 1 < document.program.statements.len() || has_final_line_ending {
            output.push('\n');
        }
    }

    Ok(output)
}

/// Parse and format the supported subset with the shared compiler frontend.
///
/// Lexical or syntax errors are returned unchanged. Unknown source constructs
/// are never silently discarded by formatting.
pub fn format_source(
    source_id: SourceId,
    source: &str,
) -> Result<String, Vec<Diagnostic>> {
    let document = parse_document(source_id, source)?;
    format_document(&document, source).map_err(|error| vec![error])
}

#[cfg(test)]
mod tests {
    use super::format_source;
    use crate::source::SourceMap;

    #[test]
    fn formats_spacing_and_line_endings_without_rewriting_tokens() {
        let source_text = "x = FACT ( 05 )\r\nPRINT x , y\r\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("format.bas", source_text);

        let formatted = format_source(source_id, source_text).expect("subset should format");
        assert_eq!(formatted, "x=FACT(05)\nPRINT x,y\n");
    }

    #[test]
    fn formatting_is_idempotent() {
        let source_text = "x=FACT(1)\r\nPRINT x\r\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("idempotent.bas", source_text);

        let once = format_source(source_id, source_text).expect("source should format");
        let twice = format_source(source_id, &once).expect("formatted source should parse");
        assert_eq!(twice, once);
    }

    #[test]
    fn refuses_unrecognized_comment_syntax_instead_of_dropping_it() {
        let source_text = "x=FACT(1) ' keep this comment\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("comment.bas", source_text);

        let errors = format_source(source_id, source_text).expect_err("comments are not supported");
        assert_eq!(errors[0].category.as_str(), "lexical");
        assert_eq!(source_text.len(), errors[0].primary_span.unwrap().end);
    }
}
