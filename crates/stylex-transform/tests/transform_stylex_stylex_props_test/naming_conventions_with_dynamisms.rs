use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
        .with_runtime_injection(),
    )
  })
}

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
            const variantStyle = `${variant}Variant`;

            return (
                <div {...stylex.props(styles[variantStyle])} />
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
            const variantStyle = `${variant}Variant`;

            return (
                <div {...stylex.props(styles[variantStyle])} />
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
            const variantStyle = `${variant}_variant`;

            return (
                <div {...stylex.props(styles[variantStyle])} />
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
            const variantStyle = `${variant}-variant`;

            return (
                <div {...stylex.props(styles[variantStyle])} />
            );
        }
    "#
);

stylex_test!(
  stylex_call_props_with_override_dynamic_styles,
  r#"
        import stylex from 'stylex';

        const styles = stylex.create({
          'primary-variant': {
              color: 'red'
          },
          secondaryVariant: {
              color: 'blue'
          },
        });

        function TestComponent({ variant }) {
            return (
                <div {...stylex.props(styles.secondaryVariant, styles[`${variant}-variant`])} />
            );
        }
    "#
);

stylex_test!(
  stylex_call_props_with_renaming_dynamic_styles_prop,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        import { fonts as f } from '@stylexjs/open-props/lib/fonts.stylex';

        const styles = stylex.create({
          text: {
            color: 'hotpink',
          },
        });

        const variants = stylex.create({
          small: {
            fontSize: f.size1
          },
          big: {
            fontSize: f.size7
          }
        })

        export function Text2({ children, size: s }) {
          return <div {...stylex.props(styles.text, variants[s])}>{children}</div>;
        }

    "#
);

stylex_test!(
  stylex_call_props_with_renaming_dynamic_styles_prop_and_conflict_import_name,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        import { fonts as foo } from '@stylexjs/open-props/lib/fonts.stylex';

        const styles = stylex.create({
          text: {
            color: 'hotpink',
          },
        });

        const variants = stylex.create({
          small: {
            fontSize: foo.size1
          },
          big: {
            fontSize: foo.size7
          }
        })

        export function Text2({ children, size: foo }) {
          return <div {...stylex.props(styles.text, variants[foo])}>{children}</div>;
        }
    "#
);

stylex_test!(
  stylex_call_props_with_varians_dynamic_key,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';

        const styles = stylex.create({
          defaultLink: {
            color: 'hotpink',
          },
        });

        export function Text({ children, variant}) {
          const variants = {
            default: { link: styles.defaultLink },
          };

          return <div {...stylex.props(variants[variant].link)}>{children}</div>;
        }
    "#
);

stylex_test!(
  stylex_call_props_with_varians_dynamic_key_directly,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';

        const styles = stylex.create({
          title: {
            color: 'hotpink',
          },
        });

        const variant = {
          title: [styles.title],
        };

        export function Text({ children, variant}) {
          return <div {...stylex.props(...variant.title)}>{children}</div>;
        }
    "#
);

stylex_test!(
  stylex_call_props_with_varians_dynamic_key_directly_v2,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';

        const styles = stylex.create({
          title: {
            color: 'hotpink',
          },
        });

        export function Text({ children, variant}) {
          const variant = {
            title: [styles.title],
          };

          return <div {...stylex.props(...variant.title)}>{children}</div>;
        }
    "#
);
