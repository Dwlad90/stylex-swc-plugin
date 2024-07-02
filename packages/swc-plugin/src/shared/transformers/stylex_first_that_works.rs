use swc_core::ecma::ast::{Expr, ExprOrSpread};

use crate::shared::utils::ast::{
  convertors::{expr_to_str, string_to_expression},
  factories::array_expression_factory,
};

fn is_var(arg: &Expr) -> bool {
  let str_arg = expr_to_str(arg, &mut Default::default(), &Default::default());

  let re = regex::Regex::new(r"^var\(--[a-zA-Z0-9-_]+\)$").unwrap();
  re.is_match(&str_arg)
}

pub(crate) fn stylex_first_that_works(args: Vec<Expr>) -> Expr {
  let first_var = args.iter().position(is_var);
  dbg!(&args);
  dbg!(&first_var);

  match first_var {
    None => {
      let mut elems = Vec::with_capacity(args.len());

      for arg in args.into_iter().rev() {
        elems.push(Some(ExprOrSpread {
          spread: None,
          expr: Box::new(arg),
        }));
      }

      array_expression_factory(elems)
    }
    Some(first_var) => {
      let priorities = args[..first_var]
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<Expr>>();
      let rest = args[first_var..].to_vec();
      let first_non_var = rest.iter().position(|arg| !is_var(arg));
      let var_parts = if let Some(first_non_var) = first_non_var {
        rest[..=first_non_var]
          .iter()
          .rev()
          .cloned()
          .collect::<Vec<Expr>>()
      } else {
        rest.iter().rev().cloned().collect::<Vec<Expr>>()
      };

      let vars = var_parts
        .into_iter()
        .map(|arg| {
          if is_var(&arg) {
            let str_arg = expr_to_str(&arg, &mut Default::default(), &Default::default());

            let cleared_str_arg = str_arg[4..str_arg.len() - 1].to_string();

            string_to_expression(&cleared_str_arg)
          } else {
            arg
          }
        })
        .collect::<Vec<Expr>>();

      let return_value = {
        let mut so_far = String::new();
        for var_name in vars.iter() {
          let var_name_str = expr_to_str(var_name, &mut Default::default(), &Default::default());
          so_far = if !so_far.is_empty() {
            format!("var({}, {})", var_name_str, so_far)
          } else if var_name_str.starts_with("--") {
            format!("var({})", var_name_str)
          } else {
            var_name_str
          };
        }

        let mut result = vec![string_to_expression(&so_far)];

        result.extend_from_slice(&priorities);

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
        .collect::<Vec<Option<ExprOrSpread>>>();

      array_expression_factory(return_value)
    }
  }
}
