use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{Pat, VarDeclarator},
    visit::VisitMutWith,
  },
};

use crate::StyleXTransform;
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_var_declarators_impl(
    &mut self,
    var_declarators: &mut Vec<VarDeclarator>,
  ) {
    match self.state.cycle {
      TransformationCycle::Skip => return,
      TransformationCycle::Cleaning => {
        var_declarators.retain_mut(|decl| {
          if let Pat::Ident(bind_ident) = &decl.name {
            let decl_id = &bind_ident.sym;

            if let Some(&count) = self.state.var_decl_count_map.get(decl_id) {
              // Remove the variable declaration if it is used only once after transformation.
              let is_used = count > 1
                || self
                  .state
                  .style_vars_to_keep
                  .iter()
                  .any(|style_var| &style_var.0 == decl_id);

              if !is_used {
                self.state.cycle = TransformationCycle::Recounting;
                decl.visit_mut_children_with(self);

                self.state.cycle = TransformationCycle::Cleaning;
              }

              return is_used;
            }
          }

          true
        });
      },
      _ => {},
    };

    var_declarators.visit_mut_children_with(self);
  }
}
