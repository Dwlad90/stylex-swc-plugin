use swc_core::{
  common::{comments::Comments, DUMMY_SP},
  ecma::{
    ast::{BindingIdent, Decl, Expr, Ident, Lit, ModuleDecl, ModuleItem, Pat, Stmt, VarDeclarator},
    visit::FoldWith,
  },
};

use crate::{shared::enums::ModuleCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
    match self.cycle {
      ModuleCycle::Skip => module_items,
      ModuleCycle::Initializing => {
        module_items.iter().for_each(|module_item| {
          if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = module_item {
            var_decl.decls.iter().for_each(|decl| {
              if let Pat::Ident(_) = &decl.name {
                let var = decl.clone();

                if !self.state.declarations.contains(&Box::new(var.clone())) {
                  self.state.declarations.push(var);
                }
              }
            });
          }
        });

        let transformed_module_items = module_items.clone().fold_children_with(self);

        if self.state.import_paths.is_empty() {
          self.cycle = ModuleCycle::Skip;

          return module_items;
        }

        transformed_module_items
      }
      ModuleCycle::TransformEnter => module_items.fold_children_with(self),
      ModuleCycle::TransformExit => module_items.fold_children_with(self),
      ModuleCycle::PreCleaning => module_items.fold_children_with(self),
      ModuleCycle::InjectStyles => {
        let mut result_module_items: Vec<ModuleItem> =
          self.state.prepend_include_module_items.clone();

        result_module_items.extend(self.state.prepend_import_module_items.clone());

        let mut items_to_skip: usize = 0;

        if let Some(first) = module_items
          .first()
          .and_then(|first| first.as_stmt())
          .and_then(|stmp| stmp.as_expr())
        {
          if let Some(Lit::Str(_)) = first.expr.as_lit() {
            result_module_items.insert(0, module_items.first().unwrap().clone());
            items_to_skip = 1;
          }
        }

        for module_item in module_items.iter().skip(items_to_skip) {
          if let Some(decls) = match module_item.clone() {
            ModuleItem::ModuleDecl(decl) => match decl {
              ModuleDecl::ExportDecl(export_decl) => export_decl.decl.var().map(|var_decl| {
                var_decl
                  .decls
                  .clone()
                  .into_iter()
                  .filter(|decl| {
                    let init = decl.init.clone().unwrap();

                    init.is_object() || init.is_lit()
                  })
                  .collect::<Vec<VarDeclarator>>()
              }),
              ModuleDecl::ExportDefaultExpr(export_default_expr) => {
                export_default_expr.expr.object().map(|obj| {
                  vec![VarDeclarator {
                    definite: true,
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                      id: Ident::new("default".into(), DUMMY_SP),
                      type_ann: None,
                    }),
                    init: Option::Some(Box::new(Expr::Object(obj))),
                  }]
                })
              }
              _ => Option::None,
            },
            ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => Option::Some(
              var_decl
                .decls
                .clone()
                .into_iter()
                .filter(|decl| {
                  let init = decl.init.clone().unwrap();

                  init.is_object() || init.is_lit()
                })
                .collect::<Vec<VarDeclarator>>(),
            ),
            _ => Option::None,
          } {
            for decl in decls {
              let key = decl.clone().init.clone().unwrap();

              if let Some(metadata_items) = self.state.styles_to_inject.get(key.as_ref()) {
                for module_item in metadata_items.iter() {
                  result_module_items.push(module_item.clone());
                }
              }
            }
          }
          result_module_items.push(module_item.clone());
        }

        result_module_items
      }
      ModuleCycle::Cleaning => {
        // We need it twice for a clear dead code after declaration transforms
        let mut module_items = module_items.clone().fold_children_with(self);

        // We remove `Stmt::Empty` from the statement list.
        // This is optional, but it's required if you don't want extra `;` in output.
        module_items.retain(|module_item| {
          !matches!(module_item, ModuleItem::Stmt(Stmt::Empty(..)))
            && module_item
              .clone()
              .module_decl()
              .and_then(|module_decl| match module_decl {
                ModuleDecl::ExportDecl(import_decl) => import_decl.decl.var(),
                _ => None,
              })
              .filter(|var| var.decls.is_empty())
              .is_none()
        });

        module_items
      }
    }
  }
}
