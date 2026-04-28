use std::{
  cell::{Cell, RefCell},
  rc::Rc,
};

use indexmap::IndexMap;
use stylex_styleq::{
  COMPILED_KEY, StyleMap, StyleValue, StyleqInput, StyleqOptions, create_styleq,
};

const LOCALIZE_MARKER: &str = "$$css$localize";

fn string(value: &str) -> StyleValue {
  StyleValue::string(value)
}

fn fixture() -> StyleMap<StyleValue> {
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
    StyleqInput::Style(fixture()),
    StyleqInput::Style(opacity_style()),
  ]);

  assert_eq!(result.class_name, "margin-left-0px margin-right-10px");
  assert_eq!(result.inline_style, Some(opacity_style()));

  is_rtl.set(true);
  let result = styleq_with_transform.styleq(&[
    StyleqInput::Style(fixture()),
    StyleqInput::Style(opacity_style()),
  ]);

  assert_eq!(result.class_name, "margin-right-0px margin-left-10px");
  assert_eq!(result.inline_style, Some(opacity_style()));
}

#[test]
fn memoizes_results() {
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

  let _ = styleq_with_transform.styleq(&[StyleqInput::Style(fixture())]);
  let _ = styleq_with_transform.styleq(&[StyleqInput::Style(fixture())]);

  assert_eq!(compile_count.get(), 1);
}
