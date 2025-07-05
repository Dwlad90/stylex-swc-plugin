use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleResolution, StyleXOptionsParams},
  },
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

// Tests with enableLogicalStylesPolyfill: true

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_inline_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginInline: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_inline_start_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginInlineStart: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_inline_end_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginInlineEnd: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_inline_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingInline: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_inline_start_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingInlineStart: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_inline_end_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingInlineEnd: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_color_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineColor: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_start_color_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineStartColor: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_style_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_width_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_block_color_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockColor: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_block_style_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_block_width_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_block_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetBlock: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_block_start_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetBlockStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_block_end_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetBlockEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_inline_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetInline: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_inline_start_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetInlineStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_inline_end_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetInlineEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_top_start_radius_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderTopStartRadius: 5 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_bottom_start_radius_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBottomStartRadius: 5 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_top_end_radius_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderTopEndRadius: 5 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_bottom_end_radius_with_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBottomEndRadius: 5 } });
    export const classnames = stylex(styles.x);
  "#
);

// Tests with enableLogicalStylesPolyfill: false

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_inline_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginInline: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_inline_start_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginInlineStart: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_inline_end_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { marginInlineEnd: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_inline_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingInline: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_inline_start_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingInlineStart: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_inline_end_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { paddingInlineEnd: '10px' } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_color_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineColor: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_start_color_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineStartColor: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_style_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_width_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderInlineWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_block_color_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockColor: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_block_style_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockStyle: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_block_width_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBlockWidth: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_block_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetBlock: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_block_start_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetBlockStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_block_end_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetBlockEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_inline_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetInline: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_inline_start_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetInlineStart: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_inline_end_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { insetInlineEnd: 0 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_top_start_radius_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderTopStartRadius: 5 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_bottom_start_radius_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBottomStartRadius: 5 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_top_end_radius_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderTopEndRadius: 5 } });
    export const classnames = stylex(styles.x);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..Default::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_bottom_end_radius_without_polyfill,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderBottomEndRadius: 5 } });
    export const classnames = stylex(styles.x);
  "#
);
