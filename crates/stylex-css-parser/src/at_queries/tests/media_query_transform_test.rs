use swc_core::ecma::ast::{Expr, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, Str};

#[cfg(test)]
mod tests {
  use crate::at_queries::media_query_transform::last_media_query_wins_transform;

  use super::*;

  fn make_kv(key: &str, value: Expr) -> KeyValueProp {
    KeyValueProp {
      key: PropName::Str(Str {
        value: key.into(),
        span: Default::default(),
        raw: None,
      }),
      value: Box::new(value),
    }
  }

  fn kv_vec_to_string(vec: &[KeyValueProp]) -> String {
    vec
      .iter()
      .map(|kv| {
        let key = match &kv.key {
          PropName::Str(s) => s.value.clone(),
          _ => "".into(),
        };
        let value = match kv.value.as_ref() {
          Expr::Lit(lit) => format!("{:?}", lit),
          Expr::Object(_) => "[object]".into(),
          _ => format!("{:?}", kv.value),
        };
        format!("{}:{}", key, value)
      })
      .collect::<Vec<_>>()
      .join(",")
  }

  // AST factory utils for tests
  fn media_prop(media: &str, value: &str) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::from(make_kv(media, Expr::from(value)))))
  }

  fn default_prop(value: &str) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::from(make_kv("default", Expr::from(value)))))
  }

  fn object(props: Vec<PropOrSpread>) -> Expr {
    Expr::Object(ObjectLit {
      span: Default::default(),
      props,
    })
  }

  fn nested_prop(key: &str, props: Vec<PropOrSpread>) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::from(make_kv(key, object(props)))))
  }

  #[test]
  fn basic_usage_multiple_widths() {
    // Matches JS test('basic usage: multiple widths')
    let input = vec![make_kv(
      "gridColumn",
      object(vec![
        default_prop("1 / 2"),
        media_prop("@media (max-width: 1440px)", "1 / 4"),
        media_prop("@media (max-width: 1024px)", "1 / 3"),
        media_prop("@media (max-width: 768px)", "1 / -1"),
      ]),
    )];
    let expected = vec![make_kv(
      "gridColumn",
      object(vec![
        default_prop("1 / 2"),
        media_prop("@media (min-width: 1024.01px) and (max-width: 1440px)", "1 / 4"),
        media_prop("@media (min-width: 768.01px) and (max-width: 1024px)", "1 / 3"),
        media_prop("@media (max-width: 768px)", "1 / -1"),
      ]),
    )];
    let output = last_media_query_wins_transform(&input);
    assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  #[ignore]
  fn basic_usage_nested_query() {
    // Matches JS test.skip('basic usage: nested query')
    let grid_column_nested = vec![
        media_prop("@media (max-height: 1024px)", "1 / 3"),
        media_prop("@media (max-height: 768px)", "1 / -1"),
    ];
    let input = vec![make_kv(
      "gridColumn",
      object(vec![
        default_prop("1 / 2"),
        nested_prop("@media (max-width: 1440px)", grid_column_nested.clone()),
        media_prop("@media (max-width: 1024px)", "1 / 3"),
        media_prop("@media (max-width: 768px)", "1 / -1"),
      ]),
    )];
    let expected_grid_column_nested = vec![
        media_prop("@media (min-height: 768.01px) and (max-height: 1024px)", "1 / 3"),
        media_prop("@media (max-height: 768px)", "1 / -1"),
    ];
    let expected = vec![make_kv(
      "gridColumn",
      object(vec![
        default_prop("1 / 2"),
        nested_prop("@media (min-width: 1024.01px) and (max-width: 1440px)", expected_grid_column_nested.clone()),
        media_prop("@media (min-width: 768.01px) and (max-width: 1024px)", "1 / 3"),
        media_prop("@media (max-width: 768px)", "1 / -1"),
      ]),
    )];
    let output = last_media_query_wins_transform(&input);
    assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  #[ignore]
  fn basic_usage_nested_query_with_padding() {
    // Matches JS test.skip('basic usage: nested query') with padding
    let grid_column_nested = vec![
        media_prop("@media (max-width: 1024px)", "1 / 3"),
        media_prop("@media (max-width: 768px)", "1 / -1"),
    ];
    let input = vec![
      make_kv(
        "gridColumn",
        object(vec![
          default_prop("1 / 2"),
          nested_prop("@media (max-width: 1440px)", grid_column_nested.clone()),
          media_prop("@media (max-width: 1024px)", "1 / 3"),
          media_prop("@media (max-width: 768px)", "1 / -1"),
        ]),
      ),
      make_kv("padding", Expr::from("10px")),
    ];
    let expected_grid_column_nested = vec![
        media_prop("@media (min-width: 768.01px) and (max-width: 1024px)", "1 / 3"),
        media_prop("@media (max-width: 768px)", "1 / -1"),
    ];
    let expected = vec![
      make_kv(
        "gridColumn",
        object(vec![
          default_prop("1 / 2"),
          nested_prop("@media (min-width: 1024.01px) and (max-width: 1440px)", expected_grid_column_nested.clone()),
          media_prop("@media (min-width: 768.01px) and (max-width: 1024px)", "1 / 3"),
          media_prop("@media (max-width: 768px)", "1 / -1"),
        ]),
      ),
      make_kv("padding", Expr::from("10px")),
    ];
    let output = last_media_query_wins_transform(&input);
    assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  #[ignore]
  fn basic_usage_complex_object() {
    // Matches JS test.skip('basic usage: complex object')
    let input = vec![
      make_kv(
        "gridColumn",
        object(vec![
          default_prop("1 / 2"),
          media_prop("@media (max-width: 1440px)", "1 / 4"),
          media_prop("@media (max-width: 1024px)", "1 / 3"),
          media_prop("@media (max-width: 768px)", "1 / -1"),
        ]),
      ),
      make_kv(
        "grid",
        object(vec![
          default_prop("1 / 2"),
          media_prop("@media (max-width: 1440px)", "1 / 4"),
        ]),
      ),
      make_kv(
        "gridRow",
        object(vec![
          default_prop("1 / 2"),
          media_prop("padding", "10px"),
        ]),
      ),
    ];
    let expected = vec![
      make_kv(
        "gridColumn",
        object(vec![
          default_prop("1 / 2"),
          media_prop("@media (min-width: 1024.01px) and (max-width: 1440px)", "1 / 4"),
          media_prop("@media (min-width: 768.01px) and (max-width: 1024px)", "1 / 3"),
          media_prop("@media (max-width: 768px)", "1 / -1"),
        ]),
      ),
      make_kv(
        "grid",
        object(vec![
          default_prop("1 / 2"),
          media_prop("@media (max-width: 1440px)", "1 / 4"),
        ]),
      ),
      make_kv(
        "gridRow",
        object(vec![
          default_prop("1 / 2"),
          media_prop("padding", "10px"),
        ]),
      ),
    ];
    let output = last_media_query_wins_transform(&input);
    assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  #[ignore]
  fn basic_usage_lots_and_lots_of_max_widths() {
    // Matches JS test.skip('basic usage: lots and lots of max-widths')
    let input = vec![make_kv(
      "gridColumn",
      object(vec![
        default_prop("1 / 2"),
        media_prop("@media (max-width: 1440px)", "1 / 4"),
        media_prop("@media (max-width: 1024px)", "1 / 3"),
        media_prop("@media (max-width: 768px)", "1 / -1"),
        media_prop("@media (max-width: 458px)", "1 / -1"),
      ]),
    )];
    let expected = vec![make_kv(
      "gridColumn",
      object(vec![
        default_prop("1 / 2"),
        media_prop("@media (min-width: 1024.01px) and (max-width: 1440px)", "1 / 4"),
        media_prop("@media (min-width: 768.01px) and (max-width: 1024px)", "1 / 3"),
        media_prop("@media (min-width: 458.01px) and (max-width: 768px)", "1 / -1"),
        media_prop("@media (max-width: 458px)", "1 / -1"),
      ]),
    )];
    let output = last_media_query_wins_transform(&input);
    assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  #[ignore]
  fn basic_usage_lots_and_lots_of_min_widths() {
    // Matches JS test.skip('basic usage: lots and lots of min-widths')
    let input = vec![make_kv(
      "gridColumn",
      object(vec![
        default_prop("1 / 2"),
        media_prop("@media (min-width: 768px)", "1 / -1"),
        media_prop("@media (min-width: 1024px)", "1 / 3"),
        media_prop("@media (min-width: 1440px)", "1 / 4"),
      ]),
    )];
    let expected = vec![make_kv(
      "gridColumn",
      object(vec![
        default_prop("1 / 2"),
        media_prop("@media (min-width: 768px) and (max-width: 1023.99px)", "1 / -1"),
        media_prop("@media (min-width: 1024px) and (max-width: 1439.99px)", "1 / 3"),
        media_prop("@media (min-width: 1440px)", "1 / 4"),
      ]),
    )];
    let output = last_media_query_wins_transform(&input);
    assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  fn basic_usage_does_not_modify_single_queries() {
    let input = vec![
      make_kv("default", Expr::from("1 / 2")),
      make_kv("@media (max-width: 1440px)", Expr::from("1 / 4")),
    ];
    let expected = vec![
      make_kv("default", Expr::from("1 / 2")),
      make_kv("@media (max-width: 1440px)", Expr::from("1 / 4")),
    ];
    let output = last_media_query_wins_transform(&input);
    assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  fn ignores_legacy_media_query_syntax() {
    let input = vec![
      make_kv("width", Expr::from("100%")),
      make_kv(
        "@media (min-width: 600px)",
        Expr::Object(ObjectLit {
          span: Default::default(),
          props: vec![PropOrSpread::Prop(Box::new(Prop::from(make_kv(
            "width",
            Expr::from("50%"),
          ))))],
        }),
      ),
    ];
    let expected = vec![
      make_kv("width", Expr::from("100%")),
      make_kv(
        "@media (min-width: 600px)",
        Expr::Object(ObjectLit {
          span: Default::default(),
          props: vec![PropOrSpread::Prop(Box::new(Prop::from(make_kv(
            "width",
            Expr::from("50%"),
          ))))],
        }),
      ),
    ];
    let output = last_media_query_wins_transform(&input);
    assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  fn basic_usage_lots_of_max_widths() {
    let input = vec![
      make_kv("default", Expr::from("1 / 2")),
      make_kv("@media (max-width: 1440px)", Expr::from("1 / 4")),
      make_kv("@media (max-width: 1024px)", Expr::from("1 / 3")),
      make_kv("@media (max-width: 768px)", Expr::from("1 / -1")),
      make_kv("@media (max-width: 458px)", Expr::from("1 / -1")),
    ];
    let expected = vec![
      make_kv("default", Expr::from("1 / 2")),
      make_kv(
        "@media (min-width: 1024.01px) and (max-width: 1440px)",
        Expr::from("1 / 4"),
      ),
      make_kv(
        "@media (min-width: 768.01px) and (max-width: 1024px)",
        Expr::from("1 / 3"),
      ),
      make_kv(
        "@media (min-width: 458.01px) and (max-width: 768px)",
        Expr::from("1 / -1"),
      ),
      make_kv("@media (max-width: 458px)", Expr::from("1 / -1")),
    ];
    let output = last_media_query_wins_transform(&input);
    assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  #[ignore]
  fn basic_usage_multiple_heights() {
      // Matches JS test.skip('basic usage: multiple heights')
      let grid_row_props = vec![
          default_prop("1 / 2"),
          media_prop("@media (max-height: 1200px)", "1 / 4"),
          media_prop("@media (max-height: 900px)", "1 / 3"),
          media_prop("@media (max-height: 600px)", "1 / -1"),
      ];
      let input = vec![make_kv(
          "foo",
          object(vec![
              nested_prop("gridRow", grid_row_props.clone()),
          ]),
      )];
      let expected_grid_row_props = vec![
          default_prop("1 / 2"),
          media_prop("@media (min-height: 900.01px) and (max-height: 1200px)", "1 / 4"),
          media_prop("@media (min-height: 600.01px) and (max-height: 900px)", "1 / 3"),
          media_prop("@media (max-height: 600px)", "1 / -1"),
      ];
      let expected = vec![make_kv(
          "foo",
          object(vec![
              nested_prop("gridRow", expected_grid_row_props.clone()),
          ]),
      )];
      let output = last_media_query_wins_transform(&input);
      assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  #[ignore]
  fn basic_usage_min_max_heights() {
      // Matches JS test.skip('basic usage: min max heights')
      let grid_row_props = vec![
          default_prop("1 / 2"),
          media_prop("@media (min-height: 1200px) and (max-height: 1400px)", "1 / 4"),
          media_prop("@media (max-height: 900px)", "1 / 3"),
          media_prop("@media (max-height: 600px)", "1 / -1"),
      ];
      let input = vec![make_kv(
          "foo",
          object(vec![
              nested_prop("gridRow", grid_row_props.clone()),
          ]),
      )];
      let expected_grid_row_props = vec![
          default_prop("1 / 2"),
          media_prop("@media (min-height: 1200px) and (max-height: 1400px)", "1 / 4"),
          media_prop("@media (min-height: 600.01px) and (max-height: 900px)", "1 / 3"),
          media_prop("@media (max-height: 600px)", "1 / -1"),
      ];
      let expected = vec![make_kv(
          "foo",
          object(vec![
              nested_prop("gridRow", expected_grid_row_props.clone()),
          ]),
      )];
      let output = last_media_query_wins_transform(&input);
      assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
  }

  #[test]
  #[ignore]
  fn single_word_condition() {
        // Matches JS test.skip('single word condition')
        let input = vec![make_kv(
            "mode",
            object(vec![
                default_prop("normal"),
                media_prop("@media (color)", "colorful"),
                media_prop("@media (monochrome)", "grayscale"),
            ]),
        )];
        let expected = vec![make_kv(
            "mode",
            object(vec![
                default_prop("normal"),
                media_prop("@media (color) and (not (monochrome))", "colorful"),
                media_prop("@media (monochrome)", "grayscale"),
            ]),
        )];
        let output = last_media_query_wins_transform(&input);
        assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
    }

    #[test]
    #[ignore]
    fn handles_comma_separated_or_media_queries() {
        // Matches JS test.skip('handles comma-separated (or) media queries')
        let input = vec![make_kv(
            "width",
            object(vec![
                default_prop("100%"),
                media_prop("@media screen, (max-width: 800px)", "80%"),
                media_prop("@media (max-width: 500px)", "60%"),
            ]),
        )];
        let expected = vec![make_kv(
            "width",
            object(vec![
                default_prop("100%"),
                media_prop("@media screen and (not (max-width: 500px)), (min-width: 500.01px) and (max-width: 800px)", "80%"),
                media_prop("@media (max-width: 500px)", "60%"),
            ]),
        )];
        let output = last_media_query_wins_transform(&input);
        assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
    }

    #[test]
    #[ignore]
    fn handles_and_media_queries() {
        // Matches JS test.skip('handles and media queries')
        let input = vec![make_kv(
            "width",
            object(vec![
                default_prop("100%"),
                media_prop("@media (min-width: 900px)", "80%"),
                media_prop("@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)", "50%"),
            ]),
        )];
        let expected = vec![make_kv(
            "width",
            object(vec![
                default_prop("100%"),
                media_prop("@media (min-width: 900px) and (not ((min-width: 500px) and (max-width: 899px) and (max-height: 300px)))", "80%"),
                media_prop("@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)", "50%"),
            ]),
        )];
        let output = last_media_query_wins_transform(&input);
        assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
    }

    #[test]
    #[ignore]
    fn combination_of_keywords_and_rules() {
        // Matches JS test.skip('combination of keywords and rules')
        let input = vec![make_kv(
            "width",
            object(vec![
                default_prop("100%"),
                media_prop("@media screen and (min-width: 900px)", "80%"),
                media_prop("@media print and (max-width: 500px)", "50%"),
            ]),
        )];
        let expected = vec![make_kv(
            "width",
            object(vec![
                default_prop("100%"),
                media_prop("@media screen and (min-width: 900px) and (not (print and (max-width: 500px)))", "80%"),
                media_prop("@media print and (max-width: 500px)", "50%"),
            ]),
        )];
        let output = last_media_query_wins_transform(&input);
        assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
    }

    #[test]
    #[ignore]
    fn basic_usage_mixed_min_max_width_and_height() {
        // Matches JS test.skip('basic usage: mixed min max width and height')
        let grid_column_props = vec![
            default_prop("1 / 2"),
            media_prop("@media (max-width: 1440px) and (max-height: 900px)", "1 / 4"),
            media_prop("@media (max-width: 1024px)", "1 / 3"),
            media_prop("@media (max-width: 768px)", "1 / -1"),
        ];
        let input = vec![make_kv(
            "foo",
            object(vec![
                nested_prop("gridColumn", grid_column_props.clone()),
            ]),
        )];
        let expected_grid_column_props = vec![
            default_prop("1 / 2"),
            media_prop("@media (min-width: 1024.01px) and (max-width: 1440px) and (max-height: 900px)", "1 / 4"),
            media_prop("@media (min-width: 768.01px) and (max-width: 1024px)", "1 / 3"),
            media_prop("@media (max-width: 768px)", "1 / -1"),
        ];
        let expected = vec![make_kv(
            "foo",
            object(vec![
                nested_prop("gridColumn", expected_grid_column_props.clone()),
            ]),
        )];
        let output = last_media_query_wins_transform(&input);
        assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
    }

    #[test]
    #[ignore]
    fn basic_usage_mixed_min_max_width_and_height_with_only_height_changing() {
        // Matches JS test.skip('basic usage: mixed min max width and height with only height changing')
        let grid_column_props = vec![
            default_prop("1 / 2"),
            media_prop("@media (max-width: 1440px) and (max-height: 900px)", "1 / 4"),
            media_prop("@media (max-height: 700px)", "1 / 3"),
            media_prop("@media (max-height: 500px)", "1 / -1"),
        ];
        let input = vec![make_kv(
            "foo",
            object(vec![
                nested_prop("gridColumn", grid_column_props.clone()),
            ]),
        )];
        let expected_grid_column_props = vec![
            default_prop("1 / 2"),
            media_prop("@media (max-width: 1440px) and (min-height: 700.01px) and (max-height: 900px)", "1 / 4"),
            media_prop("@media (min-height: 500.01px) and (max-height: 700px)", "1 / 3"),
            media_prop("@media (max-height: 500px)", "1 / -1"),
        ];
        let expected = vec![make_kv(
            "foo",
            object(vec![
                nested_prop("gridColumn", expected_grid_column_props.clone()),
            ]),
        )];
        let output = last_media_query_wins_transform(&input);
        assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
    }

    #[test]
    #[ignore]
    fn basic_usage_mixed_min_max_width_with_ranges() {
        // Matches JS test.skip('basic usage: mixed min max width with ranges')
        let grid_column_props = vec![
            default_prop("1 / 2"),
            media_prop("@media (max-width: 1440px) and (min-width: 900px)", "1 / 4"),
            media_prop("@media (max-width: 1040px) and (min-width: 600px)", "1 / 3"),
        ];
        let input = vec![make_kv(
            "foo",
            object(vec![
                nested_prop("gridColumn", grid_column_props.clone()),
            ]),
        )];
        let expected_grid_column_props = vec![
            default_prop("1 / 2"),
            media_prop("@media (min-width: 900px) and (max-width: 1440px) and (not ((min-width: 600px) and (max-width: 1040px)))", "1 / 4"),
            media_prop("@media (min-width: 600px) and (max-width: 1040px)", "1 / 3"),
        ];
        let expected = vec![make_kv(
            "foo",
            object(vec![
                nested_prop("gridColumn", expected_grid_column_props.clone()),
            ]),
        )];
        let output = last_media_query_wins_transform(&input);
        assert_eq!(kv_vec_to_string(&output), kv_vec_to_string(&expected));
    }
}
