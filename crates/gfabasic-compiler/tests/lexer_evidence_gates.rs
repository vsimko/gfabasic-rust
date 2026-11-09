//! Regression tests for evidence-gated Amiga 3.5 lexer diagnostics.
//!
//! The supplied 3.5 material does not settle source encoding or a general
//! non-ASCII identifier grammar. Until that evidence exists, non-ASCII input
//! must remain unsupported rather than being silently normalized or accepted.
//! These tests protect the byte-based diagnostic spans without choosing a
//! language-level encoding rule.

use gfabasic_compiler::frontend::lex_lossless;
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
fn unsupported_multibyte_character_does_not_shift_later_byte_offsets() {
    let source_text = "x=1\né=2\ny=3";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("mixed-language.lst", source_text);

    let diagnostics =
        lex_lossless(source_id, source_text).expect_err("non-ASCII grammar is not verified");

    let span = diagnostics[0]
        .primary_span
        .expect("lexical diagnostic should have a primary span");
    assert_eq!((span.start, span.end), (4, 6));
    assert_eq!(sources.slice(span), Some("é"));
    assert_eq!(&source_text[span.end..], "=2\ny=3");
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
