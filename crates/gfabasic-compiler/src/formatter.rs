//! Conservative, lossless-source-aware formatter for the supported frontend subset.
//!
//! Formatting is intentionally limited to syntax accepted by `parse_document`.
//! Token spellings are copied from their original source spans; the formatter
//! never case-folds or rewrites identifiers or numeric literal text.

use crate::diagnostics::Diagnostic;
use crate::frontend::{TokenKind, parse_document};
use crate::source::SourceId;

/// Format the currently supported Amiga GFA BASIC 3.5 source subset.
///
/// The formatter validates the complete input with the lossless parser before
/// producing output. Unsupported syntax (including comments and string/DATA
/// payloads until their lexical rules are verified) returns diagnostics and
/// never yields partially formatted text.
///
/// The canonical prototype layout uses one space around assignment, one space
/// after commas, no spaces inside intrinsic-call parentheses, LF line endings,
/// one statement per line, and exactly one final newline. Original token text
/// is retained verbatim, including identifier spelling and numeric literal
/// spelling. This is a project style policy for the implemented subset, not a
/// claim about historical interpreter formatting.
pub fn format_source(source_id: SourceId, source: &str) -> Result<String, Vec<Diagnostic>> {
    let document = parse_document(source_id, source)?;
    let mut output = String::with_capacity(source.len() + 1);
    let mut previous: Option<&TokenKind> = None;
    let mut line_has_tokens = false;

    for token in &document.syntax.tokens {
        match &token.kind {
            TokenKind::End => break,
            TokenKind::Newline => {
                if line_has_tokens {
                    output.push('\n');
                    line_has_tokens = false;
                }
                previous = None;
            }
            kind => {
                if let Some(previous_kind) = previous
                    && needs_space(previous_kind, kind)
                {
                    output.push(' ');
                }
                let spelling = source
                    .get(token.span.start..token.span.end)
                    .expect("lexer token spans are valid UTF-8 source ranges");
                output.push_str(spelling);
                previous = Some(kind);
                line_has_tokens = true;
            }
        }
    }

    if line_has_tokens {
        output.push('\n');
    }

    Ok(output)
}

fn needs_space(previous: &TokenKind, current: &TokenKind) -> bool {
    match (previous, current) {
        (_, TokenKind::Equals) | (TokenKind::Equals, _) => true,
        (TokenKind::Comma, _) => true,
        (_, TokenKind::Comma)
        | (_, TokenKind::LeftParen)
        | (TokenKind::LeftParen, _)
        | (_, TokenKind::RightParen) => false,
        (TokenKind::Identifier(_), TokenKind::Identifier(_)) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::format_source;
    use crate::frontend::parse_document;
    use crate::source::SourceMap;

    fn format(source: &str) -> String {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("format.bas", source);
        format_source(source_id, source).expect("supported source should format")
    }

    #[test]
    fn canonicalizes_supported_whitespace_and_line_endings() {
        let source = "  x=FACT(0001)\r\ny = VARIAT(5,2)\nPRINT   x,y,z";
        assert_eq!(
            format(source),
            "x = FACT(0001)\ny = VARIAT(5, 2)\nPRINT x, y, z\n"
        );
    }

    #[test]
    fn preserves_identifier_and_intrinsic_spelling_exactly() {
        let source = "x=FACT(1)\nPRINT x\n";
        assert_eq!(format(source), "x = FACT(1)\nPRINT x\n");
    }

    #[test]
    fn formatting_is_idempotent() {
        let once = format("x=FACT(1)\r\nPRINT   x\r\n");
        assert_eq!(format(&once), once);
    }

    #[test]
    fn formatting_preserves_parsed_program_meaning() {
        let source = "x=FACT(2)\ny=VARIAT(5,2)\nz=COMBIN(5,2)\nPRINT x,y,z\n";
        let formatted = format(source);
        let mut before_sources = SourceMap::new();
        let before_id = before_sources.add_source("before.bas", source);
        let before = parse_document(before_id, source).unwrap();
        let mut after_sources = SourceMap::new();
        let after_id = after_sources.add_source("after.bas", formatted.as_str());
        let after = parse_document(after_id, &formatted).unwrap();

        assert_eq!(before.program, after.program);
    }

    #[test]
    fn unsupported_syntax_is_rejected_without_output() {
        let source = "PRINT \"not supported yet\"\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("unsupported.bas", source);
        let errors = format_source(source_id, source).expect_err("strings are not lexed yet");

        assert!(!errors.is_empty());
    }

    #[test]
    fn empty_lines_collapse_and_output_has_one_final_newline() {
        assert_eq!(format("\n\nx=FACT(1)\n\n\n"), "x = FACT(1)\n");
    }
}
