use swc_core::{
  common::{Spanned, comments::Comments, util::take::Take},
  ecma::{
    ast::{Decl, Stmt},
    utils::IsDirective,
    visit::VisitMutWith,
  },
};

use crate::StyleXTransform;
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_stmt_impl(&mut self, stmt: &mut Stmt) {
    let previous_site_span = self.state.current_site_span;
    if self.state.cycle == TransformationCycle::Discover {
      let span = stmt.span();
      if !span.is_dummy() {
        self.state.current_site_span = span;
      }
    }

    if self.state.cycle == TransformationCycle::Finalize {
      stmt.visit_mut_children_with(self);

      if let Some(Stmt::Decl(Decl::Var(var))) = stmt.as_ref()
        && var.decls.is_empty()
      {
        // Variable declaration without declarator is invalid.
        //
        // After this, `stmt` becomes `Stmt::Empty`.
        stmt.take();
      }
    } else {
      stmt.visit_mut_children_with(self);
    }

    if self.state.cycle == TransformationCycle::Discover {
      self.state.current_site_span = previous_site_span;
    }
  }
}
