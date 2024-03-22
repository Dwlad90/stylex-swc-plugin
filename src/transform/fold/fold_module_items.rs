use colored::{Color, Colorize};
use swc_core::{
    common::{comments::Comments, plugin::metadata, DUMMY_SP},
    ecma::{
        ast::{
            BindingIdent, CallExpr, Callee, Decl, Expr, ExprStmt, Id, Ident, ImportDecl,
            ImportDefaultSpecifier, ImportPhase, ImportSpecifier, ModuleDecl, ModuleItem, Pat,
            Stmt, Str, VarDecl, VarDeclKind, VarDeclarator,
        },
        visit::FoldWith,
    },
};

use crate::{
    shared::{
        constants::constants::DEFAULT_INJECT_PATH,
        enums::ModuleCycle,
        structures::uid_generator::UidGenerator,
        utils::common::{
            expr_or_spread_number_expression_creator, expr_or_spread_string_expression_creator,
            get_pat_as_string, increase_ident_count,
        },
    },
    ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
        match self.cycle {
            ModuleCycle::Skip => module_items,
            ModuleCycle::Initializing => {
                module_items
                    .iter()
                    .for_each(|module_item| match module_item {
                        ModuleItem::Stmt(stmp) => match stmp {
                            Stmt::Decl(decl) => match decl {
                                Decl::Var(var_decl) => {
                                    var_decl.decls.iter().for_each(|decl| {
                                        if let Pat::Ident(_) = &decl.name {
                                            let var = decl.clone();

                                            if !self.state.declarations.contains(&var) {
                                                self.state.declarations.push(var);
                                            }
                                        }
                                    });
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    });

                let module_items = module_items.fold_children_with(self);

                if self.state.stylex_create_import.len() == 0 {
                    self.cycle = ModuleCycle::Skip;

                    return module_items;
                }

                // TODO: Inject comment to css extraction

                module_items
            }
            ModuleCycle::TransformEnter => module_items.fold_children_with(self),
            ModuleCycle::TransformExit => module_items.fold_children_with(self),
            ModuleCycle::InjectClassName => module_items.fold_children_with(self),
            ModuleCycle::InjectStyles => {
                let mut styles_item_idx = 0;
                let mut styles_item_target_idx: Vec<(Option<String>, usize)> = vec![];

                let prefix = "inject";

                let uid_generator_inject = UidGenerator::new(prefix);

                let inject_module_ident = uid_generator_inject.generate_ident();
                let inject_var_ident = uid_generator_inject.generate_ident();

                for module_item in module_items.clone().into_iter() {
                    match &module_item {
                        ModuleItem::ModuleDecl(import_decl) => match &import_decl {
                            ModuleDecl::ExportDecl(export_decl) => match &export_decl.decl {
                                Decl::Var(var_decl) => {
                                    for var_declarator in &var_decl.decls {
                                        if let Option::Some(ident) =
                                            match &var_declarator.name.clone() {
                                                Pat::Ident(ident) => Option::Some(ident),
                                                _ => Option::None,
                                            }
                                        {
                                            // HACK: Prevent removing named export variables
                                            increase_ident_count(&mut self.state, &ident);
                                        }

                                        let decl_names =
                                            self.state.style_vars.keys().collect::<Vec<&String>>();
                                        let var_declarator_name =
                                            get_pat_as_string(&var_declarator.name);

                                        println!(
                                            "!!!! decl_names: {:?}, var_declarator_name: {:?}, self
                                            .props_declaration: {:?}",
                                            decl_names, var_declarator_name, self.props_declaration
                                        );

                                        if decl_names.contains(&&var_declarator_name) {
                                            styles_item_target_idx
                                                .push((Some(var_declarator_name), styles_item_idx));
                                        }
                                    }
                                }
                                _ => {}
                            },
                            ModuleDecl::ExportDefaultExpr(export_default_expr) => {
                                match export_default_expr.expr.as_ref() {
                                    Expr::Object(_) => {
                                        if self.state.style_vars.is_empty() {
                                            styles_item_target_idx
                                                .push((Option::None, styles_item_idx));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        },
                        ModuleItem::Stmt(stmt) => match &stmt {
                            Stmt::Decl(expr) => match expr {
                                Decl::Var(decl_var) => {
                                    for decl in &decl_var.decls {
                                        let decl_names =
                                            self.state.style_vars.keys().collect::<Vec<&String>>();
                                        let var_declarator_name = get_pat_as_string(&decl.name);

                                        println!(
                                            "!!!! decl_names: {:?}, var_declarator_name: {:?}, self
                                        .props_declaration: {:?}",
                                            decl_names,
                                            var_declarator_name,
                                            self.state.style_vars.keys()
                                        );

                                        if decl_names.contains(&&var_declarator_name) {
                                            styles_item_target_idx
                                                .push((Some(var_declarator_name), styles_item_idx));
                                        }
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                    }

                    styles_item_idx += 1;
                }

                let module_items: Vec<ModuleItem> = if !styles_item_target_idx.is_empty() {
                    dbg!(styles_item_target_idx.clone());
                    let (first_id, first) = styles_item_target_idx.first().unwrap().clone();
                    let (last_id, last) = styles_item_target_idx.last().unwrap().clone();
                    let module_item_start_slice = &module_items[0..first];
                    let module_items_end_slice = &module_items[last + 1..];
                    let mut module_item_pre_start_vec: Vec<ModuleItem> = vec![];

                    if !self.css_output.is_empty() {
                        add_inject_import_expression(
                            &mut module_item_pre_start_vec,
                            &inject_module_ident,
                        );

                        add_inject_var_decl_expression(
                            &mut module_item_pre_start_vec,
                            &inject_var_ident,
                            &inject_module_ident,
                        );
                    }
                    let mut module_items_middle_vec: Vec<ModuleItem> = vec![];

                    let mut metadata_index = 0;
                    dbg!(&self.css_output, first..=last);

                    for index in first..=last {
                        styles_item_target_idx.retain(|(style_var_name, i)| {
                            if i == &index {
                                if let Some(metadata_items) = self
                                    .css_output
                                    .get(&style_var_name.clone().unwrap_or("default".to_string()))
                                {
                                    for metadata in metadata_items.iter() {
                                        dbg!(&index, &metadata_index, &metadata, style_var_name);

                                        eprintln!(
                                            "{}",
                                            Colorize::yellow(
                                                "!!!! registerStyles: not implemented !!!!"
                                            )
                                        );

                                        let priority = &metadata.get_priority();
                                        let css = &metadata.get_css();

                                        let stylex_inject_args = vec![
                                            expr_or_spread_string_expression_creator(css.clone()),
                                            expr_or_spread_number_expression_creator(f64::from(
                                                **priority,
                                            )),
                                        ];

                                        let _inject = Expr::Ident(inject_var_ident.clone());

                                        let stylex_call_expr = CallExpr {
                                            span: DUMMY_SP,
                                            type_args: Option::None,
                                            callee: Callee::Expr(Box::new(_inject.clone())),
                                            args: stylex_inject_args,
                                        };

                                        let stylex_call = Expr::Call(stylex_call_expr);

                                        let module = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                                            span: DUMMY_SP,
                                            expr: Box::new(stylex_call),
                                        }));

                                        module_items_middle_vec.push(module);
                                        metadata_index += 1;
                                    }
                                    return false;
                                }
                            };

                            true
                        });

                        module_items_middle_vec.push(module_items[index].clone());
                    }

                    let mut module_items: Vec<ModuleItem> = vec![];

                    module_items.extend_from_slice(&&module_item_pre_start_vec[..]);
                    module_items.extend_from_slice(&module_item_start_slice);
                    module_items.extend_from_slice(&module_items_middle_vec[..]);
                    module_items.extend_from_slice(&module_items_end_slice);

                    module_items
                } else {
                    module_items
                };

                module_items.fold_children_with(self)
            }
            ModuleCycle::Cleaning => {
                // We need it twice for a clear dead code after declaration transforms
                let mut module_items = module_items.clone().fold_children_with(self);

                // We remove `Stmt::Empty` from the statement list.
                // This is optional, but it's required if you don't want extra `;` in output.
                module_items.retain(|s| !matches!(s, ModuleItem::Stmt(Stmt::Empty(..))));

                module_items
            }
        }
    }
}

fn add_inject_import_expression(module_item_pre_start_vec: &mut Vec<ModuleItem>, ident: &Ident) {
    let inject_import_stmt = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
            span: DUMMY_SP,
            local: ident.clone(),
        })],
        src: Box::new(Str {
            span: DUMMY_SP,
            raw: Option::None,
            value: DEFAULT_INJECT_PATH.into(),
        }),
        type_only: false,
        with: Option::None,
        phase: ImportPhase::Evaluation,
    }));
    module_item_pre_start_vec.push(inject_import_stmt);
}

fn add_inject_var_decl_expression(
    module_item_pre_start_vec: &mut Vec<ModuleItem>,
    decl_ident: &Ident,
    value_ident: &Ident,
) {
    let inject_import_stmt = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        declare: false,
        decls: vec![VarDeclarator {
            definite: true,
            span: DUMMY_SP,
            name: Pat::Ident(BindingIdent {
                id: decl_ident.clone(),
                type_ann: None,
            }),
            init: Option::Some(Box::new(Expr::Ident(value_ident.clone()))),
        }],
        kind: VarDeclKind::Var,
        span: DUMMY_SP,
    }))));
    module_item_pre_start_vec.push(inject_import_stmt);
}
