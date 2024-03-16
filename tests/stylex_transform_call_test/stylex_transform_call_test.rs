use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    empty_stylex_call,
    r#"
      import stylex from 'stylex';
      stylex();
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    basic_stylex_call,
    r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    stylex(styles.red);
  "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    stylex_call_with_number,
    r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        0: {
          color: 'red',
        },
        1: {
          backgroundColor: 'blue',
        }
      });
      stylex(styles[0], styles[1]);
"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    stylex_call_with_computed_number,
    r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      [0]: {
        color: 'red',
      },
      [1]: {
        backgroundColor: 'blue',
      }
    });
    stylex(styles[0], styles[1]);
"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    stylex_call_with_computed_number_without_declaration,
    r#"
      import {create} from '@stylexjs/stylex';
      const styles = create({
        [0]: {
          color: 'red',
        },
        [1]: {
          backgroundColor: 'blue',
        }
      });
      stylex(styles[0], styles[1]);
"#
);

test!(
  Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
  stylex_call_with_multiple_namespaces,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        color: 'red',
      },
    });
    const otherStyles = stylex.create({
      default: {
        backgroundColor: 'blue',
      }
    });
    stylex(styles.default, otherStyles.default);
"#
);


test!(
  Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
  stylex_call_within_variable_declarations,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: { color: 'red' }
    });
    const a = function() {
      return stylex(styles.foo);
    }
    a
"#
);


test!(
  Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
  stylex_call_with_styles_variable_assignment,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        color: 'red',
      },
      bar: {
        backgroundColor: 'blue',
      }
    });
    stylex(styles.foo, styles.bar);
    const foo = styles;
    foo;
"#
);


test!(
  Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
  stylex_call_with_short_form_properties,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5
      }
    });
    stylex(styles.foo);
"#
);


test!(
  Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
  stylex_call_with_exported_short_form_properties,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        padding: 5
      }
    });
    stylex(styles.foo);
"#
);

test!(
  Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
  stylex_call_keeps_only_the_styles_that_are_needed,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5
      },
      bar: {
        paddingBlock: 10,
      },
      baz: {
        paddingTop: 7,
      }
    });
    stylex(styles.foo);
    stylex(styles.foo, styles.bar);
    stylex(styles.bar, styles.foo);
    stylex(styles.foo, styles.bar, styles.baz);
    stylex(styles.foo, somethingElse);
"#
);