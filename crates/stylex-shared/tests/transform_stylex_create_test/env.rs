use indexmap::IndexMap;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_env::{EnvFunction, EnvValue},
    stylex_options::StyleXOptionsParams,
  },
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

fn ts_syntax() -> Syntax {
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  })
}

fn env_config(env: IndexMap<String, EnvValue>) -> StyleXOptionsParams {
  StyleXOptionsParams {
    env: Some(env),
    ..StyleXOptionsParams::default()
  }
}

// Test 1: stylex.env resolves compile-time constants
test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "brandPrimary".to_string(),
      EnvValue::String("#123456".to_string()),
    );
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_resolves_compile_time_constants,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: stylex.env.brandPrimary,
      }
    });
  "#
);

// Test 2: stylex.env named import resolves compile-time constants
test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "brandPrimary".to_string(),
      EnvValue::String("#654321".to_string()),
    );
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_named_import_resolves_compile_time_constants,
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { env } from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: env.brandPrimary,
      }
    });
  "#
);

// Test 3: stylex.env destructured import resolves compile-time constants
test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "brandPrimary".to_string(),
      EnvValue::String("#123456".to_string()),
    );
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_destructured_import_resolves_compile_time_constants,
  r#"
    import {create, env} from '@stylexjs/stylex';
    export const styles = create({
      root: {
        color: env.brandPrimary,
      }
    });
  "#
);

// Test 4: stylex.env function call resolves at compile time
test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "colorMix".to_string(),
      EnvValue::Function(EnvFunction::new(|args: Vec<EnvValue>| {
        let c1 = args
          .first()
          .and_then(|v| v.to_css_string())
          .unwrap_or_default();
        let c2 = args
          .get(1)
          .and_then(|v| v.to_css_string())
          .unwrap_or_default();
        let pct = args
          .get(2)
          .and_then(|v| v.to_css_string())
          .unwrap_or_default();
        format!("color-mix(in srgb, {} {}%, {})", c1, pct, c2)
      })),
    );
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_call_resolves_at_compile_time,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: stylex.env.colorMix('red', 'blue', 50),
      }
    });
  "#
);

// Test 5: stylex.env named import function call resolves at compile time
test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "colorMix".to_string(),
      EnvValue::Function(EnvFunction::new(|args: Vec<EnvValue>| {
        let c1 = args
          .first()
          .and_then(|v| v.to_css_string())
          .unwrap_or_default();
        let c2 = args
          .get(1)
          .and_then(|v| v.to_css_string())
          .unwrap_or_default();
        let pct = args
          .get(2)
          .and_then(|v| v.to_css_string())
          .unwrap_or_default();
        format!("color-mix(in srgb, {} {}%, {})", c1, pct, c2)
      })),
    );
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_named_import_function_call,
  r#"
    import { create, env } from '@stylexjs/stylex';
    export const styles = create({
      root: {
        color: env.colorMix('red', 'blue', 50),
      }
    });
  "#
);

// Test 6: stylex.env function using template literals
test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "shadow".to_string(),
      EnvValue::Function(EnvFunction::new(|args: Vec<EnvValue>| {
        let color = args
          .first()
          .and_then(|v| v.to_css_string())
          .unwrap_or_default();
        let opacity = match args.get(1) {
          Some(EnvValue::Number(n)) => *n,
          _ => 0.0,
        };
        format!(
          "0 4px 4px 2px color-mix(in srgb, {} {}%, transparent)",
          color,
          (opacity * 100.0) as i64
        )
      })),
    );
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_using_template_literals,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        boxShadow: stylex.env.shadow('black', 0.35),
      }
    });
  "#
);

// Test 7: stylex.env function with multiple properties
test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "opacity".to_string(),
      EnvValue::Function(EnvFunction::new(|args: Vec<EnvValue>| {
        let color = args
          .first()
          .and_then(|v| v.to_css_string())
          .unwrap_or_default();
        let pct = match args.get(1) {
          Some(EnvValue::Number(n)) => *n,
          _ => 0.0,
        };
        format!(
          "color-mix(in srgb, {} {}%, transparent)",
          color,
          (pct * 100.0) as i64
        )
      })),
    );
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_with_multiple_properties,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: stylex.env.opacity('red', 0.5),
        backgroundColor: stylex.env.opacity('blue', 0.8),
      }
    });
  "#
);
