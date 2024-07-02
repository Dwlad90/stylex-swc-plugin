#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum VarDeclAction {
  Increase,
  Reduce,
  None,
}
