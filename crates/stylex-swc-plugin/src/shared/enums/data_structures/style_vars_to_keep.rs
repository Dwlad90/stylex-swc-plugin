use swc_core::atoms::Atom;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum NonNullProp {
  Atom(Atom),
  True,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum NonNullProps {
  Vec(Vec<Atom>),
  True,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct StyleVarsToKeep(
  pub(crate) Atom,
  pub(crate) NonNullProp,
  pub(crate) NonNullProps,
);
