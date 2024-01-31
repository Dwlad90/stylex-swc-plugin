

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct OrderPair<'a>(pub &'a str, pub Option<&'a str>);
