use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  stylex_call_props_with_camel_case_key,
  r#"
    import stylex from 'stylex';

    const styles = stylex.create({
      primaryVariant: {
          padding: '0',
          margin: '0',
          listStyle: 'none',
          display: 'grid',
          gridAutoFlow: 'column',
          width: '100%',
          justifyContent: 'flex-start',
          borderBottomStyle: 'solid',
          borderBottomWidth: '1px',
      },
    });

    function TestComponent({ variant }) {
        return (
            <div {...stylex.props(styles[`${variant}Variant`])} />
        );
    }
  "#
);

stylex_test!(
  stylex_call_props_with_pascal_case_key,
  r#"
    import stylex from 'stylex';

    const styles = stylex.create({
      PrimaryVariant: {
          padding: '0',
          margin: '0',
          listStyle: 'none',
          display: 'grid',
          gridAutoFlow: 'column',
          width: '100%',
          justifyContent: 'flex-start',
          borderBottomStyle: 'solid',
          borderBottomWidth: '1px',
      },
    });

    function TestComponent({ variant }) {
        return (
            <div {...stylex.props(styles[`${variant}Variant`])} />
        );
    }
  "#
);

stylex_test!(
  stylex_call_props_with_snake_case_key,
  r#"
    import stylex from 'stylex';

    const styles = stylex.create({
      'primary_variant': {
          padding: '0',
          margin: '0',
          listStyle: 'none',
          display: 'grid',
          gridAutoFlow: 'column',
          width: '100%',
          justifyContent: 'flex-start',
          borderBottomStyle: 'solid',
          borderBottomWidth: '1px',
      },
    });

    function TestComponent({ variant }) {
        return (
            <div {...stylex.props(styles[`${variant}_variant`])} />
        );
    }
  "#
);

stylex_test!(
  stylex_call_props_with_kebab_case_key,
  r#"
    import stylex from 'stylex';

    const styles = stylex.create({
      'primary-variant': {
          padding: '0',
          margin: '0',
          listStyle: 'none',
          display: 'grid',
          gridAutoFlow: 'column',
          width: '100%',
          justifyContent: 'flex-start',
          borderBottomStyle: 'solid',
          borderBottomWidth: '1px',
      },
    });

    function TestComponent({ variant }) {
        return (
            <div {...stylex.props(styles[`${variant}-variant`])} />
        );
    }
  "#
);
