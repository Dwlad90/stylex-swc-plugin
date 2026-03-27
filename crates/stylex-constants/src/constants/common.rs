pub static DEFAULT_INJECT_PATH: &str = "@stylexjs/stylex/lib/stylex-inject";
use once_cell::sync::Lazy;
use phf::phf_set;

// Using MDN data as a source of truth to populate the above sets
// by group in alphabetical order:

pub static VALID_CALLEES: phf::Set<&'static str> = phf_set! {

  "String", "Number", "Math", "Object", "Array"
};

pub static MUTATING_ARRAY_METHODS: phf::Set<&'static str> = phf_set! {
  "push",
  "pop",
  "shift",
  "unshift",
  "splice",
  "sort",
  "reverse",
  "fill",
  "copyWithin",
};

pub static MUTATING_OBJECT_METHODS: phf::Set<&'static str> = phf_set! {
  "assign",
  "defineProperty",
  "defineProperties",
  "setPrototypeOf",
};

pub static INVALID_METHODS: phf::Set<&'static str> = phf_set! {
  "random",
  "assign",
  "defineProperties",
  "defineProperty",
  "freeze",
  "seal",
  "splice",
};

pub static COMPILED_KEY: &str = "$$css";

pub static SPLIT_TOKEN: &str = "__$$__";

pub static ROOT_FONT_SIZE: i8 = 16;

pub static VAR_GROUP_HASH_KEY: &str = "__varGroupHash__";

pub static COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES: Lazy<[&str; 9]> = Lazy::new(|| {
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

pub static COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES: Lazy<[&str; 7]> =
  Lazy::new(|| [" a ", " b ", " c ", " l ", " h ", " s ", " w "]);

pub static CSS_CONTENT_FUNCTIONS: Lazy<[&str; 7]> = Lazy::new(|| {
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

pub static CSS_CONTENT_KEYWORDS: Lazy<[&str; 11]> = Lazy::new(|| {
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
pub static VALID_POSITION_TRY_PROPERTIES: Lazy<[&str; 40]> = Lazy::new(|| {
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
pub static VALID_VIEW_TRANSITION_CLASS_PROPERTIES: Lazy<[&str; 4]> =
  Lazy::new(|| ["group", "imagePair", "old", "new"]);

pub static CONSTS_FILE_EXTENSION: &str = ".const";

/// CSS custom property used in the logical float/clear value polyfill system.
/// Represents the logical "start" direction for float/clear, which maps to the physical
/// direction "left" in left-to-right (LTR) text direction and "right" in right-to-left (RTL).
/// This variable should be defined on the root or relevant container elements, typically by
/// a polyfill or runtime logic that sets its value based on the current text direction.
///
/// This ensures that logical float/clear values behave correctly in both LTR and RTL contexts.pub static LOGICAL_FLOAT_START_VAR: &str = "--stylex-logical-start";
pub static LOGICAL_FLOAT_START_VAR: &str = "--stylex-logical-start";

/// CSS custom property used in the logical float/clear value polyfill system.
/// Represents the logical "end" direction for float/clear, which maps to the physical
/// direction "right" in left-to-right (LTR) text direction and "left" in right-to-left (RTL).
/// This variable should be defined on the root or relevant container elements, typically by
/// a polyfill or runtime logic that sets its value based on the current text direction.
///
/// This ensures that logical float/clear values behave correctly in both LTR and RTL contexts.
pub static LOGICAL_FLOAT_END_VAR: &str = "--stylex-logical-end";

/// List of valid JSX runtime call names to check against when determining if a call expression is a JSX runtime call.
/// This includes both the classic React.createElement and the newer jsx/jsxs calls, as well as their DEV variants.
///
/// Supported:
/// - React runtime: `_jsx`, `_jsxs`
/// - React classic: `createElement`, `React.createElement`
/// - Vue runtime: `_createElementBlock`, `_createElementVNode`, `_createVNode`
///
///   The DEV variants (e.g., `_jsxDEV`) are also included by checking for a "DEV" suffix,
///   which is commonly used in development builds of React to provide additional debugging information.
pub static RUNTIME_JSX_CALL_NAMES: &[&str] = &[
  "jsx",
  "jsxs",
  "createElement",
  "createElementBlock",
  "createElementVNode",
  "createVNode",
];
