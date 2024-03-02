use std::{
    any::type_name,
    collections::{HashMap, HashSet},
};

use radix_fmt::radix;
use swc_core::{
    common::{FileName, Span, DUMMY_SP},
    ecma::{
        ast::{
            BinExpr, BinaryOp, BindingIdent, Bool, Expr, ExprOrSpread, Id, Ident, KeyValueProp,
            Lit, Number, ObjectLit, Pat, Prop, PropName, PropOrSpread, Tpl, UnaryExpr, UnaryOp,
            VarDeclarator,
        },
        visit::{Fold, FoldWith},
    },
};

use crate::shared::{
    constants::{self, messages::ILLEGAL_PROP_VALUE},
    structures::functions::{FunctionMap, FunctionType},
};

use super::css::stylex::{evaluate, evaluate_cached, State};

struct SpanReplacer;

impl Fold for SpanReplacer {
    fn fold_span(&mut self, _: Span) -> Span {
        DUMMY_SP
    }
}

fn replace_spans(expr: &mut Expr) -> Expr {
    expr.clone().fold_children_with(&mut SpanReplacer)
}



pub(crate) fn prop_or_spread_expression_creator(key: String, value: Expr) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new(key.into(), DUMMY_SP)),
        value: Box::new(value),
    })))
}

pub(crate) fn prop_or_spread_string_creator(key: String, value: String) -> PropOrSpread {
    let value = string_to_expression(value);

    match value {
        Some(value) => prop_or_spread_expression_creator(key, value),
        None => panic!("Value is not a string"),
    }
}

pub(crate) fn prop_or_spread_boolean_creator(key: String, value: Option<bool>) -> PropOrSpread {
    match value {
        Some(value) => prop_or_spread_expression_creator(
            key,
            Expr::Lit(Lit::Bool(Bool {
                span: DUMMY_SP,
                value,
            })),
        ),
        None => panic!("Value is not a boolean"),
    }
}

// Converts a string to an expression.
pub(crate) fn string_to_expression(value: String) -> Option<Expr> {
    Option::Some(Expr::Lit(Lit::Str(value.into())))
}

// Converts a number to an expression.
pub(crate) fn number_to_expression(value: f64) -> Option<Expr> {
    Option::Some(Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: value,
        raw: Option::None,
    })))
}

pub(crate) fn extract_filename_from_path(path: FileName) -> String {
    match path {
        FileName::Real(path_buf) => path_buf.file_stem().unwrap().to_str().unwrap().to_string(),
        _ => panic!("Not a real file name"),
    }
}

pub(crate) fn hash_css(value: &str) -> String {
    radix(murmur2::murmur2(value.as_bytes(), 1), 36).to_string()
}

pub(crate) fn get_string_val_from_lit(value: &Lit) -> String {
    match value {
        Lit::Str(str) => format!("{}", str.value),
        Lit::Num(num) => format!("{}", num.value),
        _ => panic!("{}", ILLEGAL_PROP_VALUE),
    }
}

pub(crate) fn get_key_str(key_value: &KeyValueProp) -> String {
    let key = &key_value.key;
    let mut should_wrap_in_quotes = false;

    let key = match key {
        PropName::Ident(ident) => &*ident.sym,
        PropName::Str(str) => {
            should_wrap_in_quotes = false;

            &*str.value
        }
        _ => panic!("Key is not recognized"),
    };

    wrap_key_in_quotes(key, &should_wrap_in_quotes)
}

pub(crate) fn wrap_key_in_quotes(key: &str, should_wrap_in_quotes: &bool) -> String {
    let key = if *should_wrap_in_quotes {
        format!("\"{}\"", key)
    } else {
        key.to_string()
    };

    key
}

pub(crate) fn push_css_anchor_prop(props: &mut Vec<PropOrSpread>) {
    props.push(prop_or_spread_boolean_creator(
        "$$css".to_string(),
        Option::Some(true),
    ))
}

pub(crate) fn get_pat_as_string(pat: &Pat) -> String {
    match pat {
        Pat::Ident(ident) => ident.sym.to_string(),
        _ => todo!("get_pat_as_string: Pat"),
    }
}

// pub(crate) fn expr_or_spread_object_expression_creator(
//     key: String,
//     value: Box<Expr>,
// ) -> ExprOrSpread {
//     let expr = Box::new(Expr::Object(ObjectLit {
//         span: DUMMY_SP,
//         props: vec![prop_or_spread_box_expression_creator(key.as_ref(), value)],
//     }));

//     ExprOrSpread {
//         expr,
//         spread: Option::None,
//     }
// }

pub(crate) fn expr_or_spread_string_expression_creator(value: String) -> ExprOrSpread {
    let expr = Box::new(string_to_expression(value).unwrap());

    ExprOrSpread {
        expr,
        spread: Option::None,
    }
}

pub(crate) fn expr_or_spread_number_expression_creator(value: f64) -> ExprOrSpread {
    let expr = Box::new(number_to_expression(value).unwrap());

    ExprOrSpread {
        expr,
        spread: Option::None,
    }
}

// pub(crate) fn prop_or_spread_box_expression_creator(key: &str, value: Box<Expr>) -> PropOrSpread {
//     PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
//         key: PropName::Ident(Ident::new(key.into(), DUMMY_SP)),
//         value,
//     })))
// }

pub fn reduce_ident_count(var_dec_count_map: &mut HashMap<Id, i8>, ident: &Ident) {
    *var_dec_count_map.entry(ident.to_id()).or_insert(1) -= 1;
}

pub fn increase_ident_count(var_dec_count_map: &mut HashMap<Id, i8>, ident: &Ident) {
    let ident_id = &ident.to_id();
    *var_dec_count_map.entry(ident_id.clone()).or_insert(-1) += 1;
}

pub fn get_var_decl_by_ident<'a>(
    ident: &'a Ident,
    declarations: &'a Vec<VarDeclarator>,
    var_dec_count_map: &'a mut HashMap<Id, i8>,
    functions: &'a FunctionMap,
) -> Option<VarDeclarator> {
    reduce_ident_count(var_dec_count_map, &ident);
    match get_var_decl_from(declarations, ident) {
        Some(var_decl) => Some(var_decl.clone()),
        None => {
            let func = functions.identifiers.get(&ident.sym.to_string());

            match func {
                Some(func) => {
                    let func = func.clone();

                    match func.fn_ptr {
                        FunctionType::Mapper(func) => {
                            let arg = Expr::Ident(ident.clone());
                            let result = func();

                            println!("!!!!! ident: {:?}, result: {:?}", ident, result);

                            let var_decl = VarDeclarator {
                                span: DUMMY_SP,
                                name: Pat::Ident(BindingIdent {
                                    id: ident.clone(),
                                    type_ann: Option::None,
                                }),
                                init: Option::Some(Box::new(result)), // Clone the result
                                definite: false,
                            };

                            let var_declarator = var_decl.clone();
                            Option::Some(var_declarator)
                        }
                        _ => panic!("Function type not supported"),
                    }
                }
                None => Option::None,
            }
        }
    }
}

pub(crate) fn get_var_decl_from<'a>(
    declarations: &'a Vec<VarDeclarator>,
    ident: &'a Ident,
) -> Option<&'a VarDeclarator> {
    declarations.iter().find(|var_declarator| {
        if let Pat::Ident(binding_indent) = &var_declarator.name {
            return binding_indent.sym == ident.sym;
        }

        false
    })
}

pub fn get_expr_from_var_decl(var_decl: &VarDeclarator) -> Expr {
    let var_decl_expr = match &var_decl.init {
        Some(var_decl_init) => unbox(var_decl_init.clone()),
        None => panic!("Variable declaration is not an expression"),
    };

    var_decl_expr
}

pub fn unbox_option<T>(item: Option<Box<T>>) -> T {
    match item {
        Some(item) => unbox(item),
        None => panic!("Item is undefined"),
    }
}

pub fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn expr_to_num(
    expr_num: &Expr,
    state: &mut State,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> f32 {
    match &expr_num {
        Expr::Ident(ident) => ident_to_number(&ident, state, declarations, var_dec_count_map),
        Expr::Lit(lit) => lit_to_num(&lit),
        Expr::Unary(unary) => unari_to_num(&unary, state, declarations, var_dec_count_map),
        Expr::Bin(lit) => match binary_expr_to_num(&lit, state, declarations, var_dec_count_map) {
            Some(result) => result,
            None => panic!("Binary expression is not a number"),
        },
        _ => panic!("Expression in not a number {:?}", expr_num),
    }
}

fn ident_to_string(
    ident: &Ident,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
    functions: &FunctionMap,
) -> String {
    let var_decl = get_var_decl_by_ident(&ident, declarations, var_dec_count_map, functions);

    println!("var_decl: {:?}, ident: {:?}", var_decl, ident);

    match &var_decl {
        Some(var_decl) => {
            let var_decl_expr = get_expr_from_var_decl(var_decl);

            match &var_decl_expr {
                Expr::Lit(lit) => get_string_val_from_lit(&lit),
                Expr::Ident(ident) => {
                    ident_to_string(ident, declarations, var_dec_count_map, functions)
                }
                _ => panic!("{}", ILLEGAL_PROP_VALUE),
            }
        }
        None => panic!("{}", ILLEGAL_PROP_VALUE),
    }
}

pub fn expr_to_str(
    expr_string: &Expr,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
    functions: &FunctionMap,
) -> String {
    match &expr_string {
        Expr::Ident(ident) => ident_to_string(&ident, declarations, var_dec_count_map, functions),
        Expr::Lit(lit) => get_string_val_from_lit(&lit),
        _ => panic!("Expression in not a string {:?}", expr_string),
    }
}

pub fn unari_to_num(
    unary_expr: &UnaryExpr,
    state: &mut State,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> f32 {
    let arg = unary_expr.arg.as_ref();
    let op = unary_expr.op;

    match &op {
        UnaryOp::Minus => expr_to_num(arg, state, declarations, var_dec_count_map) * -1.0,
        UnaryOp::Plus => expr_to_num(arg, state, declarations, var_dec_count_map),
        _ => panic!("Union operation '{}' is invalid", op),
    }
}

pub fn binary_expr_to_num(
    binary_expr: &BinExpr,
    state: &mut State,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> Option<f32> {
    let binary_expr = binary_expr.clone();

    let op = binary_expr.op;
    let Some(left) = evaluate_cached(
        &*binary_expr.left,
        state,
        declarations,
        var_dec_count_map,
    ) else {
        panic!("Left expression is not a number")
    };

    let Some(right) = evaluate_cached(
        &*binary_expr.right,
        state,
        declarations,
        var_dec_count_map,
    ) else {
        panic!("Left expression is not a number")
    };

    let result = match &op {
        BinaryOp::Add => {
            expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) + expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
        }
        BinaryOp::Sub => {
            expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) - expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
        }
        BinaryOp::Mul => {
            expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) * expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
        }
        BinaryOp::Div => {
            expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) / expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
        }
        BinaryOp::Mod => {
            expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) % expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
        }
        BinaryOp::Exp => expr_to_num(
            left.as_expr()?,
            state,
            declarations,
            var_dec_count_map,
        )
        .powf(expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)),
        BinaryOp::RShift => {
            ((expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) as i32)
                >> expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map) as i32)
                as f32
        }
        BinaryOp::LShift => {
            ((expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) as i32)
                << expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map) as i32)
                as f32
        }
        BinaryOp::BitAnd => {
            ((expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) as i32)
                & expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map) as i32)
                as f32
        }
        BinaryOp::BitOr => {
            ((expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) as i32)
                | expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map) as i32)
                as f32
        }
        BinaryOp::BitXor => {
            ((expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) as i32)
                ^ expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map) as i32)
                as f32
        }
        BinaryOp::In => {
            if expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map) == 0.0 {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::InstanceOf => {
            if expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map) == 0.0 {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::EqEq => {
            if expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) == expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::NotEq => {
            if expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) != expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::EqEqEq => {
            if expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) == expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::NotEqEq => {
            if expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) != expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::Lt => {
            if expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) < expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::LtEq => {
            if expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) <= expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::Gt => {
            if expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) > expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::GtEq => {
            if expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) >= expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map)
            {
                1.0
            } else {
                0.0
            }
        }
        // #region Logical
        BinaryOp::LogicalOr => {
            println!("!!!!__ state.confident33333: {:#?}", state.confident);

            let mut was_confident = state.confident;
            let mut left_confident = state.confident;
            let mut right_confident = state.confident;

            let result = evaluate_cached(left.as_expr()?, state, declarations, var_dec_count_map);

            let left = result.unwrap();
            let left = left.as_expr().unwrap();

            left_confident = state.confident;

            state.confident = was_confident;

            let result = evaluate_cached(right.as_expr()?, state, declarations, var_dec_count_map);

            let right = result.unwrap();
            let right = right.as_expr().unwrap();
            right_confident = state.confident;

            let left = expr_to_num(
                left,
                state,
                declarations,
                var_dec_count_map,
            );
            let right = expr_to_num(right, state, declarations, var_dec_count_map);

            state.confident = left_confident && (left != 0.0 || right_confident);
            println!("!!!!__ state.confident44444: {:#?}", state.confident);

            if !state.confident {
                return Option::None;
            }

            if left != 0.0 {
                left
            } else {
                right
            }
        }
        BinaryOp::LogicalAnd => {
            let mut was_confident = state.confident;
            let mut left_confident = state.confident;
            let mut right_confident = state.confident;

            let result = evaluate_cached(left.as_expr()?, state, declarations, var_dec_count_map);

            let left = result.unwrap();
            let left = left.as_expr().unwrap();

            left_confident = state.confident;

            state.confident = was_confident;

            let result = evaluate_cached(right.as_expr()?, state, declarations, var_dec_count_map);

            let right = result.unwrap();
            let right = right.as_expr().unwrap();
            right_confident = state.confident;

            let left = expr_to_num(
                left,
                state,
                declarations,
                var_dec_count_map,
            );
            let right = expr_to_num(right, state, declarations, var_dec_count_map);

            state.confident = left_confident && (left == 0.0 || right_confident);

            if !state.confident {
                return Option::None;
            }

            if left != 0.0 {
                right
            } else {
                left
            }
        }
        BinaryOp::NullishCoalescing => {
            let mut was_confident = state.confident;
            let mut left_confident = state.confident;
            let mut right_confident = state.confident;

            let result = evaluate_cached(left.as_expr()?, state, declarations, var_dec_count_map);

            let left = result.unwrap();
            let left = left.as_expr().unwrap();

            left_confident = state.confident;

            state.confident = was_confident;

            let result = evaluate_cached(right.as_expr()?, state, declarations, var_dec_count_map);

            let right = result.unwrap();
            let right = right.as_expr().unwrap();
            right_confident = state.confident;

            let left = expr_to_num(
                left,
                state,
                declarations,
                var_dec_count_map,
            );
            let right = expr_to_num(right, state, declarations, var_dec_count_map);

            state.confident = left_confident && !!(left == 0.0 || right_confident);

            if !state.confident {
                return Option::None;
            }

            if left == 0.0 {
                right
            } else {
                left
            }
        }
        // #endregion Logical
        BinaryOp::ZeroFillRShift => {
            ((expr_to_num(
                left.as_expr()?,
                state,
                declarations,
                var_dec_count_map,
            ) as i32)
                >> expr_to_num(right.as_expr()?, state, declarations, var_dec_count_map) as i32)
                as f32
        }
    };

    Option::Some(result)
}

pub fn ident_to_number(
    ident: &Ident,
    state: &mut State,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> f32 {
    // 1. Get the variable declaration
    let var_decl = get_var_decl_by_ident(&ident, declarations, var_dec_count_map, &state.functions);

    println!(
        "var_decl: {:?}, state.confident: {}",
        var_decl, state.confident
    );

    // 2. Check if it is a variable
    match &var_decl {
        Some(var_decl) => {
            // 3. Do the correct conversion according to the expression
            let var_decl_expr = get_expr_from_var_decl(var_decl);

            match &var_decl_expr {
                Expr::Bin(bin_expr) => {
                    match binary_expr_to_num(&bin_expr, state, declarations, var_dec_count_map) {
                        Some(result) => result,
                        None => panic!("Binary expression is not a number"),
                    }
                }
                Expr::Unary(unary_expr) => {
                    unari_to_num(&unary_expr, state, declarations, var_dec_count_map)
                }
                Expr::Lit(lit) => lit_to_num(&lit),
                _ => panic!("Varable {:?} is not a number", var_decl_expr),
            }
        }
        None => panic!("Variable {} is not declared", ident.sym),
    }
}

pub fn lit_to_num(lit_num: &Lit) -> f32 {
    match &lit_num {
        Lit::Bool(Bool { value, .. }) => {
            if value == &true {
                1.0
            } else {
                0.0
            }
        }
        Lit::Num(num) => num.value as f32,
        Lit::Str(str) => {
            let Result::Ok(num) = str.value.parse::<f32>() else {
                panic!("Value in not a number");
            };

            num
        }
        _ => {
            panic!("Value in not a number");
        }
    }
}

pub fn handle_tpl_to_expression(
    tpl: &swc_core::ecma::ast::Tpl,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
    functions: &FunctionMap,
) -> Expr {
    // Clone the template, so we can work on it
    let mut tpl = tpl.clone();

    // Loop through each expression in the template
    for expr in tpl.exprs.iter_mut() {
        // Check if the expression is an identifier
        if let Expr::Ident(ident) = expr.as_ref() {
            // Find the variable declaration for this identifier in the AST
            let var_decl =
                get_var_decl_by_ident(&ident, declarations, var_dec_count_map, functions);

            // If a variable declaration was found
            match &var_decl {
                Some(var_decl) => {
                    // Swap the placeholder expression in the template with the variable declaration's initializer
                    std::mem::swap(
                        expr,
                        &mut var_decl
                            .init
                            .clone()
                            .expect("Variable declaration has no initializer"),
                    );
                }
                None => {}
            }
        };
    }

    Expr::Tpl(tpl.clone())
}

pub fn expr_tpl_to_string(
    tpl: &Tpl,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
    functions: &FunctionMap,
) -> String {
    let mut tpl_str = String::new();

    for (i, quasi) in tpl.quasis.iter().enumerate() {
        tpl_str.push_str(&quasi.raw.as_ref());

        if i < tpl.exprs.len() {
            match &tpl.exprs[i].as_ref() {
                Expr::Ident(ident) => {
                    let ident =
                        get_var_decl_by_ident(ident, declarations, var_dec_count_map, functions);

                    match ident {
                        Some(var_decl) => {
                            let var_decl_expr = get_expr_from_var_decl(&var_decl);

                            let a = match &var_decl_expr {
                                Expr::Lit(lit) => get_string_val_from_lit(&lit),
                                _ => panic!("{}", constants::messages::ILLEGAL_PROP_VALUE),
                            };

                            tpl_str.push_str(a.as_str());
                        }
                        None => panic!("{}", constants::messages::NON_STATIC_VALUE),
                    }
                }
                Expr::Bin(bin) => tpl_str.push_str(
                    transform_bin_expr_to_number(
                        bin,
                        &mut State::default(),
                        declarations,
                        var_dec_count_map,
                    )
                    .to_string()
                    .as_str(),
                ),
                Expr::Lit(lit) => tpl_str.push_str(&get_string_val_from_lit(&lit)),
                _ => panic!("Value not suppported"), // Handle other expression types as needed
            }
        }
    }

    tpl_str
}

pub fn evaluate_bin_expr(op: BinaryOp, left: f32, right: f32) -> f32 {
    let result = match &op {
        BinaryOp::Add => left + right,
        BinaryOp::Sub => left - right,
        BinaryOp::Mul => left * right,
        BinaryOp::Div => left / right,
        _ => panic!("Operator '{}' is not supported", op),
    };
    result
}

pub fn transform_bin_expr_to_number(
    bin: &BinExpr,
    state: &mut State,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> f32 {
    let op = bin.op;
    let Some(left) = evaluate_cached(
        &*bin.left,
        state,
        declarations,
        var_dec_count_map,
    ) else {
        panic!("Left expression is not a number")
    };

    let Some(right) = evaluate_cached(
        &*bin.right,
        state,
        declarations,
        var_dec_count_map,
    ) else {
        panic!("Left expression is not a number")
    };
    let left = expr_to_num(
        left.as_expr().unwrap(),
        state,
        declarations,
        var_dec_count_map,
    );
    let right = expr_to_num(right.as_expr().unwrap(), state, declarations, var_dec_count_map);

    let result = evaluate_bin_expr(op, left, right);

    result
}

pub(crate) fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn get_value_as_string_from_ident(
    value_ident: &Ident,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> String {
    reduce_ident_count(var_dec_count_map, &value_ident);

    let var_decl = get_var_decl_from(declarations, &value_ident);

    match &var_decl {
        Some(var_decl) => {
            let var_decl_expr = get_expr_from_var_decl(var_decl);

            match &var_decl_expr {
                Expr::Lit(lit) => get_string_val_from_lit(lit),
                Expr::Ident(ident) => {
                    get_value_as_string_from_ident(ident, declarations, var_dec_count_map)
                }
                _ => panic!("Value type not supported"),
            }
        }
        None => {
            println!("value_ident: {:?}", value_ident);
            panic!("Variable not declared")
        }
    }
}

// fn handle_merge_object_exp(arg: &ExprOrSpread, props_map: &mut IndexMap<String, Expr>) {
//     match arg.expr.as_ref() {
//         Expr::Object(obj) => handle_object_param(obj, props_map),
//         Expr::Lit(lit) => handle_lit_param(lit),
//         Expr::Ident(ident) => push_shorthand_ident_props(ident, props_map, true),
//         _ => {
//             panic!("Argument type not supported: {:?}", arg.expr)
//         }
//     }
// }

// fn handle_object_param(obj: &ObjectLit, props_map: &mut IndexMap<String, Expr>) {
//     for prop in &obj.props {
//         match &prop {
//             PropOrSpread::Spread(spread) => self.handle_spread_prop(spread, props_map),
//             PropOrSpread::Prop(prop) => self.handle_object_prop(prop, props_map),
//         };
//     }
// }

// fn handle_object_prop(prop: &Box<Prop>, props_map: &mut IndexMap<String, Expr>) {
//     match prop.as_ref() {
//         Prop::Shorthand(_) => todo!(),
//         Prop::KeyValue(key_value) => {
//             let key = self.get_new_key(key_value);

//             match key_value.value.as_ref() {
//                 Expr::Object(obj) => self.push_object_to_merge_props(obj, props_map, &key),
//                 Expr::Ident(ident) => self.push_ident_to_merge_props(ident, props_map, &key),
//                 Expr::Call(call) => push_call_expr_to_merge_props(call, props_map, &key),
//                 Expr::Lit(lit) => self.push_lit_to_merge_props(lit, key_value, props_map, &key),
//                 Expr::Array(arr) => self.push_array_to_merge_props(arr, props_map, &key),
//                 Expr::Tpl(tpl) => self.push_tpl_to_merge_props(tpl, props_map, key),
//                 _ => {
//                     panic!("Type of value not recognized: {:?}", key_value.value)
//                 }
//             }
//         }
//         _ => {}
//     };
// }

// fn push_object_to_merge_props(
//     obj: &ObjectLit,
//     props_map: &mut IndexMap<String, Expr>,
//     key: &String,
// ) {
//     let sub_props = process_and_merge_object_lit(obj);

//     push_to_props_map(
//         props_map,
//         key.clone(),
//         Box::new(object_expression_factory(sub_props).expect("Object expression factory failed")),
//     );
// }

// fn push_ident_to_merge_props(ident: &Ident, props_map: &mut IndexMap<String, Expr>, key: &String) {
//     let var_decl = get_var_decl_by_ident(&ident);

//     if let Some(var_decl) = &var_decl {
//         let var_decl_expr = get_expr_from_var_decl(var_decl);

//         let props = match &var_decl_expr {
//             Expr::Object(object) => object_to_value_expr_map(object, key),
//             Expr::Ident(ident) => ident_to_value_expr_map(ident, Some(key.clone())),
//             Expr::Lit(ident) => push_lit_to_props_expr_map(ident, Some(key.clone())),
//             _ => {
//                 todo!("Variable type not supported")
//             }
//         };

//         for (key, value) in props {
//             push_to_props_map(props_map, key, Box::new(value));
//         }
//     }
// }

fn prop_name_eq(a: &PropName, b: &PropName) -> bool {
    match (a, b) {
        (PropName::Ident(a), PropName::Ident(b)) => a.sym == b.sym,
        (PropName::Str(a), PropName::Str(b)) => a.value == b.value,
        (PropName::Num(a), PropName::Num(b)) => (a.value - b.value).abs() < std::f64::EPSILON,

        (PropName::BigInt(a), PropName::BigInt(b)) => a.value == b.value,
        // Add more cases as needed
        _ => false,
    }
}

// pub(crate) fn deep_merge_props(
//     props1: &Vec<PropOrSpread>,
//     props2: &Vec<PropOrSpread>,
// ) -> ObjectLit {
//     let mut props = vec![];

//     for prop1 in props1 {
//         if let PropOrSpread::Prop(prop1) = prop1 {
//             if let Prop::KeyValue(kv1) = &**prop1 {
//                 if let Some(PropOrSpread::Prop(prop2)) = props2.iter().find(|prop2| {
//                     if let PropOrSpread::Prop(prop2) = prop2 {
//                         if let Prop::KeyValue(kv2) = &**prop2 {
//                             prop_name_eq(&kv1.key, &kv2.key)
//                         } else {
//                             false
//                         }
//                     } else {
//                         false
//                     }
//                 }) {
//                     if let Prop::KeyValue(kv2) = &**prop2 {
//                         if let Expr::Object(obj1) = &*kv1.value {
//                             if let Expr::Object(obj2) = &*kv2.value {
//                                 props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
//                                     KeyValueProp {
//                                         key: kv1.key.clone(),
//                                         value: Box::new(Expr::Object(deep_merge_props(
//                                             props1, props2,
//                                         ))),
//                                     },
//                                 ))));
//                                 continue;
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         props.push(prop1.clone());
//     }

//     for prop2 in props2 {
//         if !props.iter().any(|prop1| {
//             if let PropOrSpread::Prop(prop1) = prop1 {
//                 if let Prop::KeyValue(kv1) = &**prop1 {
//                     if let PropOrSpread::Prop(prop2) = prop2 {
//                         if let Prop::KeyValue(kv2) = &**prop2 {
//                             prop_name_eq(&kv1.key, &kv2.key)
//                         } else {
//                             false
//                         }
//                     } else {
//                         false
//                     }
//                 } else {
//                     false
//                 }
//             } else {
//                 false
//             }
//         }) {
//             props.push(prop2.clone());
//         }
//     }

//     ObjectLit {
//         span: DUMMY_SP, // replace with the appropriate span
//         props,
//     }
// }

pub(crate) fn remove_duplicates(props: Vec<PropOrSpread>) -> Vec<PropOrSpread> {
    let mut set = HashSet::new();
    let mut result = vec![];

    for prop in props.into_iter().rev() {
        let key = match &prop {
            PropOrSpread::Prop(prop) => match prop.as_ref().clone() {
                Prop::Shorthand(ident) => ident.sym.clone(),
                Prop::KeyValue(kv) => match kv.clone().key {
                    PropName::Ident(ident) => ident.sym.clone(),
                    PropName::Str(str_) => str_.value.clone(),
                    _ => continue,
                },
                _ => continue,
            },
            _ => continue,
        };

        if set.insert(key) {
            result.push(prop);
        }
    }

    result.reverse();

    result
}

pub(crate) fn deep_merge_props(
    old_props: Vec<PropOrSpread>,
    mut new_props: Vec<PropOrSpread>,
) -> Vec<PropOrSpread> {
    for prop in old_props {
        match prop {
            PropOrSpread::Prop(prop) => match *prop {
                Prop::KeyValue(mut kv) => {
                    if new_props.iter().any(|p| match p {
                        PropOrSpread::Prop(p) => match **p {
                            Prop::KeyValue(ref existing_kv) => {
                                prop_name_eq(&kv.key, &existing_kv.key)
                            }
                            _ => false,
                        },
                        _ => false,
                    }) {
                        if let Expr::Object(ref mut obj1) = *kv.value {
                            new_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                KeyValueProp {
                                    key: kv.key.clone(),
                                    value: Box::new(Expr::Object(ObjectLit {
                                        span: DUMMY_SP,
                                        props: deep_merge_props(
                                            obj1.props.clone(),
                                            obj1.props.clone(),
                                        ),
                                    })),
                                },
                            ))));
                        }
                    } else {
                        new_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(kv))));
                    }
                }
                _ => new_props.push(PropOrSpread::Prop(Box::new(*prop))),
            },
            _ => new_props.push(prop),
        }
    }

    remove_duplicates(new_props.into_iter().rev().collect())
}
