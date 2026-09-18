use crate::utils::prelude::*;

use crate::evaluation::args_module_transform::ArgsStyleXTransform;

// An argument that is not an object of namespaces is read the way any other
// expression is read.
stylex_test_transform!(
  reads_a_plain_value_as_any_other_expression,
  |_tr| ArgsStyleXTransform::default_with_pass(),
  r#"
    const x = 'a string';
  "#,
  r#"
    'a string';
  "#
);

// A namespace whose value is a plain value reaches the map as that value.
stylex_test_transform!(
  reads_a_namespace_holding_a_plain_object,
  |_tr| ArgsStyleXTransform::default_with_pass(),
  r#"
    const x = { root: { color: 'red' } };
  "#,
  r#"
    ({ root: { color: 'red' } });
  "#
);

// A spread names no namespace, so there is nothing to read it as.
stylex_test_panic!(
  refuses_a_spread_among_the_namespaces,
  "The spread operator (...) is not supported in this context.",
  |_tr| ArgsStyleXTransform::default_with_pass(),
  r#"
    const x = { ...base, root: { color: 'red' } };
  "#
);

// A namespace has to be an object of declarations. A value of any other kind
// spells no declarations at all.
stylex_test_panic!(
  refuses_a_namespace_that_is_not_an_object,
  "A StyleX namespace must be an object.",
  |_tr| ArgsStyleXTransform::default_with_pass(),
  r#"
    const x = { root: 1 };
  "#
);

// A dynamic style is an arrow that answers a style object. An arrow that
// answers anything else is read the way any other expression is read.
stylex_test_transform!(
  reads_an_arrow_that_answers_no_object_as_any_other_expression,
  |_tr| ArgsStyleXTransform::default_with_pass(),
  r#"
    const x = { root: (color) => color };
  "#,
  r#"
    ({ root: (color) => color });
  "#
);
