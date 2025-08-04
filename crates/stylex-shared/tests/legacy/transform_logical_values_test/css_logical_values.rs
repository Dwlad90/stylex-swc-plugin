use stylex_shared::{
  StyleXTransform,
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
};
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
        export const classnames = stylex(styles.x);
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
        export const classnames = stylex(styles.x);
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
        export const classnames = stylex(styles.x);
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
        export const classnames = stylex(styles.x);
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
        export const classnames = stylex(styles.x);
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
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_e_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'e-resize' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_w_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'w-resize' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_ne_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'ne-resize' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_nw_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'nw-resize' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_se_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'se-resize' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_sw_resize_for_cursor_property,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { cursor: 'sw-resize' } });
        export const classnames = stylex(styles.x);
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
        export const classnames = stylex(styles.x);
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
        export const classnames = stylex(styles.x);
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
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_box_shadow_property_none,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: 'none' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_box_shadow_property_v2,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: {
        animationName: stylex.keyframes({
          '0%': {
            boxShadow: '1px 2px 3px 4px red'
          },
          '100%': {
            boxShadow: '10px 20px 30px 40px green'
          }
        })
      }
    });
    export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_box_shadow_property_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '1px 1px #000' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_box_shadow_property_negative_1px_negative_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '-1px -1px #000' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_box_shadow_property_inset_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: 'inset 1px 1px #000' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_box_shadow_property_1px_1px_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '1px 1px 1px 1px #000' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_box_shadow_property_inset_1px_1px_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: 'inset 1px 1px 1px 1px #000' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_box_shadow_property_complex,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { boxShadow: '2px 2px 2px 2px red, inset 1px 1px 1px 1px #000' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_text_shadow_property_none,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: 'none' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_text_shadow_property_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '1px 1px #000' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_text_shadow_property_negative_1px_negative_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '-1px -1px #000' } });
        export const classnames = stylex(styles.x);
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
    Some(&mut StyleXOptionsParams {
      enable_legacy_value_flipping: Some(true),
      ..StyleXOptionsParams::default()
    }),
  ),
  legacy_value_of_text_shadow_property_1px_1px_1px_hash000,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({ x: { textShadow: '1px 1px 1px #000' } });
        export const classnames = stylex(styles.x);
    "#
);
