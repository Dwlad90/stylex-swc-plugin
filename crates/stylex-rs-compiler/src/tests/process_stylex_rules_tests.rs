//! The unpacking of the rules `fast-path-gate.ts` packs, and the config it
//! reads. The JS suite (`__test__/processStylexRules.spec.ts`) holds the whole
//! path against the reference; these pin the contract between the two halves.

use stylex_stylesheet::process::{LayerOptions, ProcessStylexRulesConfig, RuleStyle};

use super::{
  NativeConfig, RULE_KIND_CONSTANT as CONSTANT, RULE_KIND_STYLE as STYLE,
  RULE_KIND_STYLE_WITH_RTL as STYLE_WITH_RTL, catch_panic, unpack_rules,
};

fn config() -> NativeConfig {
  NativeConfig {
    use_layers: false,
    layers_before: Vec::new(),
    layers_after: Vec::new(),
    layer_prefix: String::new(),
    enable_ltr_rtl_comments: false,
    legacy_disable_layers: false,
    use_legacy_classnames_sort: false,
  }
}

#[test]
fn each_kind_unpacks_into_its_fields() {
  let texts = "x1.x1{a:b}x2.x2{c:d}.x2{d:c}x3x3keyé";
  let lengths = [2, 8, 2, 8, 8, 2, 2, 4];
  let kinds = [STYLE, STYLE_WITH_RTL, CONSTANT];
  let priorities = [3000.0, f64::NAN, 0.0];

  let Some(rules) = unpack_rules(texts, &lengths, &kinds, &priorities) else {
    panic!("the parts fit together");
  };

  assert_eq!(rules.len(), 3);
  assert_eq!(rules[0].class_name().spelling(), "x1");
  assert_eq!(
    rules[0].style(),
    &RuleStyle {
      ltr: ".x1{a:b}".into(),
      ..RuleStyle::default()
    }
  );
  assert_eq!(rules[0].priority(), 3000.0);
  assert_eq!(
    rules[1].style(),
    &RuleStyle {
      ltr: ".x2{c:d}".into(),
      rtl: ".x2{d:c}".into(),
      ..RuleStyle::default()
    }
  );
  assert!(rules[1].priority().is_nan());
  assert_eq!(
    rules[2].style(),
    &RuleStyle {
      const_key: "x3".into(),
      const_val: "keyé".into(),
      ..RuleStyle::default()
    }
  );
}

#[test]
fn no_rules_unpack_into_an_empty_list() {
  assert_eq!(unpack_rules("", &[], &[], &[]), Some(Vec::new()));
}

/// Each misfit is a fault of the gate, and answers `None`, never a panic.
#[test]
fn parts_that_do_not_fit_together_unpack_into_nothing() {
  // A priority more or less than the kinds.
  assert_eq!(unpack_rules("x.x{}", &[1, 4], &[STYLE], &[]), None);
  // A kind the gate does not write.
  assert_eq!(unpack_rules("x.x{}", &[1, 4], &[9], &[0.0]), None);
  // Too few texts for the kinds, at the class name and after it.
  assert_eq!(unpack_rules("", &[], &[STYLE], &[0.0]), None);
  assert_eq!(unpack_rules("x", &[1], &[STYLE], &[0.0]), None);
  assert_eq!(
    unpack_rules("x.x{}", &[1, 4], &[STYLE_WITH_RTL], &[0.0]),
    None
  );
  assert_eq!(unpack_rules("xk", &[1, 1], &[CONSTANT], &[0.0]), None);
  // A text that no rule reads.
  assert_eq!(unpack_rules("x.x{}y", &[1, 4, 1], &[STYLE], &[0.0]), None);
  // Lengths that do not cover the text.
  assert_eq!(unpack_rules("x.x{}", &[1, 9], &[STYLE], &[0.0]), None);
}

#[test]
fn the_config_reads_as_the_stylesheet_config() {
  assert_eq!(
    ProcessStylexRulesConfig::from(config()),
    ProcessStylexRulesConfig::default()
  );

  let config = NativeConfig {
    use_layers: true,
    layers_before: vec!["reset".into()],
    layers_after: vec!["utilities".into()],
    layer_prefix: "app".into(),
    enable_ltr_rtl_comments: true,
    legacy_disable_layers: true,
    use_legacy_classnames_sort: true,
  };
  assert_eq!(
    ProcessStylexRulesConfig::from(config),
    ProcessStylexRulesConfig {
      use_layers: Some(LayerOptions {
        before: vec!["reset".into()],
        after: vec!["utilities".into()],
        prefix: Some("app".into()),
      }),
      enable_ltr_rtl_comments: true,
      legacy_disable_layers: true,
      use_legacy_classnames_sort: true,
    }
  );
}

/// An empty prefix is no prefix: the gate packs a falsy prefix as empty.
#[test]
fn an_empty_prefix_is_no_prefix() {
  let config = NativeConfig {
    use_layers: true,
    ..config()
  };
  assert_eq!(
    ProcessStylexRulesConfig::from(config).use_layers,
    Some(LayerOptions::default())
  );
}

#[test]
fn a_panic_in_the_native_half_is_caught() {
  assert_eq!(catch_panic(|| 7), Some(7));
  assert_eq!(
    catch_panic(|| -> u8 { panic!("a fault in the native half") }),
    None
  );
}
