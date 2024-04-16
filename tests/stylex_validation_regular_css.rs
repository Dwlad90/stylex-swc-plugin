use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

test!(
  Default::default(),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  transform_simple_css_class,
  r#"
      import s from "@stylexjs/stylex";

      const c = s.create({
        base: {
          backgroundColor: 'red',
          color: 'blue',
        },
      });
    "#
);

test!(
  Default::default(),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  transform_multiple_simple_css_classes,
  r#"
    import s from "@stylexjs/stylex";

    const c = s.create({
      base: {
        color: "red",
        borderColor: "blue",
      },
      test:{
        borderColor: "pink",
        padding: "10px",
      },
      wrapper: {
        color: "red",
        borderColor: "pink",
      },
      container:{
        marginLeft: "10px",
        padding: "10px",
      }
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  transform_multiple_simple_css_classes_and_inject_to_react_component,
  r#"
    import s from "@stylexjs/stylex";

    const c = s.create({
      base: {
        color: "red",
        borderColor: "blue",
      },
      test:{
        borderColor: "pink",
        padding: "10px",
      },
      wrapper: {
        color: "red",
        borderColor: "pink",
      },
      container:{
        marginLeft: "10px",
        padding: "10px",
      }
    });

    export default function Home() {
      const { className, style } = s.props(c.base, c.test);

      return (
        <main className={className} style={style}>
          Main
        </main>
      );
    }
  "#
);
