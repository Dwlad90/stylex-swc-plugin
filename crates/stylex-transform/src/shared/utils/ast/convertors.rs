// Re-export convertor functions from stylex-ast (canonical source)
#[allow(unused_imports)]
pub use stylex_ast::ast::convertors::{
  coerce_lit_to_number, coerce_tpl_to_string_lit, convert_atom_to_str_ref,
  convert_atom_to_string, convert_concat_to_tpl_expr, convert_lit_to_string,
  convert_simple_tpl_to_str_expr, convert_str_lit_to_atom, convert_str_lit_to_string,
  convert_string_to_prop_name, convert_wtf8_to_atom, create_big_int_expr, create_bool_expr,
  create_ident_expr, create_null_expr, create_number_expr, create_string_expr,
  expand_shorthand_prop, extract_str_lit_ref, extract_tpl_cooked_value,
};

use anyhow::anyhow;
// Import error handling macros from shared utilities
use stylex_macros::{
  as_expr_or_err, as_expr_or_opt_err, as_expr_or_panic, expr_to_str_or_err, stylex_panic,
  stylex_unimplemented, unwrap_or_panic,
};
use swc_core::ecma::ast::{
  BinExpr, BinaryOp, Expr, Ident, KeyValueProp, Lit, PropName, Tpl,
  UnaryExpr, UnaryOp,
};
use swc_core::ecma::utils::ExprExt;

use crate::shared::{
  constants::messages::{
    ILLEGAL_PROP_VALUE, VAR_DECL_INIT_REQUIRED, non_static_value,
  },
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

pub fn expr_to_num(
  expr_num: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<f64, anyhow::Error> {
  let result = match &expr_num {
    Expr::Ident(ident) => ident_to_number(ident, state, traversal_state, &FunctionMap::default()),
    Expr::Lit(lit) => return coerce_lit_to_number(lit),
    Expr::Unary(unary) => unari_to_num(unary, state, traversal_state, fns),
    Expr::Bin(lit) => {
      let mut state = Box::new(EvaluationState::new());

      match binary_expr_to_num(lit, &mut state, traversal_state, fns)
        .unwrap_or_else(|error| stylex_panic!("{}", error))
      {
        BinaryExprType::Number(number) => number,
        _ => stylex_panic!(
          "Binary expression is not a number: {:?}",
          expr_num.get_type(get_default_expr_ctx())
        ),
      }
    }
    _ => stylex_panic!(
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
        Expr::Lit(lit) => match convert_lit_to_string(lit) {
          Some(s) => s,
          None => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
        },
        Expr::Ident(ident) => ident_to_string(ident, state, functions),
        _ => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
      }
    }
    None => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
  }
}

#[inline]
pub fn ident_to_expr(ident: &Ident, state: &mut StateManager, functions: &FunctionMap) -> Expr {
  match get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce) {
    Some(var_decl) => get_expr_from_var_decl(&var_decl).clone(),
    _ => {
      stylex_panic!("{}", ILLEGAL_PROP_VALUE)
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
    Expr::Lit(lit) => convert_lit_to_string(lit),
    _ => stylex_panic!(
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
      Err(error) => stylex_panic!("{}", error),
    },
    UnaryOp::Plus => match expr_to_num(arg, state, traversal_state, fns) {
      Ok(result) => result,
      Err(error) => stylex_panic!("{}", error),
    },
    _ => stylex_panic!(
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

    stylex_panic!("Left expression is not a number")
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

    stylex_panic!("Right expression is not a number")
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

    stylex_panic!("Left expression is not a string")
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

    stylex_panic!("Right expression is not a string")
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
    _ => stylex_panic!(
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
      Expr::Lit(Lit::Str(_)) => match left_expr.as_lit() {
        Some(lit) => convert_lit_to_string(lit).unwrap_or_else(|| {
          stylex_panic!(
            "Left is not a string: {:?}",
            left_expr.get_type(get_default_expr_ctx())
          )
        }),
        None => stylex_panic!(
          "Left is not a string: {:?}",
          left_expr.get_type(get_default_expr_ctx())
        ),
      },
      _ => String::default(),
    };

    let right_str = match right_expr {
      Expr::Lit(Lit::Str(_)) => match right_expr.as_lit() {
        Some(lit) => convert_lit_to_string(lit).unwrap_or_else(|| {
          stylex_panic!(
            "Right is not a string: {:?}",
            left_expr.get_type(get_default_expr_ctx())
          )
        }),
        None => stylex_panic!(
          "Right is not a string: {:?}",
          left_expr.get_type(get_default_expr_ctx())
        ),
      },
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
            .unwrap_or_else(|error| stylex_panic!("{}", error))
          {
            BinaryExprType::Number(number) => number,
            _ => stylex_panic!(
              "Binary expression is not a number: {:?}",
              var_decl_expr.get_type(get_default_expr_ctx())
            ),
          }
        }
        Expr::Unary(unary_expr) => unari_to_num(unary_expr, state, traversal_state, fns),
        Expr::Lit(lit) => coerce_lit_to_number(lit).unwrap_or_else(|error| stylex_panic!("{}", error)),
        _ => stylex_panic!(
          "Varable {:?} is not a number",
          var_decl_expr.get_type(get_default_expr_ctx())
        ),
      }
    }
    None => {
      stylex_panic!("Variable {} is not declared", ident.sym)
    }
  }
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
        *expr = match var_decl.init.clone() {
          Some(init) => init,
          None => stylex_panic!("{}", VAR_DECL_INIT_REQUIRED),
        };
      }
    };
  }

  Expr::Tpl(tpl)
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
                Expr::Lit(lit) => match convert_lit_to_string(lit) {
                  Some(s) => s,
                  None => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
                },
                _ => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
              };

              tpl_str.push_str(value.as_str());
            }
            None => stylex_panic!("{}", non_static_value("expr_tpl_to_string")),
          }
        }
        Expr::Bin(bin) => tpl_str.push_str(
          transform_bin_expr_to_number(bin, state, traversal_state, fns)
            .to_string()
            .as_str(),
        ),
        Expr::Lit(lit) => tpl_str.push_str(&match convert_lit_to_string(lit) {
          Some(s) => s,
          None => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
        }),
        _ => stylex_unimplemented!(
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
    stylex_panic!(
      "Left expression is not a number: {:?}",
      bin.left.get_type(get_default_expr_ctx())
    )
  };

  let Some(right) = evaluate_cached(&bin.right, state, traversal_state, fns) else {
    stylex_panic!(
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
pub fn expr_to_bool(expr: &Expr, state: &mut StateManager, functions: &FunctionMap) -> bool {
  match expr {
    Expr::Lit(lit) => match lit {
      Lit::Bool(b) => b.value,
      Lit::Num(n) => n.value != 0.0,
      Lit::Str(s) => !s.value.is_empty(),
      Lit::Null(_) => false,
      _ => stylex_unimplemented!(
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
      _ => stylex_unimplemented!(
        "Conversion {:?} expression to boolean",
        expr.get_type(get_default_expr_ctx())
      ),
    },
    _ => {
      stylex_unimplemented!(
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
      convert_str_lit_to_string(strng)
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
      Some(lit) => match convert_lit_to_string(lit) {
        Some(s) => s,
        None => stylex_panic!("Computed property key must be a string or number literal."),
      },
      None => stylex_unimplemented!("Computed key is not a literal"),
    },
  };

  wrap_key_in_quotes(&key, should_wrap_in_quotes)
}
