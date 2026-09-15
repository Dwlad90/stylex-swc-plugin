use std::sync::Arc;

use swc_core::common::{BytePos, FileName, SourceMap as SwcSourceMap, Span};
use swc_core::ecma::ast::{Expr, IdentName, KeyValueProp, Lit, PropName, Str};
use swc_sourcemap::SourceMapBuilder;

use super::original_position_from_input_source_map;
use stylex_state::state_manager::StateManager;

const INPUT_CODE: &str = "\
const styles = create({
  root: { color: 'red' },
  other: { display: 'flex' },
});
";

const UNICODE_INPUT_CODE: &str = "\
const styles = create({
  emoji: '🚀', root: { color: 'red' },
});
";

fn key_value_prop_at(code_offset: usize, key: &str) -> (KeyValueProp, u32) {
  let lo = code_offset as u32;

  (
    KeyValueProp {
      key: PropName::Ident(IdentName::new(
        key.into(),
        Span::new(BytePos(lo + 1), BytePos(lo + 1 + key.len() as u32)),
      )),
      value: Box::new(Expr::Lit(Lit::Str(Str {
        span: swc_core::common::DUMMY_SP,
        value: "unused".into(),
        raw: None,
      }))),
    },
    lo,
  )
}

fn state_with_input(code: &str) -> StateManager {
  let cm = SwcSourceMap::default();
  let source_file = cm.new_source_file(
    Arc::new(FileName::Custom("input_source_map_fixture.tsx".to_string())),
    code.to_string(),
  );

  let mut state = StateManager::default();
  state.set_input_source_file(source_file);
  state
}

#[test]
fn maps_key_position_through_input_source_map() {
  // `other` sits on line index 2 (0-based), column 2 of the compiler input.
  let key_offset = match INPUT_CODE.find("other") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "other");

  let mut state = state_with_input(INPUT_CODE);

  // The input map records that input line 2 originates from line index 41 of
  // the original authored file (as if a macro loader shifted the code).
  let mut builder = SourceMapBuilder::new(None);
  builder.add(2, 0, 41, 0, Some("Original.tsx".into()), None, false);
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  let position = original_position_from_input_source_map(&style_node_path, &state);

  let position = match position {
    Some(position) => position,
    None => panic!("the key position must map back to the original source"),
  };

  assert_eq!(position.filename, "Original.tsx");
  assert_eq!(position.line_number, 42);
}

#[test]
fn maps_key_position_after_non_ascii_text_using_utf16_columns() {
  let key_offset = match UNICODE_INPUT_CODE.find("root") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "root");

  let mut state = state_with_input(UNICODE_INPUT_CODE);

  let line_start = match UNICODE_INPUT_CODE.rfind("  emoji") {
    Some(offset) => offset,
    None => panic!("fixture must contain the line start"),
  };
  let prefix = &UNICODE_INPUT_CODE[line_start..key_offset];
  let utf16_col = prefix.encode_utf16().count() as u32;
  let byte_col = prefix.len() as u32;

  assert_ne!(
    utf16_col, byte_col,
    "fixture must distinguish UTF-16 columns from byte offsets"
  );

  let mut builder = SourceMapBuilder::new(None);
  builder.add(
    1,
    utf16_col,
    10,
    0,
    Some("Original.tsx".into()),
    None,
    false,
  );
  builder.add(1, byte_col, 99, 0, Some("Wrong.tsx".into()), None, false);
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  let position = match original_position_from_input_source_map(&style_node_path, &state) {
    Some(position) => position,
    None => panic!("the key position must map using UTF-16 source-map columns"),
  };

  assert_eq!(position.filename, "Original.tsx");
  assert_eq!(position.line_number, 11);
}

#[test]
fn returns_none_without_input_source_map() {
  let key_offset = match INPUT_CODE.find("other") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "other");

  let state = state_with_input(INPUT_CODE);

  assert_eq!(
    original_position_from_input_source_map(&style_node_path, &state)
      .map(|position| (position.filename, position.line_number)),
    None,
    "without an input map the caller must fall back to source-text lookups"
  );
}

#[test]
fn returns_none_for_spans_outside_the_input_file() {
  let (style_node_path, _) = key_value_prop_at(1_000_000, "other");

  let mut state = state_with_input(INPUT_CODE);

  let mut builder = SourceMapBuilder::new(None);
  builder.add(2, 0, 41, 0, Some("Original.tsx".into()), None, false);
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  assert_eq!(
    original_position_from_input_source_map(&style_node_path, &state)
      .map(|position| (position.filename, position.line_number)),
    None,
    "foreign spans must not resolve through the input file"
  );
}

#[test]
fn returns_none_for_dummy_key_spans() {
  let key = "other";
  let style_node_path = KeyValueProp {
    key: PropName::Ident(IdentName::new(key.into(), swc_core::common::DUMMY_SP)),
    value: Box::new(Expr::Lit(Lit::Str(Str {
      span: swc_core::common::DUMMY_SP,
      value: "unused".into(),
      raw: None,
    }))),
  };

  let mut state = state_with_input(INPUT_CODE);

  let mut builder = SourceMapBuilder::new(None);
  builder.add(2, 0, 41, 0, Some("Original.tsx".into()), None, false);
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  assert_eq!(
    original_position_from_input_source_map(&style_node_path, &state)
      .map(|position| (position.filename, position.line_number)),
    None,
    "dummy spans carry no position and must not resolve"
  );
}

#[test]
fn returns_none_for_sparse_maps_whose_nearest_token_is_on_an_earlier_line() {
  // `other` sits on line index 2, but the map only knows about line 0:
  // `lookup_token`'s greatest-lower-bound search would hand that earlier
  // token back, which must not be committed as this key's position.
  let key_offset = match INPUT_CODE.find("other") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "other");

  let mut state = state_with_input(INPUT_CODE);

  let mut builder = SourceMapBuilder::new(None);
  builder.add(0, 0, 7, 0, Some("Original.tsx".into()), None, false);
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  assert_eq!(
    original_position_from_input_source_map(&style_node_path, &state)
      .map(|position| (position.filename, position.line_number)),
    None,
    "a token from an earlier line must fall back to source-text lookups"
  );
}

#[test]
fn returns_none_for_scheme_qualified_source_names() {
  let key_offset = match INPUT_CODE.find("other") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "other");

  let mut state = state_with_input(INPUT_CODE);

  let mut builder = SourceMapBuilder::new(None);
  builder.add(
    2,
    0,
    41,
    0,
    Some("webpack://app/src/Original.tsx".into()),
    None,
    false,
  );
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  assert_eq!(
    original_position_from_input_source_map(&style_node_path, &state)
      .map(|position| (position.filename, position.line_number)),
    None,
    "scheme-qualified sources are not filesystem paths and must fall back"
  );
}

#[test]
fn falls_back_to_the_state_filename_when_the_token_has_no_source() {
  let key_offset = match INPUT_CODE.find("other") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "other");

  let mut state = state_with_input(INPUT_CODE);

  let mut builder = SourceMapBuilder::new(None);
  builder.add(2, 0, 41, 0, None, None, false);
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  let position = match original_position_from_input_source_map(&style_node_path, &state) {
    Some(position) => position,
    None => panic!("a same-line token without a source must still resolve"),
  };

  assert_eq!(position.filename, state.get_filename().to_string());
  assert_eq!(position.line_number, 42);
}

/// Without the compiler's own input there is nothing to map from, so the caller
/// falls back to locating the key in the source text.
#[test]
fn returns_none_without_an_input_source_file() {
  let key_offset = match INPUT_CODE.find("other") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "other");

  assert!(
    original_position_from_input_source_map(&style_node_path, &StateManager::default()).is_none()
  );
}

/// The shortening of a path, and the annotation it is written into.
mod short_filenames {
  use std::rc::Rc;

  use indexmap::IndexMap;
  use rustc_hash::FxHashMap;
  use stylex_ast::ast::convertors::create_string_expr;
  use stylex_constants::constants::common::COMPILED_KEY;
  use stylex_state::{
    flat_compiled_styles_value::FlatCompiledStylesValue, functions::FunctionMap,
    state_manager::StateManager,
  };
  use stylex_structures::{
    stylex_env::JSFunction,
    stylex_options::{CheckModuleResolution, StyleXOptions},
  };

  use super::super::{
    create_short_filename, get_package_prefix, get_short_path, insert_compiled_entry,
  };

  fn short_filename_of(path: &str, state: &StateManager) -> String {
    create_short_filename(path, state, &mut FxHashMap::default())
  }

  /// A file inside a package is named for the package it came from.
  #[test]
  fn names_a_file_after_the_package_it_came_from() {
    let state = StateManager::default();

    assert_eq!(
      short_filename_of("/somewhere/node_modules/@acme/ui/src/Card.tsx", &state),
      "@acme:src/Card.tsx"
    );
  }

  /// A path that ends at the package directory names no package, so the file is
  /// shortened the ordinary way.
  #[test]
  fn names_no_package_for_a_path_that_ends_at_the_directory() {
    assert_eq!(get_package_prefix("/somewhere/node_modules/"), None);
    assert_eq!(get_package_prefix("/somewhere/src/Card.tsx"), None);
    assert_eq!(
      get_package_prefix("/somewhere/node_modules/acme/Card.tsx"),
      Some("acme".to_owned())
    );
  }

  /// A short path keeps the last two parts of the path it was given, which is
  /// enough for a person to tell two files apart.
  #[test]
  fn keeps_the_last_two_parts_of_a_path() {
    let state = StateManager::default();

    assert_eq!(get_short_path("/a/b/c/Card.tsx", &state), "c/Card.tsx");
    assert_eq!(get_short_path("Card.tsx", &state), "Card.tsx");
  }

  /// Where the project names a root, the path is given relative to it instead,
  /// because that is the path the author reads in their editor.
  #[test]
  fn names_a_path_relative_to_the_root_the_project_declares() {
    let state = StateManager::new(StyleXOptions::default().with_unstable_module_resolution(
      CheckModuleResolution::CommonJs {
        root_dir: Some("/project".to_owned()),
        theme_file_extension: None,
      },
    ));

    assert_eq!(
      get_short_path("/project/src/components/Card.tsx", &state),
      "src/components/Card.tsx"
    );

    // A path outside the root cannot be given relative to it, so the last two
    // parts stand in.
    assert_eq!(
      get_short_path("/elsewhere/src/Card.tsx", &state),
      "src/Card.tsx"
    );
  }

  /// A haste project names a file by itself, because a haste name is unique
  /// across the whole project.
  #[test]
  fn names_a_haste_file_by_itself() {
    let state = StateManager::new(StyleXOptions::default().with_unstable_module_resolution(
      CheckModuleResolution::Haste {
        root_dir: None,
        theme_file_extension: None,
      },
    ));

    assert_eq!(
      short_filename_of("/somewhere/src/Card.tsx", &state),
      "Card.tsx"
    );
  }

  /// A path outside any package and outside the working directory is shortened
  /// to its last two parts.
  #[test]
  fn shortens_a_path_that_belongs_to_no_package() {
    let state = StateManager::default();

    assert_eq!(
      short_filename_of("/elsewhere/src/Card.tsx", &state),
      "src/Card.tsx"
    );
  }

  fn annotation_of(state: &mut StateManager, line: usize) -> FlatCompiledStylesValue {
    let mut inner_map: IndexMap<String, Rc<FlatCompiledStylesValue>> = IndexMap::new();

    insert_compiled_entry(
      &mut inner_map,
      "/elsewhere/src/Card.tsx",
      line,
      state,
      &mut FxHashMap::default(),
      &FunctionMap::default(),
    );

    (*inner_map[COMPILED_KEY]).clone()
  }

  /// The annotation names the file and the line the namespace was written on.
  #[test]
  fn names_the_file_and_the_line() {
    let mut state = StateManager::default();

    assert_eq!(
      annotation_of(&mut state, 12),
      FlatCompiledStylesValue::String("src/Card.tsx:12".to_owned())
    );
  }

  /// Without a line there is nothing to point at, so the entry is the plain
  /// marker a build with no annotation writes.
  #[test]
  fn writes_the_plain_marker_without_a_line() {
    let mut state = StateManager::default();

    assert_eq!(
      annotation_of(&mut state, 0),
      FlatCompiledStylesValue::Bool(true)
    );
  }

  /// The second namespace of a call asks for the same file, and the shortening
  /// it already paid for is what answers.
  #[test]
  fn shortens_one_path_once() {
    let mut state = StateManager::default();

    let first = annotation_of(&mut state, 1);
    let second = annotation_of(&mut state, 1);

    assert_eq!(first, second);
    assert_eq!(
      state.cached_short_filename("/elsewhere/src/Card.tsx"),
      Some("src/Card.tsx")
    );
  }

  /// A project can name the file itself, and what that answers is what the
  /// annotation says.
  #[test]
  fn lets_the_project_name_the_file() {
    let mut state = StateManager::default();

    state.options.debug_file_path = Some(JSFunction::new(|_| {
      create_string_expr("named by the project")
    }));

    assert_eq!(
      annotation_of(&mut state, 3),
      FlatCompiledStylesValue::String("named by the project:3".to_owned())
    );
  }

  /// A name that spells nothing points at no file, so the entry is the plain
  /// marker.
  #[test]
  fn writes_the_plain_marker_for_a_name_that_spells_nothing() {
    let mut state = StateManager::default();

    state.options.debug_file_path = Some(JSFunction::new(|_| create_string_expr("")));

    assert_eq!(
      annotation_of(&mut state, 3),
      FlatCompiledStylesValue::Bool(true)
    );
  }
}

/// The `$$css` entry each namespace of a `create` call is given.
mod annotations {
  use std::rc::Rc;

  use indexmap::IndexMap;
  use log::Level;
  use rustc_hash::FxHashMap;
  use stylex_constants::constants::common::COMPILED_KEY;
  use stylex_state::{
    flat_compiled_styles_value::FlatCompiledStylesValue,
    functions::FunctionMap,
    state_manager::StateManager,
    types::{FlatCompiledStyles, StylesObjectMap},
  };
  use swc_core::ecma::ast::{CallExpr, Expr};

  use super::super::add_source_map_data;
  use crate::tests::capturing_logger::logged_at;
  use crate::tests::support::expr;

  /// The call `code` spells.
  fn call(code: &str) -> CallExpr {
    match expr(code) {
      Expr::Call(call) => call,
      other => panic!("the fixture {code} is not a call: {other:?}"),
    }
  }

  fn namespaces(names: &[&str]) -> StylesObjectMap {
    let mut obj: StylesObjectMap = IndexMap::new();

    for name in names {
      let mut styles: FlatCompiledStyles = IndexMap::new();

      styles.insert(
        "color".to_owned(),
        Rc::new(FlatCompiledStylesValue::String("xabc".to_owned())),
      );

      obj.insert((*name).to_owned(), Rc::new(styles));
    }

    obj
  }

  fn annotate(code: &str, names: &[&str]) -> StylesObjectMap {
    add_source_map_data(
      namespaces(names),
      &call(code),
      &mut StateManager::default(),
      &mut FxHashMap::default(),
      &FunctionMap::default(),
    )
  }

  fn marker_of(result: &StylesObjectMap, name: &str) -> FlatCompiledStylesValue {
    match result.get(name).and_then(|styles| styles.get(COMPILED_KEY)) {
      Some(marker) => (**marker).clone(),
      None => panic!("the namespace {name} carries no marker"),
    }
  }

  /// A namespace the call wrote is annotated, and the styles it held travel
  /// with it.
  #[test]
  fn annotates_every_namespace_the_call_wrote() {
    let result = annotate(
      "create({ root: { color: 'red' }, dark: {} })",
      &["root", "dark"],
    );

    assert_eq!(result.keys().cloned().collect::<Vec<_>>(), ["root", "dark"]);
    assert_eq!(
      result["root"]["color"].as_ref(),
      &FlatCompiledStylesValue::String("xabc".to_owned())
    );
    assert_eq!(
      marker_of(&result, "root"),
      FlatCompiledStylesValue::Bool(true)
    );
  }

  /// A namespace the call did not write cannot be placed, so it keeps the
  /// styles it had and no annotation is added.
  #[test]
  fn leaves_a_namespace_the_call_did_not_write_alone() {
    let result = annotate("create({ root: { color: 'red' } })", &["other"]);

    assert_eq!(result.keys().cloned().collect::<Vec<_>>(), ["other"]);
    assert!(!result["other"].contains_key(COMPILED_KEY));
  }

  /// A key the compiler cannot name is not a namespace it can place.
  #[test]
  fn leaves_a_namespace_under_a_key_it_cannot_name_alone() {
    let result = annotate("create({ [makeKey()]: { color: 'red' } })", &["root"]);

    assert!(!result["root"].contains_key(COMPILED_KEY));
  }

  /// A namespace written twice is one namespace, and the first of the two is
  /// the place it is annotated at.
  #[test]
  fn places_a_repeated_namespace_at_its_first_spelling() {
    let result = annotate("create({ root: { color: 'red' }, root: {} })", &["root"]);

    assert_eq!(result.len(), 1);
  }

  /// Without the source the namespace was written in there is no frame to read
  /// a line off, and the reader says so and points at the setting that tells
  /// more.
  #[test]
  fn reports_a_namespace_it_cannot_place() {
    let messages = logged_at(Level::Warn, || {
      annotate("create({ root: { color: 'red' } })", &["root"]);
    });

    assert!(
      messages.iter().any(|message| {
        message.contains("Could not retrieve source code frame")
          && message.contains("For more information enable debug logging")
      }),
      "the reader said nothing: {messages:?}"
    );
  }

  /// Asked for the detail, it names the namespace it could not place rather
  /// than pointing the reader at the setting they already turned on.
  #[test]
  fn names_the_namespace_it_cannot_place_when_asked_for_the_detail() {
    let messages = logged_at(Level::Debug, || {
      annotate("create({ root: { color: 'red' } })", &["root"]);
    });

    assert!(
      messages.iter().any(|message| {
        message.contains("Style node path") && !message.contains("enable debug logging")
      }),
      "the reader named no namespace: {messages:?}"
    );
  }

  /// The first argument of the call is the object of namespaces. Anything else
  /// is a call the compiler should never have reached this reader with.
  #[test]
  #[should_panic(expected = "Expected an object expression")]
  fn refuses_a_call_whose_argument_is_not_an_object() {
    annotate("create('a string')", &["root"]);
  }

  #[test]
  #[should_panic(expected = "add_source_map_data() should have 1 argument")]
  fn refuses_a_call_with_no_argument() {
    annotate("create()", &["root"]);
  }
}
