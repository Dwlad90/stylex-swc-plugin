use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_computed_key_access,
  r#"
      import stylex from 'stylex';
      stylex(styles[variant]);
      const styles = stylex.create({
        [0]: {
          color: 'red',
        },
        [1]: {
          backgroundColor: 'blue',
        }
      });
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_mixed_access,
  r#"
      import stylex from 'stylex';

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

      const styles = stylex.create({
        foo: {
          color: 'red',
        },
        bar: {
          backgroundColor: 'blue',
        }
      });
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_mixed_access_inline_export_default,
  r#"
      import stylex from 'stylex';

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

      const styles = stylex.create({
        foo: {
          color: 'red',
        },
        bar: {
          backgroundColor: 'blue',
        }
      });
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_mixed_access_export_default,
  r#"
      import stylex from 'stylex';

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

      const styles = stylex.create({
        foo: {
          color: 'red',
        },
        bar: {
          backgroundColor: 'blue',
        }
      });

      export default MyComponent;
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_mixed_access_inline_named_export,
  r#"
      import stylex from 'stylex';

      export function MyComponent() {
        return (
          <>
            <div className={stylex(styles.foo)} />
            <div className={stylex(styles.bar)} />
            <CustomComponent xstyle={styles.foo} />
            <div className={stylex(styles.foo, styles.bar)} />
          </>
        );
      };

      const styles = stylex.create({
        foo: {
          color: 'red',
        },
        bar: {
          backgroundColor: 'blue',
        }
      });
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_mixed_access_named_export,
  r#"
      import stylex from 'stylex';

      function MyComponent() {
        return (
          <>
            <div className={stylex(styles.foo)} />
            <div className={stylex(styles.bar)} />
            <CustomComponent xstyle={styles.foo} />
            <div className={stylex(styles.foo, styles.bar)} />
          </>
        );
      };

      const styles = stylex.create({
        foo: {
          color: 'red',
        },
        bar: {
          backgroundColor: 'blue',
        }
      });

      export { MyComponent };
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_composition_of_external_styles,
  r#"
      import stylex from 'stylex';
      stylex(styles.default, props);
      const styles = stylex.create({
        default: {
          color: 'red',
        },
      });
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_with_composition_border_shorthands_with_external_styles,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          borderTop: '5px solid blue',
          borderLeft: '5px solid blue',
          borderRight: '5px solid blue',
          borderBottom: '5px solid blue',
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_call_using_exported_styles_with_pseudo_selectors_and_queries,
  r#"
      import stylex from 'stylex';
      stylex(styles.default);
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
"#
);
