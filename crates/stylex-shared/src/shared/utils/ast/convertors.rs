use swc_core::ecma::{
  ast::BigInt,
  utils::{quote_ident, quote_str, ExprExt},
};
use swc_core::{
  atoms::Atom,
  ecma::ast::{
    BinExpr, BinaryOp, Bool, Expr, Ident, KeyValueProp, Lit, Prop, PropName, Tpl, UnaryExpr,
    UnaryOp,
  },
};

use swc_ecma_parser::Context;

use crate::shared::{
  constants::messages::{ILLEGAL_PROP_VALUE, NON_STATIC_VALUE},
  enums::misc::VarDeclAction,
  structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
  utils::{
    common::{
      evaluate_bin_expr, get_expr_from_var_decl, get_string_val_from_lit, get_var_decl_by_ident,
    },
    js::evaluate::evaluate_cached,
  },
};

use super::factories::{
  ident_factory, lit_big_int_factory, lit_boolean_factory, lit_null_factory, lit_number_factory,
  lit_str_factory,
};

pub fn expr_to_num(expr_num: &Expr, traversal_state: &mut StateManager, fns: &FunctionMap) -> f64 {
  match &expr_num {
    Expr::Ident(ident) => ident_to_number(ident, traversal_state, &FunctionMap::default()),
    Expr::Lit(lit) => lit_to_num(lit),
    Expr::Unary(unary) => unari_to_num(unary, traversal_state, fns),
    Expr::Bin(lit) => {
      let mut state = Box::new(EvaluationState::new(traversal_state));

      match binary_expr_to_num(lit, &mut state, fns) {
        Some(result) => result,
        None => panic!(
          "Binary expression is not a number: {:?}",
          expr_num.get_type()
        ),
      }
    }
    _ => panic!("Expression in not a number: {:?}", expr_num.get_type()),
  }
}

fn ident_to_string(ident: &Ident, state: &mut StateManager, functions: &FunctionMap) -> String {
  let var_decl = get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce);

  match &var_decl {
    Some(var_decl) => {
      let var_decl_expr = get_expr_from_var_decl(var_decl);

      match &var_decl_expr {
        Expr::Lit(lit) => get_string_val_from_lit(lit).expect(ILLEGAL_PROP_VALUE),
        Expr::Ident(ident) => ident_to_string(ident, state, functions),
        _ => panic!("{}", ILLEGAL_PROP_VALUE),
      }
    }
    None => panic!("{}", ILLEGAL_PROP_VALUE),
  }
}

fn ident_to_expr(ident: &Ident, state: &mut StateManager, functions: &FunctionMap) -> Expr {
  let var_decl = get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce);

  match &var_decl {
    Some(var_decl) => {
      let var_decl_expr = get_expr_from_var_decl(var_decl);

      var_decl_expr.clone()
    }
    None => panic!("{}", ILLEGAL_PROP_VALUE),
  }
}

pub fn expr_to_str(
  expr_string: &Expr,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> String {
  match &expr_string {
    Expr::Ident(ident) => ident_to_string(ident, state, functions),
    Expr::Lit(lit) => get_string_val_from_lit(lit).expect("Value is not a string"),
    _ => panic!(
      "Expression in not a string, got {:?}",
      expr_string.get_type()
    ),
  }
}

pub fn unari_to_num(unary_expr: &UnaryExpr, state: &mut StateManager, fns: &FunctionMap) -> f64 {
  let arg = unary_expr.arg.as_ref();
  let op = unary_expr.op;

  match &op {
    UnaryOp::Minus => expr_to_num(arg, state, fns) * -1.0,
    UnaryOp::Plus => expr_to_num(arg, state, fns),
    _ => panic!(
      "Union operation '{:?}' is invalid",
      Expr::from(unary_expr.clone()).get_type()
    ),
  }
}

pub fn binary_expr_to_num(
  binary_expr: &BinExpr,
  state: &mut EvaluationState,
  fns: &FunctionMap,
) -> Option<f64> {
  let op = binary_expr.op;
  let Some(left) = evaluate_cached(&binary_expr.left, state, fns) else {
    if !state.confident {
      return None;
    }

    panic!("Left expression is not a number")
  };

  let Some(right) = evaluate_cached(&binary_expr.right, state, fns) else {
    if !state.confident {
      return None;
    }

    panic!("Right expression is not a number")
  };

  let result = match &op {
    BinaryOp::Add => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        + expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
    }
    BinaryOp::Sub => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        - expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
    }
    BinaryOp::Mul => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        * expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
    }
    BinaryOp::Div => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        / expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
    }
    BinaryOp::Mod => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        % expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
    }
    BinaryOp::Exp => expr_to_num(left.as_expr()?, &mut state.traversal_state, fns).powf(
      expr_to_num(right.as_expr()?, &mut state.traversal_state, fns),
    ),
    BinaryOp::RShift => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state, fns) as i32)
        >> expr_to_num(right.as_expr()?, &mut state.traversal_state, fns) as i32) as f64
    }
    BinaryOp::LShift => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state, fns) as i32)
        << expr_to_num(right.as_expr()?, &mut state.traversal_state, fns) as i32) as f64
    }
    BinaryOp::BitAnd => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state, fns) as i32)
        & expr_to_num(right.as_expr()?, &mut state.traversal_state, fns) as i32) as f64
    }
    BinaryOp::BitOr => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state, fns) as i32)
        | expr_to_num(right.as_expr()?, &mut state.traversal_state, fns) as i32) as f64
    }
    BinaryOp::BitXor => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state, fns) as i32)
        ^ expr_to_num(right.as_expr()?, &mut state.traversal_state, fns) as i32) as f64
    }
    BinaryOp::In => {
      if expr_to_num(right.as_expr()?, &mut state.traversal_state, fns) == 0.0 {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::InstanceOf => {
      if expr_to_num(right.as_expr()?, &mut state.traversal_state, fns) == 0.0 {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::EqEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        == expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::NotEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        != expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::EqEqEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        == expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::NotEqEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        != expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::Lt => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        < expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::LtEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        <= expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::Gt => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        > expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::GtEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state, fns)
        >= expr_to_num(right.as_expr()?, &mut state.traversal_state, fns)
      {
        1.0
      } else {
        0.0
      }
    }
    // #region Logical
    BinaryOp::LogicalOr => {
      let was_confident = state.confident;

      let result = evaluate_cached(&Box::new(left.as_expr()?), state, fns);

      let left = result.unwrap();
      let left = left.as_expr().unwrap();

      let left_confident = state.confident;

      state.confident = was_confident;

      let result = evaluate_cached(&Box::new(right.as_expr()?), state, fns);

      let right = result.unwrap();
      let right = right.as_expr().unwrap();
      let right_confident = state.confident;

      let left = expr_to_num(left, &mut state.traversal_state, fns);
      let right = expr_to_num(right, &mut state.traversal_state, fns);

      state.confident = left_confident && (left != 0.0 || right_confident);

      if !state.confident {
        return None;
      }

      if left != 0.0 {
        left
      } else {
        right
      }
    }
    BinaryOp::LogicalAnd => {
      let was_confident = state.confident;

      let result = evaluate_cached(&Box::new(left.as_expr()?), state, fns);

      let left = result.unwrap();
      let left = left.as_expr().unwrap();

      let left_confident = state.confident;

      state.confident = was_confident;

      let result = evaluate_cached(&Box::new(right.as_expr()?), state, fns);

      let right = result.unwrap();
      let right = right.as_expr().unwrap();
      let right_confident = state.confident;

      let left = expr_to_num(left, &mut state.traversal_state, fns);
      let right = expr_to_num(right, &mut state.traversal_state, fns);

      state.confident = left_confident && (left == 0.0 || right_confident);

      if !state.confident {
        return None;
      }

      if left != 0.0 {
        right
      } else {
        left
      }
    }
    BinaryOp::NullishCoalescing => {
      let was_confident = state.confident;

      let result = evaluate_cached(&Box::new(left.as_expr()?), state, fns);

      let left = result.unwrap();
      let left = left.as_expr().unwrap();

      let left_confident = state.confident;

      state.confident = was_confident;

      let result = evaluate_cached(&Box::new(right.as_expr()?), state, fns);

      let right = result.unwrap();
      let right = right.as_expr().unwrap();
      let right_confident = state.confident;

      let left = expr_to_num(left, &mut state.traversal_state, fns);
      let right = expr_to_num(right, &mut state.traversal_state, fns);

      state.confident = left_confident && !!(left == 0.0 || right_confident);

      if !state.confident {
        return None;
      }

      if left == 0.0 {
        right
      } else {
        left
      }
    }
    // #endregion Logical
    BinaryOp::ZeroFillRShift => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state, fns) as i32)
        >> expr_to_num(right.as_expr()?, &mut state.traversal_state, fns) as i32) as f64
    }
  };

  Some(result)
}

pub fn ident_to_number(ident: &Ident, traveral_state: &mut StateManager, fns: &FunctionMap) -> f64 {
  let var_decl = get_var_decl_by_ident(ident, traveral_state, fns, VarDeclAction::Reduce);

  match &var_decl {
    Some(var_decl) => {
      let var_decl_expr = get_expr_from_var_decl(var_decl);

      let mut state: EvaluationState = EvaluationState::new(traveral_state);

      match &var_decl_expr {
        Expr::Bin(bin_expr) => match binary_expr_to_num(bin_expr, &mut state, fns) {
          Some(result) => result,
          None => panic!("Binary expression is not a number"),
        },
        Expr::Unary(unary_expr) => unari_to_num(unary_expr, traveral_state, fns),
        Expr::Lit(lit) => lit_to_num(lit),
        _ => panic!("Varable {:?} is not a number", var_decl_expr.get_type()),
      }
    }
    None => panic!("Variable {} is not declared", ident.sym),
  }
}

pub fn lit_to_num(lit_num: &Lit) -> f64 {
  match &lit_num {
    Lit::Bool(Bool { value, .. }) => {
      if value == &true {
        1.0
      } else {
        0.0
      }
    }
    Lit::Num(num) => num.value,
    Lit::Str(strng) => {
      let Result::Ok(num) = strng.value.parse::<f64>() else {
        panic!("Value in not a number: {}", strng.value);
      };

      num
    }
    _ => {
      panic!(
        "Value in not a number: {:?}",
        Expr::from(lit_num.clone()).get_type()
      );
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
      match &var_decl {
        Some(var_decl) => {
          // Swap the placeholder expression in the template with the variable declaration's initializer
          std::mem::swap(
            expr,
            &mut var_decl
              .init
              .clone()
              .expect("Variable declaration has no initializer"),
          );
        }
        None => {}
      }
    };
  }

  Expr::Tpl(tpl)
}

pub fn expr_tpl_to_string(tpl: &Tpl, state: &mut StateManager, fns: &FunctionMap) -> String {
  let mut tpl_str: String = String::new();

  for (i, quasi) in tpl.quasis.iter().enumerate() {
    tpl_str.push_str(quasi.raw.as_ref());

    if i < tpl.exprs.len() {
      match &tpl.exprs[i].as_ref() {
        Expr::Ident(ident) => {
          let ident = get_var_decl_by_ident(ident, state, fns, VarDeclAction::Reduce);

          match ident {
            Some(var_decl) => {
              let var_decl_expr = get_expr_from_var_decl(&var_decl);

              let value = match &var_decl_expr {
                Expr::Lit(lit) => get_string_val_from_lit(lit).expect(ILLEGAL_PROP_VALUE),
                _ => panic!("{}", ILLEGAL_PROP_VALUE),
              };

              tpl_str.push_str(value.as_str());
            }
            None => panic!("{}", NON_STATIC_VALUE),
          }
        }
        Expr::Bin(bin) => tpl_str.push_str(
          transform_bin_expr_to_number(bin, state, fns)
            .to_string()
            .as_str(),
        ),
        Expr::Lit(lit) => {
          tpl_str.push_str(&get_string_val_from_lit(lit).expect(ILLEGAL_PROP_VALUE))
        }
        _ => unimplemented!("TPL expression: {:?}", tpl.exprs[i].get_type()),
      }
    }
  }

  tpl_str
}

pub fn transform_bin_expr_to_number(
  bin: &BinExpr,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> f64 {
  let mut state = Box::new(EvaluationState::new(traversal_state));
  let op = bin.op;
  let Some(left) = evaluate_cached(&bin.left, &mut state, fns) else {
    panic!("Left expression is not a number: {:?}", bin.left.get_type())
  };

  let Some(right) = evaluate_cached(&bin.right, &mut state, fns) else {
    panic!(
      "Left expression is not a number: {:?}",
      bin.right.get_type()
    )
  };
  let left = expr_to_num(left.as_expr().unwrap(), traversal_state, fns);
  let right = expr_to_num(right.as_expr().unwrap(), traversal_state, fns);

  evaluate_bin_expr(op, left, right)
}

pub fn number_to_expression(value: f64) -> Expr {
  Expr::from(lit_number_factory(value))
}

pub(crate) fn big_int_to_expression(value: BigInt) -> Expr {
  Expr::from(lit_big_int_factory(value))
}

pub fn string_to_expression(value: &str) -> Expr {
  Expr::Lit(lit_str_factory(value))
}

pub(crate) fn bool_to_expression(value: bool) -> Expr {
  Expr::Lit(lit_boolean_factory(value))
}

pub fn ident_to_expression(value: &str) -> Expr {
  Expr::Ident(ident_factory(value))
}

pub(crate) fn null_to_expression() -> Expr {
  Expr::Lit(lit_null_factory())
}

fn should_wrap_prop_name_key_with_quotes(key: &str) -> bool {
  Ident::verify_symbol(key).is_err() && {
    let ctx = Context::default();

    !ctx.is_reserved_word(&Atom::from(key))
  }
}
pub(crate) fn string_to_prop_name(value: &str) -> Option<PropName> {
  if should_wrap_prop_name_key_with_quotes(value) {
    Some(PropName::Str(quote_str!(value)))
  } else {
    Some(PropName::Ident(quote_ident!(value)))
  }
}

pub(crate) fn transform_shorthand_to_key_values(prop: &mut Box<Prop>) {
  if let Some(ident) = prop.as_shorthand() {
    *prop = Box::new(Prop::from(KeyValueProp {
      key: PropName::Ident(quote_ident!(ident.sym.as_ref())),
      value: Box::new(Expr::Ident(ident.clone())),
    }));
  }
}

pub(crate) fn expr_to_bool(expr: &Expr, state: &mut StateManager, functions: &FunctionMap) -> bool {
  match expr {
    Expr::Lit(lit) => match lit {
      Lit::Bool(b) => b.value,
      Lit::Num(n) => n.value != 0.0,
      Lit::Str(s) => !s.value.is_empty(),
      Lit::Null(_) => false,
      _ => unimplemented!("Conversion {:?} expression to boolean", expr.get_type()),
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
      _ => unimplemented!("Conversion {:?} expression to boolean", expr.get_type()),
    },
    _ => {
      unimplemented!("Conversion {:?} expression to boolean", expr.get_type())
    }
  }
}
