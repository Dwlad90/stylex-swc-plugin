use std::rc::Rc;

use stylex_ast::ast::convertors::{
  convert_lit_to_string, key_value_name, normalize_expr, normalize_expr_mut,
};

use swc_core::common::util::take::Take;
use swc_core::ecma::{
  ast::{
    AwaitExpr, BinExpr, BinaryOp, CallExpr, CondExpr, Expr, ExprOrSpread, Ident, JSXAttrOrSpread,
    JSXAttrValue, Lit, MetaPropExpr, MetaPropKind, ObjectLit, Prop, PropName, PropOrSpread, Super,
    ThisExpr, YieldExpr,
  },
  visit::{Visit, VisitMut, VisitMutWith, VisitWith},
};

use crate::shared::{
  enums::data_structures::fn_result::FnResult,
  transformers::stylex_default_marker,
  utils::core::{
    make_string_expression::make_string_expression,
    member_expression::MemberTransform,
    parse_nullable_style::{ResolvedArg, StyleObject, parse_nullable_style},
  },
};
use stylex_ast::ast::factories::{create_jsx_attr, create_jsx_attr_or_spread};
use stylex_constants::constants::{api_names::STYLEX_DEFAULT_MARKER, common::COMPILED_KEY};
use stylex_enums::style_vars_to_keep::NonNullProps;
use stylex_state::{
  functions::{FunctionConfigType, FunctionMap},
  state_manager::{ImportKind, StateManager},
};

/// Merges the arguments of a `stylex.props`-family call into one value.
///
/// The caller supplies `hoist_expression`, which lifts an expression into a
/// module-scoped `const` and returns a reference to it.
pub(crate) fn stylex_merge(
  call: &mut CallExpr,
  transform: fn(&[ResolvedArg]) -> FnResult,
  hoist_expression: fn(Expr, &mut StateManager) -> Ident,
  state: &mut StateManager,
) -> Option<Expr> {
  let mut bail_out = false;
  let mut conditional = 0;
  let mut current_index = -1;
  let mut bail_out_index = None;

  let mut evaluate_path_fn_config = FunctionMap {
    disable_imports: true,
    ..FunctionMap::default()
  };

  let marker_values = stylex_default_marker::shared_default_marker_values(state);

  if let Some(set) = state.get_stylex_api_import(ImportKind::DefaultMarker)
    && !set.is_empty()
  {
    for name in set {
      evaluate_path_fn_config.identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::IndexMap(Rc::clone(&marker_values))),
      );
    }
  }

  for name in state.stylex_imports() {
    // `or_default` answers the entry itself, new or already there, so the
    // second look-up that stood here and its refusal are both gone.
    evaluate_path_fn_config
      .member_expressions
      .entry(name.clone())
      .or_default()
      .insert(
        STYLEX_DEFAULT_MARKER.into(),
        Box::new(FunctionConfigType::IndexMap(Rc::clone(&marker_values))),
      );
  }

  state.apply_stylex_env(&mut evaluate_path_fn_config);

  // Shared rather than owned from here on. Each argument of the call asks the
  // evaluator, and the evaluator owns the map it hands to every callback, so a
  // map given by reference was copied once per argument and once per member
  // read inside it.
  let evaluate_path_fn_config = Rc::new(evaluate_path_fn_config);

  // A parenthesis is not a different argument, at either level. Read bare,
  // `stylex.props(([a, b]))` was not flattened into its elements and
  // `stylex.props((styles.root))` reached no arm of the match below, so both
  // handed the whole merge back to the runtime.
  let args_path = call
    .args
    .iter()
    .flat_map(|arg| match normalize_expr(&arg.expr) {
      Expr::Array(arr) => arr.elems.clone(),
      _ => vec![Some(arg.clone())],
    })
    .flatten()
    .collect::<Vec<ExprOrSpread>>();
  let mut resolved_args = Vec::with_capacity(args_path.len());

  for arg_path in args_path.iter() {
    current_index += 1;

    let arg = normalize_expr(&arg_path.expr);

    match &arg {
      // The four kinds that name a style, each answered once. The kind used to
      // be asked again after the argument was read, which left a kind the
      // reader never answers for in between.
      Expr::Object(_) | Expr::Ident(_) | Expr::Member(_) | Expr::Call(_) => {
        let resolved = parse_nullable_style(arg, state, &evaluate_path_fn_config);
        let is_unreadable = resolved == StyleObject::Other;

        // A style the compiler cannot read stays for the runtime to apply, and
        // the bail-out is what keeps the whole call there.
        if is_unreadable {
          bail_out_index = Some(current_index);
          bail_out = true;
        }

        // An object and a name merge as they stand. A member merges too, unless
        // it is one of those the bail-out has just kept for the runtime --
        // merging it here would apply it twice. A call never merges: a dynamic
        // atom (`_temp.color(c)`) or a dynamic create style
        // (`styles.opacity(1)`) is the runtime's to apply.
        let merges = match &arg {
          Expr::Object(_) | Expr::Ident(_) => true,
          Expr::Member(_) => !is_unreadable,
          _ => false,
        };

        if merges {
          resolved_args.push(ResolvedArg::style_object(resolved));
        }
      },
      Expr::Cond(CondExpr {
        test,
        cons: consequent,
        alt: alternate,
        ..
      }) => {
        let primary = parse_nullable_style(consequent, state, &evaluate_path_fn_config);
        let fallback = parse_nullable_style(alternate, state, &evaluate_path_fn_config);

        if primary.eq(&StyleObject::Other) || fallback.eq(&StyleObject::Other) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          resolved_args.push(ResolvedArg::conditional(
            *test.clone(),
            Some(primary),
            Some(fallback),
          ));

          conditional += 1;
        }
      },
      Expr::Bin(BinExpr {
        left: left_path,
        op,
        right: right_path,
        ..
      }) => {
        if !op.eq(&BinaryOp::LogicalAnd) {
          bail_out_index = Some(current_index);
          bail_out = true;
        } else {
          let left_resolved = parse_nullable_style(left_path, state, &evaluate_path_fn_config);
          let right_resolved = parse_nullable_style(right_path, state, &evaluate_path_fn_config);

          if !left_resolved.eq(&StyleObject::Other) || right_resolved.eq(&StyleObject::Other) {
            bail_out_index = Some(current_index);
            bail_out = true;
          } else {
            resolved_args.push(ResolvedArg::conditional(
              *left_path.clone(),
              Some(right_resolved),
              None,
            ));

            conditional += 1;
          }
        }
      },
      _ => {
        bail_out_index = Some(current_index);
        bail_out = true;
      },
    }

    if conditional > 4 {
      bail_out = true;
    }

    if bail_out {
      // Stop at the first bail. On the bail path `resolved_args`/`conditional`
      // are discarded entirely and the output is produced by the
      // `MemberTransform` re-walk over *all* `call.args` below — which already
      // registers and keeps every member arg's `stylex.create` styles. Scanning
      // further here would only repeat `parse_nullable_style`/`evaluate` work on
      // args past the bail and risk choking on a shape only that path rejects.
      break;
    }
  }

  if !state.enable_inlined_conditional_merge() && conditional > 0 {
    bail_out = true;
  }

  if bail_out {
    let mut non_null_props: NonNullProps = NonNullProps::Vec(vec![]);
    let mut index = -1;

    for arg_path in call.args.iter_mut() {
      index += 1;

      let mut member_transform = MemberTransform {
        index,
        bail_out_index,
        non_null_props,
        state: &mut *state,
        functions: &evaluate_path_fn_config,
      };

      arg_path.expr.visit_with(&mut member_transform);

      index = member_transform.index;
      bail_out_index = member_transform.bail_out_index;
      non_null_props = member_transform.non_null_props;

      // Hoist any inline compiled-style objects (produced by atoms) to module
      // scope so the runtime `stylex.props` receives a stable reference instead
      // of a re-created object literal.
      //
      // A second walk, and it stays one. The reader above stops at a member
      // expression, because counting a nested one would move the bail-out
      // point, while this walk must reach an object wherever it sits. One walk
      // could serve only one of those two rules.
      let mut object_hoister = CompiledStyleObjectHoister {
        state: &mut *state,
        hoist_expression,
      };
      object_hoister.hoist_in_argument(&mut arg_path.expr);
    }
  } else {
    let string_expression = make_string_expression(&resolved_args, transform);

    if let Expr::Object(string_expression) = &string_expression
      && state.has_jsx_spread_call(call)
      && !string_expression.props.is_empty()
    {
      let jsx_attr_expressions = string_expression
        .props
        .iter()
        .map(static_jsx_attr_from_prop)
        .collect::<Option<Vec<_>>>();

      // Store the JSX attributes to replace the spread element
      if let Some(jsx_attr_expressions) = jsx_attr_expressions {
        state.set_jsx_spread_replacement(call, jsx_attr_expressions);

        return None; // Early return to skip normal object creation
      }
    }

    return Some(string_expression);
  }

  None
}

/// The JSX attribute a compiled property spells, where it spells one.
///
/// The properties read here are the ones the merge answered with when no
/// argument was conditional. `make_string_expression` builds those through
/// `create_key_value_prop`, which spells a key as a bare name or as a quoted
/// string and never as a computed one. So `key_value_name` below has no
/// computed key to refuse, and the guard that stood here had no input. A quoted
/// key is usual on this path: `data-style-src` is one, and
/// `local_static_styles` writes it into an attribute.
///
/// This reader never sees the table that the conditional case builds, because
/// that case answers a member read on the table and the caller matches an
/// object. `files_every_answer_as_a_key_value_under_a_name` holds the table's
/// own shape.
///
/// What is left to decide is the value, and only a literal can be written into
/// an attribute.
fn static_jsx_attr_from_prop(prop: &PropOrSpread) -> Option<JSXAttrOrSpread> {
  prop
    .as_prop()
    .and_then(|prop| prop.as_key_value())
    .and_then(|key_value| {
      key_value
        .value
        .as_lit()
        .and_then(convert_lit_to_string)
        .map(|value| {
          create_jsx_attr_or_spread(create_jsx_attr(
            key_value_name(key_value).as_ref(),
            JSXAttrValue::Str(value.into()),
          ))
        })
    })
}

/// Hoists inline compiled-style objects (those carrying the `$$css: true`
/// marker, produced by the atoms transform) out of a `stylex.props(...)`
/// argument and into a module-scoped `const`, replacing the object with a
/// reference to it.
struct CompiledStyleObjectHoister<'a> {
  state: &'a mut StateManager,
  hoist_expression: fn(Expr, &mut StateManager) -> Ident,
}

impl CompiledStyleObjectHoister<'_> {
  /// Hoists the compiled style objects in one argument of the call.
  ///
  /// An argument that is itself an object is the only object checked. Any
  /// other argument is searched for objects at every depth.
  fn hoist_in_argument(&mut self, arg: &mut Expr) {
    let arg = normalize_expr_mut(arg);

    if arg.is_object() {
      self.hoist_object(arg);
    } else {
      arg.visit_mut_children_with(self);
    }
  }

  /// Hoists `expr` if it is a compiled style object that can move to program
  /// level, and tells whether it did.
  fn hoist_object(&mut self, expr: &mut Expr) -> bool {
    let Expr::Object(object) = expr else {
      return false;
    };

    if !object_has_css_marker(object) || !can_hoist_object(object, self.state) {
      return false;
    }

    *expr = Expr::Ident((self.hoist_expression)(expr.take(), self.state));

    true
  }
}

impl VisitMut for CompiledStyleObjectHoister<'_> {
  /// Outer objects come first. A hoisted object is not searched again, and an
  /// object that stays is searched for compiled objects inside it.
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    if !self.hoist_object(expr) {
      expr.visit_mut_children_with(self);
    }
  }
}

/// Whether every name the object reads is visible at program level, so the
/// object can move there.
///
/// Each property must be a plain or shorthand property. A computed key must
/// not read a hidden value anywhere in its expression. A value must be a
/// literal, a name or a template literal, and it must not read a hidden value.
/// See [`reads_a_value_hidden_from_a_hoist`] for what that is. A global is
/// visible everywhere, so it passes.
///
/// This is stricter than the reference. Its `canHoistObject` accepts any
/// template literal and any computed key that is not a bare name. So an object
/// that reads a parameter in `${size}` or in `[map.key]` moves, and the module
/// throws a `ReferenceError` when it loads. An object that reads `${this.k}`
/// in a method moves too, and then reads another value. An object that reads a
/// binding that its own statement or a later one declares moves in front of
/// that declaration, where the name has no value yet. And an object that reads
/// a binding that is assigned again keeps the value it read when the module
/// loaded. These departures from the reference are deliberate: each one
/// changes the output only where the reference output throws or reads the
/// wrong value.
fn can_hoist_object(object: &ObjectLit, state: &StateManager) -> bool {
  object.props.iter().all(|prop| {
    let PropOrSpread::Prop(prop) = prop else {
      return false;
    };

    match prop.as_ref() {
      Prop::Shorthand(ident) => !hides_its_value_from_a_hoist(ident, state),
      Prop::KeyValue(key_value) => {
        let key_is_visible = match &key_value.key {
          PropName::Computed(computed) => !reads_a_value_hidden_from_a_hoist(&computed.expr, state),
          _ => true,
        };
        let value_is_visible = match normalize_expr(&key_value.value) {
          Expr::Lit(_) => true,
          value @ (Expr::Ident(_) | Expr::Tpl(_)) => {
            !reads_a_value_hidden_from_a_hoist(value, state)
          },
          _ => false,
        };

        key_is_visible && value_is_visible
      },
      _ => false,
    }
  })
}

/// Whether `expr` reads a value that is not the same at program level.
///
/// That is a name that [`hides_its_value_from_a_hoist`], or a value the
/// enclosing function gives: `this`, `super`, `new.target`, `await` and
/// `yield`. A property name after a dot is not a read of a binding, so
/// `map.key` tests only `map`.
fn reads_a_value_hidden_from_a_hoist(expr: &Expr, state: &StateManager) -> bool {
  let mut search = HiddenRead {
    state,
    found: false,
  };
  expr.visit_with(&mut search);
  search.found
}

/// Whether the name `ident` reads a value that a hoisted declaration does not
/// see: `arguments`, a binding below program level, a program-level binding
/// that is not declared yet in front of the statement that holds the object,
/// or a binding that is assigned again. A hoisted declaration reads its value
/// once, when the module loads.
fn hides_its_value_from_a_hoist(ident: &Ident, state: &StateManager) -> bool {
  ident.sym == "arguments"
    || state.is_enclosed_binding(ident)
    || state.is_undeclared_program_binding(ident)
    || state.has_binding_reassignment(ident)
}

/// Records whether the walk visits a read that [`reads_a_value_hidden_from_a_hoist`]
/// refuses.
struct HiddenRead<'a> {
  state: &'a StateManager,
  found: bool,
}

impl Visit for HiddenRead<'_> {
  fn visit_ident(&mut self, ident: &Ident) {
    self.found |= hides_its_value_from_a_hoist(ident, self.state);
  }

  fn visit_this_expr(&mut self, _: &ThisExpr) {
    self.found = true;
  }

  fn visit_super(&mut self, _: &Super) {
    self.found = true;
  }

  fn visit_await_expr(&mut self, _: &AwaitExpr) {
    self.found = true;
  }

  fn visit_yield_expr(&mut self, _: &YieldExpr) {
    self.found = true;
  }

  /// `import.meta` is the same everywhere in the module, so only
  /// `new.target` counts.
  fn visit_meta_prop_expr(&mut self, meta: &MetaPropExpr) {
    self.found |= meta.kind == MetaPropKind::NewTarget;
  }
}

/// Whether an object literal carries a `$$css: true` property, marking it as a
/// compiled StyleX style object.
fn object_has_css_marker(object: &ObjectLit) -> bool {
  object.props.iter().any(|prop| {
    let PropOrSpread::Prop(prop) = prop else {
      return false;
    };
    let Prop::KeyValue(key_value) = prop.as_ref() else {
      return false;
    };

    let is_css_key = match &key_value.key {
      PropName::Ident(ident) => ident.sym.as_ref() == COMPILED_KEY,
      PropName::Str(strng) => strng.value.as_str() == Some(COMPILED_KEY),
      _ => false,
    };

    is_css_key
      && matches!(key_value.value.as_ref(), Expr::Lit(Lit::Bool(bool_lit)) if bool_lit.value)
  })
}

#[cfg(test)]
#[path = "tests/stylex_merge_tests.rs"]
mod tests;
