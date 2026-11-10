use gfabasic_compiler::frontend::{Statement, parse, parse_document};
use gfabasic_compiler::source::SourceMap;

#[test]
fn mat_inv_accepts_documented_destination_first_order_and_preserves_spans() {
    let source = "MAT INV b()=a()\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("mat-inv-destination-first.LST", source);

    let document = parse_document(source_id, source)
        .expect("the documented destination-first MAT INV form must parse");
    let Statement::MatInv {
        destination,
        destination_span,
        source: matrix_source,
        source_span,
        span,
    } = &document.program.statements[0]
    else {
        panic!("expected MAT INV AST node");
    };

    assert_eq!(destination, "b");
    assert_eq!(matrix_source, "a");
    assert_eq!(sources.slice(*destination_span), Some("b()"));
    assert_eq!(sources.slice(*source_span), Some("a()"));
    assert_eq!(sources.slice(*span), Some("MAT INV b()=a()"));
}

#[test]
fn mat_inv_rejects_reversed_operand_order_in_the_verified_subset() {
    let source = "MAT INV a()=b()\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("mat-inv-reversed.LST", source);

    let diagnostics = parse(source_id, source)
        .expect_err("the reversed form from the erroneous command summary must be rejected");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code, "GFA-SUPPORT-0001");
    let span = diagnostics[0]
        .primary_span
        .expect("unsupported operand order must retain a source span");
    assert_eq!(sources.slice(span), Some("a()"));
}
