pub fn parse_css(css_string: &str) -> Vec<String> {
  stylex_css_parser::value_parser::parse_css(css_string)
}

#[cfg(test)]
#[path = "../tests/values_parser_tests.rs"]
mod tests;
