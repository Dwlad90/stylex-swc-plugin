use crate::parser::Parser;

pub fn calc_constant<'a>() -> Parser<'a, String> {
  Parser::one_of(vec![
    Parser::<'a, String>::string("pi"),
    Parser::<'a, String>::string("e"),
    Parser::<'a, String>::string("infinity"),
    Parser::<'a, String>::string("-infinity"),
    Parser::<'a, String>::string("NaN"),
  ])
}
