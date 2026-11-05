//! Dialect-aware lexical analysis.
//!
//! The initial lexer deliberately implements only source forms supported by
//! the Amiga GFA BASIC 3.0 research baseline. It does not reserve command names:
//! the reference explicitly permits variables whose names match commands.

use crate::diagnostics::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::source::{SourceId, SourceSpan};

/// The source dialect selected for lexical analysis.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SourceDialect {
    /// Commodore Amiga GFA BASIC interpreter 3.0.
    AmigaGfaBasic30,
}

/// A lexical token. All spans use UTF-8 byte offsets into the original source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
}

/// Token categories supported by the initial Amiga 3.0 lexer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Identifier { text: String, suffix: Option<char> },
    Number(String),
    StringLiteral(String),
    Comment(String),
    Newline,
    Comma,
    Semicolon,
    Colon,
    LeftParen,
    RightParen,
    Operator(String),
    EndOfFile,
}

/// Result of lexing one source file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LexOutput {
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Lexes UTF-8 source using an explicitly selected source dialect.
///
/// Command spellings remain identifiers because Amiga GFA BASIC 3.0 does not
/// have reserved words. The lexer preserves original spelling and byte spans.
pub fn lex(source: SourceId, text: &str, dialect: SourceDialect) -> LexOutput {
    match dialect {
        SourceDialect::AmigaGfaBasic30 => Lexer::new(source, text).run(),
    }
}

struct Lexer<'a> {
    source: SourceId,
    text: &'a str,
    bytes: &'a [u8],
    offset: usize,
    output: LexOutput,
}

impl<'a> Lexer<'a> {
    fn new(source: SourceId, text: &'a str) -> Self {
        Self {
            source,
            text,
            bytes: text.as_bytes(),
            offset: 0,
            output: LexOutput::default(),
        }
    }

    fn run(mut self) -> LexOutput {
        while self.offset < self.bytes.len() {
            let byte = self.bytes[self.offset];
            match byte {
                b' ' | b'\t' | 0x0c => self.offset += 1,
                b'\r' | b'\n' => self.newline(),
                b'!' => self.comment(),
                b'"' => self.string(),
                b'0'..=b'9' => self.number(),
                b'A'..=b'Z' | b'a'..=b'z' => self.identifier(),
                b',' => self.single(TokenKind::Comma),
                b';' => self.single(TokenKind::Semicolon),
                b':' => self.single(TokenKind::Colon),
                b'(' => self.single(TokenKind::LeftParen),
                b')' => self.single(TokenKind::RightParen),
                b'+' | b'-' | b'*' | b'/' | b'^' | b'=' | b'<' | b'>' => self.operator(),
                _ => self.invalid_character(),
            }
        }

        let end = self.offset;
        self.output.tokens.push(Token {
            kind: TokenKind::EndOfFile,
            span: SourceSpan::new(self.source, end, end),
        });
        self.output
    }

    fn push(&mut self, kind: TokenKind, start: usize) {
        self.output.tokens.push(Token {
            kind,
            span: SourceSpan::new(self.source, start, self.offset),
        });
    }

    fn single(&mut self, kind: TokenKind) {
        let start = self.offset;
        self.offset += 1;
        self.push(kind, start);
    }

    fn newline(&mut self) {
        let start = self.offset;
        if self.bytes[self.offset] == b'\r' {
            self.offset += 1;
            if self.bytes.get(self.offset) == Some(&b'\n') {
                self.offset += 1;
            }
        } else {
            self.offset += 1;
        }
        self.push(TokenKind::Newline, start);
    }

    fn comment(&mut self) {
        let start = self.offset;
        self.offset += 1;
        let content_start = self.offset;
        while self.offset < self.bytes.len()
            && self.bytes[self.offset] != b'\r'
            && self.bytes[self.offset] != b'\n'
        {
            let ch = self.text[self.offset..]
                .chars()
                .next()
                .expect("valid UTF-8");
            self.offset += ch.len_utf8();
        }
        self.push(
            TokenKind::Comment(self.text[content_start..self.offset].to_owned()),
            start,
        );
    }

    fn string(&mut self) {
        let start = self.offset;
        self.offset += 1;
        let content_start = self.offset;
        while self.offset < self.bytes.len()
            && self.bytes[self.offset] != b'"'
            && self.bytes[self.offset] != b'\r'
            && self.bytes[self.offset] != b'\n'
        {
            let ch = self.text[self.offset..]
                .chars()
                .next()
                .expect("valid UTF-8");
            self.offset += ch.len_utf8();
        }

        if self.bytes.get(self.offset) == Some(&b'"') {
            let value = self.text[content_start..self.offset].to_owned();
            self.offset += 1;
            self.push(TokenKind::StringLiteral(value), start);
        } else {
            self.error(
                "GFA-LEX-0002",
                "unterminated string literal",
                start,
                self.offset,
            );
        }
    }

    fn number(&mut self) {
        let start = self.offset;
        while self.offset < self.bytes.len() && self.bytes[self.offset].is_ascii_digit() {
            self.offset += 1;
        }

        if self.bytes.get(self.offset) == Some(&b'.')
            && self
                .bytes
                .get(self.offset + 1)
                .is_some_and(u8::is_ascii_digit)
        {
            self.offset += 1;
            while self.offset < self.bytes.len() && self.bytes[self.offset].is_ascii_digit() {
                self.offset += 1;
            }
        }

        self.push(
            TokenKind::Number(self.text[start..self.offset].to_owned()),
            start,
        );
    }

    fn identifier(&mut self) {
        let start = self.offset;
        self.offset += 1;
        while self.offset < self.bytes.len() {
            let byte = self.bytes[self.offset];
            if byte.is_ascii_alphanumeric() || byte == b'.' || byte == b'_' {
                self.offset += 1;
            } else {
                break;
            }
        }

        let suffix = self.bytes.get(self.offset).copied().and_then(|byte| {
            matches!(byte, b'%' | b'|' | b'&' | b'!' | b'$' | b'#').then_some(byte as char)
        });
        if suffix.is_some() {
            self.offset += 1;
        }

        let end_name = if suffix.is_some() {
            self.offset - 1
        } else {
            self.offset
        };
        self.push(
            TokenKind::Identifier {
                text: self.text[start..end_name].to_owned(),
                suffix,
            },
            start,
        );
    }

    fn operator(&mut self) {
        let start = self.offset;
        let first = self.bytes[self.offset];
        self.offset += 1;
        if matches!(first, b'<' | b'>')
            && self
                .bytes
                .get(self.offset)
                .is_some_and(|byte| *byte == b'=' || (first == b'<' && *byte == b'>'))
        {
            self.offset += 1;
        }
        self.push(
            TokenKind::Operator(self.text[start..self.offset].to_owned()),
            start,
        );
    }

    fn invalid_character(&mut self) {
        let start = self.offset;
        let ch = self.text[self.offset..]
            .chars()
            .next()
            .expect("valid UTF-8");
        self.offset += ch.len_utf8();
        self.error(
            "GFA-LEX-0001",
            format!("invalid character {ch:?}"),
            start,
            self.offset,
        );
    }

    fn error(&mut self, code: &str, message: impl Into<String>, start: usize, end: usize) {
        self.output.diagnostics.push(
            Diagnostic::new(
                code,
                DiagnosticSeverity::Error,
                DiagnosticCategory::Lexical,
                message,
            )
            .with_primary_span(SourceSpan::new(self.source, start, end))
            .with_reference("docs/reference/amiga-gfa-basic-3.0.md"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{SourceDialect, TokenKind, lex};
    use crate::source::SourceMap;

    fn lex_text(text: &str) -> super::LexOutput {
        let mut sources = SourceMap::new();
        let source = sources.add_source("test.gfa", text);
        lex(source, text, SourceDialect::AmigaGfaBasic30)
    }

    #[test]
    fn command_names_are_identifiers_not_reserved_keywords() {
        let output = lex_text("PRINT TIMER DATES");
        assert!(output.diagnostics.is_empty());
        assert!(matches!(
            &output.tokens[0].kind,
            TokenKind::Identifier { text, .. } if text == "PRINT"
        ));
        assert!(matches!(
            &output.tokens[1].kind,
            TokenKind::Identifier { text, .. } if text == "TIMER"
        ));
        assert!(matches!(
            &output.tokens[2].kind,
            TokenKind::Identifier { text, .. } if text == "DATES"
        ));
    }

    #[test]
    fn recognizes_documented_type_suffixes_and_identifier_characters() {
        let output = lex_text("value% byte| word& flag! text$ real# name_2.part");
        let suffixes: Vec<_> = output
            .tokens
            .iter()
            .filter_map(|token| {
                if let TokenKind::Identifier { suffix, .. } = &token.kind {
                    Some(*suffix)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(
            suffixes,
            vec![
                Some('%'),
                Some('|'),
                Some('&'),
                Some('!'),
                Some('$'),
                Some('#'),
                None
            ]
        );
        assert!(matches!(
            &output.tokens[6].kind,
            TokenKind::Identifier { text, .. } if text == "name_2.part"
        ));
    }

    #[test]
    fn handles_strings_comments_labels_and_line_endings() {
        let output = lex_text("start: PRINT \"hello\" ! note\r\nNEXT");
        assert!(output.diagnostics.is_empty());
        assert!(matches!(
            output.tokens[0].kind,
            TokenKind::Identifier { .. }
        ));
        assert!(matches!(output.tokens[1].kind, TokenKind::Colon));
        assert!(matches!(
            output.tokens[3].kind,
            TokenKind::StringLiteral(ref value) if value == "hello"
        ));
        assert!(matches!(
            output.tokens[4].kind,
            TokenKind::Comment(ref value) if value == " note"
        ));
        assert!(matches!(output.tokens[5].kind, TokenKind::Newline));
        assert!(matches!(
            output.tokens[6].kind,
            TokenKind::Identifier { .. }
        ));
    }

    #[test]
    fn reports_unterminated_strings_and_invalid_characters_with_spans() {
        let output = lex_text("PRINT \"oops\nPRINT @");
        assert_eq!(output.diagnostics.len(), 2);
        assert_eq!(output.diagnostics[0].code, "GFA-LEX-0002");
        assert_eq!(output.diagnostics[0].primary_span.unwrap().start, 6);
        assert_eq!(output.diagnostics[1].code, "GFA-LEX-0001");
        assert_eq!(output.diagnostics[1].primary_span.unwrap().start, 18);
    }

    #[test]
    fn comments_preserve_unicode_and_stop_before_classic_newline() {
        let output = lex_text("PRINT 1 ! Grüße\rPRINT 2");
        assert!(output.diagnostics.is_empty());
        let comment = output
            .tokens
            .iter()
            .find(|token| matches!(token.kind, TokenKind::Comment(_)))
            .unwrap();
        assert!(matches!(
            &comment.kind,
            TokenKind::Comment(value) if value == " Grüße"
        ));
        assert_eq!(comment.span.start, 8);
        assert_eq!(comment.span.end, 17);
        assert!(
            output
                .tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Newline))
        );
    }

    #[test]
    fn lexical_diagnostics_render_original_source_locations() {
        let text = "PRINT \"é\"\n@";
        let mut sources = SourceMap::new();
        let source = sources.add_source("examples/bad.gfa", text);
        let output = lex(source, text, SourceDialect::AmigaGfaBasic30);
        assert_eq!(output.diagnostics.len(), 1);
        let diagnostic = &output.diagnostics[0];
        assert_eq!(diagnostic.primary_span.unwrap().start, 11);
        assert!(
            diagnostic
                .render(&sources)
                .starts_with("examples/bad.gfa:2:1: error[GFA-LEX-0001]: invalid character")
        );
    }

    #[test]
    fn preserves_utf8_byte_offsets() {
        let output = lex_text("PRINT \"é\"");
        let string = output
            .tokens
            .iter()
            .find(|token| matches!(token.kind, TokenKind::StringLiteral(_)))
            .unwrap();
        assert_eq!(string.span.start, 6);
        assert_eq!(string.span.end, 10);
    }
}
