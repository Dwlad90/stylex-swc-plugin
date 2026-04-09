use crate::utils::prelude::*;

stylex_test_panic!(
  invalid_key_non_static,
  "Referenced constant is not defined.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      [root]: {
        backgroundColor: 'red',
      }
    });
  "#
);

stylex_test_panic!(
  invalid_rule_non_object,
  "A StyleX namespace must be an object.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      namespace: false,
    });
  "#
);

stylex_test_panic!(
  invalid_rule_spread,
  "Object spreads are not allowed in create() calls.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const shared = { foo: { color: 'red' } };
    const styles = stylex.create({
      ...shared,
      bar: { color: 'blue' }
    });
  "#
);

stylex_test!(
  valid_rule_object,
  r#"
    const styles = stylex.create({
      namespace: {},
    });
  "#
);

stylex_test_panic!(
  invalid_dynamic_rule_default_object_value,
  "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      dynamic: (props = {}) => ({
        color: props.color,
      }),
    });
  "#
);

stylex_test_panic!(
  invalid_dynamic_rule_default_string_value,
  "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      dynamic: (color = 'red') => ({
        color,
      }),
    });
  "#
);

stylex_test_panic!(
  invalid_dynamic_rule_destructuring,
  "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      dynamic: ({ color }) => ({
        color,
      }),
    });
  "#
);

stylex_test_panic!(
  invalid_dynamic_rule_rest_param,
  "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      dynamic: (...rest) => ({
        color: rest[0],
      }),
    });
  "#
);

stylex_test!(
  valid_dynamic_rule,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      dynamic: (backgroundColor) => ({
        backgroundColor,
      }),
    });
  "#
);

stylex_test_panic!(
  invalid_dynamic_rule_with_block_body,
  "Block statement is not allowed in Dynamic Style functions",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const styles = stylex.create({
      button: () => {
        return {
            justifyContent: 'center',
        };
      },
    });
  "#
);
