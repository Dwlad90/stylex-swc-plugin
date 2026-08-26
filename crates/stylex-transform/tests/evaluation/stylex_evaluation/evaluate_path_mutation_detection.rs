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

// A mutating *method* is not among the cases below, and the omission is the
// rule. It was here, refused by a predicate the fold guard carried; the fold
// now evaluates every array method and the mutation is answered where the
// reference compiler answers it — by disqualifying the binding, which the
// module visitor collects and this harness does not run. The behaviour is
// pinned at the transform level, where the visitor is, in
// `transform_stylex_create_test::mutating_methods_and_bindings` and
// `::named_array_receivers`. What is left below is the mutation the evaluator
// itself refuses to evaluate: an assignment, an update and a delete, none of
// which is a call.

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
