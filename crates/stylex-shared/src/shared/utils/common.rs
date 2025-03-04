use radix_fmt::radix;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
  any::type_name,
  collections::hash_map::Entry,
  hash::{DefaultHasher, Hash, Hasher},
  ops::Deref,
  path::PathBuf,
};
use swc_core::{
  atoms::Atom,
  common::{FileName, DUMMY_SP},
  ecma::ast::{
    BinaryOp, Decl, Expr, Ident, ImportDecl, ImportSpecifier, KeyValueProp, MemberExpr, Module,
    ModuleDecl, ModuleExportName, ModuleItem, ObjectLit, Pat, Prop, PropName, PropOrSpread, Stmt,
    VarDeclarator,
  },
};

use crate::shared::{
  constants::messages::ILLEGAL_PROP_VALUE,
  enums::{
    data_structures::top_level_expression::{TopLevelExpression, TopLevelExpressionKind},
    misc::VarDeclAction,
  },
  regex::DASHIFY_REGEX,
  structures::{
    base_css_type::BaseCSSType,
    functions::{FunctionConfigType, FunctionMap, FunctionType},
    state_manager::StateManager,
  },
};

use super::ast::{convertors::transform_shorthand_to_key_values, factories::binding_ident_factory};

pub(crate) fn extract_filename_from_path(path: &FileName) -> String {
  match path {
    FileName::Real(path_buf) => path_buf.file_stem().unwrap().to_str().unwrap().to_string(),
    _ => "UnknownFile".to_string(),
  }
}

pub(crate) fn extract_path(path: &FileName) -> &str {
  match path {
    FileName::Real(path_buf) => path_buf.to_str().unwrap(),
    _ => "UnknownFile",
  }
}

pub(crate) fn extract_filename_with_ext_from_path(path: &FileName) -> Option<&str> {
  match path {
    FileName::Real(path_buf) => Some(path_buf.file_name().unwrap().to_str().unwrap()),
    _ => None,
  }
}

pub fn create_hash(value: &str) -> String {
  radix(murmur2::murmur2(value.as_bytes(), 1), 36).to_string()
}

pub(crate) fn wrap_key_in_quotes(key: &str, should_wrap_in_quotes: bool) -> String {
  if should_wrap_in_quotes {
    format!("\"{}\"", key)
  } else {
    key.to_string()
  }
}

pub fn reduce_ident_count<'a>(state: &'a mut StateManager, ident: &'a Ident) {
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
    VarDeclAction::None => {}
  };

  if let Some(var_decl) = get_var_decl_from(traversal_state, ident) {
    return Some(var_decl.clone());
  }

  if let Some(func) = functions.identifiers.get(&ident.sym) {
    match func.as_ref() {
      FunctionConfigType::Regular(func) => match &func.fn_ptr {
        FunctionType::Mapper(func) => {
          let result = func();

          let var_decl = VarDeclarator {
            span: DUMMY_SP,
            name: Pat::from(binding_ident_factory(ident.clone())),
            init: Some(Box::new(result)),
            definite: false,
          };

          return Some(var_decl);
        }
        _ => panic!("Function type not supported"),
      },
      FunctionConfigType::Map(_) => unimplemented!(),
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
  state.declarations.iter().find(|var_declarator| {
    if let Pat::Ident(binding_ident) = &var_declarator.name {
      return binding_ident.sym == ident.sym;
    }

    false
  })
}

pub(crate) fn get_import_from<'a>(
  state: &'a StateManager,
  ident: &'a Ident,
) -> Option<&'a ImportDecl> {
  state.top_imports.iter().find(|import| {
    import.specifiers.iter().any(|specifier| match specifier {
      ImportSpecifier::Named(named_import) => {
        named_import.local.sym == ident.sym || {
          if let Some(imported) = &named_import.imported {
            match imported {
              ModuleExportName::Ident(export_ident) => export_ident.sym == ident.sym,
              ModuleExportName::Str(strng) => strng.value == ident.sym,
            }
          } else {
            false
          }
        }
      }
      ImportSpecifier::Default(default_import) => default_import.local.sym == ident.sym,
      ImportSpecifier::Namespace(namespace_import) => namespace_import.local.sym == ident.sym,
    })
  })
}

pub(crate) fn _get_var_decl_by_ident_or_member<'a>(
  state: &'a StateManager,
  ident: &'a Ident,
) -> Option<VarDeclarator> {
  state
    .declarations
    .iter()
    .find(|var_declarator| {
      if let Pat::Ident(binding_indent) = &var_declarator.name {
        if binding_indent.sym == ident.sym {
          return true;
        }
      }

      var_declarator
        .init
        .as_ref()
        .and_then(|init| init.as_call())
        .and_then(|call| call.callee.as_expr())
        .and_then(|callee| callee.as_member())
        .and_then(|member| member.prop.as_ident())
        .is_some_and(|member_ident| member_ident.sym == ident.sym)
    })
    .cloned()
}

pub fn get_expr_from_var_decl(var_decl: &VarDeclarator) -> &Expr {
  match &var_decl.init {
    Some(var_decl_init) => var_decl_init,
    None => panic!("Variable declaration is not an expression"),
  }
}

pub fn evaluate_bin_expr(op: BinaryOp, left: f64, right: f64) -> f64 {
  match &op {
    BinaryOp::Add => left + right,
    BinaryOp::Sub => left - right,
    BinaryOp::Mul => left * right,
    BinaryOp::Div => left / right,
    _ => panic!("Operator '{}' is not supported", op),
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
  let mut result = vec![];

  for prop in props.into_iter().rev() {
    let key = match &prop {
      PropOrSpread::Prop(prop) => match prop.as_ref() {
        Prop::Shorthand(ident) => ident.sym.clone(),
        Prop::KeyValue(key_val) => match &key_val.key {
          PropName::Ident(ident) => ident.sym.clone(),
          PropName::Str(strng) => strng.value.clone(),
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
              Prop::KeyValue(ref existing_kv) => prop_name_eq(&kv.key, &existing_kv.key),
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
        }
        _ => new_props.push(PropOrSpread::Prop(Box::new(*prop))),
      },
      _ => new_props.push(prop),
    }
  }

  remove_duplicates(new_props.into_iter().rev().collect())
}

pub(crate) fn get_hash_map_difference<K, V>(
  orig_map: &FxHashMap<K, V>,
  compare_map: &FxHashMap<K, V>,
) -> FxHashMap<K, V>
where
  K: Eq + Hash + Clone,
  V: PartialEq + Clone,
{
  let mut diff = FxHashMap::default();

  for (key, value) in orig_map {
    if let Some(map2_value) = compare_map.get(key) {
      if value != map2_value {
        diff.insert(key.clone(), value.clone());
      }
    } else {
      diff.insert(key.clone(), value.clone());
    }
  }

  for (key, value) in compare_map {
    if !orig_map.contains_key(key) {
      diff.insert(key.clone(), value.clone());
    }
  }

  diff
}

pub(crate) fn get_hash_map_value_difference(
  orig_map: &FxHashMap<Atom, i16>,
  map2: &FxHashMap<Atom, i16>,
) -> FxHashMap<Atom, i16> {
  let mut diff = FxHashMap::default();

  for (key, value) in orig_map {
    if let Some(map2_value) = map2.get(key) {
      if value != map2_value {
        diff.insert(key.clone(), value - map2_value);
      }
    } else {
      diff.insert(key.clone(), *value);
    }
  }

  diff
}

pub(crate) fn sum_hash_map_values(
  orig_map: &FxHashMap<Atom, i16>,
  compare_map: &FxHashMap<Atom, i16>,
) -> FxHashMap<Atom, i16> {
  let mut sum_map = FxHashMap::default();

  for (key, value) in orig_map {
    sum_map.insert(key.clone(), *value);
  }

  for (key, value) in compare_map {
    sum_map
      .entry(key.clone())
      .and_modify(|e| *e += value)
      .or_insert(*value);
  }

  sum_map
}

pub(crate) fn dashify(s: &str) -> String {
  DASHIFY_REGEX.replace_all(s, "$1-$2").to_lowercase()
}

pub(crate) fn get_css_value(key_value: KeyValueProp) -> (Box<Expr>, Option<BaseCSSType>) {
  let Some(obj) = key_value.value.as_object() else {
    return (key_value.value, None);
  };

  for prop in obj.props.clone().into_iter() {
    match prop {
      PropOrSpread::Spread(_) => unimplemented!("Spread"),
      PropOrSpread::Prop(mut prop) => {
        transform_shorthand_to_key_values(&mut prop);

        match prop.deref() {
          Prop::KeyValue(key_value) => {
            if let Some(ident) = key_value.key.as_ident() {
              if ident.sym == "syntax" {
                let value = obj.props.iter().find(|prop| {
                  match prop {
                    PropOrSpread::Spread(_) => unimplemented!("Spread"),
                    PropOrSpread::Prop(prop) => {
                      let mut prop = prop.clone();
                      transform_shorthand_to_key_values(&mut prop);

                      match prop.as_ref() {
                        Prop::KeyValue(key_value) => {
                          if let Some(ident) = key_value.key.as_ident() {
                            return ident.sym == "value";
                          }
                        }
                        _ => unimplemented!(),
                      }
                    }
                  }

                  false
                });

                if let Some(value) = value {
                  let result_key_value = value
                    .as_prop()
                    .and_then(|prop| prop.as_key_value())
                    .unwrap();

                  return (result_key_value.value.clone(), Some(obj.clone().into()));
                }
              }
            }
          }
          _ => unimplemented!(),
        }
      }
    }
  }

  (key_value.value, None)
}

pub(crate) fn get_key_values_from_object(object: &ObjectLit) -> Vec<KeyValueProp> {
  let mut key_values = vec![];

  for prop in object.props.iter() {
    match prop {
      PropOrSpread::Spread(_) => unimplemented!("Spread"),
      PropOrSpread::Prop(prop) => {
        let mut prop = prop.clone();

        transform_shorthand_to_key_values(&mut prop);

        match prop.as_ref() {
          Prop::KeyValue(key_value) => {
            key_values.push(key_value.clone());
          }
          _ => panic!("{}", ILLEGAL_PROP_VALUE),
        }
      }
    }
  }
  key_values
}

pub(crate) fn fill_top_level_expressions(module: &Module, state: &mut StateManager) {
  module.clone().body.iter().for_each(|item| match &item {
    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
      if let Decl::Var(decl_var) = &export_decl.decl {
        for decl in &decl_var.decls {
          if let Some(decl_init) = decl.init.as_ref() {
            state.top_level_expressions.push(TopLevelExpression(
              TopLevelExpressionKind::NamedExport,
              *decl_init.clone(),
              Some(decl.name.as_ident().unwrap().sym.clone()),
            ));
            if !state.declarations.contains(&Box::new(&decl)) {
              state.declarations.push(decl.clone());
            }
          }
        }
      }
    }
    ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(export_decl)) => {
      if let Some(paren) = export_decl.expr.as_paren() {
        state.top_level_expressions.push(TopLevelExpression(
          TopLevelExpressionKind::DefaultExport,
          *paren.expr.clone(),
          None,
        ));
      } else {
        state.top_level_expressions.push(TopLevelExpression(
          TopLevelExpressionKind::DefaultExport,
          *export_decl.expr.clone(),
          None,
        ));
      }
    }
    ModuleItem::Stmt(Stmt::Decl(Decl::Var(var))) => {
      for decl in &var.decls {
        if let Some(decl_init) = decl.init.as_ref() {
          if decl.name.as_ident().is_some() {
            state.top_level_expressions.push(TopLevelExpression(
              TopLevelExpressionKind::Stmt,
              *decl_init.clone(),
              Some(decl.name.as_ident().unwrap().sym.clone()),
            ));

            if !state.declarations.contains(&Box::new(&decl)) {
              state.declarations.push(decl.clone());
            }
          }
        }
      }
    }
    _ => {}
  });
}

pub(crate) fn gen_file_based_identifier(
  file_name: &str,
  export_name: &str,
  key: Option<&str>,
) -> String {
  let key = key.map_or(String::new(), |k| format!(".{}", k));

  format!("{}//{}{}", file_name, export_name, key)
}

pub(crate) fn hash_f64(value: f64) -> u64 {
  let bits = value.to_bits();
  let mut hasher = DefaultHasher::new();
  bits.hash(&mut hasher);
  hasher.finish()
}

pub(crate) fn round_f64(value: f64, decimal_places: u32) -> f64 {
  let multiplier = 10f64.powi(decimal_places as i32);
  (value * multiplier).round() / multiplier
}

pub(crate) fn _resolve_node_package_path(package_name: &str) -> Result<PathBuf, String> {
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
    _ => expr,
  }
}

pub(crate) fn sort_numbers_factory() -> impl FnMut(&f64, &f64) -> std::cmp::Ordering {
  |a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
}

pub(crate) fn char_code_at(s: &str, index: usize) -> Option<u32> {
  s.chars().nth(index).map(|c| c as u32)
}

pub(crate) fn stable_hash<T: Hash>(t: &T) -> u64 {
  let mut hasher = DefaultHasher::new();
  t.hash(&mut hasher);
  hasher.finish()
}

pub(crate) fn find_and_swap_remove<T, F>(vec: &mut Vec<T>, predicate: F) -> Option<T>
where
  F: Fn(&T) -> bool,
{
  vec
    .iter()
    .position(predicate)
    .map(|index| vec.swap_remove(index))
}
