use indexmap::IndexMap;
use stylex_styleq::{
  COMPILED_KEY, StyleMap, StyleValue, StyleqInput, StyleqOptions, create_styleq,
};

fn string(value: &str) -> StyleValue {
  StyleValue::string(value)
}

#[test]
fn supports_style_transforms() {
  let mut fixture = IndexMap::new();
  fixture.insert(COMPILED_KEY.to_string(), StyleValue::Bool(true));
  fixture.insert("$$css$localize".to_string(), StyleValue::Bool(true));
  fixture.insert("marginStart".to_string(), string("margin-left-0px"));
  fixture.insert("marginEnd".to_string(), string("margin-right-10px"));

  let styleq_with_transform = create_styleq(StyleqOptions {
    transform: Some(std::rc::Rc::new(|style: StyleMap<StyleValue>| {
      style
        .into_iter()
        .filter(|(prop, _)| prop != "$$css$localize")
        .collect()
    })),
    ..Default::default()
  });

  let result = styleq_with_transform.styleq(&[
    StyleqInput::Style(fixture),
    StyleqInput::Style(StyleMap::from([(
      "opacity".to_string(),
      StyleValue::Number(1),
    )])),
  ]);

  assert_eq!(result.class_name, "margin-left-0px margin-right-10px");
  assert_eq!(
    result.inline_style,
    Some(StyleMap::from([(
      "opacity".to_string(),
      StyleValue::Number(1)
    )]))
  );
}
