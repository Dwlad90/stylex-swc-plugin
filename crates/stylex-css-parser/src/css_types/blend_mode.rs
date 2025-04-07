use crate::parser::Parser;

pub fn blend_mode<'a>() -> Parser<'a, String> {
  Parser::one_of(vec![
    Parser::<'a, String>::string("normal"),
    Parser::<'a, String>::string("multiply"),
    Parser::<'a, String>::string("screen"),
    Parser::<'a, String>::string("overlay"),
    Parser::<'a, String>::string("darken"),
    Parser::<'a, String>::string("lighten"),
    Parser::<'a, String>::string("color-dodge"),
    Parser::<'a, String>::string("color-burn"),
    Parser::<'a, String>::string("hard-light"),
    Parser::<'a, String>::string("soft-light"),
    Parser::<'a, String>::string("difference"),
    Parser::<'a, String>::string("exclusion"),
    Parser::<'a, String>::string("hue"),
    Parser::<'a, String>::string("saturation"),
    Parser::<'a, String>::string("color"),
    Parser::<'a, String>::string("luminosity"),
  ])
}
