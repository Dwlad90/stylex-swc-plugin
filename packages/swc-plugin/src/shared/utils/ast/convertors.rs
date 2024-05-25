use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{
    BinExpr, BinaryOp, Bool, Expr, Ident, KeyValueProp, Lit, Number, Prop, PropName, Str, Tpl,
    UnaryExpr, UnaryOp,
  },
};

use crate::shared::{
  constants::messages::{ILLEGAL_PROP_VALUE, NON_STATIC_VALUE},
  enums::misc::VarDeclAction,
  regex::IDENT_PROP_REGEX,
  structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
  utils::{
    common::{
      evaluate_bin_expr, get_expr_from_var_decl, get_string_val_from_lit, get_var_decl_by_ident,
    },
    js::evaluate::evaluate_cached,
  },
};

pub fn expr_to_num(expr_num: &Expr, traversal_state: &mut StateManager) -> f64 {
  match &expr_num {
    Expr::Ident(ident) => ident_to_number(ident, traversal_state, &FunctionMap::default()),
    Expr::Lit(lit) => lit_to_num(lit),
    Expr::Unary(unary) => unari_to_num(unary, traversal_state),
    Expr::Bin(lit) => {
      let mut state = Box::new(EvaluationState::new(traversal_state));

      match binary_expr_to_num(lit, &mut state) {
        Some(result) => result,
        None => panic!("Binary expression is not a number"),
      }
    }
    _ => panic!("Expression in not a number {:?}", expr_num),
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

pub fn expr_to_str(
  expr_string: &Expr,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> String {
  match &expr_string {
    Expr::Ident(ident) => ident_to_string(ident, state, functions),
    Expr::Lit(lit) => get_string_val_from_lit(lit).expect("Value is not a string"),
    _ => panic!("Expression in not a string {:?}", expr_string),
  }
}

pub fn unari_to_num(unary_expr: &UnaryExpr, state: &mut StateManager) -> f64 {
  let arg = unary_expr.arg.as_ref();
  let op = unary_expr.op;

  match &op {
    UnaryOp::Minus => expr_to_num(arg, state) * -1.0,
    UnaryOp::Plus => expr_to_num(arg, state),
    _ => panic!("Union operation '{}' is invalid", op),
  }
}

pub fn binary_expr_to_num(binary_expr: &BinExpr, state: &mut EvaluationState) -> Option<f64> {
  let binary_expr = binary_expr.clone();

  let op = binary_expr.op;
  let Some(left) = evaluate_cached(&binary_expr.left, state) else {
    if !state.confident {
      return Option::None;
    }

    panic!("Left expression is not a number")
  };

  let Some(right) = evaluate_cached(&binary_expr.right, state) else {
    if !state.confident {
      return Option::None;
    }

    panic!("Right expression is not a number")
  };

  let result = match &op {
    BinaryOp::Add => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state)
        + expr_to_num(right.as_expr()?, &mut state.traversal_state)
    }
    BinaryOp::Sub => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state)
        - expr_to_num(right.as_expr()?, &mut state.traversal_state)
    }
    BinaryOp::Mul => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state)
        * expr_to_num(right.as_expr()?, &mut state.traversal_state)
    }
    BinaryOp::Div => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state)
        / expr_to_num(right.as_expr()?, &mut state.traversal_state)
    }
    BinaryOp::Mod => {
      expr_to_num(left.as_expr()?, &mut state.traversal_state)
        % expr_to_num(right.as_expr()?, &mut state.traversal_state)
    }
    BinaryOp::Exp => expr_to_num(left.as_expr()?, &mut state.traversal_state)
      .powf(expr_to_num(right.as_expr()?, &mut state.traversal_state)),
    BinaryOp::RShift => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
        >> expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32) as f64
    }
    BinaryOp::LShift => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
        << expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32) as f64
    }
    BinaryOp::BitAnd => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
        & expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32) as f64
    }
    BinaryOp::BitOr => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
        | expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32) as f64
    }
    BinaryOp::BitXor => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
        ^ expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32) as f64
    }
    BinaryOp::In => {
      if expr_to_num(right.as_expr()?, &mut state.traversal_state) == 0.0 {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::InstanceOf => {
      if expr_to_num(right.as_expr()?, &mut state.traversal_state) == 0.0 {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::EqEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state)
        == expr_to_num(right.as_expr()?, &mut state.traversal_state)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::NotEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state)
        != expr_to_num(right.as_expr()?, &mut state.traversal_state)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::EqEqEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state)
        == expr_to_num(right.as_expr()?, &mut state.traversal_state)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::NotEqEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state)
        != expr_to_num(right.as_expr()?, &mut state.traversal_state)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::Lt => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state)
        < expr_to_num(right.as_expr()?, &mut state.traversal_state)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::LtEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state)
        <= expr_to_num(right.as_expr()?, &mut state.traversal_state)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::Gt => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state)
        > expr_to_num(right.as_expr()?, &mut state.traversal_state)
      {
        1.0
      } else {
        0.0
      }
    }
    BinaryOp::GtEq => {
      if expr_to_num(left.as_expr()?, &mut state.traversal_state)
        >= expr_to_num(right.as_expr()?, &mut state.traversal_state)
      {
        1.0
      } else {
        0.0
      }
    }
    // #region Logical
    BinaryOp::LogicalOr => {
      let was_confident = state.confident;

      let result = evaluate_cached(&Box::new(left.as_expr()?.clone()), state);

      let left = result.unwrap();
      let left = left.as_expr().unwrap();

      let left_confident = state.confident;

      state.confident = was_confident;

      let result = evaluate_cached(&Box::new(right.as_expr()?.clone()), state);

      let right = result.unwrap();
      let right = right.as_expr().unwrap();
      let right_confident = state.confident;

      let left = expr_to_num(left, &mut state.traversal_state);
      let right = expr_to_num(right, &mut state.traversal_state);

      state.confident = left_confident && (left != 0.0 || right_confident);

      if !state.confident {
        return Option::None;
      }

      if left != 0.0 {
        left
      } else {
        right
      }
    }
    BinaryOp::LogicalAnd => {
      let was_confident = state.confident;

      let result = evaluate_cached(&Box::new(left.as_expr()?.clone()), state);

      let left = result.unwrap();
      let left = left.as_expr().unwrap();

      let left_confident = state.confident;

      state.confident = was_confident;

      let result = evaluate_cached(&Box::new(right.as_expr()?.clone()), state);

      let right = result.unwrap();
      let right = right.as_expr().unwrap();
      let right_confident = state.confident;

      let left = expr_to_num(left, &mut state.traversal_state);
      let right = expr_to_num(right, &mut state.traversal_state);

      state.confident = left_confident && (left == 0.0 || right_confident);

      if !state.confident {
        return Option::None;
      }

      if left != 0.0 {
        right
      } else {
        left
      }
    }
    BinaryOp::NullishCoalescing => {
      let was_confident = state.confident;

      let result = evaluate_cached(&Box::new(left.as_expr()?.clone()), state);

      let left = result.unwrap();
      let left = left.as_expr().unwrap();

      let left_confident = state.confident;

      state.confident = was_confident;

      let result = evaluate_cached(&Box::new(right.as_expr()?.clone()), state);

      let right = result.unwrap();
      let right = right.as_expr().unwrap();
      let right_confident = state.confident;

      let left = expr_to_num(left, &mut state.traversal_state);
      let right = expr_to_num(right, &mut state.traversal_state);

      state.confident = left_confident && !!(left == 0.0 || right_confident);

      if !state.confident {
        return Option::None;
      }

      if left == 0.0 {
        right
      } else {
        left
      }
    }
    // #endregion Logical
    BinaryOp::ZeroFillRShift => {
      ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
        >> expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32) as f64
    }
  };

  Option::Some(result)
}

pub fn ident_to_number(
  ident: &Ident,
  traveral_state: &mut StateManager,
  functions: &FunctionMap,
) -> f64 {
  let var_decl = get_var_decl_by_ident(ident, traveral_state, functions, VarDeclAction::Reduce);

  match &var_decl {
    Some(var_decl) => {
      let var_decl_expr = get_expr_from_var_decl(var_decl);

      let mut state: EvaluationState = EvaluationState::new(traveral_state);

      match &var_decl_expr {
        Expr::Bin(bin_expr) => match binary_expr_to_num(bin_expr, &mut state) {
          Some(result) => result,
          None => panic!("Binary expression is not a number"),
        },
        Expr::Unary(unary_expr) => unari_to_num(unary_expr, traveral_state),
        Expr::Lit(lit) => lit_to_num(lit),
        _ => panic!("Varable {:?} is not a number", var_decl_expr),
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
    Lit::Str(str) => {
      let Result::Ok(num) = str.value.parse::<f64>() else {
        panic!("Value in not a number");
      };

      num
    }
    _ => {
      panic!("Value in not a number");
    }
  }
}

pub fn handle_tpl_to_expression(
  tpl: &swc_core::ecma::ast::Tpl,
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

  Expr::Tpl(tpl.clone())
}

pub fn expr_tpl_to_string(tpl: &Tpl, state: &mut StateManager, functions: &FunctionMap) -> String {
  let mut tpl_str: String = String::new();

  for (i, quasi) in tpl.quasis.iter().enumerate() {
    tpl_str.push_str(quasi.raw.as_ref());

    if i < tpl.exprs.len() {
      match &tpl.exprs[i].as_ref() {
        Expr::Ident(ident) => {
          let ident = get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce);

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
          transform_bin_expr_to_number(bin, state)
            .to_string()
            .as_str(),
        ),
        Expr::Lit(lit) => {
          tpl_str.push_str(&get_string_val_from_lit(lit).expect(ILLEGAL_PROP_VALUE))
        }
        _ => panic!("Value not suppported"), // Handle other expression types as needed
      }
    }
  }

  tpl_str
}

pub fn transform_bin_expr_to_number(bin: &BinExpr, traversal_state: &mut StateManager) -> f64 {
  let mut state = Box::new(EvaluationState::new(traversal_state));
  let op = bin.op;
  let Some(left) = evaluate_cached(&bin.left, &mut state) else {
    panic!("Left expression is not a number")
  };

  let Some(right) = evaluate_cached(&bin.right, &mut state) else {
    panic!("Left expression is not a number")
  };
  let left = expr_to_num(left.as_expr().unwrap(), traversal_state);
  let right = expr_to_num(right.as_expr().unwrap(), traversal_state);

  evaluate_bin_expr(op, left, right)
}

pub(crate) fn number_to_expression(value: f64) -> Option<Expr> {
  Option::Some(Expr::Lit(Lit::Num(Number {
    span: DUMMY_SP,
    value,
    // value: trancate_f64(value),
    raw: Option::None,
  })))
}

pub(crate) fn string_to_expression(value: &str) -> Option<Expr> {
  Option::Some(Expr::Lit(Lit::Str(value.into())))
}

pub(crate) fn string_to_prop_name(value: &str) -> Option<PropName> {
  if IDENT_PROP_REGEX.is_match(value) && value.parse::<i64>().is_err() {
    Some(PropName::Ident(Ident::new(value.into(), DUMMY_SP)))
  } else {
    Some(PropName::Str(Str {
      span: DUMMY_SP,
      value: value.into(),
      raw: None,
    }))
  }
}

pub(crate) fn transform_shorthand_to_key_values(prop: &mut Box<Prop>) {
  if let Some(ident) = prop.as_shorthand() {
    *prop = Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(ident.clone()),
      value: Box::new(Expr::Ident(ident.clone())),
    }));
  }
}
