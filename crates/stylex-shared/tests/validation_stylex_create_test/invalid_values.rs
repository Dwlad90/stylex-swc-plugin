use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_display_important() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { display: "block !important" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_background_position_top_left() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { backgroundPosition: "top left" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_border_color_red_blue() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { borderColor: "red blue" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_border_radius_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { borderRadius: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_border_style_solid_dashed() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { borderStyle: "solid dashed" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_border_width_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { borderWidth: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_inset_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { inset: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_inset_block_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { insetBlock: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_inset_inline_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { insetInline: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_flex_1_1_0() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { flex: "1 1 0" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_grid_1_1_0() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { grid: "1 1 0" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_margin_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { margin: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_margin_block_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { marginBlock: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_margin_inline_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { marginInline: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_outline_1px_solid_red() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { outline: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_overflow_hidden_visible() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { overflow: "hidden visible" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_padding_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { padding: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_padding_block_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { paddingBlock: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_padding_inline_1px_2px() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { paddingInline: "1px 2px" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_transition_property_all() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "all" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_transition_property_bottom() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "bottom" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_transition_property_end() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "end" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_transition_property_height() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "height" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn invalid_value_transition_property_width() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "width" } });
          "#,
    r#""#,
  )
}
