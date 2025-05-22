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
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_block_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockColor: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_block_start_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockStartColor: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_block_end_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockEndColor: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_inline_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineColor: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_inline_start_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineStartColor: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_inline_end_color,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineEndColor: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_block_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockStyle: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_block_start_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockStartStyle: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_block_end_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockEndStyle: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_inline_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineStyle: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_inline_start_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineStartStyle: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_inline_end_style,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineEndStyle: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_block_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockWidth: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_block_start_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockStartWidth: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_block_end_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderBlockEndWidth: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_inline_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineWidth: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_inline_start_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineStartWidth: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  border_inline_end_width,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { borderInlineEndWidth: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  inset_block,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetBlock: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  inset_block_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetBlockEnd: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  inset_block_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetBlockStart: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  inset_inline,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetInline: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  inset_inline_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetInlineEnd: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  inset_inline_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { insetInlineStart: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  margin_block,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginBlock: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  margin_block_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginBlockEnd: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  margin_block_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginBlockStart: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  margin_inline,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginInline: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  margin_inline_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginInlineEnd: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  margin_inline_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { marginInlineStart: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  padding_block,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingBlock: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  padding_block_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingBlockEnd: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  padding_block_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingBlockStart: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  padding_inline,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingInline: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  padding_inline_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingInlineEnd: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  padding_inline_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { paddingInlineStart: 0 } });
        const classnames = stylex(styles.x);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
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
