#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum VarDeclAction {
  Increase,
  Reduce,
  None,
}

pub enum BinaryExprType {
  Number(f64),
  String(String),
  Null,
}
