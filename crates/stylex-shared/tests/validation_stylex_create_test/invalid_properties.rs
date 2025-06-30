use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: animation")]
fn invalid_property_animation() {
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
            const styles = stylex.create({ x: { animation: "anim 1s" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: background")]
fn invalid_property_background() {
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
            const styles = stylex.create({ x: { background: "red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: border")]
fn invalid_property_border() {
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
            const styles = stylex.create({ x: { border: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderBlock")]
fn invalid_property_border_block() {
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
            const styles = stylex.create({ x: { borderBlock: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderBlockEnd")]
fn invalid_property_border_block_end() {
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
            const styles = stylex.create({ x: { borderBlockEnd: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderBlockStart")]
fn invalid_property_border_block_start() {
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
            const styles = stylex.create({ x: { borderBlockStart: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderBottom")]
fn invalid_property_border_bottom() {
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
            const styles = stylex.create({ x: { borderBottom: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderImage")]
fn invalid_property_border_image() {
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
            const styles = stylex.create({ x: { borderImage: "url(./img.jpg) 30 space" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderInline")]
fn invalid_property_border_inline() {
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
            const styles = stylex.create({ x: { borderInline: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderInlineEnd")]
fn invalid_property_border_inline_end() {
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
            const styles = stylex.create({ x: { borderInlineEnd: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderInlineStart")]
fn invalid_property_border_inline_start() {
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
            const styles = stylex.create({ x: { borderInlineStart: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderLeft")]
fn invalid_property_border_left() {
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
            const styles = stylex.create({ x: { borderLeft: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderRight")]
fn invalid_property_border_right() {
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
            const styles = stylex.create({ x: { borderRight: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: borderTop")]
fn invalid_property_border_top() {
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
            const styles = stylex.create({ x: { borderTop: "1px solid red" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: flexFlow")]
fn invalid_property_flex_flow() {
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
            const styles = stylex.create({ x: { flexFlow: "row wrap" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: font")]
fn invalid_property_font() {
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
            const styles = stylex.create({ x: { font: "16px/16 Arial" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: listStyle")]
fn invalid_property_list_style() {
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
            const styles = stylex.create({ x: { listStyle: "square inside" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: textDecoration")]
fn invalid_property_text_decoration() {
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
            const styles = stylex.create({ x: { textDecoration: "1px solid underline" } });
          "#,
    r#""#,
  )
}

#[ignore]
#[test]
#[should_panic(expected = "Unknown CSS property: transition")]
fn invalid_property_transition() {
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
            const styles = stylex.create({ x: { transition: "opacity 1s" } });
          "#,
    r#""#,
  )
}
