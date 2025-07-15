pub(crate) static DEFAULT_INJECT_PATH: &str = "@stylexjs/stylex/lib/stylex-inject";
use once_cell::sync::Lazy;
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

pub(crate) static THEME_NAME_KEY: &str = "__varGroupHash__";

pub(crate) static COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES: Lazy<[&str; 9]> =
  Lazy::new(|| {
    [
      "oklch",
      "lch",
      "oklab",
      "hsla",
      "radial-gradient",
      "hwb",
      "lab",
      "clamp",
      "hsl",
    ]
  });

pub(crate) static COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES: Lazy<[&str; 7]> =
  Lazy::new(|| [" a ", " b ", " c ", " l ", " h ", " s ", " w "]);

pub(crate) static CSS_CONTENT_FUNCTIONS: Lazy<[&str; 7]> = Lazy::new(|| {
  [
    "attr(",
    "counter(",
    "counters(",
    "url(",
    "linear-gradient(",
    "image-set(",
    "var(--",
  ]
});

pub(crate) static CSS_CONTENT_KEYWORDS: Lazy<[&str; 11]> = Lazy::new(|| {
  [
    "normal",
    "none",
    "open-quote",
    "close-quote",
    "no-open-quote",
    "no-close-quote",
    "inherit",
    "initial",
    "revert",
    "revert-layer",
    "unset",
  ]
});

// TODO: Once we have a reliable validator, these property checks should be replaced with
// validators that can also validate the values.
pub(crate) static VALID_POSITION_TRY_PROPERTIES: Lazy<[&str; 40]> = Lazy::new(|| {
  [
    // anchor Properties
    "anchorName",
    // position Properties
    "positionAnchor",
    "positionArea",
    // inset Properties
    "top",
    "right",
    "bottom",
    "left",
    "inset",
    "insetBlock",
    "insetBlockEnd",
    "insetBlockStart",
    "insetInline",
    "insetInlineEnd",
    "insetInlineStart",
    // margin Properties
    "margin",
    "marginBlock",
    "marginBlockEnd",
    "marginBlockStart",
    "marginInline",
    "marginInlineEnd",
    "marginInlineStart",
    "marginTop",
    "marginBottom",
    "marginLeft",
    "marginRight",
    // size properties
    "width",
    "height",
    "minWidth",
    "minHeight",
    "maxWidth",
    "maxHeight",
    "blockSize",
    "inlineSize",
    "minBlockSize",
    "minInlineSize",
    "maxBlockSize",
    "maxInlineSize",
    // self alignment properties
    "alignSelf",
    "justifySelf",
    "placeSelf",
  ]
});

// Validation of `stylex.viewTransitionClass` function call
pub(crate) static VALID_VIEW_TRANSITION_CLASS_PROPERTIES: Lazy<[&str; 4]> =
  Lazy::new(|| ["group", "imagePair", "old", "new"]);
