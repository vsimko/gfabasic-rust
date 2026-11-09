use gfabasic_compiler::frontend::{Statement, TokenKind, parse, parse_document};
use gfabasic_compiler::source::SourceMap;

#[test]
fn parser_reports_independent_line_errors_after_synchronizing() {
    let source = "x=FACT(0\nLET x=1\nz=FACT(2)\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("parser-recovery.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("malformed and unsupported statements must remain errors");

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].code, "GFA-PARSE-0001");
    assert_eq!(diagnostics[0].category.as_str(), "syntax");
    assert_eq!(
        diagnostics[0]
            .primary_span
            .expect("syntax error has a span")
            .start,
        8,
        "the missing ')' is reported at the first line boundary"
    );

    assert_eq!(diagnostics[1].code, "GFA-SUPPORT-0001");
    assert_eq!(diagnostics[1].category.as_str(), "unsupported");
    assert_eq!(
        diagnostics[1]
            .primary_span
            .expect("unsupported statement has a span")
            .start,
        9,
        "parser recovery must continue at the next source line"
    );
}

#[test]
fn parser_preserves_statement_spans_in_the_original_listing() {
    let source = "x=FACT(0)\nPRINT x\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("statement-spans.LST", source);

    let document = parse_document(source_id, source).expect("supported listing should parse");
    assert_eq!(document.program.statements.len(), 2);

    let first = &document.program.statements[0];
    let first_span = match first {
        Statement::AssignFact { span, .. } => *span,
        other => panic!("expected FACT assignment, got {other:?}"),
    };
    assert_eq!((first_span.start, first_span.end), (0, 9));
    assert_eq!(sources.slice(first_span), Some("x=FACT(0)"));

    let second = &document.program.statements[1];
    let second_span = match second {
        Statement::PrintVariables { span, .. } => *span,
        other => panic!("expected PRINT statement, got {other:?}"),
    };
    assert_eq!((second_span.start, second_span.end), (10, 17));
    assert_eq!(sources.slice(second_span), Some("PRINT x"));

    // The lossless syntax retains token spans in the original source.
    assert!(document.syntax.tokens.iter().any(|token| {
        matches!(&token.kind, TokenKind::Identifier(name) if name == "x")
            && token.span.start == 0
            && token.span.end == 1
    }));
}
