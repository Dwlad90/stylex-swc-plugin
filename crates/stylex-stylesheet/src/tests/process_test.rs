//! The branches of `process` the reference's own cases do not reach, and the
//! JavaScript built-ins the port stands in for. An expectation that says
//! "measured on the plugin" is what `@stylexjs/babel-plugin` 0.19.1 answers
//! for the same input in Node 24.

use stylex_js::value::JsValue;

use crate::process::{
  ClassName, LayerOptions, OrderedConstants, ProcessStylexRulesConfig, ProcessStylexRulesError,
  Rule, RuleStyle, TextDirection, add_ancestor_selector, add_specificity_level,
  get_logical_float_vars, process_stylex_rule_values, process_stylex_rules, read_rules,
  resolve_constant,
};

fn rule(class_name: &str, ltr: &str, rtl: Option<&str>, priority: f64) -> Rule {
  Rule::new(
    class_name.into(),
    RuleStyle {
      ltr: ltr.into(),
      rtl: rtl.map_or(JsValue::Null, JsValue::from),
      ..RuleStyle::default()
    },
    priority,
  )
}

fn constant(class_name: &str, value: JsValue) -> Rule {
  Rule::new(
    class_name.into(),
    RuleStyle {
      const_key: class_name.into(),
      const_val: value,
      ..RuleStyle::default()
    },
    0.0,
  )
}

fn text(value: &str) -> JsValue {
  value.into()
}

fn without_layers() -> ProcessStylexRulesConfig {
  ProcessStylexRulesConfig {
    legacy_disable_layers: true,
    ..ProcessStylexRulesConfig::default()
  }
}

fn sheet(rules: &[Rule], config: &ProcessStylexRulesConfig) -> String {
  match process_stylex_rules(rules, config) {
    Ok(sheet) => sheet,
    Err(error) => panic!("the sheet assembles: {error}"),
  }
}

/// `json` as a JavaScript value. JSON has no `undefined`, `NaN` or hole; a
/// test that needs one builds the value itself.
fn json(json: &str) -> JsValue {
  match serde_json::from_str(json) {
    Ok(value) => value,
    Err(error) => panic!("the JSON reads: {error}"),
  }
}

fn rules_of(rules: &str) -> Vec<Rule> {
  match read_rules(json(rules)) {
    Ok(read) => read.rules,
    Err(error) => panic!("the rules read: {error}"),
  }
}

fn config_of(config: &str) -> ProcessStylexRulesConfig {
  match ProcessStylexRulesConfig::read(&json(config)) {
    (config, None) => config,
    (_, Some(error)) => panic!("the config reads: {error}"),
  }
}

/// What the whole call answers for two JavaScript values.
fn answer(rules: JsValue, config: JsValue) -> Result<String, ProcessStylexRulesError> {
  process_stylex_rule_values(rules, &config)
}

/// The sheet the whole call writes, or the test fails naming the refusal.
fn answer_sheet(rules: JsValue, config: JsValue) -> String {
  match answer(rules, config) {
    Ok(sheet) => sheet,
    Err(error) => panic!("the sheet assembles: {error}"),
  }
}

fn array(elements: Vec<JsValue>) -> JsValue {
  JsValue::Array(elements)
}

fn style(ltr: &str) -> JsValue {
  json(&format!(r#"{{"ltr": {ltr:?}, "rtl": null}}"#))
}

fn tuple(class_name: JsValue, style: JsValue, priority: JsValue) -> JsValue {
  array(vec![class_name, style, priority])
}

// ==================== the config as the reference reads it ====================

#[test]
fn a_bare_boolean_config_is_use_layers() {
  assert_eq!(
    ProcessStylexRulesConfig::from(true),
    ProcessStylexRulesConfig {
      use_layers: Some(LayerOptions::default()),
      ..ProcessStylexRulesConfig::default()
    }
  );
  assert_eq!(
    ProcessStylexRulesConfig::from(false),
    ProcessStylexRulesConfig::default()
  );
}

#[test]
fn use_layers_reads_every_spelling_javascript_gives_it() {
  assert_eq!(config_of("true"), ProcessStylexRulesConfig::from(true));
  assert_eq!(config_of("false"), ProcessStylexRulesConfig::default());
  assert_eq!(config_of("{}"), ProcessStylexRulesConfig::default());
  assert_eq!(
    config_of(r#"{"useLayers": null}"#),
    ProcessStylexRulesConfig::default()
  );
  assert_eq!(
    ProcessStylexRulesConfig::read(&JsValue::Undefined),
    (ProcessStylexRulesConfig::default(), None)
  );
  assert_eq!(
    config_of(r#"{"useLayers": false}"#),
    ProcessStylexRulesConfig::default()
  );
  assert_eq!(
    config_of(r#"{"useLayers": true}"#),
    ProcessStylexRulesConfig::from(true)
  );
  assert_eq!(
    config_of(
      r#"{"useLayers": {"prefix": "x"}, "enableLTRRTLComments": true, "legacyDisableLayers": true, "useLegacyClassnamesSort": true}"#
    ),
    ProcessStylexRulesConfig {
      use_layers: Some(LayerOptions {
        prefix: Some("x".to_string()),
        ..LayerOptions::default()
      }),
      enable_ltr_rtl_comments: true,
      legacy_disable_layers: true,
      use_legacy_classnames_sort: true,
    }
  );
}

/// The layer options are read as the header spreads and joins them: a string
/// gives its characters (an emoji is one), an array element is spelled by
/// `join` (`null` as nothing, an object as `[object Object]`), and the prefix
/// is read by truthiness and then spelled, so an empty array is a prefix that
/// spells as nothing. Measured on the plugin.
#[test]
fn layer_options_are_spread_and_spelled_as_the_header_reads_them() {
  let rules = || json(r#"[["x1", {"ltr": ".x1{a:0}", "rtl": null}, 3000]]"#);
  let with_layers =
    |options: &str| answer(rules(), json(&format!(r#"{{"useLayers": {options}}}"#)));

  assert_eq!(
    with_layers(r#"{"before": "a😀", "after": [true, {}]}"#),
    Ok("\n@layer a, 😀, priority1, true, [object Object];\n@layer priority1{\n.x1{a:0}\n}".into())
  );
  assert_eq!(
    with_layers(r#"{"before": ["a", 5, null, [1, 2]]}"#),
    Ok("\n@layer a, 5, , 1,2, priority1;\n@layer priority1{\n.x1{a:0}\n}".into())
  );
  assert_eq!(
    with_layers(r#"{"prefix": 5}"#),
    Ok("\n@layer 5.priority1;\n@layer 5.priority1{\n.x1{a:0}\n}".into())
  );
  assert_eq!(
    with_layers(r#"{"prefix": true}"#),
    Ok("\n@layer true.priority1;\n@layer true.priority1{\n.x1{a:0}\n}".into())
  );
  assert_eq!(
    with_layers(r#"{"prefix": {}}"#),
    Ok(
      "\n@layer [object Object].priority1;\n@layer [object Object].priority1{\n.x1{a:0}\n}".into()
    )
  );
  assert_eq!(
    with_layers(r#"{"prefix": []}"#),
    Ok("\n@layer .priority1;\n@layer .priority1{\n.x1{a:0}\n}".into())
  );
  for falsy in ["0", "\"\"", "false", "null"] {
    assert_eq!(
      with_layers(&format!(r#"{{"prefix": {falsy}}}"#)),
      Ok("\n@layer priority1;\n@layer priority1{\n.x1{a:0}\n}".into()),
      "{falsy}"
    );
  }
  assert_eq!(
    answer(
      rules(),
      JsValue::object([(
        "useLayers".to_string(),
        JsValue::object([("prefix".to_string(), f64::NAN.into())])
      )])
    ),
    Ok("\n@layer priority1;\n@layer priority1{\n.x1{a:0}\n}".into())
  );
}

/// A layer option that is not iterable throws at the spread, once there are
/// rules to write; with no rules the reference returns before the header.
#[test]
fn a_layer_option_that_is_not_iterable_is_refused() {
  let rules = || json(r#"[["x1", {"ltr": ".x1{a:0}", "rtl": null}, 3000]]"#);

  for (options, option, kind) in [
    (r#"{"before": 5}"#, "before", "a number"),
    (r#"{"before": true}"#, "before", "a boolean"),
    (r#"{"after": {}}"#, "after", "an object"),
  ] {
    let refusal = answer(rules(), json(&format!(r#"{{"useLayers": {options}}}"#)));

    assert_eq!(
      refusal,
      Err(ProcessStylexRulesError::LayerOptionNotAList { option, kind }),
      "{options}"
    );
  }
  assert_eq!(
    answer(json("[]"), json(r#"{"useLayers": {"before": 5}}"#)),
    Ok(String::new())
  );
  assert_eq!(
    ProcessStylexRulesError::LayerOptionNotAList {
      option: "before",
      kind: "a number"
    }
    .to_string(),
    "the layer option before is a number, not a list or a string"
  );
}

/// When two reads fail, the error is the one the reference meets first. Its
/// order is: a circular constant, then the spread of a style, then the sort,
/// then the grouping, then the `@layer` header, then the output. Measured on
/// the plugin.
#[test]
fn of_two_failures_the_one_the_reference_meets_first_is_refused() {
  let circular = r#"["xA", {"constKey": "xA", "constVal": "var(--xB)"}, 0],
    ["xB", {"constKey": "xB", "constVal": "var(--xA)"}, 0]"#;
  let circular_error = Err(ProcessStylexRulesError::CircularConstant("--xB".into()));
  let before_5 = || json(r#"{"useLayers": {"before": 5}}"#);
  let layer_error = Err(ProcessStylexRulesError::LayerOptionNotAList {
    option: "before",
    kind: "a number",
  });

  assert_eq!(
    answer(
      json(&format!(r#"[["x1", null, 3000], {circular}]"#)),
      true.into()
    ),
    circular_error
  );
  assert_eq!(
    answer(
      json(&format!(
        r#"[["x1", {{"ltr": ".a{{}}"}}, 3000], {circular}]"#
      )),
      before_5()
    ),
    circular_error
  );
  assert_eq!(
    answer(
      json(r#"[["x1", {"ltr": ".a{}"}, 3000], ["x2", {}, 3000]]"#),
      before_5()
    ),
    Err(ProcessStylexRulesError::TextNotAString {
      direction: TextDirection::Ltr,
      class_name: "x2".into(),
      kind: "undefined"
    })
  );
  assert_eq!(
    answer(json(r#"[["x1", {"ltr": ".a{}"}, -1]]"#), before_5()),
    Err(ProcessStylexRulesError::FirstRuleBelowZero {
      class_name: "x1".into()
    })
  );
  assert_eq!(
    answer(
      json(r#"[["x1", {"ltr": ".a{}", "rtl": 5}, 3000]]"#),
      before_5()
    ),
    layer_error
  );
  // `filter` destructures each rule before any spread.
  assert_eq!(
    answer(json(r#"[["x1", null, 3000], 5]"#), true.into()),
    Err(ProcessStylexRulesError::RuleNotAList {
      index: 1,
      kind: "a number"
    })
  );
  // Of two styles that cannot be spread, the first is refused.
  assert_eq!(
    answer(json(r#"[["x1", null, 3000], ["x2"]]"#), true.into()),
    Err(ProcessStylexRulesError::NoStyleObject {
      index: 0,
      kind: "null"
    })
  );
  // The header spreads `before` before `after`.
  assert_eq!(
    answer(
      json(r#"[["x1", {"ltr": ".a{}"}, 3000]]"#),
      json(r#"{"useLayers": {"before": 5, "after": {}}}"#)
    ),
    layer_error
  );
  assert_eq!(
    answer(
      json(r#"[["x1", {}, 3000], ["x2", null, 3000]]"#),
      true.into()
    ),
    Err(ProcessStylexRulesError::NoStyleObject {
      index: 1,
      kind: "null"
    })
  );
}

// ==================== constants ====================

#[test]
fn a_number_constant_substitutes_as_javascript_spells_it() {
  let rules = [
    constant("xN", 10.0.into()),
    constant("xF", 1.5.into()),
    rule("xA", ".xA{margin:var(--xN)px var(--xF)px}", None, 3000.0),
  ];

  assert_eq!(sheet(&rules, &without_layers()), ".xA{margin:10px 1.5px}");
}

/// Measured on the plugin: a boolean constant substitutes as `String(flag)`,
/// and a `null` one is no constant, so a rule declaring it sorts as a style
/// (with an empty text) and a reference to it stays as written.
#[test]
fn a_boolean_constant_substitutes_as_its_word_and_a_null_one_is_no_constant() {
  let rules = rules_of(
    r#"[
      ["xT", {"constKey": "xT", "constVal": true}, 0],
      ["xF", {"constKey": "xF", "constVal": false}, 0],
      ["x1", {"ltr": ".x1{--a:var(--xT) var(--xF)}", "rtl": null}, 3000]
    ]"#,
  );

  assert_eq!(rules[0].style().constant(), Some(&JsValue::from(true)));
  assert_eq!(
    resolve_constant(
      &JsValue::from(true),
      &OrderedConstants::default(),
      &mut Vec::new()
    ),
    Ok(JsValue::from(true))
  );
  assert_eq!(sheet(&rules, &without_layers()), ".x1{--a:true false}");

  let null_constant = rules_of(
    r#"[
      ["xN", {"constKey": "xN", "constVal": null, "ltr": "", "rtl": null}, 0],
      ["x1", {"ltr": ".x1{--a:var(--xN)}", "rtl": null}, 3000]
    ]"#,
  );

  assert_eq!(null_constant[0].style().constant(), None);
  assert_eq!(
    sheet(&null_constant, &without_layers()),
    "\n.x1{--a:var(--xN)}"
  );
}

#[test]
fn a_constant_resolves_through_the_constants_it_refers_to_and_keeps_unknown_references() {
  let constants: OrderedConstants = [
    ("var(--a)".to_string(), text("1px solid var(--b)")),
    ("var(--b)".to_string(), text("var(--c) var(--unknown)")),
    ("var(--c)".to_string(), text("red")),
    ("var(--n)".to_string(), 2.0.into()),
  ]
  .into_iter()
  .collect();

  assert_eq!(
    resolve_constant(&constants["var(--a)"], &constants, &mut Vec::new()),
    Ok(text("1px solid red var(--unknown)"))
  );
  assert_eq!(
    resolve_constant(&constants["var(--n)"], &constants, &mut Vec::new()),
    Ok(2.0.into())
  );
}

#[test]
fn a_circular_constant_is_refused_by_name() {
  let rules = [
    constant("xA", text("var(--xB)")),
    constant("xB", text("var(--xA)")),
    rule("x1", ".x1{color:var(--xA)}", None, 3000.0),
  ];

  let Err(error) = process_stylex_rules(&rules, &without_layers()) else {
    panic!("a cycle is refused");
  };

  assert_eq!(
    error,
    ProcessStylexRulesError::CircularConstant("--xB".to_string())
  );
  assert_eq!(
    error.to_string(),
    "circular reference detected for constant --xB"
  );
}

/// The reference (0.19.1) hands `replaceAll` a string, so a `$&` in a
/// constant's value stands for the reference it replaces: the declaration
/// reads as it did. Measured on the plugin.
#[test]
fn a_dollar_pattern_in_a_constant_is_read_as_get_substitution_reads_it() {
  let rules = [
    constant("xA", text("$&")),
    rule("x1", ".x1{content:var(--xA)}", None, 3000.0),
  ];

  assert_eq!(sheet(&rules, &without_layers()), ".x1{content:var(--xA)}");
}

#[test]
fn a_constant_declared_twice_keeps_its_first_position_and_its_last_value() {
  let rules = [
    constant("xA", text("red")),
    constant("xA", text("blue")),
    rule("x1", ".x1{color:var(--xA)}", None, 3000.0),
  ];

  assert_eq!(sheet(&rules, &without_layers()), ".x1{color:blue}");
}

#[test]
fn a_constant_holding_a_variable_renames_the_declarations_of_the_constant() {
  let rules = [
    constant("xA", text("var(--theme-color, red)")),
    rule("x1", ".x1{--xA:blue}", None, 3000.0),
    rule("x2", ".x2{color:var(--xA)}", None, 3000.0),
  ];

  assert_eq!(
    sheet(&rules, &without_layers()),
    ".x1{--theme-color:blue}\n.x2{color:var(--theme-color, red)}"
  );
}

/// A substitution can bring in the pattern of one that comes after it: `c0`
/// writes `--c9:`, and `c9`, a `var(...)`, renames it on its turn. Measured on
/// the plugin.
#[test]
fn a_pattern_brought_in_by_a_substitution_is_applied_by_a_later_one() {
  let mut rules = vec![constant("c0", text("a --c9: b"))];
  rules.extend((1..=8).map(|index| constant(&format!("c{index}"), text(&format!("{index}px")))));
  rules.push(constant("c9", text("var(--t9)")));
  rules.push(rule(
    "x1",
    ".x1{--q:var(--c0);margin:var(--c3)}",
    None,
    3000.0,
  ));
  // Nine substitutions apply to `x2`: past four, the rest apply one by one.
  rules.push(rule(
    "x2",
    ".x2{--c9:0;a:var(--c1) var(--c2) var(--c4) var(--c5) var(--c6) var(--c7) var(--c8) \
     var(--c0)}",
    None,
    3000.0,
  ));

  assert_eq!(
    sheet(&rules, &without_layers()),
    ".x1{--q:a --t9: b;margin:3px}\n.x2{--t9:0;a:1px 2px 4px 5px 6px 7px 8px a --t9: b}"
  );
}

/// A constant that only the RTL text reads changes the RTL text alone; the
/// LTR text is kept as it was. A class name read from a JavaScript value is
/// the same class name as one read from a string.
#[test]
fn a_constant_read_by_the_rtl_text_alone_leaves_the_ltr_text_as_it_is() {
  let rules = [
    constant("xA", text("4px")),
    rule("x1", ".x1{--a:0}", Some(".x1{padding:var(--xA)}"), 3000.0),
  ];

  assert_eq!(
    sheet(&rules, &without_layers()),
    "html:not([dir='rtl']) .x1{--a:0}\nhtml[dir='rtl'] .x1{padding:4px}"
  );
  assert_eq!(ClassName::from(JsValue::from("x1")), ClassName::from("x1"));
}

/// Ten constants take the one-automaton search. Each substitution applies
/// once, in declaration order: a pattern that an earlier substitution brings
/// in is applied only by a substitution that comes after it (`--c5:` arrives
/// with `c8`, after `c5` has renamed, and stays). Measured on the plugin.
#[test]
fn many_constants_substitute_in_declaration_order() {
  let rules = [
    constant("c0", text("0px")),
    constant("c1", text("var(--t1)")),
    constant("c2", text("2px")),
    constant("c3", text("var(--c7)")),
    constant("c4", text("4px")),
    constant("c5", text("var(--u5, 1px)")),
    constant("c6", text("6px")),
    constant("c7", text("7px")),
    constant("c8", text("x --c5: y")),
    constant("c9", text("z --c1: w")),
    rule(
      "x1",
      ".x1{margin:var(--c0) var(--c3) var(--c9)}",
      None,
      3000.0,
    ),
    rule("x2", ".x2{--c1:1;--c5:5;padding:var(--c8)}", None, 3000.0),
    rule("x3", ".x3{--c3:var(--c4)}", None, 3000.0),
    rule("x4", ".x4{color:red}", None, 3000.0),
  ];

  assert_eq!(
    sheet(&rules, &without_layers()),
    ".x3{--c3:4px}\n.x2{--t1:1;--u5:5;padding:x --c5: y}\n.x4{color:red}\n.x1{margin:0px 7px z --c1: \
     w}"
  );
}

/// Every text a style holds is substituted, `ltr` then `rtl`, the rename
/// included, and the specificity polyfill and the ancestor selector reach the
/// RTL text as they reach the LTR one. Measured on the plugin.
#[test]
fn the_rtl_text_is_substituted_renamed_and_wrapped_like_the_ltr_text() {
  let rules = [
    constant("xA", text("var(--theme-inset, 4px)")),
    rule(
      "x1",
      ".x1{--xA:1px;margin-left:var(--xA)}",
      Some(".x1{--xA:1px;margin-right:var(--xA)}"),
      1000.0,
    ),
    rule("x2", ".x2{color:red}", None, 3000.0),
  ];

  assert_eq!(
    sheet(&rules, &ProcessStylexRulesConfig::default()),
    "html:not([dir='rtl']) .x1{--theme-inset:1px;margin-left:var(--theme-inset, 4px)}\nhtml[dir='rtl'] .x1{--theme-inset:1px;margin-right:var(--theme-inset, 4px)}\n.x2:not(#\\#){color:red}"
  );
}

/// A number constant read through another constant substitutes as JavaScript
/// spells it, and a `$` that ends a value stays a `$`. Measured on the plugin.
#[test]
fn a_number_reached_through_a_chain_and_a_trailing_dollar_substitute_verbatim() {
  let rules = [
    constant("xN", 4.0.into()),
    constant("xA", text("var(--xN)px")),
    constant("xD", text("1px$")),
    rule("x1", ".x1{margin:var(--xA) var(--xD)}", None, 3000.0),
  ];

  assert_eq!(sheet(&rules, &without_layers()), ".x1{margin:4px 1px$}");
}

/// An RTL twin inside a `@keyframes` block takes no ancestor, and one whose
/// at-rule has no block gets the ancestor in front of the whole text, as the
/// reference's `indexOf` of -1 slices it. Measured on the plugin.
#[test]
fn an_rtl_twin_of_a_keyframes_block_or_of_a_blockless_at_rule_follows_the_reference() {
  let keyframes = [rule(
    "k",
    "@keyframes k{0%{left:0}}",
    Some("@keyframes k{0%{right:0}}"),
    0.0,
  )];

  assert_eq!(
    sheet(&keyframes, &without_layers()),
    "@keyframes k{0%{left:0}}\n@keyframes k{0%{right:0}}"
  );

  let blockless = [rule(
    "x1",
    "@media (x) .x1",
    Some("@media (x) .x1r"),
    3000.0,
  )];

  assert_eq!(
    sheet(&blockless, &without_layers()),
    "html:not([dir='rtl']) @media (x) .x1\nhtml[dir='rtl'] @media (x) .x1r"
  );
}

// ==================== the helpers ====================

#[test]
fn the_ancestor_selector_goes_inside_the_innermost_at_rule_and_never_into_keyframes() {
  assert_eq!(
    add_ancestor_selector(".x{float:left}", "html[dir='rtl']"),
    "html[dir='rtl'] .x{float:left}"
  );
  assert_eq!(
    add_ancestor_selector(
      "@supports (x){@media (min-width: 1px){.x{float:left}}}",
      "html[dir='rtl']"
    ),
    "@supports (x){@media (min-width: 1px){html[dir='rtl'] .x{float:left}}}"
  );
  assert_eq!(
    add_ancestor_selector("@keyframes k{0%{left:0}}", "html[dir='rtl']"),
    "@keyframes k{0%{left:0}}"
  );
  // `indexOf('{', lastAtRule)` is -1 for an at-rule with no block, and the
  // ancestor then goes in front of the whole text.
  assert_eq!(
    add_ancestor_selector("@media (x) .x", "html[dir='rtl']"),
    "html[dir='rtl'] @media (x) .x"
  );
}

#[test]
fn the_specificity_level_goes_before_a_pseudo_element_else_before_the_block() {
  assert_eq!(
    add_specificity_level(".x{color:red}", 0),
    Some(".x{color:red}".into())
  );
  assert_eq!(
    add_specificity_level(".x{color:red}", 2),
    Some(".x:not(#\\#):not(#\\#){color:red}".into())
  );
  assert_eq!(
    add_specificity_level(".x::before{content:''}", 1),
    Some(".x:not(#\\#)::before{content:''}".into())
  );
  assert_eq!(
    add_specificity_level("@keyframes k{0%{left:0}}", 3),
    Some("@keyframes k{0%{left:0}}".into())
  );
  // `lastIndexOf('{')` is -1 for a text with no block, and `slice(0, -1)` then
  // keeps all but the last character.
  assert_eq!(
    add_specificity_level("abc", 1),
    Some("ab:not(#\\#)c".into())
  );
  assert_eq!(add_specificity_level("", 1), Some(":not(#\\#)".into()));
}

/// A text with no block splits before its last character: the plugin writes
/// `.a:not(#\#)é`. For a character outside the Basic Multilingual Plane,
/// JavaScript splits the UTF-16 surrogate pair (`.a\uD83D:not(#\#)\uDE00`, two
/// lone surrogates). A `str` cannot hold half a pair, so the port refuses it,
/// and the addon runs the reference instead.
#[test]
fn the_specificity_level_splits_a_blockless_text_before_its_last_character() {
  assert_eq!(
    add_specificity_level(".aé", 1),
    Some(".a:not(#\\#)é".into())
  );
  assert_eq!(
    add_specificity_level(".aé", 2),
    Some(".a:not(#\\#):not(#\\#)é".into())
  );
  assert_eq!(add_specificity_level("é", 1), Some(":not(#\\#)é".into()));
  assert_eq!(add_specificity_level(".a😀", 1), None);
  // A pseudo-element comes first, so the last character is not cut.
  assert_eq!(
    add_specificity_level(".a::b😀", 1),
    Some(".a:not(#\\#)::b😀".into())
  );

  // Through the sheet: the rule at group index 1, with an RTL twin.
  let rules = [
    rule("x", ".k{a:0}", None, 0.0),
    rule("y", ".aé", Some(".bé"), 3000.0),
  ];
  assert_eq!(
    sheet(&rules, &ProcessStylexRulesConfig::default()),
    ".k{a:0}\nhtml:not([dir='rtl']) .a:not(#\\#)é\nhtml[dir='rtl'] .b:not(#\\#)é"
  );
}

/// `rule.slice(rule.lastIndexOf('{'))` on a text with no block is
/// `slice(-1)`, the last character: two rules that tie on everything before
/// it sort by that character. Measured on the plugin.
#[test]
fn the_tie_break_reads_the_last_character_of_a_text_with_no_block() {
  let rules = [
    rule("a", "ab:yx", None, 3000.0),
    rule("b", "ab:xy", None, 3000.0),
  ];
  assert_eq!(sheet(&rules, &without_layers()), "ab:yx\nab:xy");

  let empty_first = [rule("a", "", None, 3000.0), rule("b", "x", None, 3000.0)];
  assert_eq!(sheet(&empty_first, &without_layers()), "\nx");

  let accent = [
    rule("a", "ab:é", None, 3000.0),
    rule("b", "ab:e", None, 3000.0),
  ];
  assert_eq!(sheet(&accent, &without_layers()), "ab:e\nab:é");

  // JavaScript compares the low surrogates of two emoji, which a `str` cannot
  // hold, so the port refuses the pair. V8's first comparison takes the
  // second rule as its left operand, so the refusal names `b`.
  let emoji = [
    rule("a", "ab:😁", None, 3000.0),
    rule("b", "ab:😀", None, 3000.0),
  ];
  assert_eq!(
    process_stylex_rules(&emoji, &without_layers()),
    Err(ProcessStylexRulesError::CharacterCutInTwo {
      class_name: "b".into()
    })
  );

  // The rules sort apart before the last character is read, and the sheet is
  // written with layers, so no character is cut.
  // The second operand ends in the emoji, the first in an ASCII character.
  let second = [
    rule("a", "ab:😀", None, 3000.0),
    rule("b", "ab:x", None, 3000.0),
  ];
  assert_eq!(
    process_stylex_rules(&second, &without_layers()),
    Err(ProcessStylexRulesError::CharacterCutInTwo {
      class_name: "a".into()
    })
  );

  let apart = [
    rule("a", "ab:😁", None, 3000.0),
    rule("b", "ac:😀", None, 3000.0),
  ];
  assert_eq!(
    sheet(&apart, &ProcessStylexRulesConfig::from(true)),
    "\n@layer priority1;\n@layer priority1{\nab:😁\nac:😀\n}"
  );
}

/// JavaScript cuts the character and writes on, so an error in a later rule of
/// the output is the error the reference throws: here `c`'s `ltr` is a
/// number. Measured on the plugin.
#[test]
fn a_character_cut_in_the_output_waits_for_the_rest_of_the_output() {
  let rules = [
    rule("a", ".a{color:red}", None, 0.0),
    rule("b", ".b😀", None, 1000.0),
    Rule::new(
      "c".into(),
      RuleStyle {
        ltr: 5.0.into(),
        ..RuleStyle::default()
      },
      2000.0,
    ),
  ];
  let legacy = ProcessStylexRulesConfig {
    use_legacy_classnames_sort: true,
    ..ProcessStylexRulesConfig::default()
  };

  assert_eq!(
    process_stylex_rules(&rules, &legacy),
    Err(ProcessStylexRulesError::TextNotAString {
      direction: TextDirection::Ltr,
      class_name: "c".into(),
      kind: "a number"
    })
  );

  // With two cuts and no other error, the first cut is refused.
  let two_cuts = [
    rule("a", ".a{color:red}", None, 0.0),
    rule("b", ".b😀", None, 1000.0),
    rule("d", ".d😀", None, 2000.0),
  ];
  assert_eq!(
    process_stylex_rules(&two_cuts, &legacy),
    Err(ProcessStylexRulesError::CharacterCutInTwo {
      class_name: "b".into()
    })
  );
}

/// The specificity level cuts the last character of a blockless text, for
/// the LTR and for the RTL text alike. In the first group it inserts nothing,
/// and the two halves join back into the text: the plugin writes it as it is.
#[test]
fn the_specificity_level_refuses_to_cut_a_character_in_two() {
  for (ltr, rtl) in [(".a😀", None), (".a{b:c}", Some(".a😀"))] {
    let rules = [rule("k", ".k{a:0}", None, 0.0), rule("x", ltr, rtl, 3000.0)];
    assert_eq!(
      process_stylex_rules(&rules, &ProcessStylexRulesConfig::default()),
      Err(ProcessStylexRulesError::CharacterCutInTwo {
        class_name: "x".into()
      })
    );
  }

  let first_group = [rule("x", ".a😀", None, 3000.0)];
  assert_eq!(
    process_stylex_rules(&first_group, &ProcessStylexRulesConfig::default()),
    Ok(".a😀".into())
  );
}

/// `String.prototype.trim` removes U+FEFF and keeps U+0085, where `str::trim`
/// does the opposite: the rename target of a constant holding a `var(...)` is
/// trimmed as JavaScript trims it. Measured on the plugin.
#[test]
fn the_rename_target_is_trimmed_as_javascript_trims() {
  let zwnbsp = [
    constant("xA", text("var(\u{FEFF}--t)")),
    rule("x1", ".x1{--xA:1px}", None, 3000.0),
    rule("x2", ".x2{color:var(--xA)}", None, 3000.0),
  ];
  assert_eq!(
    sheet(&zwnbsp, &without_layers()),
    ".x1{--t:1px}\n.x2{color:var(\u{FEFF}--t)}"
  );

  let nel = [
    constant("xA", text("var(\u{85}--t)")),
    rule("x1", ".x1{--xA:1px}", None, 3000.0),
    rule("x2", ".x2{color:var(--xA)}", None, 3000.0),
  ];
  assert_eq!(
    sheet(&nel, &without_layers()),
    ".x1{\u{85}--t:1px}\n.x2{color:var(\u{85}--t)}"
  );

  let both_sides = [
    constant("xA", text("var(\u{A0}--t\u{FEFF} , red)")),
    rule("x1", ".x1{--xA:1px}", None, 3000.0),
  ];
  assert_eq!(sheet(&both_sides, &without_layers()), ".x1{--t:1px}");
}

#[test]
fn the_logical_float_block_is_written_when_a_text_reads_either_variable() {
  assert_eq!(get_logical_float_vars(&[]), "");
  assert_eq!(
    get_logical_float_vars(&[rule(
      "x",
      ".x{float:right}",
      Some(".x{float:var(--stylex-logical-end)}"),
      3000.0
    )])
    .lines()
    .count(),
    8
  );
}

// ==================== the sheet ====================

#[test]
fn no_rules_is_an_empty_sheet() {
  assert_eq!(sheet(&[], &ProcessStylexRulesConfig::from(true)), "");
}

#[test]
fn an_empty_rtl_text_is_as_absent_as_a_missing_one() {
  let rules = [rule("x1", ".x1{color:red}", Some(""), 3000.0)];

  assert_eq!(sheet(&rules, &without_layers()), ".x1{color:red}");
  assert_eq!(
    sheet(
      &rules,
      &ProcessStylexRulesConfig {
        enable_ltr_rtl_comments: true,
        ..without_layers()
      }
    ),
    ".x1{color:red}"
  );
}

#[test]
fn an_rtl_rule_inside_an_at_rule_takes_its_ancestors_inside_it() {
  let rules = [rule(
    "x1",
    "@media (min-width: 1px){.x1{float:left}}",
    Some("@media (min-width: 1px){.x1{float:right}}"),
    3000.0,
  )];

  assert_eq!(
    sheet(&rules, &without_layers()),
    "@media (min-width: 1px){html:not([dir='rtl']) .x1{float:left}}\n@media (min-width: 1px){html[dir='rtl'] .x1{float:right}}"
  );
}

#[test]
fn a_rule_with_no_at_rule_sorts_after_its_breakpoints_by_text() {
  let rules = [
    rule("xB", ".xB{color:blue}", None, 3000.0),
    rule("xA", ".xA{color:red}", None, 3000.0),
    rule(
      "xM",
      "@media (min-width: 500px){.xM.xM{color:green}}",
      None,
      3000.0,
    ),
  ];

  // Same property: the breakpoint first, then the two plain rules by their
  // declaration text (`blue` before `red`).
  assert_eq!(
    sheet(&rules, &without_layers()),
    "@media (min-width: 500px){.xM.xM{color:green}}\n.xB{color:blue}\n.xA{color:red}"
  );
}

// ==================== the whole assembly, every branch ====================
//
// The reference's own cases (`tests/process_stylex_rules_cases.rs`) reach most
// of the assembly; these hold the rest from this suite, so the suite alone
// covers the step. Each expectation is measured on the plugin.

/// `useLayers` as an object: the ordinal layer names under the prefix, the
/// `before` and `after` layers around them in the statement, a rule of
/// priority 0 (`@keyframes`) outside every block, and an RTL twin inside one.
#[test]
fn layers_name_the_groups_by_ordinal_under_the_prefix_between_the_layers_before_and_after() {
  let rules = [
    rule("k", "@keyframes k{0%{a:0}}", None, 0.0),
    rule("x1", ".x1{margin:0}", None, 1000.0),
    rule("x2", ".x2{color:red}", None, 3000.0),
    rule("x3", ".x3{color:blue}", Some(".x3{color:green}"), 3000.0),
  ];
  let config = ProcessStylexRulesConfig {
    use_layers: Some(LayerOptions {
      before: vec!["reset".to_string()],
      after: vec!["overrides".to_string()],
      prefix: Some("x".to_string()),
    }),
    ..ProcessStylexRulesConfig::default()
  };

  assert_eq!(
    sheet(&rules, &config),
    "\n@layer reset, x.priority1, x.priority2, x.priority3, overrides;\n@keyframes k{0%{a:0}}\n@layer x.priority2{\n.x1{margin:0}\n}\n@layer x.priority3{\nhtml:not([dir='rtl']) .x3{color:blue}\nhtml[dir='rtl'] .x3{color:green}\n.x2{color:red}\n}"
  );
}

/// `useLayers: true`: no prefix, so the names are the bare ordinals.
#[test]
fn layers_without_a_prefix_are_the_bare_ordinals() {
  let rules = [
    rule("x1", ".x1{margin:0}", None, 1000.0),
    rule("x2", ".x2{color:red}", None, 3000.0),
  ];

  assert_eq!(
    sheet(&rules, &ProcessStylexRulesConfig::from(true)),
    "\n@layer priority1, priority2;\n@layer priority1{\n.x1{margin:0}\n}\n@layer priority2{\n.x2{color:red}\n}"
  );
}

/// `rawUseLayers !== false`: every value but `false` and `null` turns the
/// layers on, with the defaults when it is not an object (`typeof` fails for
/// a string or a number, and an array has none of the properties). Measured on
/// the plugin.
#[test]
fn use_layers_is_on_for_every_value_but_false_and_null() {
  let rules = [rule("x1", ".x1{color:red}", None, 3000.0)];
  let with_layers = "\n@layer priority1;\n@layer priority1{\n.x1{color:red}\n}";

  for spelling in [
    r#"{"useLayers": "yes"}"#,
    r#"{"useLayers": 0}"#,
    r#"{"useLayers": 1.5}"#,
    r#"{"useLayers": -1}"#,
    r#"{"useLayers": ["a"]}"#,
    r#"{"useLayers": {}}"#,
    r#"{"useLayers": true}"#,
  ] {
    assert_eq!(
      sheet(&rules, &config_of(spelling)),
      with_layers,
      "{spelling}"
    );
  }

  for spelling in [r#"{"useLayers": null}"#, r#"{"useLayers": false}"#, "{}"] {
    assert_eq!(
      sheet(&rules, &config_of(spelling)),
      ".x1{color:red}",
      "{spelling}"
    );
  }
}

/// `config ?? {}` and the property reads on it: `null`, a number, a string
/// and an array have none of the properties, so every option is at its
/// default; a boolean is `useLayers`. Measured on the plugin.
#[test]
fn a_config_that_is_not_an_object_has_every_option_at_its_default() {
  for spelling in ["null", "1", "-1", "1.5", r#""yes""#, "[true]", "false"] {
    assert_eq!(
      config_of(spelling),
      ProcessStylexRulesConfig::default(),
      "{spelling}"
    );
  }

  assert_eq!(config_of("true"), ProcessStylexRulesConfig::from(true));
}

/// `before ?? []`, `after ?? []`, `prefix ?? ''`: a `null` property is the
/// default.
#[test]
fn a_null_layer_option_is_its_default() {
  assert_eq!(
    config_of(r#"{"useLayers": {"before": null, "after": ["z"], "prefix": null}}"#),
    ProcessStylexRulesConfig {
      use_layers: Some(LayerOptions {
        before: Vec::new(),
        after: vec!["z".to_string()],
        prefix: None,
      }),
      ..ProcessStylexRulesConfig::default()
    }
  );
}

/// The three flags are read as `if (flag)` reads them: JavaScript truthiness.
/// Measured on the plugin (`enableLTRRTLComments: 1` writes the comments,
/// `""` and `null` do not).
#[test]
fn the_flags_are_read_for_their_truthiness() {
  let truthy = [
    "true",
    "1",
    "-1",
    "0.5",
    r#""no""#,
    "[]",
    r#"{"a": 1}"#,
    r#"[1, 2]"#,
  ];
  let falsy = ["false", "null", "0", "0.0", r#""""#];

  for spelling in truthy {
    let config = config_of(&format!(
      r#"{{"enableLTRRTLComments": {spelling}, "legacyDisableLayers": {spelling}, "useLegacyClassnamesSort": {spelling}}}"#
    ));
    assert!(
      config.enable_ltr_rtl_comments
        && config.legacy_disable_layers
        && config.use_legacy_classnames_sort,
      "{spelling} is truthy"
    );
  }

  for spelling in falsy {
    let config = config_of(&format!(
      r#"{{"enableLTRRTLComments": {spelling}, "legacyDisableLayers": {spelling}, "useLegacyClassnamesSort": {spelling}}}"#
    ));
    assert_eq!(
      config,
      ProcessStylexRulesConfig::default(),
      "{spelling} is falsy"
    );
  }

  let rules = [rule("x1", ".x1{a:0}", Some(".x1{b:0}"), 3000.0)];
  assert_eq!(
    sheet(
      &rules,
      &config_of(r#"{"useLayers": false, "enableLTRRTLComments": 1}"#)
    ),
    "/* @ltr begin */.x1{a:0}/* @ltr end */\n/* @rtl begin */.x1{b:0}/* @rtl end */"
  );
  assert_eq!(
    sheet(
      &rules,
      &config_of(r#"{"useLayers": false, "enableLTRRTLComments": ""}"#)
    ),
    "html:not([dir='rtl']) .x1{a:0}\nhtml[dir='rtl'] .x1{b:0}"
  );
}

/// `useLegacyClassnamesSort`: rules of one priority sort by class name, where
/// the default sorts by the declaration text first.
#[test]
fn the_legacy_sort_orders_one_priority_by_class_name() {
  let rules = [
    rule("xb", ".xb{color:red}", None, 3000.0),
    rule("xa", ".xa{margin:0}", None, 3000.0),
  ];
  let legacy = ProcessStylexRulesConfig {
    use_legacy_classnames_sort: true,
    ..without_layers()
  };

  assert_eq!(sheet(&rules, &legacy), ".xa{margin:0}\n.xb{color:red}");
  assert_eq!(
    sheet(&rules, &without_layers()),
    ".xb{color:red}\n.xa{margin:0}"
  );
}

/// Two rules whose declaration texts tie sort by their whole text.
#[test]
fn rules_that_tie_on_the_declaration_sort_by_the_whole_text() {
  let rules = [
    rule("xb", ".xb{color:red}", None, 3000.0),
    rule("xa", ".xa{color:red}", None, 3000.0),
  ];

  assert_eq!(
    sheet(&rules, &without_layers()),
    ".xa{color:red}\n.xb{color:red}"
  );
}

/// One class twice in one priority group: one entry, at the position of the
/// first (after the sort) with the text of the last, as `new Map` keeps them.
#[test]
fn a_class_repeated_in_one_group_keeps_the_first_position_and_the_last_text() {
  let rules = [
    rule("x1", ".x1{color:red}", None, 3000.0),
    rule("x2", ".x2{color:blue}", None, 3000.0),
    rule("x1", ".x1{color:green}", None, 3000.0),
  ];

  assert_eq!(
    sheet(&rules, &without_layers()),
    ".x2{color:blue}\n.x1{color:red}"
  );
}

/// `enableLTRRTLComments`: a rule with an RTL twin is written as two
/// commented blocks rather than under the two ancestor selectors.
#[test]
fn ltr_rtl_comments_replace_the_ancestor_selectors() {
  let rules = [
    rule(
      "x1",
      ".x1{margin-left:0}",
      Some(".x1{margin-right:0}"),
      3000.0,
    ),
    rule("x2", ".x2{color:red}", None, 3000.0),
  ];
  let config = ProcessStylexRulesConfig {
    enable_ltr_rtl_comments: true,
    ..without_layers()
  };

  assert_eq!(
    sheet(&rules, &config),
    ".x2{color:red}\n/* @ltr begin */.x1{margin-left:0}/* @ltr end */\n/* @rtl begin */.x1{margin-right:0}/* @rtl end */"
  );
}

/// Rules under one at-rule chain read one sort key (the chain is read once
/// per sheet), and the chains order by their width bound.
#[test]
fn rules_under_one_at_rule_chain_share_its_sort_key() {
  let rules = [
    rule(
      "x1",
      "@media (min-width: 900px){.x1{color:red}}",
      None,
      3200.0,
    ),
    rule(
      "x2",
      "@media (min-width: 500px){.x2{color:red}}",
      None,
      3200.0,
    ),
    rule(
      "x3",
      "@media (min-width: 900px){.x3{color:blue}}",
      None,
      3200.0,
    ),
  ];

  assert_eq!(
    sheet(&rules, &without_layers()),
    "@media (min-width: 500px){.x2{color:red}}\n@media (min-width: 900px){.x3{color:blue}}\n@media (min-width: 900px){.x1{color:red}}"
  );
}

/// A theme's `.x, .x:root` selector is doubled so the theme beats the default
/// values, in the LTR text and in the RTL text, at every match in a text.
#[test]
fn a_theme_root_selector_is_doubled_in_both_texts_at_every_match() {
  let rules = [
    rule(
      "xt",
      ".xt, .xt:root{--a:1}",
      Some(".xt, .xt:root{--a:2}"),
      500.0,
    ),
    rule(
      "xu",
      ".xu, .xu:root{--b:1} .xv, .xv:root{--c:1}",
      None,
      500.0,
    ),
  ];

  assert_eq!(
    sheet(&rules, &without_layers()),
    "html:not([dir='rtl']) .xt.xt, .xt.xt:root{--a:1}\nhtml[dir='rtl'] .xt.xt, .xt.xt:root{--a:2}\n.xu.xu, .xu.xu:root{--b:1} .xv.xv, .xv.xv:root{--c:1}"
  );
}

/// A constant holding a variable with no fallback renames the declarations
/// to that variable's whole name.
#[test]
fn a_constant_holding_a_variable_without_a_fallback_renames_to_the_whole_name() {
  let rules = [
    constant("xA", text("var(--theme-color)")),
    rule("x1", ".x1{--xA:blue}", None, 3000.0),
    rule("x2", ".x2{color:var(--xA)}", None, 3000.0),
  ];

  assert_eq!(
    sheet(&rules, &without_layers()),
    ".x1{--theme-color:blue}\n.x2{color:var(--theme-color)}"
  );
}

// ==================== a rule as the reference reads it ====================

/// `([key, { ...styleObj }, priority])`: the first three items are read, a
/// fourth is not, and an item past the end is `undefined`.
#[test]
fn a_rule_reads_three_items_and_ignores_the_rest() {
  let rules = rules_of(
    r#"[
      ["x1", {"ltr": ".x1{color:red}", "rtl": null}, 3000, "extra"],
      ["x2", {"ltr": ".x2{color:red}"}],
      ["c", {"constKey": "c", "constVal": 10}, 0],
      ["x3", {"ltr": ".x3{float:left}", "rtl": ".x3{float:right}"}, 3000.5]
    ]"#,
  );

  assert_eq!(rules[0], rule("x1", ".x1{color:red}", None, 3000.0));
  assert_eq!(rules[1].class_name(), &ClassName::from("x2"));
  assert_eq!(rules[1].style().rtl, JsValue::Undefined);
  assert!(rules[1].priority().is_nan());
  assert_eq!(rules[2], constant("c", 10.0.into()));
  assert_eq!(rules[2].style().ltr, JsValue::Undefined);
  assert_eq!(
    rules[3],
    rule("x3", ".x3{float:left}", Some(".x3{float:right}"), 3000.5)
  );
  assert_eq!(rules[3].priority(), 3000.5);
}

/// A string rule is iterable: its characters are the class name, the style
/// and the priority. A character as the style has no `ltr`, so the rule
/// throws where it is written, unless a later rule of its class takes its
/// place. Measured on the plugin.
#[test]
fn a_string_rule_reads_its_characters() {
  let rules = rules_of(r#"["ab1", "ab"]"#);

  assert_eq!(rules[0].class_name(), &ClassName::from("a"));
  assert_eq!(rules[0].style(), &RuleStyle::default());
  assert_eq!(rules[0].priority(), 1.0);
  assert_eq!(rules[1].class_name(), &ClassName::from("a"));
  assert!(rules[1].priority().is_nan());
  // One character has no style item, and `undefined` cannot be spread.
  assert_eq!(
    answer(json(r#"["a"]"#), true.into()),
    Err(ProcessStylexRulesError::NoStyleObject {
      index: 0,
      kind: "undefined"
    })
  );

  assert_eq!(
    answer(json(r#"["ab"]"#), true.into()),
    Err(ProcessStylexRulesError::TextNotAString {
      direction: TextDirection::Ltr,
      class_name: "a".into(),
      kind: "undefined"
    })
  );
  assert_eq!(
    answer(
      json(r#"["ab1", ["a", {"ltr": ".a{x:0}"}, 1]]"#),
      json(r#"{"useLegacyClassnamesSort": true, "useLayers": false}"#)
    ),
    Ok(".a{x:0}".into())
  );
}

/// A rule that declares half a constant is a style rule, so its missing `ltr`
/// is refused where the sort reads it (V8 first compares the second rule with
/// the first, so the first rule's text is read second). An empty tuple has no
/// style to spread. Both throw in the plugin.
#[test]
fn a_half_declared_constant_and_an_empty_tuple_are_refused() {
  for half in [r#"{"constKey": "xA"}"#, r#"{"constVal": "red"}"#] {
    assert_eq!(
      answer(
        json(&format!(
          r#"[["xA", {half}, 0], ["x1", {{"ltr": ".x1{{a:0}}"}}, 0]]"#
        )),
        true.into()
      ),
      Err(ProcessStylexRulesError::TextNotAString {
        direction: TextDirection::Ltr,
        class_name: "xA".into(),
        kind: "undefined"
      }),
      "{half}"
    );
  }

  assert_eq!(
    answer(json("[[]]"), true.into()),
    Err(ProcessStylexRulesError::NoStyleObject {
      index: 0,
      kind: "undefined"
    })
  );
}

/// `{ ...styleObj }` copies an object's properties, and nothing of a string,
/// a number or an array: the rule has no `ltr`. `null` and `undefined` cannot
/// be spread, and a rule that is not iterable cannot be destructured.
#[test]
fn a_style_that_is_not_an_object_reads_as_the_spread_reads_it() {
  for style in [r#""ab""#, "5", "true", r#"["a"]"#] {
    let rules = rules_of(&format!(r#"[["x1", {style}, 3000]]"#));
    assert_eq!(rules[0].style(), &RuleStyle::default(), "{style}");
  }

  assert_eq!(
    answer(
      json(r#"[["x0", {"ltr": ""}], ["x1", null, 3000]]"#),
      true.into()
    ),
    Err(ProcessStylexRulesError::NoStyleObject {
      index: 1,
      kind: "null"
    })
  );
  assert_eq!(
    answer(json(r#"[["x1"]]"#), true.into()),
    Err(ProcessStylexRulesError::NoStyleObject {
      index: 0,
      kind: "undefined"
    })
  );
  for (rule, kind) in [
    ("5", "a number"),
    ("null", "null"),
    ("true", "a boolean"),
    (r#"{"0": "x1"}"#, "an object"),
  ] {
    assert_eq!(
      read_rules(json(&format!("[{rule}]"))),
      Err(ProcessStylexRulesError::RuleNotAList { index: 0, kind }),
      "{rule}"
    );
  }
  assert_eq!(
    read_rules(array(vec![JsValue::Undefined])),
    Err(ProcessStylexRulesError::RuleNotAList {
      index: 0,
      kind: "undefined"
    })
  );
  assert_eq!(
    ProcessStylexRulesError::RuleNotAList {
      index: 3,
      kind: "a number"
    }
    .to_string(),
    "rule 3 is a number, not a [className, style, priority] list"
  );
  assert_eq!(
    ProcessStylexRulesError::NoStyleObject {
      index: 1,
      kind: "null"
    }
    .to_string(),
    "rule 1 has null in place of its style object"
  );
}

/// `rules.length === 0` returns before anything else is read: an empty array,
/// an empty string, or an object whose `length` is 0. Any other value that is
/// not an array has no `filter`, and the reference throws. Measured on the
/// plugin.
#[test]
fn the_rules_are_an_array_or_have_no_length() {
  for rules in [JsValue::from(""), json("[]"), json(r#"{"length": 0}"#)] {
    assert_eq!(answer(rules, true.into()), Ok(String::new()));
  }

  for (rules, kind) in [
    (JsValue::from("ab"), "a string"),
    (json(r#"{"length": 1}"#), "an object"),
    (json(r#"{"length": "0"}"#), "an object"),
    (JsValue::Null, "null"),
    (JsValue::Undefined, "undefined"),
    (5.0.into(), "a number"),
  ] {
    assert_eq!(
      answer(rules, true.into()),
      Err(ProcessStylexRulesError::RulesNotAList(kind))
    );
  }
  assert_eq!(
    ProcessStylexRulesError::RulesNotAList("null").to_string(),
    "the rules are null, not a list"
  );
}

/// `filter` skips a hole. A list of holes alone still has a length, so the
/// reference goes on and writes the `@layer` header with no group; so does a
/// list of constants alone. Measured on the plugin.
#[test]
fn a_hole_in_the_rules_is_skipped() {
  let rule_value = tuple("x1".into(), style(".x1{a:0}"), 3000.0.into());

  assert_eq!(
    answer_sheet(array(vec![JsValue::Hole, rule_value]), true.into()),
    "\n@layer priority1;\n@layer priority1{\n.x1{a:0}\n}"
  );
  let holes = || array(vec![JsValue::Hole, JsValue::Hole]);
  assert_eq!(answer_sheet(holes(), true.into()), "\n@layer ;\n");
  assert_eq!(
    answer_sheet(
      holes(),
      json(r#"{"useLayers": {"before": ["a"], "after": ["b"]}}"#)
    ),
    "\n@layer a, b;\n"
  );
  assert_eq!(answer_sheet(holes(), false.into()), "");
  assert_eq!(
    answer_sheet(
      json(r#"[["xA", {"constKey": "xA", "constVal": "red"}, 0]]"#),
      true.into()
    ),
    "\n@layer ;\n"
  );
  // Inside a rule, a hole is `undefined`.
  let rules = match read_rules(array(vec![array(vec![
    "x1".into(),
    style(".x1{a:0}"),
    JsValue::Hole,
  ])])) {
    Ok(read) => read.rules,
    Err(error) => panic!("the rule reads: {error}"),
  };
  assert!(rules[0].priority().is_nan());
}

/// A style rule's `ltr` is read with string methods where the sort compares
/// it and where it is written. A rule that the sort decides by priority alone
/// and that a later rule of its class replaces in its group is never read, so
/// a text that is not a string is refused only where it is read. Measured on
/// the plugin.
#[test]
fn an_ltr_text_that_is_not_a_string_is_refused_where_it_is_read() {
  for ltr in ["5", "null", "true", "{}", "[\".x{}\"]"] {
    let rules = json(&format!(
      r#"[["x1", {{"ltr": {ltr}}}, 1000], ["x2", {{"ltr": ".x2{{a:0}}"}}, 1500]]"#
    ));
    assert!(
      matches!(
        answer(rules, false.into()),
        Err(ProcessStylexRulesError::TextNotAString {
          direction: TextDirection::Ltr,
          ..
        })
      ),
      "{ltr}"
    );
  }

  // Replaced in its group by the later rule of its class.
  assert_eq!(
    answer(
      json(r#"[["x1", {"ltr": 5}, 1000], ["x1", {"ltr": ".x1{a:0}"}, 1500]]"#),
      false.into()
    ),
    Ok(".x1{a:0}".into())
  );
  // The legacy sort compares class names, not texts.
  assert_eq!(
    answer(
      json(r#"[["x1", {"ltr": 5}, 1000], ["x1", {"ltr": ".x1{a:0}"}, 1000]]"#),
      json(r#"{"useLegacyClassnamesSort": true}"#)
    ),
    Ok(".x1{a:0}".into())
  );
  // Compared within one priority: refused in the sort, which compares the
  // second rule with the first, so either one can be the refused operand.
  assert_eq!(
    answer(
      json(r#"[["x1", {"ltr": ".x1{a:0}"}, 1000], ["x2", {"ltr": 5}, 1000]]"#),
      false.into()
    ),
    Err(ProcessStylexRulesError::TextNotAString {
      direction: TextDirection::Ltr,
      class_name: "x2".into(),
      kind: "a number"
    })
  );
  assert_eq!(
    answer(
      json(r#"[["x1", {"ltr": 5}, 1000], ["x2", {"ltr": ".x2{a:0}"}, 1000]]"#),
      false.into()
    ),
    Err(ProcessStylexRulesError::TextNotAString {
      direction: TextDirection::Ltr,
      class_name: "x1".into(),
      kind: "a number"
    })
  );
  // `String(ltr)` finds a logical float variable in an array text, though the
  // text is replaced before it is written.
  assert!(
    answer_sheet(
      json(
        r#"[["x1", {"ltr": ["var(--stylex-logical-start)"]}, 1000], ["x1", {"ltr": ".x1{a:0}"}, 1500]]"#
      ),
      false.into()
    )
    .starts_with(":root, [dir=\"ltr\"] {")
  );
  assert_eq!(
    ProcessStylexRulesError::TextNotAString {
      direction: TextDirection::Ltr,
      class_name: "x1".into(),
      kind: "a number"
    }
    .to_string(),
    "the ltr text of class x1 is a number, not a string"
  );
}

/// A style rule's `rtl` is read behind `if (rtlRule)`: a falsy value is no
/// RTL text, and a truthy value that is not a string is refused where it is
/// written. Measured on the plugin (`rtl: false` and `rtl: 0` write the LTR
/// text alone; `rtl: true`, `{}` and `[]` throw).
#[test]
fn a_falsy_rtl_is_no_rtl_text_and_a_truthy_non_string_is_refused() {
  for rtl in ["null", "false", "0", "0.0", "\"\""] {
    let rules = json(&format!(
      r#"[["x1", {{"ltr": ".x1{{a:0}}", "rtl": {rtl}}}, 3000]]"#
    ));
    assert_eq!(answer(rules, false.into()), Ok(".x1{a:0}".into()), "{rtl}");
  }
  let nan_rtl = array(vec![tuple(
    "x1".into(),
    JsValue::object([
      ("ltr".to_string(), ".x1{a:0}".into()),
      ("rtl".to_string(), f64::NAN.into()),
    ]),
    3000.0.into(),
  )]);
  assert_eq!(answer(nan_rtl, false.into()), Ok(".x1{a:0}".into()));

  for (rtl, kind) in [
    ("true", "a boolean"),
    ("5", "a number"),
    ("-5", "a number"),
    ("{}", "an object"),
    ("[]", "an array"),
  ] {
    for config in ["false", "true", r#"{"legacyDisableLayers": true}"#] {
      let rules = json(&format!(
        r#"[["x1", {{"ltr": ".x1{{a:0}}", "rtl": {rtl}}}, 3000]]"#
      ));
      assert_eq!(
        answer(rules, json(config)),
        Err(ProcessStylexRulesError::TextNotAString {
          direction: TextDirection::Rtl,
          class_name: "x1".into(),
          kind
        }),
        "{rtl} under {config}"
      );
    }
  }
  // Replaced in its group by the later rule of its class.
  assert_eq!(
    answer(
      json(r#"[["x1", {"ltr": ".x1{a:0}", "rtl": 5}, 1000], ["x1", {"ltr": ".x1{b:0}"}, 1500]]"#),
      false.into()
    ),
    Ok(".x1{b:0}".into())
  );
  assert_eq!(
    ProcessStylexRulesError::TextNotAString {
      direction: TextDirection::Rtl,
      class_name: "x1".into(),
      kind: "a boolean"
    }
    .to_string(),
    "the rtl text of class x1 is a boolean, not a string"
  );
}

// ==================== the values a rule holds, coerced as the reference coerces them ====================

/// The priority is read as the arithmetic reads it (`Number(value)`): an array
/// through its text, an object as `NaN`.
#[test]
fn a_priority_is_read_as_javascript_arithmetic_reads_it() {
  let priority = |spelling: &str| -> f64 {
    rules_of(&format!(r#"[["x1", {{"ltr": ".x1{{a:0}}"}}, {spelling}]]"#))[0].priority()
  };

  assert_eq!(priority("3000"), 3000.0);
  assert_eq!(priority("3000.5"), 3000.5);
  assert_eq!(priority("-5"), -5.0);
  assert_eq!(priority("null"), 0.0);
  assert_eq!(priority("true"), 1.0);
  assert_eq!(priority("false"), 0.0);
  assert_eq!(priority(r#""3000""#), 3000.0);
  assert_eq!(priority(r#"" 1e3 ""#), 1000.0);
  assert_eq!(priority(r#""""#), 0.0);
  assert_eq!(priority(r#""0x10""#), 16.0);
  assert_eq!(priority(r#""0o17""#), 15.0);
  assert_eq!(priority(r#""0b101""#), 5.0);
  assert_eq!(priority(r#""Infinity""#), f64::INFINITY);
  assert_eq!(priority(r#""-Infinity""#), f64::NEG_INFINITY);
  assert_eq!(priority(r#""+.5""#), 0.5);
  assert_eq!(priority("[]"), 0.0);
  assert_eq!(priority("[1000]"), 1000.0);
  assert_eq!(priority(r#"["2000"]"#), 2000.0);
  for spelling in [
    r#""abc""#,
    r#""inf""#,
    r#""nan""#,
    r#""1,000""#,
    r#""-0x10""#,
    r#""+0b1""#,
    r#""0x""#,
    r#""0xg""#,
    r#""1e""#,
    r#""+Infinityx""#,
    r#""-0o7""#,
    r#""0b2""#,
    "[1, 2]",
    "{}",
  ] {
    assert!(priority(spelling).is_nan(), "{spelling} is NaN");
  }
  // A priority past 2^63 keeps its value.
  assert_eq!(priority("1e21"), 1e21);
}

/// A priority that is no number writes the sheet the reference writes: an
/// array or an object priority is coerced, and an `undefined` one is `NaN`
/// where a `null` one is 0. Measured on the plugin.
#[test]
fn a_loose_priority_writes_the_sheet_the_reference_writes() {
  let layers = ProcessStylexRulesConfig::from(true);
  let sheet_of = |rules: &str, config: &ProcessStylexRulesConfig| sheet(&rules_of(rules), config);

  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{a:0}"}, [1000]], ["x2", {"ltr": ".x2{b:0}"}, 3000]]"#,
      &layers
    ),
    "\n@layer priority1, priority2;\n@layer priority1{\n.x1{a:0}\n}\n@layer priority2{\n.x2{b:0}\n}"
  );
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{a:0}"}, {}], ["x2", {"ltr": ".x2{b:0}"}, 3000]]"#,
      &layers
    ),
    "\n@layer priority1, priority2;\n.x1{a:0}\n@layer priority2{\n.x2{b:0}\n}"
  );
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{a:0}"}, [1, 2]], ["x2", {"ltr": ".x2{b:0}"}, 3000]]"#,
      &ProcessStylexRulesConfig::default()
    ),
    ".x1{a:0}\n.x2:not(#\\#){b:0}"
  );
  assert_eq!(
    sheet_of(r#"[["x1", {"ltr": ".x1{color:red}"}]]"#, &layers),
    "\n@layer priority1;\n.x1{color:red}"
  );
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{a:0}"}, 1000], ["x2", {"ltr": ".x2{b:0}"}], ["x3", {"ltr": ".x3{c:0}"}, 1000]]"#,
      &layers
    ),
    "\n@layer priority1, priority2, priority3;\n@layer priority1{\n.x1{a:0}\n}\n.x2{b:0}\n@layer priority3{\n.x3{c:0}\n}"
  );
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{a:0}"}, 1000], ["x2", {"ltr": ".x2{b:0}"}], ["x3", {"ltr": ".x3{c:0}"}, 3000], ["x4", {"ltr": ".x4{d:0}"}, 2000]]"#,
      &layers
    ),
    "\n@layer priority1, priority2, priority3, priority4;\n@layer priority1{\n.x1{a:0}\n}\n.x2{b:0}\n@layer priority3{\n.x4{d:0}\n}\n@layer priority4{\n.x3{c:0}\n}"
  );
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{a:0}"}, 1000], ["x2", {"ltr": ".x2{b:0}"}, null]]"#,
      &layers
    ),
    "\n@layer priority1, priority2;\n.x2{b:0}\n@layer priority2{\n.x1{a:0}\n}"
  );
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{color:red}"}, "3000"], ["x2", {"ltr": ".x2{margin:0}"}, 1000]]"#,
      &layers
    ),
    "\n@layer priority1, priority2;\n@layer priority1{\n.x2{margin:0}\n}\n@layer priority2{\n.x1{color:red}\n}"
  );
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{color:red}"}, true], ["x2", {"ltr": ".x2{margin:0}"}, 1000]]"#,
      &layers
    ),
    "\n@layer priority1, priority2;\n@layer priority1{\n.x1{color:red}\n}\n@layer priority2{\n.x2{margin:0}\n}"
  );
  // A constant rule with no priority declares its constant all the same.
  assert_eq!(
    sheet_of(
      r#"[["xA", {"constKey": "xA", "constVal": "red"}], ["x1", {"ltr": ".x1{color:var(--xA)}"}, 3000]]"#,
      &without_layers()
    ),
    ".x1{color:red}"
  );
  // `undefined` is NaN, so the two rules tie and keep their order.
  let undefined_priority = array(vec![
    tuple("x1".into(), style(".x1{a:0}"), 1000.0.into()),
    tuple("x2".into(), style(".x2{b:0}"), JsValue::Undefined),
  ]);
  assert_eq!(
    answer_sheet(undefined_priority, false.into()),
    ".x1{a:0}\n.x2:not(#\\#){b:0}"
  );
}

/// `firstPriority - secondPriority` is `NaN` for a missing priority and for
/// two equal infinities, and the reference returns it: the sort reads it as a
/// tie, and the later keys are not read. Measured on the plugin, where these
/// keep their order.
#[test]
fn a_nan_priority_comparison_is_a_tie_that_reads_no_later_key() {
  assert_eq!(
    sheet(
      &rules_of(r#"[["x1", {"ltr": ".x1{b:0}"}], ["x2", {"ltr": ".x2{a:0}"}]]"#),
      &ProcessStylexRulesConfig::default()
    ),
    ".x1{b:0}\n.x2:not(#\\#){a:0}"
  );
  assert_eq!(
    sheet(
      &rules_of(
        r#"[["x1", {"ltr": ".x1{b:0}"}, "Infinity"], ["x2", {"ltr": ".x2{a:0}"}, "Infinity"]]"#
      ),
      &ProcessStylexRulesConfig::default()
    ),
    ".x1{b:0}\n.x2{a:0}"
  );
  // Two infinite media bounds subtract to NaN too.
  assert_eq!(
    sheet(
      &[
        rule("x1", "@media (min-width: 1e999px){.x1{a:0}}", None, 3000.0),
        rule("x2", "@media (min-width: 1e999px){.x2{a:0}}", None, 3000.0),
      ],
      &ProcessStylexRulesConfig::default()
    ),
    "@media (min-width: 1e999px){.x1{a:0}}\n@media (min-width: 1e999px){.x2{a:0}}"
  );
}

/// Eighty rules with `NaN` priorities among them: the sort is not a
/// consistent order, so only V8's algorithm gives the reference's sheet.
/// Each pair is the class number and the number of `:not(#\#)` copies, which
/// is the index of its group. Measured on the plugin.
#[test]
fn eighty_rules_with_nan_priorities_sort_as_the_reference_sorts() {
  let properties = ["color", "margin", "padding", "top", "left"];
  let rules: Vec<Rule> = (0..80_usize)
    .map(|i| {
      let priority = if i % 7 == 3 || i % 11 == 5 {
        f64::NAN
      } else {
        ((i % 3) * 1000 + (i % 4) * 100) as f64
      };
      rule(
        &format!("x{i}"),
        &format!(".x{i}{{{}:{}}}", properties[(i * 7) % 5], (i * 13) % 17),
        None,
        priority,
      )
    })
    .collect();

  let written: Vec<String> = sheet(&rules, &ProcessStylexRulesConfig::default())
    .lines()
    .map(|line| {
      let class_number = line[2..]
        .split(|c: char| !c.is_ascii_digit())
        .next()
        .unwrap_or("");
      format!("{class_number}/{}", line.matches(":not").count())
    })
    .collect();

  assert_eq!(
    written.join(" "),
    "0/0 36/0 21/0 9/0 1/1 2/2 3/3 4/4 5/5 12/6 33/6 30/6 18/6 6/6 15/6 39/6 28/7 25/7 37/7 13/7 \
     22/7 34/7 7/7 20/8 8/8 10/9 32/10 29/10 26/10 14/10 11/10 16/11 17/12 19/13 35/14 23/14 24/15 \
     27/16 31/17 38/18 72/19 48/19 57/19 69/19 42/19 78/19 54/19 75/19 63/19 51/19 40/20 76/20 \
     64/20 61/20 70/20 58/20 46/20 55/20 67/20 43/20 68/21 56/21 44/21 41/21 45/22 79/23 65/24 \
     77/24 53/24 50/24 62/24 74/24 47/24 49/25 52/26 59/27 60/28 66/29 71/30 73/31"
  );
}

/// The class name is a `Map` key by `SameValueZero`: `NaN` is one key, `0`
/// and `-0` are one key, `true` and `'true'` are two, and two objects are two.
/// Measured on the plugin.
#[test]
fn a_class_name_is_a_map_key_by_same_value_zero() {
  let two = |first: JsValue, second: JsValue| {
    answer_sheet(
      array(vec![
        tuple(first, style(".a{a:0}"), 3000.0.into()),
        tuple(second, style(".b{a:0}"), 3000.0.into()),
      ]),
      false.into(),
    )
  };

  assert_eq!(two(f64::NAN.into(), f64::NAN.into()), ".b{a:0}");
  assert_eq!(two(0.0.into(), (-0.0).into()), ".b{a:0}");
  assert_eq!(two(1.0.into(), "1".into()), ".a{a:0}\n.b{a:0}");
  assert_eq!(two(true.into(), "true".into()), ".a{a:0}\n.b{a:0}");
  assert_eq!(two(json("{}"), json("{}")), ".a{a:0}\n.b{a:0}");
  assert_eq!(two(JsValue::Undefined, JsValue::Null), ".a{a:0}\n.b{a:0}");
  assert_eq!(
    answer_sheet(json(r#"[[12, {"ltr": ".x12{a:0}"}, 3000]]"#), true.into()),
    "\n@layer priority1;\n@layer priority1{\n.x12{a:0}\n}"
  );
}

/// A constant's key is only tested against `null`, and its value is spelled
/// with `String(...)` and `.toString()`: an array joins, an object is
/// `[object Object]`. Its name is `var(--${String(className)})`. Measured on
/// the plugin.
#[test]
fn a_constant_of_any_kind_substitutes_as_its_text() {
  let sheet_of = |rules: &str| answer_sheet(json(rules), false.into());

  assert_eq!(
    sheet_of(
      r#"[["xA", {"constKey": 0, "constVal": "red"}, 0], ["x1", {"ltr": ".x1{color:var(--xA)}"}, 3000]]"#
    ),
    ".x1{color:red}"
  );
  assert_eq!(
    sheet_of(
      r#"[["xA", {"constKey": "xA", "constVal": [1, [2, null]]}, 0], ["x1", {"ltr": ".x1{m:var(--xA)}"}, 3000]]"#
    ),
    ".x1{m:1,2,}"
  );
  assert_eq!(
    sheet_of(
      r#"[["xA", {"constKey": "xA", "constVal": {}}, 0], ["x1", {"ltr": ".x1{color:var(--xA)}"}, 3000]]"#
    ),
    ".x1{color:[object Object]}"
  );
  assert_eq!(
    sheet_of(
      r#"[[7, {"constKey": "k", "constVal": "red"}, 0], ["x1", {"ltr": ".x1{color:var(--7)}"}, 3000]]"#
    ),
    ".x1{color:red}"
  );
  assert_eq!(
    sheet_of(
      r#"[["xA", {"constKey": "xA", "constVal": {}}, 0], ["xB", {"constKey": "xB", "constVal": "a var(--xA) b"}, 0], ["x1", {"ltr": ".x1{c:var(--xB)}"}, 3000]]"#
    ),
    ".x1{c:a [object Object] b}"
  );
}

/// `classname1.localeCompare(classname2)` under the legacy sort: the receiver
/// must be a string, and the argument is spelled. V8's sort compares the
/// second item with the first, so the receiver is the later rule. Measured on
/// the plugin.
#[test]
fn the_legacy_sort_refuses_a_receiver_that_is_not_a_string() {
  let legacy = || json(r#"{"useLegacyClassnamesSort": true}"#);
  let rules = |first: &str, second: &str| {
    json(&format!(
      r#"[[{first}, {{"ltr": ".a{{a:0}}"}}, 3000], [{second}, {{"ltr": ".b{{a:0}}"}}, 3000]]"#
    ))
  };

  assert_eq!(
    answer(rules("1", r#""b""#), legacy()),
    Ok(".a{a:0}\n.b{a:0}".into())
  );
  assert_eq!(
    answer(rules(r#""b""#, "{}"), legacy()),
    Err(ProcessStylexRulesError::ClassNameNotAString {
      class_name: "[object Object]".into(),
      kind: "an object"
    })
  );
  assert_eq!(
    answer(rules("1", "2"), legacy()),
    Err(ProcessStylexRulesError::ClassNameNotAString {
      class_name: "2".into(),
      kind: "a number"
    })
  );
  // A different priority decides before the class names are read.
  assert_eq!(
    answer(
      json(r#"[[1, {"ltr": ".a{a:0}"}, 3000], ["b", {"ltr": ".b{a:0}"}, 1000]]"#),
      legacy()
    ),
    Ok(".b{a:0}\n.a:not(#\\#){a:0}".into())
  );
  assert_eq!(
    ProcessStylexRulesError::ClassNameNotAString {
      class_name: "2".into(),
      kind: "a number"
    }
    .to_string(),
    "the legacy sort cannot compare class 2: it is a number, not a string"
  );
}

/// The reference's grouping starts at level -1 with no group, so a first rule
/// at level -1 (a priority from -1000 to less than 0) has no group to join,
/// and the reference throws. A rule at that level after another one is
/// grouped. Measured on the plugin.
#[test]
fn a_first_rule_at_priority_level_minus_one_is_refused() {
  for priority in [-1.0, -0.5, -1000.0, -999.999] {
    assert_eq!(
      process_stylex_rules(
        &[
          rule("x1", ".x1{a:0}", None, priority),
          rule("x2", ".x2{a:0}", None, 10.0)
        ],
        &ProcessStylexRulesConfig::default()
      ),
      Err(ProcessStylexRulesError::FirstRuleBelowZero {
        class_name: "x1".into()
      }),
      "{priority}"
    );
  }

  assert_eq!(
    sheet(
      &[rule("x1", ".x1{a:0}", None, -1001.0)],
      &ProcessStylexRulesConfig::default()
    ),
    ".x1{a:0}"
  );
  assert_eq!(
    sheet(
      &[
        rule("x1", ".x1{a:0}", None, -2000.0),
        rule("x2", ".x2{a:0}", None, -1.0)
      ],
      &ProcessStylexRulesConfig::default()
    ),
    ".x1{a:0}\n.x2:not(#\\#){a:0}"
  );
  assert_eq!(
    sheet(
      &[
        rule("x0", ".x0{a:0}", None, f64::NAN),
        rule("x1", ".x1{a:0}", None, -1.0)
      ],
      &ProcessStylexRulesConfig::default()
    ),
    ".x0{a:0}\n.x1:not(#\\#){a:0}"
  );
  assert_eq!(
    sheet(
      &[rule("x1", ".x1{a:0}", None, -0.0)],
      &ProcessStylexRulesConfig::default()
    ),
    ".x1{a:0}"
  );
  assert_eq!(
    ProcessStylexRulesError::FirstRuleBelowZero {
      class_name: "x1".into()
    }
    .to_string(),
    "class x1 sorts first with a priority from -1000 to less than 0, which cannot be grouped; use \
     a priority of 0 or more"
  );
}

// ==================== extreme inputs ====================

/// Many rules, many constants and long texts write one sheet without a
/// refusal, and every rule is written once.
#[test]
fn a_large_sheet_writes_every_rule_once() {
  let mut rules: Vec<Rule> = (0..300)
    .map(|i| constant(&format!("c{i}"), text(&format!("{i}px"))))
    .collect();
  rules.extend((0..20_000).map(|i| {
    let ltr = format!(
      "@media (min-width: {}px){{.x{i}{{margin:var(--c{}) {}}}}}",
      i % 50,
      i % 300,
      "a".repeat(i % 40)
    );
    rule(&format!("x{i}"), &ltr, None, (i % 5) as f64 * 1000.0)
  }));

  let written = sheet(&rules, &ProcessStylexRulesConfig::from(true));

  for i in [0, 1, 299, 19_999] {
    assert_eq!(written.matches(&format!(".x{i}{{")).count(), 1, "x{i}");
  }
  assert!(!written.contains("var(--c"));
  assert_eq!(written.matches("@layer priority").count(), 5);
}

/// A long chain of constants, each the next one's reference, resolves to the
/// last value.
#[test]
fn a_long_chain_of_constants_resolves() {
  let mut rules: Vec<Rule> = (0..200)
    .map(|i| constant(&format!("c{i}"), text(&format!("var(--c{})", i + 1))))
    .collect();
  rules.push(constant("c200", text("red")));
  rules.push(rule("x1", ".x1{color:var(--c0)}", None, 3000.0));

  assert_eq!(sheet(&rules, &without_layers()), ".x1{color:red}");
}
