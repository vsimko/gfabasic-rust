//! Conservative, lossless-source-aware formatter for the supported frontend subset.
//!
//! Formatting is intentionally limited to syntax accepted by `parse_document`.
//! Token spellings and inline-comment text are copied from their original source spans;
//! the formatter never case-folds or rewrites identifiers or numeric literal text.

use crate::diagnostics::Diagnostic;
use crate::frontend::{TokenKind, TriviaKind, parse_document};
use crate::source::SourceId;

/// Format the currently supported Amiga GFA BASIC 3.5 source subset.
///
/// The formatter validates the complete input with the lossless parser before
/// producing output. Inline comments are retained verbatim; unsupported
/// string/DATA payloads return diagnostics instead of partially formatted text.
///
/// The canonical prototype layout uses one space around assignment, one space
/// after commas, no spaces inside intrinsic-call parentheses, LF line endings,
/// one statement per line, and exactly one final newline. Original token and
/// comment text is retained verbatim. This is a project style policy for the
/// implemented subset, not a claim about historical interpreter formatting.
pub fn format_source(source_id: SourceId, source: &str) -> Result<String, Vec<Diagnostic>> {
    let document = parse_document(source_id, source)?;
    let mut output = String::with_capacity(source.len() + 1);
    let mut previous: Option<&TokenKind> = None;
    let mut line_has_tokens = false;
    let mut comments = document
        .syntax
        .trivia
        .iter()
        .filter(|trivia| trivia.kind == TriviaKind::Comment)
        .peekable();

    for token in &document.syntax.tokens {
        while comments
            .peek()
            .is_some_and(|comment| comment.span.start < token.span.start)
        {
            let comment = comments.next().expect("peeked comment exists");
            if line_has_tokens {
                output.push(' ');
            }
            output.push_str(
                source
                    .get(comment.span.start..comment.span.end)
                    .expect("comment trivia span is a valid source range"),
            );
        }

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
        (_, TokenKind::Equals)
        | (TokenKind::Equals, _)
        | (_, TokenKind::Plus)
        | (TokenKind::Plus, _)
        | (_, TokenKind::Minus)
        | (TokenKind::Minus, _)
        | (_, TokenKind::Star)
        | (TokenKind::Star, _) => true,
        (_, TokenKind::Hash) => true,
        (TokenKind::Comma, _) => true,
        (_, TokenKind::Comma)
        | (_, TokenKind::LeftParen)
        | (TokenKind::LeftParen, _)
        | (_, TokenKind::RightParen) => false,
        (TokenKind::Identifier(_), TokenKind::Identifier(_))
        | (TokenKind::Identifier(_), TokenKind::Integer(_)) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::format_source;
    use crate::frontend::{Program, Statement, parse_document};
    use crate::source::SourceMap;

    #[derive(Debug, PartialEq)]
    enum Meaning {
        Integer(String, i64),
        Fact(String, i64),
        Variat(String, i64, i64),
        Combin(String, i64, i64),
        Print(Vec<String>),
        DimMatrix(String, i64, Option<i64>),
        MatOne(String),
        MatSet(String, i64),
        MatClr(String),
        MatBase(i64),
        MatTrans(String, Option<String>),
        MatNorm(String, i64),
        MatArithmetic(String, Option<String>, String, String),
        MatMulMatrix(String, String, String),
        MatMulScalar(String, Vec<String>),
        MatMulScale(String, String),
        MatAnalysis(String, String, Option<(i64, i64)>, Option<i64>, String),
        MatCopy(
            String,
            Option<(i64, i64)>,
            String,
            Option<(i64, i64)>,
            Option<(i64, i64)>,
            String,
        ),
        MatRead(String),
        MatInput(i64, String),
        MatPrint(String, Option<i64>, Option<(i64, i64)>),
    }

    fn meanings(program: &Program) -> Vec<Meaning> {
        program
            .statements
            .iter()
            .map(|statement| match statement {
                Statement::AssignInteger { name, value, .. } => {
                    Meaning::Integer(name.clone(), *value)
                }
                Statement::AssignFact { name, argument, .. } => {
                    Meaning::Fact(name.clone(), *argument)
                }
                Statement::AssignVariat { name, n, k, .. } => Meaning::Variat(name.clone(), *n, *k),
                Statement::AssignCombin { name, n, k, .. } => Meaning::Combin(name.clone(), *n, *k),
                Statement::PrintVariables { names, .. } => Meaning::Print(names.clone()),
                Statement::DimMatrix {
                    name,
                    rows,
                    columns,
                    ..
                } => Meaning::DimMatrix(name.clone(), *rows, *columns),
                Statement::MatOne { name, .. } => Meaning::MatOne(name.clone()),
                Statement::MatSet { name, value, .. } => Meaning::MatSet(name.clone(), *value),
                Statement::MatClr { name, .. } => Meaning::MatClr(name.clone()),
                Statement::MatBase { index, .. } => Meaning::MatBase(*index),
                Statement::MatTrans {
                    destination,
                    source,
                    ..
                } => Meaning::MatTrans(destination.clone(), source.clone()),
                Statement::MatNorm { name, mode, .. } => Meaning::MatNorm(name.clone(), *mode),
                Statement::MatMulMatrix {
                    destination,
                    left,
                    right,
                    ..
                } => Meaning::MatMulMatrix(destination.clone(), left.clone(), right.clone()),
                Statement::MatMulScalar {
                    destination,
                    operands,
                    ..
                } => Meaning::MatMulScalar(destination.clone(), operands.clone()),
                Statement::MatMulScale {
                    destination,
                    scalar,
                    ..
                } => Meaning::MatMulScale(destination.clone(), scalar.clone()),
                Statement::MatRead { matrix, .. } => Meaning::MatRead(matrix.clone()),
                Statement::MatPrint {
                    name,
                    channel,
                    format,
                    ..
                } => Meaning::MatPrint(name.clone(), *channel, *format),
                Statement::MatInput {
                    channel, matrix, ..
                } => Meaning::MatInput(*channel, matrix.clone()),
                Statement::MatCopy {
                    operation,
                    destination,
                    destination_offset,
                    source,
                    source_offset,
                    extent,
                    ..
                } => {
                    let operation = match operation {
                        crate::frontend::MatCopyOperation::Copy => "cpy",
                        crate::frontend::MatCopyOperation::TransposedCopy => "xcpy",
                    };
                    Meaning::MatCopy(
                        destination.clone(),
                        *destination_offset,
                        source.clone(),
                        *source_offset,
                        *extent,
                        operation.to_owned(),
                    )
                }
                Statement::MatAnalysis {
                    operation,
                    destination,
                    matrix,
                    offsets,
                    order,
                    ..
                } => {
                    let operation = match operation {
                        crate::frontend::MatAnalysisOperation::Det => "det",
                        crate::frontend::MatAnalysisOperation::QDet => "qdet",
                        crate::frontend::MatAnalysisOperation::Rang => "rang",
                    };
                    Meaning::MatAnalysis(
                        destination.clone(),
                        matrix.clone(),
                        *offsets,
                        *order,
                        operation.to_owned(),
                    )
                }
                Statement::MatArithmetic {
                    operation,
                    destination,
                    left,
                    right,
                    ..
                } => {
                    let (operand, operand_kind) = match right {
                        crate::frontend::MatArithmeticOperand::Matrix(name) => {
                            (name.clone(), "matrix")
                        }
                        crate::frontend::MatArithmeticOperand::Scalar(name) => {
                            (name.clone(), "scalar")
                        }
                    };
                    let operation = match operation {
                        crate::frontend::MatArithmeticOperation::Add => "add",
                        crate::frontend::MatArithmeticOperation::Sub => "sub",
                    };
                    Meaning::MatArithmetic(
                        destination.clone(),
                        left.clone(),
                        operand,
                        format!("{operation}:{operand_kind}"),
                    )
                }
            })
            .collect()
    }

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
    fn golden_cases_cover_each_supported_statement() {
        let cases = [
            ("x=FACT(0001)", "x = FACT(0001)\n"),
            ("y=VARIAT(5,2)", "y = VARIAT(5, 2)\n"),
            ("z=COMBIN(5,2)", "z = COMBIN(5, 2)\n"),
            ("PRINT   x,y,z", "PRINT x, y, z\n"),
        ];

        for (source, expected) in cases {
            assert_eq!(format(source), expected, "source: {source:?}");
        }
    }

    #[test]
    fn formatting_is_idempotent_across_supported_corpus() {
        let cases = [
            "x=FACT(1)\r\nPRINT   x\r\n",
            "y = VARIAT(5, 2)\n",
            "z=COMBIN(0005,02)",
            "\n\nx=FACT(3)\n\nPRINT x,y,z\n",
            "x=FACT(1)\ny=VARIAT(5,2)\nz=COMBIN(5,2)\nPRINT x,y,z\n",
        ];

        for source in cases {
            let once = format(source);
            assert_eq!(format(&once), once, "source: {source:?}");
        }
    }

    #[test]
    fn verified_amiga_35_fixtures_are_idempotent_and_semantics_preserving() {
        let fixtures = [
            include_str!("../../../tests/corpus/amiga/3.5/combinatorics/fact-zero.lst"),
            include_str!("../../../tests/corpus/amiga/3.5/combinatorics/three-intrinsics.lst"),
        ];

        for source in fixtures {
            let formatted = format(source);
            assert_eq!(
                format(&formatted),
                formatted,
                "verified fixture is not idempotent: {source:?}"
            );

            let mut before_sources = SourceMap::new();
            let before_id = before_sources.add_source("fixture-before.bas", source);
            let before = parse_document(before_id, source).unwrap();
            let mut after_sources = SourceMap::new();
            let after_id = after_sources.add_source("fixture-after.bas", formatted.as_str());
            let after = parse_document(after_id, &formatted).unwrap();

            assert_eq!(
                meanings(&before.program),
                meanings(&after.program),
                "formatting changed verified fixture meaning: {source:?}"
            );
        }
    }

    #[test]
    fn formatting_preserves_parsed_program_meaning() {
        let cases = [
            "x=FACT(2)\n",
            "y=VARIAT(5,2)\n",
            "z=COMBIN(5,2)\n",
            "PRINT x,y,z\n",
            "x=FACT(2)\ny=VARIAT(5,2)\nz=COMBIN(5,2)\nPRINT x,y,z\n",
        ];

        for source in cases {
            let formatted = format(source);
            let mut before_sources = SourceMap::new();
            let before_id = before_sources.add_source("before.bas", source);
            let before = parse_document(before_id, source).unwrap();
            let mut after_sources = SourceMap::new();
            let after_id = after_sources.add_source("after.bas", formatted.as_str());
            let after = parse_document(after_id, &formatted).unwrap();

            assert_eq!(
                meanings(&before.program),
                meanings(&after.program),
                "source: {source:?}"
            );
        }
    }

    #[test]
    fn golden_cases_cover_verified_matrix_statements() {
        let cases = [
            ("DIM a(3,3)", "DIM a(3, 3)\n"),
            ("MAT ONE a()", "MAT ONE a()\n"),
            ("MAT SET a()=7", "MAT SET a() = 7\n"),
            ("MAT CLR a()", "MAT CLR a()\n"),
            ("MAT PRINT a()", "MAT PRINT a()\n"),
        ];

        for (source, expected) in cases {
            assert_eq!(format(source), expected, "source: {source:?}");
        }
    }

    #[test]
    fn golden_cases_cover_verified_mat_base_and_transpose_forms() {
        let cases = [
            ("MAT BASE 0", "MAT BASE 0\n"),
            ("MAT BASE 1", "MAT BASE 1\n"),
            ("MAT TRANS a()", "MAT TRANS a()\n"),
            ("MAT TRANS a()=b()", "MAT TRANS a() = b()\n"),
        ];

        for (source, expected) in cases {
            assert_eq!(format(source), expected, "source: {source:?}");
        }
    }

    #[test]
    fn mat_base_and_transpose_formatting_is_idempotent_and_semantics_preserving() {
        let cases = [
            "MAT BASE 0\nMAT TRANS a()\n",
            "MAT BASE 1\nMAT TRANS a()=b()\n",
        ];

        for source in cases {
            let formatted = format(source);
            assert_eq!(format(&formatted), formatted);

            let mut before_sources = SourceMap::new();
            let before_id = before_sources.add_source("before.bas", source);
            let before = parse_document(before_id, source).unwrap();
            let mut after_sources = SourceMap::new();
            let after_id = after_sources.add_source("after.bas", formatted.as_str());
            let after = parse_document(after_id, &formatted).unwrap();
            assert_eq!(meanings(&before.program), meanings(&after.program));
        }
    }

    #[test]
    fn golden_cases_cover_verified_mat_norm_modes() {
        let cases = [
            ("MAT NORM a(),0", "MAT NORM a(), 0\n"),
            ("MAT NORM a(),1", "MAT NORM a(), 1\n"),
            ("MAT ADD a()=b()+a()", "MAT ADD a() = b() + a()\n"),
            ("MAT SUB a()=b()-a()", "MAT SUB a() = b() - a()\n"),
            ("MAT ADD a(),b()", "MAT ADD a(), b()\n"),
            ("MAT SUB a(),x", "MAT SUB a(), x\n"),
            ("MAT MUL a()=b()*a()", "MAT MUL a() = b() * a()\n"),
            ("MAT MUL x=a()*b()", "MAT MUL x = a() * b()\n"),
            ("MAT MUL y=a()*b()*a()", "MAT MUL y = a() * b() * a()\n"),
            ("MAT MUL a(),x", "MAT MUL a(), x\n"),
            ("MAT DET x=a()", "MAT DET x = a()\n"),
            ("MAT DET x=a(1,2),3", "MAT DET x = a(1, 2), 3\n"),
            ("MAT QDET x=b(0,0)", "MAT QDET x = b(0, 0)\n"),
            ("MAT RANG y=a(2,1),4", "MAT RANG y = a(2, 1), 4\n"),
            ("MAT CPY a()=b()", "MAT CPY a() = b()\n"),
            (
                "MAT XCPY a(1,2)=b(3,4),5,6",
                "MAT XCPY a(1, 2) = b(3, 4), 5, 6\n",
            ),
            ("MAT READ a()", "MAT READ a()\n"),
            ("MAT INPUT #1,a()", "MAT INPUT #1, a()\n"),
            ("DIM a(5)", "DIM a(5)\n"),
            ("DIM a(1,5)", "DIM a(1, 5)\n"),
            ("DIM b(5,1)", "DIM b(5, 1)\n"),
            ("MAT PRINT a()", "MAT PRINT a()\n"),
            ("MAT PRINT #1,a()", "MAT PRINT #1, a()\n"),
            ("MAT PRINT a(),7,3", "MAT PRINT a(), 7, 3\n"),
            ("MAT PRINT #2,b(),8,4", "MAT PRINT #2, b(), 8, 4\n"),
        ];

        for (source, expected) in cases {
            assert_eq!(format(source), expected, "source: {source:?}");
        }
    }

    #[test]
    fn mat_print_options_formatting_is_idempotent_and_semantics_preserving() {
        let source = "MAT PRINT a()\nMAT PRINT #1,a()\nMAT PRINT a(),7,3\nMAT PRINT #2,b(),8,4\n";
        let formatted = format(source);
        assert_eq!(format(&formatted), formatted);
        let mut before_sources = SourceMap::new();
        let before_id = before_sources.add_source("before.bas", source);
        let before = parse_document(before_id, source).unwrap();
        let mut after_sources = SourceMap::new();
        let after_id = after_sources.add_source("after.bas", formatted.as_str());
        let after = parse_document(after_id, &formatted).unwrap();
        assert_eq!(meanings(&before.program), meanings(&after.program));
    }

    #[test]
    fn mat_read_input_formatting_is_idempotent_and_semantics_preserving() {
        let source = "MAT READ a()\nMAT INPUT #1,a()\n";
        let formatted = format(source);
        assert_eq!(format(&formatted), formatted);
        let mut before_sources = SourceMap::new();
        let before_id = before_sources.add_source("before.bas", source);
        let before = parse_document(before_id, source).unwrap();
        let mut after_sources = SourceMap::new();
        let after_id = after_sources.add_source("after.bas", formatted.as_str());
        let after = parse_document(after_id, &formatted).unwrap();
        assert_eq!(meanings(&before.program), meanings(&after.program));
    }

    #[test]
    fn mat_copy_formatting_is_idempotent_and_semantics_preserving() {
        let source = "MAT CPY a()=b()\nMAT XCPY a(1,2)=b(3,4),5,6\n";
        let formatted = format(source);
        assert_eq!(format(&formatted), formatted);
        let mut before_sources = SourceMap::new();
        let before_id = before_sources.add_source("before.bas", source);
        let before = parse_document(before_id, source).unwrap();
        let mut after_sources = SourceMap::new();
        let after_id = after_sources.add_source("after.bas", formatted.as_str());
        let after = parse_document(after_id, &formatted).unwrap();
        assert_eq!(meanings(&before.program), meanings(&after.program));
    }

    #[test]
    fn mat_analysis_formatting_is_idempotent_and_semantics_preserving() {
        let source = "MAT DET x=a()\nMAT QDET x=b(0,0)\nMAT RANG y=a(2,1),4\n";
        let formatted = format(source);
        assert_eq!(format(&formatted), formatted);
        let mut before_sources = SourceMap::new();
        let before_id = before_sources.add_source("before.bas", source);
        let before = parse_document(before_id, source).unwrap();
        let mut after_sources = SourceMap::new();
        let after_id = after_sources.add_source("after.bas", formatted.as_str());
        let after = parse_document(after_id, &formatted).unwrap();
        assert_eq!(meanings(&before.program), meanings(&after.program));
    }

    #[test]
    fn mat_mul_formatting_is_idempotent_and_semantics_preserving() {
        let source =
            "MAT MUL a()=b()*a()\nMAT MUL x=a()*b()\nMAT MUL y=a()*b()*a()\nMAT MUL a(),x\n";
        let formatted = format(source);
        assert_eq!(format(&formatted), formatted);
        let mut before_sources = SourceMap::new();
        let before_id = before_sources.add_source("before.bas", source);
        let before = parse_document(before_id, source).unwrap();
        let mut after_sources = SourceMap::new();
        let after_id = after_sources.add_source("after.bas", formatted.as_str());
        let after = parse_document(after_id, &formatted).unwrap();
        assert_eq!(meanings(&before.program), meanings(&after.program));
    }

    #[test]
    fn mat_arithmetic_formatting_is_idempotent_and_semantics_preserving() {
        let source = "MAT ADD a()=b()+a()\nMAT SUB a(),x\n";
        let formatted = format(source);
        assert_eq!(format(&formatted), formatted);
        let mut before_sources = SourceMap::new();
        let before_id = before_sources.add_source("before.bas", source);
        let before = parse_document(before_id, source).unwrap();
        let mut after_sources = SourceMap::new();
        let after_id = after_sources.add_source("after.bas", formatted.as_str());
        let after = parse_document(after_id, &formatted).unwrap();
        assert_eq!(meanings(&before.program), meanings(&after.program));
    }

    #[test]
    fn mat_norm_formatting_is_idempotent_and_semantics_preserving() {
        let source = "MAT NORM a(),0\nMAT NORM a(),1\n";
        let formatted = format(source);
        assert_eq!(format(&formatted), formatted);

        let mut before_sources = SourceMap::new();
        let before_id = before_sources.add_source("before.bas", source);
        let before = parse_document(before_id, source).unwrap();
        let mut after_sources = SourceMap::new();
        let after_id = after_sources.add_source("after.bas", formatted.as_str());
        let after = parse_document(after_id, &formatted).unwrap();
        assert_eq!(meanings(&before.program), meanings(&after.program));
    }

    #[test]
    fn verified_amiga_35_matrix_fixtures_are_idempotent_and_semantics_preserving() {
        let fixtures = [
            include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-one-identity.lst"),
            include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-clear-zero.lst"),
            include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-set-scalar.lst"),
            include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-base-transpose.lst"),
            include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-norm-row.lst"),
        ];

        for source in fixtures {
            let formatted = format(source);
            assert_eq!(format(&formatted), formatted, "not idempotent: {source:?}");

            let mut before_sources = SourceMap::new();
            let before_id = before_sources.add_source("matrix-before.bas", source);
            let before = parse_document(before_id, source).unwrap();
            let mut after_sources = SourceMap::new();
            let after_id = after_sources.add_source("matrix-after.bas", formatted.as_str());
            let after = parse_document(after_id, &formatted).unwrap();
            assert_eq!(
                meanings(&before.program),
                meanings(&after.program),
                "formatting changed matrix fixture meaning: {source:?}"
            );
        }
    }

    #[test]
    fn unsupported_syntax_is_rejected_without_output() {
        let cases = [
            "PRINT \"not supported yet\"\n",
            "REM comment\n",
            "' apostrophe comment\n",
            "DATA 1,2,3\n",
        ];

        for source in cases {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("unsupported.bas", source);
            let errors = format_source(source_id, source)
                .expect_err("syntax outside the verified formatter subset must be rejected");

            assert!(
                !errors.is_empty(),
                "source unexpectedly accepted: {source:?}"
            );
        }
    }

    #[test]
    fn integer_literal_assignments_are_formatted_idempotently() {
        let source = "x=42\\nLET y=7\\n";
        let formatted = format(source);
        assert_eq!(formatted, "x = 42\\nLET y = 7\\n");
        assert_eq!(format(&formatted), formatted);
    }

    #[test]
    fn let_assignment_prefix_is_preserved_by_formatting() {
        let source = "LET x=FACT(1)\n";
        let formatted = format(source);
        assert_eq!(formatted, "LET x = FACT(1)\n");

        let mut before_sources = SourceMap::new();
        let before_id = before_sources.add_source("before-let.bas", source);
        let before = parse_document(before_id, source).unwrap();
        let mut after_sources = SourceMap::new();
        let after_id = after_sources.add_source("after-let.bas", formatted.as_str());
        let after = parse_document(after_id, &formatted).unwrap();
        assert_eq!(meanings(&before.program), meanings(&after.program));
        assert_eq!(format(&formatted), formatted);
    }

    #[test]
    fn inline_comments_are_preserved_and_formatting_is_idempotent() {
        let source = "x=FACT(1)  ! keep  exact text\r\nPRINT x ! second comment\n";
        let formatted = format(source);

        assert_eq!(
            formatted,
            "x = FACT(1) ! keep  exact text\nPRINT x ! second comment\n"
        );
        assert_eq!(format(&formatted), formatted);

        let mut sources = SourceMap::new();
        let source_id = sources.add_source("inline-comments.bas", formatted.as_str());
        let document = parse_document(source_id, &formatted).expect("formatted comments parse");
        let comments: Vec<_> = document
            .syntax
            .trivia
            .iter()
            .filter(|trivia| trivia.kind == crate::frontend::TriviaKind::Comment)
            .map(|trivia| &formatted[trivia.span.start..trivia.span.end])
            .collect();
        assert_eq!(comments, ["! keep  exact text", "! second comment"]);
    }

    #[test]
    fn empty_lines_collapse_and_output_has_one_final_newline() {
        assert_eq!(format("\n\nx=FACT(1)\n\n\n"), "x = FACT(1)\n");
    }
}
