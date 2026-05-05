#[derive(Debug, PartialEq)]
pub enum BinaryExprType {
  Number(f64),
  String(String),
  Null,
}
