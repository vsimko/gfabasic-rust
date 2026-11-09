//! Regression tests for evidence-gated Amiga 3.5 lexer diagnostics.
//!
//! The supplied 3.5 material does not settle source encoding or a general
//! non-ASCII identifier grammar. Until that evidence exists, non-ASCII input
//! must remain unsupported rather than being silently normalized or accepted.
//! These tests protect the byte-based diagnostic spans without choosing a
//! language-level encoding rule.

use gfabasic_compiler::frontend::{TokenKind, lex_lossless};
use gfabasic_compiler::source::SourceMap;

#[test]
fn unsupported_multibyte_character_has_exact_utf8_byte_span() {
    let source_text = "x=1\nčíslo=2";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("unicode.lst", source_text);

    let diagnostics =
        lex_lossless(source_id, source_text).expect_err("non-ASCII grammar is not verified");

    let span = diagnostics[0]
        .primary_span
        .expect("lexical diagnostic should have a primary span");
    assert_eq!((span.start, span.end), (4, 6));
    assert_eq!(sources.slice(span), Some("č"));
}

#[test]
fn unsupported_multibyte_characters_keep_subsequent_diagnostic_offsets() {
    let source_text = "x=1\né=2\nø=3";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("mixed-language.lst", source_text);

    let diagnostics =
        lex_lossless(source_id, source_text).expect_err("non-ASCII grammar is not verified");

    let span = diagnostics[0]
        .primary_span
        .expect("lexical diagnostic should have a primary span");
    assert_eq!((span.start, span.end), (4, 6));
    assert_eq!(sources.slice(span), Some("é"));

    let later_span = diagnostics[1]
        .primary_span
        .expect("later lexical diagnostic should have a primary span");
    assert_eq!((later_span.start, later_span.end), (9, 11));
    assert_eq!(sources.slice(later_span), Some("ø"));
    assert!(diagnostics[1].message.contains("ø"));
}

#[test]
fn unsupported_unicode_is_not_replaced_or_case_normalized() {
    let source_text = "x=1\nŽluťoučký=2";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("original-spelling.lst", source_text);

    let diagnostics =
        lex_lossless(source_id, source_text).expect_err("Unicode identifier rule is unresolved");

    let span = diagnostics[0]
        .primary_span
        .expect("lexical diagnostic should have a primary span");
    assert_eq!(sources.slice(span), Some("Ž"));
    assert!(diagnostics[0].message.contains("Ž"));
}

#[test]
fn recognized_tokens_and_trivia_cover_the_original_source_without_gaps() {
    // CR, CRLF, and LF are exercised as the prototype's transport separators;
    // this test does not define the Amiga 3.5 .LST line-ending contract.
    let source_text = "x = 1\t+\r\n_DATA=2\rPRINT x";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("lossless.lst", source_text);

    let lexed = lex_lossless(source_id, source_text)
        .expect("all characters belong to the currently supported token/trivia subset");

    let mut spans = lexed
        .tokens
        .iter()
        .filter(|token| !matches!(token.kind, TokenKind::End))
        .map(|token| token.span)
        .chain(lexed.trivia.iter().map(|trivia| trivia.span))
        .map(|span| (span.start, span.end))
        .collect::<Vec<_>>();
    spans.sort_unstable();

    let mut covered_until = 0;
    for (start, end) in spans {
        assert_eq!(start, covered_until, "gap or overlap before byte {start}");
        assert!(end > start, "empty non-EOF span at byte {start}");
        covered_until = end;
    }
    assert_eq!(covered_until, source_text.len());
}
