use super::*;

/// Normalizes different argument types into an ObjectLit for JavaScript object
/// methods.
pub(super) fn normalize_js_object_method_args(
  cached_arg: Option<EvaluateResultValue>,
) -> Option<ObjectLit> {
  cached_arg.and_then(|arg| match arg {
    EvaluateResultValue::Expr(expr) => expr.as_object().cloned().or_else(|| {
      if let Expr::Lit(Lit::Str(ref strng)) = expr {
        let keys = convert_atom_to_string(&strng.value)
          .chars()
          .enumerate()
          .map(|(i, c)| {
            create_ident_key_value_prop(&i.to_string(), create_string_expr(&c.to_string()))
          })
          .collect::<Vec<PropOrSpread>>();

        Some(create_object_lit(keys))
      } else {
        None
      }
    }),

    EvaluateResultValue::Vec(arr) => {
      let props = arr
        .iter()
        .enumerate()
        .filter_map(|(index, elem)| {
          if matches!(elem, EvaluateResultValue::Null) {
            return None;
          }

          let expr = match elem {
            EvaluateResultValue::Expr(expr) => expr.clone(),
            EvaluateResultValue::Vec(vec)
              if vec
                .iter()
                .any(|item| matches!(item, EvaluateResultValue::Null)) =>
            {
              return None;
            },
            EvaluateResultValue::Vec(vec) => normalize_js_object_method_nested_vector_arg(vec),
            #[cfg_attr(coverage_nightly, coverage(off))]
            _ => stylex_panic!("{}", ILLEGAL_PROP_ARRAY_VALUE),
          };

          Some(create_ident_key_value_prop(&index.to_string(), expr))
        })
        .collect();

      Some(create_object_lit(props))
    },

    _ => None,
  })
}

pub(super) fn normalize_js_object_method_array_arg(
  arr: &ArrayLit,
  traversal_state: &mut StateManager,
  functions: Rc<FunctionMap>,
) -> ObjectLit {
  let mut props = Vec::with_capacity(arr.elems.len());

  for (index, elem) in arr.elems.iter().enumerate() {
    let Some(elem) = elem else {
      continue;
    };

    let result = evaluate_with_functions(&elem.expr, traversal_state, Rc::clone(&functions));

    if !result.confident {
      continue;
    }

    let Some(value) = result.value else {
      continue;
    };

    let expr = match value {
      EvaluateResultValue::Expr(expr) => expr,
      EvaluateResultValue::Vec(items) => evaluate_result_vec_to_array_expr(&items),
      EvaluateResultValue::Null => continue,
      _ => continue,
    };

    props.push(create_ident_key_value_prop(&index.to_string(), expr));
  }

  create_object_lit(props)
}

/// Helper function to convert a nested vector of EvaluateResultValues to an
/// array expression
fn normalize_js_object_method_nested_vector_arg(vec: &[EvaluateResultValue]) -> Expr {
  let elems = vec
    .iter()
    .filter(|entry| !matches!(entry, EvaluateResultValue::Null))
    .map(|entry| {
      let expr = entry
        .as_vec()
        .map(|nested_vec| {
          let nested_elems = nested_vec
            .iter()
            .filter(|item| !matches!(item, EvaluateResultValue::Null))
            .map(|item| {
              let expr = match item.as_expr() {
                Some(e) => e,
                #[cfg_attr(coverage_nightly, coverage(off))]
                None => stylex_panic!("{}", ARGUMENT_NOT_EXPRESSION),
              };
              Some(create_expr_or_spread(expr.clone()))
            })
            .collect();

          create_array_expression(nested_elems)
        })
        .or_else(|| entry.as_expr().cloned())
        .unwrap_or_else(|| {
          #[cfg_attr(coverage_nightly, coverage(off))]
          {
            stylex_panic!("{}", ILLEGAL_PROP_ARRAY_VALUE)
          }
        });

      Some(create_expr_or_spread(expr))
    })
    .collect();

  create_array_expression(elems)
}

pub(super) fn evaluate_func_call_args(
  call: &CallExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Vec<EvaluateResultValue> {
  call
    .args
    .iter()
    .filter_map(|arg| evaluate_cached(&arg.expr, state, traversal_state, fns))
    .collect()
}

pub(super) fn args_to_numbers(
  args: &[EvaluateResultValue],
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Vec<f64> {
  let mut numbers = Vec::with_capacity(args.len());
  push_args_to_numbers(args, state, traversal_state, fns, &mut numbers);
  numbers
}

fn push_args_to_numbers(
  args: &[EvaluateResultValue],
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
  numbers: &mut Vec<f64>,
) {
  for arg in args {
    match arg {
      EvaluateResultValue::Expr(expr) => numbers.push(
        expr_to_num(expr, state, traversal_state, fns).unwrap_or_else(|error| {
          stylex_panic_with_context!(expr, traversal_state, error.to_string().as_str())
        }),
      ),
      EvaluateResultValue::Vec(vec) => {
        push_args_to_numbers(vec, state, traversal_state, fns, numbers);
      },
      EvaluateResultValue::Null => {},
      #[cfg_attr(coverage_nightly, coverage(off))]
      _ => stylex_unreachable!("Math.min/max requires a number"),
    }
  }
}

pub(super) fn get_binding<'a>(
  callee: &'a Expr,
  state: &'a StateManager,
) -> Option<&'a VarDeclarator> {
  match callee {
    Expr::Ident(ident) => get_var_decl_from(state, ident),
    _ => None,
  }
}

pub(super) fn is_valid_callee(callee: &Expr) -> bool {
  if let Expr::Ident(ident) = callee {
    VALID_CALLEES.contains(ident.sym.as_ref())
  } else {
    false
  }
}

pub(super) fn get_callee_name(callee: &Expr) -> &str {
  match callee {
    Expr::Ident(ident) => &ident.sym,
    #[cfg_attr(coverage_nightly, coverage(off))]
    _ => stylex_panic!("The function being called must be a static identifier."),
  }
}

pub(super) fn is_invalid_method(prop: &MemberProp) -> bool {
  match prop {
    MemberProp::Ident(ident_prop) => INVALID_METHODS.contains(&*ident_prop.sym),
    _ => false,
  }
}

/// Checks if a member property represents a mutating object method
/// (Object.assign, etc.)
pub(super) fn is_mutating_object_method(prop: &MemberProp) -> bool {
  if let MemberProp::Ident(ident_prop) = prop {
    MUTATING_OBJECT_METHODS.contains(&*ident_prop.sym)
  } else {
    false
  }
}

/// Checks if a member property represents a mutating array method (push, pop,
/// splice, etc.)
pub(super) fn is_mutating_array_method(prop: &MemberProp) -> bool {
  if let MemberProp::Ident(ident_prop) = prop {
    MUTATING_ARRAY_METHODS.contains(&*ident_prop.sym)
  } else {
    false
  }
}

/// Checks if an expression represents a mutation operation
/// Returns true if any of the following conditions are met:
/// - Assignment to a member expression (e.g., `a.x = 1` or `a[0] = 1`)
/// - Update expression on a member (e.g., `++a.x` or `a[0]++`)
/// - Delete operation on a member (e.g., `delete a.x`)
pub(super) fn is_mutation_expr(expr: &Expr) -> bool {
  match expr {
    // Check for assignment to member: a.x = 1 or a[0] = 1
    Expr::Assign(assign)
      if matches!(
        &assign.left,
        AssignTarget::Simple(SimpleAssignTarget::Member(member)) if member.obj.is_ident()
      ) =>
    {
      true
    },

    // Check for update on member: ++a.x or a[0]++
    Expr::Update(update) if matches!(&*update.arg, Expr::Member(member) if member.obj.is_ident()) => {
      true
    },

    // Check for delete on member: delete a.x
    Expr::Unary(unary)
      if unary.op == UnaryOp::Delete
        && matches!(&*unary.arg, Expr::Member(member) if member.obj.is_ident()) =>
    {
      true
    },

    _ => false,
  }
}

pub(super) fn get_method_name(prop: &MemberProp) -> &str {
  match prop {
    MemberProp::Ident(ident_prop) => &ident_prop.sym,
    #[cfg_attr(coverage_nightly, coverage(off))]
    _ => stylex_panic!("The method name in a call expression must be a static identifier."),
  }
}

pub(super) fn is_id_prop(prop: &MemberProp) -> Option<&Atom> {
  if let MemberProp::Computed(comp_prop) = prop
    && let Expr::Lit(Lit::Str(strng)) = comp_prop.expr.as_ref()
  {
    return Some(match strng.value.as_atom() {
      Some(a) => a,
      #[cfg_attr(coverage_nightly, coverage(off))]
      None => stylex_panic!("{}", INVALID_UTF8),
    });
  }

  None
}

pub(super) fn evaluate_theme_ref(
  file_name: &str,
  export_name: impl Into<String>,
  state: &StateManager,
) -> ThemeRef {
  ThemeRef::new(
    file_name,
    export_name,
    state.options.class_name_prefix.clone(),
  )
}
