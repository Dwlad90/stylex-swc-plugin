use rustc_hash::FxHashSet;
use std::{any::type_name, collections::hash_map::Entry, ops::Deref, path::PathBuf};
use stylex_macros::{stylex_panic, stylex_unimplemented, stylex_unreachable};
use stylex_types::traits::StyleOptions;
use stylex_utils::string::remove_quotes;
use swc_core::{
  atoms::Atom,
  common::{DUMMY_SP, EqIgnoreSpan, FileName},
  ecma::{
    ast::{
      BinaryOp, Decl, Expr, Ident, ImportDecl, ImportSpecifier, KeyValueProp, MemberExpr, Module,
      ModuleDecl, ModuleExportName, ModuleItem, ObjectLit, ObjectPatProp, Pat, Prop, PropName,
      PropOrSpread, Stmt, VarDeclarator,
    },
    utils::drop_span,
  },
};

use stylex_enums::top_level_expression::TopLevelExpressionKind;
use stylex_structures::top_level_expression::TopLevelExpression;

use crate::shared::{
  structures::{
    base_css_type::BaseCSSType,
    functions::{FunctionConfigType, FunctionMap, FunctionType},
    state_manager::StateManager,
  },
  utils::ast::convertors::{convert_str_lit_to_atom, convert_wtf8_to_atom},
};
use stylex_constants::constants::messages::{
  ILLEGAL_PROP_VALUE, INVALID_UTF8, SPREAD_NOT_SUPPORTED, VAR_DECL_NAME_NOT_IDENT,
};
use stylex_enums::misc::VarDeclAction;
use stylex_regex::regex::JSON_REGEX;

use super::ast::convertors::expand_shorthand_prop;
use stylex_ast::ast::factories::create_var_declarator;

pub(crate) fn extract_filename_from_path(path: &FileName) -> String {
  match path {
    FileName::Real(path_buf) => {
      let stem = match path_buf.file_stem() {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("File path has no file stem component."),
      };
      match stem.to_str() {
        Some(s) => s.to_string(),
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", INVALID_UTF8),
      }
    },
    _ => "".to_string(),
  }
}

pub(crate) fn extract_path(path: &FileName) -> &str {
  match path {
    FileName::Real(path_buf) => match path_buf.to_str() {
      Some(s) => s,
      #[cfg_attr(coverage_nightly, coverage(off))]
      None => stylex_panic!("{}", INVALID_UTF8),
    },
    _ => "",
  }
}

pub(crate) fn extract_filename_with_ext_from_path(path: &FileName) -> Option<&str> {
  match path {
    FileName::Real(path_buf) => {
      let name = match path_buf.file_name() {
        Some(n) => n,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("File path has no file name component."),
      };
      Some(match name.to_str() {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", INVALID_UTF8),
      })
    },
    _ => None,
  }
}

pub fn reduce_ident_count(state: &mut StateManager, ident: &Ident) {
  if let Entry::Occupied(mut entry) = state.var_decl_count_map.entry(ident.sym.clone()) {
    *entry.get_mut() -= 1;
  }
}

pub fn increase_member_ident(state: &mut StateManager, member_obj: &MemberExpr) {
  if let Some(obj_ident) = member_obj.obj.as_ident() {
    increase_member_ident_count(state, &obj_ident.sym);
  }
}

pub fn reduce_member_expression_count(state: &mut StateManager, member_expression: &MemberExpr) {
  if let Some(obj_ident) = member_expression.obj.as_ident() {
    reduce_member_ident_count(state, &obj_ident.sym);
  }
}

pub fn reduce_member_ident_count(state: &mut StateManager, ident_atom: &Atom) {
  if let Entry::Occupied(mut entry) = state
    .member_object_ident_count_map
    .entry(ident_atom.clone())
  {
    *entry.get_mut() -= 1;
  }
}
pub fn increase_ident_count(state: &mut StateManager, ident: &Ident) {
  increase_ident_count_by_count(state, ident, 1);
}

pub fn increase_member_ident_count(state: &mut StateManager, ident_atom: &Atom) {
  increase_member_ident_count_by_count(state, ident_atom, 1);
}
pub fn increase_ident_count_by_count(state: &mut StateManager, ident: &Ident, count: i16) {
  let ident_id = &ident.sym;

  *state
    .var_decl_count_map
    .entry(ident_id.clone())
    .or_insert(0) += count;
}

pub fn increase_member_ident_count_by_count(
  state: &mut StateManager,
  ident_atom: &Atom,
  count: i16,
) {
  *state
    .member_object_ident_count_map
    .entry(ident_atom.clone())
    .or_insert(0) += count;
}

pub fn get_var_decl_by_ident<'a>(
  ident: &'a Ident,
  traversal_state: &'a mut StateManager,
  functions: &'a FunctionMap,
  action: VarDeclAction,
) -> Option<VarDeclarator> {
  match action {
    VarDeclAction::Increase => increase_ident_count(traversal_state, ident),
    VarDeclAction::Reduce => reduce_ident_count(traversal_state, ident),
    VarDeclAction::None => {},
  };

  if let Some(var_decl) = get_var_decl_from(traversal_state, ident) {
    return Some(var_decl.clone());
  }

  if let Some(func) = functions.identifiers.get(&ident.sym) {
    match func.as_ref() {
      FunctionConfigType::Regular(func) => match &func.fn_ptr {
        FunctionType::Mapper(func) => {
          let result = func();

          let var_decl = create_var_declarator(ident.clone(), result);

          return Some(var_decl);
        },
        #[cfg_attr(coverage_nightly, coverage(off))]
        _ => stylex_panic!("Function type not supported: {:?}", func),
      },
      #[cfg_attr(coverage_nightly, coverage(off))]
      FunctionConfigType::Map(_) => {
        stylex_unimplemented!("Map values are not supported in this context.")
      },
      #[cfg_attr(coverage_nightly, coverage(off))]
      FunctionConfigType::IndexMap(_) => {
        stylex_unimplemented!("IndexMap values are not supported in this context.")
      },
      FunctionConfigType::EnvObject(_) => return None,
    }
  }

  None
}

pub fn get_import_by_ident<'a>(
  ident: &'a Ident,
  state: &'a StateManager,
) -> Option<&'a ImportDecl> {
  get_import_from(state, ident)
}

pub(crate) fn get_var_decl_from<'a>(
  state: &'a StateManager,
  ident: &'a Ident,
) -> Option<&'a VarDeclarator> {
  state
    .declarations
    .iter()
    .find(|var_declarator| matches_ident_with_var_decl_name(ident, var_declarator))
}

fn matches_ident_with_var_decl_name(ident: &Ident, var_declarator: &&VarDeclarator) -> bool {
  var_declarator
    .name
    .clone()
    .ident()
    .is_some_and(|var_decl_ident| &var_decl_ident.id == ident)
}

pub(crate) fn get_import_from<'a>(
  state: &'a StateManager,
  ident: &'a Ident,
) -> Option<&'a ImportDecl> {
  state.top_imports.iter().find(|import| {
    import.specifiers.iter().any(|specifier| match specifier {
      ImportSpecifier::Named(named_import) => {
        named_import.local.sym == ident.sym || {
          match &named_import.imported {
            Some(imported) => match imported {
              ModuleExportName::Ident(export_ident) => export_ident.eq_ignore_span(ident),
              ModuleExportName::Str(strng) => convert_str_lit_to_atom(strng) == ident.sym,
            },
            _ => false,
          }
        }
      },
      ImportSpecifier::Default(default_import) => default_import.local.eq_ignore_span(ident),
      ImportSpecifier::Namespace(namespace_import) => namespace_import.local.eq_ignore_span(ident),
    })
  })
}

pub fn get_expr_from_var_decl(var_decl: &VarDeclarator) -> &Expr {
  match &var_decl.init {
    Some(var_decl_init) => var_decl_init,
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("Variable declaration must be initialized with an expression."),
  }
}

pub fn evaluate_bin_expr(op: BinaryOp, left: f64, right: f64) -> f64 {
  match &op {
    BinaryOp::Add => left + right,
    BinaryOp::Sub => left - right,
    BinaryOp::Mul => left * right,
    BinaryOp::Div => left / right,
    _ => stylex_panic!("Operator '{}' is not supported", op),
  }
}
#[allow(dead_code)]
pub(crate) fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}

fn prop_name_eq(a: &PropName, b: &PropName) -> bool {
  match (a, b) {
    (PropName::Ident(a), PropName::Ident(b)) => a.sym == b.sym,
    (PropName::Str(a), PropName::Str(b)) => a.value == b.value,
    (PropName::Num(a), PropName::Num(b)) => (a.value - b.value).abs() < f64::EPSILON,

    (PropName::BigInt(a), PropName::BigInt(b)) => a.value == b.value,
    // Add more cases as needed
    _ => false,
  }
}

pub(crate) fn remove_duplicates(props: Vec<PropOrSpread>) -> Vec<PropOrSpread> {
  let mut set = FxHashSet::default();
  let mut result = Vec::with_capacity(props.len());

  for prop in props.into_iter().rev() {
    let key = match &prop {
      PropOrSpread::Prop(prop) => match prop.as_ref() {
        Prop::Shorthand(ident) => ident.sym.clone(),
        Prop::KeyValue(key_val) => match &key_val.key {
          PropName::Ident(ident) => ident.sym.clone(),
          PropName::Str(strng) => convert_wtf8_to_atom(&strng.value),
          _ => continue,
        },
        _ => continue,
      },
      _ => continue,
    };

    if set.insert(key) {
      result.push(prop);
    }
  }

  result.reverse();

  result
}

pub(crate) fn deep_merge_props(
  old_props: Vec<PropOrSpread>,
  mut new_props: Vec<PropOrSpread>,
) -> Vec<PropOrSpread> {
  for prop in old_props {
    match prop {
      PropOrSpread::Prop(prop) => match *prop {
        Prop::KeyValue(mut kv) => {
          if new_props.iter().any(|p| match p {
            PropOrSpread::Prop(p) => match p.as_ref() {
              Prop::KeyValue(existing_kv) => prop_name_eq(&kv.key, &existing_kv.key),
              _ => false,
            },
            _ => false,
          }) {
            if let Expr::Object(ref mut obj) = *kv.value {
              new_props.push(PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
                key: kv.key.clone(),
                value: Box::new(Expr::Object(ObjectLit {
                  span: DUMMY_SP,
                  props: obj.props.clone(),
                })),
              }))));
            }
          } else {
            new_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(kv))));
          }
        },
        _ => new_props.push(PropOrSpread::Prop(Box::new(*prop))),
      },
      _ => new_props.push(prop),
    }
  }

  remove_duplicates(new_props.into_iter().rev().collect())
}

pub(crate) fn get_css_value(key_value: KeyValueProp) -> (Box<Expr>, Option<BaseCSSType>) {
  let Some(obj) = key_value.value.as_object() else {
    return (key_value.value, None);
  };

  for prop in obj.props.clone().into_iter() {
    match prop {
      #[cfg_attr(coverage_nightly, coverage(off))]
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
                  #[cfg_attr(coverage_nightly, coverage(off))]
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
                      #[cfg_attr(coverage_nightly, coverage(off))]
                      _ => stylex_unimplemented!("Unsupported prop type in CSS value"),
                    }
                  },
                }

                false
              });

              if let Some(value) = value {
                let result_key_value = match value.as_prop().and_then(|prop| prop.as_key_value()) {
                  Some(kv) => kv,
                  #[cfg_attr(coverage_nightly, coverage(off))]
                  None => stylex_panic!("Expected key-value property"),
                };

                return (result_key_value.value.clone(), Some(obj.clone().into()));
              }
            }
          },
          #[cfg_attr(coverage_nightly, coverage(off))]
          _ => stylex_unimplemented!("Unsupported prop type in CSS value"),
        }
      },
    }
  }

  (key_value.value, None)
}

pub(crate) fn get_key_values_from_object(object: &ObjectLit) -> Vec<KeyValueProp> {
  object
    .props
    .iter()
    .map(|prop| match prop {
      PropOrSpread::Spread(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
      PropOrSpread::Prop(prop) => {
        let mut prop = prop.clone();

        expand_shorthand_prop(&mut prop);

        match prop.as_ref() {
          Prop::KeyValue(key_value) => key_value.clone(),
          _ => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
        }
      },
    })
    .collect()
}

pub fn fill_top_level_expressions(module: &Module, state: &mut StateManager) {
  module.body.iter().for_each(|item| match item {
    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
      if let Decl::Var(decl_var) = &export_decl.decl {
        for decl in &decl_var.decls {
          if let Some(decl_init) = decl.init.as_ref() {
            let ident_sym = match decl.name.as_ident() {
              Some(i) => i.sym.clone(),
              #[cfg_attr(coverage_nightly, coverage(off))]
              None => stylex_panic!("{}", VAR_DECL_NAME_NOT_IDENT),
            };
            state.top_level_expressions.push(TopLevelExpression(
              TopLevelExpressionKind::NamedExport,
              drop_span(decl_init.as_ref().clone()),
              Some(ident_sym),
            ));
            fill_state_declarations(state, decl);
          }
        }
      }
    },
    ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(export_decl)) => {
      match export_decl.expr.as_paren() {
        Some(paren) => {
          state.top_level_expressions.push(TopLevelExpression(
            TopLevelExpressionKind::DefaultExport,
            drop_span(paren.expr.as_ref().clone()),
            None,
          ));
        },
        _ => {
          state.top_level_expressions.push(TopLevelExpression(
            TopLevelExpressionKind::DefaultExport,
            drop_span(export_decl.expr.as_ref().clone()),
            None,
          ));
        },
      }
    },
    ModuleItem::Stmt(Stmt::Decl(Decl::Var(var))) => {
      for decl in &var.decls {
        if let Some(decl_init) = decl.init.as_ref()
          && decl.name.as_ident().is_some()
        {
          let stmt_ident_sym = match decl.name.as_ident() {
            Some(i) => i.sym.clone(),
            #[cfg_attr(coverage_nightly, coverage(off))]
            None => stylex_panic!("{}", VAR_DECL_NAME_NOT_IDENT),
          };
          state.top_level_expressions.push(TopLevelExpression(
            TopLevelExpressionKind::Stmt,
            drop_span(decl_init.as_ref().clone()),
            Some(stmt_ident_sym),
          ));

          fill_state_declarations(state, decl);
        }
      }
    },
    _ => {},
  });
}

pub fn fill_state_declarations(state: &mut StateManager, decl: &VarDeclarator) {
  let normalized_decl = drop_span(decl.clone());

  if !state.declarations.contains(&normalized_decl) {
    state.declarations.push(normalized_decl);
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

pub(crate) fn gen_file_based_identifier(
  file_name: &str,
  export_name: &str,
  key: Option<&str>,
) -> String {
  let key = key.map_or(String::new(), |k| format!(".{}", k));

  format!("{}//{}{}", file_name, export_name, key)
}

#[allow(dead_code)]
pub(crate) fn resolve_node_package_path(package_name: &str) -> Result<PathBuf, String> {
  match node_resolve::Resolver::default()
    .with_basedir(PathBuf::from("./cwd"))
    .preserve_symlinks(true)
    .with_extensions([".ts", ".tsx", ".js", ".jsx", ".json"])
    .with_main_fields(vec![String::from("main"), String::from("module")])
    .resolve(package_name)
  {
    Ok(path) => Ok(path),
    Err(error) => Err(format!(
      "Error resolving package {}: {:?}",
      package_name, error
    )),
  }
}

pub(crate) fn normalize_expr(expr: &mut Expr) -> &mut Expr {
  match expr {
    Expr::Paren(paren) => normalize_expr(paren.expr.as_mut()),
    _ => {
      *expr = drop_span(expr.clone());
      expr
    },
  }
}

pub(crate) fn serialize_value_to_json_string<T: serde::Serialize>(value: T) -> String {
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

            remove_quotes(&inner_string)
          },
          _ => remove_quotes(&json_str),
        }
      } else {
        json_str
      }
    },
    Err(err) => {
      #[cfg_attr(coverage_nightly, coverage(off))]
      {
        stylex_panic!("Failed to serialize value. Error: {}", err)
      }
    },
  }
}

pub(crate) fn js_object_to_json(js_str: &str) -> String {
  JSON_REGEX.replace_all(js_str, r#"$1"$2":"#).to_string()
}

/// Utility function to get the `StateManager` from the `StyleOptions` trait.
/// This is a helper function to get the `StateManager` from the `StyleOptions`
/// trait.
pub(crate) fn downcast_style_options_to_state_manager(
  state: &mut dyn StyleOptions,
) -> &mut StateManager {
  state
    .as_any_mut()
    .downcast_mut::<StateManager>()
    .unwrap_or_else(|| stylex_unreachable!("StyleOptions must be StateManager"))
}
