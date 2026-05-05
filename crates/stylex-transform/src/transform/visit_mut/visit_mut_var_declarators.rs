use rustc_hash::FxHashSet;
use swc_core::{
  atoms::Atom,
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
    if self.state.cycle == TransformationCycle::Finalize {
      // Project the live-set down to bare `Atom`s. The original
      // count-based cleanup keyed reference counts on `Atom` only, so
      // shadowed-but-unused top-level decls (an outer `const x = …` with
      // an inner `const x = …` inside a function) survived if *any*
      // binding sharing the atom had references. The graph-based pass
      // tracks `(Atom, SyntaxContext)` precisely; this projection
      // restores the legacy "keep if any same-name binding is live"
      // semantics so the snapshot suite stays byte-identical.
      let live_atoms: FxHashSet<Atom> =
        self.state.live_set.iter().map(|(sym, _)| sym.clone()).collect();

      var_declarators.retain(|decl| {
        if let Pat::Ident(bind_ident) = &decl.name {
          let decl_id = bind_ident.id.to_id();

          // Drop only declarators that the graph captured (`decl_uses`
          // is the observable-set) but whose name does not appear in
          // any live binding. Anything outside the graph — non
          // `Pat::Ident` patterns, helpers introduced after graph
          // capture, etc. — falls through to the "keep by default"
          // fallback.
          if self.state.decl_uses.contains_key(&decl_id) {
            return live_atoms.contains(&decl_id.0);
          }
        }

        true
      });
    }

    var_declarators.visit_mut_children_with(self);
  }
}
