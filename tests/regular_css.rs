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
        className: "page__c.base b0d669ae 57b485a9"
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
      className: "page__c.base b0d669ae 57b485a9 page__c.test 49381fc6 10be0f0e page__c.wrapper b0d669ae 49381fc6 page__c.container f3732245 10be0f0e",
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
      className: "page__c.base b0d669ae 57b485a9 page__c.test 49381fc6 10be0f0e page__c.wrapper b0d669ae 49381fc6 page__c.container f3732245 10be0f0e",
    };

    export default function Home() {
      const { className, style } = _stylex$props;

      return <main className={className} style={style}>
          Main
        </main>;
    }
  "#
);
