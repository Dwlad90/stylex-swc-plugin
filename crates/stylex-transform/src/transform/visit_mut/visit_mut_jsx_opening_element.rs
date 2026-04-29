use stylex_macros::stylex_panic;
use swc_core::{
  common::{DUMMY_SP, comments::Comments},
  ecma::{
    ast::{
      Bool, CallExpr, Callee, Expr, ExprOrSpread, JSXAttrName, JSXAttrOrSpread, JSXAttrValue,
      JSXElementName, JSXExpr, JSXOpeningElement, Lit, MemberExpr, MemberProp, Prop, PropName,
      PropOrSpread,
    },
    visit::VisitMutWith,
  },
};

use crate::{StyleXTransform, shared::structures::state_manager::ImportKind};
use stylex_ast::ast::factories::{
  create_arrow_expression, create_ident, create_ident_call_expr, create_ident_name,
  create_jsx_spread_attr, create_member_call_expr, create_object_lit, create_spread_prop,
};
use stylex_constants::constants::{api_names::STYLEX_PROPS, common::RUNTIME_JSX_CALL_NAMES};
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_jsx_opening_element_impl(
    &mut self,
    jsx_opening_element: &mut JSXOpeningElement,
  ) {
    // Only transform during Initializing cycle
    if self.state.cycle != TransformationCycle::Initializing {
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
            let stylex_ident_name = self.get_stylex_ident_name();
            let props_ident_name = self.get_props_ident_name();

            // sx={[styles.a, styles.b]} → {...stylex.props(styles.a, styles.b)}
            let args = sx_value_to_props_args(value_expr);
            let call = Expr::Call(build_stylex_props_call(
              stylex_ident_name,
              props_ident_name,
              args,
            ));

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
  pub(crate) fn transform_sx_in_compiled_jsx(&self, expr: &Expr) -> Option<Expr> {
    let sx_prop_name = self.state.options.sx_prop_name.as_deref()?;

    let call = expr.as_call()?;

    if !is_jsx_runtime_call(call) {
      return None;
    }

    // First arg must be a lowercase string literal (HTML element)
    let first_arg = call.args.first()?;
    let element_name = match first_arg.expr.as_ref() {
      Expr::Lit(Lit::Str(s)) => s.value.as_str().unwrap_or("").to_string(),
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
    let stylex_ident_name = self.get_stylex_ident_name();
    let props_ident_name = self.get_props_ident_name();
    let args = sx_value_to_props_args(sx_value);

    // Replace the sx prop with: ...stylex.props(...args)
    let mut new_props = obj_lit.props.clone();
    let call_expr = Expr::Call(build_stylex_props_call(
      stylex_ident_name,
      props_ident_name,
      args,
    ));
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
  pub(crate) fn transform_sx_in_solid_set_attribute(&self, expr: &Expr) -> Option<Expr> {
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
      Expr::Lit(Lit::Str(s)) => s.value.as_str().unwrap_or("").to_string(),
      _ => return None,
    };
    if attr_name != sx_prop_name {
      return None;
    }

    let el_arg = call.args[0].clone();
    let value_expr = *call.args[2].expr.clone();

    let stylex_ident_name = self.get_stylex_ident_name();
    let props_ident_name = self.get_props_ident_name();
    let args = sx_value_to_props_args(value_expr);

    // Build: () => stylex.props(args...)
    let props_call = Expr::Call(build_stylex_props_call(
      stylex_ident_name,
      props_ident_name,
      args,
    ));
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

  /// Get the stylex ident name from the import paths.
  /// Returns the first stylex import name from the import paths.
  fn get_stylex_ident_name(&self) -> Option<String> {
    self.state.stylex_import_stringified().into_iter().next()
  }

  /// Get the props ident name from the import paths.
  /// Returns the first props import name from the import paths.
  fn get_props_ident_name(&self) -> Option<String> {
    self
      .state
      .get_stylex_api_import(ImportKind::Props)
      .and_then(|set| set.iter().next())
      .map(|ident| ident.to_string())
  }
}

/// Convert an `sx` value expression into args for `stylex.props(...)`.
/// Array literals are unpacked: `[a, b]` → `[a, b]` as separate args.
/// Other expressions are wrapped: `expr` → `[expr]`.
fn sx_value_to_props_args(value_expr: Expr) -> Vec<ExprOrSpread> {
  match &value_expr {
    Expr::Array(array_lit) => array_lit
      .elems
      .iter()
      .filter_map(|elem| elem.clone())
      .collect(),
    _ => vec![ExprOrSpread {
      spread: None,
      expr: Box::new(value_expr),
    }],
  }
}

/// Build a call expression for stylex props:
/// - `stylex.props(args...)` when `stylex_ident_name` is available
/// - `<props_ident_name>(args...)` (e.g. `props(args...)` or `sx(args...)`)
///   otherwise
fn build_stylex_props_call(
  stylex_ident_name: Option<String>,
  props_ident_name: Option<String>,
  args: Vec<ExprOrSpread>,
) -> CallExpr {
  if let Some(stylex_ident_name) = stylex_ident_name {
    let member = MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(create_ident(&stylex_ident_name))),
      prop: MemberProp::Ident(create_ident_name(STYLEX_PROPS)),
    };
    create_member_call_expr(member, args)
  } else if let Some(props_ident_name) = props_ident_name {
    create_ident_call_expr(&props_ident_name, args)
  } else {
    #[cfg_attr(coverage_nightly, coverage(off))]
    {
      stylex_panic!(
        "Could not resolve StyleX import. Ensure you have imported stylex or the props function."
      );
    }
  }
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
