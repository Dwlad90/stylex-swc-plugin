use radix_fmt::radix;
use swc_core::{
    common::{FileName, DUMMY_SP},
    ecma::ast::{Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread},
};

pub fn object_expression_factory(props: Vec<PropOrSpread>) -> Option<Expr> {
    Some(Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
    }))
}

pub fn prop_or_spread_string_creator(key: &str, value: String) -> PropOrSpread {
    let value = string_to_expression(value);

    match value {
        Some(value) => PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new(key.into(), DUMMY_SP)),
            value: Box::new(value),
        }))),
        None => panic!("Value is not a string"),
    }
}

// Converts a string to an expression.
pub fn string_to_expression(value: String) -> Option<Expr> {
    Option::Some(Expr::Lit(Lit::Str(value.into())))
}

pub fn extract_filename_from_path(path: FileName) -> String {
    match path {
        FileName::Real(path_buf) => path_buf.file_stem().unwrap().to_str().unwrap().to_string(),
        _ => panic!("Not a real file name"),
    }
}

pub fn hash_css(value: &str) -> String {
    radix(murmur2::murmur2(value.as_bytes(), 1), 36).to_string()
}
