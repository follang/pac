use crate::driver::Flavor;
use crate::parse;

#[test]
fn parse_api_constant_works() {
    let constant = parse::constant("42", Flavor::StdC11).expect("parsing constant");
    match constant {
        crate::ast::Constant::Integer(_) => {}
        other => panic!("expected integer constant, got {:?}", other),
    }
}

#[test]
fn parse_api_expression_works() {
    parse::expression("value + 1", Flavor::StdC11).expect("parsing expression");
}

#[test]
fn parse_api_declaration_works() {
    parse::declaration("int value;", Flavor::StdC11).expect("parsing declaration");
}

#[test]
fn parse_api_statement_works() {
    parse::statement("return 0;", Flavor::StdC11).expect("parsing statement");
}

#[test]
fn parse_api_translation_unit_works() {
    parse::translation_unit("typedef int counter_t;\nint main(void) { return 0; }\n", Flavor::StdC11)
        .expect("parsing translation unit");
}
