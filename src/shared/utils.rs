use radix_fmt::radix;
use swc_core::{
    common::{FileName, DUMMY_SP},
    ecma::ast::{
        Bool, Expr, ExprOrSpread, Ident, KeyValueProp, Lit, Number, ObjectLit, Pat, Prop, PropName,
        PropOrSpread,
    },
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
        _ => panic!("Type of lit doesn't support"),
    }
}

pub(crate) fn get_key_str(key_value: &KeyValueProp) -> String {
    let key = &key_value.key;
    let mut should_wrap_in_quotes = false;

    let key = match key {
        PropName::Ident(ident) => &*ident.sym,
        PropName::Str(str) => {
            should_wrap_in_quotes = true;

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
