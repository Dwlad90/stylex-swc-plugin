use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

test!(
    Default::default(),
    |tr| { ModuleTransformVisitor::new_test(tr.comments.clone()) },
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
        className: "page__c.base x9b88efe xdaaef87"
      };
    "#
);

test!(
    Default::default(),
    |tr| { ModuleTransformVisitor::new_test(tr.comments.clone()) },
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
      className: "page__c.base x9b88efe xdaaef87 page__c.test x39a4b12 xc3d638e page__c.wrapper x9b88efe x39a4b12 page__c.container xefba2c9 xc3d638e",
    };
  "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| { ModuleTransformVisitor::new_test(tr.comments.clone()) },
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
      const { className, style } = s.props(s.main, s.title);

      return (
        <main className={className} style={style}>
          Main
        </main>
      );
    }
  "#,
    r#"
    const _stylex$props = {
      className: "page__c.base x9b88efe xdaaef87 page__c.test x39a4b12 xc3d638e page__c.wrapper x9b88efe x39a4b12 page__c.container xefba2c9 xc3d638e",
    };

    export default function Home() {
      const { className, style } = _stylex$props;

      return <main className={className} style={style}>
          Main
        </main>;
    }
  "#
);
