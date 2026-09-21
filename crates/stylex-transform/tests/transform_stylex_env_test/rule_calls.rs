//! `stylex.env.<name>` read inside the three rule calls.
//!
//! `keyframes`, `positionTry` and `viewTransitionClass` each evaluate their
//! argument with a function map of their own, and none of the three registers
//! the namespace name itself -- a bare `stylex` written where a static value
//! belongs must refuse rather than materialize into an object and drop the
//! declaration. The two-level read is answered off the map of what the
//! namespace has instead, so the value folds in every call while the bare name
//! goes on refusing.
//!
//! Both halves are measured here, for all three calls: a module that writes one
//! environment value in a `create` and the same value in a `keyframes` used to
//! stop on the second.

use crate::transform_stylex_env_test::brand_env;
use crate::utils::prelude::*;
use stylex_ast::ast::convertors::create_string_expr;
use stylex_structures::stylex_env::JSFunction;
use swc_core::ecma::ast::Expr;

use crate::utils::ast::convert_expr_to_str_wrapper;

/// An environment function, for the case that folds a call rather than a value.
fn mix_env() -> IndexMap<String, EnvEntry> {
  let mut env = IndexMap::new();

  env.insert(
    "colorMix".to_string(),
    EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
      let first = args
        .first()
        .and_then(convert_expr_to_str_wrapper)
        .unwrap_or_default();
      let second = args
        .get(1)
        .and_then(convert_expr_to_str_wrapper)
        .unwrap_or_default();

      create_string_expr(&format!("color-mix(in srgb, {}, {})", first, second))
    })),
  );

  env
}

fn stylex_transform(comments: TestComments, env: IndexMap<String, EnvEntry>) -> impl Pass + use<> {
  build_test_transform(comments, |b| b.with_env(env).with_runtime_injection())
}

stylex_test!(
  the_env_folds_inside_a_keyframes_call,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex.env.brandPrimary },
      to: { color: 'blue' },
    });
  "#
);

stylex_test!(
  the_env_folds_inside_a_position_try_call,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const anchor = stylex.positionTry({
      positionAnchor: '--anchor',
      top: stylex.env.brandPrimary,
    });
  "#
);

stylex_test!(
  the_env_folds_inside_a_view_transition_class_call,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const transition = stylex.viewTransitionClass({
      group: { color: stylex.env.brandPrimary },
    });
  "#
);

// The `create` call this branch already folded, beside a rule call reading the
// same value. This is the module the divergence was reported on: the first call
// compiled and the second stopped the build.
stylex_test!(
  one_module_folds_the_env_in_a_create_call_and_in_a_keyframes_call,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex.env.brandPrimary },
    });
    export const styles = stylex.create({
      root: { color: stylex.env.brandPrimary },
    });
  "#
);

// Every rule call of the module reads the environment, not only the first: the
// map is built once and shared, so a value lost on the way in would be lost for
// the rest of the file.
stylex_test!(
  every_rule_call_of_a_module_reads_the_env,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex.env.brandPrimary },
    });
    export const fadeOut = stylex.keyframes({
      to: { color: stylex.env.brandPrimary },
    });
    export const anchor = stylex.positionTry({
      positionAnchor: '--anchor',
      top: stylex.env.brandPrimary,
    });
    export const transition = stylex.viewTransitionClass({
      group: { color: stylex.env.brandPrimary },
    });
  "#
);

// The namespace under the name the import gave it, rather than under `stylex`.
stylex_test!(
  a_renamed_namespace_import_reads_the_env_inside_a_keyframes_call,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as sx from '@stylexjs/stylex';
    export const fadeIn = sx.keyframes({
      from: { color: sx.env.brandPrimary },
    });
  "#
);

// A named import writes `env` on its own, with no namespace to read it off.
stylex_test!(
  a_named_env_import_reads_the_env_inside_a_keyframes_call,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import { keyframes, env } from '@stylexjs/stylex';
    export const fadeIn = keyframes({
      from: { color: env.brandPrimary },
    });
  "#
);

// `stylex["env"]` names what `stylex.env` names, so the two read alike.
stylex_test!(
  a_computed_env_read_folds_inside_a_keyframes_call,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex['env'].brandPrimary },
    });
  "#
);

// A computed key held by a name reads the same entry the written name does.
stylex_test!(
  a_computed_env_read_through_a_binding_folds_inside_a_keyframes_call,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const key = 'env';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex[key].brandPrimary },
    });
  "#
);

// A computed key with no compile-time value is not a name, and the call says so
// rather than reading some other entry.
stylex_test_panic!(
  a_computed_key_with_no_value_refuses_inside_a_keyframes_call,
  "Only static values are allowed inside of a keyframes() call.",
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex[globalThis.pick()].brandPrimary },
    });
  "#
);

// An environment function, called inside a rule call rather than read.
stylex_test!(
  an_env_function_call_folds_inside_a_keyframes_call,
  |tr| stylex_transform(tr.comments.clone(), mix_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex.env.colorMix('red', 'blue') },
    });
  "#
);

// The environment value inside a fallback list, which is the one helper a rule
// call folds beside it.
stylex_test!(
  the_env_folds_inside_a_fallback_list_in_a_keyframes_call,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex.firstThatWorks('var(--a)', stylex.env.brandPrimary) },
    });
  "#
);

// A keyframes call with a step per percent, each reading the environment. The
// map is read once per step, so a module this size is what a per-read cost
// would show up in.
stylex_test!(
  a_keyframes_call_of_many_steps_reads_the_env_in_every_one,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      '0%': { color: stylex.env.brandPrimary },
      '10%': { backgroundColor: stylex.env.brandPrimary },
      '20%': { borderColor: stylex.env.brandPrimary },
      '30%': { outlineColor: stylex.env.brandPrimary },
      '40%': { caretColor: stylex.env.brandPrimary },
      '50%': { columnRuleColor: stylex.env.brandPrimary },
      '60%': { textDecorationColor: stylex.env.brandPrimary },
      '70%': { textEmphasisColor: stylex.env.brandPrimary },
      '80%': { fill: stylex.env.brandPrimary },
      '90%': { stroke: stylex.env.brandPrimary },
      '100%': { color: stylex.firstThatWorks('var(--a)', stylex.env.brandPrimary) },
    });
  "#
);

// The bare namespace, in each of the four calls that build a function map. It
// stands for no static value, and every one of them has to say so rather than
// write an object into the declaration and drop it.
stylex_test_panic!(
  a_bare_namespace_refuses_inside_a_keyframes_call,
  "Only static values are allowed inside of a keyframes() call.",
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex },
    });
  "#
);

stylex_test_panic!(
  a_bare_namespace_refuses_inside_a_position_try_call,
  "Only static values are allowed inside of a positionTry() call.",
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const anchor = stylex.positionTry({
      positionAnchor: '--anchor',
      top: stylex,
    });
  "#
);

stylex_test_panic!(
  a_bare_namespace_refuses_inside_a_view_transition_class_call,
  "Only static values are allowed inside of a viewTransitionClass() call.",
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const transition = stylex.viewTransitionClass({
      group: { color: stylex },
    });
  "#
);

stylex_test_panic!(
  a_bare_namespace_refuses_inside_a_create_call,
  "Invalid pseudo or at-rule.",
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: stylex },
    });
  "#
);

// A name the configuration has not got stands for no value, and the call says
// so. The reference implementation reads it as `undefined` and writes the step
// with the declaration left out, which is a rule nobody asked for under a name
// the module goes on using; refusing names the mistake while it can still be
// fixed.
stylex_test_panic!(
  an_env_name_the_configuration_has_not_got_refuses_inside_a_keyframes_call,
  "Only static values are allowed inside of a keyframes() call.",
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex.env.brandSecondary },
    });
  "#
);

// A dynamic style's parameter spelled `stylex` reads the namespace import, not
// the parameter: the function map is keyed by name rather than by binding, and
// an arrow parameter is injected into that same map. Deliberate on both
// compilers -- see "folded function map" in the evaluator's glossary -- and the
// reference implementation folds this source the same way.
//
// What decides between the two is the map, so the rule itself is pinned where
// the map is built, by `a_name_bound_to_a_value_wins_over_the_namespace_of_the_-
// same_name`. This case records what a whole module does with the source.
stylex_test!(
  a_dynamic_style_parameter_named_like_the_namespace_reads_the_import,
  |tr| stylex_transform(tr.comments.clone(), brand_env()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: (stylex) => ({ color: stylex.env.brandPrimary }),
    });
  "#
);
