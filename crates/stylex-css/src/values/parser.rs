pub fn format_ident(ident: &str) -> String {
  stylex_css_parser::value_parser::format_ident(ident)
}

pub fn _format_quoted_string(string: &str) -> String {
  stylex_css_parser::value_parser::format_quoted_string(string)
}

pub fn parse_css(css_string: &str) -> Vec<String> {
  stylex_css_parser::value_parser::parse_css(css_string)
}

fn join_css(nodes: &[String]) -> String {
  stylex_css_parser::value_parser::join_css(nodes)
}

#[cfg(test)]
#[path = "../tests/values_parser_tests.rs"]
mod tests;
