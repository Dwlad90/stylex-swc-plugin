use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    importing_file_with_stylex_suffix_works,
    r#"
    import stylex from 'stylex';
    // import { MyTheme } from 'otherFile.stylex';
    const styles = stylex.create({
      green: {
        color: "red",
      }
    });
    stylex(styles.green);
    "#
);
