#[cfg(test)]
mod tests {

  use indexmap::IndexMap;
  use swc_core::{
    common::DUMMY_SP,
    ecma::{
      ast::{MemberExpr, MemberProp},
      utils::quote_ident,
    },
  };

  use crate::shared::{
    enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
    utils::{
      ast::{convertors::string_to_expression, factories::ident_factory},
      common::get_string_val_from_lit,
      core::{
        parse_nullable_style::{ResolvedArg, StyleObject},
        stylex::stylex,
      },
    },
  };

  fn create_style_object_args(args: &[&[(&str, FlatCompiledStylesValue)]]) -> Vec<ResolvedArg> {
    let mut result_args = vec![];
    for arg in args.iter() {
      // Arrange
      let mut object = IndexMap::new();

      for (key, value) in arg.iter() {
        object.insert(key.to_string(), Box::new(value.clone()));
      }

      result_args.push(ResolvedArg::StyleObject(
        StyleObject::Style(object),
        ident_factory("test"),
        MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(string_to_expression("test")),
          prop: MemberProp::Ident(quote_ident!("test")),
        },
      ))
    }

    result_args
  }

  #[test]
  fn stylex_inject() {
    let args = create_style_object_args(&[&[
      ("a", FlatCompiledStylesValue::String("aaa".into())),
      ("b", FlatCompiledStylesValue::String("bbb".into())),
      ("$$css", FlatCompiledStylesValue::Bool(true)),
    ]]);

    // Act
    let result = stylex(&args).expect("Expected result to be Some");

    // Assert
    let classname_string = result
      .as_stylex()
      .and_then(|expr| expr.as_lit())
      .and_then(get_string_val_from_lit)
      .expect("Expected classname_string to be Some");

    assert_eq!(classname_string, "aaa bbb");
  }

  #[test]
  fn merge_order() {
    let first = [
      ("a", FlatCompiledStylesValue::String("a".into())),
      (":hover__aa", FlatCompiledStylesValue::String("aa".into())),
      ("$$css", FlatCompiledStylesValue::Bool(true)),
    ];

    let second = [
      ("b", FlatCompiledStylesValue::String("b".into())),
      ("$$css", FlatCompiledStylesValue::Bool(true)),
    ];

    let third = [
      ("c", FlatCompiledStylesValue::String("c".into())),
      (":hover__cc", FlatCompiledStylesValue::String("cc".into())),
      ("$$css", FlatCompiledStylesValue::Bool(true)),
    ];

    let args = create_style_object_args(&[&first, &second, &third]);

    // Act
    let result = stylex(&args).expect("Expected result to be Some");

    // Assert
    let classname_string = result
      .as_stylex()
      .and_then(|expr| expr.as_lit())
      .and_then(get_string_val_from_lit)
      .expect("Expected classname_string to be Some");

    assert_eq!(classname_string, "a aa b c cc");
  }

  #[test]
  fn with_a_top_level_array_of_simple_overridden_classes() {
    let first = [
      (
        "backgroundColor",
        FlatCompiledStylesValue::String("nu7423ey".into()),
      ),
      ("$$css", FlatCompiledStylesValue::Bool(true)),
    ];

    let second = [
      (
        "backgroundColor",
        FlatCompiledStylesValue::String("gh25dzvf".into()),
      ),
      ("$$css", FlatCompiledStylesValue::Bool(true)),
    ];

    let args = create_style_object_args(&[&first, &second]);

    // Act
    let result = stylex(&args).expect("Expected result to be Some");

    // Assert
    let classname_string = result
      .as_stylex()
      .and_then(|expr| expr.as_lit())
      .and_then(get_string_val_from_lit)
      .expect("Expected classname_string to be Some");

    assert_eq!(classname_string, "gh25dzvf");
  }

  #[test]
  fn with_nested_arrays_and_pseudo_classes_overriding_things() {
    let first = [
      (
        "backgroundColor",
        FlatCompiledStylesValue::String("nu7423ey".into()),
      ),
      ("$$css", FlatCompiledStylesValue::Bool(true)),
    ];

    let second = [
      (
        "backgroundColor",
        FlatCompiledStylesValue::String("abcdefg".into()),
      ),
      (
        ":hover__backgroundColor",
        FlatCompiledStylesValue::String("ksdfmwjs".into()),
      ),
      ("$$css", FlatCompiledStylesValue::Bool(true)),
    ];

    let third = [
      ("color", FlatCompiledStylesValue::String("gofk2cf1".into())),
      (
        ":hover__backgroundColor",
        FlatCompiledStylesValue::String("rse6dlih".into()),
      ),
      ("$$css", FlatCompiledStylesValue::Bool(true)),
    ];

    let args = create_style_object_args(&[&first, &second, &third]);

    // Act
    let result = stylex(&args).expect("Expected result to be Some");

    // Assert
    let classname_string = result
      .as_stylex()
      .and_then(|expr| expr.as_lit())
      .and_then(get_string_val_from_lit)
      .expect("Expected classname_string to be Some");

    assert_eq!(classname_string, "abcdefg gofk2cf1 rse6dlih");
  }

  #[test]
  fn with_complicated_set_of_arguments() {
    let styles = create_style_object_args(&[
      &[
        (
          "backgroundColor",
          FlatCompiledStylesValue::String("nu7423ey".into()),
        ),
        (
          "borderColor",
          FlatCompiledStylesValue::String("tpe1esc0".into()),
        ),
        (
          "borderStyle",
          FlatCompiledStylesValue::String("gewhe1h2".into()),
        ),
        (
          "borderWidth",
          FlatCompiledStylesValue::String("gcovof34".into()),
        ),
        (
          "boxSizing",
          FlatCompiledStylesValue::String("bdao358l".into()),
        ),
        (
          "display",
          FlatCompiledStylesValue::String("rse6dlih".into()),
        ),
        (
          "listStyle",
          FlatCompiledStylesValue::String("s5oniofx".into()),
        ),
        (
          "marginTop",
          FlatCompiledStylesValue::String("m8h3af8h".into()),
        ),
        (
          "marginEnd",
          FlatCompiledStylesValue::String("l7ghb35v".into()),
        ),
        (
          "marginBottom",
          FlatCompiledStylesValue::String("kjdc1dyq".into()),
        ),
        (
          "marginStart",
          FlatCompiledStylesValue::String("kmwttqpk".into()),
        ),
        (
          "paddingTop",
          FlatCompiledStylesValue::String("srn514ro".into()),
        ),
        (
          "paddingEnd",
          FlatCompiledStylesValue::String("oxkhqvkx".into()),
        ),
        (
          "paddingBottom",
          FlatCompiledStylesValue::String("rl78xhln".into()),
        ),
        (
          "paddingStart",
          FlatCompiledStylesValue::String("nch0832m".into()),
        ),
        (
          "WebkitTapHighlightColor",
          FlatCompiledStylesValue::String("qi72231t".into()),
        ),
        (
          "textAlign",
          FlatCompiledStylesValue::String("cr00lzj9".into()),
        ),
        (
          "textDecoration",
          FlatCompiledStylesValue::String("rn8ck1ys".into()),
        ),
        (
          "whiteSpace",
          FlatCompiledStylesValue::String("n3t5jt4f".into()),
        ),
        (
          "wordWrap",
          FlatCompiledStylesValue::String("gh25dzvf".into()),
        ),
        ("zIndex", FlatCompiledStylesValue::String("g4tp4svg".into())),
        ("$$css", FlatCompiledStylesValue::Bool(true)),
      ],
      &[
        ("cursor", FlatCompiledStylesValue::String("fsf7x5fv".into())),
        (
          ":touchAction",
          FlatCompiledStylesValue::String("s3jn8y49".into()),
        ),
        ("$$css", FlatCompiledStylesValue::Bool(true)),
      ],
      &[
        (
          "outline",
          FlatCompiledStylesValue::String("icdlwmnq".into()),
        ),
        ("$$css", FlatCompiledStylesValue::Bool(true)),
      ],
      &[
        (
          "WebkitTapHighlightColor",
          FlatCompiledStylesValue::String("oajrlxb2".into()),
        ),
        ("cursor", FlatCompiledStylesValue::String("nhd2j8a9".into())),
        (
          ":touchAction",
          FlatCompiledStylesValue::String("f1sip0of".into()),
        ),
        ("$$css", FlatCompiledStylesValue::Bool(true)),
      ],
      &[
        (
          "textDecoration",
          FlatCompiledStylesValue::String("esuyzwwr".into()),
        ),
        (
          ":hover__textDecoration",
          FlatCompiledStylesValue::String("p8dawk7l".into()),
        ),
        ("$$css", FlatCompiledStylesValue::Bool(true)),
      ],
      &[
        (
          "backgroundColor",
          FlatCompiledStylesValue::String("g5ia77u1".into()),
        ),
        ("border", FlatCompiledStylesValue::String("e4t7hp5w".into())),
        ("color", FlatCompiledStylesValue::String("gmql0nx0".into())),
        ("cursor", FlatCompiledStylesValue::String("nhd2j8a9".into())),
        (
          "display",
          FlatCompiledStylesValue::String("q9uorilb".into()),
        ),
        (
          "fontFamily",
          FlatCompiledStylesValue::String("ihxqhq3m".into()),
        ),
        (
          "fontSize",
          FlatCompiledStylesValue::String("l94mrbxd".into()),
        ),
        (
          "lineHeight",
          FlatCompiledStylesValue::String("aenfhxwr".into()),
        ),
        (
          "marginTop",
          FlatCompiledStylesValue::String("kvgmc6g5".into()),
        ),
        (
          "marginEnd",
          FlatCompiledStylesValue::String("cxmmr5t8".into()),
        ),
        (
          "marginBottom",
          FlatCompiledStylesValue::String("oygrvhab".into()),
        ),
        (
          "marginStart",
          FlatCompiledStylesValue::String("hcukyx3x".into()),
        ),
        (
          "paddingTop",
          FlatCompiledStylesValue::String("jb3vyjys".into()),
        ),
        (
          "paddingEnd",
          FlatCompiledStylesValue::String("rz4wbd8a".into()),
        ),
        (
          "paddingBottom",
          FlatCompiledStylesValue::String("qt6c0cv9".into()),
        ),
        (
          "paddingStart",
          FlatCompiledStylesValue::String("a8nywdso".into()),
        ),
        (
          "textAlign",
          FlatCompiledStylesValue::String("i1ao9s8h".into()),
        ),
        (
          "textDecoration",
          FlatCompiledStylesValue::String("myohyog2".into()),
        ),
        (
          ":hover__color",
          FlatCompiledStylesValue::String("ksdfmwjs".into()),
        ),
        (
          ":hover__textDecoration",
          FlatCompiledStylesValue::String("gofk2cf1".into()),
        ),
        (
          ":active__transform",
          FlatCompiledStylesValue::String("lsqurvkf".into()),
        ),
        (
          ":active__transition",
          FlatCompiledStylesValue::String("bj9fd4vl".into()),
        ),
        ("$$css", FlatCompiledStylesValue::Bool(true)),
      ],
      &[
        (
          "display",
          FlatCompiledStylesValue::String("a8c37x1j".into()),
        ),
        ("width", FlatCompiledStylesValue::String("k4urcfbm".into())),
        ("$$css", FlatCompiledStylesValue::Bool(true)),
      ],
      &[
        (
          ":active__transform",
          FlatCompiledStylesValue::String("tm8avpzi".into()),
        ),
        ("$$css", FlatCompiledStylesValue::Bool(true)),
      ],
    ]);

    // Act
    let result = stylex(&styles).expect("Expected result to be Some");
    let repeat = stylex(&styles).expect("Expected result to be Some");

    // Assert
    let classname_string = result
      .as_stylex()
      .and_then(|expr| expr.as_lit())
      .and_then(get_string_val_from_lit)
      .expect("Expected classname_string to be Some");

    let repeat_classname_string = repeat
      .as_stylex()
      .and_then(|expr| expr.as_lit())
      .and_then(get_string_val_from_lit)
      .expect("Expected classname_string to be Some");

    assert_eq!(classname_string, repeat_classname_string);

    let mut parts: Vec<_> = classname_string.split_whitespace().collect();
    parts.sort();
    let result_classname_string = parts.join(" ");

    let mut parts: Vec<_> = "g5ia77u1 tpe1esc0 gewhe1h2 gcovof34 bdao358l a8c37x1j s5oniofx kvgmc6g5 cxmmr5t8 oygrvhab hcukyx3x jb3vyjys rz4wbd8a qt6c0cv9 a8nywdso oajrlxb2 i1ao9s8h myohyog2 n3t5jt4f gh25dzvf g4tp4svg nhd2j8a9 f1sip0of icdlwmnq e4t7hp5w gmql0nx0 ihxqhq3m l94mrbxd aenfhxwr k4urcfbm gofk2cf1 ksdfmwjs tm8avpzi bj9fd4vl".split_whitespace().collect();
    parts.sort();
    let expected_classname_string = parts.join(" ");

    assert_eq!(result_classname_string, expected_classname_string);
  }
}
