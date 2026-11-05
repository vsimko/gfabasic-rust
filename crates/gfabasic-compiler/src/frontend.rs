//! Evidence-gated frontend for the first Amiga GFA BASIC 3.5 vertical slice.
//!
//! This is deliberately not a general GFA BASIC lexer/parser. It recognizes
//! only the source forms demonstrated by the 3.5 addendum's FACT example:
//! assignments to x/y/z using FACT(integer-literal), followed by PRINT x,y,z.
//! Other syntax is rejected rather than guessed.

use crate::diagnostics::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::source::{SourceId, SourceSpan};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    AssignFact {
        name: String,
        argument: i64,
        span: SourceSpan,
    },
    AssignVariat {
        name: String,
        n: i64,
        k: i64,
        span: SourceSpan,
    },
    AssignCombin {
        name: String,
        n: i64,
        k: i64,
        span: SourceSpan,
    },
    PrintVariables {
        names: Vec<String>,
        span: SourceSpan,
    },
}

/// A significant token kind recognized by the current evidence-gated lexer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    /// An identifier candidate with its exact source spelling.
    Identifier(String),
    /// A decimal integer literal in the current supported subset.
    Integer(i64),
    /// The assignment operator.
    Equals,
    /// An opening parenthesis.
    LeftParen,
    /// A closing parenthesis.
    RightParen,
    /// An argument or print-list separator.
    Comma,
    /// A source line boundary, preserving CR, LF, or CRLF in its span.
    Newline,
    /// A zero-width end-of-input marker.
    End,
}

/// A significant token and its original source span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    /// The interpreted token kind; exact spelling remains in the source span.
    pub kind: TokenKind,
    /// Half-open UTF-8 byte range in the original source.
    pub span: SourceSpan,
}

/// Non-semantic source text retained between significant tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TriviaKind {
    /// Horizontal ASCII spaces and tabs. Their spelling remains in the source span.
    HorizontalWhitespace,
}

/// A lossless span of source trivia.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Trivia {
    /// The recognized trivia category.
    pub kind: TriviaKind,
    /// Half-open UTF-8 byte range in the original source.
    pub span: SourceSpan,
}

/// Token and trivia spans for the currently implemented lexical subset.
///
/// The original source remains authoritative; spans refer to its UTF-8 bytes.
/// Unsupported syntax still produces diagnostics rather than being guessed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LexedSource {
    /// Significant tokens, including line boundaries and the end marker.
    pub tokens: Vec<Token>,
    /// Non-semantic source spans between significant tokens.
    pub trivia: Vec<Trivia>,
}

fn diagnostic(
    code: &str,
    category: DiagnosticCategory,
    message: impl Into<String>,
    span: SourceSpan,
) -> Diagnostic {
    Diagnostic::new(code, DiagnosticSeverity::Error, category, message).with_primary_span(span)
}

// CR, CRLF, and LF are accepted here only as transport-level separators for
// this prototype. This is not a claim about the original Amiga 3.5 source
// encoding/line-ending contract; that remains subject to #66 observation.
/// Lexes the currently supported source subset without discarding recognized trivia.
///
/// Token and trivia spans partition all successfully recognized source bytes.
/// Unknown syntax is reported as a diagnostic instead of being normalized.
pub fn lex_lossless(source_id: SourceId, source: &str) -> Result<LexedSource, Vec<Diagnostic>> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut trivia = Vec::new();
    let mut diagnostics = Vec::new();
    let mut offset = 0;

    while offset < bytes.len() {
        match bytes[offset] {
            b' ' | b'\t' => {
                let start = offset;
                while bytes.get(offset).is_some_and(|byte| matches!(byte, b' ' | b'\t')) {
                    offset += 1;
                }
                trivia.push(Trivia {
                    kind: TriviaKind::HorizontalWhitespace,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b'\r' => {
                let start = offset;
                offset += 1;
                if bytes.get(offset) == Some(&b'\n') {
                    offset += 1;
                }
                tokens.push(Token {
                    kind: TokenKind::Newline,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b'\n' => {
                let start = offset;
                offset += 1;
                tokens.push(Token {
                    kind: TokenKind::Newline,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b'=' => {
                let start = offset;
                offset += 1;
                tokens.push(Token {
                    kind: TokenKind::Equals,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b'(' => {
                let start = offset;
                offset += 1;
                tokens.push(Token {
                    kind: TokenKind::LeftParen,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b')' => {
                let start = offset;
                offset += 1;
                tokens.push(Token {
                    kind: TokenKind::RightParen,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b',' => {
                let start = offset;
                offset += 1;
                tokens.push(Token {
                    kind: TokenKind::Comma,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            byte if byte.is_ascii_alphabetic() => {
                let start = offset;
                offset += 1;
                while bytes
                    .get(offset)
                    .is_some_and(|next| next.is_ascii_alphanumeric())
                {
                    offset += 1;
                }
                let text = &source[start..offset];
                tokens.push(Token {
                    kind: TokenKind::Identifier(text.to_owned()),
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            byte if byte.is_ascii_digit() => {
                let start = offset;
                offset += 1;
                while bytes.get(offset).is_some_and(|next| next.is_ascii_digit()) {
                    offset += 1;
                }
                let text = &source[start..offset];
                match text.parse::<i64>() {
                    Ok(value) => tokens.push(Token {
                        kind: TokenKind::Integer(value),
                        span: SourceSpan::new(source_id, start, offset),
                    }),
                    Err(_) => diagnostics.push(diagnostic(
                        "GFA-LEX-0002",
                        DiagnosticCategory::Lexical,
                        "integer literal is outside the supported MVP range",
                        SourceSpan::new(source_id, start, offset),
                    )),
                }
            }
            _ => {
                let start = offset;
                let character = source[offset..]
                    .chars()
                    .next()
                    .expect("valid UTF-8 boundary");
                offset += character.len_utf8();
                diagnostics.push(diagnostic(
                    "GFA-LEX-0001",
                    DiagnosticCategory::Lexical,
                    format!("character {character:?} is not supported by the current MVP frontend"),
                    SourceSpan::new(source_id, start, offset),
                ));
            }
        }
    }

    if diagnostics.is_empty() {
        tokens.push(Token {
            kind: TokenKind::End,
            span: SourceSpan::new(source_id, source.len(), source.len()),
        });
        Ok(LexedSource { tokens, trivia })
    } else {
        Err(diagnostics)
    }
}

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    diagnostics: Vec<Diagnostic>,
    source_id: SourceId,
    source_len: usize,
}

impl Parser {
    fn current(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.cursor].clone();
        if !matches!(token.kind, TokenKind::End) {
            self.cursor += 1;
        }
        token
    }

    fn at_end(&self) -> bool {
        matches!(self.current().kind, TokenKind::End)
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current().kind, TokenKind::Newline) {
            self.advance();
        }
    }

    fn syntax_error(&self, message: impl Into<String>, span: SourceSpan) -> Box<Diagnostic> {
        Box::new(diagnostic(
            "GFA-PARSE-0001",
            DiagnosticCategory::Syntax,
            message,
            span,
        ))
    }

    fn unsupported_error(&self, message: impl Into<String>, span: SourceSpan) -> Box<Diagnostic> {
        Box::new(diagnostic(
            "GFA-SUPPORT-0001",
            DiagnosticCategory::Unsupported,
            message,
            span,
        ))
    }

    fn parse_variable(&mut self) -> Result<(String, SourceSpan), Box<Diagnostic>> {
        let token = self.advance();
        match token.kind {
            TokenKind::Identifier(name) if matches!(name.as_str(), "x" | "y" | "z") => {
                Ok((name, token.span))
            }
            TokenKind::Identifier(name) => Err(self.unsupported_error(
                format!("variable {name:?} is outside the current MVP subset (supported: x, y, z)"),
                token.span,
            )),
            _ => Err(self.syntax_error("expected one of the variables x, y, or z", token.span)),
        }
    }

    fn expect(&mut self, expected: TokenKind, description: &str) -> Result<Token, Box<Diagnostic>> {
        let token = self.current().clone();
        if std::mem::discriminant(&token.kind) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(self.syntax_error(format!("expected {description}"), token.span))
        }
    }

    fn parse_assignment(
        &mut self,
        name: String,
        start: SourceSpan,
    ) -> Result<Statement, Box<Diagnostic>> {
        self.expect(TokenKind::Equals, "'='")?;
        let function = self.advance();
        let function_name = match function.kind {
            TokenKind::Identifier(name) => name,
            _ => {
                return Err(self.syntax_error(
                    "expected a supported intrinsic after assignment",
                    function.span,
                ));
            }
        };

        if !matches!(function_name.as_str(), "FACT" | "VARIAT" | "COMBIN") {
            return Err(self.unsupported_error(
                format!("intrinsic {function_name:?} is outside the current MVP subset"),
                function.span,
            ));
        }

        self.expect(TokenKind::LeftParen, &format!("'(' after {function_name}"))?;
        let first = self.advance();
        let first_value = match first.kind {
            TokenKind::Integer(value) => value,
            _ => {
                return Err(self.syntax_error(
                    "intrinsic arguments must be non-negative integer literals in this MVP",
                    first.span,
                ));
            }
        };

        if function_name == "FACT" {
            self.expect(TokenKind::RightParen, "')' after FACT argument")?;
            let end = self.tokens[self.cursor - 1].span;
            return Ok(Statement::AssignFact {
                name,
                argument: first_value,
                span: SourceSpan::new(self.source_id, start.start, end.end),
            });
        }

        self.expect(TokenKind::Comma, "',' between intrinsic arguments")?;
        let second = self.advance();
        let second_value = match second.kind {
            TokenKind::Integer(value) => value,
            _ => {
                return Err(self.syntax_error(
                    "intrinsic arguments must be non-negative integer literals in this MVP",
                    second.span,
                ));
            }
        };
        self.expect(TokenKind::RightParen, "')' after intrinsic arguments")?;
        let end = self.tokens[self.cursor - 1].span;
        let span = SourceSpan::new(self.source_id, start.start, end.end);

        match function_name.as_str() {
            "VARIAT" => Ok(Statement::AssignVariat {
                name,
                n: first_value,
                k: second_value,
                span,
            }),
            "COMBIN" => Ok(Statement::AssignCombin {
                name,
                n: first_value,
                k: second_value,
                span,
            }),
            _ => unreachable!("FACT returned above"),
        }
    }

    fn parse_print(&mut self, start: SourceSpan) -> Result<Statement, Box<Diagnostic>> {
        let mut names = Vec::new();
        let (first, _) = self.parse_variable()?;
        names.push(first);
        while matches!(self.current().kind, TokenKind::Comma) {
            self.advance();
            let (name, _) = self.parse_variable()?;
            names.push(name);
        }
        let end = self.tokens[self.cursor - 1].span;
        Ok(Statement::PrintVariables {
            names,
            span: SourceSpan::new(self.source_id, start.start, end.end),
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, Box<Diagnostic>> {
        let first = self.advance();
        match first.kind {
            TokenKind::Identifier(name) if name == "PRINT" => self.parse_print(first.span),
            TokenKind::Identifier(name) if matches!(name.as_str(), "x" | "y" | "z") => {
                self.parse_assignment(name, first.span)
            }
            TokenKind::Identifier(name) => Err(self.unsupported_error(
                format!("statement {name:?} is not supported by the current MVP frontend"),
                first.span,
            )),
            _ => Err(self.syntax_error("expected an assignment or PRINT statement", first.span)),
        }
    }

    fn synchronize(&mut self) {
        while !self.at_end() && !matches!(self.current().kind, TokenKind::Newline) {
            self.advance();
        }
        if matches!(self.current().kind, TokenKind::Newline) {
            self.advance();
        }
    }

    fn parse_program(mut self) -> Result<Program, Vec<Diagnostic>> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while !self.at_end() {
            match self.parse_statement() {
                Ok(statement) => {
                    statements.push(statement);
                    if !self.at_end() && !matches!(self.current().kind, TokenKind::Newline) {
                        let token = self.current().clone();
                        self.diagnostics.push(*self.syntax_error(
                            "unexpected token after statement; statements must be on separate lines in this MVP",
                            token.span,
                        ));
                        self.synchronize();
                    } else {
                        self.skip_newlines();
                    }
                }
                Err(error) => {
                    self.diagnostics.push(*error);
                    self.synchronize();
                }
            }
        }

        if statements.is_empty() && self.diagnostics.is_empty() {
            self.diagnostics.push(*self.syntax_error(
                "source contains no supported statements",
                SourceSpan::new(self.source_id, 0, self.source_len),
            ));
        }

        if self.diagnostics.is_empty() {
            Ok(Program { statements })
        } else {
            Err(self.diagnostics)
        }
    }
}

/// Parse the evidence-backed Amiga GFA BASIC 3.5 FACT/PRINT MVP subset.
///
/// This parser intentionally rejects all syntax outside the demonstrated
/// vertical-slice fixture. It does not claim general GFA BASIC compatibility.
pub fn parse(source_id: SourceId, source: &str) -> Result<Program, Vec<Diagnostic>> {
    let tokens = lex_lossless(source_id, source)?.tokens;
    Parser {
        tokens,
        cursor: 0,
        diagnostics: Vec::new(),
        source_id,
        source_len: source.len(),
    }
    .parse_program()
}

#[cfg(test)]
mod tests {
    use super::{Statement, parse};
    use crate::diagnostics::DiagnosticCategory;
    use crate::source::SourceMap;

    #[test]
    fn parses_the_fact_zero_vertical_slice_fixture() {
        let source_text = "x=FACT(0)\ny=FACT(1)\nz=FACT(2)\nPRINT x,y,z\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("fact-zero.bas", source_text);

        let program = parse(source_id, source_text).expect("fixture should parse");
        assert_eq!(program.statements.len(), 4);
        assert!(matches!(
            &program.statements[0],
            Statement::AssignFact { name, argument: 0, .. } if name == "x"
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::AssignFact { name, argument: 1, .. } if name == "y"
        ));
        assert!(matches!(
            &program.statements[2],
            Statement::AssignFact { name, argument: 2, .. } if name == "z"
        ));
        assert!(matches!(
            &program.statements[3],
            Statement::PrintVariables { names, .. } if names.iter().map(String::as_str).collect::<Vec<_>>() == ["x", "y", "z"]
        ));
    }

    #[test]
    fn parses_the_three_documented_amiga_35_combinatorics_intrinsics() {
        let source_text = "x=FACT(5)\ny=VARIAT(5,2)\nz=COMBIN(5,2)\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("combinatorics.bas", source_text);

        let program = parse(source_id, source_text).expect("documented intrinsics should parse");
        assert!(matches!(
            &program.statements[0],
            Statement::AssignFact { name, argument: 5, .. } if name == "x"
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::AssignVariat { name, n: 5, k: 2, .. } if name == "y"
        ));
        assert!(matches!(
            &program.statements[2],
            Statement::AssignCombin { name, n: 5, k: 2, .. } if name == "z"
        ));
    }

    #[test]
    fn rejects_malformed_combinatorics_argument_lists() {
        for source_text in ["x=VARIAT(5)\n", "x=COMBIN(5,2,1)\n"] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("malformed.bas", source_text);
            assert!(
                parse(source_id, source_text).is_err(),
                "accepted: {source_text}"
            );
        }
    }

    #[test]
    fn rejects_unsupported_statement_instead_of_ignoring_it() {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("unsupported.bas", "LET x=1\n");

        let errors = parse(source_id, "LET x=1\n").expect_err("LET is outside the MVP");
        assert_eq!(errors[0].code, "GFA-SUPPORT-0001");
        assert_eq!(errors[0].category, DiagnosticCategory::Unsupported);
        assert!(errors[0].message.contains("not supported"));
        assert_eq!(errors[0].primary_span.unwrap().start, 0);
    }

    #[test]
    fn rejects_variables_outside_the_demonstrated_mvp_names() {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("unknown-variable.bas", "PRINT count\n");

        let errors = parse(source_id, "PRINT count\n").expect_err("name is outside MVP");
        assert!(errors[0].message.contains("supported: x, y, z"));
    }

    #[test]
    fn rejects_unrecognized_punctuation_with_a_lexical_span() {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("bad-token.bas", "x=FACT(0);\n");

        let errors = parse(source_id, "x=FACT(0);\n").expect_err("semicolon is not in the MVP");
        assert_eq!(errors[0].category.as_str(), "lexical");
        assert_eq!(errors[0].primary_span.unwrap().start, 9);
    }

    #[test]
    fn reports_malformed_fact_call_as_syntax_not_unsupported() {
        let source_text = "x=FACT(0\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("malformed.bas", source_text);

        let errors = parse(source_id, source_text).expect_err("missing ')' is malformed syntax");
        assert_eq!(errors[0].category, DiagnosticCategory::Syntax);
        assert_eq!(errors[0].code, "GFA-PARSE-0001");
    }

    #[test]
    fn accepts_crlf_as_transport_level_separators_without_claiming_dialect_conformance() {
        let source_text = "x=FACT(0)\r\ny=FACT(1)\r\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("crlf.bas", source_text);

        let program = parse(source_id, source_text).expect("CRLF line endings should parse");
        assert_eq!(program.statements.len(), 2);
    }
    #[test]
    fn lossless_lexer_preserves_whitespace_and_line_ending_spans() {
        use super::{lex_lossless, TriviaKind};
        let source_text = "x = FACT(5)\\r\\nPRINT x, y\\t\\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("lossless.bas", source_text);
        let lexed = lex_lossless(source_id, source_text).expect("supported tokens should lex");

        assert!(lexed.trivia.iter().all(|item| item.kind == TriviaKind::HorizontalWhitespace));
        let mut spans: Vec<_> = lexed.tokens.iter()
            .filter(|token| !matches!(&token.kind, super::TokenKind::End))
            .map(|token| token.span)
            .chain(lexed.trivia.iter().map(|item| item.span))
            .collect();
        spans.sort_by_key(|span| span.start);

        let reconstructed = spans.into_iter()
            .map(|span| &source_text[span.start..span.end])
            .collect::<String>();
        assert_eq!(reconstructed, source_text);
    }

    #[test]
    fn lossless_lexer_retains_original_identifier_spelling() {
        use super::{lex_lossless, TokenKind};
        let source_text = "print FACT(1)\\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("spelling.bas", source_text);
        let lexed = lex_lossless(source_id, source_text).expect("identifier candidates should lex");

        assert!(matches!(&lexed.tokens[0].kind, TokenKind::Identifier(name) if name == "print"));
        assert_eq!(
            &source_text[lexed.tokens[0].span.start..lexed.tokens[0].span.end],
            "print"
        );
    }

}
