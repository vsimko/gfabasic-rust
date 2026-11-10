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
fn mat_inv_operand_roles_are_positional_not_name_dependent() {
    let source = "MAT INV a()=b()\n";
    let mut sources = SourceMap::new();
    let source_id = sources.add_source("mat-inv-positional-operands.LST", source);

    let document = parse_document(source_id, source)
        .expect("both supported matrix identifiers can occupy either positional role");
    let Statement::MatInv {
        destination,
        source: matrix_source,
        destination_span,
        source_span,
        ..
    } = &document.program.statements[0]
    else {
        panic!("expected MAT INV AST node");
    };

    // The first operand is the destination even when its identifier is "a".
    // The parser cannot reject this token sequence as "reversed": the
    // documented grammar assigns roles by position, not by variable spelling.
    assert_eq!(destination, "a");
    assert_eq!(matrix_source, "b");
    assert_eq!(sources.slice(*destination_span), Some("a()"));
    assert_eq!(sources.slice(*source_span), Some("b()"));
}
