use std::{any::type_name, collections::HashSet};

use radix_fmt::radix;
use swc_core::{
    common::{FileName, Span, DUMMY_SP},
    ecma::{
        ast::{
            BinExpr, BinaryOp, BindingIdent, Bool, Decl, Expr, ExprOrSpread, Ident, KeyValueProp,
            Lit, Module, ModuleDecl, ModuleItem, Number, ObjectLit, Pat, Prop, PropName,
            PropOrSpread, Stmt, Str, Tpl, UnaryExpr, UnaryOp, VarDeclarator,
        },
        visit::{Fold, FoldWith},
    },
};

use crate::shared::{
    constants::{self, messages::ILLEGAL_PROP_VALUE},
    enums::{TopLevelExpression, TopLevelExpressionKind, VarDeclAction},
    regex::DASHIFY_REGEX,
    structures::{
        functions::{FunctionMap, FunctionType},
        state_manager::StateManager,
    },
};

use super::css::stylex::evaluate::{evaluate, evaluate_cached, State};

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
        key: string_to_prop_name(key).unwrap(),
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

// Converts a string to an expression.
pub(crate) fn string_to_prop_name(value: String) -> Option<PropName> {
    if value.contains(".") {
        Some(PropName::Str(Str {
            span: DUMMY_SP,
            value: value.clone().into(),
            raw: None,
        }))
    } else {
        Some(PropName::Ident(Ident::new(value.clone().into(), DUMMY_SP)))
    }
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
        _ => "UnknownFile".to_string(),
    }
}

pub(crate) fn hash_css(value: &str) -> String {
    radix(murmur2::murmur2(value.as_bytes(), 1), 36).to_string()
}

pub(crate) fn get_string_val_from_lit(value: &Lit) -> String {
    match value {
        Lit::Str(str) => format!("{}", str.value),
        Lit::Num(num) => format!("{}", num.value),
        Lit::Null(_) => "".to_string(),
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

// pub(crate) fn push_css_anchor_prop(props: &mut Vec<PropOrSpread>) {
//     props.push(prop_or_spread_boolean_creator(
//         "$$css".to_string(),
//         Option::Some(true),
//     ))
// }

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
    let expr = Box::new(string_to_expression(value).expect(constants::messages::NON_STATIC_VALUE));

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

pub fn reduce_ident_count<'a>(state: &'a mut StateManager, ident: &'a Ident) {
    *state.var_decl_count_map.entry(ident.to_id()).or_insert(1) -= 1;
}

pub fn increase_ident_count(state: &mut StateManager, ident: &Ident) {
    let ident_id = &ident.to_id();
    *state
        .var_decl_count_map
        .entry(ident_id.clone())
        .or_insert(-1) += 1;
}

pub fn get_var_decl_by_ident<'a>(
    ident: &'a Ident,
    state: &'a mut StateManager,
    functions: &'a FunctionMap,
    action: VarDeclAction,
) -> Option<VarDeclarator> {
    match action {
        VarDeclAction::Increase => increase_ident_count(state, ident),
        VarDeclAction::Reduce => reduce_ident_count(state, &ident),
        VarDeclAction::None => {}
    };

    match get_var_decl_from(state, ident) {
        Some(var_decl) => Some(var_decl.clone()),
        None => {
            let func = functions.identifiers.get(&ident.to_id());

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
    state: &'a StateManager,
    ident: &'a Ident,
) -> Option<&'a VarDeclarator> {
    state.declarations.iter().find(|var_declarator| {
        if let Pat::Ident(binding_indent) = &var_declarator.name {
            return binding_indent.sym == ident.sym;
        }

        false
    })
}

pub(crate) fn get_var_decl_by_ident_or_member<'a>(
    state: &'a StateManager,
    ident: &'a Ident,
) -> Option<&'a VarDeclarator> {
    dbg!(&state.declarations);
    state.declarations.iter().find(|var_declarator| {
        if let Pat::Ident(binding_indent) = &var_declarator.name {
            if binding_indent.sym == ident.sym {
                return true;
            }
        }

        var_declarator
            .init
            .as_ref()
            .and_then(|init| init.as_call())
            .and_then(|call| call.callee.as_expr())
            .and_then(|callee| callee.as_member())
            .and_then(|member| member.prop.as_ident())
            .map_or(false, |member_ident| member_ident.sym == ident.sym)
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

pub fn expr_to_num(expr_num: &Expr, traversal_state: &mut StateManager) -> f32 {
    match &expr_num {
        Expr::Ident(ident) => ident_to_number(&ident, traversal_state, &FunctionMap::default()),
        Expr::Lit(lit) => lit_to_num(&lit),
        Expr::Unary(unary) => unari_to_num(&unary, traversal_state),
        Expr::Bin(lit) => {
            dbg!(&traversal_state.var_decl_count_map);

            let mut state = State::new(traversal_state);

            match binary_expr_to_num(&lit, &mut state) {
                Some(result) => result,
                None => panic!("Binary expression is not a number"),
            }
        }
        _ => panic!("Expression in not a number {:?}", expr_num),
    }
}

fn ident_to_string(ident: &Ident, state: &mut StateManager, functions: &FunctionMap) -> String {
    let var_decl = get_var_decl_by_ident(&ident, state, functions, VarDeclAction::Reduce);

    println!("var_decl: {:?}, ident: {:?}", var_decl, ident);

    match &var_decl {
        Some(var_decl) => {
            let var_decl_expr = get_expr_from_var_decl(var_decl);

            match &var_decl_expr {
                Expr::Lit(lit) => get_string_val_from_lit(&lit),
                Expr::Ident(ident) => ident_to_string(ident, state, functions),
                _ => panic!("{}", ILLEGAL_PROP_VALUE),
            }
        }
        None => panic!("{}", ILLEGAL_PROP_VALUE),
    }
}

pub fn expr_to_str(
    expr_string: &Expr,
    state: &mut StateManager,
    functions: &FunctionMap,
) -> String {
    match &expr_string {
        Expr::Ident(ident) => ident_to_string(&ident, state, functions),
        Expr::Lit(lit) => get_string_val_from_lit(&lit),
        _ => panic!("Expression in not a string {:?}", expr_string),
    }
}

pub fn unari_to_num(unary_expr: &UnaryExpr, state: &mut StateManager) -> f32 {
    let arg = unary_expr.arg.as_ref();
    let op = unary_expr.op;

    match &op {
        UnaryOp::Minus => expr_to_num(arg, state) * -1.0,
        UnaryOp::Plus => expr_to_num(arg, state),
        _ => panic!("Union operation '{}' is invalid", op),
    }
}

pub fn binary_expr_to_num(binary_expr: &BinExpr, state: &mut State) -> Option<f32> {
    let binary_expr = binary_expr.clone();

    let op = binary_expr.op;
    let Some(left) = evaluate_cached(&*binary_expr.left, state) else {
        panic!("Left expression is not a number")
    };

    let Some(right) = evaluate_cached(&*binary_expr.right, state) else {
        panic!("Left expression is not a number")
    };

    let result = match &op {
        BinaryOp::Add => {
            expr_to_num(left.as_expr()?, &mut state.traversal_state)
                + expr_to_num(right.as_expr()?, &mut state.traversal_state)
        }
        BinaryOp::Sub => {
            expr_to_num(left.as_expr()?, &mut state.traversal_state)
                - expr_to_num(right.as_expr()?, &mut state.traversal_state)
        }
        BinaryOp::Mul => {
            expr_to_num(left.as_expr()?, &mut state.traversal_state)
                * expr_to_num(right.as_expr()?, &mut state.traversal_state)
        }
        BinaryOp::Div => {
            expr_to_num(left.as_expr()?, &mut state.traversal_state)
                / expr_to_num(right.as_expr()?, &mut state.traversal_state)
        }
        BinaryOp::Mod => {
            expr_to_num(left.as_expr()?, &mut state.traversal_state)
                % expr_to_num(right.as_expr()?, &mut state.traversal_state)
        }
        BinaryOp::Exp => expr_to_num(left.as_expr()?, &mut state.traversal_state)
            .powf(expr_to_num(right.as_expr()?, &mut state.traversal_state)),
        BinaryOp::RShift => {
            ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
                >> expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32)
                as f32
        }
        BinaryOp::LShift => {
            ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
                << expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32)
                as f32
        }
        BinaryOp::BitAnd => {
            ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
                & expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32)
                as f32
        }
        BinaryOp::BitOr => {
            ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
                | expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32)
                as f32
        }
        BinaryOp::BitXor => {
            ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
                ^ expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32)
                as f32
        }
        BinaryOp::In => {
            if expr_to_num(right.as_expr()?, &mut state.traversal_state) == 0.0 {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::InstanceOf => {
            if expr_to_num(right.as_expr()?, &mut state.traversal_state) == 0.0 {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::EqEq => {
            if expr_to_num(left.as_expr()?, &mut state.traversal_state)
                == expr_to_num(right.as_expr()?, &mut state.traversal_state)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::NotEq => {
            if expr_to_num(left.as_expr()?, &mut state.traversal_state)
                != expr_to_num(right.as_expr()?, &mut state.traversal_state)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::EqEqEq => {
            if expr_to_num(left.as_expr()?, &mut state.traversal_state)
                == expr_to_num(right.as_expr()?, &mut state.traversal_state)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::NotEqEq => {
            if expr_to_num(left.as_expr()?, &mut state.traversal_state)
                != expr_to_num(right.as_expr()?, &mut state.traversal_state)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::Lt => {
            if expr_to_num(left.as_expr()?, &mut state.traversal_state)
                < expr_to_num(right.as_expr()?, &mut state.traversal_state)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::LtEq => {
            if expr_to_num(left.as_expr()?, &mut state.traversal_state)
                <= expr_to_num(right.as_expr()?, &mut state.traversal_state)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::Gt => {
            if expr_to_num(left.as_expr()?, &mut state.traversal_state)
                > expr_to_num(right.as_expr()?, &mut state.traversal_state)
            {
                1.0
            } else {
                0.0
            }
        }
        BinaryOp::GtEq => {
            if expr_to_num(left.as_expr()?, &mut state.traversal_state)
                >= expr_to_num(right.as_expr()?, &mut state.traversal_state)
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

            let result = evaluate_cached(left.as_expr()?, state);

            let left = result.unwrap();
            let left = left.as_expr().unwrap();

            left_confident = state.confident;

            state.confident = was_confident;

            let result = evaluate_cached(right.as_expr()?, state);

            let right = result.unwrap();
            let right = right.as_expr().unwrap();
            right_confident = state.confident;

            let left = expr_to_num(left, &mut state.traversal_state);
            let right = expr_to_num(right, &mut state.traversal_state);

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

            let result = evaluate_cached(left.as_expr()?, state);

            let left = result.unwrap();
            let left = left.as_expr().unwrap();

            left_confident = state.confident;

            state.confident = was_confident;

            let result = evaluate_cached(right.as_expr()?, state);

            let right = result.unwrap();
            let right = right.as_expr().unwrap();
            right_confident = state.confident;

            let left = expr_to_num(left, &mut state.traversal_state);
            let right = expr_to_num(right, &mut state.traversal_state);

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
            let was_confident = state.confident;
            let mut left_confident = state.confident;
            let mut right_confident = state.confident;

            let result = evaluate_cached(left.as_expr()?, state);

            let left = result.unwrap();
            let left = left.as_expr().unwrap();

            left_confident = state.confident;

            state.confident = was_confident;

            let result = evaluate_cached(right.as_expr()?, state);

            let right = result.unwrap();
            let right = right.as_expr().unwrap();
            right_confident = state.confident;

            let left = expr_to_num(left, &mut state.traversal_state);
            let right = expr_to_num(right, &mut state.traversal_state);

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
            ((expr_to_num(left.as_expr()?, &mut state.traversal_state) as i32)
                >> expr_to_num(right.as_expr()?, &mut state.traversal_state) as i32)
                as f32
        }
    };

    Option::Some(result)
}

pub fn ident_to_number(
    ident: &Ident,
    traveral_state: &mut StateManager,
    functions: &FunctionMap,
) -> f32 {
    // 1. Get the variable declaration
    let var_decl = get_var_decl_by_ident(&ident, traveral_state, functions, VarDeclAction::Reduce);

    // 2. Check if it is a variable
    match &var_decl {
        Some(var_decl) => {
            // 3. Do the correct conversion according to the expression
            let var_decl_expr = get_expr_from_var_decl(var_decl);

            let mut state: State = State::new(&traveral_state);

            match &var_decl_expr {
                Expr::Bin(bin_expr) => match binary_expr_to_num(&bin_expr, &mut state) {
                    Some(result) => result,
                    None => panic!("Binary expression is not a number"),
                },
                Expr::Unary(unary_expr) => unari_to_num(&unary_expr, traveral_state),
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
    state: &mut StateManager,
    functions: &FunctionMap,
) -> Expr {
    // Clone the template, so we can work on it
    let mut tpl = tpl.clone();

    // Loop through each expression in the template
    for expr in tpl.exprs.iter_mut() {
        // Check if the expression is an identifier
        if let Expr::Ident(ident) = expr.as_ref() {
            // Find the variable declaration for this identifier in the AST
            let var_decl = get_var_decl_by_ident(&ident, state, functions, VarDeclAction::Reduce);

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

pub fn expr_tpl_to_string(tpl: &Tpl, state: &mut StateManager, functions: &FunctionMap) -> String {
    let mut tpl_str: String = String::new();

    for (i, quasi) in tpl.quasis.iter().enumerate() {
        tpl_str.push_str(&quasi.raw.as_ref());

        if i < tpl.exprs.len() {
            match &tpl.exprs[i].as_ref() {
                Expr::Ident(ident) => {
                    let ident =
                        get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce);

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
                    transform_bin_expr_to_number(bin, state)
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

pub fn transform_bin_expr_to_number(bin: &BinExpr, traversal_state: &mut StateManager) -> f32 {
    let mut state = State::new(traversal_state);
    let op = bin.op;
    let Some(left) = evaluate_cached(&*bin.left, &mut state) else {
        panic!("Left expression is not a number")
    };

    let Some(right) = evaluate_cached(&*bin.right, &mut state) else {
        panic!("Left expression is not a number")
    };
    let left = expr_to_num(left.as_expr().unwrap(), traversal_state);
    let right = expr_to_num(right.as_expr().unwrap(), traversal_state);

    let result = evaluate_bin_expr(op, left, right);

    result
}

pub(crate) fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// pub fn get_value_as_string_from_ident(
//     value_ident: &Ident,
//     declarations: &Vec<VarDeclarator>,
//     var_dec_count_map: &mut HashMap<Id, i8>,
// ) -> String {
//     reduce_ident_count(var_dec_count_map, &value_ident);

//     let var_decl = get_var_decl_from(declarations, &value_ident);

//     match &var_decl {
//         Some(var_decl) => {
//             let var_decl_expr = get_expr_from_var_decl(var_decl);

//             match &var_decl_expr {
//                 Expr::Lit(lit) => get_string_val_from_lit(lit),
//                 Expr::Ident(ident) => {
//                     get_value_as_string_from_ident(ident)
//                 }
//                 _ => panic!("Value type not supported"),
//             }
//         }
//         None => {
//             println!("value_ident: {:?}", value_ident);
//             panic!("Variable not declared")
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

pub(crate) fn get_key_values_from_object(
    object: &swc_core::ecma::ast::ObjectLit,
) -> Vec<KeyValueProp> {
    let mut key_values = vec![];

    for prop in object.props.iter() {
        match prop {
            PropOrSpread::Spread(_) => todo!("Spread operator"),
            PropOrSpread::Prop(prop) => match prop.as_ref() {
                Prop::KeyValue(key_value) => {
                    key_values.push(key_value.clone());
                }
                _ => panic!("{}", constants::messages::ILLEGAL_PROP_VALUE),
            },
        }
    }
    key_values
}

pub(crate) fn dashify(s: &str) -> String {
    let after = DASHIFY_REGEX.replace_all(s, "$1-$2");
    after.to_lowercase()
}

pub(crate) fn fill_top_level_expressions(module: &Module, state: &mut StateManager) {
    module.clone().body.iter().for_each(|item| match &item {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
            if let Decl::Var(decl_var) = &export_decl.decl {
                for decl in &decl_var.decls {
                    if let Some(decl_init) = decl.init.as_ref() {
                        state.top_level_expressions.push(TopLevelExpression(
                            TopLevelExpressionKind::NamedExport,
                            *decl_init.clone(),
                        ));
                        state.declarations.push(decl.clone());
                    }
                }
            }
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(export_decl)) => {
            if let Some(paren) = export_decl.expr.as_paren() {
                state.top_level_expressions.push(TopLevelExpression(
                    TopLevelExpressionKind::DefaultExport,
                    *paren.expr.clone(),
                ));
            } else {
                state.top_level_expressions.push(TopLevelExpression(
                    TopLevelExpressionKind::DefaultExport,
                    *export_decl.expr.clone(),
                ));
            }
        }
        ModuleItem::Stmt(Stmt::Decl(Decl::Var(var))) => {
            for decl in &var.decls {
                if let Some(decl_init) = decl.init.as_ref() {
                    state.top_level_expressions.push(TopLevelExpression(
                        TopLevelExpressionKind::Stmt,
                        *decl_init.clone(),
                    ));
                    state.declarations.push(decl.clone());
                }
            }
        }
        _ => {}
    });
}
