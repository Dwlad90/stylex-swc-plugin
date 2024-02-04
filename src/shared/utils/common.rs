use std::{
    any::{type_name, Any},
    collections::HashMap,
};

use radix_fmt::radix;
use swc_core::{
    common::{FileName, DUMMY_SP},
    ecma::ast::{
        BinExpr, BinaryOp, Bool, Expr, ExprOrSpread, Id, Ident, KeyValueProp, Lit, Number,
        ObjectLit, Pat, Prop, PropName, PropOrSpread, Tpl, UnaryExpr, UnaryOp, VarDeclarator,
    },
};

use crate::shared::constants::{
    self,
    messages::{ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE},
};

pub(crate) fn object_expression_factory(props: Vec<PropOrSpread>) -> Option<Expr> {
    Some(Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
    }))
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

pub(crate) fn push_css_anchor_prop(object: &mut swc_core::ecma::ast::ObjectLit) {
    object.props.push(prop_or_spread_boolean_creator(
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
) -> Option<&'a VarDeclarator> {
    reduce_ident_count(var_dec_count_map, &ident);

    get_var_decl_from(declarations, ident)
}

fn get_var_decl_from<'a>(
    declarations: &'a Vec<VarDeclarator>,
    ident: &'a Ident,
) -> Option<&'a VarDeclarator> {
    declarations.iter().find(|var_declarator| {
        if let Pat::Ident(binding_indent) = &var_declarator.name {
            return binding_indent.sym == ident.sym;
        }

        return false;
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
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> f32 {
    match &expr_num {
        Expr::Ident(ident) => ident_to_number(&ident, declarations, var_dec_count_map),
        Expr::Lit(lit) => lit_to_num(&lit),
        Expr::Unary(unary) => unari_to_num(&unary, declarations, var_dec_count_map),
        Expr::Bin(lit) => binary_expr_to_num(&lit, declarations, var_dec_count_map),
        _ => panic!("Expression in not a number {:?}", expr_num),
    }
}

pub fn unari_to_num(
    unary_expr: &UnaryExpr,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> f32 {
    let arg = unary_expr.arg.as_ref();
    let op = unary_expr.op;

    match &op {
        UnaryOp::Minus => expr_to_num(arg, declarations, var_dec_count_map) * -1.0,
        UnaryOp::Plus => expr_to_num(arg, declarations, var_dec_count_map),
        _ => panic!("Union operation '{}' is invalid", op),
    }
}

pub fn binary_expr_to_num(
    binary_expr: &BinExpr,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> f32 {
    let binary_expr = binary_expr.clone();

    let op = binary_expr.op;
    let left = binary_expr.left.as_ref();
    let right = binary_expr.right.as_ref();

    match &op {
        BinaryOp::Add => {
            expr_to_num(left, declarations, var_dec_count_map)
                + expr_to_num(right, declarations, var_dec_count_map)
        }
        BinaryOp::Sub => {
            expr_to_num(left, declarations, var_dec_count_map)
                - expr_to_num(right, declarations, var_dec_count_map)
        }
        BinaryOp::Mul => {
            expr_to_num(left, declarations, var_dec_count_map)
                * expr_to_num(right, declarations, var_dec_count_map)
        }
        BinaryOp::Div => {
            expr_to_num(left, declarations, var_dec_count_map)
                / expr_to_num(right, declarations, var_dec_count_map)
        }
        _ => panic!("Operator '{}' is not supported", op),
    }
}

pub fn ident_to_number(
    ident: &Ident,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> f32 {
    // 1. Get the variable declaration
    let var_decl = get_var_decl_by_ident(&ident, declarations, var_dec_count_map);

    // 2. Check if it is a variable
    match &var_decl {
        Some(var_decl) => {
            // 3. Do the correct conversion according to the expression
            let var_decl_expr = get_expr_from_var_decl(var_decl);

            match &var_decl_expr {
                Expr::Bin(bin_expr) => {
                    binary_expr_to_num(&bin_expr, declarations, var_dec_count_map)
                }
                Expr::Unary(unary_expr) => {
                    unari_to_num(&unary_expr, declarations, var_dec_count_map)
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
        Lit::Num(num) => num.value as f32,
        _ => panic!("Value in not a number"),
    }
}

pub fn handle_tpl_to_expression(
    tpl: &swc_core::ecma::ast::Tpl,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> Expr {
    // Clone the template, so we can work on it
    let mut tpl = tpl.clone();

    // Loop through each expression in the template
    for expr in tpl.exprs.iter_mut() {
        // Check if the expression is an identifier
        if let Expr::Ident(ident) = expr.as_ref() {
            // Find the variable declaration for this identifier in the AST
            let var_decl = get_var_decl_by_ident(&ident, declarations, var_dec_count_map);

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
) -> String {
    let mut tpl_str = String::new();

    for (i, quasi) in tpl.quasis.iter().enumerate() {
        tpl_str.push_str(&quasi.raw.as_ref());

        if i < tpl.exprs.len() {
            match &tpl.exprs[i].as_ref() {
                Expr::Ident(ident) => {
                    let ident = get_var_decl_by_ident(ident, declarations, var_dec_count_map);

                    match ident {
                        Some(var_decl) => {
                            let var_decl_expr = get_expr_from_var_decl(var_decl);

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
                    transform_bin_expr_to_number(bin, declarations, var_dec_count_map)
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
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> f32 {
    let op = bin.op;
    let left = bin.left.as_ref();
    let right = bin.right.as_ref();

    let left = expr_to_num(left, declarations, var_dec_count_map);
    let right = expr_to_num(right, declarations, var_dec_count_map);

    let result = evaluate_bin_expr(op, left, right);

    result
}

pub(crate) fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
