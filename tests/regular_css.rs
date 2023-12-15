use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

test!(
    Default::default(),
    |tr| { ModuleTransformVisitor::new_test_classname(tr.comments.clone(), Option::None) },
    transform_simple_css_class,
    r#"
      import s from "@stylexjs/stylex";

      const c = s.create({
        base: {
          backgroundColor: 'red',
          color: 'blue',
        },
      });
    "#,
    r#"
    import s from "@stylexjs/stylex";

    const _stylex$props = {
        className: "page__c.base xrkmrrc xju2f9n"
      };
    "#
);

test!(
    Default::default(),
    |tr| { ModuleTransformVisitor::new_test_classname(tr.comments.clone(), Option::None) },
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
    import s from "@stylexjs/stylex";

    const _stylex$props = {
      className: "page__c.base x1e2nbdu x1118g2m page__c.test x15hxx75 x7z7khe page__c.wrapper x1e2nbdu x15hxx75 page__c.container x16ydxro x7z7khe",
    };
  "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| { ModuleTransformVisitor::new_test_classname(tr.comments.clone(), Option::None) },
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
    import s from "@stylexjs/stylex";

    const _stylex$props = {
      className: "page__c.base x1e2nbdu x1118g2m page__c.test x15hxx75 x7z7khe page__c.wrapper x1e2nbdu x15hxx75 page__c.container x16ydxro x7z7khe",
    };

    export default function Home() {
      const { className, style } = _stylex$props;

      return <main className={className} style={style}>
          Main
        </main>;
    }
  "#
);
