use crate::parser::Parser;

pub fn number<'a>() -> Parser<'a, f32> {
  Parser::<'a, f32>::float()
}
