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
