//! Value-normalization coverage asserted at the public entry point.
//!
//! Every expectation here is a spelling the parity harness in
//! `crates/stylex-rs-compiler/parity` confirms `@stylexjs/babel-plugin`
//! produces — never judgement. Regenerate with the two runs below; the second
//! is what the cases under
//! [`rem_enabled_options`](super::support::rem_enabled_options) come from,
//! since the harness defaults the font-size option off and a default run cannot
//! verdict them at all:
//!
//! ```sh
//! pnpm run --filter=@stylexswc/rs-compiler build
//! pnpm run --filter=@stylexswc/rs-compiler parity -- --json parity/results/default.json
//! pnpm run --filter=@stylexswc/rs-compiler parity -- --font-size-px-to-rem \
//!   --json parity/results/font-size-px-to-rem.json
//! ```
//!
//! The `Case` machinery and the shared `check` runner live in `support`,
//! alongside the same pieces used by `spacing_repair_parity_test`.
//!
//! Read `entries[].babel.declarations` for the reference spelling and
//! `entries[].rust.declarations` for this compiler's. Overlap with the older
//! `normalize_css_property_value_tests` module in `common_test.rs` is
//! deliberate: those assertions predate the harness, so they say what this
//! compiler does without saying whether anything else agrees.

use std::panic::{AssertUnwindSafe, catch_unwind};

use super::support::{check, default_options, panic_message, rem_enabled_options, same, unchanged};
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
      unchanged("transitionDuration", "2s"),
    ],
    &default_options(),
  );
}

/// Under ten milliseconds the conversion is skipped.
#[test]
fn keeps_milliseconds_below_the_conversion_threshold() {
  check(
    &[unchanged("transitionDuration", "5ms")],
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
      unchanged("width", "0%"),
      unchanged("gridTemplateColumns", "0fr"),
      unchanged("gridTemplateColumns", "0fr 1fr"),
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
      unchanged("margin", "10px"),
      unchanged("width", "16px"),
      unchanged("image-orientation", "90deg"),
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
      unchanged("color", "calc(0 - var(--someVar))"),
      unchanged("color", "calc(0px - var(--someVar) + 10px)"),
      unchanged("width", "calc(0px + 10px)"),
    ],
    &default_options(),
  );
}

/// A zero angle inside a function does get its unit rewritten. The window the
/// zero-dimension normalizer opens for function arguments is what decides this,
/// and it is the reference compiler's window, quirks included.
#[test]
fn rewrites_a_zero_angle_inside_a_transform_function() {
  check(
    &[same("transform", "rotate(0rad)", "rotate(0deg)")],
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
      unchanged("--myVar", "0px"),
      unchanged("--myVar", "backgroundColor"),
      unchanged("--my-var", "foo"),
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
      unchanged("grid-column-start", "-1"),
    ],
    &default_options(),
  );
}

// ── Quotes ───────────────────────────────────────────────────────────

/// The quote character the author chose is the quote character that reaches the
/// hash. No normalizer understands quote characters, so none of them can swap
/// one for the other.
#[test]
fn keeps_the_authored_quote_character() {
  check(
    &[
      unchanged("quotes", r#"'""'"#),
      unchanged("quotes", r#"'"123"'"#),
      unchanged("quotes", r#""""#),
      unchanged("quotes", r#""123""#),
    ],
    &default_options(),
  );
}

/// The same holds where a value is *made* of quoted strings.
#[test]
fn keeps_the_authored_quote_character_in_grid_template_areas() {
  check(
    &[
      unchanged("gridTemplateAreas", r#"'"content"'"#),
      unchanged("gridTemplateAreas", r#"'"content" "sidebar"'"#),
      unchanged("gridTemplateAreas", r#""content""#),
      unchanged("gridTemplateAreas", r#""content" "sidebar""#),
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
      unchanged("transitionProperty", "--myVar"),
      same("willChange", "marginTop", "margin-top"),
      same("willChange", "backgroundColor", "background-color"),
    ],
    &default_options(),
  );
}

#[test]
fn leaves_ordinary_keyword_values_alone() {
  check(&[unchanged("color", "red")], &default_options());
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
      unchanged("boxShadow", "1px 1px #000"),
    ],
    &default_options(),
  );
}

#[test]
fn normalizes_adjacent_custom_property_references() {
  check(
    &[
      unchanged("color", "var(--a) var(--b) var(--c)"),
      unchanged("color", "var(--someVar)"),
    ],
    &default_options(),
  );
}

/// References the author wrote flush against each other stay flush. Nothing
/// inserts a separator that the author did not write.
#[test]
fn keeps_flush_references_flush() {
  check(
    &[unchanged("color", "var(--a)var(--b)var(--c)")],
    &default_options(),
  );
}

// ── Colour and math functions ────────────────────────────────────────

/// A colour function takes the same path every other value takes, so its
/// arguments lose their leading zeros like any other number. There is no
/// list of function names anywhere that routes them around anything.
#[test]
fn strips_leading_zeros_inside_colour_functions() {
  check(
    &[
      same(
        "color",
        "oklch(from var(--xs74gcj) l c h / 0.5)",
        "oklch(from var(--xs74gcj) l c h / .5)",
      ),
      same(
        "color",
        "oklab(40.101% 0.1147 0.0453)",
        "oklab(40.101% .1147 .0453)",
      ),
      same(
        "color",
        "oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9))",
        "oklab(from #0000FF calc(l + .1) a b / calc(alpha * .9))",
      ),
      same(
        "color",
        "oklab(from hsl(180 100% 50%) calc(l - 0.1) a b)",
        "oklab(from hsl(180 100% 50%) calc(l - .1) a b)",
      ),
      same(
        "color",
        "oklab(from green l a b / 0.5)",
        "oklab(from green l a b / .5)",
      ),
    ],
    &default_options(),
  );
}

/// The same goes for spacing: a space after a comma is removed inside a colour
/// or math function exactly as it is anywhere else.
#[test]
fn removes_spacing_after_commas_inside_functions() {
  check(
    &[
      same(
        "color",
        "clamp(200px,  40%,     400px)",
        "clamp(200px,40%,400px)",
      ),
      same(
        "color",
        "clamp(min(10vw,      20rem),     300px,     max(90vw,     55rem))",
        "clamp(min(10vw,20rem),300px,max(90vw,55rem))",
      ),
      same(
        "color",
        "clamp(0, (var(--l-threshold, 0.623)   /  l - 1)   *    infinity,    1)",
        "clamp(0,(var(--l-threshold,.623) / l - 1) * infinity,1)",
      ),
    ],
    &default_options(),
  );
}

/// A value already spelled the way the normalizers would spell it is left
/// alone, which is what makes a second pass over it a no-op.
#[test]
fn keeps_an_already_tight_function_value() {
  check(
    &[unchanged(
      "color",
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
      unchanged("fontSize", "2em"),
      unchanged("fontSize", "1rem"),
      unchanged("width", "16px"),
    ],
    &rem_enabled_options(),
  );
}

/// A converted value below one rem keeps its leading zero: the conversion runs
/// last, after the leading-zero normalizer has already had its turn, so the
/// number it produces is never revisited.
#[test]
fn keeps_the_leading_zero_on_a_sub_rem_font_size() {
  check(&[same("fontSize", "8px", "0.5rem")], &rem_enabled_options());
}

#[test]
fn leaves_font_size_alone_when_the_option_is_off() {
  check(&[unchanged("fontSize", "16px")], &default_options());
}

// ── Vendor prefixes ──────────────────────────────────────────────────

/// A vendor prefix is an ordinary part of a keyword or function name and is
/// carried through untouched — including when camel-case conversion has to
/// produce one.
#[test]
fn carries_vendor_prefixes_through_unchanged() {
  check(
    &[
      unchanged("display", "-webkit-box"),
      same("transitionProperty", "WebkitTransform", "-webkit-transform"),
      unchanged("transitionProperty", "-webkit-transform"),
    ],
    &default_options(),
  );
}

/// A hex colour inside a vendor-prefixed function keeps all six digits and its
/// letter case, like any other hex colour.
#[test]
fn keeps_a_hex_colour_inside_a_vendor_prefixed_function() {
  check(
    &[same(
      "backgroundImage",
      "-webkit-linear-gradient(top, #FFFFFF, #000000)",
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

/// An escape inside a string keeps the author's spelling rather than being
/// resolved to the character it names. It used to be resolved, which put
/// different bytes into the hash than the reference compiler used.
#[test]
fn keeps_an_escape_inside_a_string_unresolved() {
  check(
    &[
      same(
        "fontFamily",
        r#""\2014 A", sans-serif"#,
        r#""\2014 A",sans-serif"#,
      ),
      same(
        "fontFamily",
        r#""\1F600", sans-serif"#,
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
      unchanged("gridTemplateColumns", "[full-start 1fr [content-start"),
      unchanged("width", "10px ++ 20px"),
    ],
    &default_options(),
  );
}

/// A semicolon at the end of a value opens nothing, so it survives into the
/// declaration exactly as written — however many of them there are, and
/// whatever whitespace trails them. The reference compiler emits the same
/// bytes, so the harness verdict for all three is `identical`.
///
/// Stray trailing semicolons are common in hand-written style objects — the
/// project's own large fixture carries more than twenty — and none of them can
/// splice a second declaration into the stylesheet.
#[test]
fn keeps_a_trailing_semicolon() {
  check(
    &[
      unchanged("backgroundColor", "var(--web-wash);"),
      same("color", "red ; ", "red ;"),
      unchanged("color", "red;;"),
    ],
    &default_options(),
  );
}

/// A semicolon with a declaration behind it is the shape the guard exists for:
/// emitted verbatim, `color: 'red; margin: 10px'` would close its own
/// declaration and add one the author never asked this rule for.
///
/// The harness verdict is `acceptance divergent` — the reference compiler emits
/// both declarations. Before the pipeline swap this compiler emitted only the
/// first and dropped the rest with no diagnostic, which is the behaviour this
/// replaces.
#[test]
fn rejects_a_semicolon_that_starts_a_second_declaration() {
  let options = default_options();

  for value in ["red; margin: 10px", "red;background:blue", "red; /* x */"] {
    let result = catch_unwind(AssertUnwindSafe(|| {
      normalize_css_property_value("color", value, &options)
    }));

    let message = panic_message(result);

    assert!(
      message.contains("outside of a string or comment"),
      "expected `color: {value}` to be rejected, got: {message}"
    );
  }
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
    message.contains("* { color: red { }"),
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
