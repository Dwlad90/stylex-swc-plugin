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
/* CSS logical values transform */

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
  value_inline_end_for_clear_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { clear: 'inline-end' } });
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
  value_inline_start_for_clear_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { clear: 'inline-start' } });
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
  value_inline_end_for_float_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { float: 'inline-end' } });
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
  value_inline_start_for_float_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { float: 'inline-start' } });
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
  value_end_for_text_align_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { textAlign: 'end' } });
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
  value_start_for_text_align_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { textAlign: 'start' } });
    export const classnames = stylex(styles.x);
  "#
);

/* Non-standard values */

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
  non_standard_value_end_aka_inline_end_for_clear_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { clear: 'end' } });
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
  non_standard_value_start_aka_inline_start_for_clear_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { clear: 'start' } });
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
  non_standard_value_end_aka_inline_end_for_float_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { float: 'end' } });
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
  non_standard_value_start_aka_inline_start_for_float_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { float: 'start' } });
    export const classnames = stylex(styles.x);
  "#
);

/* Non-standard bidi transforms */

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
  legacy_value_e_resize_for_cursor_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { cursor: 'e-resize' } });
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
  legacy_value_w_resize_for_cursor_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { cursor: 'w-resize' } });
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
  legacy_value_ne_resize_for_cursor_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { cursor: 'ne-resize' } });
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
  legacy_value_nw_resize_for_cursor_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { cursor: 'nw-resize' } });
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
  legacy_value_se_resize_for_cursor_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { cursor: 'se-resize' } });
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
  legacy_value_sw_resize_for_cursor_property,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { cursor: 'sw-resize' } });
    export const classnames = stylex(styles.x);
  "#
);

/**
 * Legacy transforms
 * TODO(#33): Remove once support for multi-sided values is removed from shortforms.
 */

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
  legacy_value_of_animation_name_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { animationName: 'ignore' } });
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
  legacy_value_of_background_position_property_top_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { backgroundPosition: 'top end' } });
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
  legacy_value_of_background_position_property_top_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { backgroundPosition: 'top start' } });
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
  legacy_value_of_box_shadow_property_none,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: 'none' } });
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
  legacy_value_of_box_shadow_property_positive_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '1px 1px #000' } });
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
  legacy_value_of_box_shadow_property_negative_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '-1px -1px #000' } });
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
  legacy_value_of_box_shadow_property_inset_short_positive_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: 'inset 1px 1px #000' } });
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
  legacy_value_of_box_shadow_property_full_positive_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '1px 1px 1px 1px #000' } });
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
  legacy_value_of_box_shadow_property_inset_full_positive_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: 'inset 1px 1px 1px 1px #000' } });
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
  legacy_value_of_box_shadow_property_inset_full_positive_value_double,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '2px 2px 2px 2px red, inset 1px 1px 1px 1px #000' } });
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
  legacy_value_of_text_shadow_property_none,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: 'none' } });
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
  legacy_value_of_text_shadow_property_positive_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '1px 1px #000' } });
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
  legacy_value_of_text_shadow_property_negative_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '-1px -1px #000' } });
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
  legacy_value_of_text_shadow_property_short_positive_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '1px 1px 1px #000' } });
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
  legacy_value_of_text_shadow_property_full_positive_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '1px 1px 1px 1px #000' } });
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
  legacy_value_of_text_shadow_property_inset_full_positive_value,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: 'inset 1px 1px 1px 1px #000' } });
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
  legacy_value_of_text_shadow_property_inset_full_positive_value_double,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '2px 2px 2px 2px red, inset 1px 1px 1px 1px #000' } });
        export const classnames = stylex(styles.x);
      "#
);
