use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test_transform,
};

use crate::evaluation::evaluation_module_transform::EvaluationStyleXTransform;

#[test]
fn function_with_a_single_params() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const double = x => x * 2;
        "#,
    r#"
            4;
        "#,
  )
}

#[test]
fn function_with_a_two_params() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const double = (a, b) => a + b;
        "#,
    r#"
            9;
        "#,
  )
}

#[test]
fn array_map() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const x = [1, 2, 3].map(x => x * 2);
        "#,
    r#"
            [2, 4, 6];
        "#,
  )
}

#[test]
fn array_filter() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const x = [1, 2, 3].filter(x => x % 2 === 0);
        "#,
    r#"
            [2]
        "#,
  )
}

#[test]
fn array_join() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const x = [1, 2, 3].join(", ");
            const x = ['a', 'b', 'c'].join(":");
        "#,
    r#"
            "1, 2, 3";
            "a:b:c";
        "#,
  )
}

#[test]
fn array_map_and_filter() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const x = [1, 2, 3].map(x => x * 2).filter(x => x % 2 === 0);
            const y = [1, 2, 3].map(x => x ** 2).filter(x => x % 3 !== 0).map(x => x * 3);
            const y = [1, 2, 3].map(x => x ** 2).filter(x => x % 3 !== 0).map(x => x * 3).filter(x => x % 4 === 0);
        "#,
    r#"
            [2, 4, 6];
            [3, 12];
            [12];
        "#,
  )
}

#[test]
fn array_methods() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const a = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(x => x * 2);
            const b = [1, 2, 3, 4, 5, 6, 7, 8, 9].filter(x => x % 3 !== 0);
            const c = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(x => x * 2).filter(x => x % 3 !== 0);
        "#,
    r#"
            [2, 4, 6, 8, 10, 12, 14, 16, 18];
            [1, 2, 4, 5, 7, 8];
            [2, 4, 8, 10, 14, 16];
        "#,
  )
}

#[test]
fn object_methods() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const a = Object.keys({a: 1, b: 2, c: 3});
            const b = Object.values({a: 1, b: 2, c: 3});
            const c = Object.entries({a: 1, b: 2, c: 3});
            const d = Object.fromEntries([["a", 1], ["b", 2], ["c", 3]]);
        "#,
    r#"
            ["a", "b", "c"];
            [1, 2, 3];
            [["a", 1], ["b", 2], ["c", 3]];
            ({
                a: 1,
                b: 2,
                c: 3,
            });
        "#,
  )
}

#[test]
fn object_entries() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const x = Object.entries({a: 1, b: 2, c: 4}).filter((entry) => entry[1] % 2 === 0);
            const x = Object.fromEntries(Object.entries({a: 1, b: 2, c: 4}).filter((entry) => entry[1] % 2 === 0));
        "#,
    r#"
            [
                ["b", 2],
                ["c", 4],
            ];

            ({
                b: 2,
                c: 4,
            });
        "#,
  )
}

#[test]
fn methods_called_by_string_should_be_bind() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const x = "".concat("10px"," ").concat("10px");
            const x = "abc".charCodeAt(0);
            const x = "abc".charCodeAt(2);
        "#,
    r#"
            "10px 10px";

            97;
            99;
        "#,
  )
}

#[test]
fn math_pow() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
            const x = Math.pow(2, 3);
            const x = Math.pow(8, 4);
        "#,
    r#"
            8;
            4096;
        "#,
  )
}

#[test]
fn math_round() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
          const x = Math.round(2);
          const x = Math.round(2.5);
          const x = Math.round(2.51);
          const x = Math.round(2.49);
        "#,
    r#"
            2;
            3;
            3;
            2;
        "#,
  )
}

#[test]
fn math_ceil() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
          const x = Math.ceil(2);
          const x = Math.ceil(2.5);
          const x = Math.ceil(2.51);
          const x = Math.ceil(2.49);
        "#,
    r#"
            2;
            3;
            3;
            3;
        "#,
  )
}

#[test]
fn math_floor() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
          const x = Math.floor(2);
          const x = Math.floor(2.5);
          const x = Math.floor(2.51);
          const x = Math.floor(2.49);
          const x = Math.floor(2.99);
          const x = Math.floor(3);
        "#,
    r#"
            2;
            2;
            2;
            2;
            2;
            3;
        "#,
  )
}

#[test]
fn math_min() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
          const x = Math.min(2);
          const x = Math.min(3,1,2);
          const x = Math.min(3,1,2, [0.5], 5);
          const x = Math.min(3,1,2, ...[0.5, 0.1, 0.3]);
        "#,
    r#"
            2;
            1;
            0.5;
            0.1;
        "#,
  )
}

#[test]
fn math_max() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
          const x = Math.max(2);
          const x = Math.max(2,3,1);
          const x = Math.max(3,1,2, [5], 0.5);
          const x = Math.max(3,1,2, ...[5, 0.1, 0.3], 4);
        "#,
    r#"
            2;
            3;
            5;
            5;
        "#,
  )
}

#[test]
fn math_complicated() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXTransform::default_with_pass(),
    r#"
          const x =  Math.min(Math.round(16 / Math.pow(1.2, 3) / 0.16) / 100);
          const x =  Math.round(100 * (Math.round(16 / Math.pow(1.2, 3) / 0.16) / 100 - (16 * (Math.round(20 / Math.pow(1.333, 3) / 0.16) / 100 - Math.round(15 / Math.pow(1.2, 2) / 0.16) / 10016)) / (1240 - 320) * (320 / 16))) / 100
        "#,
    r#"
          0.58;
          0.4;
        "#,
  )
}
