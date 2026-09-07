use super::*;
use stylex_ast::ast::convertors::normalize_expr;

pub(super) fn legacy_expand_shorthands(dynamic_styles: Vec<DynamicStyle>) -> Vec<DynamicStyle> {
  let expanded_keys_to_key_paths: Vec<DynamicStyle> = dynamic_styles
    .iter()
    .enumerate()
    .flat_map(|(i, dynamic_style)| {
      let obj_entry = (
        dynamic_style.key.clone(),
        PreRuleValue::string(create_shorthand_key(i)),
      );

      let options = StyleXStateOptions::default()
        .with_style_resolution(StyleResolution::LegacyExpandShorthands);

      flat_map_expanded_shorthands(obj_entry, &options)
    })
    .filter_map(|OrderPair(key, value)| {
      let value = value?;

      let index = value.as_css_text()[1..].parse::<usize>().ok()?;
      let that_dyn_style = dynamic_styles.get(index)?;

      let key = key.into_owned();
      Some(DynamicStyle {
        key: key.clone(),
        path: if that_dyn_style.path == that_dyn_style.key {
          key
        } else if that_dyn_style
          .path
          .contains(&(that_dyn_style.key.clone() + "_"))
        {
          that_dyn_style
            .path
            .replace(&(that_dyn_style.key.clone() + "_"), &(key + "_"))
        } else {
          that_dyn_style.path.replace(
            &("_".to_string() + that_dyn_style.key.as_str()),
            &("_".to_string() + key.as_str()),
          )
        },
        ..that_dyn_style.clone()
      })
    })
    .collect();

  expanded_keys_to_key_paths
}

pub(super) fn create_property_rule(variable_name: &str, is_pseudo_element: bool) -> String {
  let inherits = if is_pseudo_element { "true" } else { "false" };
  let mut rule = String::with_capacity(variable_name.len() + inherits.len() + 40);
  rule.push_str("@property ");
  rule.push_str(variable_name);
  rule.push_str(" { syntax: \"*\"; inherits: ");
  rule.push_str(inherits);
  rule.push_str(";}");
  rule
}

fn create_shorthand_key(index: usize) -> String {
  let digit_count = if index == 0 {
    1
  } else {
    index.ilog10() as usize + 1
  };
  let mut key = String::with_capacity(digit_count + 1);
  key.push('p');
  let _ = write!(key, "{index}");
  key
}

pub(super) fn is_safe_to_skip_null_check(expr: &Expr) -> bool {
  let expr = normalize_expr(expr);

  match expr {
    Expr::Tpl(_) => true,
    Expr::Lit(lit) => matches!(lit, Lit::Str(_) | Lit::Num(_) | Lit::Bool(_)),
    Expr::Bin(bin_expr) => match bin_expr.op {
      BinaryOp::Add
      | BinaryOp::Sub
      | BinaryOp::Mul
      | BinaryOp::Div
      | BinaryOp::Mod
      | BinaryOp::Exp => true,
      BinaryOp::NullishCoalescing | BinaryOp::LogicalOr => {
        is_safe_to_skip_null_check(&bin_expr.left) || is_safe_to_skip_null_check(&bin_expr.right)
      },
      BinaryOp::LogicalAnd => {
        is_safe_to_skip_null_check(&bin_expr.left) && is_safe_to_skip_null_check(&bin_expr.right)
      },
      _ => false,
    },
    Expr::Unary(unary_expr) => matches!(unary_expr.op, UnaryOp::Minus | UnaryOp::Plus),
    Expr::Cond(cond_expr) => {
      is_safe_to_skip_null_check(&cond_expr.cons) && is_safe_to_skip_null_check(&cond_expr.alt)
    },
    _ => false,
  }
}

pub(super) fn has_explicit_nullish_fallback(expr: &Expr) -> bool {
  let expr = normalize_expr(expr);
  match expr {
    Expr::Lit(Lit::Null(_)) => true,
    Expr::Ident(ident) if ident.sym == "undefined" => true,
    Expr::Unary(unary) if matches!(unary.op, UnaryOp::Void) => true,
    Expr::Cond(cond) => {
      has_explicit_nullish_fallback(&cond.cons) || has_explicit_nullish_fallback(&cond.alt)
    },
    Expr::Bin(bin) => match bin.op {
      BinaryOp::LogicalOr | BinaryOp::NullishCoalescing | BinaryOp::LogicalAnd => {
        has_explicit_nullish_fallback(&bin.left) || has_explicit_nullish_fallback(&bin.right)
      },
      _ => false,
    },
    _ => false,
  }
}

pub(super) fn extract_expr_from_rule(
  rule: &str,
  nullish_var_expressions: &FxHashMap<String, Expr>,
) -> Option<Expr> {
  for cap in VAR_EXTRACTION_REGEX.captures_iter(rule).flatten() {
    if let Some(var_match) = cap.get(1) {
      let var_name = var_match.as_str();
      if let Some(expr) = nullish_var_expressions.get(var_name) {
        return Some(expr.clone());
      }
    }
  }
  None
}

/// Hoist an expression to the module level as a `const` declaration.
///
/// The declaration is queued after the imports and the returned identifier
/// stands for the expression at the call site. `stem` is what the generated
/// name is built on.
fn hoist_to_module_level(
  stem: &'static str,
  ast_expression: Expr,
  state: &mut stylex_state::state_manager::StateManager,
) -> Expr {
  let hoisted_ident = state.next_hoisted_ident(stem);

  let var_decl = VarDecl {
    span: DUMMY_SP,
    kind: VarDeclKind::Const,
    declare: false,
    decls: vec![create_var_declarator(hoisted_ident.clone(), ast_expression)],
    ctxt: swc_core::common::SyntaxContext::empty(),
  };

  let module_item = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(var_decl))));
  state.queue_insertion(
    stylex_state::state_manager::InsertionSlot::AfterImports,
    module_item,
  );

  Expr::Ident(hoisted_ident)
}

/// Hoist a static fragment of a style value to a `_temp` constant.
pub(crate) fn hoist_expression(
  ast_expression: Expr,
  state: &mut stylex_state::state_manager::StateManager,
) -> Expr {
  hoist_to_module_level("temp", ast_expression, state)
}

/// Hoist the compiled styles object of a `stylex.create` call that is not a
/// module-level statement to a `_styles` constant.
///
/// # A dynamic entry can hold a reference the module level does not
///
/// A value the compiler cannot fold becomes an inline style that holds the
/// authored expression as it is written, and no rule says that expression may
/// name only the parameters of the entry. So an entry that reads a constant of
/// the enclosing scope carries that name to the module level, where it is not
/// declared:
///
/// ```js
/// export function render() {
///   const gap = compute();
///   const styles = stylex.create({ box: (v) => ({ width: v, margin: gap }) });
///   return stylex.props(styles.box(1));
/// }
/// ```
///
/// The module still loads, because the body of the entry runs only when it is
/// called. The call is what fails, which puts the fault a long way from its
/// cause. The snapshot
/// `a_nested_dynamic_entry_keeps_a_reference_to_the_scope_it_left` records the
/// output as it is.
pub(crate) fn hoist_styles_object(
  ast_expression: Expr,
  state: &mut stylex_state::state_manager::StateManager,
) -> Expr {
  hoist_to_module_level("styles", ast_expression, state)
}
