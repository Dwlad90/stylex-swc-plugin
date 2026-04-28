use indexmap::IndexMap;
use stylex_styleq::{
  COMPILED_KEY, StyleMap, StyleValue, StyleqInput, StyleqOptions, create_styleq, styleq,
};

fn compiled(entries: &[(&str, StyleValue)]) -> StyleqInput<StyleValue> {
  let mut style = IndexMap::new();
  style.insert(COMPILED_KEY.to_string(), StyleValue::Bool(true));

  for (key, value) in entries {
    style.insert((*key).to_string(), value.clone());
  }

  StyleqInput::Style(style)
}

fn inline(entries: &[(&str, StyleValue)]) -> StyleqInput<StyleValue> {
  StyleqInput::Style(
    entries
      .iter()
      .map(|(key, value)| ((*key).to_string(), value.clone()))
      .collect(),
  )
}

fn string(value: &str) -> StyleValue {
  StyleValue::string(value)
}

#[test]
fn combines_different_class_names() {
  let style = compiled(&[("a", string("aaa")), ("b", string("bbb"))]);

  assert_eq!(styleq(&[style]).class_name, "aaa bbb");
}

#[test]
fn combines_different_class_names_in_order() {
  let a = compiled(&[("a", string("a")), (":focus$aa", string("focus$aa"))]);
  let b = compiled(&[("b", string("b"))]);
  let c = compiled(&[("c", string("c")), (":focus$cc", string("focus$cc"))]);

  assert_eq!(
    styleq(&[StyleqInput::Nested(vec![a, b, c])]).class_name,
    "a focus$aa b c focus$cc"
  );
}

#[test]
fn dedupes_class_names_for_same_key() {
  let a = compiled(&[("backgroundColor", string("backgroundColor-a"))]);
  let b = compiled(&[("backgroundColor", string("backgroundColor-b"))]);
  let c = compiled(&[("backgroundColor", string("backgroundColor-c"))]);

  assert_eq!(
    styleq(&[StyleqInput::Nested(vec![a.clone(), b.clone()])]).class_name,
    "backgroundColor-b"
  );
  assert_eq!(
    styleq(&[StyleqInput::Nested(vec![c, a, b])]).class_name,
    "backgroundColor-b"
  );
}

#[test]
fn dedupes_class_names_with_null_value() {
  let a = compiled(&[("backgroundColor", string("backgroundColor-a"))]);
  let b = compiled(&[("backgroundColor", StyleValue::Null)]);

  assert_eq!(styleq(&[StyleqInput::Nested(vec![a, b])]).class_name, "");
}

#[test]
fn dedupes_inline_styles() {
  let result = styleq(&[StyleqInput::Nested(vec![
    inline(&[("a", string("a"))]),
    inline(&[("a", string("aa"))]),
  ])]);

  assert_eq!(
    result.inline_style,
    Some(StyleMap::from([("a".to_string(), string("aa"))]))
  );

  let result = styleq(&[StyleqInput::Nested(vec![
    inline(&[("a", string("a"))]),
    inline(&[("a", StyleValue::Null)]),
  ])]);

  assert_eq!(result.inline_style, None);
}

#[test]
fn preserves_order_of_inline_styles() {
  let result = styleq(&[StyleqInput::Nested(vec![
    inline(&[("font", string("inherit"))]),
    inline(&[("fontSize", StyleValue::Number(12))]),
  ])]);

  assert_eq!(
    result.inline_style,
    Some(StyleMap::from([
      ("font".to_string(), string("inherit")),
      ("fontSize".to_string(), StyleValue::Number(12)),
    ]))
  );
}

#[test]
fn dedupes_class_names_and_inline_styles() {
  let a = compiled(&[("a", string("a")), (":focus$a", string("focus$a"))]);
  let b = compiled(&[("b", string("b"))]);
  let inline_b = inline(&[("b", string("b")), ("bb", StyleValue::Null)]);

  let result = styleq(&[StyleqInput::Nested(vec![
    a.clone(),
    b.clone(),
    inline_b.clone(),
  ])]);
  assert_eq!(result.class_name, "a focus$a");
  assert_eq!(
    result.inline_style,
    Some(StyleMap::from([("b".to_string(), string("b"))]))
  );

  let result = styleq(&[StyleqInput::Nested(vec![a, inline_b, b])]);
  assert_eq!(result.class_name, "a focus$a b");
  assert_eq!(result.inline_style, None);
}

#[test]
fn disable_mix_dedupes_inline_styles() {
  let styleq_no_mix = create_styleq(StyleqOptions {
    disable_mix: true,
    ..Default::default()
  });

  let result = styleq_no_mix.styleq(&[StyleqInput::Nested(vec![
    inline(&[("a", string("a"))]),
    inline(&[("a", string("aa"))]),
  ])]);

  assert_eq!(
    result.inline_style,
    Some(StyleMap::from([("a".to_string(), string("aa"))]))
  );
}

#[test]
fn supports_generating_debug_strings() {
  let mut a = IndexMap::new();
  a.insert(COMPILED_KEY.to_string(), string("path/to/a:1"));
  a.insert("a".to_string(), string("aaa"));

  let mut b = IndexMap::new();
  b.insert(COMPILED_KEY.to_string(), string("path/to/b:2"));
  b.insert("b".to_string(), string("bbb"));

  let mut c = IndexMap::new();
  c.insert(COMPILED_KEY.to_string(), string("path/to/c:3"));
  c.insert("b".to_string(), string("ccc"));

  let result = styleq(&[StyleqInput::Nested(vec![
    StyleqInput::Style(a),
    StyleqInput::Nested(vec![StyleqInput::Style(b), StyleqInput::Style(c)]),
  ])]);

  assert_eq!(
    result.data_style_src,
    "path/to/a:1; path/to/b:2; path/to/c:3"
  );
}
