/// A global the compiler folds when it is *called* — `String(x)`.
///
/// The only enum of its kind left here: what may be called *on* a global is no
/// longer a list of names, because the statics of `Math` and `Object` are
/// evaluated as JavaScript rather than matched against a table.
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum CallableGlobalJS {
  String,
  Number,
  Array,
  Object,
}

impl CallableGlobalJS {
  /// The name the global is called by — the inverse of [`TryFrom<&str>`], so a
  /// diagnostic naming the callee reads the name off the enum rather than
  /// repeating a literal that can drift from it.
  pub fn name(self) -> &'static str {
    match self {
      CallableGlobalJS::String => "String",
      CallableGlobalJS::Number => "Number",
      CallableGlobalJS::Array => "Array",
      CallableGlobalJS::Object => "Object",
    }
  }
}

impl TryFrom<&str> for CallableGlobalJS {
  type Error = ();

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "String" => Ok(CallableGlobalJS::String),
      "Number" => Ok(CallableGlobalJS::Number),
      "Array" => Ok(CallableGlobalJS::Array),
      "Object" => Ok(CallableGlobalJS::Object),
      _ => Err(()),
    }
  }
}

#[cfg(test)]
#[path = "tests/js_test.rs"]
mod tests;
