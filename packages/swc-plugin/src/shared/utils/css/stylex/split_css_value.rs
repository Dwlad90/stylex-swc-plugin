use super::parse_css::parse_css;

pub(crate) fn split_value_required(str: Option<&str>) -> (String, String, String, String) {
  let values = split_value(str);

  let top = values.0;
  let right = values.1.unwrap_or(top.clone());
  let bottom = values.2.unwrap_or(top.clone());
  let left = values.3.unwrap_or(right.clone());

  (top, right, bottom, left)
}

pub(crate) fn split_value(
  str: Option<&str>,
) -> (String, Option<String>, Option<String>, Option<String>) {
  let nodes = parse_css(str.unwrap_or(""));

  let top = nodes.first().cloned().unwrap_or("".to_string());
  let right = nodes.get(1).cloned();
  let bottom = nodes.get(2).cloned();
  let left = nodes.get(3).cloned();

  (top, right, bottom, left)
}
