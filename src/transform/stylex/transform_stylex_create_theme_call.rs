use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Id, Ident, KeyValueProp, Lit, Str, VarDeclarator};
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Expr, Prop, PropOrSpread},
};

use crate::shared::constants::{self, messages};
use crate::shared::enums::TopLevelExpression;
use crate::shared::structures::evaluate_result::EvaluateResultValue;
use crate::shared::structures::flat_compiled_styles::FlatCompiledStyles;
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, FunctionType};
use crate::shared::structures::injectable_style::{self, InjectableStyle};
use crate::shared::structures::meta_data::MetaData;
use crate::shared::structures::named_import_source::ImportSources;
use crate::shared::utils::common::{
    get_key_str, get_key_values_from_object, get_string_val_from_lit,
    prop_or_spread_string_creator, string_to_expression,
};
use crate::shared::utils::css::factories::object_expression_factory;
use crate::shared::utils::css::stylex::evaluate::evaluate;
use crate::shared::utils::css::stylex::evaluate_stylex_create_arg::evaluate_stylex_create_arg;
use crate::shared::utils::js::stylex::stylex_create::stylex_create_set;
use crate::shared::utils::js::stylex::stylex_create_theme::stylex_create_theme;
use crate::shared::utils::js::stylex::stylex_first_that_works::stylex_first_that_works;
use crate::shared::utils::js::stylex::stylex_include::stylex_include;
use crate::shared::utils::js::stylex::stylex_keyframes::stylex_keyframes;
use crate::shared::utils::stylex::dev_class_name::{
    convert_to_test_styles, inject_dev_class_names,
};
use crate::shared::utils::stylex::js_to_expr::{
    convert_object_to_ast, remove_objects_with_spreads, NestedStringObject,
};
use crate::shared::utils::validators::{
    assert_valid_keyframes, is_create_call, is_create_theme_call, is_keyframes_call,
    validate_namespace, validate_stylex_create_indent, validate_stylex_create_theme_indent,
    validate_stylex_keyframes_indent, validate_theme_variables,
};
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn transform_stylex_create_theme_call(&mut self, call: &CallExpr) -> Option<Expr> {
        let is_create_theme_call = is_create_theme_call(call, &mut self.state);

        let result = if is_create_theme_call {
            let (_, parent_var_decl) = &self.get_call_var_name(call);

            validate_stylex_create_theme_indent(parent_var_decl, call, &mut self.state);

            let id = parent_var_decl
                .as_ref()
                .expect(constants::messages::UNBOUND_STYLEX_CALL_VALUE)
                .clone()
                .name;

            let variable_name = id
                .as_ident()
                .expect(constants::messages::UNBOUND_STYLEX_CALL_VALUE)
                .to_id();

            let first_arg = call.args.get(0);
            let second_arg = call.args.get(1);

            let Some(first_arg) = first_arg.and_then(|first_arg| match &first_arg.spread {
                Some(_) => todo!(),
                None => Option::Some(first_arg.expr.clone()),
            }) else {
                return Option::None;
            };

            let Some(second_arg) = second_arg.and_then(|second_arg| match &second_arg.spread {
                Some(_) => todo!(),
                None => Option::Some(second_arg.expr.clone()),
            }) else {
                return Option::None;
            };

            // let mut resolved_namespaces: IndexMap<String, FlatCompiledStyles> =
            //     IndexMap::new();

            // // let injected_keyframes: IndexMap<String, InjectableStyle> = IndexMap::new();

            // let mut identifiers: HashMap<Id, FunctionConfig> = HashMap::new();
            // let mut member_expressions: HashMap<
            //     ImportSources,
            //     HashMap<Id, FunctionConfig>,
            // > = HashMap::new();

            // let include_fn = FunctionConfig {
            //     fn_ptr: FunctionType::ArrayArgs(stylex_include),
            //     takes_path: true,
            // };

            // let first_that_works_fn = FunctionConfig {
            //     fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
            //     takes_path: false,
            // };

            // for name in &self.state.stylex_include_import {
            //     identifiers.insert(name.clone(), include_fn.clone());
            // }

            // for name in &self.state.stylex_first_that_works_import {
            //     identifiers.insert(name.clone(), first_that_works_fn.clone());
            // }

            // for name in &self.state.stylex_import {
            //     member_expressions
            //         .entry(name.clone())
            //         .or_insert(HashMap::new());

            //     let member_expression = member_expressions.get_mut(name).unwrap();

            //     member_expression.insert(
            //         Ident::new("include".into(), DUMMY_SP).to_id(),
            //         include_fn.clone(),
            //     );

            //     member_expression.insert(
            //         Ident::new("firstThatWorks".into(), DUMMY_SP).to_id(),
            //         first_that_works_fn.clone(),
            //     );
            // }

            // let function_map: FunctionMap = FunctionMap {
            //     identifiers,
            //     member_expressions,
            // };

            let evaluated_arg1 = evaluate(&first_arg, &mut self.state, &FunctionMap::default());

            assert!(
                evaluated_arg1.confident,
                "{}",
                constants::messages::NON_STATIC_VALUE
            );

            let evaluated_arg2 = evaluate(&second_arg, &mut self.state, &FunctionMap::default());

            assert!(
                evaluated_arg2.confident,
                "{}",
                constants::messages::NON_STATIC_VALUE
            );

            let variables = match evaluated_arg1.value {
                Some(value) => {
                    validate_theme_variables(&value);

                    value
                }
                None => {
                    panic!("Can only override variables theme created with stylex.defineVars().")
                }
            };

            let overrides = match evaluated_arg2.value {
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
                    panic!("{}", constants::messages::NON_OBJECT_FOR_STYLEX_CALL)
                }
            };

            // let plain_object = value;

            // assert_valid_keyframes(&plain_object);

            let (overrides_obj, inject_styles) =
                stylex_create_theme(&variables, &overrides, &mut self.state);

            let (var_name, _) = self.get_call_var_name(call);

            let result_ast =
                convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(overrides_obj));

            self.state
                .register_styles(call, &inject_styles, &result_ast, &var_name);

            return Option::Some(result_ast);
        } else {
            None
        };

        dbg!(&result);

        result
    }
}
