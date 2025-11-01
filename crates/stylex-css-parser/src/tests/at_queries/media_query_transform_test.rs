//! Media query transform tests.
//! Media query transform tests for CSS-in-JS functionality

use crate::at_queries::{
  media_query::MediaQuery,
  media_query_transform::{
    last_media_query_wins_transform, last_media_query_wins_transform_internal,
  },
};
use serde_json::{Value, json};
use swc_core::atoms::Wtf8Atom;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Expr, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, Str};

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
    }
    _ => Expr::Lit(swc_core::ecma::ast::Lit::Str(Str {
      span: DUMMY_SP,
      value: Wtf8Atom::from(value.to_string()),
      raw: None,
    })),
  }
}

/// Helper to convert Wtf8Atom to String
fn atom_to_string(atom: &Wtf8Atom) -> String {
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
      PropName::Str(s) => atom_to_string(&s.value),
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
      swc_core::ecma::ast::Lit::Str(s) => Value::String(atom_to_string(&s.value)),
      _ => Value::String(format!("{:?}", lit)),
    },
    Expr::Object(obj) => {
      let mut result = serde_json::Map::new();
      for prop in &obj.props {
        if let PropOrSpread::Prop(p) = prop
          && let Prop::KeyValue(kv) = &**p
        {
          let key = match &kv.key {
            PropName::Str(s) => atom_to_string(&s.value),
            PropName::Ident(id) => id.sym.to_string(),
            _ => continue,
          };
          let value = expr_to_json(&kv.value);
          result.insert(key, value);
        }
      }
      Value::Object(result)
    }
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
        "@media screen and (not (max-width: 500px)), (min-width: 500.01px) and (max-width: 800px)": "80%",
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
      "Mixed width disjoint ranges should not be modified"
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
  #[ignore]
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
  #[ignore]
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
        "@media screen and (min-width: 900px) and (not (print and (max-width: 500px)))": "80%",
        "@media print and (max-width: 500px)": "50%"
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

  // Helper functions for backwards compatibility with internal API

  /// Internal tests (for legacy MediaQuery API compatibility)

  #[test]
  fn internal_basic_usage_multiple_widths() {
    let queries = vec![
      MediaQuery::parser()
        .parse_to_end("@media (max-width: 1440px)")
        .unwrap(),
      MediaQuery::parser()
        .parse_to_end("@media (max-width: 1024px)")
        .unwrap(),
      MediaQuery::parser()
        .parse_to_end("@media (max-width: 768px)")
        .unwrap(),
    ];

    let result = last_media_query_wins_transform_internal(queries);

    // Simple test - just verify the transformation doesn't crash
    assert!(result.len() >= 3);
  }

  #[test]
  fn internal_does_not_modify_single_queries() {
    let queries = vec![
      MediaQuery::parser()
        .parse_to_end("@media (max-width: 1440px)")
        .unwrap(),
    ];

    let result = last_media_query_wins_transform_internal(queries);
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn internal_empty_queries() {
    let queries = vec![];
    let result = last_media_query_wins_transform_internal(queries);
    assert_eq!(result.len(), 0);
  }

  #[test]
  fn media_queries_with_em_units() {
    let original_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (max-width: 90em) and (min-width: 60em)": "1 / 4",
        "@media (max-width: 70em) and (min-width: 65em)": "1 / 3"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media ((min-width: 60em) and (max-width: 64.99em)) or ((min-width: 70.01em) and (max-width: 90em))": "1 / 4",
        "@media (min-width: 65em) and (max-width: 70em)": "1 / 3"
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
      "gridColumn": {
        "default": "1 / 2",
        "@media (max-width: 1200px) and (min-height: 50vh)": "1 / 4",
        "@media (max-width: 800px) and (min-height: 30vh)": "1 / 3"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 800.01px) and (max-width: 1200px) and (min-height: 50vh)": "1 / 4",
        "@media (max-width: 800px) and (min-height: 30vh)": "1 / 3"
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
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 768px) and (max-width: 1200px)": "1 / 4",
        "@media (min-width: 50em)": "1 / 3"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 768px) and (max-width: 1200px) and (not (min-width: 50em))": "1 / 4",
        "@media (min-width: 50em)": "1 / 3"
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
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 768px) and (max-width: 1200em)": "1 / 4",
        "@media (min-width: 50em)": "1 / 3"
      }
    });

    let expected_styles = json!({
      "gridColumn": {
        "default": "1 / 2",
        "@media (min-width: 768px) and (max-width: 1200em) and (not (min-width: 50em))": "1 / 4",
        "@media (min-width: 50em)": "1 / 3"
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
}
