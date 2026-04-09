use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  border_block_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockColor: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_block_start_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockStartColor: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_block_end_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockEndColor: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_inline_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineColor: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_inline_start_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineStartColor: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_inline_end_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineEndColor: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_block_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockStyle: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_block_start_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockStartStyle: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_block_end_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockEndStyle: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_inline_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineStyle: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_inline_start_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineStartStyle: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_inline_end_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineEndStyle: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_block_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockWidth: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_block_start_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockStartWidth: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_block_end_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockEndWidth: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_inline_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineWidth: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_inline_start_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineStartWidth: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_inline_end_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineEndWidth: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_top_start_radius,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderTopStartRadius: 5 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_bottom_start_radius,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBottomStartRadius: 5 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_top_end_radius,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderTopEndRadius: 5 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  border_bottom_end_radius,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBottomEndRadius: 5 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  inset_block,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetBlock: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  inset_block_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetBlockEnd: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  inset_block_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetBlockStart: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  inset_inline,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetInline: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  inset_inline_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetInlineEnd: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  inset_inline_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetInlineStart: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  margin_block,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginBlock: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  margin_block_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginBlockEnd: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  margin_block_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginBlockStart: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  margin_inline,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginInline: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  margin_inline_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginInlineEnd: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  margin_inline_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginInlineStart: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  padding_block,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingBlock: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  padding_block_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingBlockEnd: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  padding_block_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingBlockStart: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  padding_inline,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingInline: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  padding_inline_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingInlineEnd: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  padding_inline_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingInlineStart: 0 } });
        export const classnames = stylex(styles.x);
    "#
);

// Corner shape tests
stylex_test!(
  corner_shape,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cornerShape: 'squircle' } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  corner_start_start_shape,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cornerStartStartShape: 'bevel' } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  corner_top_left_shape,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cornerTopLeftShape: 'notch' } });
        export const classnames = stylex(styles.x);
    "#
);

stylex_test!(
  legacy_short_form_property_value_flipping,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            four: {
            margin: '1 2 3 4',
            }
        });
        stylex(styles.four);
    "#
);
