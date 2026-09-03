//! Which declaration binds a name.
//!
//! Four readers over the state's own indices. They answer the first steps of a
//! reference resolution: the declarator a name is bound by, the import that
//! declared it, and the two parts of a declarator a caller actually reads.

use stylex_ast::ast::factories::create_var_declarator;
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::{
  common::Span,
  ecma::ast::{Expr, Ident, ImportDecl, ImportSpecifier, VarDeclarator},
};

use crate::{
  functions::{FunctionConfigType, FunctionMap, FunctionType},
  state_manager::StateManager,
};

/// The two parts of a declarator the reference chain actually reads.
///
/// Step 5 asks only for the span, which is `Copy`, and step 8 for the
/// initializer. Answering either by cloning the whole declarator also copies its
/// name pattern and type annotation, and does so on the path a real module takes
/// -- the state hit -- where a borrow is available and only the initializer has
/// to be owned.
///
/// Falls through to [`get_var_decl_by_ident`] for the synthesized declarators
/// the function map builds, rather than repeating its arms: that path has to
/// construct one anyway, so there is nothing to save there.
pub fn get_var_decl_parts_by_ident(
  ident: &Ident,
  traversal_state: &mut StateManager,
  functions: &FunctionMap,
) -> Option<(Span, Option<Box<Expr>>)> {
  if let Some(declarator) = get_var_decl_from(traversal_state, ident) {
    return Some((declarator.span, declarator.init.clone()));
  }

  get_var_decl_by_ident(ident, traversal_state, functions)
    .map(|declarator| (declarator.span, declarator.init))
}

pub fn get_var_decl_by_ident<'a>(
  ident: &'a Ident,
  traversal_state: &'a mut StateManager,
  functions: &'a FunctionMap,
) -> Option<VarDeclarator> {
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
        _ => stylex_panic!("Function type not supported: {:?}", func),
      },
      FunctionConfigType::Map(_) => {
        stylex_unimplemented!("Map values are not supported in this context.")
      },
      FunctionConfigType::IndexMap(_) => {
        stylex_unimplemented!("IndexMap values are not supported in this context.")
      },
      FunctionConfigType::EnvObject(_) => return None,
    }
  }

  None
}

/// The import declaration and the specifier that bind `ident`, or `None` where
/// no import binds it.
///
/// Asked of the state, which indexes the bindings its imports declare -- see
/// [`StateManager::import_binding`] for what the lookup answers and why it is
/// the binding rather than the name.
pub fn get_import_by_ident<'a>(
  ident: &Ident,
  state: &'a StateManager,
) -> Option<(&'a ImportDecl, &'a ImportSpecifier)> {
  state.import_binding(ident)
}

pub fn get_var_decl_from<'a>(
  state: &'a StateManager,
  ident: &'a Ident,
) -> Option<&'a VarDeclarator> {
  // One hash probe rather than a scan of every declarator the module holds. The
  // `Vec` keeps its source order, which `find_top_level_expr` and the insertion
  // queue both read; the index only says where in it to look.
  state.declaration_of(ident)
}
