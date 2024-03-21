use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

// test!(
//   Syntax::Typescript(TsConfig {
//       tsx: true,
//       ..Default::default()
//   }),
//   |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), PluginPass::default(), Option::None),
//   does_nothing_when_stylex_not_imported,
//   r#"
//       export const styles = stylex.create({
//           foo: {
//               color: 'red'
//           },
//       });
//   "#
// );