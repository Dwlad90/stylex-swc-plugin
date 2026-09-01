use rustc_hash::{FxHashMap, FxHashSet};
use std::{any::type_name, ops::Deref};
use stylex_macros::{stylex_panic, stylex_unimplemented, stylex_unreachable};
use stylex_types::traits::StyleOptions;
use stylex_utils::{number::to_js_string, string::remove_quotes};
use swc_core::atoms::Atom;
use swc_core::ecma::ast::{
  Decl, Expr, KeyValueProp, Module, ModuleDecl, ModuleItem, ObjectPatProp, Pat, Prop, PropName,
  PropOrSpread, Stmt, VarDeclarator,
};

use stylex_ast::ast::convertors::{convert_wtf8_to_atom, expand_shorthand_prop};
use stylex_enums::top_level_expression::TopLevelExpressionKind;
use stylex_structures::{base_css_type::BaseCSSType, top_level_expression::TopLevelExpression};

use crate::state_manager::StateManager;
use stylex_constants::constants::messages::SPREAD_NOT_SUPPORTED;
use stylex_regex::regex::JSON_REGEX;

#[allow(dead_code)]
pub fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}

/// The name a property declares, or `None` where it declares no nameable one.
///
/// `None` is a spread, a getter, a setter, a method, or a computed key -- each
/// of which either has no key at all or has one this cannot read without
/// evaluating it. Stated once because two readers depend on the answer, and two
/// spellings of "which key is this" would let them disagree about whether two
/// properties collide.
fn prop_key(prop: &PropOrSpread) -> Option<Atom> {
  match prop {
    PropOrSpread::Prop(prop) => match prop.as_ref() {
      Prop::Shorthand(ident) => Some(ident.sym.clone()),
      Prop::KeyValue(key_val) => match &key_val.key {
        PropName::Ident(ident) => Some(ident.sym.clone()),
        PropName::Str(strng) => Some(convert_wtf8_to_atom(&strng.value)),
        // A numeric key is a string key spelled as a number: `{ 42: x }` and
        // `{ '42': x }` name one property in the language, so they have to read
        // as one name here. Rendered through `to_js_string` so the spelling is
        // the language's -- `1e21`, not `1000000000000000000000`.
        PropName::Num(number) => Some(Atom::from(to_js_string(number.value))),
        // `{ 1n: x }` names the property `"1"`, as every non-computed key that
        // is not already a string does.
        PropName::BigInt(big_int) => Some(Atom::from(big_int.value.to_string())),
        _ => None,
      },
      _ => None,
    },
    _ => None,
  }
}

/// Whether a key is an array index, in the sense JavaScript uses to decide
/// enumeration order: the canonical decimal spelling of an integer below
/// 2^32 - 1.
///
/// Canonical is what makes `"0"` one and `"00"`, `"+0"` and `"01"` not — those
/// round-trip to a different string, so the language treats them as ordinary
/// string keys and enumerates them in insertion order.
fn array_index_of(key: &str) -> Option<u32> {
  if key.len() > 1 && key.starts_with('0') {
    return None;
  }

  let index = key.parse::<u32>().ok()?;

  match index == u32::MAX {
    true => None,
    false => Some(index),
  }
}

/// Reorders an object's own properties the way JavaScript enumerates them:
/// every array-index key first in ascending numeric order, then every other key
/// in insertion order.
///
/// Not a detail of the object literal's spelling. The order properties come out
/// in is the order their declarations reach the stylesheet, so it decides which
/// of two rules at equal specificity wins -- and `{ color: 'red', ...['a'] }`
/// was emitting `color` before `0` where the language, and so upstream, puts
/// `0` first.
///
/// Stable within each group, so the insertion order of the string keys is
/// preserved exactly.
pub fn order_own_keys(props: Vec<PropOrSpread>) -> Vec<PropOrSpread> {
  let mut indexed: Vec<(u32, PropOrSpread)> = Vec::new();
  let mut named: Vec<PropOrSpread> = Vec::with_capacity(props.len());

  for prop in props {
    match prop_key(&prop).as_deref().and_then(array_index_of) {
      Some(index) => indexed.push((index, prop)),
      None => named.push(prop),
    }
  }

  if indexed.is_empty() {
    return named;
  }

  indexed.sort_by_key(|(index, _)| *index);

  let mut ordered = Vec::with_capacity(indexed.len() + named.len());
  ordered.extend(indexed.into_iter().map(|(_, prop)| prop));
  ordered.extend(named);

  ordered
}

pub fn remove_duplicates(props: Vec<PropOrSpread>) -> Vec<PropOrSpread> {
  let mut set = FxHashSet::default();
  let mut result = Vec::with_capacity(props.len());

  for prop in props.into_iter().rev() {
    let Some(key) = prop_key(&prop) else {
      continue;
    };

    if set.insert(key) {
      result.push(prop);
    }
  }

  result.reverse();

  result
}

/// The properties of `{ ...old_props, ...new_props }`.
///
/// This is `Object.assign`, which is what the reference implementation calls at
/// the spread and so is what "merge" has to mean here: shallow. A repeated key
/// takes the later value and keeps the position the key first took, so
/// `{ ...{ a: 1, b: 2 }, ...{ a: 3 } }` is `{ a: 3, b: 2 }` -- `a` is third in
/// no ordering.
///
/// Not a deep merge, despite what this used to be called: `{ ...{ a: { x: 1 } },
/// ...{ a: { y: 2 } } }` is `{ a: { y: 2 } }` in the language, and the nested
/// `x` is gone rather than merged in. Nesting is what a style object is mostly
/// made of -- a pseudo, an at-rule -- so a deep merge here quietly kept
/// declarations the source had replaced.
///
/// It also used to reverse. The result was assembled by pushing the old
/// properties onto the new and reversing the whole thing, which put the groups
/// in the right order and every property inside them in the wrong one, so
/// `{ ...{ color: 'red', opacity: 1 } }` emitted its two rules back to front.
/// One property is the common case and hid it.
pub fn assign_props(
  old_props: Vec<PropOrSpread>,
  new_props: Vec<PropOrSpread>,
) -> Vec<PropOrSpread> {
  let mut merged: Vec<PropOrSpread> = Vec::with_capacity(old_props.len() + new_props.len());
  // The position each key already took, so a repeated one is found by lookup
  // rather than by re-reading every property placed so far -- which was
  // quadratic, and cloned an `Atom` per comparison, on the path every object
  // spread goes through.
  let mut positions: FxHashMap<Atom, usize> = FxHashMap::default();

  for prop in old_props.into_iter().chain(new_props) {
    let Some(key) = prop_key(&prop) else {
      // A spread, a getter, a computed key: nothing here can say which key it
      // would land on, so it keeps its place and collides with nothing.
      merged.push(prop);
      continue;
    };

    match positions.get(&key) {
      Some(&position) => merged[position] = prop,
      None => {
        positions.insert(key, merged.len());
        merged.push(prop);
      },
    }
  }

  merged
}

pub fn get_css_value(key_value: KeyValueProp) -> (Box<Expr>, Option<BaseCSSType>) {
  let Some(obj) = key_value.value.as_object() else {
    return (key_value.value, None);
  };

  for prop in obj.props.clone().into_iter() {
    match prop {
      PropOrSpread::Spread(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
      PropOrSpread::Prop(mut prop) => {
        expand_shorthand_prop(&mut prop);

        match prop.deref() {
          Prop::KeyValue(key_value) => {
            if let Some(ident) = key_value.key.as_ident()
              && ident.sym == "syntax"
            {
              let value = obj.props.iter().find(|prop| {
                match prop {
                  PropOrSpread::Spread(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
                  PropOrSpread::Prop(prop) => {
                    let mut prop = prop.clone();
                    expand_shorthand_prop(&mut prop);

                    match prop.as_ref() {
                      Prop::KeyValue(key_value) => {
                        if let Some(ident) = key_value.key.as_ident() {
                          return ident.sym == "value";
                        }
                      },
                      _ => stylex_unimplemented!("Unsupported prop type in CSS value"),
                    }
                  },
                }

                false
              });

              if let Some(value) = value {
                let result_key_value = match value.as_prop().and_then(|prop| prop.as_key_value()) {
                  Some(kv) => kv,
                  None => stylex_panic!("Expected key-value property"),
                };

                return (result_key_value.value.clone(), Some(obj.clone().into()));
              }
            }
          },
          _ => stylex_unimplemented!("Unsupported prop type in CSS value"),
        }
      },
    }
  }

  (key_value.value, None)
}

pub fn fill_top_level_expressions(module: &Module, state: &mut StateManager) {
  module.body.iter().for_each(|item| match item {
    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
      if let Decl::Var(decl_var) = &export_decl.decl {
        for decl in &decl_var.decls {
          record_top_level_declarator(state, TopLevelExpressionKind::NamedExport, decl);
        }
      }
    },
    ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(export_decl)) => {
      match export_decl.expr.as_paren() {
        Some(paren) => {
          state.push_top_level_expression(TopLevelExpression(
            TopLevelExpressionKind::DefaultExport,
            paren.expr.as_ref().clone(),
            None,
          ));
        },
        _ => {
          state.push_top_level_expression(TopLevelExpression(
            TopLevelExpressionKind::DefaultExport,
            export_decl.expr.as_ref().clone(),
            None,
          ));
        },
      }
    },
    ModuleItem::Stmt(Stmt::Decl(Decl::Var(var))) => {
      for decl in &var.decls {
        record_top_level_declarator(state, TopLevelExpressionKind::Stmt, decl);
      }
    },
    _ => {},
  });
}

/// Record one declarator of a top-level variable declaration, `kind` telling
/// an exported one from a plain statement.
///
/// A declarator bound to a pattern rather than a name declares no single name
/// to record, so it contributes no top-level expression — `export const { a } =
/// expr;` is ordinary JavaScript, and an API that does require a name reports
/// that itself, against the call the author wrote. Its position is still worth
/// keeping: nothing else marks the call as program level.
fn record_top_level_declarator(
  state: &mut StateManager,
  kind: TopLevelExpressionKind,
  decl: &VarDeclarator,
) {
  let Some(decl_init) = decl.init.as_ref() else {
    return;
  };

  match decl.name.as_ident() {
    Some(ident) => {
      state.push_top_level_expression(TopLevelExpression(
        kind,
        decl_init.as_ref().clone(),
        Some(ident.sym.clone()),
      ));

      fill_state_declarations(state, decl);
    },
    None => {
      if let Expr::Call(call) = decl_init.as_ref()
        && !call.span.is_dummy()
      {
        state.pattern_bound_top_level_calls.insert(call.span);
      }
    },
  }
}

pub fn fill_state_declarations(state: &mut StateManager, decl: &VarDeclarator) {
  if !state.holds_declaration(decl) {
    state.push_declaration(decl.clone());
  }
}

#[allow(dead_code)]
fn get_variable_names(name: &Pat) -> Vec<String> {
  match name {
    Pat::Ident(ident) => vec![ident.id.sym.to_string()],
    Pat::Object(pat_object) => pat_object
      .props
      .iter()
      .flat_map(|prop| match prop {
        ObjectPatProp::KeyValue(kv) => get_variable_names(&kv.value),
        ObjectPatProp::Assign(assign) => get_variable_names(&Pat::Ident(assign.key.clone())),
        ObjectPatProp::Rest(rest) => get_variable_names(&rest.arg),
      })
      .collect(),
    Pat::Array(pat_array) => pat_array
      .elems
      .iter()
      .flatten()
      .flat_map(get_variable_names)
      .collect(),
    Pat::Rest(rest_pat) => get_variable_names(&rest_pat.arg),
    Pat::Invalid(_) | Pat::Expr(_) => vec![],
    Pat::Assign(assign) => get_variable_names(&assign.left),
  }
}

pub fn gen_file_based_identifier(file_name: &str, export_name: &str, key: Option<&str>) -> String {
  let key = key.map_or(String::new(), |k| format!(".{}", k));

  format!("{}//{}{}", file_name, export_name, key)
}

pub fn serialize_value_to_json_string<T: serde::Serialize>(value: T) -> String {
  match serde_json::to_string(&value) {
    Ok(json_str) => {
      if json_str.starts_with('"') && json_str.ends_with('"') && json_str.len() > 2 {
        match serde_json::from_str::<String>(&json_str) {
          Ok(inner_string) => {
            if inner_string.trim_start().starts_with('{') && !inner_string.contains("\":") {
              return js_object_to_json(&inner_string);
            }

            if inner_string.parse::<f64>().is_ok() {
              return inner_string;
            }

            remove_quotes(&inner_string).into_owned()
          },
          _ => remove_quotes(&json_str).into_owned(),
        }
      } else {
        json_str
      }
    },
    Err(err) => {
      stylex_panic!("Failed to serialize value. Error: {}", err)
    },
  }
}

pub(crate) fn js_object_to_json(js_str: &str) -> String {
  JSON_REGEX.replace_all(js_str, r#"$1"$2":"#).to_string()
}

/// Utility function to get the `StateManager` from the `StyleOptions` trait.
/// This is a helper function to get the `StateManager` from the `StyleOptions`
/// trait.
pub fn downcast_style_options_to_state_manager(state: &mut dyn StyleOptions) -> &mut StateManager {
  state
    .as_any_mut()
    .downcast_mut::<StateManager>()
    .unwrap_or_else(|| stylex_unreachable!("StyleOptions must be StateManager"))
}
