mod ast;
mod subst;
mod sld;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammer); // synthesized by LALRPOP

#[test]
fn grammer() {
    assert!(grammer::TermParser::new().parse("(x)").is_ok());
    assert!(grammer::TermParser::new().parse("f(x, y, Z)").is_ok());

    assert!(grammer::TermParser::new().parse("22").is_err());
    assert!(grammer::TermParser::new().parse("(22)").is_err());
    assert!(grammer::TermParser::new().parse("((((22))))").is_err());
    assert!(grammer::TermParser::new().parse("((22)").is_err());
}

fn main() {}
