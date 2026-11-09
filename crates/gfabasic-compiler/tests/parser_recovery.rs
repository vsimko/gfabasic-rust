use gfabasic_compiler::frontend::parse;
use gfabasic_compiler::source::SourceMap;

#[test]
fn parser_reports_independent_line_errors_after_synchronizing() {
    let source = "x=FACT(0\nLET x=1\nz=FACT(2)\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("parser-recovery.bas", source);

    let diagnostics = parse(source_id, source)
        .expect_err("malformed and unsupported statements must remain errors");

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].code, "GFA-PARSE-0001");
    assert_eq!(diagnostics[0].category.as_str(), "syntax");
    assert_eq!(
        diagnostics[0].primary_span.expect("syntax error has a span").start,
        8,
        "the missing ')' is reported at the first line boundary"
    );

    assert_eq!(diagnostics[1].code, "GFA-SUPPORT-0001");
    assert_eq!(diagnostics[1].category.as_str(), "unsupported");
    assert_eq!(
        diagnostics[1].primary_span.expect("unsupported statement has a span").start,
        9,
        "parser recovery must continue at the next source line"
    );
}
