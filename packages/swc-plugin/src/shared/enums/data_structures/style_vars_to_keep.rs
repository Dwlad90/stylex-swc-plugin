use swc_ecma_ast::Id;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum NonNullProp {
  Id(Id),
  True,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum NonNullProps {
  Vec(Vec<Id>),
  True,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct StyleVarsToKeep(
  pub(crate) Id,
  pub(crate) NonNullProp,
  pub(crate) NonNullProps,
);
