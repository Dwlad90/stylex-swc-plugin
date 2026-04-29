use std::{
  cell::{Cell, RefCell},
  rc::Rc,
  sync::atomic::{AtomicUsize, Ordering},
};

use indexmap::IndexMap;
use log::{Level, LevelFilter, Metadata, Record};
use serial_test::serial;
use stylex_styleq::{
  COMPILED_KEY, StyleMap, StyleValue, StyleqArgument, StyleqInput, StyleqOptions, StyleqValue,
  create_styleq, styleq,
};

static STYLEQ_ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static TEST_LOGGER: TestLogger = TestLogger;
const LOCALIZE_MARKER: &str = "$$css$localize";

struct TestLogger;

struct DefaultArgument;

#[derive(Clone)]
enum TestArgument {
  Empty,
  Skip,
  Style {
    style: StyleMap<StyleValue>,
    cache_key: Option<usize>,
  },
  Nested(Vec<TestArgument>),
}

impl StyleqArgument<StyleValue> for DefaultArgument {
  fn as_style(&self) -> Option<&StyleMap<StyleValue>> {
    None
  }
}

impl StyleqArgument<StyleValue> for TestArgument {
  fn as_style(&self) -> Option<&StyleMap<StyleValue>> {
    match self {
      TestArgument::Style { style, .. } => Some(style),
      TestArgument::Empty | TestArgument::Skip | TestArgument::Nested(_) => None,
    }
  }

  fn cache_key(&self) -> Option<usize> {
    match self {
      TestArgument::Style { cache_key, .. } => *cache_key,
      TestArgument::Empty | TestArgument::Skip | TestArgument::Nested(_) => None,
    }
  }

  fn as_nested(&self) -> Option<&[Self]> {
    match self {
      TestArgument::Nested(styles) => Some(styles),
      TestArgument::Empty | TestArgument::Skip | TestArgument::Style { .. } => None,
    }
  }

  fn should_skip(&self) -> bool {
    matches!(self, TestArgument::Skip)
  }
}

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

fn compiled_map(entries: &[(&str, StyleValue)]) -> StyleMap<StyleValue> {
  let mut style = IndexMap::new();
  style.insert(COMPILED_KEY.to_string(), StyleValue::Bool(true));

  for (key, value) in entries {
    style.insert((*key).to_string(), value.clone());
  }

  style
}

fn inline_map(entries: &[(&str, StyleValue)]) -> StyleMap<StyleValue> {
  entries
    .iter()
    .map(|(key, value)| ((*key).to_string(), value.clone()))
    .collect()
}

fn string(value: &str) -> StyleValue {
  StyleValue::string(value)
}

fn transform_fixture() -> StyleMap<StyleValue> {
  let mut fixture = IndexMap::new();
  fixture.insert(COMPILED_KEY.to_string(), StyleValue::Bool(true));
  fixture.insert(LOCALIZE_MARKER.to_string(), StyleValue::Bool(true));
  fixture.insert("marginStart".to_string(), string("marginStart"));
  fixture.insert("marginEnd".to_string(), string("marginEnd"));
  fixture
}

fn opacity_style() -> StyleMap<StyleValue> {
  StyleMap::from([("opacity".to_string(), StyleValue::Number(1))])
}

fn localize_style(style: StyleMap<StyleValue>, is_rtl: bool) -> StyleMap<StyleValue> {
  style
    .into_iter()
    .filter_map(|(prop, value)| {
      if prop == LOCALIZE_MARKER {
        None
      } else if prop == "marginStart" {
        Some((
          prop,
          string(if is_rtl {
            "margin-right-0px"
          } else {
            "margin-left-0px"
          }),
        ))
      } else if prop == "marginEnd" {
        Some((
          prop,
          string(if is_rtl {
            "margin-left-10px"
          } else {
            "margin-right-10px"
          }),
        ))
      } else {
        Some((prop, value))
      }
    })
    .collect()
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
#[serial]
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
    styleq_no_cache
      .styleq(std::slice::from_ref(&style))
      .class_name,
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

  let one_value = styleq(std::slice::from_ref(&one)).class_name;
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

  let two_value = styleq(std::slice::from_ref(&two)).class_name;
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

#[test]
fn supports_style_transforms() {
  let is_rtl = Rc::new(Cell::new(false));
  let is_rtl_for_transform = Rc::clone(&is_rtl);
  let styleq_with_transform = create_styleq(StyleqOptions {
    transform: Some(Rc::new(move |style: StyleMap<StyleValue>| {
      localize_style(style, is_rtl_for_transform.get())
    })),
    ..Default::default()
  });

  is_rtl.set(false);
  let result = styleq_with_transform.styleq(&[
    StyleqInput::Style(transform_fixture()),
    StyleqInput::Style(opacity_style()),
  ]);

  assert_eq!(result.class_name, "margin-left-0px margin-right-10px");
  assert_eq!(result.inline_style, Some(opacity_style()));

  is_rtl.set(true);
  let result = styleq_with_transform.styleq(&[
    StyleqInput::Style(transform_fixture()),
    StyleqInput::Style(opacity_style()),
  ]);

  assert_eq!(result.class_name, "margin-right-0px margin-left-10px");
  assert_eq!(result.inline_style, Some(opacity_style()));
}

#[test]
fn memoizes_transform_results() {
  let cache = Rc::new(RefCell::new(
    None::<(StyleMap<StyleValue>, StyleMap<StyleValue>)>,
  ));
  let compile_count = Rc::new(Cell::new(0));
  let cache_for_transform = Rc::clone(&cache);
  let compile_count_for_transform = Rc::clone(&compile_count);
  let styleq_with_transform = create_styleq(StyleqOptions {
    transform: Some(Rc::new(move |style: StyleMap<StyleValue>| {
      {
        let cache = cache_for_transform.borrow();
        if let Some((cached_style, cached_result)) = cache.as_ref()
          && cached_style == &style
        {
          return cached_result.clone();
        }
      }

      compile_count_for_transform.set(compile_count_for_transform.get() + 1);
      let localized_style = localize_style(style.clone(), false);
      cache_for_transform
        .borrow_mut()
        .replace((style, localized_style.clone()));
      localized_style
    })),
    ..Default::default()
  });

  let _ = styleq_with_transform.styleq(&[StyleqInput::Style(transform_fixture())]);
  let _ = styleq_with_transform.styleq(&[StyleqInput::Style(transform_fixture())]);

  assert_eq!(compile_count.get(), 1);
}

#[test]
fn styleq_options_default_matches_runtime_defaults() {
  let options = StyleqOptions::<StyleValue>::default();

  assert!(!options.disable_cache);
  assert!(!options.disable_mix);
  assert!(!options.dedupe_class_name_chunks);
  assert!(options.transform.is_none());
}

#[test]
fn styleq_argument_default_methods_are_noops() {
  let argument = DefaultArgument;

  assert_eq!(argument.cache_key(), None);
  assert!(argument.as_nested().is_none());
  assert!(!argument.should_skip());
}

#[test]
fn ignores_custom_argument_that_is_neither_style_nested_nor_skip() {
  let result = create_styleq(StyleqOptions::default()).styleq(&[TestArgument::Empty]);

  assert_eq!(result.class_name, "");
  assert_eq!(result.inline_style, None);
  assert_eq!(result.data_style_src, "");
}

#[test]
fn custom_argument_covers_skip_nested_inline_and_disable_mix_paths() {
  let styleq_default = create_styleq(StyleqOptions::default());
  let result = styleq_default.styleq(&[
    TestArgument::Skip,
    TestArgument::Nested(vec![
      TestArgument::Style {
        style: compiled_map(&[("display", string("display-block"))]),
        cache_key: None,
      },
      TestArgument::Style {
        style: inline_map(&[("color", string("red"))]),
        cache_key: None,
      },
    ]),
    TestArgument::Empty,
  ]);

  assert_eq!(result.class_name, "display-block");
  assert_eq!(
    result.inline_style,
    Some(StyleMap::from([("color".to_string(), string("red"))]))
  );

  let styleq_no_mix = create_styleq(StyleqOptions {
    disable_mix: true,
    ..Default::default()
  });
  let result = styleq_no_mix.styleq(&[
    TestArgument::Style {
      style: inline_map(&[("color", string("red"))]),
      cache_key: None,
    },
    TestArgument::Style {
      style: inline_map(&[("backgroundColor", string("blue"))]),
      cache_key: None,
    },
  ]);

  assert_eq!(
    result.inline_style,
    Some(StyleMap::from([
      ("backgroundColor".to_string(), string("blue")),
      ("color".to_string(), string("red")),
    ]))
  );
}

#[test]
fn uses_identity_cache_key_when_argument_provides_stable_identity() {
  let styleq_cached = create_styleq(StyleqOptions::default());
  let argument = TestArgument::Style {
    style: compiled_map(&[("display", string("display-block"))]),
    cache_key: Some(42),
  };

  assert_eq!(
    styleq_cached
      .styleq(std::slice::from_ref(&argument))
      .class_name,
    "display-block"
  );
  assert_eq!(
    styleq_cached.styleq(&[argument]).class_name,
    "display-block"
  );
}

#[test]
fn ignores_identity_cache_key_when_transform_is_configured() {
  let styleq_with_transform = create_styleq(StyleqOptions {
    transform: Some(Rc::new(|style| style)),
    ..Default::default()
  });

  assert_eq!(
    styleq_with_transform
      .styleq(&[TestArgument::Style {
        style: compiled_map(&[("display", string("display-block"))]),
        cache_key: Some(42),
      }])
      .class_name,
    "display-block"
  );
}

#[test]
#[serial]
fn warns_if_compiled_marker_is_not_true_or_debug_string() {
  init_error_logger();
  STYLEQ_ERROR_COUNT.store(0, Ordering::SeqCst);

  let _ = styleq(&[compiled_with_marker(
    StyleValue::Number(1),
    &[("a", string("aaa"))],
  )]);

  assert_eq!(STYLEQ_ERROR_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn dedupes_repeated_class_name_chunks_when_option_is_enabled() {
  let styleq_dedupe_chunks = create_styleq(StyleqOptions {
    dedupe_class_name_chunks: true,
    ..Default::default()
  });
  let style = compiled(&[("display", string("display-block"))]);

  let result = styleq_dedupe_chunks.styleq(&[style.clone(), style]);

  assert_eq!(result.class_name, "display-block");
}

#[test]
fn keeps_distinct_class_name_chunks_when_chunk_deduping_is_enabled() {
  let styleq_dedupe_chunks = create_styleq(StyleqOptions {
    dedupe_class_name_chunks: true,
    ..Default::default()
  });
  let a = compiled(&[("display", string("display-block"))]);
  let b = compiled(&[("color", string("color-red"))]);

  let result = styleq_dedupe_chunks.styleq(&[a, b]);

  assert_eq!(result.class_name, "display-block color-red");
}

#[test]
fn styleq_input_trait_methods_cover_all_variants() {
  let style = inline(&[("color", string("red"))]);
  let nested = StyleqInput::Nested(vec![style.clone()]);

  assert!(style.as_style().is_some());
  assert!(StyleqInput::<StyleValue>::Null.as_style().is_none());
  assert!(StyleqInput::<StyleValue>::False.as_style().is_none());
  assert!(nested.as_style().is_none());

  assert!(matches!(nested.as_nested(), Some(values) if values == [style]));
  assert!(
    StyleqInput::<StyleValue>::Style(StyleMap::new())
      .as_nested()
      .is_none()
  );
  assert!(StyleqInput::<StyleValue>::Null.as_nested().is_none());
  assert!(StyleqInput::<StyleValue>::False.as_nested().is_none());

  assert!(StyleqInput::<StyleValue>::Null.should_skip());
  assert!(StyleqInput::<StyleValue>::False.should_skip());
  assert!(!StyleqInput::<StyleValue>::Style(StyleMap::new()).should_skip());
}

#[test]
fn style_value_trait_methods_cover_all_value_kinds() {
  let class_name = StyleValue::string("x1abc");
  let class_name_rc = Rc::new(class_name.clone());

  assert_eq!(class_name.as_class_name(), Some("x1abc"));
  assert_eq!(class_name_rc.as_class_name(), Some("x1abc"));
  assert!(!class_name.is_null());
  assert!(!class_name_rc.is_null());
  assert!(!class_name.is_true_bool());
  assert!(!class_name_rc.is_true_bool());

  assert_eq!(StyleValue::Number(1).as_class_name(), None);
  assert!(StyleValue::Null.is_null());
  assert!(!StyleValue::Bool(false).is_true_bool());
  assert!(StyleValue::Bool(true).is_true_bool());
}
