use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Id, Ident};
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Expr},
};

use crate::shared::constants;
use crate::shared::enums::{TopLevelExpression, TopLevelExpressionKind};
use crate::shared::structures::flat_compiled_styles::FlatCompiledStyles;
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, FunctionType};
use crate::shared::structures::injectable_style::{self, InjectableStyle};
use crate::shared::structures::meta_data::MetaData;
use crate::shared::structures::named_import_source::ImportSources;
use crate::shared::utils::common::gen_file_based_identifier;
use crate::shared::utils::css::stylex::evaluate::evaluate;
use crate::shared::utils::css::stylex::evaluate_stylex_create_arg::evaluate_stylex_create_arg;
use crate::shared::utils::js::stylex::stylex_create::stylex_create_set;
use crate::shared::utils::js::stylex::stylex_define_vars::stylex_define_vars;
use crate::shared::utils::js::stylex::stylex_first_that_works::stylex_first_that_works;
use crate::shared::utils::js::stylex::stylex_include::stylex_include;
use crate::shared::utils::stylex::dev_class_name::{
    convert_to_test_styles, inject_dev_class_names,
};
use crate::shared::utils::stylex::js_to_expr::{
    convert_object_to_ast, remove_objects_with_spreads, NestedStringObject,
};
use crate::shared::utils::validators::{is_define_vars_call, validate_stylex_define_vars};
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn transform_stylex_define_vars(&mut self, call: &CallExpr) -> Option<Expr> {
        self.state.in_stylex_create = true;
        let is_define_vars = is_define_vars_call(call, &mut self.state);

        let result = if is_define_vars {
            validate_stylex_define_vars(call, &mut self.state);

            let first_arg = call.args.get(0);

            match first_arg {
                Some(first_arg) => match &first_arg.spread {
                    Some(_) => todo!(),
                    None => {
                        let mut resolved_namespaces: IndexMap<String, FlatCompiledStyles> =
                            IndexMap::new();

                        let injected_keyframes: IndexMap<String, InjectableStyle> = IndexMap::new();

                        let mut identifiers: HashMap<Id, FunctionConfig> = HashMap::new();
                        let mut member_expressions: HashMap<
                            ImportSources,
                            HashMap<Id, FunctionConfig>,
                        > = HashMap::new();

                        let include_fn = FunctionConfig {
                            fn_ptr: FunctionType::ArrayArgs(stylex_include),
                            takes_path: true,
                        };

                        let first_that_works_fn = FunctionConfig {
                            fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
                            takes_path: false,
                        };

                        let keyframes_fn = FunctionConfig {
                            fn_ptr: FunctionType::OneArg(|_arg| {
                                panic!("Keyframes not implemented")
                            }),
                            takes_path: false,
                        };

                        for name in &self.state.stylex_include_import {
                            identifiers.insert(name.clone(), include_fn.clone());
                        }

                        for name in &self.state.stylex_first_that_works_import {
                            identifiers.insert(name.clone(), first_that_works_fn.clone());
                        }

                        for name in &self.state.stylex_keyframes_import {
                            identifiers.insert(name.clone(), keyframes_fn.clone());
                        }

                        for name in &self.state.stylex_import {
                            member_expressions
                                .entry(name.clone())
                                .or_insert(HashMap::new());

                            let member_expression = member_expressions.get_mut(name).unwrap();

                            member_expression.insert(
                                Ident::new("include".into(), DUMMY_SP).to_id(),
                                include_fn.clone(),
                            );

                            member_expression.insert(
                                Ident::new("firstThatWorks".into(), DUMMY_SP).to_id(),
                                first_that_works_fn.clone(),
                            );

                            member_expression.insert(
                                Ident::new("keyframes".into(), DUMMY_SP).to_id(),
                                keyframes_fn.clone(),
                            );
                        }

                        let function_map: FunctionMap = FunctionMap {
                            identifiers,
                            member_expressions,
                        };

                        let evaluated_arg =
                            evaluate(&first_arg.expr, &mut self.state, &function_map);

                        dbg!(evaluated_arg.clone());

                        assert!(
                            evaluated_arg.confident,
                            "{}",
                            constants::messages::NON_STATIC_VALUE
                        );

                        let value = match evaluated_arg.value {
                            Some(value) => {
                                assert!(
                                    value
                                        .as_expr()
                                        .and_then(|expr| Option::Some(expr.is_object()))
                                        .unwrap_or(false),
                                    "{}",
                                    constants::messages::NON_OBJECT_FOR_STYLEX_CALL
                                );
                                value
                            }
                            None => {
                                panic!("{}", constants::messages::NON_STATIC_VALUE)
                            }
                        };
                        dbg!(&evaluated_arg.confident, &value);

                        let Some(file_name) = self.state.get_filename_for_hashing() else {
                            panic!("No filename found for generating theme name.")
                        };

                        let export_expr = self
                            .state
                            .get_top_level_expr(&TopLevelExpressionKind::NamedExport, call);

                        let Some(export_name) = export_expr
                            .and_then(|expr| expr.2)
                            .and_then(|decl| Option::Some(decl.0.to_string()))
                        else {
                            panic!("Export variable not found")
                        };

                        self.state.theme_name = Option::Some(gen_file_based_identifier(
                            &file_name,
                            &export_name,
                            Option::None,
                        ));

                        dbg!(&self.state.theme_name);

                        let (variables_obj, injected_styles_sans_keyframes) =
                            stylex_define_vars(&value, &mut self.state);

                        dbg!(&variables_obj, &injected_styles_sans_keyframes);

                        let mut injectable_style = injected_keyframes.clone();
                        injectable_style.extend(injected_styles_sans_keyframes);

                        dbg!(&variables_obj);

                        let (var_name, _) = self.get_call_var_name(call);

                        let result_ast = convert_object_to_ast(
                            &NestedStringObject::FlatCompiledStylesValues(variables_obj),
                        );

                        self.state
                            .register_styles(call, &injectable_style, &result_ast, &var_name);

                        return Option::Some(result_ast);
                    }
                },
                None => Option::None,
            }
        } else {
            Option::None
        };

        self.state.in_stylex_create = false;

        result
    }
}
