use stylex_shared::{shared::structures::plugin_pass::PluginPass, StyleXTransform};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_computed_key_access,
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
      stylex(styles[variant])
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_keeps_spaces_around_operators,
  r#"
      import stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        default: {
          margin: 'max(0px, (48px - var(--x16dnrjz)) / 2)',
        },
      });
      stylex(styles.default, props);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_composition_of_external_styles,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          color: 'red',
        },
      });
      stylex(styles.default, props);
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_using_exported_styles_with_pseudo_selectors_and_queries,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        default: {
          ':hover': {
            color: 'blue',
          },
          '@media (min-width: 1000px)': {
            backgroundColor: 'blue',
          },
        }
      });
      stylex(styles.default);
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_using_exported_styles_with_pseudo_selectors_and_queries_within_props,
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        default: {
          color: {
            ':hover': 'blue',
          },
          backgroundColor: {
            '@media (min-width: 1000px)': 'blue'
          },
        }
      });
      stylex(styles.default);
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_export_default,
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

      function MyComponent() {
        return (
          <>
            <div className={stylex(styles.foo)} />
            <div className={stylex(styles.bar)} />
            <CustomComponent xstyle={styles.foo} />
            <div className={stylex(styles.foo, styles.bar)} />
          </>
        );
      }

      export default MyComponent;
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_inline_export_default,
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

      export default function MyComponent() {
        return (
          <>
            <div className={stylex(styles.foo)} />
            <div className={stylex(styles.bar)} />
            <CustomComponent xstyle={styles.foo} />
            <div className={stylex(styles.foo, styles.bar)} />
          </>
        );
      }
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_named_export,
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

      function MyComponent() {
        return (
          <>
            <div className={stylex(styles.foo)} />
            <div className={stylex(styles.bar)} />
            <CustomComponent xstyle={styles.foo} />
            <div className={stylex(styles.foo, styles.bar)} />
          </>
        );
      }

      export { MyComponent };
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_inline_named_export,
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

      export function MyComponent() {
        return (
          <>
            <div className={stylex(styles.foo)} />
            <div className={stylex(styles.bar)} />
            <CustomComponent xstyle={styles.foo} />
            <div className={stylex(styles.foo, styles.bar)} />
          </>
        );
      }

"#
);
