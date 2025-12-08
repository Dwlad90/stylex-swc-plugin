use std::rc::Rc;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::ecma::ast::CallExpr;
use swc_core::{common::comments::Comments, ecma::ast::Expr};

use crate::shared::{
  constants::{common::COMPILED_KEY, messages::cannot_generate_hash},
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  utils::{
    common::{create_hash, gen_file_based_identifier},
    core::js_to_expr::NestedStringObject,
    validators::{is_define_marker_call, validate_stylex_define_marker_indent},
  },
};
use crate::{StyleXTransform, shared::utils::core::js_to_expr::convert_object_to_ast};

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

    let file_name = self
      .state
      .get_filename_for_hashing(&mut FxHashMap::default())
      .unwrap_or_else(|| panic!("{}", cannot_generate_hash("defineMarker")));

    let export_name = var_name.expect(" var_name must be present for defineMarker");
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
