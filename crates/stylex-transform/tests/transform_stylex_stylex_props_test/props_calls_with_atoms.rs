use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

fn stylex_transform_dev_debug(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_filename(FileName::Real("/tmp/Foo.js".into()))
        .with_dev(true)
        .with_enable_dev_class_names(true)
        .with_enable_debug_class_names(true)
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  inline_static_styles_match_stylex_create,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(css.display.flex);
  "#
);

stylex_test!(
  inline_static_supports_leading_underscore_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(css.padding._16px);
  "#
);

stylex_test!(
  inline_static_supports_computed_key_syntax,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(css.width['calc(100% - 20px)']);
  "#
);

stylex_test!(
  inline_css_supports_default_imports,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(css.color.blue);
  "#
);

stylex_test!(
  dedupes_duplicate_properties_across_create_and_atoms,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    const styles = stylex.create({
      base: { color: 'red', backgroundColor: 'white' },
    });
    stylex.props(styles.base, css.color.blue, css.backgroundColor.white);
  "#
);

stylex_test!(
  dynamic_style,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(css.color(color));
  "#
);

stylex_test!(
  inline_static_inline_dynamic_coexist,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(css.display.flex, css.color(color));
  "#
);

stylex_test!(
  inline_static_create_dynamic,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    const styles = stylex.create({
      opacity: (o) => ({ opacity: o }),
    });
    stylex.props(css.display.flex, styles.opacity(0.5));
  "#
);

stylex_test!(
  inline_dynamic_create_dynamic,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    const styles = stylex.create({
      opacity: (o) => ({ opacity: o }),
    });
    stylex.props(css.color(color), styles.opacity(0.5));
  "#
);

stylex_test!(
  atoms_with_and_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(css.display.flex, isActive && css.color.blue);
  "#
);

stylex_test!(
  atoms_with_ternary_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(isActive ? css.color.blue : css.color.red);
  "#
);

stylex_test!(
  atoms_with_conditional_and_create_styles,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    const styles = stylex.create({
      active: { color: 'blue' },
    });
    stylex.props(css.display.flex, isActive && styles.active);
  "#
);

stylex_test!(
  dynamic_atoms_with_ternary_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(isActive ? css.color(activeColor) : css.backgroundColor(inactiveBg));
  "#
);

stylex_test!(
  atoms_mixed_with_unknown_external_styles,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    import { externalStyles } from './other.stylex';
    stylex.props(css.color.blue, externalStyles.root);
  "#
);

stylex_test!(
  atoms_with_dynamic_conditional_inside_function,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    function Button({ isActive, color }) {
      return stylex.props(
        css.padding._8px,
        css.borderRadius._4px,
        isActive && css.color(color),
      );
    }
  "#
);

stylex_test!(
  atoms_with_variable_reference_forces_bailout,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    function Component({ xstyle }) {
      return stylex.props(css.display.flex, xstyle);
    }
  "#
);

stylex_test!(
  static_atoms_inside_function_declaration,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    function Component() {
      return stylex.props(css.display.flex, css.color.blue);
    }
  "#
);

stylex_test!(
  dynamic_atoms_inside_function_declaration,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    function Component(color) {
      return stylex.props(css.display.flex, css.color(color));
    }
  "#
);

stylex_test!(
  atoms_inside_arrow_function,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    const Component = () => stylex.props(css.display.flex, css.color.blue);
  "#
);

stylex_test!(
  multiple_stylex_props_calls_with_atoms_in_same_function,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    function Component(isHeader) {
      const headerStyles = stylex.props(css.fontSize['24px'], css.fontWeight.bold);
      const bodyStyles = stylex.props(css.fontSize['16px'], css.color.black);
      return [headerStyles, bodyStyles];
    }
  "#
);

stylex_test!(
  inline_css_supports_commonjs_default_require,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const css = require('@stylexjs/atoms');
    stylex.props(css.display.flex);
  "#
);

stylex_test!(
  inline_css_supports_commonjs_destructured_require,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const { color } = require('@stylexjs/atoms');
    stylex.props(color.blue);
  "#
);

stylex_test!(
  uncalled_dynamic_create_style_member_bails_out,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    const styles = stylex.create({
      opacity: (o) => ({ opacity: o }),
    });
    stylex.props(css.display.flex, styles.opacity);
  "#
);

stylex_test!(
  dev_debug_classnames_for_atoms,
  |tr| stylex_transform_dev_debug(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    import css from '@stylexjs/atoms';
    stylex.props(css.display.flex);
  "#
);
