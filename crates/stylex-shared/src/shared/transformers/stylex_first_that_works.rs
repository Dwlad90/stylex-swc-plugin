use log::warn;
use swc_core::ecma::ast::{Expr, ExprOrSpread};

use crate::shared::{
  regex::IS_CSS_VAR,
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::ast::{
    convertors::{expr_to_str, string_to_expression},
    factories::array_expression_factory,
  },
};

fn is_var(arg: &Expr, state: &mut StateManager, functions: &FunctionMap) -> bool {
  let str_arg = expr_to_str(arg, state, functions).expect("Expression is not a string");

  IS_CSS_VAR.is_match(&str_arg).unwrap_or_else(|err| {
    warn!(
      "Error matching IS_CSS_VAR for '{}': {}. Skipping pattern match.",
      str_arg, err
    );

    false
  })
}

pub(crate) fn stylex_first_that_works(
  args: Vec<Expr>,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Expr {
  let first_var = args.iter().position(|arg| is_var(arg, state, functions));

  match first_var {
    None => {
      let elems = args
        .into_iter()
        .rev()
        .map(|arg| {
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(arg),
          })
        })
        .collect();

      array_expression_factory(elems)
    }
    Some(first_var) => {
      let priorities = args[..first_var].iter().rev().collect::<Vec<_>>();
      let rest = &args[first_var..];
      let first_non_var = rest.iter().position(|arg| !is_var(arg, state, functions));
      let var_parts = if let Some(first_non_var) = first_non_var {
        rest[..=first_non_var].iter().rev().collect::<Vec<_>>()
      } else {
        rest.iter().rev().collect::<Vec<_>>()
      };

      let vars = var_parts
        .into_iter()
        .map(|arg| {
          if is_var(arg, state, functions) {
            let str_arg = expr_to_str(arg, state, functions).expect("Argument is not a string");
            let cleared_str_arg = &str_arg[4..str_arg.len() - 1];
            string_to_expression(cleared_str_arg)
          } else {
            arg.clone()
          }
        })
        .collect::<Vec<_>>();

      let return_value = {
        let mut so_far = String::new();
        for var_name in vars.iter() {
          let var_name_str =
            expr_to_str(var_name, state, functions).expect("Expression is not a string");

          so_far = if !so_far.is_empty() {
            format!("var({}, {})", var_name_str, so_far)
          } else if var_name_str.starts_with("--") {
            format!("var({})", var_name_str)
          } else {
            var_name_str
          };
        }

        let mut result = vec![string_to_expression(&so_far)];
        result.extend(priorities.iter().map(|&expr| expr.clone()));
        result
      };

      if return_value.len() == 1 {
        return return_value[0].clone();
      }

      let return_value = return_value
        .into_iter()
        .map(|expr| {
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(expr),
          })
        })
        .collect();

      array_expression_factory(return_value)
    }
  }
}
