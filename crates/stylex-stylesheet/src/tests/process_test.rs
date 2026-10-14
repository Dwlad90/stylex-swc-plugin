//! The branches of `process` the reference's own cases do not reach, and the
//! JavaScript built-ins the port stands in for.

use serde::de::{self, Unexpected, Visitor};

use crate::process::{
  ConfigVisitor, ConstantValue, JsNumber, JsNumberVisitor, LayerOptions, Loose, LooseVisitor,
  OrderedConstants, ProcessStylexRulesConfig, ProcessStylexRulesError, Rule, RuleStyle,
  RuleVisitor, TruthyVisitor, UseLayersVisitor, add_ancestor_selector, add_specificity_level,
  get_logical_float_vars, js_replace_all, js_replace_first, js_substitution, js_trim,
  process_stylex_rules, resolve_constant,
};

fn rule(class_name: &str, ltr: &str, rtl: Option<&str>, priority: f64) -> Rule {
  Rule(
    class_name.to_string(),
    RuleStyle {
      ltr: ltr.to_string(),
      rtl: rtl.map(str::to_string),
      ..RuleStyle::default()
    },
    priority,
  )
}

fn constant(class_name: &str, value: ConstantValue) -> Rule {
  Rule(
    class_name.to_string(),
    RuleStyle {
      const_key: Some(class_name.to_string()),
      const_val: Some(value),
      ..RuleStyle::default()
    },
    0.0,
  )
}

fn text(value: &str) -> ConstantValue {
  ConstantValue::String(value.to_string())
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

/// Reads `json` as the type, or fails the test naming the parse error.
fn read<T: serde::de::DeserializeOwned>(json: &str) -> T {
  match serde_json::from_str(json) {
    Ok(value) => value,
    Err(error) => panic!("the JSON is the shape the test expects: {error}"),
  }
}

// ==================== the config's spellings ====================

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
  assert_eq!(
    read::<ProcessStylexRulesConfig>("true"),
    ProcessStylexRulesConfig::from(true)
  );
  assert_eq!(
    read::<ProcessStylexRulesConfig>("false"),
    ProcessStylexRulesConfig::default()
  );
  assert_eq!(
    read::<ProcessStylexRulesConfig>("{}"),
    ProcessStylexRulesConfig::default()
  );
  assert_eq!(
    read::<ProcessStylexRulesConfig>(r#"{"useLayers": null}"#),
    ProcessStylexRulesConfig::default()
  );
  assert_eq!(
    read::<ProcessStylexRulesConfig>(r#"{"useLayers": false}"#),
    ProcessStylexRulesConfig::default()
  );
  assert_eq!(
    read::<ProcessStylexRulesConfig>(r#"{"useLayers": true}"#),
    ProcessStylexRulesConfig::from(true)
  );
  assert_eq!(
    read::<ProcessStylexRulesConfig>(
      r#"{"useLayers": {"prefix": "x"}, "enableLTRRTLComments": true, "legacyDisableLayers": true, "useLegacyClassnamesSort": true}"#
    ),
    ProcessStylexRulesConfig {
      use_layers: Some(LayerOptions {
        prefix: "x".to_string(),
        ..LayerOptions::default()
      }),
      enable_ltr_rtl_comments: true,
      legacy_disable_layers: true,
      use_legacy_classnames_sort: true,
    }
  );
}

#[test]
fn a_rule_reads_the_reference_tuple_with_an_absent_or_null_text() {
  let rules: Vec<Rule> = read(
    r#"[
      ["x1", {"ltr": ".x1{color:red}", "rtl": null}, 3000],
      ["c", {"constKey": "c", "constVal": 10}, 0],
      ["x2", {"ltr": ".x2{float:left}", "rtl": ".x2{float:right}"}, 3000.5]
    ]"#,
  );

  assert_eq!(rules[0], rule("x1", ".x1{color:red}", None, 3000.0));
  assert_eq!(rules[1], constant("c", ConstantValue::Number(10.0)));
  assert_eq!(rules[1].style().ltr, "");
  assert_eq!(
    rules[2],
    rule("x2", ".x2{float:left}", Some(".x2{float:right}"), 3000.5)
  );
  assert_eq!(rules[2].class_name(), "x2");
  assert_eq!(rules[2].priority(), 3000.5);
}

// ==================== the JavaScript built-ins ====================

#[test]
fn a_string_replacement_reads_the_dollar_patterns_of_get_substitution() {
  assert_eq!(js_substitution("[$&]", "m", "b", "a"), "[m]");
  assert_eq!(
    js_substitution("$`|$'", "m", "before", "after"),
    "before|after"
  );
  assert_eq!(js_substitution("$$", "m", "", ""), "$");
  assert_eq!(js_substitution("$1 $<n> $x $", "m", "", ""), "$1 $<n> $x $");
  assert_eq!(js_substitution("plain", "m", "", ""), "plain");
}

#[test]
fn replace_first_replaces_one_occurrence_or_none() {
  assert_eq!(js_replace_first("a-b-a", "a", "x"), "x-b-a");
  assert_eq!(js_replace_first("a-b-a", "z", "x"), "a-b-a");
  // The reference's `result.replace(match[0], replacement.toString())` passes a
  // string, so a `$&` in a constant's value doubles the match (0.19.1).
  assert_eq!(
    js_replace_first("var(--a)", "var(--a)", "$&$&"),
    "var(--a)var(--a)"
  );
}

#[test]
fn replace_all_replaces_every_occurrence_each_with_its_own_surroundings() {
  assert_eq!(js_replace_all("a-b-a", "a", "x"), "x-b-x");
  assert_eq!(js_replace_all("aaa", "a", "$`"), "aaa");
  assert_eq!(js_replace_all("aaa", "a", "$'"), "aaa");
  assert_eq!(
    js_replace_all("1 var(--a) 2 var(--a)", "var(--a)", "[$&]"),
    "1 [var(--a)] 2 [var(--a)]"
  );
  assert!(matches!(
    js_replace_all("a-b", "z", "x"),
    std::borrow::Cow::Borrowed("a-b")
  ));
}

// ==================== constants ====================

#[test]
fn a_number_constant_substitutes_as_javascript_spells_it() {
  let rules = [
    constant("xN", ConstantValue::Number(10.0)),
    constant("xF", ConstantValue::Number(1.5)),
    rule("xA", ".xA{margin:var(--xN)px var(--xF)px}", None, 3000.0),
  ];

  assert_eq!(sheet(&rules, &without_layers()), ".xA{margin:10px 1.5px}");
}

/// Measured on the plugin: a boolean constant substitutes as `String(flag)`,
/// and a `null` one is no constant, so a rule declaring it sorts as a style
/// (with an empty text) and a reference to it stays as written.
#[test]
fn a_boolean_constant_substitutes_as_its_word_and_a_null_one_is_no_constant() {
  let rules: Vec<Rule> = read(
    r#"[
      ["xT", {"constKey": "xT", "constVal": true}, 0],
      ["xF", {"constKey": "xF", "constVal": false}, 0],
      ["x1", {"ltr": ".x1{--a:var(--xT) var(--xF)}", "rtl": null}, 3000]
    ]"#,
  );

  assert_eq!(rules[0].style().const_val, Some(ConstantValue::Bool(true)));
  assert_eq!(
    resolve_constant(
      &ConstantValue::Bool(true),
      &OrderedConstants::default(),
      &mut Vec::new()
    ),
    Ok(ConstantValue::Bool(true))
  );
  assert_eq!(sheet(&rules, &without_layers()), ".x1{--a:true false}");

  let null_constant: Vec<Rule> = read(
    r#"[
      ["xN", {"constKey": "xN", "constVal": null, "ltr": "", "rtl": null}, 0],
      ["x1", {"ltr": ".x1{--a:var(--xN)}", "rtl": null}, 3000]
    ]"#,
  );

  assert_eq!(null_constant[0].style().const_val, None);
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
    ("var(--n)".to_string(), ConstantValue::Number(2.0)),
  ]
  .into_iter()
  .collect();

  assert_eq!(
    resolve_constant(&constants["var(--a)"], &constants, &mut Vec::new()),
    Ok(text("1px solid red var(--unknown)"))
  );
  assert_eq!(
    resolve_constant(&constants["var(--n)"], &constants, &mut Vec::new()),
    Ok(ConstantValue::Number(2.0))
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
    constant("xN", ConstantValue::Number(4.0)),
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
  assert_eq!(add_specificity_level(".x{color:red}", 0), ".x{color:red}");
  assert_eq!(
    add_specificity_level(".x{color:red}", 2),
    ".x:not(#\\#):not(#\\#){color:red}"
  );
  assert_eq!(
    add_specificity_level(".x::before{content:''}", 1),
    ".x:not(#\\#)::before{content:''}"
  );
  assert_eq!(
    add_specificity_level("@keyframes k{0%{left:0}}", 3),
    "@keyframes k{0%{left:0}}"
  );
  // `lastIndexOf('{')` is -1 for a text with no block, and `slice(0, -1)` then
  // keeps all but the last character.
  assert_eq!(add_specificity_level("abc", 1), "ab:not(#\\#)c");
  assert_eq!(add_specificity_level("", 1), ":not(#\\#)");
}

/// A text with no block splits before its last character, whatever its
/// width: the plugin writes `.a:not(#\#)é`. For a character outside the
/// Basic Multilingual Plane, JavaScript splits the UTF-16 surrogate pair
/// (`.a\uD83D:not(#\#)\uDE00`, two lone surrogates); a `str` cannot hold half
/// a pair, so the port splits before the whole character.
#[test]
fn the_specificity_level_splits_a_blockless_text_before_its_last_character() {
  assert_eq!(add_specificity_level(".aé", 1), ".a:not(#\\#)é");
  assert_eq!(add_specificity_level(".aé", 2), ".a:not(#\\#):not(#\\#)é");
  assert_eq!(add_specificity_level("é", 1), ":not(#\\#)é");
  assert_eq!(add_specificity_level(".a😀", 1), ".a:not(#\\#)😀");

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

  // JavaScript compares the low surrogates of two emoji; the port compares the
  // whole characters, and the two agree for these.
  let emoji = [
    rule("a", "ab:😁", None, 3000.0),
    rule("b", "ab:😀", None, 3000.0),
  ];
  assert_eq!(sheet(&emoji, &without_layers()), "ab:😀\nab:😁");
}

/// `String.prototype.trim` removes U+FEFF and keeps U+0085, where `str::trim`
/// does the opposite: the rename target of a constant holding a `var(...)` is
/// trimmed as JavaScript trims it. Measured on the plugin.
#[test]
fn the_rename_target_is_trimmed_as_javascript_trims() {
  // ECMA-262 WhiteSpace and LineTerminator, one by one.
  for character in [
    '\u{9}', '\u{A}', '\u{B}', '\u{C}', '\u{D}', ' ', '\u{A0}', '\u{1680}', '\u{2000}', '\u{2003}',
    '\u{200A}', '\u{2028}', '\u{2029}', '\u{202F}', '\u{205F}', '\u{3000}', '\u{FEFF}',
  ] {
    assert_eq!(
      js_trim(&format!("{character}x{character}")),
      "x",
      "U+{:04X}",
      character as u32
    );
  }
  // Unicode `White_Space` characters JavaScript keeps, and a zero-width space,
  // which neither set holds.
  for character in ['\u{85}', '\u{200B}'] {
    let text = format!("{character}x{character}");
    assert_eq!(js_trim(&text), text, "U+{:04X}", character as u32);
  }
  assert_eq!(js_trim("\u{FEFF} --t\u{A0}\u{3000}"), "--t");
  assert_eq!(js_trim(""), "");

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
      prefix: "x".to_string(),
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
      sheet(&rules, &read::<ProcessStylexRulesConfig>(spelling)),
      with_layers,
      "{spelling}"
    );
  }

  for spelling in [r#"{"useLayers": null}"#, r#"{"useLayers": false}"#, "{}"] {
    assert_eq!(
      sheet(&rules, &read::<ProcessStylexRulesConfig>(spelling)),
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
      read::<ProcessStylexRulesConfig>(spelling),
      ProcessStylexRulesConfig::default(),
      "{spelling}"
    );
  }

  assert_eq!(
    read::<ProcessStylexRulesConfig>("true"),
    ProcessStylexRulesConfig::from(true)
  );
}

/// `before ?? []`, `after ?? []`, `prefix ?? ''`: a `null` property is the
/// default. A value outside the declared type is refused, where the reference
/// would spread a string into characters or print a number (`CONTEXT.md`).
#[test]
fn a_null_layer_option_is_its_default_and_a_wrong_type_is_refused() {
  assert_eq!(
    read::<ProcessStylexRulesConfig>(
      r#"{"useLayers": {"before": null, "after": ["z"], "prefix": null}}"#
    ),
    ProcessStylexRulesConfig {
      use_layers: Some(LayerOptions {
        before: Vec::new(),
        after: vec!["z".to_string()],
        prefix: String::new(),
      }),
      ..ProcessStylexRulesConfig::default()
    }
  );

  for spelling in [
    r#"{"useLayers": {"before": "reset"}}"#,
    r#"{"useLayers": {"prefix": 5}}"#,
    r#"{"useLayers": {"after": [1]}}"#,
  ] {
    assert!(
      serde_json::from_str::<ProcessStylexRulesConfig>(spelling).is_err(),
      "{spelling} is outside the declared type"
    );
  }
}

/// A value a format hands the visitors outside JSON's types (bytes, a wide
/// integer) is refused with the message that names what each one reads.
#[test]
fn the_config_visitors_name_what_they_expect() {
  let refusal = |expected: &dyn de::Expected| {
    <serde_json::Error as de::Error>::invalid_type(Unexpected::Bytes(b"x"), expected).to_string()
  };

  assert_eq!(
    refusal(&ConfigVisitor),
    "invalid type: byte array, expected a boolean, null, or the options object"
  );
  assert_eq!(
    refusal(&UseLayersVisitor),
    "invalid type: byte array, expected a boolean, null, or the layer options object"
  );
  assert_eq!(
    refusal(&TruthyVisitor),
    "invalid type: byte array, expected a value read for its truthiness"
  );
}

/// A sequence or an object the reference reads no element of is drained, and
/// a fault inside it is the parse error it is.
#[test]
fn a_fault_inside_an_ignored_sequence_or_object_is_reported() {
  for spelling in [
    "[1,",
    r#"{"useLayers": [1,"#,
    r#"{"enableLTRRTLComments": [1,"#,
    r#"{"enableLTRRTLComments": {"a":"#,
  ] {
    assert!(
      serde_json::from_str::<ProcessStylexRulesConfig>(spelling).is_err(),
      "{spelling} is cut short"
    );
  }
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
    let config = read::<ProcessStylexRulesConfig>(&format!(
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
    let config = read::<ProcessStylexRulesConfig>(&format!(
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
      &read::<ProcessStylexRulesConfig>(r#"{"useLayers": false, "enableLTRRTLComments": 1}"#)
    ),
    "/* @ltr begin */.x1{a:0}/* @ltr end */\n/* @rtl begin */.x1{b:0}/* @rtl end */"
  );
  assert_eq!(
    sheet(
      &rules,
      &read::<ProcessStylexRulesConfig>(r#"{"useLayers": false, "enableLTRRTLComments": ""}"#)
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

/// `([key, { ...styleObj }, priority])`: the first three elements are read,
/// a fourth is not, and a missing priority is `NaN`. Measured on the plugin:
/// a four-element tuple and a two-element tuple both write the rule.
#[test]
fn a_rule_reads_three_elements_and_ignores_the_rest() {
  let four: Rule = read(r#"["x1", {"ltr": ".x1{color:red}"}, 3000, "extra"]"#);
  assert_eq!(four, rule("x1", ".x1{color:red}", None, 3000.0));

  let two: Rule = read(r#"["x1", {"ltr": ".x1{color:red}"}]"#);
  assert_eq!(two.class_name(), "x1");
  assert!(two.priority().is_nan());

  for spelling in [
    "[]",
    r#"["x1"]"#,
    r#"["x1", null, 3000]"#,
    r#"["x1", "style", 3000]"#,
  ] {
    assert!(
      serde_json::from_str::<Rule>(spelling).is_err(),
      "{spelling}: the reference throws on a rule with no style object"
    );
  }
  // A class name is a string; the reference writes a rule with a numeric one.
  assert!(serde_json::from_str::<Rule>(r#"[12, {"ltr": ".x1{color:red}"}, 3000]"#).is_err());
}

/// A style rule's `ltr` is read with `.replace`: a missing, `null` or
/// non-string one is refused as the reference throws. A constant rule is
/// filtered out before any text is read, so its texts may be anything.
#[test]
fn a_style_rule_needs_an_ltr_text_and_a_constant_rule_does_not() {
  for spelling in [
    r#"{"rtl": ".x1{color:red}"}"#,
    r#"{}"#,
    r#"{"ltr": null}"#,
    r#"{"ltr": 5}"#,
    r#"{"ltr": 0}"#,
    r#"{"ltr": false}"#,
    r#"{"ltr": true}"#,
    r#"{"ltr": []}"#,
    r#"{"ltr": {}}"#,
    r#"{"constKey": "xA"}"#,
    r#"{"constVal": "red"}"#,
  ] {
    assert!(
      serde_json::from_str::<RuleStyle>(spelling).is_err(),
      "{spelling}: the reference throws reading `ltr`"
    );
  }

  let constant: RuleStyle = read(r#"{"constKey": "xA", "constVal": "red", "ltr": 5, "rtl": 7}"#);
  assert_eq!(
    constant,
    RuleStyle {
      ltr: String::new(),
      rtl: None,
      const_key: Some("xA".to_string()),
      const_val: Some(text("red")),
    }
  );
  let constant_with_texts: RuleStyle =
    read(r#"{"constKey": "xA", "constVal": "red", "ltr": "a", "rtl": "b"}"#);
  assert_eq!(constant_with_texts.ltr, "a");
  assert_eq!(constant_with_texts.rtl, Some("b".to_string()));
}

/// A style rule's `rtl` is read behind `if (rtlRule)`: a falsy value is no
/// RTL text, a truthy value that is not a string is refused as the reference
/// throws. Measured on the plugin (`rtl: false` and `rtl: 0` write the LTR
/// text alone; `rtl: true`, `{}` and `[]` throw).
#[test]
fn a_falsy_rtl_is_no_rtl_text_and_a_truthy_non_string_is_refused() {
  for spelling in [
    r#"{"ltr": ".x1{a:0}", "rtl": null}"#,
    r#"{"ltr": ".x1{a:0}", "rtl": false}"#,
    r#"{"ltr": ".x1{a:0}", "rtl": 0}"#,
    r#"{"ltr": ".x1{a:0}", "rtl": 0.0}"#,
    r#"{"ltr": ".x1{a:0}"}"#,
  ] {
    let style: RuleStyle = read(spelling);
    assert_eq!(style.rtl, None, "{spelling}");
  }
  let empty: RuleStyle = read(r#"{"ltr": ".x1{a:0}", "rtl": ""}"#);
  assert_eq!(empty.rtl, Some(String::new()));

  for spelling in [
    r#"{"ltr": ".x1{a:0}", "rtl": true}"#,
    r#"{"ltr": ".x1{a:0}", "rtl": 5}"#,
    r#"{"ltr": ".x1{a:0}", "rtl": -5}"#,
    r#"{"ltr": ".x1{a:0}", "rtl": 1.5}"#,
    r#"{"ltr": ".x1{a:0}", "rtl": {}}"#,
    r#"{"ltr": ".x1{a:0}", "rtl": []}"#,
  ] {
    assert!(
      serde_json::from_str::<RuleStyle>(spelling).is_err(),
      "{spelling}: the reference throws reading `rtl`"
    );
  }
}

/// `NaN` is falsy where the reference reads a text behind `if (value)`.
#[test]
fn a_nan_text_is_falsy() {
  let Ok(Loose::Falsy) = LooseVisitor.visit_f64::<serde_json::Error>(f64::NAN) else {
    panic!("NaN is falsy");
  };
  let Ok(Loose::Other(kind)) = LooseVisitor.visit_f64::<serde_json::Error>(2.5) else {
    panic!("2.5 is truthy");
  };
  assert_eq!(kind, "a number");
  let Ok(Loose::Other(kind)) = LooseVisitor.visit_u64::<serde_json::Error>(7) else {
    panic!("7 is truthy");
  };
  assert_eq!(kind, "a number");
  let Ok(false) = TruthyVisitor.visit_f64::<serde_json::Error>(f64::NAN) else {
    panic!("NaN is falsy");
  };
  // A format that hands 0 as a signed integer (JSON hands it unsigned).
  let Ok(Loose::Falsy) = LooseVisitor.visit_i64::<serde_json::Error>(0) else {
    panic!("0 is falsy");
  };
  // A fault inside an array or an object the reference never reads is still
  // a parse error.
  for spelling in [
    r#"{"ltr": ".x1{a:0}", "rtl": [1,"#,
    r#"{"ltr": ".x1{a:0}", "rtl": {"a":"#,
    r#"{"constKey": "xA", "constVal": "red", "ltr": [1,"#,
  ] {
    assert!(
      serde_json::from_str::<RuleStyle>(spelling).is_err(),
      "{spelling} is cut short"
    );
  }
  let refusal =
    <serde_json::Error as de::Error>::invalid_type(Unexpected::Bytes(b"x"), &LooseVisitor)
      .to_string();
  assert_eq!(
    refusal,
    "invalid type: byte array, expected a string, or a value read for its truthiness"
  );
}

/// The priority is read as the arithmetic reads it (`Number(value)`).
#[test]
fn a_priority_is_read_as_javascript_arithmetic_reads_it() {
  let priority = |spelling: &str| -> f64 {
    let rule: Rule = read(&format!(r#"["x1", {{"ltr": ".x1{{a:0}}"}}, {spelling}]"#));
    rule.priority()
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
  ] {
    assert!(priority(spelling).is_nan(), "{spelling} is NaN");
  }
  let refusal =
    <serde_json::Error as de::Error>::invalid_type(Unexpected::Bytes(b"x"), &JsNumberVisitor)
      .to_string();
  assert_eq!(
    refusal,
    "invalid type: byte array, expected a number, or a value JavaScript arithmetic reads as one"
  );
  for spelling in ["[]", "[5]", "{}"] {
    assert!(
      serde_json::from_str::<JsNumber>(spelling).is_err(),
      "{spelling}: an array or an object is refused"
    );
  }
  // The refusal reaches the rule, and a fault after the third element too.
  for spelling in [
    r#"["x1", {"ltr": ".x1{a:0}"}, [5]]"#,
    r#"["x1", {"ltr": ".x1{a:0}"}, 3000, [1,"#,
  ] {
    assert!(
      serde_json::from_str::<Rule>(spelling).is_err(),
      "{spelling}"
    );
  }
  let refusal =
    <serde_json::Error as de::Error>::invalid_type(Unexpected::Bytes(b"x"), &RuleVisitor)
      .to_string();
  assert_eq!(
    refusal,
    "invalid type: byte array, expected a [className, { ltr, rtl, constKey, constVal }, priority] tuple"
  );
  // A signed non-decimal literal is NaN in every radix.
  for spelling in [r#""-0o7""#, r#""+0o7""#, r#""-0b1""#, r#""0o""#, r#""0b2""#] {
    assert!(priority(spelling).is_nan(), "{spelling} is NaN");
  }
}

/// A `NaN` priority is its own group outside every layer and compares equal
/// to everything in the sort; a `null` priority is 0; a numeric-string
/// priority is its number; two infinite priorities share a group. Measured on
/// the plugin.
#[test]
fn a_loose_priority_writes_the_sheet_the_reference_writes() {
  let sheet_of = |json: &str, config: &ProcessStylexRulesConfig| -> String {
    let rules: Vec<Rule> = read(json);
    sheet(&rules, config)
  };
  let layers = ProcessStylexRulesConfig::from(true);

  assert_eq!(
    sheet_of(r#"[["x1", {"ltr": ".x1{color:red}"}]]"#, &layers),
    "\n@layer priority1;\n.x1{color:red}"
  );
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{color:red}"}], ["x2", {"ltr": ".x2{margin:0}"}]]"#,
      &layers
    ),
    "\n@layer priority1, priority2;\n.x1{color:red}\n.x2{margin:0}"
  );
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{a:0}"}, 1000], ["x2", {"ltr": ".x2{b:0}"}], ["x3", {"ltr": ".x3{c:0}"}, 1000]]"#,
      &layers
    ),
    "\n@layer priority1, priority2, priority3;\n@layer priority1{\n.x1{a:0}\n}\n.x2{b:0}\n@layer priority3{\n.x3{c:0}\n}"
  );
  // The reference's comparator answers 0 for a NaN against anything, which is
  // not a consistent order; its sort and this one both leave these in place.
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{a:0}"}, 3000], ["x2", {"ltr": ".x2{b:0}"}], ["x3", {"ltr": ".x3{c:0}"}, 1000]]"#,
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
  assert_eq!(
    sheet_of(
      r#"[["x1", {"ltr": ".x1{a:0}"}, "Infinity"], ["x2", {"ltr": ".x2{b:0}"}, "Infinity"]]"#,
      &layers
    ),
    "\n@layer priority1;\n@layer priority1{\n.x1{a:0}\n.x2{b:0}\n}"
  );
  // A constant rule with no priority declares its constant all the same.
  assert_eq!(
    sheet_of(
      r#"[["xA", {"constKey": "xA", "constVal": "red"}], ["x1", {"ltr": ".x1{color:var(--xA)}"}, 3000]]"#,
      &without_layers()
    ),
    ".x1{color:red}"
  );
}
