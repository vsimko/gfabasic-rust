//! Evidence-gated frontend for the first Amiga GFA BASIC 3.5 vertical slice.
//!
//! This is deliberately not a general GFA BASIC lexer/parser. It recognizes
//! only the source forms demonstrated by the 3.5 addendum: FACT/VARIAT/COMBIN
//! assignments and PRINT, plus selected verified MAT matrix commands. Other
//! syntax is rejected rather than guessed.

use crate::diagnostics::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::source::{SourceId, SourceSpan};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// A successful parse together with the lossless syntax representation.
///
/// The syntax stream retains original source spans for recognized tokens and
/// trivia; the AST contains the semantic structure used by later compiler stages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedDocument {
    /// Parsed statements for the currently supported language subset.
    pub program: Program,
    /// Lossless token/trivia representation of the same source document.
    pub syntax: LexedSource,
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
    DimMatrix {
        name: String,
        rows: i64,
        columns: Option<i64>,
        span: SourceSpan,
    },
    MatOne {
        name: String,
        span: SourceSpan,
    },
    MatSet {
        name: String,
        value: i64,
        span: SourceSpan,
    },
    MatClr {
        name: String,
        span: SourceSpan,
    },
    MatPrint {
        name: String,
        channel: Option<i64>,
        format: Option<(i64, i64)>,
        span: SourceSpan,
    },
    MatBase {
        index: i64,
        span: SourceSpan,
    },
    MatTrans {
        destination: String,
        source: Option<String>,
        span: SourceSpan,
    },
    MatNorm {
        name: String,
        mode: i64,
        span: SourceSpan,
    },
    MatArithmetic {
        operation: MatArithmeticOperation,
        destination: String,
        left: Option<String>,
        right: MatArithmeticOperand,
        span: SourceSpan,
    },
    MatMulMatrix {
        destination: String,
        left: String,
        right: String,
        span: SourceSpan,
    },
    MatMulScalar {
        destination: String,
        operands: Vec<String>,
        span: SourceSpan,
    },
    MatMulScale {
        destination: String,
        scalar: String,
        span: SourceSpan,
    },
    MatAnalysis {
        operation: MatAnalysisOperation,
        destination: String,
        matrix: String,
        offsets: Option<(i64, i64)>,
        order: Option<i64>,
        span: SourceSpan,
    },
    MatCopy {
        operation: MatCopyOperation,
        destination: String,
        destination_offset: Option<(i64, i64)>,
        source: String,
        source_offset: Option<(i64, i64)>,
        extent: Option<(i64, i64)>,
        span: SourceSpan,
    },
    MatRead {
        matrix: String,
        span: SourceSpan,
    },
    MatInput {
        channel: i64,
        matrix: String,
        span: SourceSpan,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatCopyOperation {
    Copy,
    TransposedCopy,
}

type ParsedMatrixSelection = (String, Option<(i64, i64)>, SourceSpan);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatAnalysisOperation {
    Det,
    QDet,
    Rang,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatArithmeticOperation {
    Add,
    Sub,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatArithmeticOperand {
    Matrix(String),
    Scalar(String),
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
    /// Matrix addition operator.
    Plus,
    /// Matrix subtraction operator.
    Minus,
    /// Matrix multiplication operator.
    Star,
    /// File channel prefix.
    Hash,
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
    /// A GFA BASIC inline comment beginning with ! and ending before the line break.
    Comment,
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

fn is_identifier_continuation(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// A boundary accepted by the currently implemented, evidence-gated token
/// subset. Requiring a known boundary prevents a documented special name from
/// being accepted as a prefix of otherwise unsupported adjacent source text.
fn is_supported_token_boundary(byte: u8) -> bool {
    matches!(
        byte,
        b' ' | b'\t' | b'\r' | b'\n' | b'+' | b'-' | b'#' | b'*' | b'=' | b'(' | b')' | b','
    )
}

/// Recognizes only the underscore-prefixed names explicitly listed by the
/// Amiga GFA BASIC 3.5 addendum: _DATA and _0 through _9. This deliberately
/// does not define a general underscore-identifier grammar.
fn amiga_35_special_identifier_end(bytes: &[u8], start: usize) -> Option<usize> {
    let end = if bytes.get(start..)?.starts_with(b"_DATA") {
        start + 5
    } else if bytes.get(start + 1).is_some_and(u8::is_ascii_digit) {
        start + 2
    } else {
        return None;
    };

    match bytes.get(end) {
        None => Some(end),
        Some(next) if is_supported_token_boundary(*next) => Some(end),
        Some(_) => None,
    }
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
            b'!' => {
                let start = offset;
                while bytes
                    .get(offset)
                    .is_some_and(|byte| !matches!(byte, b'\r' | b'\n'))
                {
                    offset += 1;
                }
                trivia.push(Trivia {
                    kind: TriviaKind::Comment,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b' ' | b'\t' => {
                let start = offset;
                while bytes
                    .get(offset)
                    .is_some_and(|byte| matches!(byte, b' ' | b'\t'))
                {
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
            b'+' => {
                let start = offset;
                offset += 1;
                tokens.push(Token {
                    kind: TokenKind::Plus,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b'-' => {
                let start = offset;
                offset += 1;
                tokens.push(Token {
                    kind: TokenKind::Minus,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b'#' => {
                let start = offset;
                offset += 1;
                tokens.push(Token {
                    kind: TokenKind::Hash,
                    span: SourceSpan::new(source_id, start, offset),
                });
            }
            b'*' => {
                let start = offset;
                offset += 1;
                tokens.push(Token {
                    kind: TokenKind::Star,
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
            b'_' => {
                let start = offset;
                if let Some(end) = amiga_35_special_identifier_end(bytes, start) {
                    offset = end;
                    tokens.push(Token {
                        kind: TokenKind::Identifier(source[start..end].to_owned()),
                        span: SourceSpan::new(source_id, start, end),
                    });
                } else {
                    offset += 1;
                    while bytes
                        .get(offset)
                        .is_some_and(|next| is_identifier_continuation(*next))
                    {
                        offset += 1;
                    }
                    diagnostics.push(diagnostic(
                        "GFA-LEX-0001",
                        DiagnosticCategory::Lexical,
                        "underscore-prefixed name is not one of the explicitly documented Amiga GFA BASIC 3.5 forms (_DATA or _0 through _9)",
                        SourceSpan::new(source_id, start, offset),
                    ));
                }
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

    /// Consume a grammar token without stealing a line boundary from recovery.
    fn advance_required(&mut self, description: &str) -> Result<Token, Box<Diagnostic>> {
        let token = self.current().clone();
        if matches!(token.kind, TokenKind::Newline | TokenKind::End) {
            Err(self.syntax_error(format!("expected {description}"), token.span))
        } else {
            Ok(self.advance())
        }
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
        // A missing variable at a line boundary must not consume the newline:
        // `synchronize` owns that token and uses it to resume at the next line.
        // Consuming it here would make recovery skip the following statement.
        if matches!(self.current().kind, TokenKind::Newline | TokenKind::End) {
            return Err(self.syntax_error(
                "expected one of the variables x, y, or z",
                self.current().span,
            ));
        }
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
        let function = self.advance_required("a supported intrinsic after assignment")?;
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
        let first = self.advance_required("an intrinsic argument")?;
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
        let second = self.advance_required("an intrinsic argument")?;
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

    fn parse_matrix_name(&mut self) -> Result<(String, SourceSpan), Box<Diagnostic>> {
        let token = self.advance_required("matrix name 'a' or 'b'")?;
        match token.kind {
            TokenKind::Identifier(name) if matches!(name.as_str(), "a" | "b") => Ok((name, token.span)),
            TokenKind::Identifier(name) => Err(self.unsupported_error(
                format!("matrix name {name:?} is outside the current verified matrix subset (supported: a, b)"),
                token.span,
            )),
            _ => Err(self.syntax_error("expected matrix name 'a'", token.span)),
        }
    }

    fn parse_matrix_reference(&mut self) -> Result<(String, SourceSpan), Box<Diagnostic>> {
        let (name, start) = self.parse_matrix_name()?;
        self.expect(TokenKind::LeftParen, "'(' after matrix name")?;
        let end = self
            .expect(TokenKind::RightParen, "')' after matrix name")?
            .span;
        Ok((name, SourceSpan::new(self.source_id, start.start, end.end)))
    }

    fn parse_nonnegative_matrix_integer(&mut self) -> Result<i64, Box<Diagnostic>> {
        let token = self.advance_required("a non-negative integer literal")?;
        match token.kind {
            TokenKind::Integer(value) => Ok(value),
            _ => Err(self.syntax_error(
                "expected a non-negative integer literal in the supported matrix command subset",
                token.span,
            )),
        }
    }

    fn parse_dim_matrix(&mut self, start: SourceSpan) -> Result<Statement, Box<Diagnostic>> {
        let (name, _) = self.parse_matrix_name()?;
        self.expect(TokenKind::LeftParen, "'(' after matrix name")?;
        let rows = self.parse_nonnegative_matrix_integer()?;
        let columns = if matches!(self.current().kind, TokenKind::Comma) {
            self.advance();
            Some(self.parse_nonnegative_matrix_integer()?)
        } else {
            None
        };
        let end = self
            .expect(TokenKind::RightParen, "')' after matrix dimensions")?
            .span;
        Ok(Statement::DimMatrix {
            name,
            rows,
            columns,
            span: SourceSpan::new(self.source_id, start.start, end.end),
        })
    }

    fn parse_mat_arithmetic(
        &mut self,
        start: SourceSpan,
        operation: MatArithmeticOperation,
    ) -> Result<Statement, Box<Diagnostic>> {
        let (destination, _) = self.parse_matrix_reference()?;
        let (left, right) =
            if matches!(self.current().kind, TokenKind::Equals) {
                self.advance();
                let (left, _) = self.parse_matrix_reference()?;
                let operator = self.advance_required("an operator in MAT arithmetic")?;
                let expected = match operation {
                    MatArithmeticOperation::Add => TokenKind::Plus,
                    MatArithmeticOperation::Sub => TokenKind::Minus,
                };
                if std::mem::discriminant(&operator.kind) != std::mem::discriminant(&expected) {
                    return Err(self.syntax_error(
                        match operation {
                            MatArithmeticOperation::Add => "expected '+' in MAT ADD expression",
                            MatArithmeticOperation::Sub => "expected '-' in MAT SUB expression",
                        },
                        operator.span,
                    ));
                }
                let (right, _) = self.parse_matrix_reference()?;
                (Some(left), MatArithmeticOperand::Matrix(right))
            } else {
                self.expect(TokenKind::Comma, "',' in in-place MAT arithmetic")?;
                let token = self.current().clone();
                match &token.kind {
                    TokenKind::Identifier(name)
                        if matches!(name.as_str(), "a" | "b")
                            && self
                                .tokens
                                .get(self.cursor + 1)
                                .is_some_and(|next| matches!(&next.kind, TokenKind::LeftParen)) =>
                    {
                        let (matrix, _) = self.parse_matrix_reference()?;
                        (None, MatArithmeticOperand::Matrix(matrix))
                    }
                    TokenKind::Identifier(name) => {
                        let scalar = name.clone();
                        self.advance();
                        (None, MatArithmeticOperand::Scalar(scalar))
                    }
                    _ => return Err(self.syntax_error(
                        "expected a matrix reference or scalar variable after MAT arithmetic comma",
                        token.span,
                    )),
                }
            };
        let end = self.tokens[self.cursor - 1].span;
        Ok(Statement::MatArithmetic {
            operation,
            destination,
            left,
            right,
            span: SourceSpan::new(self.source_id, start.start, end.end),
        })
    }

    fn parse_mat_mul(&mut self, start: SourceSpan) -> Result<Statement, Box<Diagnostic>> {
        let token = self.current().clone();
        let matrix_destination = matches!(
            &token.kind,
            TokenKind::Identifier(name) if matches!(name.as_str(), "a" | "b")
        ) && self
            .tokens
            .get(self.cursor + 1)
            .is_some_and(|next| matches!(&next.kind, TokenKind::LeftParen));

        if matrix_destination {
            let (destination, _) = self.parse_matrix_reference()?;
            if matches!(self.current().kind, TokenKind::Equals) {
                self.advance();
                let (left, _) = self.parse_matrix_reference()?;
                self.expect(TokenKind::Star, "'*' between MAT MUL matrix operands")?;
                let (right, _) = self.parse_matrix_reference()?;
                let end = self.tokens[self.cursor - 1].span;
                Ok(Statement::MatMulMatrix {
                    destination,
                    left,
                    right,
                    span: SourceSpan::new(self.source_id, start.start, end.end),
                })
            } else {
                self.expect(TokenKind::Comma, "',' in in-place MAT MUL")?;
                let scalar_token =
                    self.advance_required("a scalar variable after MAT MUL destination comma")?;
                let scalar = match scalar_token.kind {
                    TokenKind::Identifier(name) => name,
                    _ => {
                        return Err(self.syntax_error(
                            "expected a scalar variable after MAT MUL destination comma",
                            scalar_token.span,
                        ));
                    }
                };
                let end = self.tokens[self.cursor - 1].span;
                Ok(Statement::MatMulScale {
                    destination,
                    scalar,
                    span: SourceSpan::new(self.source_id, start.start, end.end),
                })
            }
        } else {
            let destination_token =
                self.advance_required("a scalar or matrix destination in MAT MUL")?;
            let destination = match destination_token.kind {
                TokenKind::Identifier(name) => name,
                _ => {
                    return Err(self.syntax_error(
                        "expected a scalar destination or matrix destination in MAT MUL",
                        destination_token.span,
                    ));
                }
            };
            self.expect(TokenKind::Equals, "'=' in scalar-result MAT MUL")?;
            let (first, _) = self.parse_matrix_reference()?;
            self.expect(TokenKind::Star, "'*' between MAT MUL operands")?;
            let (second, _) = self.parse_matrix_reference()?;
            let mut operands = vec![first, second];
            if matches!(self.current().kind, TokenKind::Star) {
                self.advance();
                let (third, _) = self.parse_matrix_reference()?;
                operands.push(third);
            }
            let end = self.tokens[self.cursor - 1].span;
            Ok(Statement::MatMulScalar {
                destination,
                operands,
                span: SourceSpan::new(self.source_id, start.start, end.end),
            })
        }
    }

    fn parse_mat_analysis(
        &mut self,
        start: SourceSpan,
        operation: MatAnalysisOperation,
    ) -> Result<Statement, Box<Diagnostic>> {
        let destination_token =
            self.advance_required("a scalar destination in MAT matrix analysis")?;
        let destination = match destination_token.kind {
            TokenKind::Identifier(name) => name,
            _ => {
                return Err(self.syntax_error(
                    "expected a scalar destination in MAT matrix analysis",
                    destination_token.span,
                ));
            }
        };
        self.expect(TokenKind::Equals, "'=' in MAT matrix analysis")?;
        let (matrix, _) = self.parse_matrix_name()?;
        self.expect(TokenKind::LeftParen, "'(' after matrix name")?;
        let offsets = if matches!(self.current().kind, TokenKind::RightParen) {
            None
        } else {
            let row = self.parse_nonnegative_matrix_integer()?;
            self.expect(
                TokenKind::Comma,
                "',' between matrix row and column offsets",
            )?;
            let column = self.parse_nonnegative_matrix_integer()?;
            Some((row, column))
        };
        self.expect(TokenKind::RightParen, "')' after matrix selection")?;
        let order = if matches!(self.current().kind, TokenKind::Comma) {
            self.advance();
            Some(self.parse_nonnegative_matrix_integer()?)
        } else {
            None
        };
        let end = self.tokens[self.cursor - 1].span;
        Ok(Statement::MatAnalysis {
            operation,
            destination,
            matrix,
            offsets,
            order,
            span: SourceSpan::new(self.source_id, start.start, end.end),
        })
    }

    fn parse_matrix_selection(&mut self) -> Result<ParsedMatrixSelection, Box<Diagnostic>> {
        let (name, name_span) = self.parse_matrix_name()?;
        self.expect(TokenKind::LeftParen, "'(' after matrix name")?;
        let offset = if matches!(self.current().kind, TokenKind::RightParen) {
            None
        } else {
            let row = self.parse_nonnegative_matrix_integer()?;
            self.expect(
                TokenKind::Comma,
                "',' between matrix row and column offsets",
            )?;
            let column = self.parse_nonnegative_matrix_integer()?;
            Some((row, column))
        };
        let end = self.expect(TokenKind::RightParen, "')' after matrix selection")?;
        Ok((
            name,
            offset,
            SourceSpan::new(self.source_id, name_span.start, end.span.end),
        ))
    }

    fn parse_mat_copy(
        &mut self,
        start: SourceSpan,
        operation: MatCopyOperation,
    ) -> Result<Statement, Box<Diagnostic>> {
        let (destination, destination_offset, _) = self.parse_matrix_selection()?;
        self.expect(TokenKind::Equals, "'=' in MAT copy statement")?;
        let (source, source_offset, _) = self.parse_matrix_selection()?;
        let extent = if matches!(self.current().kind, TokenKind::Comma) {
            self.advance();
            let height = self.parse_nonnegative_matrix_integer()?;
            self.expect(TokenKind::Comma, "',' between MAT copy height and width")?;
            let width = self.parse_nonnegative_matrix_integer()?;
            Some((height, width))
        } else {
            None
        };
        let end = self.tokens[self.cursor - 1].span;
        Ok(Statement::MatCopy {
            operation,
            destination,
            destination_offset,
            source,
            source_offset,
            extent,
            span: SourceSpan::new(self.source_id, start.start, end.end),
        })
    }

    fn parse_mat_read(&mut self, start: SourceSpan) -> Result<Statement, Box<Diagnostic>> {
        let (matrix, _) = self.parse_matrix_reference()?;
        let end = self.tokens[self.cursor - 1].span;
        Ok(Statement::MatRead {
            matrix,
            span: SourceSpan::new(self.source_id, start.start, end.end),
        })
    }

    fn parse_mat_input(&mut self, start: SourceSpan) -> Result<Statement, Box<Diagnostic>> {
        self.expect(TokenKind::Hash, "'#' before MAT INPUT channel")?;
        let channel = self.parse_nonnegative_matrix_integer()?;
        self.expect(TokenKind::Comma, "',' after MAT INPUT channel")?;
        let (matrix, _) = self.parse_matrix_reference()?;
        let end = self.tokens[self.cursor - 1].span;
        Ok(Statement::MatInput {
            channel,
            matrix,
            span: SourceSpan::new(self.source_id, start.start, end.end),
        })
    }

    fn parse_mat_statement(&mut self, start: SourceSpan) -> Result<Statement, Box<Diagnostic>> {
        let operation = self.advance_required("a MAT operation name")?;
        let operation_name = match operation.kind {
            TokenKind::Identifier(name) => name,
            _ => {
                return Err(self.syntax_error(
                    "expected MAT ONE, MAT SET, MAT CLR, or MAT PRINT",
                    operation.span,
                ));
            }
        };
        match operation_name.as_str() {
            "BASE" => {
                let index = self.parse_nonnegative_matrix_integer()?;
                if !matches!(index, 0 | 1) {
                    return Err(self.unsupported_error(
                        "MAT BASE supports only the documented index offsets 0 and 1",
                        self.tokens[self.cursor - 1].span,
                    ));
                }
                let end = self.tokens[self.cursor - 1].span;
                Ok(Statement::MatBase {
                    index,
                    span: SourceSpan::new(self.source_id, start.start, end.end),
                })
            }
            "READ" => self.parse_mat_read(start),
            "INPUT" => self.parse_mat_input(start),
            "CPY" => self.parse_mat_copy(start, MatCopyOperation::Copy),
            "XCPY" => self.parse_mat_copy(start, MatCopyOperation::TransposedCopy),
            "DET" => self.parse_mat_analysis(start, MatAnalysisOperation::Det),
            "QDET" => self.parse_mat_analysis(start, MatAnalysisOperation::QDet),
            "RANG" => self.parse_mat_analysis(start, MatAnalysisOperation::Rang),
            "MUL" => self.parse_mat_mul(start),
            "ADD" => self.parse_mat_arithmetic(start, MatArithmeticOperation::Add),
            "SUB" => self.parse_mat_arithmetic(start, MatArithmeticOperation::Sub),
            "NORM" => {
                let (name, matrix_span) = self.parse_matrix_reference()?;
                self.expect(TokenKind::Comma, "',' before MAT NORM mode")?;
                let mode = self.parse_nonnegative_matrix_integer()?;
                if !matches!(mode, 0 | 1) {
                    return Err(self.unsupported_error(
                        "MAT NORM supports only the documented row mode 0 and column mode 1",
                        self.tokens[self.cursor - 1].span,
                    ));
                }
                let end = self.tokens[self.cursor - 1].span;
                Ok(Statement::MatNorm {
                    name,
                    mode,
                    span: SourceSpan::new(self.source_id, start.start, end.end.max(matrix_span.end)),
                })
            }
            "TRANS" => {
                let (destination, destination_span) = self.parse_matrix_reference()?;
                let source = if matches!(self.current().kind, TokenKind::Equals) {
                    self.advance();
                    Some(self.parse_matrix_reference()?.0)
                } else {
                    None
                };
                let end = self.tokens[self.cursor - 1].span;
                Ok(Statement::MatTrans {
                    destination,
                    source,
                    span: SourceSpan::new(self.source_id, start.start, end.end.max(destination_span.end)),
                })
            }
            "ONE" | "CLR" => {
                let (name, end) = self.parse_matrix_reference()?;
                let span = SourceSpan::new(self.source_id, start.start, end.end);
                match operation_name.as_str() {
                    "ONE" => Ok(Statement::MatOne { name, span }),
                    "CLR" => Ok(Statement::MatClr { name, span }),
                    _ => unreachable!(),
                }
            }
            "PRINT" => {
                let channel = if matches!(self.current().kind, TokenKind::Hash) {
                    self.advance();
                    let channel = self.parse_nonnegative_matrix_integer()?;
                    self.expect(TokenKind::Comma, "',' after MAT PRINT channel")?;
                    Some(channel)
                } else {
                    None
                };
                let (name, _) = self.parse_matrix_reference()?;
                let format = if matches!(self.current().kind, TokenKind::Comma) {
                    self.advance();
                    let width = self.parse_nonnegative_matrix_integer()?;
                    self.expect(TokenKind::Comma, "',' between MAT PRINT width and precision")?;
                    let precision = self.parse_nonnegative_matrix_integer()?;
                    Some((width, precision))
                } else {
                    None
                };
                let end = self.tokens[self.cursor - 1].span;
                Ok(Statement::MatPrint {
                    name,
                    channel,
                    format,
                    span: SourceSpan::new(self.source_id, start.start, end.end),
                })
            }
            "SET" => {
                let (name, _) = self.parse_matrix_reference()?;
                self.expect(TokenKind::Equals, "'=' in MAT SET")?;
                let value = self.parse_nonnegative_matrix_integer()?;
                let span = SourceSpan::new(self.source_id, start.start, self.tokens[self.cursor - 1].span.end);
                Ok(Statement::MatSet { name, value, span })
            }
            _ => Err(self.unsupported_error(
                format!("MAT operation {operation_name:?} is outside the current verified fixture subset"),
                operation.span,
            )),
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, Box<Diagnostic>> {
        let first = self.advance();
        match first.kind {
            TokenKind::Identifier(name) if name == "PRINT" => self.parse_print(first.span),
            TokenKind::Identifier(name) if name == "DIM" => self.parse_dim_matrix(first.span),
            TokenKind::Identifier(name) if name == "MAT" => self.parse_mat_statement(first.span),
            TokenKind::Identifier(name) if matches!(name.as_str(), "x" | "y" | "z") => {
                self.parse_assignment(name, first.span)
            }
            TokenKind::Identifier(name) => Err(self.unsupported_error(
                format!("statement {name:?} is not supported by the current MVP frontend"),
                first.span,
            )),
            _ => Err(self.syntax_error(
                "expected an assignment, PRINT, DIM, or MAT statement",
                first.span,
            )),
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

/// Parses the supported Amiga GFA BASIC 3.5 subset and retains its lossless syntax.
///
/// The returned token/trivia stream and semantic AST refer to the same original
/// source. Unsupported syntax remains diagnostic and is not silently accepted.
pub fn parse_document(
    source_id: SourceId,
    source: &str,
) -> Result<ParsedDocument, Vec<Diagnostic>> {
    let syntax = lex_lossless(source_id, source)?;
    let program = Parser {
        tokens: syntax.tokens.clone(),
        cursor: 0,
        diagnostics: Vec::new(),
        source_id,
        source_len: source.len(),
    }
    .parse_program()?;

    Ok(ParsedDocument { program, syntax })
}

/// Parses the supported subset and returns only its semantic AST.
///
/// This compatibility wrapper preserves the original API; callers that need
/// formatting or editor source fidelity should call parse_document.
pub fn parse(source_id: SourceId, source: &str) -> Result<Program, Vec<Diagnostic>> {
    parse_document(source_id, source).map(|document| document.program)
}

#[cfg(test)]
mod tests {
    use super::{Statement, parse, parse_document};
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
    fn parses_one_and_two_dimensional_matrix_declarations() {
        let source = "DIM a(5)\nDIM b(1,5)\nDIM b(5,1)\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("dim-vectors.bas", source);
        let program = parse(source_id, source).unwrap();

        assert!(matches!(
            &program.statements[0],
            Statement::DimMatrix {
                name,
                rows: 5,
                columns: None,
                ..
            } if name == "a"
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::DimMatrix {
                name,
                rows: 1,
                columns: Some(5),
                ..
            } if name == "b"
        ));
        assert!(matches!(
            &program.statements[2],
            Statement::DimMatrix {
                name,
                rows: 5,
                columns: Some(1),
                ..
            } if name == "b"
        ));
    }

    #[test]
    fn parses_verified_amiga_35_matrix_fixtures() {
        let fixtures = [
            include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-one-identity.lst"),
            include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-clear-zero.lst"),
            include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-set-scalar.lst"),
        ];

        for source_text in fixtures {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("verified-matrix.bas", source_text);
            let program = parse(source_id, source_text)
                .expect("verified Amiga 3.5 matrix fixture should parse");
            assert!(matches!(
                program.statements.first(),
                Some(Statement::DimMatrix { name, rows: 3, columns: Some(3), .. }) if name == "a"
            ));
            assert!(matches!(
                program.statements.last(),
                Some(Statement::MatPrint { name, .. }) if name == "a"
            ));
        }
    }

    #[test]
    fn parses_matrix_operations_with_exact_fixture_shapes() {
        let source_text = "DIM a(3,3)\nMAT ONE a()\nMAT SET a()=7\nMAT CLR a()\nMAT PRINT a()\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("matrix.bas", source_text);
        let program = parse(source_id, source_text).unwrap();

        assert!(matches!(&program.statements[1], Statement::MatOne { name, .. } if name == "a"));
        assert!(
            matches!(&program.statements[2], Statement::MatSet { name, value: 7, .. } if name == "a")
        );
        assert!(matches!(&program.statements[3], Statement::MatClr { name, .. } if name == "a"));
    }

    #[test]
    fn parses_verified_mat_base_and_transpose_forms() {
        let source = "MAT BASE 0\nMAT BASE 1\nMAT TRANS a()\nMAT TRANS a()=b()\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("mat-transpose.bas", source);
        let program = parse(source_id, source).unwrap();

        assert!(matches!(
            &program.statements[0],
            Statement::MatBase { index: 0, .. }
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::MatBase { index: 1, .. }
        ));
        assert!(matches!(
            &program.statements[2],
            Statement::MatTrans { destination, source: None, .. } if destination == "a"
        ));
        assert!(matches!(
            &program.statements[3],
            Statement::MatTrans { destination, source: Some(source), .. }
                if destination == "a" && source == "b"
        ));

        let fixture = include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-base-transpose.lst");
        let mut fixture_sources = SourceMap::new();
        let fixture_id = fixture_sources.add_source("mat-base-transpose.bas", fixture);
        let fixture_program = parse(fixture_id, fixture).unwrap();
        assert!(fixture_program.statements.iter().any(|statement| matches!(
            statement,
            Statement::MatTrans { destination, source: Some(source), .. }
                if destination == "a" && source == "b"
        )));
    }

    #[test]
    fn parses_verified_mat_norm_row_fixture_and_column_mode() {
        let source = include_str!("../../../tests/corpus/amiga/3.5/matrix/mat-norm-row.lst");
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("mat-norm-row.bas", source);
        let program = parse(source_id, source).unwrap();
        assert!(matches!(
            &program.statements[1],
            Statement::MatNorm { name, mode: 0, .. } if name == "a"
        ));

        let column_source = "MAT NORM a(),1\n";
        let mut column_sources = SourceMap::new();
        let column_id = column_sources.add_source("mat-norm-column.bas", column_source);
        let column_program = parse(column_id, column_source).unwrap();
        assert!(matches!(
            &column_program.statements[0],
            Statement::MatNorm { name, mode: 1, .. } if name == "a"
        ));
    }

    #[test]
    fn parses_verified_mat_print_channel_and_format_options() {
        let source = "MAT PRINT a()\nMAT PRINT #1,a()\nMAT PRINT a(),7,3\nMAT PRINT #2,b(),8,4\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("mat-print-options.bas", source);
        let program = parse(source_id, source).unwrap();

        assert!(matches!(
            &program.statements[0],
            Statement::MatPrint {
                name,
                channel: None,
                format: None,
                ..
            } if name == "a"
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::MatPrint {
                name,
                channel: Some(1),
                format: None,
                ..
            } if name == "a"
        ));
        assert!(matches!(
            &program.statements[2],
            Statement::MatPrint {
                name,
                channel: None,
                format: Some((7, 3)),
                ..
            } if name == "a"
        ));
        assert!(matches!(
            &program.statements[3],
            Statement::MatPrint {
                name,
                channel: Some(2),
                format: Some((8, 4)),
                ..
            } if name == "b"
        ));
    }

    #[test]
    fn rejects_malformed_mat_print_options() {
        for source in [
            "MAT PRINT #1 a()\n",
            "MAT PRINT a(),7\n",
            "MAT PRINT a(),7,3,2\n",
        ] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("invalid-mat-print.bas", source);
            assert!(parse(source_id, source).is_err(), "accepted: {source:?}");
        }
    }

    #[test]
    fn parses_verified_mat_read_and_input_forms() {
        let source = "MAT READ a()\nMAT INPUT #1,b()\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("mat-read-input.bas", source);
        let program = parse(source_id, source).unwrap();

        assert!(matches!(
            &program.statements[0],
            Statement::MatRead { matrix, .. } if matrix == "a"
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::MatInput { channel: 1, matrix, .. } if matrix == "b"
        ));
    }

    #[test]
    fn rejects_malformed_mat_input_channel_and_target() {
        for source in [
            "MAT INPUT a()\n",
            "MAT INPUT #1\n",
            "MAT INPUT #x,a()\n",
            "MAT INPUT #1,7\n",
        ] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("invalid-mat-input.bas", source);
            assert!(parse(source_id, source).is_err(), "accepted: {source:?}");
        }
    }

    #[test]
    fn parses_verified_mat_cpy_and_xcpy_forms() {
        use super::MatCopyOperation;

        let source = "MAT CPY a()=b()\nMAT CPY a(1,2)=b(3,4),5,6\nMAT XCPY a()=b()\nMAT XCPY b(2,1)=a(0,3),4,5\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("mat-copy.bas", source);
        let program = parse(source_id, source).unwrap();

        assert!(matches!(
            &program.statements[0],
            Statement::MatCopy {
                operation: MatCopyOperation::Copy,
                destination,
                destination_offset: None,
                source,
                source_offset: None,
                extent: None,
                ..
            } if destination == "a" && source == "b"
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::MatCopy {
                operation: MatCopyOperation::Copy,
                destination_offset: Some((1, 2)),
                source_offset: Some((3, 4)),
                extent: Some((5, 6)),
                ..
            }
        ));
        assert!(matches!(
            &program.statements[2],
            Statement::MatCopy {
                operation: MatCopyOperation::TransposedCopy,
                destination,
                source,
                ..
            } if destination == "a" && source == "b"
        ));
        assert!(matches!(
            &program.statements[3],
            Statement::MatCopy {
                operation: MatCopyOperation::TransposedCopy,
                destination,
                destination_offset: Some((2, 1)),
                source,
                source_offset: Some((0, 3)),
                extent: Some((4, 5)),
                ..
            } if destination == "b" && source == "a"
        ));
    }

    #[test]
    fn rejects_malformed_mat_copy_selections() {
        for source in [
            "MAT CPY a(1)=b()\n",
            "MAT XCPY a()=b(1,2,3)\n",
            "MAT CPY a()=b(),4\n",
        ] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("invalid-mat-copy.bas", source);
            assert!(parse(source_id, source).is_err(), "accepted: {source:?}");
        }
    }

    #[test]
    fn parses_verified_mat_det_qdet_and_rang_forms() {
        use super::MatAnalysisOperation;

        let source = "MAT DET x=a()\nMAT DET x=a(1,2),3\nMAT QDET x=b(0,0)\nMAT RANG y=a(2,1),4\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("mat-analysis.bas", source);
        let program = parse(source_id, source).unwrap();

        assert!(matches!(
            &program.statements[0],
            Statement::MatAnalysis {
                operation: MatAnalysisOperation::Det,
                destination,
                matrix,
                offsets: None,
                order: None,
                ..
            } if destination == "x" && matrix == "a"
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::MatAnalysis {
                operation: MatAnalysisOperation::Det,
                offsets: Some((1, 2)),
                order: Some(3),
                ..
            }
        ));
        assert!(matches!(
            &program.statements[2],
            Statement::MatAnalysis {
                operation: MatAnalysisOperation::QDet,
                destination,
                matrix,
                offsets: Some((0, 0)),
                order: None,
                ..
            } if destination == "x" && matrix == "b"
        ));
        assert!(matches!(
            &program.statements[3],
            Statement::MatAnalysis {
                operation: MatAnalysisOperation::Rang,
                destination,
                offsets: Some((2, 1)),
                order: Some(4),
                ..
            } if destination == "y"
        ));
    }

    #[test]
    fn rejects_malformed_matrix_analysis_selection() {
        for source in [
            "MAT DET x=a(1)\n",
            "MAT QDET x=a(1,2,3)\n",
            "MAT RANG x=a(),-1\n",
        ] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("invalid-mat-analysis.bas", source);
            assert!(parse(source_id, source).is_err(), "accepted: {source:?}");
        }
    }

    #[test]
    fn parses_verified_mat_mul_forms() {
        let source =
            "MAT MUL a()=b()*a()\nMAT MUL x=a()*b()\nMAT MUL y=a()*b()*a()\nMAT MUL a(),x\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("mat-mul.bas", source);
        let program = parse(source_id, source).unwrap();

        assert!(matches!(
            &program.statements[0],
            Statement::MatMulMatrix { destination, left, right, .. }
                if destination == "a" && left == "b" && right == "a"
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::MatMulScalar { destination, operands, .. }
                if destination == "x" && operands == &["a", "b"]
        ));
        assert!(matches!(
            &program.statements[2],
            Statement::MatMulScalar { destination, operands, .. }
                if destination == "y" && operands == &["a", "b", "a"]
        ));
        assert!(matches!(
            &program.statements[3],
            Statement::MatMulScale { destination, scalar, .. }
                if destination == "a" && scalar == "x"
        ));
    }

    #[test]
    fn rejects_unsupported_mat_mul_shapes() {
        for source in [
            "MAT MUL a()=b()+a()\n",
            "MAT MUL x=a()\n",
            "MAT MUL a(),7\n",
            "MAT MUL x=a()*b()*a()*b()\n",
        ] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("invalid-mat-mul.bas", source);
            assert!(parse(source_id, source).is_err(), "accepted: {source:?}");
        }
    }

    #[test]
    fn parses_verified_mat_add_and_sub_forms() {
        use super::{MatArithmeticOperand, MatArithmeticOperation};

        let source = "MAT ADD a()=b()+a()\nMAT ADD a(),b()\nMAT ADD a(),x\nMAT SUB b()=a()-b()\nMAT SUB b(),a()\nMAT SUB b(),y\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("mat-arithmetic.bas", source);
        let program = parse(source_id, source).unwrap();

        assert!(matches!(
            &program.statements[0],
            Statement::MatArithmetic {
                operation: MatArithmeticOperation::Add,
                destination,
                left: Some(left),
                right: MatArithmeticOperand::Matrix(right),
                ..
            } if destination == "a" && left == "b" && right == "a"
        ));
        assert!(matches!(
            &program.statements[1],
            Statement::MatArithmetic {
                operation: MatArithmeticOperation::Add,
                destination,
                left: None,
                right: MatArithmeticOperand::Matrix(right),
                ..
            } if destination == "a" && right == "b"
        ));
        assert!(matches!(
            &program.statements[2],
            Statement::MatArithmetic {
                operation: MatArithmeticOperation::Add,
                destination,
                left: None,
                right: MatArithmeticOperand::Scalar(scalar),
                ..
            } if destination == "a" && scalar == "x"
        ));
        assert!(matches!(
            &program.statements[3],
            Statement::MatArithmetic {
                operation: MatArithmeticOperation::Sub,
                destination,
                left: Some(left),
                right: MatArithmeticOperand::Matrix(right),
                ..
            } if destination == "b" && left == "a" && right == "b"
        ));
        assert!(matches!(
            &program.statements[4],
            Statement::MatArithmetic {
                operation: MatArithmeticOperation::Sub,
                destination,
                left: None,
                right: MatArithmeticOperand::Matrix(right),
                ..
            } if destination == "b" && right == "a"
        ));
        assert!(matches!(
            &program.statements[5],
            Statement::MatArithmetic {
                operation: MatArithmeticOperation::Sub,
                destination,
                left: None,
                right: MatArithmeticOperand::Scalar(scalar),
                ..
            } if destination == "b" && scalar == "y"
        ));
    }

    #[test]
    fn rejects_unsupported_mat_arithmetic_operators() {
        for source in [
            "MAT ADD a()=b()-a()\n",
            "MAT SUB a()=b()+a()\n",
            "MAT ADD a(),7\n",
        ] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("invalid-mat-arithmetic.bas", source);
            assert!(parse(source_id, source).is_err(), "accepted: {source:?}");
        }
    }

    #[test]
    fn rejects_unsupported_mat_base_offsets_and_transpose_shapes() {
        for source in ["MAT BASE 2\n", "MAT BASE\n", "MAT TRANS a()=b(),c()\n"] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("invalid-mat.bas", source);
            assert!(parse(source_id, source).is_err(), "accepted: {source:?}");
        }
    }

    #[test]
    fn rejects_malformed_matrix_statement_shapes() {
        for source_text in [
            "MAT ONE a(3)\n",
            "MAT SET a()\n",
            "MAT SET a()=7,8\n",
            "MAT INV a()\n",
            "MAT INV b()=a()\n",
            "MAT INV a()=b()\n",
        ] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("malformed-matrix.bas", source_text);
            if source_text.starts_with("DIM ") {
                assert!(
                    parse(source_id, source_text).is_ok(),
                    "verified vector declaration was rejected: {source_text:?}"
                );
            } else {
                assert!(
                    parse(source_id, source_text).is_err(),
                    "accepted malformed matrix statement: {source_text:?}"
                );
            }
        }
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
        use super::{TriviaKind, lex_lossless};
        let source_text = "x = FACT(5)\r\nPRINT x, y\t\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("lossless.bas", source_text);
        let lexed = lex_lossless(source_id, source_text).expect("supported tokens should lex");

        assert!(
            lexed
                .trivia
                .iter()
                .all(|item| item.kind == TriviaKind::HorizontalWhitespace)
        );
        let mut spans: Vec<_> = lexed
            .tokens
            .iter()
            .filter(|token| !matches!(&token.kind, super::TokenKind::End))
            .map(|token| token.span)
            .chain(lexed.trivia.iter().map(|item| item.span))
            .collect();
        spans.sort_by_key(|span| span.start);

        let reconstructed = spans
            .into_iter()
            .map(|span| &source_text[span.start..span.end])
            .collect::<String>();
        assert_eq!(reconstructed, source_text);
    }

    #[test]
    fn lossless_lexer_retains_original_identifier_spelling() {
        use super::{TokenKind, lex_lossless};
        let source_text = "print FACT(1)\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("spelling.bas", source_text);
        let lexed = lex_lossless(source_id, source_text).expect("identifier candidates should lex");

        assert!(matches!(&lexed.tokens[0].kind, TokenKind::Identifier(name) if name == "print"));
        assert_eq!(
            &source_text[lexed.tokens[0].span.start..lexed.tokens[0].span.end],
            "print"
        );
    }

    #[test]
    fn parse_document_keeps_lossless_syntax_alongside_the_ast() {
        use super::TokenKind;
        let source_text = "x = FACT(5)\r\nPRINT x,y\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("parsed-document.bas", source_text);
        let document =
            parse_document(source_id, source_text).expect("supported source should parse");

        assert_eq!(document.program.statements.len(), 2);
        let mut spans: Vec<_> = document
            .syntax
            .tokens
            .iter()
            .filter(|token| !matches!(&token.kind, TokenKind::End))
            .map(|token| token.span)
            .chain(document.syntax.trivia.iter().map(|item| item.span))
            .collect();
        spans.sort_by_key(|span| span.start);

        let reconstructed = spans
            .into_iter()
            .map(|span| &source_text[span.start..span.end])
            .collect::<String>();
        assert_eq!(reconstructed, source_text);
    }
    // Evidence: docs/research/amiga-gfa-basic-35-lexer-evidence.md §2.1–2.2;
    // AM-INTERPRETER-35 printed pp. 13-34–13-35. `_DATA=` is `_DATA` + `=`.
    #[test]
    fn lexes_amiga_35_addendum_special_names_without_generalizing_identifiers() {
        use super::{TokenKind, lex_lossless};

        let mut source_text = String::from("_DATA=1\n");
        let mut expected = vec!["_DATA".to_owned()];
        for digit in '0'..='9' {
            source_text.push_str(&format!("_{digit}=1\n"));
            expected.push(format!("_{digit}"));
        }

        let mut sources = SourceMap::new();
        let source_id = sources.add_source("amiga-35-special-names.bas", source_text.as_str());
        let lexed = lex_lossless(source_id, &source_text).expect("3.5 addendum names should lex");
        let names: Vec<_> = lexed
            .tokens
            .iter()
            .filter_map(|token| match &token.kind {
                TokenKind::Identifier(name) => Some(name.clone()),
                _ => None,
            })
            .collect();

        assert_eq!(names, expected);
        let assignment_count = lexed
            .tokens
            .iter()
            .filter(|token| matches!(token.kind, TokenKind::Equals))
            .count();
        assert_eq!(assignment_count, expected.len());

        for token in lexed.tokens.iter().filter(
            |token| matches!(&token.kind, TokenKind::Identifier(name) if name.starts_with('_')),
        ) {
            assert!(lexed.tokens.iter().any(|next| {
                matches!(next.kind, TokenKind::Equals) && next.span.start == token.span.end
            }));
            assert_eq!(
                &source_text[token.span.start..token.span.end],
                match &token.kind {
                    TokenKind::Identifier(name) => name.as_str(),
                    _ => unreachable!(),
                }
            );
        }

        let mut spans: Vec<_> = lexed
            .tokens
            .iter()
            .filter(|token| !matches!(&token.kind, TokenKind::End))
            .map(|token| token.span)
            .chain(lexed.trivia.iter().map(|item| item.span))
            .collect();
        spans.sort_by_key(|span| span.start);
        let reconstructed = spans
            .into_iter()
            .map(|span| &source_text[span.start..span.end])
            .collect::<String>();
        assert_eq!(reconstructed, source_text);
    }

    // A documented special name must not be accepted as a prefix when
    // followed by source text that is not a known token boundary. This is a
    // conservative rejection, not an extension of the 3.5 identifier grammar.
    #[test]
    fn special_names_require_a_boundary_from_the_supported_token_subset() {
        use super::{TokenKind, lex_lossless};

        for source_text in ["_DATA.", "_DATA\"quoted\"", "_0:", "_9?"] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("special-name-adjacency.lst", source_text);
            let errors = lex_lossless(source_id, source_text).expect_err(
                "unsupported adjacent text must not follow a valid special-name prefix",
            );
            assert_eq!(errors[0].code, "GFA-LEX-0001");
            assert_eq!(errors[0].primary_span.unwrap().start, 0);
        }

        for source_text in ["_DATA=", "(_0)", "_9,"] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("special-name-boundary.lst", source_text);
            let lexed = lex_lossless(source_id, source_text)
                .expect("documented names followed by supported boundaries should lex");
            assert!(lexed.tokens.iter().any(|token| matches!(
                &token.kind,
                TokenKind::Identifier(name) if name == "_DATA" || name == "_0" || name == "_9"
            )));
        }
    }

    // Token-boundary coverage only; this does not claim expression grammar support.
    #[test]
    fn data_pointer_setter_keeps_identifier_and_equals_as_separate_tokens() {
        use super::{TokenKind, lex_lossless};

        let source_text = "_DATA=42\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("data-pointer-setter.bas", source_text);
        let lexed = lex_lossless(source_id, source_text).expect("documented 3.5 form should lex");

        assert!(matches!(&lexed.tokens[0].kind, TokenKind::Identifier(name) if name == "_DATA"));
        assert_eq!(lexed.tokens[0].span.start, 0);
        assert_eq!(lexed.tokens[0].span.end, 5);
        assert!(matches!(lexed.tokens[1].kind, TokenKind::Equals));
        assert_eq!(lexed.tokens[1].span.start, 5);
        assert_eq!(lexed.tokens[1].span.end, 6);
        assert!(matches!(lexed.tokens[2].kind, TokenKind::Integer(42)));
        assert_eq!(lexed.tokens[2].span.start, 6);
        assert_eq!(lexed.tokens[2].span.end, 8);
    }
    #[test]
    fn special_names_are_recognized_at_eof_and_before_existing_delimiters() {
        use super::{TokenKind, lex_lossless};

        let mut spellings = vec!["_DATA".to_owned()];
        spellings.extend(('0'..='9').map(|digit| format!("_{digit}")));

        for spelling in spellings {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("special-name-eof.bas", spelling.as_str());
            let lexed = lex_lossless(source_id, &spelling)
                .expect("documented Amiga 3.5 special name should lex at EOF");

            assert!(matches!(
                &lexed.tokens[0].kind,
                TokenKind::Identifier(name) if name == &spelling
            ));
            assert_eq!(lexed.tokens[0].span.start, 0);
            assert_eq!(lexed.tokens[0].span.end, spelling.len());
            assert!(matches!(lexed.tokens[1].kind, TokenKind::End));
            assert_eq!(lexed.tokens[1].span.start, spelling.len());
        }

        let source_text = "(_DATA,_0,_9)";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("special-name-delimiters.bas", source_text);
        let lexed = lex_lossless(source_id, source_text)
            .expect("documented names should remain separate from existing punctuation");

        let names: Vec<_> = lexed
            .tokens
            .iter()
            .filter_map(|token| match &token.kind {
                TokenKind::Identifier(name) => Some(name.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(names, ["_DATA", "_0", "_9"]);
        assert!(matches!(lexed.tokens[0].kind, TokenKind::LeftParen));
        assert!(matches!(lexed.tokens[2].kind, TokenKind::Comma));
        assert!(matches!(lexed.tokens[4].kind, TokenKind::Comma));
        assert!(matches!(lexed.tokens[6].kind, TokenKind::RightParen));

        let mut spans: Vec<_> = lexed
            .tokens
            .iter()
            .filter(|token| !matches!(&token.kind, TokenKind::End))
            .map(|token| token.span)
            .chain(lexed.trivia.iter().map(|item| item.span))
            .collect();
        spans.sort_by_key(|span| span.start);
        let reconstructed = spans
            .into_iter()
            .map(|span| &source_text[span.start..span.end])
            .collect::<String>();
        assert_eq!(reconstructed, source_text);
    }

    #[test]
    fn unsupported_unicode_characters_keep_exact_utf8_byte_spans() {
        use super::lex_lossless;

        // Non-ASCII identifiers are not accepted without a verified 3.5 rule.
        // Their original UTF-8 spelling must nevertheless remain addressable
        // through byte-based diagnostics for editor/source-map consumers.
        let source_text = "PRINT ž漢\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("unicode-diagnostic.lst", source_text);
        let errors = lex_lossless(source_id, source_text)
            .expect_err("unverified non-ASCII identifier characters stay unsupported");

        assert_eq!(errors.len(), 2);
        let spans: Vec<_> = errors
            .iter()
            .map(|error| error.primary_span.expect("lexical error has a span"))
            .collect();
        assert_eq!(
            spans
                .iter()
                .map(|span| &source_text[span.start..span.end])
                .collect::<Vec<_>>(),
            ["ž", "漢"]
        );
        assert_eq!(spans[0].end - spans[0].start, "ž".len());
        assert_eq!(spans[1].end - spans[1].start, "漢".len());
        assert_eq!(spans[1].start, spans[0].end);
    }

    // Negative boundary cases: the 3.5 addendum lists only `_DATA` and `_0`…`_9`;
    // it does not authorize a general underscore-initial identifier rule.
    #[test]
    fn rejects_unverified_underscore_identifier_spellings() {
        use super::lex_lossless;

        for source_text in [
            "_data=1\n",
            "_DATA2=1\n",
            "_DATAx=1\n",
            "_DATA_=1\n",
            "_10=1\n",
            "_0x=1\n",
            "_1_=1\n",
            "_foo=1\n",
        ] {
            let mut sources = SourceMap::new();
            let source_id = sources.add_source("unverified-name.bas", source_text);
            let errors = lex_lossless(source_id, source_text)
                .expect_err("only exact 3.5 addendum spellings are accepted");
            assert_eq!(errors[0].code, "GFA-LEX-0001");
            assert!(errors[0].message.contains("underscore-prefixed name"));
            assert_eq!(errors[0].primary_span.unwrap().start, 0);
        }
    }
    // These delimiters are already part of the implemented token subset.
    // This checks only that exact 3.5 special-name spellings are not split
    // differently at those existing boundaries; it adds no lexical forms.
    #[test]
    fn special_names_are_separated_at_every_existing_lexer_boundary() {
        use super::{TokenKind, lex_lossless};
        use crate::source::SourceSpan;

        let boundaries = [
            " ", "\t", "\r", "\n", "+", "-", "#", "*", "=", "(", ")", ",",
        ];
        for boundary in boundaries {
            for spelling in ["_DATA", "_0", "_9"] {
                let source_text = format!("{spelling}{boundary}");
                let mut sources = SourceMap::new();
                let source_id =
                    sources.add_source("special-name-boundaries.lst", source_text.as_str());
                let lexed = lex_lossless(source_id, &source_text).unwrap_or_else(|errors| {
                    panic!("{spelling:?} before {boundary:?}: {errors:?}")
                });

                assert!(matches!(
                    &lexed.tokens[0].kind,
                    TokenKind::Identifier(name) if name == spelling
                ));
                assert_eq!(
                    lexed.tokens[0].span,
                    SourceSpan::new(source_id, 0, spelling.len())
                );
                assert_eq!(
                    &source_text[lexed.tokens[0].span.start..lexed.tokens[0].span.end],
                    spelling
                );
            }
        }
    }
}
