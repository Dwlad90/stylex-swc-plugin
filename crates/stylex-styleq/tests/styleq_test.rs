use std::sync::atomic::{AtomicUsize, Ordering};

use indexmap::IndexMap;
use log::{Level, LevelFilter, Metadata, Record};
use stylex_styleq::{
  COMPILED_KEY, StyleMap, StyleValue, StyleqInput, StyleqOptions, create_styleq, styleq,
};

static STYLEQ_ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static TEST_LOGGER: TestLogger = TestLogger;

struct TestLogger;

impl log::Log for TestLogger {
  fn enabled(&self, metadata: &Metadata<'_>) -> bool {
    metadata.level() <= Level::Error
  }

  fn log(&self, record: &Record<'_>) {
    if self.enabled(record.metadata()) {
      STYLEQ_ERROR_COUNT.fetch_add(1, Ordering::SeqCst);
    }
  }

  fn flush(&self) {}
}

fn init_error_logger() {
  let _ = log::set_logger(&TEST_LOGGER);
  log::set_max_level(LevelFilter::Error);
}

fn compiled(entries: &[(&str, StyleValue)]) -> StyleqInput<StyleValue> {
  compiled_with_marker(StyleValue::Bool(true), entries)
}

fn compiled_with_marker(
  marker: StyleValue,
  entries: &[(&str, StyleValue)],
) -> StyleqInput<StyleValue> {
  let mut style = IndexMap::new();
  style.insert(COMPILED_KEY.to_string(), marker);

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

fn stringify_inline_style(inline_style: &StyleMap<StyleValue>) -> String {
  let mut result = String::new();

  for (prop, value) in inline_style {
    result.push_str(prop);
    result.push(':');
    match value {
      StyleValue::String(value) => result.push_str(value),
      StyleValue::Number(value) => result.push_str(&value.to_string()),
      StyleValue::Null => result.push_str("null"),
      StyleValue::Bool(value) => result.push_str(&value.to_string()),
      StyleValue::Object => result.push_str("[object Object]"),
      StyleValue::Array => {},
      StyleValue::Undefined => result.push_str("undefined"),
    }
    result.push(';');
  }

  result
}

fn stringify_inline_result(inline_style: &Option<StyleMap<StyleValue>>) -> String {
  match inline_style {
    Some(inline_style) => stringify_inline_style(inline_style),
    None => String::new(),
  }
}

#[test]
fn warns_if_extracted_property_values_are_not_strings_or_null() {
  init_error_logger();
  STYLEQ_ERROR_COUNT.store(0, Ordering::SeqCst);

  for value in [
    StyleValue::Number(1),
    StyleValue::Undefined,
    StyleValue::Bool(false),
    StyleValue::Bool(true),
    StyleValue::Object,
    StyleValue::Array,
  ] {
    let _ = styleq(&[compiled(&[("a", value)])]);
  }

  assert_eq!(STYLEQ_ERROR_COUNT.load(Ordering::SeqCst), 6);
}

#[test]
fn combines_different_class_names() {
  let style = compiled(&[("a", string("aaa")), ("b", string("bbb"))]);
  let styleq_no_cache = create_styleq(StyleqOptions {
    disable_cache: true,
    ..Default::default()
  });

  assert_eq!(
    styleq_no_cache.styleq(&[style.clone()]).class_name,
    "aaa bbb"
  );
  assert_eq!(styleq(&[style]).class_name, "aaa bbb");
}

#[test]
fn combines_different_class_names_in_order() {
  let a = compiled(&[("a", string("a")), (":focus$aa", string("focus$aa"))]);
  let b = compiled(&[("b", string("b"))]);
  let c = compiled(&[("c", string("c")), (":focus$cc", string("focus$cc"))]);
  let styleq_no_cache = create_styleq(StyleqOptions {
    disable_cache: true,
    ..Default::default()
  });

  assert_eq!(
    styleq_no_cache
      .styleq(&[StyleqInput::Nested(vec![a.clone(), b.clone(), c.clone()])])
      .class_name,
    "a focus$aa b c focus$cc"
  );

  assert_eq!(
    styleq(&[StyleqInput::Nested(vec![a, b, c])]).class_name,
    "a focus$aa b c focus$cc"
  );
}

#[test]
fn dedupes_class_names_for_same_key() {
  let a = compiled(&[("backgroundColor", string("backgroundColor-a"))]);
  let b = compiled(&[("backgroundColor", string("backgoundColor-b"))]);
  let c = compiled(&[("backgroundColor", string("backgoundColor-c"))]);
  let styleq_no_cache = create_styleq(StyleqOptions {
    disable_cache: true,
    ..Default::default()
  });

  assert_eq!(
    styleq_no_cache
      .styleq(&[StyleqInput::Nested(vec![a.clone(), b.clone()])])
      .class_name,
    "backgoundColor-b"
  );
  assert_eq!(
    styleq(&[StyleqInput::Nested(vec![a.clone(), b.clone()])]).class_name,
    "backgoundColor-b"
  );
  assert_eq!(
    styleq(&[StyleqInput::Nested(vec![c, a, b])]).class_name,
    "backgoundColor-b"
  );
}

#[test]
fn dedupes_class_names_with_null_value() {
  let a = compiled(&[("backgroundColor", string("backgroundColor-a"))]);
  let b = compiled(&[("backgroundColor", StyleValue::Null)]);
  let styleq_no_cache = create_styleq(StyleqOptions {
    disable_cache: true,
    ..Default::default()
  });

  assert_eq!(
    styleq_no_cache
      .styleq(&[StyleqInput::Nested(vec![a.clone(), b.clone()])])
      .class_name,
    ""
  );
  assert_eq!(styleq(&[StyleqInput::Nested(vec![a, b])]).class_name, "");
}

#[test]
fn dedupes_class_names_in_complex_merges() {
  let a = compiled(&[
    ("backgroundColor", string("backgroundColor-a")),
    ("borderColor", string("borderColor-a")),
    ("borderStyle", string("borderStyle-a")),
    ("borderWidth", string("borderWidth-a")),
    ("boxSizing", string("boxSizing-a")),
    ("display", string("display-a")),
    ("listStyle", string("listStyle-a")),
    ("marginTop", string("marginTop-a")),
    ("marginEnd", string("marginEnd-a")),
    ("marginBottom", string("marginBottom-a")),
    ("marginStart", string("marginStart-a")),
    ("paddingTop", string("paddingTop-a")),
    ("paddingEnd", string("paddingEnd-a")),
    ("paddingBottom", string("paddingBottom-a")),
    ("paddingStart", string("paddingStart-a")),
    ("textAlign", string("textAlign-a")),
    ("textDecoration", string("textDecoration-a")),
    ("whiteSpace", string("whiteSpace-a")),
    ("wordWrap", string("wordWrap-a")),
    ("zIndex", string("zIndex-a")),
  ]);
  let b = compiled(&[
    ("cursor", string("cursor-b")),
    ("touchAction", string("touchAction-b")),
  ]);
  let c = compiled(&[("outline", string("outline-c"))]);
  let d = compiled(&[
    ("cursor", string("cursor-d")),
    ("touchAction", string("touchAction-d")),
  ]);
  let e = compiled(&[
    ("textDecoration", string("textDecoration-e")),
    (":focus$textDecoration", string("focus$textDecoration-e")),
  ]);
  let f = compiled(&[
    ("backgroundColor", string("backgroundColor-f")),
    ("color", string("color-f")),
    ("cursor", string("cursor-f")),
    ("display", string("display-f")),
    ("marginEnd", string("marginEnd-f")),
    ("marginStart", string("marginStart-f")),
    ("textAlign", string("textAlign-f")),
    ("textDecoration", string("textDecoration-f")),
    (":focus$color", string("focus$color-f")),
    (":focus$textDecoration", string("focus$textDecoration-f")),
    (":active$transform", string("active$transform-f")),
    (":active$transition", string("active$transition-f")),
  ]);
  let g = compiled(&[
    ("display", string("display-g")),
    ("width", string("width-g")),
  ]);
  let h = compiled(&[(":active$transform", string("active$transform-h"))]);

  let one = StyleqInput::Nested(vec![
    a.clone(),
    StyleqInput::False,
    StyleqInput::Nested(vec![
      b.clone(),
      StyleqInput::False,
      c.clone(),
      StyleqInput::Nested(vec![
        d.clone(),
        StyleqInput::False,
        e.clone(),
        StyleqInput::False,
        StyleqInput::Nested(vec![f.clone(), g.clone()]),
        StyleqInput::Nested(vec![h.clone()]),
      ]),
    ]),
  ]);

  let one_value = styleq(&[one.clone()]).class_name;
  let one_repeat = styleq(&[one]).class_name;
  assert_eq!(one_value, one_repeat);
  assert_eq!(
    one_value,
    "borderColor-a borderStyle-a borderWidth-a boxSizing-a listStyle-a marginTop-a \
marginBottom-a paddingTop-a paddingEnd-a paddingBottom-a paddingStart-a whiteSpace-a wordWrap-a \
zIndex-a outline-c touchAction-d backgroundColor-f color-f cursor-f marginEnd-f marginStart-f \
textAlign-f textDecoration-f focus$color-f focus$textDecoration-f active$transition-f display-g \
width-g active$transform-h"
  );

  let two = StyleqInput::Nested(vec![
    d,
    StyleqInput::False,
    StyleqInput::Nested(vec![
      c,
      StyleqInput::False,
      b,
      StyleqInput::Nested(vec![
        a,
        StyleqInput::False,
        e,
        StyleqInput::False,
        StyleqInput::Nested(vec![f, g]),
        StyleqInput::Nested(vec![h]),
      ]),
    ]),
  ]);

  let two_value = styleq(&[two.clone()]).class_name;
  let two_repeat = styleq(&[two]).class_name;
  assert_eq!(two_value, two_repeat);
  assert_eq!(
    two_value,
    "outline-c touchAction-b borderColor-a borderStyle-a borderWidth-a boxSizing-a listStyle-a \
marginTop-a marginBottom-a paddingTop-a paddingEnd-a paddingBottom-a paddingStart-a whiteSpace-a \
wordWrap-a zIndex-a backgroundColor-f color-f cursor-f marginEnd-f marginStart-f textAlign-f \
textDecoration-f focus$color-f focus$textDecoration-f active$transition-f display-g width-g \
active$transform-h"
  );
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
fn preserves_order_of_stringified_inline_style() {
  let result = styleq(&[StyleqInput::Nested(vec![inline(&[
    ("font", string("inherit")),
    ("fontSize", StyleValue::Number(12)),
  ])])]);

  assert_eq!(
    result.inline_style,
    Some(StyleMap::from([
      ("font".to_string(), string("inherit")),
      ("fontSize".to_string(), StyleValue::Number(12)),
    ]))
  );
  assert_eq!(
    stringify_inline_result(&result.inline_style),
    "font:inherit;fontSize:12;"
  );

  let result = styleq(&[StyleqInput::Nested(vec![
    inline(&[("font", string("inherit"))]),
    inline(&[("fontSize", StyleValue::Number(12))]),
  ])]);

  assert_eq!(
    stringify_inline_result(&result.inline_style),
    "font:inherit;fontSize:12;"
  );
}

#[test]
fn dedupes_class_names_and_inline_styles() {
  let a = compiled(&[("a", string("a")), (":focus$a", string("focus$a"))]);
  let b = compiled(&[("b", string("b"))]);
  let inline_b = inline(&[("b", string("b")), ("bb", StyleValue::Null)]);
  let inline_b_alt = inline(&[("b", StyleValue::Null)]);

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

  let result = styleq(&[StyleqInput::Nested(vec![a.clone(), inline_b, b.clone()])]);
  assert_eq!(result.class_name, "a focus$a b");
  assert_eq!(result.inline_style, None);

  let result = styleq(&[StyleqInput::Nested(vec![a, b, inline_b_alt])]);
  assert_eq!(result.class_name, "a focus$a");
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

  let result = styleq_no_mix.styleq(&[StyleqInput::Nested(vec![
    inline(&[("a", string("a"))]),
    inline(&[("a", StyleValue::Null)]),
  ])]);

  assert_eq!(
    result.inline_style,
    Some(StyleMap::from([("a".to_string(), StyleValue::Null)]))
  );
}

#[test]
fn disable_mix_preserves_order_of_stringified_inline_style() {
  let styleq_no_mix = create_styleq(StyleqOptions {
    disable_mix: true,
    ..Default::default()
  });

  let result = styleq_no_mix.styleq(&[StyleqInput::Nested(vec![inline(&[
    ("font", string("inherit")),
    ("fontSize", StyleValue::Number(12)),
  ])])]);

  assert_eq!(
    stringify_inline_result(&result.inline_style),
    "font:inherit;fontSize:12;"
  );

  let result = styleq_no_mix.styleq(&[StyleqInput::Nested(vec![
    inline(&[("font", string("inherit"))]),
    inline(&[("fontSize", StyleValue::Number(12))]),
  ])]);

  assert_eq!(
    stringify_inline_result(&result.inline_style),
    "font:inherit;fontSize:12;"
  );
}

#[test]
fn disable_mix_does_not_dedupe_class_names_and_inline_styles() {
  let styleq_no_mix = create_styleq(StyleqOptions {
    disable_mix: true,
    ..Default::default()
  });
  let a = compiled(&[("a", string("a")), (":focus$a", string("focus$a"))]);
  let b = compiled(&[("b", string("b"))]);
  let inline_b = inline(&[("b", string("b")), ("bb", StyleValue::Null)]);

  assert_eq!(
    styleq_no_mix.styleq(&[StyleqInput::Nested(vec![
      a.clone(),
      b.clone(),
      inline_b.clone()
    ])]),
    styleq_no_mix.styleq(&[StyleqInput::Nested(vec![a, inline_b, b])])
  );
}

#[test]
fn supports_generating_debug_strings() {
  let a = compiled_with_marker(string("path/to/a:1"), &[("a", string("aaa"))]);
  let b = compiled_with_marker(string("path/to/b:2"), &[("b", string("bbb"))]);
  let c = compiled_with_marker(string("path/to/c:3"), &[("b", string("ccc"))]);

  let result = styleq(&[StyleqInput::Nested(vec![a.clone()])]);
  assert_eq!(result.data_style_src, "path/to/a:1");

  let result = styleq(&[StyleqInput::Nested(vec![
    a.clone(),
    StyleqInput::Nested(vec![b.clone(), c.clone()]),
  ])]);

  assert_eq!(
    result.data_style_src,
    "path/to/a:1; path/to/b:2; path/to/c:3"
  );

  let styleq_no_cache = create_styleq(StyleqOptions {
    disable_cache: true,
    ..Default::default()
  });
  let result = styleq_no_cache.styleq(&[StyleqInput::Nested(vec![
    a,
    StyleqInput::Nested(vec![b, c]),
  ])]);

  assert_eq!(
    result.data_style_src,
    "path/to/a:1; path/to/b:2; path/to/c:3"
  );
}
