use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::RuntimeInjection,
    plugin_pass::PluginPass,
    stylex_options::{StyleResolution, StyleXOptionsParams},
  },
  ModuleTransformVisitor,
};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_block_color,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockColor: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

/* Border colors */
test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_block_start_color,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockStartColor: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_block_end_color,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineEndColor: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

/* Border styles */

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_block_style,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_block_start_style,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockStartStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_block_end_style,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockEndStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_inline_style,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_inline_start_style,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineStartStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_inline_end_style,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineEndStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

/* Border widths */

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_block_width,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_block_start_width,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockStartWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_block_end_width,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockEndWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_inline_width,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_inline_start_width,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineStartWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  border_inline_end_width,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineEndWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

/* Position offsets */

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  inset_block_start,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetBlockStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  inset_block,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetBlock: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  inset_block_end,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetBlockEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  inset_inline,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetInline: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  inset_inline_start,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetInlineStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  inset_inline_end,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetInlineEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

/* Margins */

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  margin_block,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginBlock: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  margin_block_start,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginBlockStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  margin_inline,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginInline: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  margin_inline_end,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginInlineEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  margin_inline_start,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginInlineStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

/* Padding */

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  padding_block,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingBlock: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  padding_block_end,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingBlockEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  padding_block_start,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingBlockStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  padding_inline,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingInline: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  padding_inline_end,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingInlineEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  padding_inline_start,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingInlineStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

/* Non-standard properties */

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  non_standard_end_aka_inset_inline_end,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { end: 5 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  non_standard_margin_end_aka_margin_inline_end,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  non_standard_margin_horizontal_aka_margin_inline,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginHorizontal: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  non_standard_margin_vertical_aka_margin_block,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginVertical: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  non_standard_padding_end_aka_padding_inline_end,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  non_standard_padding_horizontal_aka_padding_inline,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingHorizontal: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  non_standard_padding_start_aka_padding_inline_start,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  non_standard_padding_vertical_aka_padding_block,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingVertical: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
  non_standard_start_aka_inset_inline_start,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { start: 5 } });
    export const classnames = stylex(styles.x);
  "#
);

/* Legacy transforms */

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::None,
    )
  },
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
