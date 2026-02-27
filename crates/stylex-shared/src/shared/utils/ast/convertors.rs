use anyhow::anyhow;
use swc_core::{
  atoms::Atom,
  ecma::{
    ast::{
      BinExpr, BinaryOp, Bool, CallExpr, Expr, Ident, KeyValueProp, Lit, Prop, PropName, Str, Tpl,
      TplElement, UnaryExpr, UnaryOp,
    },
    parser::Context,
  },
};
use swc_core::{
  atoms::Wtf8Atom,
  ecma::{
    ast::BigInt,
    utils::{ExprExt, quote_ident, quote_str},
  },
};

// Import error handling macros from shared utilities
use crate::{
  as_expr_or_err, as_expr_or_opt_err, as_expr_or_panic, expr_to_str_or_err, unwrap_or_panic,
};

use crate::shared::{
  constants::messages::{ILLEGAL_PROP_VALUE, non_static_value},
  enums::{
    data_structures::evaluate_result_value::EvaluateResultValue,
    misc::{BinaryExprType, VarDeclAction},
  },
  structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
  swc::get_default_expr_ctx,
  utils::{
    common::{
      evaluate_bin_expr, get_expr_from_var_decl, get_var_decl_by_ident, wrap_key_in_quotes,
    },
    js::evaluate::{deopt, evaluate_cached},
  },
};

use super::factories::{
  ident_factory, lit_big_int_factory, lit_boolean_factory, lit_null_factory, lit_number_factory,
  lit_str_factory,
};

pub fn expr_to_num(
  expr_num: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<f64, anyhow::Error> {
  let result = match &expr_num {
    Expr::Ident(ident) => ident_to_number(ident, state, traversal_state, &FunctionMap::default()),
    Expr::Lit(lit) => return lit_to_num(lit),
    Expr::Unary(unary) => unari_to_num(unary, state, traversal_state, fns),
    Expr::Bin(lit) => {
      let mut state = Box::new(EvaluationState::new());

      match binary_expr_to_num(lit, &mut state, traversal_state, fns)
        .unwrap_or_else(|error| panic!("{}", error))
      {
        BinaryExprType::Number(number) => number,
        _ => panic!(
          "Binary expression is not a number: {:?}",
          expr_num.get_type(get_default_expr_ctx())
        ),
      }
    }
    _ => panic!(
      "Expression in not a number: {:?}",
      expr_num.get_type(get_default_expr_ctx())
    ),
  };

  Result::Ok(result)
}

fn ident_to_string(ident: &Ident, state: &mut StateManager, functions: &FunctionMap) -> String {
  let var_decl = get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce);

  match &var_decl {
    Some(var_decl) => {
      let var_decl_expr = get_expr_from_var_decl(var_decl);

      match &var_decl_expr {
        Expr::Lit(lit) => lit_to_string(lit).expect(ILLEGAL_PROP_VALUE),
        Expr::Ident(ident) => ident_to_string(ident, state, functions),
        _ => panic!("{}", ILLEGAL_PROP_VALUE),
      }
    }
    None => panic!("{}", ILLEGAL_PROP_VALUE),
  }
}

#[inline]
pub fn ident_to_expr(ident: &Ident, state: &mut StateManager, functions: &FunctionMap) -> Expr {
  match get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce) {
    Some(var_decl) => get_expr_from_var_decl(&var_decl).clone(),
    _ => {
      panic!("{}", ILLEGAL_PROP_VALUE)
    }
  }
}

pub fn expr_to_str(
  expr_string: &Expr,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Option<String> {
  match &expr_string {
    Expr::Ident(ident) => Some(ident_to_string(ident, state, functions)),
    Expr::Lit(lit) => lit_to_string(lit),
    _ => panic!(
      "Expression in not a string, got {:?}",
      expr_string.get_type(get_default_expr_ctx())
    ),
  }
}

pub fn unari_to_num(
  unary_expr: &UnaryExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> f64 {
  let arg = unary_expr.arg.as_ref();
  let op = unary_expr.op;

  match &op {
    UnaryOp::Minus => match expr_to_num(arg, state, traversal_state, fns) {
      Ok(result) => -result,
      Err(error) => panic!("{}", error),
    },
    UnaryOp::Plus => match expr_to_num(arg, state, traversal_state, fns) {
      Ok(result) => result,
      Err(error) => panic!("{}", error),
    },
    _ => panic!(
      "Union operation '{:?}' is invalid",
      Expr::from(unary_expr.clone()).get_type(get_default_expr_ctx())
    ),
  }
}

pub fn binary_expr_to_num(
  binary_expr: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<BinaryExprType, anyhow::Error> {
  let op = binary_expr.op;
  let Some(left) = evaluate_cached(&binary_expr.left, state, traversal_state, fns) else {
    if !state.confident {
      return Result::Err(anyhow::anyhow!("Left expression is not a number"));
    }

    panic!("Left expression is not a number")
  };

  let left_expr = as_expr_or_err!(left, "Left argument not expression");
  let left_num = expr_to_num(left_expr, state, traversal_state, fns)?;

  let Some(right) = evaluate_cached(&binary_expr.right, state, traversal_state, fns) else {
    if !state.confident {
      if op == BinaryOp::LogicalOr && left_num != 0.0 {
        state.confident = true;

        return Result::Ok(BinaryExprType::Number(left_num));
      }

      return Result::Err(anyhow::anyhow!("Right expression is not a number"));
    }

    panic!("Right expression is not a number")
  };

  let right_expr = as_expr_or_err!(right, "Right argument not expression");
  let right_num = expr_to_num(right_expr, state, traversal_state, fns)?;

  let result = match &op {
    BinaryOp::Add => {
      if let Some(value) =
        evaluate_left_and_right_expression(state, traversal_state, fns, &left, &right)
      {
        return value;
      }

      left_num + right_num
    }
    BinaryOp::Sub => left_num - right_num,
    BinaryOp::Mul => left_num * right_num,
    BinaryOp::Div => left_num / right_num,
    BinaryOp::Mod => left_num % right_num,
    BinaryOp::Exp => left_num.powf(right_num),
    BinaryOp::RShift => ((left_num as i32) >> right_num as i32) as f64,
    BinaryOp::LShift => ((left_num as i32) << right_num as i32) as f64,
    BinaryOp::BitAnd => ((left_num as i32) & right_num as i32) as f64,
    BinaryOp::BitOr => ((left_num as i32) | right_num as i32) as f64,
    BinaryOp::BitXor => ((left_num as i32) ^ right_num as i32) as f64,
    BinaryOp::In => {
      if right_num == 0.0 {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::InstanceOf => {
      if right_num == 0.0 {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::EqEq => {
      if left_num == right_num {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::NotEq => {
      if left_num != right_num {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::EqEqEq => {
      if left_num == right_num {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::NotEqEq => {
      if left_num != right_num {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::Lt => {
      if left_num < right_num {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::LtEq => {
      if left_num <= right_num {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::Gt => {
      if left_num > right_num {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::GtEq => {
      if left_num >= right_num {
        1.0
      } else {
        0.0
      }
    }
    // #region Logical
    BinaryOp::LogicalOr => {
      if let Some(value) =
        evaluate_left_and_right_expression(state, traversal_state, fns, &left, &right)
      {
        return value;
      }

      if left_num != 0.0 { left_num } else { right_num }
    }
    BinaryOp::LogicalAnd => {
      if let Some(value) =
        evaluate_left_and_right_expression(state, traversal_state, fns, &left, &right)
      {
        return value;
      }

      if left_num != 0.0 { right_num } else { left_num }
    }
    BinaryOp::NullishCoalescing => {
      if let Some(value) =
        evaluate_left_and_right_expression(state, traversal_state, fns, &left, &right)
      {
        return value;
      }

      if left_num == 0.0 { right_num } else { left_num }
    }
    // #endregion Logical
    BinaryOp::ZeroFillRShift => ((left_num as i32) >> right_num as i32) as f64,
  };

  Result::Ok(BinaryExprType::Number(result))
}

pub fn binary_expr_to_string(
  binary_expr: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<BinaryExprType, anyhow::Error> {
  let op = binary_expr.op;
  let Some(left) = evaluate_cached(&binary_expr.left, state, traversal_state, fns) else {
    if !state.confident {
      return Result::Err(anyhow::anyhow!("Left expression is not a string"));
    }

    panic!("Left expression is not a string")
  };

  let left_expr = as_expr_or_err!(left, "Left argument not expression");
  let left_str = expr_to_str_or_err!(
    left_expr,
    traversal_state,
    fns,
    "Left expression is not a string"
  );

  let Some(right) = evaluate_cached(&binary_expr.right, state, traversal_state, fns) else {
    if !state.confident {
      if op == BinaryOp::LogicalOr {
        state.confident = true;

        return Result::Ok(BinaryExprType::String(left_str));
      }

      return Result::Err(anyhow::anyhow!("Right expression is not a string"));
    }

    panic!("Right expression is not a string")
  };

  let right_expr = as_expr_or_err!(right, "Right argument not expression");
  let right_str = expr_to_str_or_err!(
    right_expr,
    traversal_state,
    fns,
    "Right expression is not a string"
  );

  let result = match &op {
    BinaryOp::Add => {
      format!("{}{}", left_str, right_str)
    }
    _ => panic!(
      "For string expressions, only addition is supported, got {:?}",
      op
    ),
  };

  Result::Ok(BinaryExprType::String(result))
}

fn evaluate_left_and_right_expression(
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
  left: &EvaluateResultValue,
  right: &EvaluateResultValue,
) -> Option<Result<BinaryExprType, anyhow::Error>> {
  let left_expr = as_expr_or_opt_err!(left, "Left argument not expression");
  let right_expr = as_expr_or_opt_err!(right, "Right argument not expression");

  let mut state_for_left = EvaluationState {
    confident: true,
    deopt_path: None,
    ..state.clone()
  };
  let left_result = expr_to_num(left_expr, &mut state_for_left, traversal_state, fns);
  let left_confident = state.confident;

  let mut state_for_right = EvaluationState {
    confident: true,
    deopt_path: None,
    ..state.clone()
  };
  let right_result = expr_to_num(right_expr, &mut state_for_right, traversal_state, fns);
  let right_confident = state.confident;

  if left_result.is_err() || right_result.is_err() {
    let left_str = match left_expr {
      Expr::Lit(Lit::Str(_)) => lit_to_string(left_expr.as_lit().unwrap()).unwrap_or_else(|| {
        panic!(
          "Left is not a string: {:?}",
          left_expr.get_type(get_default_expr_ctx())
        )
      }),
      _ => String::default(),
    };

    let right_str = match right_expr {
      Expr::Lit(Lit::Str(_)) => lit_to_string(right_expr.as_lit().unwrap()).unwrap_or_else(|| {
        panic!(
          "Right is not a string: {:?}",
          left_expr.get_type(get_default_expr_ctx())
        )
      }),
      _ => String::default(),
    };

    if !left_str.is_empty() && !right_str.is_empty() {
      return Some(Result::Ok(BinaryExprType::String(format!(
        "{}{}",
        left_str, right_str
      ))));
    }
  }

  if !left_confident {
    let deopt_reason = state_for_left
      .deopt_reason
      .as_deref()
      .unwrap_or("unknown error")
      .to_string();

    deopt(left_expr, state, &deopt_reason);

    return Some(Result::Ok(BinaryExprType::Null));
  }

  if !right_confident {
    let deopt_reason = state_for_right
      .deopt_reason
      .as_deref()
      .unwrap_or("unknown error")
      .to_string();

    deopt(right_expr, state, &deopt_reason);

    return Some(Result::Ok(BinaryExprType::Null));
  }

  None
}

pub fn ident_to_number(
  ident: &Ident,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> f64 {
  let var_decl = get_var_decl_by_ident(ident, traversal_state, fns, VarDeclAction::Reduce);

  match &var_decl {
    Some(var_decl) => {
      let var_decl_expr = get_expr_from_var_decl(var_decl);

      match &var_decl_expr {
        Expr::Bin(bin_expr) => {
          match binary_expr_to_num(bin_expr, state, traversal_state, fns)
            .unwrap_or_else(|error| panic!("{}", error))
          {
            BinaryExprType::Number(number) => number,
            _ => panic!(
              "Binary expression is not a number: {:?}",
              var_decl_expr.get_type(get_default_expr_ctx())
            ),
          }
        }
        Expr::Unary(unary_expr) => unari_to_num(unary_expr, state, traversal_state, fns),
        Expr::Lit(lit) => lit_to_num(lit).unwrap_or_else(|error| panic!("{}", error)),
        _ => panic!(
          "Varable {:?} is not a number",
          var_decl_expr.get_type(get_default_expr_ctx())
        ),
      }
    }
    None => {
      panic!("Variable {} is not declared", ident.sym)
    }
  }
}

#[inline]
pub fn lit_to_num(lit_num: &Lit) -> Result<f64, anyhow::Error> {
  let result = match &lit_num {
    Lit::Bool(Bool { value, .. }) => {
      if value == &true {
        1.0
      } else {
        0.0
      }
    }
    Lit::Num(num) => num.value,
    Lit::Str(strng) => {
      let Result::Ok(num) = atom_to_string(&strng.value).parse::<f64>() else {
        return Err(anyhow!(
          "Value in not a number: {}",
          atom_to_string(&strng.value)
        ));
      };

      num
    }
    _ => {
      return Err(anyhow!(
        "Value in not a number: {:?}",
        Expr::from(lit_num.clone()).get_type(get_default_expr_ctx())
      ));
    }
  };

  Result::Ok(result)
}

pub fn handle_tpl_to_expression(
  tpl: &Tpl,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Expr {
  // Clone the template, so we can work on it
  let mut tpl = tpl.clone();

  // Loop through each expression in the template
  for expr in tpl.exprs.iter_mut() {
    // Check if the expression is an identifier
    if let Expr::Ident(ident) = expr.as_ref() {
      // Find the variable declaration for this identifier in the AST
      let var_decl = get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce);

      // If a variable declaration was found
      if let Some(var_decl) = &var_decl {
        // Swap the placeholder expression in the template with the variable declaration's initializer
        *expr = var_decl
          .init
          .clone()
          .expect("Variable declaration has no initializer");
      }
    };
  }

  Expr::Tpl(tpl)
}

/// Converts a simple template literal (without interpolations) to a regular string literal.
/// Returns `Some(Str)` if the template has no expressions and exactly one quasi element,
/// otherwise returns `None`.
///
/// # Arguments
/// * `tpl` - The template literal to convert
///
/// # Returns
/// * `Some(Str)` - If the template is a simple string (no interpolations)
/// * `None` - If the template has interpolations or is malformed
///
/// # Example
/// ```ignore
/// Template: `hello world` (no ${...} interpolations)
/// Returns: Str { value: "hello world", ... }
/// ```
#[inline]
pub fn simple_tpl_to_string(tpl: &Tpl) -> Option<Lit> {
  // Check if it's a simple template (no expressions)
  if tpl.exprs.is_empty() && tpl.quasis.len() == 1 {
    let quasi = &tpl.quasis[0];

    // Get the string value (prefer cooked if available, otherwise use raw)
    let value = quasi
      .cooked
      .as_ref()
      .expect("Failed to get cooked value")
      .as_str()
      .expect("Failed to get string value");

    return Some(lit_str_factory(value));
  }

  None
}

/// Converts a simple template literal expression to a regular string literal expression.
/// This is a convenience wrapper around `simple_tpl_to_string` that works with `Expr::Tpl`.
///
/// # Arguments
/// * `expr` - The expression to check and potentially convert
///
/// # Returns
/// * The original expression if it's not a simple template literal
/// * A string literal expression if the template is simple (no interpolations)
#[inline]
pub fn convert_simple_tpl_to_str_expr(expr: Expr) -> Expr {
  match expr {
    Expr::Tpl(ref tpl) => {
      if let Some(str_lit) = simple_tpl_to_string(tpl) {
        return Expr::Lit(str_lit);
      }
      expr
    }
    _ => expr,
  }
}

/// Converts a string `.concat()` call expression to a template literal expression.
///
/// # Arguments
/// * `expr` - The expression to check and potentially convert
///
/// # Returns
/// * The original expression if it's not a concat call
/// * A template literal expression if the expression is a valid concat call
///
/// # Example
/// ```javascript
/// Input: "hello".concat(world, "!")
/// Output: `hello${world}!`
/// ```
#[inline]
pub fn convert_concat_to_tpl_expr(expr: Expr) -> Expr {
  match expr {
    Expr::Call(ref call_expr) => {
      if let Some(tpl_expr) = concat_call_to_template_literal(call_expr) {
        return tpl_expr;
      }
      expr
    }
    _ => expr,
  }
}

/// Helper function that converts a CallExpr representing `.concat()` to a template literal.
///
/// # Arguments
/// * `call_expr` - The call expression to convert
///
/// # Returns
/// * `Some(Expr)` - Template literal expression if conversion is successful
/// * `None` - If the call expression is not a valid concat call
fn concat_call_to_template_literal(call_expr: &CallExpr) -> Option<Expr> {
  use swc_core::common::DUMMY_SP;

  // Check if this is a member expression with a "concat" property
  let member_expr = call_expr.callee.as_expr()?.as_member()?;
  let prop_ident = member_expr.prop.as_ident()?;

  if prop_ident.sym.as_ref() != "concat" {
    return None;
  }

  // Get the base string from the object being called
  let base_string = lit_str_to_str_ref(member_expr.obj.as_lit()?).map(|s| s.to_string())?;

  let mut exprs = Vec::new();
  let mut quasis = Vec::new();

  // Add the base string as the first quasi
  quasis.push(TplElement {
    span: DUMMY_SP,
    tail: false,
    cooked: Some(base_string.clone().into()),
    raw: base_string.into(),
  });

  // Process each argument
  for (i, arg) in call_expr.args.iter().enumerate() {
    // Skip spread arguments
    if arg.spread.is_some() {
      continue;
    }

    exprs.push(arg.expr.clone());

    let is_last = i == call_expr.args.len() - 1;
    quasis.push(TplElement {
      span: DUMMY_SP,
      tail: is_last,
      cooked: Some("".into()),
      raw: "".into(),
    });
  }

  let template_literal = Tpl {
    span: DUMMY_SP,
    exprs,
    quasis,
  };

  Some(Expr::Tpl(template_literal))
}

pub fn expr_tpl_to_string(
  tpl: &Tpl,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> String {
  let mut tpl_str: String = String::new();

  for (i, quasi) in tpl.quasis.iter().enumerate() {
    tpl_str.push_str(quasi.raw.as_ref());

    if i < tpl.exprs.len() {
      match &tpl.exprs[i].as_ref() {
        Expr::Ident(ident) => {
          let ident = get_var_decl_by_ident(ident, traversal_state, fns, VarDeclAction::Reduce);

          match ident {
            Some(var_decl) => {
              let var_decl_expr = get_expr_from_var_decl(&var_decl);

              let value = match &var_decl_expr {
                Expr::Lit(lit) => lit_to_string(lit).expect(ILLEGAL_PROP_VALUE),
                _ => panic!("{}", ILLEGAL_PROP_VALUE),
              };

              tpl_str.push_str(value.as_str());
            }
            None => panic!("{}", non_static_value("expr_tpl_to_string")),
          }
        }
        Expr::Bin(bin) => tpl_str.push_str(
          transform_bin_expr_to_number(bin, state, traversal_state, fns)
            .to_string()
            .as_str(),
        ),
        Expr::Lit(lit) => tpl_str.push_str(&lit_to_string(lit).expect(ILLEGAL_PROP_VALUE)),
        _ => unimplemented!(
          "TPL expression: {:?}",
          tpl.exprs[i].get_type(get_default_expr_ctx())
        ),
      }
    }
  }

  tpl_str
}

pub fn transform_bin_expr_to_number(
  bin: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> f64 {
  let op = bin.op;
  let Some(left) = evaluate_cached(&bin.left, state, traversal_state, fns) else {
    panic!(
      "Left expression is not a number: {:?}",
      bin.left.get_type(get_default_expr_ctx())
    )
  };

  let Some(right) = evaluate_cached(&bin.right, state, traversal_state, fns) else {
    panic!(
      "Left expression is not a number: {:?}",
      bin.right.get_type(get_default_expr_ctx())
    )
  };

  let left_expr = as_expr_or_panic!(left, "Left argument not expression");
  let right_expr = as_expr_or_panic!(right, "Right argument not expression");

  let left = unwrap_or_panic!(expr_to_num(left_expr, state, traversal_state, fns));
  let right = unwrap_or_panic!(expr_to_num(right_expr, state, traversal_state, fns));

  evaluate_bin_expr(op, left, right)
}

#[inline]
pub fn number_to_expression(value: f64) -> Expr {
  Expr::from(lit_number_factory(value))
}

#[inline]
pub(crate) fn big_int_to_expression(value: BigInt) -> Expr {
  Expr::from(lit_big_int_factory(value))
}

#[inline]
pub fn string_to_expression(value: &str) -> Expr {
  Expr::Lit(lit_str_factory(value))
}

#[inline]
pub(crate) fn bool_to_expression(value: bool) -> Expr {
  Expr::Lit(lit_boolean_factory(value))
}

#[inline]
pub fn ident_to_expression(value: &str) -> Expr {
  Expr::Ident(ident_factory(value))
}

#[inline]
pub(crate) fn null_to_expression() -> Expr {
  Expr::Lit(lit_null_factory())
}

#[inline]
fn should_wrap_prop_name_key_with_quotes(key: &str) -> bool {
  Ident::verify_symbol(key).is_err() && {
    let ctx = Context::default();

    !ctx.is_reserved_word(&key.into())
  }
}
#[inline]
pub(crate) fn string_to_prop_name(value: &str) -> Option<PropName> {
  if should_wrap_prop_name_key_with_quotes(value) {
    Some(PropName::Str(quote_str!(value)))
  } else {
    Some(PropName::Ident(quote_ident!(value)))
  }
}

pub(crate) fn transform_shorthand_to_key_values(prop: &mut Box<Prop>) {
  if let Some(ident) = prop.as_shorthand() {
    **prop = Prop::from(KeyValueProp {
      key: string_to_prop_name(ident.sym.as_ref()).expect("Failed to convert string to prop name"),
      value: Box::new(Expr::Ident(ident.clone())),
    });
  }
}

pub(crate) fn expr_to_bool(expr: &Expr, state: &mut StateManager, functions: &FunctionMap) -> bool {
  match expr {
    Expr::Lit(lit) => match lit {
      Lit::Bool(b) => b.value,
      Lit::Num(n) => n.value != 0.0,
      Lit::Str(s) => !s.value.is_empty(),
      Lit::Null(_) => false,
      _ => unimplemented!(
        "Conversion {:?} expression to boolean",
        expr.get_type(get_default_expr_ctx())
      ),
    },
    Expr::Ident(ident) => expr_to_bool(&ident_to_expr(ident, state, functions), state, functions),
    Expr::Array(_) => true,
    Expr::Object(_) => true,
    Expr::Fn(_) | Expr::Class(_) => true,
    Expr::Unary(unary) => match unary.op {
      UnaryOp::Void => false,
      UnaryOp::TypeOf => true,
      UnaryOp::Bang => !expr_to_bool(&unary.arg, state, functions),
      UnaryOp::Minus => !expr_to_bool(&unary.arg, state, functions),
      UnaryOp::Plus => !expr_to_bool(&unary.arg, state, functions),
      UnaryOp::Tilde => !expr_to_bool(&unary.arg, state, functions),
      _ => unimplemented!(
        "Conversion {:?} expression to boolean",
        expr.get_type(get_default_expr_ctx())
      ),
    },
    _ => {
      unimplemented!(
        "Conversion {:?} expression to boolean",
        expr.get_type(get_default_expr_ctx())
      )
    }
  }
}

#[inline]
pub(crate) fn key_value_to_str(key_value: &KeyValueProp) -> String {
  let key = &key_value.key;
  let mut should_wrap_in_quotes = false;

  let key = match key {
    PropName::Ident(ident) => ident.sym.to_string(),
    PropName::Str(strng) => {
      should_wrap_in_quotes = false;
      lit_str_to_string(strng)
    }
    PropName::Num(num) => {
      should_wrap_in_quotes = false;
      num.value.to_string()
    }
    PropName::BigInt(big_int) => {
      should_wrap_in_quotes = false;
      big_int.value.to_string()
    }
    PropName::Computed(computed) => match computed.expr.as_lit() {
      Some(lit) => lit_to_string(lit).expect("Computed key is not a valid literal"),
      None => unimplemented!("Computed key is not a literal"),
    },
  };

  wrap_key_in_quotes(&key, should_wrap_in_quotes)
}

/// Helper function to convert Wtf8Atom to String
/// Note: `.as_str()` returns an `Option<&str>` that only fails when the string contains invalid UTF-8
#[inline]
pub(crate) fn atom_to_string(atom: &Wtf8Atom) -> String {
  atom
    .as_str()
    .expect("String contains invalid UTF-8")
    .to_string()
}

pub(crate) fn wtf8_atom_to_atom(atom: &Wtf8Atom) -> Atom {
  atom
    .as_atom()
    .expect("String contains invalid UTF-8")
    .clone()
}

/// Helper function to safely get string from Lit::Str
#[inline]
pub(crate) fn lit_str_to_string(str_lit: &Str) -> String {
  str_lit
    .value
    .as_str()
    .expect("String contains invalid UTF-8")
    .to_string()
}

/// Helper function to safely get Atom from Lit::Str
pub(crate) fn lit_str_to_atom(str_lit: &Str) -> Atom {
  str_lit
    .value
    .as_atom()
    .expect("String contains invalid UTF-8")
    .clone()
}

/// Helper function to safely get cooked string from TplElement
#[inline]
pub(crate) fn tpl_element_cooked_to_string(elem: &TplElement) -> String {
  elem
    .cooked
    .as_ref()
    .expect("Cooked should be some")
    .as_str()
    .expect("String contains invalid UTF-8")
    .to_string()
}

/// Helper function to convert Atom to &str (reference, not owned String)
/// Useful when you need a reference instead of an owned String
#[inline]
pub(crate) fn atom_to_str(atom: &swc_core::atoms::Wtf8Atom) -> &str {
  atom.as_str().expect("Failed to convert Wtf8Atom to &str")
}

#[inline]
pub(crate) fn lit_to_string(value: &Lit) -> Option<String> {
  match value {
    Lit::Str(strng) => Some(lit_str_to_string(strng)),
    Lit::Num(num) => Some(format!("{}", num.value)),
    Lit::BigInt(big_int) => Some(format!("{}", big_int.value)),
    _ => None,
  }
}

/// Helper function to safely extract string from Lit::Str using Option pattern
#[inline]
pub(crate) fn lit_str_to_str_ref(lit: &Lit) -> Option<&str> {
  match lit {
    Lit::Str(s) => Some(atom_to_str(&s.value)),
    _ => None,
  }
}
