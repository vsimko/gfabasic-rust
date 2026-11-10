use gfabasic_compiler::frontend::{Statement, TokenKind, parse, parse_document};
use gfabasic_compiler::source::SourceMap;

#[test]
fn parser_reports_independent_line_errors_after_synchronizing() {
    let source = "x=FACT(0\nFOO x=1\nz=FACT(2)\n";
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
fn parser_recovery_keeps_the_next_line_after_missing_print_operand() {
    let source = "PRINT\nFOO x=1\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("missing-print-operand.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("both the malformed PRINT and unsupported next statement are errors");

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].code, "GFA-PARSE-0001");
    assert_eq!(
        diagnostics[0]
            .primary_span
            .expect("syntax error has a span")
            .start,
        5,
        "missing PRINT operand is reported at the line boundary"
    );
    assert_eq!(diagnostics[1].code, "GFA-SUPPORT-0001");
    assert_eq!(
        diagnostics[1]
            .primary_span
            .expect("unsupported statement has a span")
            .start,
        6,
        "the next statement must still be visited after recovery"
    );
}

#[test]
fn parser_recovery_preserves_line_boundaries_for_missing_grammar_tokens() {
    let cases = [
        ("x=\nFOO x=1\n", 2),
        ("DIM\nFOO x=1\n", 3),
        ("MAT\nFOO x=1\n", 3),
    ];

    for (source, boundary) in cases {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("missing-grammar-token.LST", source);
        let diagnostics = parse(source_id, source)
            .expect_err("malformed and unsupported statements must remain errors");

        assert_eq!(diagnostics.len(), 2, "source: {source:?}");
        assert_eq!(diagnostics[0].code, "GFA-PARSE-0001", "source: {source:?}");
        assert_eq!(
            diagnostics[0]
                .primary_span
                .expect("syntax error has a span")
                .start,
            boundary,
            "missing grammar token is reported at the line boundary: {source:?}"
        );
        assert_eq!(
            diagnostics[1].code, "GFA-SUPPORT-0001",
            "source: {source:?}"
        );
        assert_eq!(
            diagnostics[1]
                .primary_span
                .expect("unsupported statement has a span")
                .start,
            source.find("FOO").expect("second statement exists"),
            "the next statement must still be visited: {source:?}"
        );
    }
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

#[test]
fn parser_recovery_keeps_the_next_line_after_trailing_print_comma() {
    let source = "PRINT x,\nFOO y=1\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("trailing-print-comma.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("a trailing comma must not turn an incomplete PRINT into a valid statement");

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].code, "GFA-PARSE-0001");
    assert_eq!(
        diagnostics[0]
            .primary_span
            .expect("syntax error has a span")
            .start,
        8,
        "missing PRINT operand is reported at the line boundary"
    );
    assert_eq!(diagnostics[1].code, "GFA-SUPPORT-0001");
    assert_eq!(
        diagnostics[1]
            .primary_span
            .expect("unsupported statement has a span")
            .start,
        9,
        "recovery must resume at the following source line"
    );
}

#[test]
fn frontend_diagnostics_resolve_to_original_source_locations() {
    let source = "PRINT x,\nFOO y=1\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("diagnostic-locations.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("the incomplete PRINT and following unsupported statement are errors");

    let syntax_span = diagnostics[0]
        .primary_span
        .expect("syntax diagnostic retains its original source span");
    let syntax_file = sources
        .get(syntax_span.source)
        .expect("syntax diagnostic source remains in the source map");
    assert_eq!(syntax_file.name(), "diagnostic-locations.LST");
    assert_eq!(syntax_file.line_column(syntax_span.start), Some((1, 9)));
    assert_eq!(sources.slice(syntax_span), Some("\n"));

    let unsupported_span = diagnostics[1]
        .primary_span
        .expect("unsupported diagnostic retains its original source span");
    let unsupported_file = sources
        .get(unsupported_span.source)
        .expect("unsupported diagnostic source remains in the source map");
    assert_eq!(unsupported_file.name(), "diagnostic-locations.LST");
    assert_eq!(
        unsupported_file.line_column(unsupported_span.start),
        Some((2, 1))
    );
    assert_eq!(sources.slice(unsupported_span), Some("FOO"));
}

#[test]
fn parser_recovery_handles_classic_cr_line_endings() {
    // CR is a complete line ending here, not the CR half of a CRLF pair.
    let source = "x=FACT(0\rFOO x=1\rz=FACT(2)\r";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("parser-recovery-cr.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("malformed and unsupported statements must remain errors");

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].code, "GFA-PARSE-0001");
    let syntax_span = diagnostics[0]
        .primary_span
        .expect("syntax diagnostic has a source span");
    assert_eq!((syntax_span.start, syntax_span.end), (8, 9));
    assert_eq!(sources.slice(syntax_span), Some("\r"));

    assert_eq!(diagnostics[1].code, "GFA-SUPPORT-0001");
    let unsupported_span = diagnostics[1]
        .primary_span
        .expect("unsupported diagnostic has a source span");
    assert_eq!(unsupported_span.start, 9);
    assert_eq!(sources.slice(unsupported_span), Some("FOO"));

    let source_file = sources
        .get(unsupported_span.source)
        .expect("diagnostic source remains registered");
    assert_eq!(
        source_file.line_column(unsupported_span.start),
        Some((2, 1))
    );
}

#[test]
fn parser_rejects_same_line_statements_and_recovers_at_the_next_line() {
    // The current MVP parser deliberately accepts one statement per source
    // line. This guards that implementation boundary; it does not assert
    // that the original Amiga 3.5 interpreter forbids same-line statements.
    let source = "x=FACT(0) y=FACT(1)\nz=FACT(2)\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("same-line-statements.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("a second statement on the same line is outside the current MVP");

    assert_eq!(
        diagnostics.len(),
        1,
        "the following line should still parse"
    );
    assert_eq!(diagnostics[0].code, "GFA-PARSE-0001");
    let span = diagnostics[0]
        .primary_span
        .expect("unexpected same-line token has a source span");
    assert_eq!(&source[span.start..span.end], "y");
    assert_eq!(sources.slice(span), Some("y"));
}

#[test]
fn parser_ignores_and_preserves_inline_comments() {
    use gfabasic_compiler::frontend::{TriviaKind, parse_document};

    let source = "x=FACT(0) ! inline comment\r\nPRINT x ! adjacent comment";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("inline-comments.LST", source);

    let document = parse_document(source_id, source)
        .expect("verified inline comments must not affect supported statements");

    assert_eq!(document.program.statements.len(), 2);
    let comments: Vec<_> = document
        .syntax
        .trivia
        .iter()
        .filter(|trivia| trivia.kind == TriviaKind::Comment)
        .map(|trivia| &source[trivia.span.start..trivia.span.end])
        .collect();
    assert_eq!(comments, ["! inline comment", "! adjacent comment"]);

    let mut spans: Vec<_> = document
        .syntax
        .tokens
        .iter()
        .filter(|token| !matches!(&token.kind, TokenKind::End))
        .map(|token| token.span)
        .chain(document.syntax.trivia.iter().map(|trivia| trivia.span))
        .collect();
    spans.sort_by_key(|span| span.start);

    let reconstructed = spans
        .into_iter()
        .map(|span| &source[span.start..span.end])
        .collect::<String>();
    assert_eq!(reconstructed, source);
}

#[test]
fn exclamation_mark_in_data_line_is_not_an_inline_comment() {
    use gfabasic_compiler::frontend::lex_lossless;

    let source = "DATA 1,2,3 ! this remains DATA text";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("data-comment-marker.LST", source);

    let diagnostics = lex_lossless(source_id, source)
        .expect_err("the current subset must reject DATA syntax, not reinterpret ! as trivia");
    assert_eq!(diagnostics[0].code, "GFA-LEX-0001");
    let span = diagnostics[0]
        .primary_span
        .expect("unsupported DATA text has a lexical span");
    assert_eq!(&source[span.start..span.end], "!");
}

#[test]
fn exclamation_mark_attached_to_identifier_is_not_a_comment() {
    use gfabasic_compiler::frontend::lex_lossless;

    let source = "line!=TRUE";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("boolean-suffix.LST", source);

    let diagnostics = lex_lossless(source_id, source)
        .expect_err("the unsupported boolean suffix must not be reinterpreted as a comment");
    assert_eq!(diagnostics[0].code, "GFA-LEX-0001");
    let span = diagnostics[0]
        .primary_span
        .expect("unsupported suffix has a lexical span");
    assert_eq!(&source[span.start..span.end], "!");
}

#[test]
fn parser_accepts_let_prefix_for_supported_intrinsic_assignments() {
    let source = "LET x=FACT(0)\nLET y=VARIAT(5,2)\nLET z=COMBIN(5,2)\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("let-assignments.LST", source);

    let document = parse_document(source_id, source).expect("documented LET forms should parse");
    assert_eq!(document.program.statements.len(), 3);

    let expected = ["LET x=FACT(0)", "LET y=VARIAT(5,2)", "LET z=COMBIN(5,2)"];
    for (statement, expected_text) in document.program.statements.iter().zip(expected) {
        let span = match statement {
            Statement::AssignFact { span, .. }
            | Statement::AssignVariat { span, .. }
            | Statement::AssignCombin { span, .. } => *span,
            other => panic!("unexpected LET statement shape: {other:?}"),
        };
        assert_eq!(&source[span.start..span.end], expected_text);
    }
}

#[test]
fn parser_rejects_let_assignment_to_variables_outside_the_mvp() {
    let source = "LET count=FACT(0)\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("let-unsupported-variable.LST", source);

    let diagnostics = parse(source_id, source).expect_err("only x, y, and z are supported targets");
    assert_eq!(diagnostics[0].code, "GFA-SUPPORT-0001");
    let span = diagnostics[0]
        .primary_span
        .expect("unsupported variable has a source span");
    assert_eq!(&source[span.start..span.end], "count");
}

#[test]
fn parser_rejects_let_rhs_outside_the_supported_intrinsic_subset() {
    let source = "LET x=RND(1)\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("let-unsupported-intrinsic.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("the current subset accepts only the documented intrinsic-call RHS");
    assert_eq!(diagnostics[0].code, "GFA-SUPPORT-0001");
    let span = diagnostics[0]
        .primary_span
        .expect("unsupported LET intrinsic has a source span");
    assert_eq!(&source[span.start..span.end], "RND");
}

#[test]
fn parser_accepts_integer_literal_assignments_with_or_without_let() {
    let source = "x=42\nLET y=7\nPRINT x,y\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("integer-assignments.LST", source);

    let document =
        parse_document(source_id, source).expect("documented integer assignments should parse");
    assert_eq!(document.program.statements.len(), 3);

    let expected = [("x", 42, "x=42"), ("y", 7, "LET y=7")];
    for (statement, (expected_name, expected_value, expected_text)) in
        document.program.statements.iter().zip(expected)
    {
        let (name, value, span) = match statement {
            Statement::AssignInteger { name, value, span } => (name, value, *span),
            other => panic!("expected integer assignment, got {other:?}"),
        };
        assert_eq!(name, expected_name);
        assert_eq!(*value, expected_value);
        assert_eq!(&source[span.start..span.end], expected_text);
    }
}

#[test]
fn parser_rejects_arithmetic_rhs_until_expression_grammar_is_implemented() {
    let source = "x=1+2\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("integer-expression.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("the integer-literal subset must not accept an incomplete expression grammar");
    assert_eq!(diagnostics[0].code, "GFA-PARSE-0001");
    let span = diagnostics[0]
        .primary_span
        .expect("unsupported arithmetic tail has a syntax span");
    assert_eq!(&source[span.start..span.end], "+");
}

#[test]
fn parser_handles_the_integer_literal_i64_boundary() {
    let valid_source = "x=9223372036854775807\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("integer-literal-max.LST", valid_source);
    let document = parse_document(source_id, valid_source)
        .expect("the largest supported signed integer literal should parse");

    let statement = document
        .program
        .statements
        .first()
        .expect("one integer assignment is parsed");
    match statement {
        Statement::AssignInteger { value, span, .. } => {
            assert_eq!(*value, i64::MAX);
            assert_eq!(&valid_source[span.start..span.end], "x=9223372036854775807");
        }
        other => panic!("expected integer assignment, got {other:?}"),
    }

    let overflow_source = "x=9223372036854775808\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("integer-literal-overflow.LST", overflow_source);
    let diagnostics = parse(source_id, overflow_source)
        .expect_err("an integer outside the frontend's i64 representation must be rejected");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code, "GFA-LEX-0002");
    assert_eq!(diagnostics[0].category.as_str(), "lexical");
    let span = diagnostics[0]
        .primary_span
        .expect("integer range diagnostic retains its source span");
    assert_eq!((span.start, span.end), (2, 21));
    assert_eq!(
        &overflow_source[span.start..span.end],
        "9223372036854775808"
    );
}

#[test]
fn parser_preserves_ast_spans_across_crlf_line_endings() {
    let source = "x=FACT(0)\r\nPRINT x\r\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("statement-spans-crlf.LST", source);

    let document =
        parse_document(source_id, source).expect("supported statements parse with CRLF separators");
    assert_eq!(document.program.statements.len(), 2);

    let first_span = match &document.program.statements[0] {
        Statement::AssignFact { span, .. } => *span,
        other => panic!("expected FACT assignment, got {other:?}"),
    };
    assert_eq!((first_span.start, first_span.end), (0, 9));
    assert_eq!(sources.slice(first_span), Some("x=FACT(0)"));

    let second_span = match &document.program.statements[1] {
        Statement::PrintVariables { span, .. } => *span,
        other => panic!("expected PRINT statement, got {other:?}"),
    };
    assert_eq!((second_span.start, second_span.end), (11, 18));
    assert_eq!(sources.slice(second_span), Some("PRINT x"));

    let print_token = document
        .syntax
        .tokens
        .iter()
        .find(|token| matches!(&token.kind, TokenKind::Identifier(name) if name == "PRINT"))
        .expect("PRINT token is retained in the lossless syntax");
    assert_eq!((print_token.span.start, print_token.span.end), (11, 16));
    assert_eq!(sources.slice(print_token.span), Some("PRINT"));
}

#[test]
fn parser_rejects_signed_integer_rhs_and_recovers_at_the_next_line() {
    // The current MVP accepts only unsigned decimal integer tokens as direct
    // assignment values. This guards the implementation boundary; it does not
    // claim that signed literals are invalid in the Amiga dialect.
    let source = "x=-1\nz=42\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("signed-integer-rhs.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("a signed expression is outside the current integer-literal subset");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code, "GFA-PARSE-0001");
    let span = diagnostics[0]
        .primary_span
        .expect("unsupported signed RHS has a source span");
    assert_eq!(&source[span.start..span.end], "-");
    assert_eq!(sources.slice(span), Some("-"));
}
