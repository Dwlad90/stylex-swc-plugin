use std::{
    collections::HashMap,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

use swc_core::{
    common::{comments::Comments, FileName, Mark, DUMMY_SP},
    ecma::ast::{CallExpr, Callee, Expr, Id, Ident, MemberProp, VarDeclarator},
};

use crate::{
    shared::{
        enums::ModuleCycle,
        structures::{meta_data::MetaData, uid_generator::UidGenerator},
        utils::common::{extract_filename_from_path, increase_ident_count},
    },
    StylexConfig, StylexConfigParams,
};

mod css_map;
mod fold;

pub struct ModuleTransformVisitor<C>
where
    C: Comments,
{
    comments: C,
    declaration: Option<Id>,
    cycle: ModuleCycle,
    package_name: String,
    file_name: String,
    props_declaration: Option<Id>,
    css_output: Vec<MetaData>,
    config: StylexConfig,
    declarations: Vec<VarDeclarator>,
    var_decl_count_map: HashMap<Id, i8>,
}

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn new(comments: C, file_name: FileName, config: StylexConfigParams) -> Self {
        ModuleTransformVisitor {
            comments,
            declaration: Option::None,
            cycle: ModuleCycle::Initializing,
            package_name: "@stylexjs/stylex".to_string(),
            file_name: extract_filename_from_path(file_name),
            props_declaration: Option::None,
            css_output: vec![],
            config: config.into(),
            declarations: vec![],
            var_decl_count_map: HashMap::new(),
        }
    }

    pub fn new_test_classname(comments: C, config: Option<StylexConfigParams>) -> Self {
        ModuleTransformVisitor {
            comments,
            declaration: Option::None,
            cycle: ModuleCycle::Initializing,
            package_name: "@stylexjs/stylex".to_string(),
            file_name: extract_filename_from_path(FileName::Real(PathBuf::from("app/page.tsx"))),
            props_declaration: Option::None,
            css_output: vec![],
            config: config.unwrap_or(StylexConfigParams::default()).into(),
            declarations: vec![],
            var_decl_count_map: HashMap::new(),
        }
    }
    pub fn new_test_styles(comments: C, config: Option<StylexConfigParams>) -> Self {
        ModuleTransformVisitor {
            comments,
            declaration: Option::None,
            cycle: ModuleCycle::Initializing,
            package_name: "@stylexjs/stylex".to_string(),
            file_name: extract_filename_from_path(FileName::Real(PathBuf::from("app/page.tsx"))),
            props_declaration: Option::None,
            css_output: vec![],
            config: match &config {
                Some(config) => config.clone().into(),
                None => {
                    let mut config = StylexConfig::default();

                    config.runtime_injection = true;

                    config
                }
            },
            declarations: vec![],
            var_decl_count_map: HashMap::new(),
        }
    }

    pub(crate) fn process_declaration(&mut self, call_expr: &CallExpr) -> Option<(Id, String)> {
        match &mut call_expr.callee.clone() {
            Callee::Expr(callee) => match callee.as_ref() {
                Expr::Member(member) => match member.obj.as_ref() {
                    Expr::Ident(ident) => {
                        let ident_id = ident.to_id();

                        if self.declaration.clone().unwrap_or_default().eq(&ident_id) {
                            match member.prop.clone() {
                                MemberProp::Ident(ident) => {
                                    increase_ident_count(&mut self.var_decl_count_map, &ident);

                                    return Option::Some((
                                        ident_id.clone(),
                                        format!("{}", ident.sym),
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        Option::None
    }

    pub(crate) fn transform_call_expression(&mut self, expr: &mut Expr) -> Option<Expr> {
        match expr {
            Expr::Call(ex) => {
                let declaration = self.process_declaration(&ex);

                if declaration.is_some() {
                    let value = if self.config.runtime_injection {
                        self.transform_call_expression_to_styles_expr(&ex)
                    } else {
                        self.transform_call_expression_to_css_map_expr(&ex)
                    };

                    match value {
                        Some(value) => {
                            return Some(value);
                        }
                        None => {}
                    }
                }
            }
            _ => {}
        }
        None
    }

    pub(crate) fn push_to_css_output(&mut self, metadata: MetaData) {
        if self
            .css_output
            .iter()
            .any(|x| x.get_class_name() == metadata.get_class_name())
        {
            return;
        }

        println!("!!!!__ metadata: {:#?}", metadata);

        self.css_output.push(metadata);
    }
}

// static COUNTER: AtomicUsize = AtomicUsize::new(0);

// fn generate_unique_identifier(name: &str) -> Ident {
//     let mark = Mark::fresh(Mark::root());
//     let count = COUNTER.fetch_add(1, Ordering::SeqCst);
//     let unique_name = format!("_{}_{}", name, count);
//     Ident::new(unique_name.into(), DUMMY_SP.apply_mark(mark))
// }
