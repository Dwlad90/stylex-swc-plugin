use super::extract_css_value;

#[test]
fn extract_css_value_keeps_full_composite_url_rule_value() {
  let css = r#"*{background:url("asset.png") no-repeat center/cover}"#;
  let value = extract_css_value(css);

  assert_eq!(value, r#"url("asset.png") no-repeat center/cover"#);
}

#[test]
fn extract_css_value_keeps_plain_composite_url_value() {
  let css = r#"url("asset.png") no-repeat center/cover"#;
  let value = extract_css_value(css);

  assert_eq!(value, css);
}

#[test]
fn extract_css_value_keeps_data_url_semicolon_content() {
  let css = r#"*{background-image:url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAMUlEQVQ4T2NkYGAQYcAP3uCTZhw1gGGYhAGBZIA/nYDCgBDAm9BGDWAAJyRCgLaBCAAgXwixzAS0pgAAAABJRU5ErkJggg==")}"#;
  let value = extract_css_value(css);

  assert_eq!(
    value,
    r#"url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAMUlEQVQ4T2NkYGAQYcAP3uCTZhw1gGGYhAGBZIA/nYDCgBDAm9BGDWAAJyRCgLaBCAAgXwixzAS0pgAAAABJRU5ErkJggg==")"#
  );
}
