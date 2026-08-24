//! Media query transform tests.

use crate::at_queries::media_query_transform::last_media_query_wins_transform;
use serde_json::{Value, json};
use swc_core::{
  atoms::Wtf8Atom,
  common::DUMMY_SP,
  ecma::ast::{Expr, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, Str},
};

// Helper functions

/// Helper function to create KeyValueProp from key and JSON value
fn create_key_value_prop(key: &str, value: Value) -> KeyValueProp {
  KeyValueProp {
    key: PropName::Str(Str {
      span: DUMMY_SP,
      value: Wtf8Atom::from(key),
      raw: None,
    }),
    value: Box::new(json_to_expr(value)),
  }
}

/// Convert JSON Value to SWC AST Expr
fn json_to_expr(value: Value) -> Expr {
  match value {
    Value::String(s) => Expr::Lit(swc_core::ecma::ast::Lit::Str(Str {
      span: DUMMY_SP,
      value: Wtf8Atom::from(s),
      raw: None,
    })),
    Value::Object(map) => {
      let props = map
        .into_iter()
        .map(|(k, v)| {
          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Str(Str {
              span: DUMMY_SP,
              value: Wtf8Atom::from(k),
              raw: None,
            }),
            value: Box::new(json_to_expr(v)),
          })))
        })
        .collect();

      Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
      })
    },
    _ => Expr::Lit(swc_core::ecma::ast::Lit::Str(Str {
      span: DUMMY_SP,
      value: Wtf8Atom::from(value.to_string()),
      raw: None,
    })),
  }
}

/// Helper to convert Wtf8Atom to String
fn convert_atom_to_string(atom: &Wtf8Atom) -> String {
  atom
    .as_str()
    .expect("Failed to convert Wtf8Atom to &str")
    .to_string()
}

/// Helper to convert KeyValueProp back to JSON (for backwards compatibility)
fn key_value_prop_to_json(props: &[KeyValueProp]) -> Value {
  let mut map = serde_json::Map::new();

  for prop in props {
    let key = match &prop.key {
      PropName::Str(s) => convert_atom_to_string(&s.value),
      PropName::Ident(id) => id.sym.to_string(),
      _ => continue,
    };

    let value = expr_to_json(&prop.value);
    map.insert(key, value);
  }

  Value::Object(map)
}

/// Helper to convert SWC AST Expr back to JSON
fn expr_to_json(expr: &Expr) -> Value {
  match expr {
    Expr::Lit(lit) => match lit {
      swc_core::ecma::ast::Lit::Str(s) => Value::String(convert_atom_to_string(&s.value)),
      _ => Value::String(format!("{:?}", lit)),
    },
    Expr::Object(obj) => {
      let mut result = serde_json::Map::new();
      for prop in &obj.props {
        if let PropOrSpread::Prop(p) = prop
          && let Prop::KeyValue(kv) = &**p
        {
          let key = match &kv.key {
            PropName::Str(s) => convert_atom_to_string(&s.value),
            PropName::Ident(id) => id.sym.to_string(),
            _ => continue,
          };
          let value = expr_to_json(&kv.value);
          result.insert(key, value);
        }
      }
      Value::Object(result)
    },
    _ => Value::String(format!("{:?}", expr)),
  }
}

#[cfg(test)]
mod media_query_transformer {
  use super::*;

  /// Test: basic usage: multiple widths
  #[test]
  fn basic_usage_multiple_widths() {
    let original_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (max-width: 1440px)": "1 / 4",
        "@media (max-width: 1024px)": "1 / 3",
        "@media (max-width: 768px)": "1 / -1"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 1024.01px) and (max-width: 1440px)": "1 / 4",
        "@media (min-width: 768.01px) and (max-width: 1024px)": "1 / 3",
        "@media (max-width: 768px)": "1 / -1"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap()
    );
  }

  /// Test: basic usage: nested query
  #[test]
  fn basic_usage_nested_query() {
    let original_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (max-width: 1440px)": {
          "@media (max-height: 1024px)": "1 / 3",
          "@media (max-height: 768px)": "1 / -1"
        },
        "@media (max-width: 1024px)": "1 / 3",
        "@media (max-width: 768px)": "1 / -1"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 1024.01px) and (max-width: 1440px)": {
          "@media (min-height: 768.01px) and (max-height: 1024px)": "1 / 3",
          "@media (max-height: 768px)": "1 / -1"
        },
        "@media (min-width: 768.01px) and (max-width: 1024px)": "1 / 3",
        "@media (max-width: 768px)": "1 / -1"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Nested queries should be transformed correctly"
    );
  }

  /// Test: basic usage: nested query
  #[test]
  fn basic_usage_nested_query_with_padding() {
    let original_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (max-width: 1440px)": {
          "@media (max-width: 1024px)": "1 / 3",
          "@media (max-width: 768px)": "1 / -1"
        },
        "@media (max-width: 1024px)": "1 / 3",
        "@media (max-width: 768px)": "1 / -1"
      },
      "padding": "10px"
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 1024.01px) and (max-width: 1440px)": {
          "@media (min-width: 768.01px) and (max-width: 1024px)": "1 / 3",
          "@media (max-width: 768px)": "1 / -1"
        },
        "@media (min-width: 768.01px) and (max-width: 1024px)": "1 / 3",
        "@media (max-width: 768px)": "1 / -1"
      },
      "padding": "10px"
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Nested queries with additional properties should be transformed correctly"
    );
  }

  /// Test: basic usage: complex object
  #[test]
  fn basic_usage_complex_object() {
    let original_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (max-width: 1440px)": "1 / 4",
        "@media (max-width: 1024px)": "1 / 3",
        "@media (max-width: 768px)": "1 / -1"
      },
      "grid": {
        "default": "1 / 2",
        "@media (max-width: 1440px)": "1 / 4"
      },
      "gridRow": {
        "default": "1 / 2",
        "padding": "10px"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 1024.01px) and (max-width: 1440px)": "1 / 4",
        "@media (min-width: 768.01px) and (max-width: 1024px)": "1 / 3",
        "@media (max-width: 768px)": "1 / -1"
      },
      "grid": {
        "default": "1 / 2",
        "@media (max-width: 1440px)": "1 / 4"
      },
      "gridRow": {
        "default": "1 / 2",
        "padding": "10px"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Complex object with multiple properties should be handled correctly"
    );
  }

  /// Test: basic usage: lots and lots of max-widths
  #[test]
  fn basic_usage_lots_and_lots_of_max_widths() {
    let original_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (max-width: 1440px)": "1 / 4",
        "@media (max-width: 1024px)": "1 / 3",
        "@media (max-width: 768px)": "1 / -1",
        "@media (max-width: 458px)": "1 / -1"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 1024.01px) and (max-width: 1440px)": "1 / 4",
        "@media (min-width: 768.01px) and (max-width: 1024px)": "1 / 3",
        "@media (min-width: 458.01px) and (max-width: 768px)": "1 / -1",
        "@media (max-width: 458px)": "1 / -1"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Multiple max-width queries should be transformed into ranges"
    );
  }

  /// Test: basic usage: lots and lots of min-widths
  #[test]
  fn basic_usage_lots_and_lots_of_min_widths() {
    let original_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 768px)": "1 / -1",
        "@media (min-width: 1024px)": "1 / 3",
        "@media (min-width: 1440px)": "1 / 4"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 768px) and (max-width: 1023.99px)": "1 / -1",
        "@media (min-width: 1024px) and (max-width: 1439.99px)": "1 / 3",
        "@media (min-width: 1440px)": "1 / 4"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Multiple min-width queries should be transformed into ranges with .99px boundaries"
    );
  }

  /// Test: basic usage: multiple heights
  #[test]
  fn basic_usage_multiple_heights() {
    let original_styles = json!({
      "foo": {
        "gridRow": {
          "default": "1 / 2",
          "@media (max-height: 1200px)": "1 / 4",
          "@media (max-height: 900px)": "1 / 3",
          "@media (max-height: 600px)": "1 / -1"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridRow": {
          "default": "1 / 2",
          "@media (min-height: 900.01px) and (max-height: 1200px)": "1 / 4",
          "@media (min-height: 600.01px) and (max-height: 900px)": "1 / 3",
          "@media (max-height: 600px)": "1 / -1"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Multiple height queries should be transformed into ranges"
    );
  }

  /// Test: basic usage: min/max heights
  #[test]
  fn basic_usage_min_max_heights() {
    let original_styles = json!({
      "foo": {
        "gridRow": {
          "default": "1 / 2",
          "@media (min-height: 1200px) and (max-height: 1400px)": "1 / 4",
          "@media (max-height: 900px)": "1 / 3",
          "@media (max-height: 600px)": "1 / -1"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridRow": {
          "default": "1 / 2",
          "@media (min-height: 1200px) and (max-height: 1400px)": "1 / 4",
          "@media (min-height: 600.01px) and (max-height: 900px)": "1 / 3",
          "@media (max-height: 600px)": "1 / -1"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Min/max height queries should preserve existing ranges"
    );
  }

  /// Test: single word condition
  #[test]
  fn single_word_condition() {
    let original_styles = json!({
      "mode": {
        "default": "normal",
        "@media (color)": "colorful",
        "@media (monochrome)": "grayscale"
      }
    });

    let expected_styles = json!({
      "mode": {
        "default": "normal",
        "@media (color) and (not (monochrome))": "colorful",
        "@media (monochrome)": "grayscale"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Single word conditions should include negations"
    );
  }

  /// Test: handles comma-separated (or) media queries
  #[test]
  fn handles_comma_separated_or_media_queries() {
    let original_styles = json!({
      "width": {
        "default": "100%",
        "@media screen, (max-width: 800px)": "80%",
        "@media (max-width: 500px)": "60%"
      }
    });

    let expected_styles = json!({
      "width": {
        "default": "100%",
        "@media (screen) and (not (max-width: 500px)), (min-width: 500.01px) and (max-width: 800px)": "80%",
        "@media (max-width: 500px)": "60%"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Comma-separated media queries should handle OR logic with negations"
    );
  }

  /// Test: basic usage: does not modify single queries
  #[test]
  fn basic_usage_does_not_modify_single_queries() {
    let original_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (max-width: 1440px)": "1 / 4"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (max-width: 1440px)": "1 / 4"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Single queries should not be modified"
    );
  }

  /// Test: ignores legacy media query syntax
  #[test]
  fn ignores_legacy_media_query_syntax() {
    let original_styles = json!({
      "width": "100%",
      "@media (min-width: 600px)": {
        "width": "50%"
      }
    });

    let expected_styles = json!({
      "width": "100%",
      "@media (min-width: 600px)": {
        "width": "50%"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Legacy media query syntax should be ignored"
    );
  }

  /// Test: mixed min/max width and height
  #[test]
  fn mixed_min_max_width_and_height() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1440px) and (max-height: 900px)": "1 / 4",
          "@media (max-width: 1024px)": "1 / 3",
          "@media (max-width: 768px)": "1 / -1"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (min-width: 1024.01px) and (max-width: 1440px) and (max-height: 900px)": "1 / 4",
          "@media (min-width: 768.01px) and (max-width: 1024px)": "1 / 3",
          "@media (max-width: 768px)": "1 / -1"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Mixed width/height queries should preserve height conditions"
    );
  }

  /// Test: mixed min/max width and height with only height changing
  #[test]
  fn mixed_min_max_width_and_height_with_only_height_changing() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1440px) and (max-height: 900px)": "1 / 4",
          "@media (max-height: 700px)": "1 / 3",
          "@media (max-height: 500px)": "1 / -1"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1440px) and (min-height: 700.01px) and (max-height: 900px)": "1 / 4",
          "@media (min-height: 500.01px) and (max-height: 700px)": "1 / 3",
          "@media (max-height: 500px)": "1 / -1"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Mixed queries with only height changing should transform height ranges"
    );
  }

  /// Test: mixed min/max width with disjoint ranges
  #[test]
  fn mixed_min_max_width_with_disjoint_ranges() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
          "@media (max-width: 800px) and (min-width: 600px)": "1 / 3"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (min-width: 900px) and (max-width: 1440px)": "1 / 4",
          "@media (min-width: 600px) and (max-width: 800px)": "1 / 3"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Mixed width disjoint ranges should be transformed with negation logic"
    );
  }

  /// Test: mixed min/max width with many disjoint ranges
  #[test]
  fn mixed_min_max_width_with_many_disjoint_ranges() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
          "@media (max-width: 800px) and (min-width: 600px)": "1 / 3",
          "@media (max-width: 500px) and (min-width: 400px)": "1 / 1",
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (min-width: 900px) and (max-width: 1440px)": "1 / 4",
          "@media (min-width: 600px) and (max-width: 800px)": "1 / 3",
          "@media (min-width: 400px) and (max-width: 500px)": "1 / 1",
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Mixed width many disjoint ranges should be transformed with negation logic"
    );
  }

  /// Test: mixed min/max width with mixed ranges
  #[test]
  fn mixed_min_max_width_with_mixed_ranges() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
          "@media (max-width: 1100px) and (min-width: 1000px)": "1 / 3",
          "@media (max-width: 500px) and (min-width: 400px)": "1 / 1"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))": "1 / 4",
          "@media (min-width: 1000px) and (max-width: 1100px)": "1 / 3",
          "@media (min-width: 400px) and (max-width: 500px)": "1 / 1"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Mixed width ranges with intersections should use OR logic"
    );
  }

  /// Test: mixed min/max width with intersecting ranges
  #[test]
  fn mixed_min_max_width_with_intersecting_ranges() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
          "@media (max-width: 1100px) and (min-width: 1000px)": "1 / 3"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))": "1 / 4",
          "@media (min-width: 1000px) and (max-width: 1100px)": "1 / 3"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Mixed width intersecting ranges should split with OR logic"
    );
  }

  /// Test: mixed min/max width with many intersecting ranges
  #[test]
  fn mixed_min_max_width_with_many_intersecting_ranges() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
          "@media (max-width: 1100px) and (min-width: 1000px)": "1 / 3",
          "@media (max-width: 1050px) and (min-width: 1010px)": "1 / -1"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))": "1 / 4",
          "@media ((min-width: 1000px) and (max-width: 1009.99px)) or ((min-width: 1050.01px) and (max-width: 1100px))": "1 / 3",
          "@media (min-width: 1010px) and (max-width: 1050px)": "1 / -1"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Mixed width many intersecting ranges should split with complex OR logic"
    );
  }

  /// Test: mixed min/max width with overlapping ranges
  #[test]
  fn mixed_min_max_width_with_overlapping_ranges() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
          "@media (max-width: 1040px) and (min-width: 600px)": "1 / 3"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (min-width: 1040.01px) and (max-width: 1440px)": "1 / 4",
          "@media (min-width: 600px) and (max-width: 1040px)": "1 / 3"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Mixed width overlapping ranges should split at boundaries"
    );
  }

  /// Test: handles and media queries
  #[test]
  fn handles_and_media_queries() {
    let original_styles = json!({
      "width": {
        "default": "100%",
        "@media (min-width: 900px)": "80%",
        "@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)": "50%"
      }
    });

    let expected_styles = json!({
      "width": {
        "default": "100%",
        "@media (min-width: 900px) and (not ((min-width: 500px) and (max-width: 899px) and (max-height: 300px)))": "80%",
        "@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)": "50%"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Complex AND combinations should handle negations correctly"
    );
  }

  /// Test: combination of keywords and rules
  #[test]
  fn combination_of_keywords_and_rules() {
    let original_styles = json!({
      "width": {
        "default": "100%",
        "@media screen and (min-width: 900px)": "80%",
        "@media print and (max-width: 500px)": "50%"
      }
    });

    let expected_styles = json!({
      "width": {
        "default": "100%",
        "@media ((screen) and (min-width: 900px) and (not (print))) or ((screen) and (min-width: 900px) and (not (max-width: 500px)))": "80%",
        "@media (print) and (max-width: 500px)": "50%"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap(),
      "Combination of keywords and rules should handle negations correctly"
    );
  }

  #[test]
  fn media_queries_with_em_units() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 90em) and (min-width: 60em)": "1 / 4",
          "@media (max-width: 70em) and (min-width: 65em)": "1 / 3"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media ((min-width: 60em) and (max-width: 64.99em)) or ((min-width: 70.01em) and (max-width: 90em))": "1 / 4",
          "@media (min-width: 65em) and (max-width: 70em)": "1 / 3"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap()
    );
  }

  #[test]
  fn media_queries_with_mixed_units() {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (max-width: 1200px) and (min-height: 50vh)": "1 / 4",
          "@media (max-width: 800px) and (min-height: 30vh)": "1 / 3"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (min-width: 800.01px) and (max-width: 1200px) and (min-height: 50vh)": "1 / 4",
          "@media (max-width: 800px) and (min-height: 30vh)": "1 / 3"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap()
    );
  }

  #[test]
  fn skips_range_simplification_for_media_queries_with_conflicting_units_in_same_dimension_across_queries()
   {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (min-width: 768px) and (max-width: 1200px)": "1 / 4",
          "@media (min-width: 50em)": "1 / 3"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (min-width: 768px) and (max-width: 1200px) and (not (min-width: 50em))": "1 / 4",
          "@media (min-width: 50em)": "1 / 3"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap()
    );
  }

  #[test]
  fn skips_range_simplification_for_media_queries_with_conflicting_units_in_same_dimension_and_query()
   {
    let original_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (min-width: 768px) and (max-width: 1200em)": "1 / 4",
          "@media (min-width: 50em)": "1 / 3"
        }
      }
    });

    let expected_styles = json!({
      "foo": {
        "gridColumn": {
          "default": "1 / 2",
          "@media (min-width: 768px) and (max-width: 1200em) and (not (min-width: 50em))": "1 / 4",
          "@media (min-width: 50em)": "1 / 3"
        }
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);

    assert_eq!(
      serde_json::to_string(&result_json).unwrap(),
      serde_json::to_string(&expected_styles).unwrap()
    );
  }

  #[test]
  fn handles_only_screen_media_queries_without_parenthesizing_the_media_type() {
    let original_styles = json!({
      "color": {
        "default": null,
        "@media only screen and (max-width: 600px)": "red",
        "@media only screen and (max-width: 400px)": "blue"
      }
    });

    let input_props = if let Value::Object(obj) = original_styles {
      obj
        .into_iter()
        .map(|(k, v)| create_key_value_prop(&k, v))
        .collect::<Vec<_>>()
    } else {
      vec![]
    };

    let result = last_media_query_wins_transform(&input_props);
    let result_json = key_value_prop_to_json(&result);
    let result_str = serde_json::to_string(&result_json).unwrap();

    assert!(!result_str.contains("only (screen)"));
    assert!(result_str.contains("only screen"));
  }
}

/// Every `@media` key is deleted and re-added, which moves it to the end of the
/// object — including when there is only one, where nothing else about the key
/// changes. The resulting order reaches the emitted CSS, so it is asserted
/// here.
#[test]
fn single_media_query_moves_after_the_default() {
  let props = vec![create_key_value_prop(
    "color",
    json!({
      "@media (max-width: 900px) and (min-width: 100px)": "red",
      "default": "blue",
    }),
  )];

  let result = last_media_query_wins_transform(&props);
  let color = match &*result[0].value {
    Expr::Object(object) => object,
    _ => panic!("expected an object value"),
  };

  let keys: Vec<String> = color
    .props
    .iter()
    .filter_map(|prop| match prop {
      PropOrSpread::Prop(prop) => match &**prop {
        Prop::KeyValue(kv) => match &kv.key {
          PropName::Str(key) => Some(convert_atom_to_string(&key.value)),
          _ => None,
        },
        _ => None,
      },
      _ => None,
    })
    .collect();

  assert_eq!(
    keys,
    vec![
      "default".to_string(),
      // The lone query is re-stringified too, which reorders its features.
      "@media (min-width: 100px) and (max-width: 900px)".to_string(),
    ]
  );
}

// ---------------------------------------------------------------------------
// Computed bounds at double precision — https://github.com/Dwlad90/stylex-swc-plugin/issues/1267
// ---------------------------------------------------------------------------

/// Run the transform over `styles` and return the keys of the single property
/// it contains, in order.
fn transformed_keys(styles: Value) -> Vec<String> {
  let props = match styles {
    Value::Object(obj) => obj
      .into_iter()
      .map(|(k, v)| create_key_value_prop(&k, v))
      .collect::<Vec<_>>(),
    _ => vec![],
  };

  let result = last_media_query_wins_transform(&props);
  let json = key_value_prop_to_json(&result);

  match json {
    Value::Object(obj) => match obj.into_iter().next() {
      Some((_, Value::Object(inner))) => inner.into_iter().map(|(k, _)| k).collect(),
      other => panic!("expected one property holding an object, got {other:?}"),
    },
    other => panic!("expected an object, got {other:?}"),
  }
}

#[cfg(test)]
mod computed_bounds_carry_the_authored_digits {
  use super::*;

  /// The reproduction from issue #1267. Each derived upper bound is
  /// `next - 0.01` in double precision, which is what
  /// `@stylexjs/babel-plugin@0.19.0` emits for the same input — a rounder
  /// string such as `28.8rem` is the bug, not the baseline.
  #[test]
  fn fractional_rem_breakpoints_derive_the_bounds_babel_derives() {
    assert_eq!(
      transformed_keys(json!({
        "minHeight": {
          "default": "100px",
          "@media (min-width: 25rem)": "200px",
          "@media (min-width: 28.81rem)": "300px",
          "@media (min-width: 32.88rem)": "400px"
        }
      })),
      vec![
        "default",
        "@media (min-width: 25rem) and (max-width: 28.799999999999997rem)",
        "@media (min-width: 28.81rem) and (max-width: 32.870000000000005rem)",
        "@media (min-width: 32.88rem)",
      ]
    );
  }

  /// Every bound in a chain of five fractional breakpoints, so that a passing
  /// assertion cannot be explained by the values that happen to survive single
  /// precision. Two of these five print short and three print long.
  #[test]
  fn every_bound_in_a_long_fractional_chain_matches() {
    assert_eq!(
      transformed_keys(json!({
        "width": {
          "default": "1px",
          "@media (min-width: 1.1rem)": "2px",
          "@media (min-width: 2.2rem)": "3px",
          "@media (min-width: 3.3rem)": "4px",
          "@media (min-width: 4.4rem)": "5px"
        }
      })),
      vec![
        "default",
        "@media (min-width: 1.1rem) and (max-width: 2.1900000000000004rem)",
        "@media (min-width: 2.2rem) and (max-width: 3.29rem)",
        "@media (min-width: 3.3rem) and (max-width: 4.390000000000001rem)",
        "@media (min-width: 4.4rem)",
      ]
    );
  }

  /// A fractional aspect-ratio survives the round trip.
  ///
  /// This is the test that says the fraction is reachable at all. The transform
  /// reprints every `@media` key it is handed, including one it had nothing to
  /// negate -- `combine_media_query_with_negations` returns the query unchanged
  /// and the printer still runs -- so a fraction held at the wrong width did not
  /// stay inside the parser. `16.5/9` reprinted as `16 / 9`, and a ratio of
  /// sixteen to nine is a different shape of screen from one of eleven to six.
  ///
  /// The second key is here for the saturating half of the same bug: past
  /// `i32::MAX` every numerator collapsed onto `2147483647`.
  ///
  /// The negation on the first key is the transform doing its own job -- the
  /// later query wins, so the earlier one is narrowed by its negation. It is
  /// incidental here, and left in rather than filtered out so the assertion
  /// reads against what the transform actually emits.
  #[test]
  fn a_fractional_aspect_ratio_reprints_at_the_width_it_was_written() {
    assert_eq!(
      transformed_keys(json!({
        "width": {
          "default": "1px",
          "@media (aspect-ratio: 16.5/9)": "2px",
          "@media (aspect-ratio: 3000000000/1)": "3px"
        }
      })),
      vec![
        "default",
        "@media (aspect-ratio: 16.5 / 9) and (not (aspect-ratio: 3000000000 / 1))",
        "@media (aspect-ratio: 3000000000 / 1)",
      ]
    );
  }

  /// Round breakpoints print identically at either width. Pinned so that the
  /// widening is shown to move only the values that were wrong.
  #[test]
  fn round_breakpoints_are_undisturbed() {
    assert_eq!(
      transformed_keys(json!({
        "width": {
          "default": "1px",
          "@media (min-width: 1024px)": "2px",
          "@media (min-width: 1440px)": "3px"
        }
      })),
      vec![
        "default",
        "@media (min-width: 1024px) and (max-width: 1439.99px)",
        "@media (min-width: 1440px)",
      ]
    );
  }
}
