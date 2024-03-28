use std::sync::atomic::{AtomicUsize, Ordering};

use swc_core::ecma::ast::{Expr, Ident, KeyValueProp, Prop, PropName, PropOrSpread};

use crate::shared::{constants, utils::css::factories::object_expression_factory};

static NUMBER: AtomicUsize = AtomicUsize::new(1);

fn uuid() -> String {
    let number = NUMBER.fetch_add(1, Ordering::SeqCst);
    format!("__included_{}__", number)
}

pub(crate) fn stylex_include(args: Vec<Expr>) -> Expr {
    if args.len() > 1 {
        panic!("{}", constants::messages::ILLEGAL_ARGUMENT_LENGTH);
    }

    let first_arg = &args[0];

    let prop = Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::from(uuid().as_str())),
        value: Box::new(first_arg.clone()),
    });
    dbg!(&prop, &args);

    object_expression_factory(vec![PropOrSpread::Prop(Box::new(prop))]).unwrap()
}
