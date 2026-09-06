#[derive(Debug, PartialEq)]
pub enum BinaryExprType {
  Number(f64),
  /// A concatenation, with the count it was measured to.
  ///
  /// The count travels with the text so a chain of `+` measures each operand
  /// once: the link above adopts this buffer rather than reading the length of
  /// everything already joined into it. Without it the accumulated left side is
  /// re-read at every link, which costs the square of a chain's text rather
  /// than its length.
  String {
    text: String,
    /// UTF-16 code units of `text` -- the length JavaScript reports, and the
    /// unit the character ceiling is spent in.
    units: usize,
  },
  Null,
}
