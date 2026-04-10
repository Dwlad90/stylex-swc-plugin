use crate::utils::prelude::*;

use crate::evaluation::evaluation_module_transform::EvaluationStyleXLastStatementTransform;

stylex_test_transform!(
  evaluates_constant_array_correctly,
  |_tr| EvaluationStyleXLastStatementTransform::default_with_pass(),
  r#"
    import react from 'react';
    const a = [1, 2];
    a;
  "#,
  r#"
    import react from 'react';
    const a = [1, 2];
    [1, 2];
  "#
);

stylex_test_panic!(
  should_bail_out_when_array_is_mutated_via_push,
  "Referenced value is not a constant",
  |_tr| EvaluationStyleXLastStatementTransform::default_with_pass(),
  r#"
    import react from 'react';
    const a = [1, 2];
    a.push(3);
    a;
  "#
);

stylex_test_panic!(
  should_bail_out_when_array_is_mutated_via_assignment,
  "Referenced value is not a constant",
  |_tr| EvaluationStyleXLastStatementTransform::default_with_pass(),
  r#"
    import react from 'react';
    const a = [1, 2];
    a[0] = 3;
    a;
  "#
);

stylex_test_panic!(
  should_bail_out_when_object_is_mutated_via_object_assign,
  "Referenced value is not a constant",
  |_tr| EvaluationStyleXLastStatementTransform::default_with_pass(),
  r#"
    import react from 'react';
    const a = {bar: 'baz'};
    Object.assign(a, {foo: 1});
      a;
  "#
);

stylex_test_panic!(
  should_bail_out_when_array_is_mutated_via_update,
  "Referenced value is not a constant",
  |_tr| EvaluationStyleXLastStatementTransform::default_with_pass(),
  r#"
    import react from 'react';
    const a = [1, 2];
    ++a[0];
    a;
  "#
);

stylex_test_panic!(
  should_bail_out_when_primitive_is_mutated_via_delete,
  "Referenced value is not a constant",
  |_tr| EvaluationStyleXLastStatementTransform::default_with_pass(),
  r#"
    import react from 'react';
    const a = {foo: 'bar'};
    delete a.foo;
    a;
  "#
);
