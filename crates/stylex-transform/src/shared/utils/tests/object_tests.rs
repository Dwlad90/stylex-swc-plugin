//! Tests for the object helpers the theme and variable transforms build on.

use std::rc::Rc;

use indexmap::IndexMap;
use stylex_state::{
  flat_compiled_styles_value::FlatCompiledStylesValue, state_manager::StateManager,
  types::FlatCompiledStyles,
};
use stylex_structures::{order_pair::OrderPair, pair::Pair, raw_value::TRawValue};

use crate::shared::{
  enums::data_structures::obj_map_type::ObjMapType,
  utils::object::{
    Pipe, obj_entries, obj_from_entries, obj_map, obj_map_keys_and_transform_values,
    obj_map_keys_key_value, preprocess_object_properties,
  },
};
use crate::tests::support::{expr, object};

/// Names the value it is given, so one instantiation of `obj_map` reads both
/// kinds of input. A closure per call would be a second instantiation, and no
/// one instantiation would then run the whole function.
fn name_the_value(
  value: Rc<FlatCompiledStylesValue>,
  _state: &mut StateManager,
) -> Rc<FlatCompiledStylesValue> {
  let name = match value.as_ref() {
    FlatCompiledStylesValue::Tuple(key, _, _) => format!("tuple:{key}"),
    FlatCompiledStylesValue::String(text) => format!("string:{text}"),
    other => format!("other:{other:?}"),
  };

  Rc::new(FlatCompiledStylesValue::String(name))
}

fn text_of(styles: &FlatCompiledStyles, key: &str) -> String {
  match styles.get(key).map(Rc::as_ref) {
    Some(FlatCompiledStylesValue::String(text)) => text.clone(),
    other => panic!("the key {key} does not hold a string: {other:?}"),
  }
}

/// An object literal reaches the mapper one key at a time, and the key it is
/// named by is the one the author wrote.
#[test]
fn maps_every_key_of_an_object_literal() {
  let mut state = StateManager::default();

  let result = obj_map(
    ObjMapType::Object(object("{ color: 'red', 'background-color': 'blue' }")),
    &mut state,
    name_the_value,
  );

  assert_eq!(
    result.keys().cloned().collect::<Vec<_>>(),
    ["color", "background-color"]
  );
  assert_eq!(text_of(&result, "color"), "tuple:color");
  assert_eq!(
    text_of(&result, "background-color"),
    "tuple:background-color"
  );
}

/// A map reaches the mapper as the values it already holds, with no key beside
/// them. The same call site reads both kinds, so the whole function runs in one
/// instantiation.
#[test]
fn maps_every_value_of_a_map() {
  let mut state = StateManager::default();
  let mut entries: FlatCompiledStyles = IndexMap::new();

  entries.insert(
    "--color".to_owned(),
    Rc::new(FlatCompiledStylesValue::String("red".to_owned())),
  );
  entries.insert(
    "--size".to_owned(),
    Rc::new(FlatCompiledStylesValue::String("1px".to_owned())),
  );

  let result = obj_map(ObjMapType::Map(entries), &mut state, name_the_value);

  assert_eq!(
    result.keys().cloned().collect::<Vec<_>>(),
    ["--color", "--size"]
  );
  assert_eq!(text_of(&result, "--color"), "string:red");
  assert_eq!(text_of(&result, "--size"), "string:1px");
}

#[test]
fn maps_an_empty_input_of_either_kind_to_an_empty_answer() {
  let mut state = StateManager::default();

  assert!(obj_map(ObjMapType::Object(object("{}")), &mut state, name_the_value).is_empty());
  assert!(
    obj_map(
      ObjMapType::Map(FlatCompiledStyles::new()),
      &mut state,
      name_the_value
    )
    .is_empty()
  );
}

/// A key the author wrote twice is one key in the answer, and it holds what the
/// last of the two mapped.
#[test]
fn a_repeated_key_of_an_object_literal_is_mapped_once() {
  let mut state = StateManager::default();

  let result = obj_map(
    ObjMapType::Object(object("{ color: 'red', color: 'blue' }")),
    &mut state,
    name_the_value,
  );

  assert_eq!(result.len(), 1);
  assert_eq!(text_of(&result, "color"), "tuple:color");
}

#[test]
fn reads_the_entries_of_an_object_literal() {
  let entries = obj_entries(&expr("{ color: 'red', size: 1 }"));

  assert_eq!(entries.len(), 2);
}

#[test]
fn reads_no_entry_from_an_empty_object_literal() {
  assert!(obj_entries(&expr("{}")).is_empty());
}

/// Only an object literal holds entries to read. Anything else is a fault in
/// the caller, so the call is refused rather than answered with nothing.
#[test]
#[should_panic(expected = "Object expected")]
fn refuses_to_read_the_entries_of_something_that_is_not_an_object() {
  obj_entries(&expr("[1, 2]"));
}

#[test]
fn builds_a_map_from_the_pairs_that_hold_a_value() {
  let entries = vec![
    OrderPair("color".into(), Some("red".into())),
    OrderPair("size".into(), Some("1px".into())),
  ];

  let result = obj_from_entries(&entries);

  assert_eq!(
    result.keys().cloned().collect::<Vec<_>>(),
    ["color", "size"]
  );
  assert_eq!(result["color"], "red");
}

/// A repeated key keeps the place the first of the two gave it, and holds the
/// value of the last.
#[test]
fn a_repeated_pair_keeps_its_first_place() {
  let entries = vec![
    OrderPair("color".into(), Some("red".into())),
    OrderPair("size".into(), Some("1px".into())),
    OrderPair("color".into(), Some("blue".into())),
  ];

  let result = obj_from_entries(&entries);

  assert_eq!(
    result.keys().cloned().collect::<Vec<_>>(),
    ["color", "size"]
  );
  assert_eq!(result["color"], "blue");
}

#[test]
fn builds_an_empty_map_from_no_pair() {
  assert!(obj_from_entries(&[]).is_empty());
}

/// A pair holding no value cannot become an entry, and dropping it silently
/// would leave the caller with a map short of a key it wrote.
#[test]
#[should_panic(expected = "Value is not a string")]
fn refuses_a_pair_that_holds_no_value() {
  obj_from_entries(&[OrderPair("color".into(), None)]);
}

/// The key is dashed before the value is read, because the dashed name is the
/// one that decides whether a plain number gets a unit.
#[test]
fn dashes_the_key_before_it_transforms_the_value() {
  let mut state = StateManager::default();
  let mut entries: IndexMap<String, TRawValue> = IndexMap::new();

  entries.insert("marginTop".to_owned(), TRawValue::Number(10.0));

  let result = obj_map_keys_and_transform_values(
    &entries,
    &mut state,
    |key| key.replace("marginTop", "margin-top"),
    FlatCompiledStylesValue::KeyValue,
  );

  match result["margin-top"].as_ref() {
    FlatCompiledStylesValue::KeyValue(pair) => {
      assert_eq!(pair.key, "margin-top");
      assert_eq!(pair.value, "10px");
    },
    other => panic!("the entry is not a key-value pair: {other:?}"),
  }
}

/// A property that takes no unit keeps the number it was written with.
#[test]
fn keeps_a_unitless_value_as_it_was_written() {
  let mut state = StateManager::default();
  let mut entries: IndexMap<String, TRawValue> = IndexMap::new();

  entries.insert("opacity".to_owned(), TRawValue::Number(0.5));
  entries.insert("zIndex".to_owned(), TRawValue::Number(2.0));

  let result = obj_map_keys_and_transform_values(
    &entries,
    &mut state,
    |key| key.to_owned(),
    FlatCompiledStylesValue::KeyValue,
  );

  match result["zIndex"].as_ref() {
    FlatCompiledStylesValue::KeyValue(pair) => assert_eq!(pair.value, "2"),
    other => panic!("the entry is not a key-value pair: {other:?}"),
  }
}

#[test]
fn maps_no_key_of_an_empty_entry_set() {
  let mut state = StateManager::default();

  assert!(
    obj_map_keys_and_transform_values(
      &IndexMap::new(),
      &mut state,
      |key| key.to_owned(),
      FlatCompiledStylesValue::KeyValue,
    )
    .is_empty()
  );
}

#[test]
fn maps_the_key_of_every_key_value_entry() {
  let mut entries: FlatCompiledStyles = IndexMap::new();

  entries.insert(
    "root".to_owned(),
    Rc::new(FlatCompiledStylesValue::KeyValues(vec![Pair::new(
      "color".to_owned(),
      "red".to_owned(),
    )])),
  );

  let result = obj_map_keys_key_value(&entries, |key| format!("--{key}"));

  assert_eq!(result.keys().cloned().collect::<Vec<_>>(), ["--root"]);

  match result["--root"].as_ref() {
    FlatCompiledStylesValue::KeyValues(pairs) => {
      assert_eq!(pairs.len(), 1);
      assert_eq!(pairs[0].key, "color");
      assert_eq!(pairs[0].value, "red");
    },
    other => panic!("the entry is not a set of key-value pairs: {other:?}"),
  }
}

#[test]
fn maps_no_key_of_an_empty_key_value_set() {
  assert!(obj_map_keys_key_value(&FlatCompiledStyles::new(), |key| key.to_owned()).is_empty());
}

/// Only a set of key-value pairs can have its keys mapped. Any other value is a
/// fault in the caller.
#[test]
#[should_panic(expected = "Value must be a key-value pairs")]
fn refuses_a_value_that_holds_no_key_value_pairs() {
  let mut entries: FlatCompiledStyles = IndexMap::new();

  entries.insert(
    "root".to_owned(),
    Rc::new(FlatCompiledStylesValue::String("red".to_owned())),
  );

  obj_map_keys_key_value(&entries, |key| key.to_owned());
}

/// An alias is written under the property it stands for. Only the properties
/// that carry a value survive, because the rest declare nothing.
#[test]
fn writes_an_alias_under_the_property_it_stands_for() {
  let mut state = StateManager::default();

  let result = preprocess_object_properties(&expr("{ blockSize: '10px' }"), &mut state);

  assert_eq!(result.keys().cloned().collect::<Vec<_>>(), ["height"]);
  assert_eq!(result["height"], "10px");
}

/// A shorthand keeps the value on its own key and leaves the properties it
/// reaches empty, so only the shorthand itself reaches the answer.
#[test]
fn expands_a_shorthand_and_keeps_the_keys_that_carry_a_value() {
  let mut state = StateManager::default();

  let result = preprocess_object_properties(&expr("{ margin: '10px' }"), &mut state);

  assert_eq!(result.keys().cloned().collect::<Vec<_>>(), ["margin"]);
  assert_eq!(result["margin"], "10px");
}

/// A number stays a number through the expansion, so the unit is added by the
/// property that reads it rather than by the shorthand.
#[test]
fn a_number_reaches_the_expansion_as_a_number() {
  let mut state = StateManager::default();

  let result = preprocess_object_properties(&expr("{ margin: 10 }"), &mut state);

  assert_eq!(result["margin"].as_number(), Some(10.0));
}

#[test]
fn keeps_a_property_that_stands_for_itself() {
  let mut state = StateManager::default();

  let result = preprocess_object_properties(&expr("{ color: 'red' }"), &mut state);

  assert_eq!(result.keys().cloned().collect::<Vec<_>>(), ["color"]);
  assert_eq!(result["color"], "red");
}

#[test]
fn preprocesses_an_empty_object_to_nothing() {
  let mut state = StateManager::default();

  assert!(preprocess_object_properties(&expr("{}"), &mut state).is_empty());
}

#[test]
fn a_pipe_hands_its_value_to_each_step_in_turn() {
  let result = Pipe::create(2)
    .pipe(|value| value * 3)
    .pipe(|value| format!("{value}!"))
    .done();

  assert_eq!(result, "6!");
}

#[test]
fn a_pipe_of_no_step_answers_what_it_was_given() {
  assert_eq!(Pipe::new("value").done(), "value");
}
