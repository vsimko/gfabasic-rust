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
    PrintVariables {
        names: Vec<String>,
        span: SourceSpan,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum TokenKind {
    Identifier(String),
    Integer(i64),
    Equals,
    LeftParen,
    RightParen,
    Comma,
    Newline,
    End,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Token {
    kind: TokenKind,
    span: SourceSpan,
}

fn diagnostic(
    code: &str,
    category: DiagnosticCategory,
    message: impl Into<String>,
    span: SourceSpan,
) -> Diagnostic {
    Diagnostic::new(
        code,
        DiagnosticSeverity::Error,
        category,
        message,
    )
    .with_primary_span(span)
}

fn lex(source_id: SourceId, source: &str) -> Result<Vec<Token>, Vec<Diagnostic>> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();
    let mut offset = 0;

    while offset < bytes.len() {
        match bytes[offset] {
            b' ' | b'\t' => offset += 1,
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
                while bytes
                    .get(offset)
                    .is_some_and(|next| next.is_ascii_digit())
                {
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
                let character = source[offset..].chars().next().expect("valid UTF-8 boundary");
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
        Ok(tokens)
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

    fn error(&self, message: impl Into<String>, span: SourceSpan) -> Diagnostic {
        diagnostic(
            "GFA-PARSE-0001",
            DiagnosticCategory::Unsupported,
            message,
            span,
        )
    }

    fn parse_variable(&mut self) -> Result<(String, SourceSpan), Diagnostic> {
        let token = self.advance();
        match token.kind {
            TokenKind::Identifier(name) if matches!(name.as_str(), "x" | "y" | "z") => {
                Ok((name, token.span))
            }
            TokenKind::Identifier(name) => Err(self.error(
                format!("variable {name:?} is outside the current MVP subset (supported: x, y, z)"),
                token.span,
            )),
            _ => Err(self.error("expected one of the variables x, y, or z", token.span)),
        }
    }

    fn expect(&mut self, expected: TokenKind, description: &str) -> Result<Token, Diagnostic> {
        let token = self.current().clone();
        if std::mem::discriminant(&token.kind) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(self.error(format!("expected {description}"), token.span))
        }
    }

    fn parse_assignment(&mut self, name: String, start: SourceSpan) -> Result<Statement, Diagnostic> {
        self.expect(TokenKind::Equals, "'='")?;
        let function = self.advance();
        match function.kind {
            TokenKind::Identifier(name) if name == "FACT" => {}
            _ => return Err(self.error("only the FACT intrinsic is supported in this MVP", function.span)),
        }
        self.expect(TokenKind::LeftParen, "'(' after FACT")?;
        let argument = self.advance();
        let value = match argument.kind {
            TokenKind::Integer(value) => value,
            _ => return Err(self.error("FACT argument must be an integer literal in this MVP", argument.span)),
        };
        self.expect(TokenKind::RightParen, "')' after FACT argument")?;
        let end = self.tokens[self.cursor - 1].span;
        Ok(Statement::AssignFact {
            name,
            argument: value,
            span: SourceSpan::new(self.source_id, start.start, end.end),
        })
    }

    fn parse_print(&mut self, start: SourceSpan) -> Result<Statement, Diagnostic> {
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

    fn parse_statement(&mut self) -> Result<Statement, Diagnostic> {
        let first = self.advance();
        match first.kind {
            TokenKind::Identifier(name) if name == "PRINT" => self.parse_print(first.span),
            TokenKind::Identifier(name) if matches!(name.as_str(), "x" | "y" | "z") => {
                self.parse_assignment(name, first.span)
            }
            TokenKind::Identifier(name) => Err(self.error(
                format!("statement {name:?} is not supported by the current MVP frontend"),
                first.span,
            )),
            _ => Err(self.error("expected an assignment or PRINT statement", first.span)),
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
                        self.diagnostics.push(self.error(
                            "unexpected token after statement; statements must be on separate lines in this MVP",
                            token.span,
                        ));
                        self.synchronize();
                    } else {
                        self.skip_newlines();
                    }
                }
                Err(error) => {
                    self.diagnostics.push(error);
                    self.synchronize();
                }
            }
        }

        if statements.is_empty() && self.diagnostics.is_empty() {
            self.diagnostics.push(self.error(
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
    let tokens = lex(source_id, source)?;
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
    use super::{parse, Statement};
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
    fn rejects_unsupported_statement_instead_of_ignoring_it() {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("unsupported.bas", "LET x=1\n");

        let errors = parse(source_id, "LET x=1\n").expect_err("LET is outside the MVP");
        assert_eq!(errors[0].code, "GFA-PARSE-0001");
        assert!(errors[0].message.contains("not supported"));
        assert_eq!(errors[0].primary_span.unwrap().start, 0);
    }

    #[test]
    fn rejects_variables_outside_the_demonstrated_mvp_names() {
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("unknown-variable.bas", "count=FACT(1)\n");

        let errors = parse(source_id, "count=FACT(1)\n").expect_err("name is outside MVP");
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
    fn accepts_crlf_source_lines_without_changing_statement_spans() {
        let source_text = "x=FACT(0)\r\ny=FACT(1)\r\n";
        let mut sources = SourceMap::new();
        let source_id = sources.add_source("crlf.bas", source_text);

        let program = parse(source_id, source_text).expect("CRLF line endings should parse");
        assert_eq!(program.statements.len(), 2);
    }
}
