//! Value-normalization coverage asserted at the public entry point, together
//! with the verdict `@stylexjs/babel-plugin` returns for each case.
//!
//! Every expectation here is what this compiler produces **today**, so the
//! suite is green before the normalization pipeline is replaced and stays a net
//! under that change. Alongside each one sits a [`Reference`](super::support::Reference) verdict taken
//! from the parity harness in `crates/stylex-rs-compiler/parity` — never from
//! judgement — recording whether the reference compiler agrees. A case marked
//! [`Reference::Diverges`](super::support::Reference::Diverges) is scheduled to
//! change; a case marked [`Reference::Same`](super::support::Reference::Same)
//! must survive untouched.
//!
//! Regenerate the verdicts with the two runs below — the second is what the
//! cases under [`rem_enabled_options`](super::support::rem_enabled_options) come from, since the harness defaults
//! the font-size option off and a default run cannot verdict them at all:
//!
//! ```sh
//! pnpm run --filter=@stylexswc/rs-compiler build
//! pnpm run --filter=@stylexswc/rs-compiler parity -- --json parity/results/default.json
//! pnpm run --filter=@stylexswc/rs-compiler parity -- --font-size-px-to-rem \
//!   --json parity/results/font-size-px-to-rem.json
//! ```
//!
//! The `Case`/`Reference` machinery and the shared `check` runner live in
//! `support`, alongside the same pieces used by `spacing_repair_parity_test`.
//!
//! Read `entries[].babel.declarations` for the reference spelling and
//! `entries[].rust.declarations` for this compiler's. Overlap with the older
//! `normalize_css_property_value_tests` module in `common_test.rs` is
//! deliberate: those assertions predate the harness and carry no verdict, so
//! they cannot say which of them a pipeline change is allowed to move.

use std::panic::{AssertUnwindSafe, catch_unwind};

use super::support::{check, default_options, diverges, panic_message, rem_enabled_options, same};
use crate::css::common::{MAX_VALUE_NESTING_DEPTH, normalize_css_property_value};

// ── Timings ──────────────────────────────────────────────────────────

/// Milliseconds shorten to seconds, except below the threshold where the
/// shorter spelling would not actually be shorter.
#[test]
fn converts_milliseconds_to_seconds() {
  check(
    &[
      same("transitionDuration", "500ms", ".5s"),
      same("transitionDuration", "1000ms", "1s"),
      same("transitionDuration", "0ms", "0s"),
      same("transitionDuration", "2s", "2s"),
    ],
    &default_options(),
  );
}

/// Under ten milliseconds the conversion is skipped.
#[test]
fn keeps_milliseconds_below_the_conversion_threshold() {
  check(
    &[same("transitionDuration", "5ms", "5ms")],
    &default_options(),
  );
}

// ── Zero dimensions ──────────────────────────────────────────────────

/// A zero length loses its unit; a zero fraction, percentage or angle keeps
/// one, because dropping it would change what the value means.
#[test]
fn normalizes_zero_dimensions_by_unit_kind() {
  check(
    &[
      same("margin", "0px", "0"),
      same("width", "0%", "0%"),
      same("gridTemplateColumns", "0fr", "0fr"),
      same("gridTemplateColumns", "0fr 1fr", "0fr 1fr"),
      same("image-resolution", "0dpi", "0"),
      same("color", "0Hz", "0"),
      same("color", "0unknown", "0"),
      same("image-orientation", "0rad", "0deg"),
    ],
    &default_options(),
  );
}

#[test]
fn leaves_non_zero_dimensions_alone() {
  check(
    &[
      same("margin", "10px", "10px"),
      same("width", "16px", "16px"),
      same("image-orientation", "90deg", "90deg"),
    ],
    &default_options(),
  );
}

/// A zero inside a function argument is left as the author wrote it — the
/// surrounding expression, not the property, decides what the unit means.
#[test]
fn leaves_zeros_inside_functions_alone() {
  check(
    &[
      same(
        "color",
        "calc(0 - var(--someVar))",
        "calc(0 - var(--someVar))",
      ),
      same(
        "color",
        "calc(0px - var(--someVar) + 10px)",
        "calc(0px - var(--someVar) + 10px)",
      ),
      same("width", "calc(0px + 10px)", "calc(0px + 10px)"),
    ],
    &default_options(),
  );
}

/// The one zero-angle case the reference compiler and this compiler disagree
/// on: upstream rewrites the unit inside the function, this compiler does not.
#[test]
fn diverges_on_a_zero_angle_inside_a_transform_function() {
  check(
    &[diverges(
      "transform",
      "rotate(0rad)",
      "rotate(0rad)",
      "rotate(0deg)",
    )],
    &default_options(),
  );
}

// ── Custom properties ────────────────────────────────────────────────

/// A custom property's value has no CSS grammar, so none of the value
/// normalizations apply to it.
#[test]
fn exempts_custom_properties_from_value_normalization() {
  check(
    &[
      same("--myVar", "0px", "0px"),
      same("--myVar", "backgroundColor", "backgroundColor"),
      same("--my-var", "foo", "foo"),
    ],
    &default_options(),
  );
}

// ── Leading zero ─────────────────────────────────────────────────────

#[test]
fn strips_the_leading_zero_from_a_decimal() {
  check(
    &[
      same("opacity", "0.5", ".5"),
      same("grid-column-start", "-1", "-1"),
    ],
    &default_options(),
  );
}

// ── Quotes ───────────────────────────────────────────────────────────

/// Single quotes are rewritten to double quotes; already-double-quoted values
/// pass through. The rewrite is the divergence — upstream keeps the author's
/// quote character.
#[test]
fn rewrites_single_quotes_to_double_quotes() {
  check(
    &[
      diverges("quotes", r#"'""'"#, r#""""#, r#"'""'"#),
      diverges("quotes", r#"'"123"'"#, r#""123""#, r#"'"123"'"#),
      same("quotes", r#""""#, r#""""#),
      same("quotes", r#""123""#, r#""123""#),
    ],
    &default_options(),
  );
}

#[test]
fn rewrites_quotes_in_grid_template_areas() {
  check(
    &[
      diverges(
        "gridTemplateAreas",
        r#"'"content"'"#,
        r#""content""#,
        r#"'"content"'"#,
      ),
      diverges(
        "gridTemplateAreas",
        r#"'"content" "sidebar"'"#,
        r#""content" "sidebar""#,
        r#"'"content" "sidebar"'"#,
      ),
      same("gridTemplateAreas", r#""content""#, r#""content""#),
      same(
        "gridTemplateAreas",
        r#""content" "sidebar""#,
        r#""content" "sidebar""#,
      ),
    ],
    &default_options(),
  );
}

// ── Camel-case value conversion ──────────────────────────────────────

/// Only the properties whose values name other properties get their values
/// hyphenated.
#[test]
fn hyphenates_camel_case_values_for_the_properties_that_take_property_names() {
  check(
    &[
      same("transitionProperty", "backgroundColor", "background-color"),
      same(
        "transitionProperty",
        "opacity, margin-top",
        "opacity,margin-top",
      ),
      same("transitionProperty", "--myVar", "--myVar"),
      same("willChange", "marginTop", "margin-top"),
      same("willChange", "backgroundColor", "background-color"),
    ],
    &default_options(),
  );
}

#[test]
fn leaves_ordinary_keyword_values_alone() {
  check(&[same("color", "red", "red")], &default_options());
}

// ── Whole-value shapes ───────────────────────────────────────────────

#[test]
fn normalizes_shadow_values() {
  check(
    &[
      same(
        "boxShadow",
        "0px 2px 4px var(--shadow-1)",
        "0 2px 4px var(--shadow-1)",
      ),
      same("boxShadow", "1px 1px #000", "1px 1px #000"),
    ],
    &default_options(),
  );
}

#[test]
fn normalizes_adjacent_custom_property_references() {
  check(
    &[
      same(
        "color",
        "var(--a) var(--b) var(--c)",
        "var(--a) var(--b) var(--c)",
      ),
      same("color", "var(--someVar)", "var(--someVar)"),
    ],
    &default_options(),
  );
}

/// Separating references that the author wrote flush against each other is a
/// divergence: upstream leaves the value exactly as written.
#[test]
fn diverges_on_inserting_spaces_between_flush_references() {
  check(
    &[diverges(
      "color",
      "var(--a)var(--b)var(--c)",
      "var(--a) var(--b) var(--c)",
      "var(--a)var(--b)var(--c)",
    )],
    &default_options(),
  );
}

// ── Colour and math functions ────────────────────────────────────────

/// Values routed around the CSS parser by the colour-function allowlist keep
/// their leading zeros where upstream strips them.
#[test]
fn diverges_on_leading_zeros_inside_colour_functions() {
  check(
    &[
      diverges(
        "color",
        "oklch(from var(--xs74gcj) l c h / 0.5)",
        "oklch(from var(--xs74gcj) l c h / 0.5)",
        "oklch(from var(--xs74gcj) l c h / .5)",
      ),
      diverges(
        "color",
        "oklab(40.101% 0.1147 0.0453)",
        "oklab(40.101% 0.1147 0.0453)",
        "oklab(40.101% .1147 .0453)",
      ),
      diverges(
        "color",
        "oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9))",
        "oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9))",
        "oklab(from #0000FF calc(l + .1) a b / calc(alpha * .9))",
      ),
      diverges(
        "color",
        "oklab(from hsl(180 100% 50%) calc(l - 0.1) a b)",
        "oklab(from hsl(180 100% 50%) calc(l - 0.1) a b)",
        "oklab(from hsl(180 100% 50%) calc(l - .1) a b)",
      ),
      diverges(
        "color",
        "oklab(from green l a b / 0.5)",
        "oklab(from green l a b / 0.5)",
        "oklab(from green l a b / .5)",
      ),
    ],
    &default_options(),
  );
}

/// The same allowlist leaves the spaces after commas that upstream removes.
#[test]
fn diverges_on_spacing_inside_allowlisted_functions() {
  check(
    &[
      diverges(
        "color",
        "clamp(200px,  40%,     400px)",
        "clamp(200px, 40%, 400px)",
        "clamp(200px,40%,400px)",
      ),
      diverges(
        "color",
        "clamp(min(10vw,      20rem),     300px,     max(90vw,     55rem))",
        "clamp(min(10vw, 20rem), 300px, max(90vw, 55rem))",
        "clamp(min(10vw,20rem),300px,max(90vw,55rem))",
      ),
      diverges(
        "color",
        "clamp(0, (var(--l-threshold, 0.623)   /  l - 1)   *    infinity,    1)",
        "clamp(0, (var(--l-threshold, 0.623) / l - 1) * infinity, 1)",
        "clamp(0,(var(--l-threshold,.623) / l - 1) * infinity,1)",
      ),
    ],
    &default_options(),
  );
}

/// An already-tight allowlisted value is left alone by both compilers.
#[test]
fn keeps_an_already_tight_allowlisted_value() {
  check(
    &[same(
      "color",
      "clamp(min(10vw,20rem),300px,max(90vw,55rem))",
      "clamp(min(10vw,20rem),300px,max(90vw,55rem))",
    )],
    &default_options(),
  );
}

// ── Font-size conversion ─────────────────────────────────────────────

#[test]
fn converts_font_size_pixels_to_rem_when_enabled() {
  check(
    &[
      same("fontSize", "16px", "1rem"),
      same("fontSize", "32px", "2rem"),
      same("fontSize", "0px", "0"),
    ],
    &rem_enabled_options(),
  );
}

/// The conversion applies only to `fontSize`, and only to pixel lengths.
#[test]
fn converts_nothing_else_when_font_size_conversion_is_enabled() {
  check(
    &[
      same("fontSize", "2em", "2em"),
      same("fontSize", "1rem", "1rem"),
      same("width", "16px", "16px"),
    ],
    &rem_enabled_options(),
  );
}

/// A converted value below one rem loses its leading zero here and keeps it
/// upstream, so the option's own output diverges.
#[test]
fn diverges_on_a_sub_rem_font_size_conversion() {
  check(
    &[diverges("fontSize", "8px", ".5rem", "0.5rem")],
    &rem_enabled_options(),
  );
}

#[test]
fn leaves_font_size_alone_when_the_option_is_off() {
  check(&[same("fontSize", "16px", "16px")], &default_options());
}

// ── Vendor prefixes ──────────────────────────────────────────────────

/// A vendor prefix is an ordinary part of a keyword or function name and is
/// carried through untouched — including when camel-case conversion has to
/// produce one.
#[test]
fn carries_vendor_prefixes_through_unchanged() {
  check(
    &[
      same("display", "-webkit-box", "-webkit-box"),
      same("transitionProperty", "WebkitTransform", "-webkit-transform"),
      same(
        "transitionProperty",
        "-webkit-transform",
        "-webkit-transform",
      ),
    ],
    &default_options(),
  );
}

/// Hex shortening applies inside a vendor-prefixed function too, and upstream
/// does not shorten at all.
#[test]
fn diverges_on_hex_shortening_inside_a_vendor_prefixed_function() {
  check(
    &[diverges(
      "backgroundImage",
      "-webkit-linear-gradient(top, #FFFFFF, #000000)",
      "-webkit-linear-gradient(top,#FFF,#000)",
      "-webkit-linear-gradient(top,#FFFFFF,#000000)",
    )],
    &default_options(),
  );
}

// ── Unicode and escapes ──────────────────────────────────────────────

/// An escape outside a string keeps its source spelling.
#[test]
fn preserves_an_escape_outside_a_string() {
  check(
    &[same(
      "fontFamily",
      r"My\ Font, sans-serif",
      r"My\ Font,sans-serif",
    )],
    &default_options(),
  );
}

/// Inside a string, this compiler resolves an escape to the character it names
/// while upstream keeps the author's spelling. Different bytes reach the hash,
/// so the class name differs too.
#[test]
fn diverges_on_resolving_escapes_inside_a_string() {
  check(
    &[
      diverges(
        "fontFamily",
        r#""\2014 A", sans-serif"#,
        "\"—A\",sans-serif",
        r#""\2014 A",sans-serif"#,
      ),
      diverges(
        "fontFamily",
        r#""\1F600", sans-serif"#,
        "\"😀\",sans-serif",
        r#""\1F600",sans-serif"#,
      ),
    ],
    &default_options(),
  );
}

/// Non-ASCII text that needs no escape survives byte for byte, inside a string
/// and as a bare identifier.
#[test]
fn preserves_non_ascii_content() {
  check(
    &[
      same(
        "fontFamily",
        "日本語フォント, sans-serif",
        "日本語フォント,sans-serif",
      ),
      same(
        "fontFamily",
        "\"→ Привет 日本語 🙂\", sans-serif",
        "\"→ Привет 日本語 🙂\",sans-serif",
      ),
    ],
    &default_options(),
  );
}

// ── Malformed input ──────────────────────────────────────────────────

/// A value SWC cannot parse is emitted verbatim as long as it cannot break out
/// of the rule being generated — which is why an unclosed bracket and a
/// nonsense operator sequence are both accepted rather than rejected.
#[test]
fn passes_inert_unparsable_values_through() {
  check(
    &[
      same(
        "gridTemplateColumns",
        "[full-start 1fr [content-start",
        "[full-start 1fr [content-start",
      ),
      same("width", "10px ++ 20px", "10px ++ 20px"),
    ],
    &default_options(),
  );
}

/// An opening brace could open a block inside the generated stylesheet, so it
/// is rejected here. The harness verdict is `acceptance divergent`: the
/// reference compiler emits `color:red {`. This is a local guard rather than a
/// spelling difference, so it is asserted on the message rather than through
/// the case table — a bare "it panicked" would pass on any panic at all,
/// including one from a future bug elsewhere in the pipeline.
#[test]
fn rejects_a_value_carrying_an_opening_brace() {
  let options = default_options();

  let result = catch_unwind(AssertUnwindSafe(|| {
    normalize_css_property_value("color", "red {", &options)
  }));

  let message = panic_message(result);

  assert!(
    message.contains("* { stylexValue: red { }"),
    "expected the rejection to quote the generated rule, got: {message}"
  );
}

// ── Robustness ───────────────────────────────────────────────────────

/// Builds a value nesting `calc()` `depth` levels deep.
fn nested_calc(depth: usize) -> String {
  let mut value = String::from("1px");
  for _ in 0..depth {
    value = format!("calc({value} + 1px)");
  }
  value
}

/// Nesting far past anything a person writes by hand must not lose a level.
/// Generated, so the parity harness carries no verdict for it; the checked-in
/// `edge-deeply-nested-calc` case covers the depth a human would write.
#[test]
fn survives_function_nesting_up_to_the_limit() {
  let depth = MAX_VALUE_NESTING_DEPTH;

  let result = normalize_css_property_value("width", &nested_calc(depth), &default_options());

  assert_eq!(result.matches("calc(").count(), depth);
  assert!(result.starts_with("calc("));
}

/// One level past the limit is rejected with a diagnostic naming both depths.
///
/// The limit exists because every stage of the pipeline recurses once per
/// nesting level: without it a deep enough value exhausts the stack, and a
/// stack overflow aborts the process rather than panicking, so nothing catches
/// it and no diagnostic is produced. Rejecting early is what keeps the failure
/// reportable.
#[test]
fn rejects_function_nesting_past_the_limit() {
  let options = default_options();
  let value = nested_calc(MAX_VALUE_NESTING_DEPTH + 1);

  let result = catch_unwind(AssertUnwindSafe(|| {
    normalize_css_property_value("width", &value, &options)
  }));

  let message = panic_message(result);

  assert!(
    message.contains("nested more deeply") && message.contains("limit 64, found 65"),
    "expected the rejection to state both depths, got: {message}"
  );

  // The guard reads the value rather than parsing it, so it holds at any depth
  // — including ones that used to take the process down with them.
  let far_past = nested_calc(5_000);
  let result = catch_unwind(AssertUnwindSafe(|| {
    normalize_css_property_value("width", &far_past, &options)
  }));

  assert!(
    panic_message(result).contains("limit 64, found 5000"),
    "expected the guard to hold without recursing"
  );
}

/// The limit is a property of the compiler, not of which branch a value takes:
/// the colour-function path bypasses SWC's codegen, and is guarded the same.
#[test]
fn rejects_deep_nesting_on_the_colour_function_path() {
  let options = default_options();
  let mut value = String::from("red");
  for _ in 0..=MAX_VALUE_NESTING_DEPTH {
    value = format!("oklch(from {value} l c h)");
  }

  let result = catch_unwind(AssertUnwindSafe(|| {
    normalize_css_property_value("color", &value, &options)
  }));

  assert!(
    panic_message(result).contains("nested more deeply"),
    "expected the colour-function path to reject deep nesting too"
  );
}

/// A long comma-separated list is normalized entry by entry, with no truncation
/// at any list length.
#[test]
fn normalizes_every_entry_of_a_long_list() {
  let entries = 500;
  let value = (1..=entries)
    .map(|n| format!("0px 0px {n}px #000"))
    .collect::<Vec<_>>()
    .join(", ");

  let result = normalize_css_property_value("boxShadow", &value, &default_options());

  assert_eq!(result.matches('#').count(), entries);
  assert!(result.starts_with("0 0 1px #000,"));
}
