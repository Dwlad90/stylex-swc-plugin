use std::rc::Rc;

use swc_core::ecma::ast::{Expr, Ident, Lit, MemberExpr, MemberProp};

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
  },
  structures::{functions::FunctionMap, state_manager::StateManager, types::FlatCompiledStyles},
  utils::{
    ast::convertors::{expr_to_bool, expr_to_str, key_value_to_str, lit_to_string},
    common::reduce_ident_count,
    js::evaluate::evaluate,
  },
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum StyleObject {
  Style(FlatCompiledStyles),
  Nullable,
  Other,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ResolvedArg {
  StyleObject(StyleObject, Ident, MemberExpr),
  ConditionalStyle(
    Expr,
    Option<StyleObject>,
    Option<StyleObject>,
    Ident,
    MemberExpr,
  ),
}

pub(crate) fn parse_nullable_style(
  path: &Expr,
  state: &mut StateManager,
  should_reduce_count: bool,
) -> StyleObject {
  match path {
    Expr::Lit(lit) => {
      if let Lit::Null(_) = lit {
        StyleObject::Nullable
      } else {
        StyleObject::Other
      }
    }
    Expr::Ident(ident) => {
      if ident.sym == "undefined" {
        StyleObject::Nullable
      } else {
        if should_reduce_count {
          reduce_ident_count(state, ident);
        }
        StyleObject::Other
      }
    }
    Expr::Member(member) => {
      let mut obj_name: Option<String> = None;
      let mut prop_name: Option<String> = None;

      if let Some(obj_ident) = member.obj.as_ident()
        && state.style_map.contains_key(obj_ident.sym.as_str())
      {
        if should_reduce_count && let Some(member_ident) = member.obj.as_ident() {
          reduce_ident_count(state, member_ident);
        }

        match &member.prop {
          MemberProp::Ident(prop_ident) => {
            obj_name = Some(obj_ident.sym.as_str().to_string());
            prop_name = Some(prop_ident.sym.as_str().to_string());
          }
          MemberProp::Computed(computed) => {
            if let Some(lit) = computed.expr.as_lit() {
              obj_name = Some(obj_ident.sym.as_str().to_string());
              prop_name = lit_to_string(lit);
            }
          }
          MemberProp::PrivateName(_) => {}
        }
      }

      if let Some(obj_name) = obj_name
        && let Some(prop_name) = prop_name
      {
        let style = state.style_map.get(&obj_name);

        if let Some(style) = style {
          let style_value = style.get(&prop_name);

          if let Some(style_value) = style_value {
            return StyleObject::Style((**style_value).clone());
          }
        }
      }

      StyleObject::Other
    }
    _ => StyleObject::Other,
  }
}

fn _evaluate_style_object(
  path: &Expr,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Option<StyleObject> {
  let parsed_obj = evaluate(path, state, functions);
  if parsed_obj.confident
    && let Some(EvaluateResultValue::Expr(Expr::Object(obj))) = parsed_obj.value.as_ref()
  {
    let style_value: FlatCompiledStyles = obj
      .props
      .iter()
      .filter_map(|prop| {
        prop
          .as_prop()
          .and_then(|prop| prop.as_key_value())
          .map(|key_value| {
            let key = key_value_to_str(key_value);
            let value = if let Some(strng) = expr_to_str(key_value.value.as_ref(), state, functions)
            {
              FlatCompiledStylesValue::String(strng)
            } else {
              FlatCompiledStylesValue::Bool(expr_to_bool(
                key_value.value.as_ref(),
                state,
                functions,
              ))
            };

            (key, Rc::new(value))
          })
      })
      .collect();

    return Some(StyleObject::Style(style_value));
  };

  None
}
