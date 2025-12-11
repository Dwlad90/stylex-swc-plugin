use std::rc::Rc;

use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, Ident, Lit, MemberExpr, MemberProp, ObjectLit};

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
  Unreachable,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ResolvedArg {
  StyleObject(StyleObject, Vec<Ident>, Vec<MemberExpr>),
  ConditionalStyle(
    Expr,
    Option<StyleObject>,
    Option<StyleObject>,
    Vec<Ident>,
    Vec<MemberExpr>,
  ),
}

pub(crate) fn parse_nullable_style(
  path: &Expr,
  state: &mut StateManager,
  evaluate_path_fn_config: &FunctionMap,
  should_reduce_count: bool,
) -> StyleObject {
  let result = match path {
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
  };

  if result == StyleObject::Other {
    let parsed_obj = evaluate(path, state, evaluate_path_fn_config);

    if parsed_obj.confident
      && let Some(result) = parsed_obj.value.as_ref()
    {
      let mut compiled_styles: IndexMap<String, Rc<FlatCompiledStylesValue>> = IndexMap::new();

      if let Some(value) = parse_compiled_styles(&mut compiled_styles, result) {
        return value;
      }
    }
  }

  result
}

fn parse_compiled_styles(
  compiled_styles: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  result: &EvaluateResultValue,
) -> Option<StyleObject> {
  match result {
    EvaluateResultValue::Vec(arr) => {
      for item in arr.iter() {
        match item.as_ref() {
          Some(EvaluateResultValue::Expr(expr)) => parse_nullable_object(compiled_styles, expr),
          Some(EvaluateResultValue::Vec(arr)) => {
            parse_compiled_styles(compiled_styles, &EvaluateResultValue::Vec(arr.clone()));
          }
          _ => {
            unimplemented!("Unhandled EvaluateResultValue in nullable style parsing array");
          }
        };
      }
      if compiled_styles.is_empty() {
        return Some(StyleObject::Other);
      }
      return Some(StyleObject::Style(compiled_styles.clone()));
    }
    EvaluateResultValue::Expr(expr) => {
      if expr.is_object() {
        parse_nullable_object(compiled_styles, expr);
        return Some(StyleObject::Style(compiled_styles.clone()));
      }
    }
    EvaluateResultValue::ThemeRef(_) => {
      return Some(StyleObject::Other);
    }
    _ => {
      unimplemented!("Unhandled EvaluateResultValue in nullable style parsing");
    }
  }
  None
}

fn parse_nullable_object(
  compiled_styles: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  expr: &Expr,
) {
  match expr {
    Expr::Object(ObjectLit { props, .. }) => {
      for prop in props.iter() {
        if let Some(key_value) = prop.as_prop().and_then(|p| p.as_key_value()) {
          let key = key_value_to_str(key_value);
          match key_value.value.as_ref() {
            Expr::Lit(lit) => parse_nullable_key_value(compiled_styles, key, lit),

            _ => {
              unimplemented!("Unhandled Expr type in nullable style parsing array");
            }
          };
        }
      }
    }
    _ => {
      unimplemented!("Unhandled Expr type in nullable style parsing array");
    }
  }
}

fn parse_nullable_key_value(
  compiled_styles: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  key: String,
  lit: &Lit,
) {
  match lit {
    Lit::Str(_) => {
      let value = lit_to_string(lit).expect("Failed to convert literal to string");

      compiled_styles.insert(key, Rc::new(FlatCompiledStylesValue::String(value)));
    }
    Lit::Bool(bool_lit) => {
      let value = bool_lit.value;
      compiled_styles.insert(key, Rc::new(FlatCompiledStylesValue::Bool(value)));
    }
    Lit::Null(_) => {
      compiled_styles.insert(key, Rc::new(FlatCompiledStylesValue::Null));
    }
    _ => {
      panic!("Unhandled literal type in nullable style parsing array",);
    }
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
