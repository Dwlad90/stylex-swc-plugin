use std::rc::Rc;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use stylex_macros::stylex_panic;
use swc_core::{
  common::{EqIgnoreSpan, comments::Comments},
  ecma::ast::{CallExpr, Expr},
};

use crate::{
  StyleXTransform,
  shared::{
    enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
    utils::{
      common::gen_file_based_identifier,
      core::js_to_ast::{NestedStringObject, convert_object_to_ast},
      validators::{is_define_marker_call, validate_stylex_define_marker_indent},
    },
  },
};
use stylex_constants::constants::{
  api_names::STYLEX_DEFINE_MARKER, common::COMPILED_KEY, messages::cannot_generate_hash,
};
use stylex_utils::hash::create_hash;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_define_marker_call(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_define_marker_call = is_define_marker_call(call, &self.state);

    if is_define_marker_call {
      validate_stylex_define_marker_indent(call, &mut self.state);
    } else {
      return None;
    }

    let (var_name, parent_var_decl) = self.get_call_var_name(call);

    let parent_var_decl = parent_var_decl?;

    parent_var_decl.name.as_ident()?;

    let file_name = match self
      .state
      .get_filename_for_hashing(&mut FxHashMap::default())
    {
      Some(name) => name,
      None => stylex_panic!("{}", cannot_generate_hash(STYLEX_DEFINE_MARKER)),
    };

    let export_name = match var_name {
      Some(name) => name,
      None => stylex_panic!(
        "defineMarker(): The variable name could not be determined. Ensure the call is bound to a named variable."
      ),
    };
    let export_id = gen_file_based_identifier(&file_name, &export_name, None);

    self.state.export_id = Some(export_id.clone());

    let hash = create_hash(&export_id);
    let mut id = String::with_capacity(self.state.options.class_name_prefix.len() + hash.len());
    id.push_str(&self.state.options.class_name_prefix);
    id.push_str(&hash);

    let mut marker_result = IndexMap::new();

    marker_result.insert(id.clone(), Rc::new(FlatCompiledStylesValue::String(id)));

    marker_result.insert(
      COMPILED_KEY.to_string(),
      Rc::new(FlatCompiledStylesValue::Bool(true)),
    );
    let marker_obj_ast =
      convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(marker_result));

    // The recorded declaration still holds the `defineMarker()` call this
    // returns a marker object in place of. A `when` selector in the same file
    // resolves the marker through that declaration, so it has to see the
    // object rather than the call it can no longer evaluate.
    //
    // Matched on the call rather than on `export_name`, as
    // `StateManager::update_references` does for the other transforms: it
    // pins the one declaration this call initialises instead of the first
    // one that happens to share its name, and it makes a second pass a no-op
    // once the call has been replaced.
    if let Some(declaration) = self.state.declarations.iter_mut().find(|declaration| {
      declaration.init.as_ref().is_some_and(|init| {
        matches!(**init, Expr::Call(ref existing_call) if existing_call.eq_ignore_span(call))
      })
    }) {
      declaration.init = Some(Box::new(marker_obj_ast.clone()));
    }

    Some(marker_obj_ast)
  }
}
