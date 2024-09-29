use std::collections::HashMap;

use stylex_shared::shared::{
  structures::{
    functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
    named_import_source::ImportSources,
    state_manager::StateManager,
  },
  utils::ast::convertors::{ident_to_expression, string_to_expression},
};
use swc_core::{
  atoms::Atom,
  common::DUMMY_SP,
  ecma::{
    ast::{
      ArrayLit, Expr, ExprOrSpread, KeyValueProp, NewExpr, ObjectLit, Prop, PropName, PropOrSpread,
    },
    parser::{Syntax, TsSyntax},
    transforms::testing::{test, test_transform},
    utils::quote_ident,
  },
};

use crate::evaluation::evaluation_module_transform::EvaluationStyleXTransform;

#[test]
fn evaluates_primitive_value_expressions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
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
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
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
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
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
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
    r#"
            const x = Object.getOwnPropertyNames({a: 2});
        "#,
    r#""#,
    false,
  )
}

#[test]
fn evaluates_customs_functions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
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

      identifiers.insert(
        Atom::from("makeArray"),
        Box::new(FunctionConfigType::Regular(make_array)),
      );

      let mut member_expressions = HashMap::new();

      member_expressions.insert(
        ImportSources::Regular("stylex".to_string()),
        Box::new(identifiers.clone()),
      );

      EvaluationStyleXTransform {
        functions: FunctionMap {
          identifiers,
          member_expressions,
        },
        declarations: vec![],
        state: StateManager::default(),
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
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| {
      let mut identifiers = HashMap::new();

      let make_class = FunctionConfig {
        fn_ptr: FunctionType::StylexExprFn(|arg, _| {
          let new_expr = NewExpr {
            span: DUMMY_SP,
            callee: Box::new(ident_to_expression("MyClass")),
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

      identifiers.insert(
        Atom::from("makeClass"),
        Box::new(FunctionConfigType::Regular(make_class)),
      );

      EvaluationStyleXTransform {
        functions: FunctionMap {
          identifiers,
          member_expressions: HashMap::new(),
        },
        declarations: vec![],
        state: StateManager::default(),
      }
    },
    r#"
            const x = makeClass("Hello");
        "#,
    r#"
            new MyClass("Hello");
        "#,
    false,
  )
}

#[test]
fn evaluates_custom_functions_used_as_spread_values() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| {
      let mut identifiers = HashMap::new();

      let make_obj = FunctionConfig {
        fn_ptr: FunctionType::StylexExprFn(|arg, _| {
          let object_lit = ObjectLit {
            span: DUMMY_SP,
            props: vec![PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
              key: PropName::Ident(quote_ident!("spreadValue")),
              value: Box::new(arg),
            })))],
          };

          Expr::Object(object_lit)
        }),
        takes_path: false,
      };

      identifiers.insert(
        Atom::from("makeObj"),
        Box::new(FunctionConfigType::Regular(make_obj)),
      );

      EvaluationStyleXTransform {
        functions: FunctionMap {
          identifiers,
          member_expressions: HashMap::new(),
        },
        declarations: vec![],
        state: StateManager::default(),
      }
    },
    r#"
            const x = {name: "Name", ...makeObj("Hello"), age: 30};
        "#,
    r#"
        ({ name: "Name", spreadValue: "Hello", age: 30 });
        "#,
    false,
  )
}

#[test]
fn evaluates_custom_functions_that_take_paths() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| {
      let mut identifiers = HashMap::new();

      let get_node = FunctionConfig {
        fn_ptr: FunctionType::StylexExprFn(|arg, _| {
          let object_lit = ObjectLit {
            span: DUMMY_SP,
            props: vec![
              PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
                key: PropName::Ident(quote_ident!("type")),
                value: Box::new(string_to_expression("StringLiteral")),
              }))),
              PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
                key: PropName::Ident(quote_ident!("value")),
                value: Box::new(arg),
              }))),
            ],
          };

          Expr::Object(object_lit)
        }),
        takes_path: true,
      };

      identifiers.insert(
        Atom::from("getNode"),
        Box::new(FunctionConfigType::Regular(get_node)),
      );

      EvaluationStyleXTransform {
        functions: FunctionMap {
          identifiers,
          member_expressions: HashMap::new(),
        },
        declarations: vec![],
        state: StateManager::default(),
      }
    },
    r#"
            const x = getNode("Hello");
        "#,
    r#"
            ({ type: "StringLiteral", value: "Hello" });
        "#,
    false,
  )
}

#[test]
fn evaluates_unary_value_expressions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
    r#"
            !1;
            !0;
            !{};
            !null;
            !false;
            !true;
            +1;
            +"1";
            -1;
            -"1";
            ~1;
            ~3;
            typeof 1;
            typeof "a";
            typeof null;
            typeof {};
            typeof undefined;
        "#,
    r#"
            false;
            true;
            false;
            true;
            true;
            false;
            1;
            1;
            -1;
            -1;
            -2;
            -4;
            "number";
            "string";
            "object";
            "object";
            "undefined";
        "#,
    false,
  )
}

#[test]
#[should_panic(expected = "Failed to evaluate expression")]
fn evaluates_void_unary_value_expressions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
    r#"
              void 1;

        "#,
    r#""#,
    false,
  )
}

#[test]
#[should_panic(expected = "Failed to evaluate expression")]
fn evaluates_delete_unary_value_expressions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
    r#"
              delete a.b;

        "#,
    r#""#,
    false,
  )
}

#[test]
fn evaluates_sequence_value_expressions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
    r#"
            (1,2,3);
            (1,2,3,4,5);
            (1,2,3,4,5,6,7,8,9,10);
            (-1,-2,-3);
            (-1,-2,-3,-4,-5);
            (-1,-2,-3,-4,-5,-6,-7,-8,-9,-10);
        "#,
    r#"
            3;
            5;
            10;
            -3;
            -5;
            -10;
        "#,
    false,
  )
}

#[test]
fn evaluates_ts_as_value_expressions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
    r#"
            (3 as number) * (4 as number);
        "#,
    r#"
            12
        "#,
    false,
  )
}

#[test]
fn evaluates_ts_satisfies_value_expressions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
    r#"
            (3 satisfies number) * (4 satisfies number);
        "#,
    r#"
            12
        "#,
    false,
  )
}

#[test]
fn evaluates_condition_value_expressions() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |_| EvaluationStyleXTransform::default(),
    r#"
            2 > 1 ? 1 : 0
            2 < 1 ? 1 : 0
        "#,
    r#"
            1
            0
        "#,
    false,
  )
}
