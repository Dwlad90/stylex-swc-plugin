#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum VarDeclAction {
  Increase,
  Reduce,
  None,
}

#[derive(Debug, PartialEq)]
pub enum BinaryExprType {
  Number(f64),
  String(String),
  Null,
}
