use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test_transform,
};

use crate::evaluation::evaluation_module_transform::EvaluationStyleXLastStatementTransform;

#[test]
fn evaluates_constant_array_correctly() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXLastStatementTransform::default_with_pass(),
    r#"
    import react from 'react';
            const a = [1, 2];
            a;
        "#,
    r#"
          import react from 'react';
          const a = [1, 2];
          [1, 2];
        "#,
  )
}

#[test]
#[should_panic(expected = "Referenced value is not a constant")]
fn should_bail_out_when_array_is_mutated_via_push() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXLastStatementTransform::default_with_pass(),
    r#"
          import react from 'react';
            const a = [1, 2];
            a.push(3);
            a;
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Referenced value is not a constant")]
fn should_bail_out_when_array_is_mutated_via_assignment() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXLastStatementTransform::default_with_pass(),
    r#"
            import react from 'react';
            const a = [1, 2];
            a[0] = 3;
            a;
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Referenced value is not a constant")]
fn should_bail_out_when_object_is_mutated_via_object_assign() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXLastStatementTransform::default_with_pass(),
    r#"
            import react from 'react';
            const a = {bar: 'baz'};
            Object.assign(a, {foo: 1});
            a;
        "#,
    r#"
            const a = {bar: 'baz'};
            Object.assign(a, {foo: 1});
            ({bar: 'baz'});
        "#,
  )
}

#[test]
#[should_panic(expected = "Referenced value is not a constant")]
fn should_bail_out_when_array_is_mutated_via_update() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXLastStatementTransform::default_with_pass(),
    r#"
            import react from 'react';
            const a = [1, 2];
            ++a[0];
            a;
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Referenced value is not a constant")]
fn should_bail_out_when_primitive_is_mutated_via_delete() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |_| EvaluationStyleXLastStatementTransform::default_with_pass(),
    r#"
            import react from 'react';
            const a = {foo: 'bar'};
            delete a.foo;
            a;
        "#,
    r#""#,
  )
}
