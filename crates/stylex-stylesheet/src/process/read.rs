//! The rules and the config of `processStylexRules`, as the reference reads
//! them.
//!
//! The reference reads its arguments with plain JavaScript: destructuring, a
//! spread, property reads, `String(...)` and `Number(...)`. The port reads a
//! `JsValue` the same way, so a value outside the published types writes the
//! sheet the reference writes. A read the reference throws on is refused here.
//! Where the reference throws only when a value is used (a class name compared
//! under the legacy sort, an `ltr` text rendered), the value is kept as it is
//! and refused at that use.
//!
//! One case is refused where the reference goes on: an array `ltr` text that
//! the sort reads only as the argument of `localeCompare`, which coerces it.
//! The addon's gate never packs an array, so the export runs the reference
//! for it.

use std::borrow::Cow;
use std::fmt;

use stylex_js::value::{JsValue, MapKey};

use super::MAX_CONSTANT_DEPTH;

/// A rule's class name, which the reference uses three ways: spelled with
/// `String(...)` in a constant's name, as a `Map` key in the dedupe
/// (`SameValueZero`, so `1` and `'1'` are two class names), and as the
/// receiver of `localeCompare` under the legacy sort.
#[derive(Clone, Debug, PartialEq)]
pub struct ClassName(JsValue);

impl ClassName {
  /// `String(className)`.
  pub fn spelling(&self) -> Cow<'_, str> {
    self.0.to_js_string()
  }

  /// The class name as a `Map` key, or `None` for an array or an object,
  /// which is a key of its own.
  pub(super) fn map_key(&self) -> Option<MapKey<'_>> {
    self.0.same_value_zero_key()
  }

  /// The class name as the receiver of `localeCompare`, which only a string
  /// is.
  pub(super) fn as_receiver(&self) -> Result<&str, ProcessStylexRulesError> {
    match &self.0 {
      JsValue::String(class_name) => Ok(class_name),
      other => Err(ProcessStylexRulesError::ClassNameNotAString {
        class_name: self.spelling().into_owned(),
        kind: other.describe(),
      }),
    }
  }
}

impl From<JsValue> for ClassName {
  fn from(class_name: JsValue) -> Self {
    ClassName(class_name)
  }
}

impl From<&str> for ClassName {
  fn from(class_name: &str) -> Self {
    ClassName(class_name.into())
  }
}

/// What one rule injects: its LTR text, its RTL text where the two differ,
/// and, for a `defineConsts` rule, the constant it declares. Each field holds
/// the value as it arrived; a missing property is `undefined`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuleStyle {
  /// `ltr`: the rule's text, written for both directions unless `rtl` is set.
  pub ltr: JsValue,
  /// `rtl`: the text for right-to-left pages, or a falsy value for none.
  pub rtl: JsValue,
  /// `constKey`: the name of the constant a `defineConsts` rule declares.
  pub const_key: JsValue,
  /// `constVal`: the value of that constant.
  pub const_val: JsValue,
}

impl RuleStyle {
  /// `[key, { ...styleObj }, priority]`: the spread copies an object's
  /// properties, and gives no property for a string, a number, a boolean or
  /// an array (an array's elements are not the properties this reads). It
  /// throws for `null` and `undefined`, which is the `Err` here.
  fn read(index: usize, style: JsValue) -> Result<Self, ProcessStylexRulesError> {
    match style {
      mut style @ JsValue::Object(_) => Ok(RuleStyle {
        ltr: style.take_property("ltr"),
        rtl: style.take_property("rtl"),
        const_key: style.take_property("constKey"),
        const_val: style.take_property("constVal"),
      }),
      _ if style.is_nullish() => Err(ProcessStylexRulesError::NoStyleObject {
        index,
        kind: style.describe(),
      }),
      _ => Ok(RuleStyle::default()),
    }
  }

  /// `ruleObj?.constKey != null && ruleObj?.constVal != null`: the value of
  /// the constant this rule declares, or `None` for a style rule.
  pub(super) fn constant(&self) -> Option<&JsValue> {
    (!self.const_key.is_nullish() && !self.const_val.is_nullish()).then_some(&self.const_val)
  }

  /// Whether a text of this style can hold a reference to a constant or a
  /// declaration of one. Every pattern the substitution searches for,
  /// `var(--x)` and `--x:`, holds `--`.
  pub(super) fn mentions_a_variable(&self) -> bool {
    [&self.ltr, &self.rtl]
      .into_iter()
      .any(|text| matches!(text, JsValue::String(text) if text.contains("--")))
  }
}

/// One emitted rule: `[className, { ltr, rtl, constKey, constVal }, priority]`,
/// the reference's `Rule` tuple, with the priority read as the reference's
/// arithmetic reads it (`Number(priority)`: a missing one is `NaN`).
#[derive(Clone, Debug, PartialEq)]
pub struct Rule(ClassName, RuleStyle, f64);

impl Rule {
  /// A rule from its three parts, with the priority already read as a number.
  pub fn new(class_name: ClassName, style: RuleStyle, priority: f64) -> Self {
    Rule(class_name, style, priority)
  }

  /// The class name, the first item of the tuple.
  pub fn class_name(&self) -> &ClassName {
    &self.0
  }

  /// The style object, the second item of the tuple.
  pub fn style(&self) -> &RuleStyle {
    &self.1
  }

  /// The priority, the third item of the tuple. A lower priority sorts first.
  pub fn priority(&self) -> f64 {
    self.2
  }

  /// The `ltr` text, which the reference reads with string methods
  /// (`slice` in the sort, `replace` and `startsWith` in the output). Any
  /// other value throws there.
  pub(super) fn ltr(&self) -> Result<&str, ProcessStylexRulesError> {
    match &self.style().ltr {
      JsValue::String(text) => Ok(text),
      other => Err(self.text_not_a_string(TextDirection::Ltr, other)),
    }
  }

  /// The `rtl` text behind `if (rtlRule)`: a falsy value (the empty string
  /// included) is no RTL text, and a truthy value that is not a string throws
  /// at its string methods.
  pub(super) fn rtl(&self) -> Result<Option<&str>, ProcessStylexRulesError> {
    match &self.style().rtl {
      rtl if !rtl.to_js_boolean() => Ok(None),
      JsValue::String(text) => Ok(Some(text)),
      other => Err(self.text_not_a_string(TextDirection::Rtl, other)),
    }
  }

  fn text_not_a_string(
    &self,
    direction: TextDirection,
    found: &JsValue,
  ) -> ProcessStylexRulesError {
    ProcessStylexRulesError::TextNotAString {
      direction,
      class_name: self.class_name().spelling().into_owned(),
      kind: found.describe(),
    }
  }
}

/// Which of a style's two texts a refusal names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextDirection {
  /// The `ltr` text.
  Ltr,
  /// The `rtl` text.
  Rtl,
}

impl fmt::Display for TextDirection {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(match self {
      TextDirection::Ltr => "ltr",
      TextDirection::Rtl => "rtl",
    })
  }
}

/// `rules.length === 0`, the one early return of the reference: an empty
/// array, an empty string, or an object whose `length` is 0. Every other
/// value goes on to `rules.filter`.
pub(super) fn has_no_length(rules: &JsValue) -> bool {
  match rules {
    JsValue::Array(elements) => elements.is_empty(),
    JsValue::String(text) => text.is_empty(),
    JsValue::Object(_) => {
      matches!(rules.property("length"), JsValue::Number(length) if *length == 0.0)
    },
    _ => false,
  }
}

/// The rules as `rules.filter(([, ruleObj]) => ...)` reads them: only an
/// array has `filter`, a hole is skipped, and the callback destructures each
/// rule, so a rule that is not iterable is refused.
pub(crate) fn read_rules(rules: JsValue) -> Result<ReadRules, ProcessStylexRulesError> {
  let JsValue::Array(elements) = rules else {
    return Err(ProcessStylexRulesError::RulesNotAList(rules.describe()));
  };

  let mut read = ReadRules {
    rules: Vec::with_capacity(elements.len()),
    unspreadable: None,
  };

  for (index, rule) in elements.into_iter().enumerate() {
    if matches!(rule, JsValue::Hole) {
      continue;
    }

    match read_rule(index, rule) {
      Ok(rule) => read.rules.push(rule),
      Err(error @ ProcessStylexRulesError::NoStyleObject { .. }) => {
        read.unspreadable.get_or_insert(error);
      },
      Err(error) => return Err(error),
    }
  }

  Ok(read)
}

/// The rules as `filter` reads them, and the refusal of the first style that
/// the later spread cannot read. A `null` style is no constant
/// (`ruleObj?.constKey` is `undefined`), and the reference throws on its
/// spread only after it resolves the constants, so the refusal waits until
/// then.
#[derive(Debug, PartialEq)]
pub(crate) struct ReadRules {
  pub(crate) rules: Vec<Rule>,
  pub(crate) unspreadable: Option<ProcessStylexRulesError>,
}

/// `([key, { ...styleObj }, priority])`: the first three items of the rule's
/// iterator, `undefined` past its end. An array iterates its elements and a
/// string its characters; any other value is not iterable.
fn read_rule(index: usize, rule: JsValue) -> Result<Rule, ProcessStylexRulesError> {
  let items: Vec<JsValue> = match rule {
    // Only the first three are read; the rest drop with the vector.
    JsValue::Array(elements) => elements,
    JsValue::String(text) => text
      .chars()
      .take(3)
      .map(|character| JsValue::String(character.to_string()))
      .collect(),
    other => {
      return Err(ProcessStylexRulesError::RuleNotAList {
        index,
        kind: other.describe(),
      });
    },
  };
  let mut items = items.into_iter();
  let mut next = || items.next().unwrap_or_default();

  let class_name = ClassName(next());
  let style = RuleStyle::read(index, next())?;
  let priority = next().to_js_number();

  Ok(Rule(class_name, style, priority))
}

/// The `useLayers` object form: layers to declare before and after the
/// priority layers, and a prefix for their names, each as the header spells
/// it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayerOptions {
  /// `before`: the layers declared ahead of the priority layers.
  pub before: Vec<String>,
  /// `after`: the layers declared after the priority layers.
  pub after: Vec<String>,
  /// `None` when the prefix is falsy; else `String(prefix)`, which can be
  /// empty (an empty array is truthy and spells as nothing).
  pub prefix: Option<String>,
}

impl LayerOptions {
  /// `rawUseLayers.before ?? []`, `rawUseLayers.after ?? []` and
  /// `rawUseLayers.prefix ?? ''`, read off an object. Any other value has none
  /// of the properties, which gives the defaults. A list the header cannot
  /// spread is read as empty, and its refusal is handed back: the reference
  /// throws it only where it writes the header.
  fn read(use_layers: &JsValue) -> (Self, Option<ProcessStylexRulesError>) {
    let prefix = use_layers.property("prefix");
    // The header spreads `before` first, so its refusal comes first.
    let (before, after, refusal) = match (
      read_layer_names("before", use_layers.property("before")),
      read_layer_names("after", use_layers.property("after")),
    ) {
      (Ok(before), Ok(after)) => (before, after, None),
      (Err(refusal), _) | (Ok(_), Err(refusal)) => (Vec::new(), Vec::new(), Some(refusal)),
    };

    let options = LayerOptions {
      before,
      after,
      prefix: prefix
        .to_js_boolean()
        .then(|| prefix.to_js_string().into_owned()),
    };

    (options, refusal)
  }
}

/// `[...(names ?? [])]` in the header, joined with `', '`: an array gives its
/// elements, spelled as `join` spells them (`null` and `undefined` as
/// nothing), and a string gives its characters. Any other value is not
/// iterable, and the spread throws.
fn read_layer_names(
  option: &'static str,
  names: &JsValue,
) -> Result<Vec<String>, ProcessStylexRulesError> {
  match names {
    JsValue::Array(elements) => Ok(
      elements
        .iter()
        .map(|name| name.to_js_join_element().into_owned())
        .collect(),
    ),
    JsValue::String(text) => Ok(text.chars().map(String::from).collect()),
    _ if names.is_nullish() => Ok(Vec::new()),
    other => Err(ProcessStylexRulesError::LayerOptionNotAList {
      option,
      kind: other.describe(),
    }),
  }
}

/// The reference's `config` argument. `useLayers: true` is
/// `Some(LayerOptions::default())`; `false`, `null` and an absent key are
/// `None`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProcessStylexRulesConfig {
  /// `useLayers`: each priority group in its own `@layer`, or `None` for no
  /// layers.
  pub use_layers: Option<LayerOptions>,
  /// `enableLTRRTLComments`: a rule with an RTL text is written between
  /// `/* @ltr begin */` and `/* @rtl end */` comments, in place of the
  /// direction selectors.
  pub enable_ltr_rtl_comments: bool,
  /// `legacyDisableLayers`: a sheet without layers gets no specificity
  /// polyfill either.
  pub legacy_disable_layers: bool,
  /// `useLegacyClassnamesSort`: rules of one priority sorted by class name,
  /// not by their text.
  pub use_legacy_classnames_sort: bool,
}

impl From<bool> for ProcessStylexRulesConfig {
  /// `processStylexRules(rules, true)`: a bare boolean is `useLayers`.
  fn from(use_layers: bool) -> Self {
    Self {
      use_layers: use_layers.then(LayerOptions::default),
      ..Self::default()
    }
  }
}

impl ProcessStylexRulesConfig {
  /// `typeof config === 'boolean' ? { useLayers: config } : config ?? {}`,
  /// then one property read each. Only an object has the properties, so for
  /// any other value every option is at its default. `useLayers` is on for
  /// every value but `false` and a nullish one (`(useLayers ?? false) !==
  /// false`), and the three flags are read by truthiness.
  ///
  /// The refusal handed back is the one of a layer list that the header
  /// cannot spread ([`LayerOptions`]), which waits for the header.
  pub(crate) fn read(config: &JsValue) -> (Self, Option<ProcessStylexRulesError>) {
    if let JsValue::Boolean(use_layers) = config {
      return (Self::from(*use_layers), None);
    }

    let (use_layers, refusal) = match config.property("useLayers") {
      JsValue::Boolean(false) => (None, None),
      use_layers if use_layers.is_nullish() => (None, None),
      use_layers => {
        let (options, refusal) = LayerOptions::read(use_layers);
        (Some(options), refusal)
      },
    };

    let config = Self {
      use_layers,
      enable_ltr_rtl_comments: config.property("enableLTRRTLComments").to_js_boolean(),
      legacy_disable_layers: config.property("legacyDisableLayers").to_js_boolean(),
      use_legacy_classnames_sort: config.property("useLegacyClassnamesSort").to_js_boolean(),
    };

    (config, refusal)
  }
}

/// Why a sheet could not be assembled. Each one is a point where the
/// reference throws.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProcessStylexRulesError {
  /// A constant's value refers, through any chain of constants, to itself.
  #[error("circular reference detected for constant {0}")]
  CircularConstant(String),
  /// A chain of constants deeper than the native resolver follows. The
  /// reference decides: it resolves the chain, or its stack overflows.
  #[error("a constant chain is deeper than {MAX_CONSTANT_DEPTH} links")]
  ConstantChainTooDeep,
  /// The rules are not an array, so they have no `filter`.
  #[error("the rules are {0}, not a list")]
  RulesNotAList(&'static str),
  /// A rule is not iterable, so it cannot be destructured.
  #[error("rule {index} is {kind}, not a [className, style, priority] list")]
  RuleNotAList { index: usize, kind: &'static str },
  /// A style rule's style is `null` or `undefined`, which a spread cannot
  /// read.
  #[error("rule {index} has {kind} in place of its style object")]
  NoStyleObject { index: usize, kind: &'static str },
  /// A style rule's `ltr` text, or its truthy `rtl` text, is not a string
  /// where it is read as one. An array `ltr` text is refused also where the
  /// reference coerces it, as the argument of `localeCompare`.
  #[error("the {direction} text of class {class_name} is {kind}, not a string")]
  TextNotAString {
    direction: TextDirection,
    class_name: String,
    kind: &'static str,
  },
  /// Under `useLegacyClassnamesSort`, the class name that `localeCompare` is
  /// called on is not a string.
  #[error("the legacy sort cannot compare class {class_name}: it is {kind}, not a string")]
  ClassNameNotAString {
    class_name: String,
    kind: &'static str,
  },
  /// A layer option is not iterable, so the header cannot spread it.
  #[error("the layer option {option} is {kind}, not a list or a string")]
  LayerOptionNotAList {
    option: &'static str,
    kind: &'static str,
  },
  /// The first rule after the sort is in priority level -1 (a priority from
  /// -1000 to less than 0). The reference's grouping starts at level -1 with
  /// no group, so it fails on this rule.
  #[error(
    "class {class_name} sorts first with a priority from -1000 to less than 0, which cannot be \
     grouped; use a priority of 0 or more"
  )]
  FirstRuleBelowZero { class_name: String },
  /// A text with no `{` ends with a character outside the Basic Multilingual
  /// Plane, and the reference cuts it in two with `slice`. A `str` cannot hold
  /// half a character.
  #[error(
    "the text of class {class_name} has no declaration block and ends with a character that \
     JavaScript cuts in two"
  )]
  CharacterCutInTwo { class_name: String },
}
