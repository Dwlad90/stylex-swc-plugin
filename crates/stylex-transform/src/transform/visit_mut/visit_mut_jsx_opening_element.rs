use swc_core::{
  common::{DUMMY_SP, Span, Spanned, comments::Comments},
  ecma::{
    ast::{
      Bool, CallExpr, Callee, Expr, ExprOrSpread, JSXAttrName, JSXAttrOrSpread, JSXAttrValue,
      JSXElementName, JSXExpr, JSXOpeningElement, Lit, MemberExpr, MemberProp, Prop, PropName,
      PropOrSpread,
    },
    visit::VisitMutWith,
  },
};

use crate::{StyleXTransform, shared::structures::state_manager::InsertionSlot};
use stylex_ast::ast::factories::{
  create_arrow_expression, create_ident, create_ident_call_expr, create_ident_name,
  create_import_namespace_decl, create_jsx_spread_attr, create_member_call_expr, create_object_lit,
  create_spread_prop,
};
use stylex_constants::constants::{api_names::STYLEX_PROPS, common::RUNTIME_JSX_CALL_NAMES};
use stylex_enums::{core::TransformationCycle, counter_mode::CounterMode};
use stylex_structures::{named_import_source::ImportSources, uid_generator::UidGenerator};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_jsx_opening_element_impl(
    &mut self,
    jsx_opening_element: &mut JSXOpeningElement,
  ) {
    // Only transform during Discover cycle
    if self.state.cycle != TransformationCycle::Discover {
      jsx_opening_element.visit_mut_children_with(self);
      return;
    }

    // Check if sxPropName is enabled
    let sx_prop_name = match self.state.options.sx_prop_name.clone() {
      Some(name) => name,
      None => {
        jsx_opening_element.visit_mut_children_with(self);
        return;
      },
    };

    // Only transform lowercase JSX element names (HTML elements, not React
    // components)
    let is_lowercase_element = match &jsx_opening_element.name {
      JSXElementName::Ident(ident) => ident
        .sym
        .chars()
        .next()
        .map(|c| c.is_lowercase())
        .unwrap_or(false),
      _ => false,
    };

    if !is_lowercase_element {
      jsx_opening_element.visit_mut_children_with(self);
      return;
    }

    // Find the sx attribute index
    let sx_attr_idx = jsx_opening_element.attrs.iter().position(|attr| {
      if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr
        && let JSXAttrName::Ident(name) = &jsx_attr.name
      {
        return name.sym.as_str() == sx_prop_name.as_str();
      }
      false
    });

    if let Some(idx) = sx_attr_idx {
      let replacement = if let JSXAttrOrSpread::JSXAttr(jsx_attr) = &jsx_opening_element.attrs[idx]
      {
        if let Some(JSXAttrValue::JSXExprContainer(container)) = &jsx_attr.value {
          if let JSXExpr::Expr(expr) = &container.expr {
            let value_expr = *expr.clone();
            let stylex_local_name = self.get_stylex_runtime_binding(jsx_opening_element.span);

            // sx={[styles.a, styles.b]} → {...stylex.props(styles.a, styles.b)}
            let args = sx_value_to_props_args(value_expr);
            let call = Expr::Call(build_stylex_props_call(stylex_local_name, args));

            Some(create_jsx_spread_attr(call))
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };

      if let Some(spread) = replacement {
        jsx_opening_element.attrs[idx] = spread;
      }
    }

    jsx_opening_element.visit_mut_children_with(self);
  }

  /// Transform compiled JSX/VDOM calls with an `sx` prop during the
  /// Initializing cycle.
  ///
  /// Handles:
  /// - React: `_jsx("div", { sx: expr })` / `_jsxs("div", { sx: expr })`
  /// - React classic: `React.createElement("div", { sx: expr })`
  /// - Vue: `_createElementBlock("div", { sx: expr })` /
  ///   `_createElementVNode("div", { sx: expr })`
  ///
  /// Transforms to: `fn("div", { ...stylex.props(expr), ... })`
  pub(crate) fn transform_sx_in_compiled_jsx(&mut self, expr: &Expr) -> Option<Expr> {
    let sx_prop_name = self.state.options.sx_prop_name.as_deref()?;

    let call = expr.as_call()?;

    if !is_jsx_runtime_call(call) {
      return None;
    }

    // First arg must be a lowercase string literal (HTML element)
    let first_arg = call.args.first()?;
    let element_name = match first_arg.expr.as_ref() {
      Expr::Lit(Lit::Str(s)) => s.value.as_str().unwrap_or(""),
      _ => return None,
    };
    if !element_name
      .chars()
      .next()
      .map(|c: char| c.is_lowercase())
      .unwrap_or(false)
    {
      return None;
    }

    // Second arg must be an object literal
    let second_arg = call.args.get(1)?;
    let obj_lit = match second_arg.expr.as_ref() {
      Expr::Object(o) => o.clone(),
      _ => return None,
    };

    // Find the sx prop index
    let sx_prop_idx = find_sx_prop_idx(&obj_lit.props, sx_prop_name)?;

    // Extract the sx value and build args
    let sx_value = extract_prop_value(&obj_lit.props[sx_prop_idx])?;
    let stylex_local_name = self.get_stylex_runtime_binding(expr.span());
    let args = sx_value_to_props_args(sx_value);

    // Replace the sx prop with: ...stylex.props(...args)
    let mut new_props = obj_lit.props;
    let call_expr = Expr::Call(build_stylex_props_call(stylex_local_name, args));
    new_props[sx_prop_idx] = create_spread_prop(call_expr);

    let mut new_call = call.clone();
    new_call.args[1] = ExprOrSpread {
      spread: None,
      expr: Box::new(Expr::Object(create_object_lit(new_props))),
    };

    Some(Expr::Call(new_call))
  }

  /// Transform Solid.js compiled `sx` attribute during the Initializing cycle.
  ///
  /// Solid.js compiles `<div sx={styles.main}>` to:
  /// `_$setAttribute(_el$, "sx", styles.main)`
  ///
  /// We transform it to:
  /// `_$spread(_el$, _$mergeProps(() => stylex.props(styles.main)), false,
  /// true)`
  pub(crate) fn transform_sx_in_solid_set_attribute(&mut self, expr: &Expr) -> Option<Expr> {
    let sx_prop_name = self.state.options.sx_prop_name.as_deref()?;

    let call = expr.as_call()?;

    // Check callee is _$setAttribute
    let is_set_attribute = match &call.callee {
      Callee::Expr(e) => match e.as_ref() {
        Expr::Ident(ident) => ident.sym.as_str() == "_$setAttribute",
        _ => false,
      },
      _ => false,
    };
    if !is_set_attribute {
      return None;
    }

    // Need at least 3 args: (el, "sx", value)
    if call.args.len() < 3 {
      return None;
    }

    // Args[1] must be the string matching sx_prop_name
    let attr_name = match call.args[1].expr.as_ref() {
      Expr::Lit(Lit::Str(s)) => s.value.as_str().unwrap_or(""),
      _ => return None,
    };
    if attr_name != sx_prop_name {
      return None;
    }

    let el_arg = call.args[0].clone();
    let value_expr = *call.args[2].expr.clone();

    let stylex_local_name = self.get_stylex_runtime_binding(expr.span());
    let args = sx_value_to_props_args(value_expr);

    // Build: () => stylex.props(args...)
    let props_call = Expr::Call(build_stylex_props_call(stylex_local_name, args));
    let arrow_fn = create_arrow_expression(props_call);

    // Build: _$mergeProps(() => stylex.props(...))
    let merge_props_call = Expr::Call(create_ident_call_expr(
      "_$mergeProps",
      vec![ExprOrSpread {
        spread: None,
        expr: Box::new(arrow_fn),
      }],
    ));

    // Build: _$spread(el, _$mergeProps(() => stylex.props(...)), false, true)
    use stylex_ast::ast::factories::create_expr_or_spread;
    Some(Expr::Call(create_ident_call_expr(
      "_$spread",
      vec![
        el_arg,
        create_expr_or_spread(merge_props_call),
        create_expr_or_spread(Expr::Lit(Lit::Bool(Bool {
          span: DUMMY_SP,
          value: false,
        }))),
        create_expr_or_spread(Expr::Lit(Lit::Bool(Bool {
          span: DUMMY_SP,
          value: true,
        }))),
      ],
    )))
  }

  /// Resolve the local name of the value-level `stylex` namespace binding to
  /// use for an `sx` runtime call, injecting an `import * as <name> from
  /// '<source>'` declaration when none exists.
  ///
  /// 1. Reuse an existing value-level namespace/default import if present and
  ///    not shadowed by a local binding in scope at the `sx` site.
  /// 2. Otherwise find the first existing import declaration (including
  ///    type-only) whose source is a configured import source.
  /// 3. Pick the import source: existing import source, else the first
  ///    configured custom source, else the default `@stylexjs/stylex`.
  /// 4. Pick the local name: a generated uid (`_stylex`) when an existing
  ///    import source was found or a `stylex` binding already exists,
  ///    otherwise plain `stylex`.
  /// 5. Prepend the namespace import, register it, and return the local name.
  fn get_stylex_runtime_binding(&mut self, site: Span) -> String {
    let site = if site.is_dummy() {
      self.state.current_site_span
    } else {
      site
    };

    // 1. Reuse an existing value-level namespace/default import, skipping any
    //    whose local name is shadowed by a non-import binding in scope at the
    //    `sx` site. The shadow check is position-aware, using the pre-scanned
    //    scope spans and the current `sx` site position.
    if let Some(local_name) = self
      .state
      .stylex_import_stringified()
      .into_iter()
      .find(|name| !self.state.is_locally_rebound_at(name, site))
    {
      return local_name;
    }

    // 2. First existing import declaration (incl. type-only) whose source is
    //    a configured import source.
    let existing_import_source = self
      .state
      .existing_import_sources
      .iter()
      .find(|source| self.state.is_import_source(source))
      .cloned();

    // 3. importSource = existing ?? first configured custom ?? default.
    let import_source = existing_import_source.clone().unwrap_or_else(|| {
      self
        .state
        .import_sources_stringified()
        .into_iter()
        .find(|source| !is_default_import_source(source))
        .unwrap_or_else(|| "@stylexjs/stylex".to_string())
    });

    // 4. localName = uid when an existing import source was found or a
    //    `stylex` binding already exists, else plain `stylex`. The uid is
    //    `_stylex`, then `_stylex2`, ..., skipping any name already bound in
    //    the module.
    let stylex_local_name = if existing_import_source.is_some() || self.state.has_binding("stylex")
    {
      let uid_generator = UidGenerator::new("stylex", CounterMode::Local);
      let mut candidate = uid_generator.generate();
      while self.state.has_binding(&candidate) {
        candidate = uid_generator.generate();
      }
      candidate
    } else {
      "stylex".to_string()
    };

    // 5. Prepend `import * as <local> from '<source>'`, register it so the
    //    pipeline proceeds (the module visitor bails when no import path was
    //    discovered) and subsequent `sx` attributes reuse this binding.
    let import_item = create_import_namespace_decl(&stylex_local_name, &import_source);
    self
      .state
      .queue_insertion(InsertionSlot::PrependImport, import_item);
    self.state.insert_import_path(import_source);
    self
      .state
      .insert_stylex_import(ImportSources::Regular(stylex_local_name.clone()));

    stylex_local_name
  }
}

/// Whether `source` is one of the default StyleX import sources (`stylex` /
/// `@stylexjs/stylex`) seeded into every configuration, as opposed to a
/// user-configured custom source.
fn is_default_import_source(source: &str) -> bool {
  source == "stylex" || source == "@stylexjs/stylex"
}

/// Convert an `sx` value expression into args for `stylex.props(...)`.
/// Array literals are unpacked: `[a, b]` → `[a, b]` as separate args.
/// Other expressions are wrapped: `expr` → `[expr]`.
fn sx_value_to_props_args(value_expr: Expr) -> Vec<ExprOrSpread> {
  match value_expr {
    Expr::Array(array_lit) => array_lit.elems.into_iter().flatten().collect(),
    _ => vec![ExprOrSpread {
      spread: None,
      expr: Box::new(value_expr),
    }],
  }
}

/// Build a `<stylex_local_name>.props(args...)` call expression, where
/// `stylex_local_name` is the value-level namespace binding resolved by
/// [`StyleXTransform::get_stylex_runtime_binding`].
fn build_stylex_props_call(stylex_local_name: String, args: Vec<ExprOrSpread>) -> CallExpr {
  let member = MemberExpr {
    span: DUMMY_SP,
    obj: Box::new(Expr::Ident(create_ident(&stylex_local_name))),
    prop: MemberProp::Ident(create_ident_name(STYLEX_PROPS)),
  };
  create_member_call_expr(member, args)
}

/// Find the index of the prop with key matching `sx_prop_name` in a props list.
fn find_sx_prop_idx(props: &[PropOrSpread], sx_prop_name: &str) -> Option<usize> {
  props.iter().position(|prop| {
    if let PropOrSpread::Prop(p) = prop
      && let Prop::KeyValue(kv) = p.as_ref()
    {
      return match &kv.key {
        PropName::Ident(ident) => ident.sym.as_str() == sx_prop_name,
        PropName::Str(s) => s.value.as_str().unwrap_or("") == sx_prop_name,
        _ => false,
      };
    }
    false
  })
}

/// Extract the value from a `KeyValue` prop entry.
fn extract_prop_value(prop: &PropOrSpread) -> Option<Expr> {
  if let PropOrSpread::Prop(p) = prop
    && let Prop::KeyValue(kv) = p.as_ref()
  {
    return Some(kv.value.as_ref().clone());
  }
  None
}

/// Check if a `CallExpr` is a JSX/VDOM runtime call that takes `(elementName,
/// props, ...)`.
fn is_jsx_runtime_call(call: &CallExpr) -> bool {
  match &call.callee {
    Callee::Expr(e) => match e.as_ref() {
      Expr::Ident(ident) => {
        let name = ident.sym.as_str();

        let name = name.strip_suffix("DEV").unwrap_or(name);
        let name = name.strip_prefix('_').unwrap_or(name);

        RUNTIME_JSX_CALL_NAMES.contains(&name)
      },
      Expr::Member(member) => {
        if let (Expr::Ident(obj), MemberProp::Ident(prop)) = (member.obj.as_ref(), &member.prop) {
          obj.sym.as_str() == "React" && prop.sym.as_str() == "createElement"
        } else {
          false
        }
      },
      _ => false,
    },
    _ => false,
  }
}
