// Import error handling macros from shared utilities
use stylex_macros::{
  as_expr_or_panic, stylex_bail, stylex_panic, stylex_unimplemented, unwrap_or_panic,
};
use swc_core::ecma::ast::{BinExpr, Expr, Ident, Lit, Tpl, UnaryExpr, UnaryOp};

use crate::shared::structures::state::EvaluationState;
use crate::shared::utils::js::evaluate::{binary_expr_to_num_or_str, evaluate_cached};
use stylex_ast::ast::convertors::{
  convert_lit_to_number, convert_lit_to_string, get_expr_from_var_decl,
};
use stylex_constants::constants::messages::{
  ILLEGAL_PROP_VALUE, VAR_DECL_INIT_REQUIRED, non_static_value,
};
use stylex_enums::misc::BinaryExprType;
use stylex_js::{coercions, operators::evaluate_bin_expr};
use stylex_state::{
  common::get_var_decl_by_ident, functions::FunctionMap, state_manager::StateManager,
};
use stylex_structures::raw_value::TRawValue;
use stylex_utils::swc::get_expr_node_kind;

pub fn expr_to_num(
  expr_num: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<f64, anyhow::Error> {
  let result = match &expr_num {
    // `undefined`, `NaN` and `Infinity` are ordinary identifiers rather than
    // literals, and the evaluator now hands two of them back as values in their
    // own right — `void x`, and a key an object does not carry. Asked of the
    // binding table instead, they resolve to no declaration and fail the build,
    // where JavaScript answers `NaN`, `NaN` and `Infinity`. The shared coercion
    // names exactly those three and refuses every other identifier, so an
    // ordinary binding still reads its declaration below.
    Expr::Ident(ident) => match coercions::to_js_number(expr_num) {
      Some(number) => number,
      None => ident_to_number(ident, state, traversal_state, &FunctionMap::default()),
    },
    Expr::Lit(lit) => return convert_lit_to_number(lit),
    Expr::Unary(unary) => convert_unary_to_num(unary, state, traversal_state, fns),
    Expr::Bin(lit) => {
      let mut state = Box::new(EvaluationState::new());

      match binary_expr_to_num_or_str(lit, &mut state, traversal_state, fns)? {
        BinaryExprType::Number(number) => number,
        _ => stylex_bail!(
          "Binary expression is not a number: {}",
          get_expr_node_kind(expr_num)
        ),
      }
    },
    // An expression with no numeric reading is reported rather than fatal:
    // `-{}` and `Math.abs([])` are ordinary JavaScript, and this returns a
    // `Result` precisely so the evaluator can refuse to fold them instead of
    // aborting the build from inside an evaluation allowed to fail.
    _ => stylex_bail!(
      "Expression is not a number: {}",
      get_expr_node_kind(expr_num)
    ),
  };

  Result::Ok(result)
}

/// The string the binding behind an identifier spells, or `None` when it spells
/// no string.
///
/// `None` covers both an identifier bound to something that is not a string and
/// one with no binding to read at all -- `undefined`, which is an ordinary
/// global identifier rather than a literal. Neither is decided here, for the
/// reason given on [`convert_expr_to_str`].
fn ident_to_string(
  ident: &Ident,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Option<String> {
  let var_decl = get_var_decl_by_ident(ident, state, functions);

  match &var_decl {
    Some(var_decl) => match get_expr_from_var_decl(var_decl) {
      Expr::Lit(lit) => convert_lit_to_string(lit),
      Expr::Ident(ident) => ident_to_string(ident, state, functions),
      _ => None,
    },
    None => None,
  }
}

#[inline]
pub fn convert_ident_to_expr(
  ident: &Ident,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Expr {
  match get_var_decl_by_ident(ident, state, functions) {
    Some(var_decl) => get_expr_from_var_decl(&var_decl).clone(),
    _ => {
      stylex_panic!("{}", ILLEGAL_PROP_VALUE)
    },
  }
}

/// The string an expression spells, or `None` when it spells no string —
/// an object, an array, `null`, a boolean.
///
/// `None` is the answer for every non-string rather than a panic, because what
/// a non-string means is the caller's question, not the converter's: a step of
/// an animation declares nothing, a namespace name is a hard error. Answering
/// it here would force one of those onto the other.
pub fn convert_expr_to_str(
  expr_string: &Expr,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Option<String> {
  match &expr_string {
    Expr::Ident(ident) => ident_to_string(ident, state, functions),
    Expr::Lit(lit) => convert_lit_to_string(lit),
    _ => None,
  }
}

pub fn convert_unary_to_num(
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
      "Union operation '{}' is invalid",
      get_expr_node_kind(&Expr::from(unary_expr.clone()))
    ),
  }
}

pub fn ident_to_number(
  ident: &Ident,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> f64 {
  let var_decl = get_var_decl_by_ident(ident, traversal_state, fns);

  match &var_decl {
    Some(var_decl) => {
      let var_decl_expr = get_expr_from_var_decl(var_decl);

      match &var_decl_expr {
        Expr::Bin(bin_expr) => {
          match binary_expr_to_num_or_str(bin_expr, state, traversal_state, fns)
            .unwrap_or_else(|error| stylex_panic!("{}", error))
          {
            BinaryExprType::Number(number) => number,
            _ => stylex_panic!(
              "Binary expression is not a number: {}",
              get_expr_node_kind(var_decl_expr)
            ),
          }
        },
        Expr::Unary(unary_expr) => convert_unary_to_num(unary_expr, state, traversal_state, fns),
        Expr::Lit(lit) => {
          convert_lit_to_number(lit).unwrap_or_else(|error| stylex_panic!("{}", error))
        },
        _ => stylex_panic!(
          "Varable {} is not a number",
          get_expr_node_kind(var_decl_expr)
        ),
      }
    },
    None => {
      stylex_panic!("Variable {} is not declared", ident.sym)
    },
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
      let var_decl = get_var_decl_by_ident(ident, state, functions);

      // If a variable declaration was found
      if let Some(var_decl) = &var_decl {
        // Swap the placeholder expression in the template with the variable
        // declaration's initializer
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
  let quasi_len = tpl.quasis.iter().map(|quasi| quasi.raw.len()).sum();
  let mut tpl_str = String::with_capacity(quasi_len);

  for (i, quasi) in tpl.quasis.iter().enumerate() {
    tpl_str.push_str(quasi.raw.as_ref());

    if i < tpl.exprs.len() {
      match &tpl.exprs[i].as_ref() {
        Expr::Ident(ident) => {
          let ident = get_var_decl_by_ident(ident, traversal_state, fns);

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
            },
            None => stylex_panic!("{}", non_static_value("expr_tpl_to_string")),
          }
        },
        Expr::Bin(bin) => tpl_str.push_str(
          transform_bin_expr_to_number(bin, state, traversal_state, fns)
            .to_string()
            .as_str(),
        ),
        Expr::Lit(lit) => tpl_str.push_str(&match convert_lit_to_string(lit) {
          Some(s) => s,
          None => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
        }),
        _ => stylex_unimplemented!("TPL expression: {}", get_expr_node_kind(&tpl.exprs[i])),
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
    {
      stylex_panic!(
        "Left expression is not a number: {}",
        get_expr_node_kind(&bin.left)
      )
    }
  };

  let Some(right) = evaluate_cached(&bin.right, state, traversal_state, fns) else {
    {
      stylex_panic!(
        "Left expression is not a number: {}",
        get_expr_node_kind(&bin.right)
      )
    }
  };

  let left_expr = as_expr_or_panic!(left, "Left argument not expression");
  let right_expr = as_expr_or_panic!(right, "Right argument not expression");

  let left = unwrap_or_panic!(expr_to_num(left_expr, state, traversal_state, fns));
  let right = unwrap_or_panic!(expr_to_num(right_expr, state, traversal_state, fns));

  evaluate_bin_expr(op, left, right)
}

/// Reads a literal as an authored style value, keeping the JS type distinction
/// that decides whether a unit suffix is appended later.
///
/// A numeric literal stays a number; everything else that has a string form
/// becomes one. `None` means the literal is not a usable style value.
pub(crate) fn convert_lit_to_raw_value(lit: &Lit) -> Option<TRawValue> {
  match lit {
    Lit::Num(num) => Some(TRawValue::Number(num.value)),
    _ => convert_lit_to_string(lit).map(TRawValue::String),
  }
}
