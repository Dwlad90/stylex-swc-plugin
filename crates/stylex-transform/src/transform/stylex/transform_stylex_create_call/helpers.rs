use super::*;

pub(super) fn legacy_expand_shorthands(dynamic_styles: Vec<DynamicStyle>) -> Vec<DynamicStyle> {
  let expanded_keys_to_key_paths: Vec<DynamicStyle> = dynamic_styles
    .iter()
    .enumerate()
    .flat_map(|(i, dynamic_style)| {
      let obj_entry = (
        dynamic_style.key.clone(),
        PreRuleValue::String(create_shorthand_key(i)),
      );

      let options = StyleXStateOptions::default()
        .with_style_resolution(StyleResolution::LegacyExpandShorthands);

      flat_map_expanded_shorthands(obj_entry, &options)
    })
    .filter_map(|OrderPair(key, value)| {
      let value = value?;

      let index = value[1..].parse::<usize>().ok()?;
      let that_dyn_style = dynamic_styles.get(index)?;

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

pub(super) fn is_safe_to_skip_null_check(expr: &mut Expr) -> bool {
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
        is_safe_to_skip_null_check(&mut bin_expr.left)
          || is_safe_to_skip_null_check(&mut bin_expr.right)
      },
      BinaryOp::LogicalAnd => {
        is_safe_to_skip_null_check(&mut bin_expr.left)
          && is_safe_to_skip_null_check(&mut bin_expr.right)
      },
      _ => false,
    },
    Expr::Unary(unary_expr) => matches!(unary_expr.op, UnaryOp::Minus | UnaryOp::Plus),
    Expr::Cond(cond_expr) => {
      is_safe_to_skip_null_check(&mut cond_expr.cons)
        && is_safe_to_skip_null_check(&mut cond_expr.alt)
    },
    _ => false,
  }
}

pub(super) fn has_explicit_nullish_fallback(expr: &mut Expr) -> bool {
  let expr = normalize_expr(expr);
  match expr {
    Expr::Lit(Lit::Null(_)) => true,
    Expr::Ident(ident) if ident.sym == "undefined" => true,
    Expr::Unary(unary) if matches!(unary.op, UnaryOp::Void) => true,
    Expr::Cond(cond) => {
      has_explicit_nullish_fallback(&mut cond.cons) || has_explicit_nullish_fallback(&mut cond.alt)
    },
    Expr::Bin(bin) => match bin.op {
      BinaryOp::LogicalOr | BinaryOp::NullishCoalescing | BinaryOp::LogicalAnd => {
        has_explicit_nullish_fallback(&mut bin.left)
          || has_explicit_nullish_fallback(&mut bin.right)
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

/// Hoists an expression to the program level by creating a const variable
/// declaration. This is the Rust equivalent of the JavaScript `hoistExpression`
/// function.
///
/// # Arguments
/// * `ast_expression` - The expression to hoist
/// * `state` - The state manager to add the hoisted declaration to
///
/// # Returns
/// An identifier referencing the hoisted variable
pub(super) fn hoist_expression(
  ast_expression: Expr,
  state: &mut crate::shared::structures::state_manager::StateManager,
) -> Expr {
  let uid_generator = UidGenerator::new("temp", CounterMode::ThreadLocal);
  let hoisted_ident = uid_generator.generate_ident();

  let var_decl = VarDecl {
    span: DUMMY_SP,
    kind: VarDeclKind::Const,
    declare: false,
    decls: vec![create_var_declarator(hoisted_ident.clone(), ast_expression)],
    ctxt: swc_core::common::SyntaxContext::empty(),
  };

  let module_item = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(var_decl))));
  state.hoisted_module_items.push(module_item);

  Expr::Ident(hoisted_ident)
}

pub(crate) fn path_replace_hoisted(
  ast_expression: Expr,
  is_program_level: bool,
  state: &mut crate::shared::structures::state_manager::StateManager,
) -> Expr {
  if is_program_level {
    return ast_expression;
  }

  let uid_generator = UidGenerator::new("styles", CounterMode::ThreadLocal);
  let name_ident = uid_generator.generate_ident();

  let var_decl = VarDecl {
    span: DUMMY_SP,
    kind: VarDeclKind::Const,
    declare: false,
    decls: vec![create_var_declarator(name_ident.clone(), ast_expression)],
    ctxt: swc_core::common::SyntaxContext::empty(),
  };

  let module_item = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(var_decl))));
  state.hoisted_module_items.push(module_item);

  Expr::Ident(name_ident)
}
