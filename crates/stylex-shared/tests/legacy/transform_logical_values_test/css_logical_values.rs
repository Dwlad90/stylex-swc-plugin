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
  value_inline_end_for_clear_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { clear: 'inline-end' } });
        const classnames = stylex(styles.x);
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
  value_inline_start_for_clear_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { clear: 'inline-start' } });
        const classnames = stylex(styles.x);
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
  value_inline_end_for_float_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { float: 'inline-end' } });
        const classnames = stylex(styles.x);
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
  value_inline_start_for_float_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { float: 'inline-start' } });
        const classnames = stylex(styles.x);
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
  value_end_for_text_align_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textAlign: 'end' } });
        const classnames = stylex(styles.x);
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
  value_start_for_text_align_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textAlign: 'start' } });
        const classnames = stylex(styles.x);
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
  legacy_value_e_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'e-resize' } });
        const classnames = stylex(styles.x);
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
  legacy_value_w_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'w-resize' } });
        const classnames = stylex(styles.x);
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
  legacy_value_ne_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'ne-resize' } });
        const classnames = stylex(styles.x);
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
  legacy_value_nw_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'nw-resize' } });
        const classnames = stylex(styles.x);
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
  legacy_value_se_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'se-resize' } });
        const classnames = stylex(styles.x);
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
  legacy_value_sw_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'sw-resize' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_animation_name_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { animationName: 'ignore' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_background_position_property_top_end,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { backgroundPosition: 'top end' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_background_position_property_top_start,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { backgroundPosition: 'top start' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_box_shadow_property_none,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: 'none' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_box_shadow_property_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '1px 1px #000' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_box_shadow_property_negative_1px_negative_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '-1px -1px #000' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_box_shadow_property_inset_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: 'inset 1px 1px #000' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_box_shadow_property_1px_1px_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '1px 1px 1px 1px #000' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_box_shadow_property_inset_1px_1px_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: 'inset 1px 1px 1px 1px #000' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_box_shadow_property_complex,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '2px 2px 2px 2px red, inset 1px 1px 1px 1px #000' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_text_shadow_property_none,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: 'none' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_text_shadow_property_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '1px 1px #000' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_text_shadow_property_negative_1px_negative_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '-1px -1px #000' } });
        const classnames = stylex(styles.x);
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
  legacy_value_of_text_shadow_property_1px_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '1px 1px 1px #000' } });
        const classnames = stylex(styles.x);
    "#
);
