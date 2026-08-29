//! A `defineVars` member read inside an expression the fold claims.
//!
//! A theme group is this compiler's own value, and it used to cross into the
//! engine as nothing more than the string its own `toString` answers. A string
//! has none of the group's members, so every expression that read one was handed
//! back — and once the whole of `Array.prototype` belonged to the fold, there was
//! nothing below to hand it back *to*: `[colors.glow, '0 0 1px'].join(' ')`
//! failed a build the reference compiler compiles.
//!
//! What crosses now is what the group is: a value whose members are derived from
//! its identity as they are read, rather than stored on it. So a member reads at
//! every depth a value can sit at, a key the walk never saw reads as well as one
//! it did, and the group asked for its own text still answers the variable-group
//! hash.
//!
//! Which path answers matters for one shape in particular. An element that is
//! itself an array has no reading below the fold — the dispatch there answers no
//! `Array.prototype` method — so `[[vars.primary, 'a'], 'b'].join('|')` folds
//! only because the group crosses and the engine reads the nesting. That is the
//! section below.
//!
//! Every class name and rule text below is measured output of
//! `@stylexjs/babel-plugin` 0.19.0 under the same options, so each case asserts
//! agreement with the reference compiler rather than agreement with this
//! compiler's own previous answer.

use swc_core::common::FileName;

use crate::utils::{
  prelude::*,
  transform::{
    assert_folds, assert_folds_with, assert_refuses, build_test_transform, fold_module as fold,
    stringify_js, theme_import_transform_with, ts_syntax,
  },
};

/// The theme module every case below imports, and the one member most of them
/// read: `vars.primary` is `var(--x1ineb92)` and `vars.secondary` is
/// `var(--x15zbqoj)`, both hashed from the theme file rather than from the file
/// that reads them.
const IMPORT: &str = "import { vars } from 'vars.stylex.js';";

// ──────────────────────────────────────────────
// The member, at every depth a value sits at
// ──────────────────────────────────────────────

/// The ticket's own example: a member as an element of an array the fold joins.
#[test]
fn a_member_is_an_element_of_a_joined_array() {
  assert_folds(
    IMPORT,
    "boxShadow: [vars.primary, '0 0 1px'].join(' '),",
    ".x1mrxl98{box-shadow:var(--x1ineb92) 0 0 1px}",
  );
}

/// A member as a template hole, inside a callback the engine runs per element.
#[test]
fn a_member_is_a_template_hole() {
  assert_folds(
    IMPORT,
    "content: ['a'].map(x => `${vars.primary}-${x}`).join(''),",
    ".x1le0975{content:var(--x1ineb92)-a}",
  );
}

/// A member as the value of an object property.
#[test]
fn a_member_is_an_object_value() {
  assert_folds(
    IMPORT,
    "color: Object.values({ a: vars.primary }).join(''),",
    ".x9263yc{color:var(--x1ineb92)}",
  );
}

/// A member as an argument of the call itself.
#[test]
fn a_member_is_an_argument() {
  assert_folds(
    IMPORT,
    "content: 'x'.concat(vars.primary),",
    ".x19kvkqu{content:xvar(--x1ineb92)}",
  );
}

/// Two members of one group in one expression, each answering its own variable.
#[test]
fn two_members_of_one_group_answer_separately() {
  assert_folds(
    IMPORT,
    "boxShadow: [vars.primary, vars.secondary].join(' '),",
    ".xw8l6dy{box-shadow:var(--x1ineb92) var(--x15zbqoj)}",
  );
}

/// A member as the receiver of the call — the read is the whole of the chain's
/// left-hand side rather than a value inside it.
#[test]
fn a_member_is_the_receiver_of_the_call() {
  assert_folds(
    IMPORT,
    "content: vars.primary.toUpperCase(),",
    ".xhu3y8r{content:\"VAR(--X1INEB92)\"}",
  );
}

/// A member reached off a value the expression produced rather than off the name
/// the module bound. Nothing at the read says which value it will land on, which
/// is exactly why the group has to carry its members rather than be predicted.
#[test]
fn a_member_is_read_off_an_element() {
  assert_folds(
    IMPORT,
    "content: [vars][0].primary.toUpperCase(),",
    ".xhu3y8r{content:\"VAR(--X1INEB92)\"}",
  );
}

/// A group reached through a second name of the author's own.
#[test]
fn a_member_is_read_through_a_named_group() {
  assert_folds(
    &format!("{} const g = vars;", IMPORT),
    "content: [g.primary].join(''),",
    ".xfbywio{content:var(--x1ineb92)}",
  );
}

/// A member of a member: the variable a group answers is a string, and the
/// string's own prototype folds on it.
#[test]
fn a_member_is_the_receiver_of_a_further_read() {
  assert_folds(
    IMPORT,
    "content: [vars.primary.slice(0,3)].join(''),",
    ".xb4xlmw{content:\"var\"}",
  );
}

/// A member nested two arrays deep, flattened back out.
#[test]
fn a_member_survives_being_nested_and_flattened() {
  assert_folds(
    IMPORT,
    "content: [[vars.primary]].flat().join(''),",
    ".xfbywio{content:var(--x1ineb92)}",
  );
}

/// The length of what a member answers, measured inside a callback.
#[test]
fn a_callback_measures_what_a_member_answers() {
  assert_folds(
    IMPORT,
    "zIndex: [vars.primary].map(v => v.length)[0],",
    ".x52sccv{z-index:15}",
  );
}

// ──────────────────────────────────────────────
// The array that holds an array
// ──────────────────────────────────────────────

/// An element that is itself an array, which is the shape that used to be
/// refused for reasons that have since gone away.
///
/// While a group crossed the bridge as nothing but the string its `toString`
/// answers, a member read beside one was handed back — and the dispatch below
/// answers no `Array.prototype` method at all, so `join` refused. What crosses
/// now is the group stand-in, so the whole expression folds in the engine, which
/// reads a nested element the way the language does. `Array` in the receiver's
/// place is the same expression written differently and folds the same.
#[test]
fn an_element_that_is_an_array_folds_in_the_engine() {
  assert_folds(
    IMPORT,
    "content: [[vars.primary, 'a'], 'b'].join('|'),",
    ".xb5soey{content:var(--x1ineb92),a|b}",
  );

  assert_folds(
    IMPORT,
    "content: Array([vars.primary, 'a'], 'b').join('|'),",
    ".xb5soey{content:var(--x1ineb92),a|b}",
  );
}

/// Every way of reaching a nested element, since one of them folding says
/// nothing about the rest: the engine owns the array whole, so the nesting reads
/// the same under each. Mostly methods, and two that are not — an index and
/// `length`, which reach an element and count one without calling anything.
///
/// `join` with no argument, `toString` and the `String` global are the same
/// coercion reached three ways, and all three flatten with a comma — which is
/// why the separator they answer differs from the one `join('|')` was given.
///
/// `flat` is measured further up as well, on a nesting with nothing beside it;
/// the rows here give it a sibling element and an argument, which is what makes
/// the depth it flattens observable.
#[test]
fn every_read_reaching_a_nested_element_folds() {
  let cases: &[(&str, &str)] = &[
    (
      "content: [[vars.primary, 'a'], 'b'].flat().join('|'),",
      ".x1y5rhow{content:var(--x1ineb92)|a|b}",
    ),
    (
      "content: [[[vars.primary], 'a'], 'b'].flat(Infinity).join('|'),",
      ".x1y5rhow{content:var(--x1ineb92)|a|b}",
    ),
    (
      "content: [[[vars.primary]], [['a']]].flat(2).join('|'),",
      ".x1si2cuh{content:var(--x1ineb92)|a}",
    ),
    (
      "content: [[vars.primary, 'a'], 'b'].toString(),",
      ".x1yfb1rq{content:var(--x1ineb92),a,b}",
    ),
    (
      "content: [[vars.primary, 'a'], 'b'].join(),",
      ".x1yfb1rq{content:var(--x1ineb92),a,b}",
    ),
    (
      "content: String([[vars.primary, 'a'], 'b']),",
      ".x1yfb1rq{content:var(--x1ineb92),a,b}",
    ),
    (
      "content: [[vars.primary, 'a']].concat(['b']).join('|'),",
      ".xb5soey{content:var(--x1ineb92),a|b}",
    ),
    (
      "content: [[vars.primary, 'a'], 'b'][0].join('|'),",
      ".x1si2cuh{content:var(--x1ineb92)|a}",
    ),
    (
      "content: [[vars.primary, 'a'], ['b']].map(x => x.join('-')).join('|'),",
      ".xxm0u1e{content:var(--x1ineb92)-a|b}",
    ),
    (
      "content: [[vars.primary, 'a'], []].filter(x => x.length).join('|'),",
      ".xsgtmi3{content:var(--x1ineb92),a}",
    ),
    (
      "content: [[vars.primary, 'a'], 'b'].reverse().join('|'),",
      ".x3hocjs{content:b|var(--x1ineb92),a}",
    ),
    (
      "content: [[vars.primary, 'a'], 'b'].slice(0, 1).join('|'),",
      ".xsgtmi3{content:var(--x1ineb92),a}",
    ),
    (
      "content: [[vars.primary], ['a']].sort().join('|'),",
      ".x1tdztuk{content:a|var(--x1ineb92)}",
    ),
    (
      "content: [[vars.primary], 'b'].indexOf('b'),",
      ".x1fy28pd{content:\"1px\"}",
    ),
    (
      "content: [[vars.primary, 'a'], 'b'].length,",
      ".x1sn2kax{content:\"2px\"}",
    ),
  ];

  for (body, rule) in cases {
    assert_folds(IMPORT, body, rule);
  }
}

/// The elements a nested one can be that carry no text: an empty array and the
/// two nullish values each read as nothing, so the separators around them are
/// all that is left.
#[test]
fn every_element_carrying_no_text_reads_as_the_empty_string() {
  assert_folds(
    IMPORT,
    "content: [[], [], vars.primary].join('|'),",
    ".x1lak4p8{content:||var(--x1ineb92)}",
  );

  assert_folds(
    IMPORT,
    "content: [[vars.primary, null, undefined], 'b'].join('|'),",
    ".xarj5bt{content:var(--x1ineb92),,|b}",
  );
}

/// An object in the same position is not an array, so it reads as the text the
/// language gives an object rather than as its contents — and the member inside
/// it is never reached.
#[test]
fn an_object_element_reads_as_the_text_the_language_gives_an_object() {
  assert_folds(
    IMPORT,
    "content: [{ a: vars.primary }, 'b'].join('|'),",
    ".x1lfvzc0{content:\"[object Object]|b\"}",
  );
}

/// Nesting wide and nesting deep, so the shape is pinned past the size a case
/// written out by hand reaches. Sixty nested elements and eight levels of
/// nesting both fold whole, and the deep one shows that only the outermost level
/// takes the separator it was given.
#[test]
fn a_wide_and_a_deep_nesting_both_fold_whole() {
  let elements = (0..60)
    .map(|index| format!("[vars.primary, {}]", index))
    .collect::<Vec<_>>()
    .join(", ");
  let joined = (0..60)
    .map(|index| format!("var(--x1ineb92),{}", index))
    .collect::<Vec<_>>()
    .join("|");

  assert_folds(
    IMPORT,
    &format!("content: [{}].join('|'),", elements),
    &format!(".x1axehh6{{content:{}}}", joined),
  );

  let mut nested = String::from("vars.primary");

  for level in 0..8 {
    nested = format!("[{}, {}]", nested, level);
  }

  assert_folds(
    IMPORT,
    &format!("content: {}.join('|'),", nested),
    ".xbt3iq1{content:var(--x1ineb92),0,1,2,3,4,5,6|7}",
  );
}

/// Two nested shapes neither compiler accepts, pinned so the agreement is on
/// the record rather than assumed. A hole inside a nested element is refused
/// before the fold, and a nested element the method answers a boolean for is
/// folded and then refused by the guard on style values. The sentences differ;
/// what matters is that no class name is invented on either side.
#[test]
fn a_nested_shape_neither_compiler_accepts_is_refused_here_too() {
  assert_refuses(
    IMPORT,
    "content: [[vars.primary, , 'a'], 'b'].join('|'),",
    "Could not resolve the code being evaluated.",
  );

  assert_refuses(
    IMPORT,
    "content: [[vars.primary], 'b'].includes('b'),",
    "A style value can only contain an array, string or number.",
  );
}

// ──────────────────────────────────────────────
// The key the read names
// ──────────────────────────────────────────────

/// A computed key written out.
#[test]
fn a_computed_key_reads_the_member_it_spells() {
  assert_folds(
    IMPORT,
    "content: [vars['primary']].join(''),",
    ".xfbywio{content:var(--x1ineb92)}",
  );
}

/// A computed key the module bound a name to.
#[test]
fn a_computed_key_reads_through_a_name() {
  assert_folds(
    &format!("{} const k = 'primary';", IMPORT),
    "content: [vars[k]].join(''),",
    ".xfbywio{content:var(--x1ineb92)}",
  );
}

/// A computed key no walk could have read: it is an element the callback is
/// handed, and there is one key per element. The group answers each as it is
/// read, which is what a stored set of members could not do.
#[test]
fn a_computed_key_is_a_callback_s_own_element() {
  assert_folds(
    IMPORT,
    "content: ['primary','secondary'].map(k => vars[k]).join('|'),",
    ".xokalxg{content:var(--x1ineb92)|var(--x15zbqoj)}",
  );
}

/// A key an author spelled as a variable name of their own is used as written
/// rather than hashed.
#[test]
fn a_dashed_key_names_the_variable_the_author_wrote() {
  assert_folds(
    IMPORT,
    "content: [vars['--custom']].join(''),",
    ".x1m5897t{content:var(--custom)}",
  );
}

/// A numeric key is the member its own string form spells.
#[test]
fn a_numeric_key_reads_as_the_name_it_spells() {
  assert_folds(
    IMPORT,
    "content: [vars[0]].join(''),",
    ".xraqffh{content:var(--xmqjkd)}",
  );
}

/// A group has no member it does not answer, so no read of one is `undefined` —
/// which is the difference between a value whose members are derived and one
/// whose members are stored.
#[test]
fn a_member_nobody_declared_still_answers() {
  assert_folds(
    IMPORT,
    "content: [vars.nothing === undefined].join(''),",
    ".x9g66vw{content:\"false\"}",
  );
}

// ──────────────────────────────────────────────
// The group as a whole
// ──────────────────────────────────────────────

/// The group carried as a value, joined: its own text is the variable-group
/// hash, not a member.
#[test]
fn a_carried_group_answers_its_own_hash() {
  assert_folds(
    IMPORT,
    "content: [vars].join(''),",
    ".xccb8e5{content:\"xop34xu\"}",
  );
}

/// The three other spellings that ask a group for its text.
#[test]
fn every_conversion_of_a_group_answers_the_same_hash() {
  let cases: &[&str] = &[
    "content: String(vars),",
    "content: vars.toString(),",
    "content: [`${vars}`].join(''),",
  ];

  for body in cases {
    assert_folds(IMPORT, body, ".xccb8e5{content:\"xop34xu\"}");
  }
}

/// A group and one of its members in the same expression: the group answers its
/// hash and the member answers its variable, which is what says the two readings
/// are separate.
#[test]
fn a_group_and_a_member_answer_separately() {
  assert_folds(
    IMPORT,
    "content: [vars, vars.primary].join('|'),",
    ".xdqok2s{content:xop34xu|var(--x1ineb92)}",
  );
}

/// A group as an object's value, converted where it stands.
#[test]
fn a_group_is_a_usable_object_value() {
  assert_folds(
    IMPORT,
    "content: Object.values({ a: vars }).join(''),",
    ".xccb8e5{content:\"xop34xu\"}",
  );
}

/// A group is an object rather than the string it used to cross as.
#[test]
fn a_group_answers_the_kind_it_is() {
  assert_folds(
    IMPORT,
    "content: [typeof vars].join(''),",
    ".xm7l0um{content:\"object\"}",
  );
}

/// A group answers every member and *holds* none, which the two questions that
/// ask about holding rather than reading both report.
#[test]
fn a_group_holds_none_of_the_members_it_answers() {
  assert_folds(
    IMPORT,
    "content: ['primary' in vars].join(''),",
    ".x9g66vw{content:\"false\"}",
  );

  for body in [
    "content: Object.keys(vars).join(','),",
    "content: Object.keys({...vars}).join(','),",
  ] {
    assert_folds(IMPORT, body, ".x14axycx{content:\"\"}");
  }
}

/// Every static that would write to a group is refused by name before a fold
/// begins, which is why nothing below the read traps a write.
#[test]
fn a_static_that_would_write_to_a_group_is_refused_by_name() {
  assert_refuses(
    IMPORT,
    "content: [Object.assign(vars, { a: 1 })].join(''),",
    "Cannot fold 'Object.assign' at compile time.",
  );
}

// ──────────────────────────────────────────────
// The import the read leaves behind
// ──────────────────────────────────────────────

/// A member read inside a fold is a read of the theme module, so the side-effect
/// import that compensates for tree shaking is queued for it — exactly as the
/// same read outside a call queues one.
#[test]
fn a_folded_member_read_leaves_the_theme_import_behind() {
  let output = fold(&format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      {}
      export const styles = stylex.create({{
        base: {{ boxShadow: [vars.primary, '0 0 1px'].join(' ') }},
      }});
    "#,
    IMPORT
  ));

  assert!(
    output.contains("import \"vars.stylex.js\";"),
    "expected the fold to leave the theme's side-effect import behind, got:\n{}",
    output
  );

  assert!(
    output.contains(".x1mrxl98{box-shadow:var(--x1ineb92) 0 0 1px}"),
    "expected the measured rule, got:\n{}",
    output
  );
}

// ──────────────────────────────────────────────
// The chain the fold cannot express
// ──────────────────────────────────────────────

/// A chain of two names read off a group is one *token*, not a read of a read:
/// `vars.brand.primary` names `brand.primary`, which is what a nested
/// `defineVars` group declares. Outside a call this compiler already answers it,
/// and it answers the same inside one.
#[test]
fn a_nested_token_read_answers_the_one_variable_it_names() {
  assert_folds(
    IMPORT,
    "color: vars.brand.primary,",
    ".x16ps5g4{color:var(--x1tr9ywo)}",
  );
}

/// The same chain as the receiver of a call, where the chain is resolved before
/// the method runs on what it answered.
#[test]
fn a_nested_token_read_is_a_usable_receiver() {
  assert_folds(
    IMPORT,
    "content: vars.brand.primary.toUpperCase(),",
    ".x1hbsnfa{content:\"VAR(--X1TR9YWO)\"}",
  );
}

/// The same chain inside an expression the fold claims, where the group has to
/// answer `brand` with a stand-in rather than with the variable `brand` alone
/// names — otherwise the printed source would read `primary` off that variable's
/// text.
#[test]
fn a_nested_token_read_folds_inside_a_claimed_expression() {
  assert_folds(
    IMPORT,
    "content: [vars.brand.primary].join(''),",
    ".x10luyfz{content:var(--x1tr9ywo)}",
  );
}

/// Which paths nest is read off the source and not off any value, so the same
/// name read off something the expression *produced* is still a member: the
/// element a callback is handed is the variable's text, and its length is the
/// text's.
#[test]
fn the_same_name_read_off_a_produced_value_is_still_a_member() {
  assert_folds(
    IMPORT,
    "zIndex: [vars.brand.primary].map(v => v.length)[0],",
    ".x52sccv{z-index:15}",
  );
}

/// A dotted chain answers a variable, and a variable is a string — so a call
/// that wanted a number from one gets what the language gives it: `Array()` of a
/// string is the array of a single element, exactly as the reference compiler
/// builds it.
#[test]
fn a_dotted_chain_answers_a_variable_and_not_the_number_beside_it() {
  assert_folds(
    IMPORT,
    "content: Array(vars.primary.length).length,",
    "content:\"1px\"",
  );
}

/// The same fold with tree-shake compensation off: the rule and the class name
/// are the option's business as little as the member read is, so only the
/// side-effect import goes.
#[test]
fn the_same_fold_answers_the_same_rule_without_treeshake_compensation() {
  let output = stringify_js(
    &format!(
      r#"
        import * as stylex from '@stylexjs/stylex';
        {}
        export const styles = stylex.create({{
          base: {{ boxShadow: [vars.primary, '0 0 1px'].join(' ') }},
        }});
      "#,
      IMPORT
    ),
    ts_syntax(),
    |tr| {
      build_test_transform(tr.comments.clone(), |b| {
        b.with_filename(FileName::Real("MyComponent.js".into()))
          .with_unstable_module_resolution(ModuleResolution::haste(None))
          .with_treeshake_compensation(false)
      })
    },
  );

  assert!(
    !output.contains("import \"vars.stylex.js\";"),
    "expected no side-effect import with compensation off, got:\n{}",
    output
  );

  assert!(
    output.contains("x1mrxl98"),
    "expected the same class the compensated build names, got:\n{}",
    output
  );
}

// ──────────────────────────────────────────────
// The name the read is written through
// ──────────────────────────────────────────────

/// A callback parameter that shadows the group is the callback's name, so the
/// chain written on it reads properties off whatever the callback was handed —
/// and says nothing about the group standing outside it.
#[test]
fn a_parameter_shadowing_a_group_reads_off_what_it_was_handed() {
  assert_folds(
    IMPORT,
    "content: [vars.brand.primary].map(vars => vars.length).join(''),",
    ".xkm8edl{content:\"15\"}",
  );
}

// ──────────────────────────────────────────────
// The naming the project asked for
// ──────────────────────────────────────────────

/// A variable is spelled the same whether the engine derived it or the
/// evaluator's own lookup did, so a project that asks for readable names gets
/// them from a folded read as much as from a bare one.
///
/// Asserted through a fold rather than through the derivation alone, because
/// what the two options travel across is the bridge: the group's stand-in is
/// built from plain values, and a fold under these options is the only thing
/// that shows they arrived.
#[test]
fn a_folded_member_is_spelled_the_way_the_project_asks() {
  let cases: &[(&str, &str)] = &[
    (
      "boxShadow: [vars.primary, '0 0 1px'].join(' '),",
      ".boxShadow-xu8oibe{box-shadow:var(--primary-x1ineb92) 0 0 1px}",
    ),
    (
      "content: [vars.brand.primary].join(''),",
      ".content-xxhlut6{content:var(--brand_primary-x1tr9ywo)}",
    ),
    // The group's own text is a hash and not a variable, so there is nothing to
    // make readable and the two options change nothing about it.
    (
      "content: [vars].join(''),",
      ".content-xccb8e5{content:\"xop34xu\"}",
    ),
  ];

  for (body, rule) in cases {
    assert_folds_with(IMPORT, body, rule, " under readable names", |module| {
      stringify_js(module, ts_syntax(), |tr| {
        theme_import_transform_with(tr.comments.clone(), |b| {
          b.with_debug(true).with_enable_debug_class_names(true)
        })
      })
    });
  }
}
