use indexmap::IndexMap;
use stylex_ast::ast::factories::{create_key_value_prop, create_object_expression};
use stylex_structures::{
  plugin_pass::PluginPass,
  stylex_env::{EnvEntry, JSFunction},
  stylex_options::StyleXOptionsParams,
};
use stylex_transform::{
  StyleXTransform,
  shared::{
    structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
    utils::ast::{
      convertors::{
        create_null_expr, create_number_expr, create_string_expr, expr_to_bool, expr_to_num,
        expr_to_str,
      },
      helpers::get_property_by_key,
    },
  },
};
use swc_core::ecma::{
  ast::Expr,
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

fn ts_syntax() -> Syntax {
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  })
}

fn env_config(env: IndexMap<String, EnvEntry>) -> StyleXOptionsParams {
  StyleXOptionsParams {
    env: Some(env),
    ..StyleXOptionsParams::default()
  }
}

fn expr_to_str_wrapper(expr: &Expr) -> Option<String> {
  expr_to_str(expr, &mut StateManager::default(), &FunctionMap::default())
}

fn expr_to_num_wrapper(expr: &Expr) -> Option<f64> {
  expr_to_num(
    expr,
    &mut EvaluationState::default(),
    &mut StateManager::default(),
    &FunctionMap::default(),
  )
  .ok()
}

fn expr_to_bool_wrapper(expr: &Expr) -> bool {
  expr_to_bool(expr, &mut StateManager::default(), &FunctionMap::default())
}

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "brandPrimary".to_string(),
      EnvEntry::Expr(create_string_expr("#123456")),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
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

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "brandPrimary".to_string(),
      EnvEntry::Expr(create_string_expr("#654321")),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
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

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "brandPrimary".to_string(),
      EnvEntry::Expr(create_string_expr("#123456")),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
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

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "colorMix".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let c1 = args
          .first()
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let c2 = args
          .get(1)
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let pct = args
          .get(2)
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        create_string_expr(&format!("color-mix(in srgb, {} {}%, {})", c1, pct, c2))
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
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

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "colorMix".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let c1 = args
          .first()
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let c2 = args
          .get(1)
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let pct = args
          .get(2)
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        create_string_expr(&format!("color-mix(in srgb, {} {}%, {})", c1, pct, c2))
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
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

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "shadow".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let color = args
          .first()
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let opacity = args.get(1).and_then(expr_to_num_wrapper).unwrap_or(0.0);
        create_string_expr(&format!(
          "0 4px 4px 2px color-mix(in srgb, {} {}%, transparent)",
          color,
          (opacity * 100.0) as i64
        ))
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
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

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "opacity".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let color = args
          .first()
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let pct = args.get(1).and_then(expr_to_num_wrapper).unwrap_or(0.0);
        create_string_expr(&format!(
          "color-mix(in srgb, {} {}%, transparent)",
          color,
          (pct * 100.0) as i64
        ))
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
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

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "makeTransition".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let prop = args
          .first()
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let duration = args
          .get(1)
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        create_object_expression(vec![
          create_key_value_prop("transitionProperty", create_string_expr(&prop)),
          create_key_value_prop("transitionDuration", create_string_expr(&duration)),
        ])
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_returns_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: stylex.env.makeTransition('opacity', '0.3s'),
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "makeTransition".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let prop = args
          .first()
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let duration = args
          .get(1)
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        create_object_expression(vec![
          create_key_value_prop("transitionProperty", create_string_expr(&prop)),
          create_key_value_prop("transitionDuration", create_string_expr(&duration)),
        ])
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_named_import_function_returns_object,
  r#"
    import { create, env } from '@stylexjs/stylex';
    export const styles = create({
      root: env.makeTransition('opacity', '0.3s'),
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "makeButton".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let color = args
          .first()
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let size = args
          .get(1)
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        create_object_expression(vec![
          create_key_value_prop("color", create_string_expr(&color)),
          create_key_value_prop("fontSize", create_string_expr(&format!("{}px", size))),
          create_key_value_prop("padding", create_string_expr("8px 16px")),
        ])
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_returns_object_multiple_props,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: stylex.env.makeButton('red', 16),
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "colorMix".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let c1 = args
          .first()
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let c2 = args
          .get(1)
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        create_string_expr(&format!("color-mix(in srgb, {}, {})", c1, c2))
      })),
    );
    env.insert(
      "makeSpacing".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let size = args
          .first()
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        create_object_expression(vec![
          create_key_value_prop("paddingTop", create_string_expr(&format!("{}px", size))),
          create_key_value_prop("paddingBottom", create_string_expr(&format!("{}px", size))),
        ])
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_mixed_string_and_object_functions,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: stylex.env.colorMix('red', 'blue'),
        ...stylex.env.makeSpacing(8),
      }
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "getLineHeight".to_string(),
      EnvEntry::Function(JSFunction::new(|_args: Vec<Expr>| create_number_expr(1.5))),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_returns_number,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        lineHeight: stylex.env.getLineHeight(),
      }
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "direction".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let is_rtl = args.first().map(expr_to_bool_wrapper).unwrap_or(false);
        create_string_expr(if is_rtl { "rtl" } else { "ltr" })
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_accepts_bool_param,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        direction: stylex.env.direction(true),
      }
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "getOptional".to_string(),
      EnvEntry::Function(JSFunction::new(|_args: Vec<Expr>| create_null_expr())),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_returns_null,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: stylex.env.getOptional(),
      }
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "makeTypography".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let scale = args.first().and_then(expr_to_num_wrapper).unwrap_or(1.0);
        let bold = args.get(1).map(expr_to_bool_wrapper).unwrap_or(false);
        create_object_expression(vec![
          create_key_value_prop(
            "fontSize",
            create_string_expr(&format!("{}px", (16.0 * scale) as i64)),
          ),
          create_key_value_prop(
            "lineHeight",
            create_number_expr(if scale > 1.5 { 1.8 } else { 1.5 }),
          ),
          create_key_value_prop(
            "fontWeight",
            create_string_expr(if bold { "bold" } else { "normal" }),
          ),
        ])
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_with_all_primitive_params,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: stylex.env.makeTypography(2, true),
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "makeGrid".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let columns = args.first().and_then(expr_to_num_wrapper).unwrap_or(1.0) as i64;
        create_object_expression(vec![
          create_key_value_prop("display", create_string_expr("grid")),
          create_key_value_prop(
            "gridTemplateColumns",
            create_string_expr(&format!("repeat({}, 1fr)", columns)),
          ),
          create_key_value_prop("gap", create_number_expr(8.0)),
        ])
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_returns_object_with_mixed_primitives,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: stylex.env.makeGrid(3),
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "colors".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let null = create_null_expr();
        let obj = args.first().unwrap_or(&null);
        let bg = get_property_by_key(obj, "backgroundColor")
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let color = get_property_by_key(obj, "color")
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        create_object_expression(vec![
          create_key_value_prop("backgroundColor", create_string_expr(&bg)),
          create_key_value_prop("color", create_string_expr(&color)),
        ])
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_accepts_object_param,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        ...stylex.env.colors({
          backgroundColor: 'blue',
          color: 'yellow',
        }),
      }
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "colors".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let null = create_null_expr();
        let obj = args.first().unwrap_or(&null);
        let bg = get_property_by_key(obj, "backgroundColor")
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let color = get_property_by_key(obj, "color")
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let opacity = get_property_by_key(obj, "opacity")
          .and_then(expr_to_num_wrapper)
          .unwrap_or(1.0);
        create_object_expression(vec![
          create_key_value_prop("backgroundColor", create_string_expr(&bg)),
          create_key_value_prop("color", create_string_expr(&color)),
          create_key_value_prop("opacity", create_number_expr(opacity)),
        ])
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_accepts_object_param_with_mixed_primitives,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        ...stylex.env.colors({
          backgroundColor: 'blue',
          color: 'yellow',
          opacity: 0.5,
        }),
      }
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "theme".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
        let null = create_null_expr();
        let config = args.first().unwrap_or(&null);
        let mode = args
          .get(1)
          .and_then(expr_to_str_wrapper)
          .unwrap_or_else(|| "light".to_string());
        let primary = get_property_by_key(config, "primary")
          .and_then(expr_to_str_wrapper)
          .unwrap_or_default();
        let size = get_property_by_key(config, "size")
          .and_then(expr_to_num_wrapper)
          .unwrap_or(16.0) as i64;
        create_object_expression(vec![
          create_key_value_prop(
            "color",
            create_string_expr(if mode == "dark" { "white" } else { &primary }),
          ),
          create_key_value_prop("fontSize", create_string_expr(&format!("{}px", size))),
        ])
      })),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_function_object_param_with_scalar_params,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: stylex.env.theme({ primary: 'red', size: 24 }, 'dark'),
    });
  "#
);

// --- Generic pass-through / branch selection tests ---

fn select_branch(args: &[Expr]) -> Expr {
  let key = args
    .get(1)
    .and_then(expr_to_str_wrapper)
    .unwrap_or_else(|| "default".to_string());
  args
    .first()
    .and_then(|e| get_property_by_key(e, &key))
    .cloned()
    .unwrap_or_else(create_null_expr)
}

fn pick_default(args: &[Expr]) -> Expr {
  args
    .first()
    .and_then(|e| get_property_by_key(e, "default"))
    .cloned()
    .unwrap_or_else(create_null_expr)
}

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "select".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| select_branch(&args))),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_generic_branch_selector_with_object_branches,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: stylex.env.select(
        { primary: { color: 'red', fontSize: '16px' }, secondary: { color: 'blue', fontSize: '14px' } },
        'primary'
      ),
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "select".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| select_branch(&args))),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_generic_branch_selector_returns_string,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: stylex.env.select({ primary: 'red', secondary: 'blue' }, 'secondary'),
      }
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "pick".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| select_branch(&args))),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_generic_selector_passes_through_array_value,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: stylex.env.pick({ a: ['red', 'darkred'], b: ['blue', 'navy'] }, 'a'),
      }
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "select".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| select_branch(&args))),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_generic_selector_passes_through_nested_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: stylex.env.select(
        {
          compact: { padding: '4px', gap: '2px' },
          spacious: { padding: '16px', gap: '8px' },
        },
        'spacious'
      ),
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "branch".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| pick_default(&args))),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_branch_selector_always_picks_default,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: stylex.env.branch({
        default: { color: 'black', backgroundColor: 'white' },
        alternate: { color: 'white', backgroundColor: 'black' },
      }),
    });
  "#
);

test!(
  ts_syntax(),
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "branch".to_string(),
      EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| pick_default(&args))),
    );
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut env_config(env)),
    )
  },
  stylex_env_branch_selector_with_nested_expression_values,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: stylex.env.branch({
          default: 'red',
          alternate: 'blue',
        }),
        padding: stylex.env.branch({
          default: '8px',
          alternate: '16px',
        }),
      }
    });
  "#
);
