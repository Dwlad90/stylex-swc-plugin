use super::parser::parse_css;

pub fn split_value_required(strng: Option<&str>) -> (String, String, String, String) {
  let values = split_value(strng);

  let top = values.0;
  let right = values.1.unwrap_or(top.clone());
  let bottom = values.2.unwrap_or(top.clone());
  let left = values.3.unwrap_or(right.clone());

  (top, right, bottom, left)
}

pub fn split_value(
  value: Option<&str>,
) -> (String, Option<String>, Option<String>, Option<String>) {
  let nodes = parse_css(value.unwrap_or_default());

  let top = nodes.first().cloned().unwrap_or(String::default());
  let right = nodes.get(1).cloned();
  let bottom = nodes.get(2).cloned();
  let left = nodes.get(3).cloned();

  (top, right, bottom, left)
}
