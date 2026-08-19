use crate::utils::prelude::*;

// Upstream runs this whole `describe` through a `transform()` that pins
// `styleResolution: 'application-order'`. Without it the port measures the
// default resolution instead, and a border shorthand answers a different
// question than the one upstream asked.
fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_style_resolution(StyleResolution::ApplicationOrder)
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  stylex_call_with_computed_key_access,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_mixed_access,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_mixed_access_inline_export_default,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_mixed_access_export_default,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_mixed_access_inline_named_export,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_mixed_access_named_export,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_composition_of_external_styles,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_composition_border_shorthands_with_external_styles,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_using_exported_styles_with_pseudo_selectors_and_queries,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
