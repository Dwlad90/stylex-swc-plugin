use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::{ecma::transforms::testing::test, plugin::proxies::PluginCommentsProxy};

test!(
    Default::default(),
    |_| { ModuleTransformVisitor::new_test(PluginCommentsProxy) },
    transform_simple_css_class,
    r#"
      import s from "@stylexjs/stylex";

      const c = s.create({
        base: {
          color: "red",
          borderColor: "blue",
        },
      });
    "#,
    r#"
      const _stylex$props = {
        className: "page__c.base b0d669ae 57b485a9"
      };
    "#
);



test!(
  Default::default(),
  |_| { ModuleTransformVisitor::new_test(PluginCommentsProxy) },
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
  "#,
  r#"
    const _stylex$props = {
      className: "page__c.base b0d669ae 57b485a9 page__c.test 49381fc6 10be0f0e page__c.wrapper b0d669ae 49381fc6 page__c.container f3732245 10be0f0e",
    };
  "#
);
