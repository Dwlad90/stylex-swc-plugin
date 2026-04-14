use std::rc::Rc;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use stylex_macros::stylex_panic;
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::{
  StyleXTransform,
  shared::{
    enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
    utils::{
      common::gen_file_based_identifier,
      core::js_to_expr::{NestedStringObject, convert_object_to_ast},
      validators::{is_define_marker_call, validate_stylex_define_marker_indent},
    },
  },
};
use stylex_constants::constants::{common::COMPILED_KEY, messages::cannot_generate_hash};
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
      #[cfg_attr(coverage_nightly, coverage(off))]
      None => stylex_panic!("{}", cannot_generate_hash("defineMarker")),
    };

    let export_name = match var_name {
      Some(name) => name,
      #[cfg_attr(coverage_nightly, coverage(off))]
      None => stylex_panic!(
        "defineMarker(): The variable name could not be determined. Ensure the call is bound to a named variable."
      ),
    };
    let export_id = gen_file_based_identifier(&file_name, &export_name, None);

    self.state.export_id = Some(export_id.clone());

    let id = format!(
      "{}{}",
      self.state.options.class_name_prefix,
      create_hash(&export_id)
    );

    let mut marker_result = IndexMap::new();

    marker_result.insert(id.clone(), Rc::new(FlatCompiledStylesValue::String(id)));

    marker_result.insert(
      COMPILED_KEY.to_string(),
      Rc::new(FlatCompiledStylesValue::Bool(true)),
    );
    let marker_obj_ast =
      convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(marker_result));

    Some(marker_obj_ast)
  }
}
