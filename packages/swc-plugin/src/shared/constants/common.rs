pub(crate) static DEFAULT_INJECT_PATH: &str = "@stylexjs/stylex/lib/stylex-inject";
use phf::phf_set;

// Using MDN data as a source of truth to populate the above sets
// by group in alphabetical order:

pub(crate) static VALID_CALLEES: phf::Set<&'static str> = phf_set! {

  "String", "Number", "Math", "Object", "Array"
};

pub(crate) static INVALID_METHODS: phf::Set<&'static str> = phf_set! {
  "random",
  "assign",
  "defineProperties",
  "defineProperty",
  "freeze",
  "seal",
  "splice",
};

pub(crate) static COMPILED_KEY: &str = "$$css";

pub(crate) static SPLIT_TOKEN: &str = "__$$__";

pub(crate) static ROOT_FONT_SIZE: i8 = 16;
