use std::collections::HashMap;

use stylex_swc_plugin::shared::structures::{
    functions::{FunctionConfig, FunctionMap, FunctionType},
    named_import_source::ImportSources,
};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{ArrayLit, Expr, ExprOrSpread, Ident, Lit, NewExpr, Str},
        parser::{Syntax, TsConfig},
        transforms::testing::test,
    },
};

use crate::evaluation::evaluation_module_transform::EvaluationModuleTransformVisitor;

#[test]
fn evaluates_primitive_value_expressions() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| EvaluationModuleTransformVisitor::default(),
        r#"
            1 + 2;
            1 - 2;
            1 * 2;
            1 / 2;
            1 % 2;
            1 ** 2;
            1 << 2;
            1 >> 2;
            1 & 2;
            1 | 2;
            1 ^ 2;
            1 && 2;
            1 || 2;

            null;
            undefined;
            true;
            false;
            let x = "hello";
        "#,
        r#"
            3;
            -1;
            2;
            0.5;
            1;
            1;
            4;
            0;
            0;
            3;
            3;
            2;
            1;

            null
            undefined
            true
            false
            "hello"
        "#,
        false,
    )
}

#[test]
fn evaluates_simple_arrays_and_objects() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| EvaluationModuleTransformVisitor::default(),
        r#"
            const x = {};
            const x = {name: "Name", age: 43};
            const x = [];
            const x = [1, 2, 3];
            const x = [1, 2, 3, 4, 5];
        "#,
        r#"
            ({});
            ({name: "Name", age: 43});
            [];
            [1, 2, 3];
            [1, 2, 3, 4, 5];
        "#,
        false,
    )
}

#[test]
fn evaluates_objects_with_spreads() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| EvaluationModuleTransformVisitor::default(),
        r#"
            const x = {name: "Name", ...({hero: true}), age: 43};
            const x = {name: "Name", ...({name: "StyleXToOverride", age: 1, name: "StyleX"}), age: 43};
            const x = {name: "Name", ...({name: "NameToOverride", age: 1, name: "SecondnameToOverride"}), age: 43, name: "StyleX"};
        "#,
        r#"
            ({ name: "Name", hero: true, age: 43 });
            ({ name: "StyleX", age: 43 });
            ({ age: 43 , name: "StyleX", });
        "#,
        false,
    )
}

#[test]
#[should_panic(expected = "Evaluation built-in functions not supported")]
fn evaluates_built_in_functions() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| EvaluationModuleTransformVisitor::default(),
        r#"
            const x = Math.max(1, 2, 3);
            const x = Math.min(1, 2, 3)
        "#,
        r#""#,
        false,
    )
}

#[test]
fn evaluates_customs_functions() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| {
            let mut identifiers = HashMap::new();

            let make_array = FunctionConfig {
                fn_ptr: FunctionType::ArrayArgs(|args| {
                    let mut reversed = args;
                    reversed.reverse();
                    Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems: reversed
                            .into_iter()
                            .map(|expr| {
                                Some(ExprOrSpread {
                                    spread: None,
                                    expr: Box::new(expr),
                                })
                            })
                            .collect(),
                    })
                }),
                takes_path: false,
            };

            identifiers.insert("makeArray".to_string(), make_array);

            let mut member_expressions = HashMap::new();

            member_expressions.insert(
                ImportSources::Regular("stylex".to_string()),
                identifiers.clone(),
            );

            EvaluationModuleTransformVisitor {
                functions: FunctionMap {
                    identifiers,
                    member_expressions,
                },
            }
        },
        r#"
            const x = makeArray(1, 2, 3);
            const x = stylex.makeArray(1, 2, 3);
        "#,
        r#"
            [3, 2, 1];
            [3, 2, 1];
        "#,
        false,
    )
}

#[test]
fn evaluates_custom_functions_that_return_non_static_values() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |_| {
            let mut identifiers = HashMap::new();

            let make_class = FunctionConfig {
                fn_ptr: FunctionType::OneArg(|arg: Expr| {
                    let new_expr = NewExpr {
                        span: DUMMY_SP,
                        callee: Box::new(Expr::Ident(Ident::new("MyClass".into(), DUMMY_SP))),
                        args: Some(vec![ExprOrSpread {
                            spread: None,
                            expr: Box::new(arg),
                        }]),
                        type_args: None,
                    };

                    Expr::New(new_expr)
                }),
                takes_path: false,
            };

            identifiers.insert("makeClass".to_string(), make_class);

            EvaluationModuleTransformVisitor {
                functions: FunctionMap {
                    identifiers,
                    member_expressions: HashMap::new(),
                },
            }
        },
        r#"
            const x = makeClass("Hello");
        "#,
        r#"
            new MyClass("Hello")
        "#,
        false,
    )
}
