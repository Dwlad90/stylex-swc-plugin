//! Spacing-repair coverage asserted at the public entry point, together with
//! the verdict `@stylexjs/babel-plugin` returns for each case.
//!
//! The whitespace repair pass and the helpers around it are where years of
//! individually reported defects accumulated: a function result immediately
//! followed by a unit, `url()` bodies containing characters that look like CSS
//! syntax, comments inside values, adjacent quoted strings, non-ASCII content,
//! a percentage followed by a number, and the leading zero on a negative
//! decimal. Each assertion below exists because somebody hit the bug. They were
//! written against the repair pass itself, which is scheduled for deletion, so
//! they are re-expressed here one level up — at the entry point that survives
//! the rewrite.
//!
//! Every expectation is what this compiler produces **today**, so the suite is
//! green before the normalization pipeline is replaced and stays a net under
//! that change. That matters more here than elsewhere: the repair pass runs on
//! the *output* of SWC's minifying codegen, so an input that once addressed the
//! pass directly now has to survive a parse and a re-serialization first. Where
//! the two disagree, the seam expectation is the one that describes the
//! compiler, and the difference is called out in the case's doc comment.
//!
//! **What the verdicts say, read as a whole.** The reference compiler does not
//! repair spacing, because it never damages it: it keeps the value the author
//! wrote. So almost every space this pass inserts is a divergence, and the
//! cases that agree are the ones where it inserts nothing. The exception is the
//! `/` operator, which the reference compiler also spaces — and spaces
//! differently at the start of a value, where it emits a leading space this
//! compiler does not. Read that way, the module is a list of the spellings the
//! pipeline replacement is expected to move.
//!
//! Alongside each expectation sits a [`Reference`](super::support::Reference)
//! verdict taken from the parity harness in `crates/stylex-rs-compiler/parity`
//! — never from judgement. Regenerate the verdicts with:
//!
//! ```sh
//! pnpm run --filter=@stylexswc/rs-compiler build
//! pnpm run --filter=@stylexswc/rs-compiler parity -- --json parity/results/default.json
//! ```
//!
//! Read `entries[].babel.declarations` for the reference spelling and
//! `entries[].rust.declarations` for this compiler's. The `Case`/`Reference`
//! machinery and the shared `check` runner live in `support`. Most cases here
//! use [`unchanged`](super::support::unchanged), which is the case whose
//! expectation is its own input — the majority shape, since most of what this
//! module asserts is that a value is not rewritten.
//!
//! Two shapes of case are asserted outside the case table, because a `Case`
//! compares two spellings and these have only one: a rejection, and a
//! declaration the reference compiler does not emit at all.

use super::support::{check, default_options, diverges, rejects, same, unchanged};
use crate::css::common::normalize_css_property_value;

/// The diagnostic every rejection below is asserted on.
///
/// Kept out of the calls that use it: the parity harness harvests CSS values
/// out of literal tables in these sources, and a second string literal sitting
/// next to a property name is exactly the shape it reads as one.
const UNCLOSED_FUNCTION: &str = "unclosed function";

// ── A unit after a function result (issue #927) ──────────────────────

/// The oldest defect in this pass: a dimension whose number came from a
/// function reads as `)` immediately followed by a unit, and inserting a space
/// there would turn `var(--x)px` into two tokens and lose the dimension.
///
/// Every unit the compiler knows is listed rather than looped, so each one
/// carries its own harness verdict. All of them agree with the reference
/// compiler, which is the whole point: this is the case where the repair
/// correctly does nothing.
#[test]
fn keeps_a_unit_glued_to_a_function_result() {
  check(
    &[
      // Absolute lengths
      unchanged("width", "var(--x)px"),
      unchanged("width", "var(--x)cm"),
      unchanged("width", "var(--x)mm"),
      unchanged("width", "var(--x)in"),
      unchanged("width", "var(--x)pt"),
      unchanged("width", "var(--x)pc"),
      unchanged("width", "var(--x)Q"),
      // Font-relative lengths
      unchanged("width", "var(--x)em"),
      unchanged("gap", "var(--gap)rem"),
      unchanged("width", "var(--x)ex"),
      unchanged("width", "var(--x)ch"),
      unchanged("width", "var(--x)lh"),
      unchanged("width", "var(--x)rlh"),
      unchanged("width", "var(--x)cap"),
      unchanged("width", "var(--x)ic"),
      // Viewport-relative lengths
      unchanged("width", "var(--x)vw"),
      unchanged("height", "var(--h)vh"),
      unchanged("width", "var(--x)vi"),
      unchanged("width", "var(--x)vb"),
      unchanged("width", "var(--x)vmin"),
      unchanged("width", "var(--x)vmax"),
      unchanged("width", "var(--x)dvw"),
      unchanged("width", "var(--x)dvh"),
      unchanged("width", "var(--x)lvw"),
      unchanged("width", "var(--x)lvh"),
      unchanged("width", "var(--x)svw"),
      unchanged("width", "var(--x)svh"),
      // Container-relative lengths
      unchanged("width", "var(--x)cqw"),
      unchanged("width", "var(--x)cqh"),
      unchanged("width", "var(--x)cqi"),
      unchanged("width", "var(--x)cqb"),
      unchanged("width", "var(--x)cqmin"),
      unchanged("width", "var(--x)cqmax"),
      // Time
      unchanged("transitionDuration", "var(--d)ms"),
      unchanged("transitionDuration", "var(--x)s"),
      // Angles
      unchanged("transform", "rotate(var(--a)deg)"),
      unchanged("transform", "rotate(var(--x)rad)"),
      unchanged("transform", "rotate(var(--x)grad)"),
      unchanged("transform", "rotate(var(--x)turn)"),
      // Resolution, flex, frequency
      unchanged("width", "var(--x)dpi"),
      unchanged("width", "var(--x)dpcm"),
      unchanged("width", "var(--x)dppx"),
      unchanged("gridTemplateColumns", "var(--x)fr"),
      unchanged("width", "var(--x)Hz"),
      unchanged("width", "var(--x)kHz"),
      // The rule is about function results, not about `var()`: any `)` can be
      // followed by a unit.
      unchanged("width", "calc(1+2)em"),
    ],
    &default_options(),
  );
}

/// `%` is not in the unit list — it is its own token — but nothing separates it
/// from the `)` before it either, so a percentage built from a function survives
/// the same way a dimension does.
#[test]
fn keeps_a_percent_glued_to_a_function_result() {
  check(&[unchanged("width", "var(--x)%")], &default_options());
}

/// Unit matching is case-sensitive, because CSS spells two of its units with
/// capitals and the rest without. `PX` and `EM` are not units and get
/// separated like any other word; the capitalised units that *are* — `Q`, `Hz`
/// and `kHz` — are asserted glued by
/// [`keeps_a_unit_glued_to_a_function_result`] above, which is where the whole
/// unit list lives. The two halves together are what pin case-sensitivity:
/// lowercasing before the match would keep `PX` glued and fail here, while
/// rejecting every capital would separate `Q` and fail there.
///
/// Separating these is a divergence — the reference compiler leaves all three
/// exactly as written — but the case-sensitivity is not.
#[test]
fn matches_units_case_sensitively() {
  check(
    &[
      diverges("width", "var(--x)PX", "var(--x) PX", "var(--x)PX"),
      diverges("width", "var(--x)Px", "var(--x) Px", "var(--x)Px"),
      diverges("width", "var(--x)EM", "var(--x) EM", "var(--x)EM"),
    ],
    &default_options(),
  );
}

/// A word that is not a unit is read as a separate value component, so the
/// space the minifier dropped is put back. Function names, keywords and
/// ordinary identifiers all take this path.
///
/// Every one of them diverges: the reference compiler never lost the space in
/// the first place, so it never has to guess whether one belongs here.
#[test]
fn separates_a_non_unit_word_from_a_function_result() {
  check(
    &[
      diverges("width", "var(--x)auto", "var(--x) auto", "var(--x)auto"),
      diverges("width", "var(--x)calc", "var(--x) calc", "var(--x)calc"),
      diverges("width", "var(--x)rgb", "var(--x) rgb", "var(--x)rgb"),
      diverges(
        "width",
        "var(--x)translate3d",
        "var(--x) translate3d",
        "var(--x)translate3d",
      ),
      diverges("width", "var(--x)solid", "var(--x) solid", "var(--x)solid"),
      diverges("width", "var(--x)abc", "var(--x) abc", "var(--x)abc"),
      diverges("width", "var(--x)hello", "var(--x) hello", "var(--x)hello"),
      diverges("width", "var(--x)div", "var(--x) div", "var(--x)div"),
      diverges("width", "var(--x)span", "var(--x) span", "var(--x)span"),
      diverges("width", "var(--x)ABC", "var(--x) ABC", "var(--x)ABC"),
      // Two function names, which look most like units of anything here and
      // are the two the unit list was originally pinned against.
      diverges("width", "var(--x)var", "var(--x) var", "var(--x)var"),
      diverges(
        "width",
        "var(--x)rotate",
        "var(--x) rotate",
        "var(--x)rotate",
      ),
      // A single letter is a word too, and neither `p` nor `a` is a unit.
      diverges("width", "var(--x)p", "var(--x) p", "var(--x)p"),
      diverges("width", "var(--x)a", "var(--x) a", "var(--x)a"),
    ],
    &default_options(),
  );
}

/// A non-ASCII word cannot be a unit, so it is separated without consulting the
/// unit list at all — which is also what keeps the lookahead from slicing a
/// multi-byte character in half.
#[test]
fn separates_a_non_ascii_word_from_a_function_result() {
  check(
    &[diverges(
      "width",
      "var(--x)日本語",
      "var(--x) 日本語",
      "var(--x)日本語",
    )],
    &default_options(),
  );
}

// ── Adjacent function calls ──────────────────────────────────────────

/// Two function calls written flush against each other are two value
/// components. The minifier drops the space between them and the repair puts it
/// back; the reference compiler still has the author's.
#[test]
fn separates_adjacent_function_calls() {
  check(
    &[
      diverges(
        "transform",
        "rotate(10deg)translate3d(0,0,0)",
        "rotate(10deg) translate3d(0,0,0)",
        "rotate(10deg)translate3d(0,0,0)",
      ),
      diverges(
        "width",
        "calc(1px)calc(2px)calc(3px)",
        "calc(1px) calc(2px) calc(3px)",
        "calc(1px)calc(2px)calc(3px)",
      ),
      diverges(
        "color",
        "rgb(0,0,0)rgba(255,255,255,.5)",
        "rgb(0,0,0) rgba(255,255,255,.5)",
        "rgb(0,0,0)rgba(255,255,255,.5)",
      ),
      // Flush `var()` references, the shape the unit rule above has to be
      // careful not to break. Also covered in the sibling module, from the
      // other direction — there as one of the reported divergences, here as
      // the spacing rule that produces it.
      diverges(
        "color",
        "var(--a)var(--b)var(--c)",
        "var(--a) var(--b) var(--c)",
        "var(--a)var(--b)var(--c)",
      ),
    ],
    &default_options(),
  );
}

// ── What follows a closing paren ─────────────────────────────────────

/// Everything a `)` can be followed by, one character class at a time. These
/// inputs are degenerate on purpose — the pass is a string scan, and a bare `)`
/// is how each rule was originally pinned.
///
/// The `/` pair is the one place in the module where both compilers insert the
/// same space, so it is the only inserted space here that is not a divergence.
#[test]
fn separates_a_closing_paren_from_what_follows() {
  check(
    &[
      // digit
      diverges("width", ")3", ") 3", ")3"),
      diverges("width", "calc(a)42", "calc(a) 42", "calc(a)42"),
      // hex colour
      diverges("color", ")#fff", ") #fff", ")#fff"),
      // multiplication operator
      diverges("width", ")*3", ") * 3", ")*3"),
      // an uppercase word that is not a unit
      diverges("color", ")A", ") A", ")A"),
      // division operator: both compilers space it
      same("width", ")/7", ") / 7"),
      // an uppercase word that is a unit
      unchanged("width", ")Q"),
      // A `-` is a sign or a subtraction operator, never a separator, so this
      // is the one pair that is deliberately left alone.
      unchanged("width", ")-1"),
      // Nothing to separate it from.
      unchanged("width", ")"),
    ],
    &default_options(),
  );
}

/// A `)` immediately followed by `(` is two adjacent groups, and the repair
/// separates them — but only when the value gets that far. `)(` opens a
/// function that is never closed, and the structural guard ahead of the parser
/// rejects it before any spacing runs.
///
/// The harness verdict is `both reject`: the reference compiler refuses these
/// too, so the rejection is parity even though the messages are not compared.
/// Asserted on the diagnostic rather than through the case table, because a
/// rejection has no spelling to compare and a bare "it panicked" would pass on
/// any panic at all.
#[test]
fn rejects_an_unclosed_group_after_a_closing_paren() {
  rejects(
    "color",
    &[")(", ")a )Z )0 )# )("],
    UNCLOSED_FUNCTION,
    &default_options(),
  );
}

/// Balanced degenerate parens do reach the repair, and every rule fires — but
/// the pass only ever *inserts* spaces, so a space the minifier already
/// swallowed before a `)` stays swallowed. `) / 1) * .5` is that: the space
/// before the second `)` is gone and no rule puts it back. The reference
/// compiler keeps it, and adds no space of its own after it.
///
/// Recorded rather than fixed. It is the same lost-whitespace family as the
/// reported divergences, and the pipeline replacement is what closes it.
#[test]
fn does_not_restore_a_space_swallowed_before_a_closing_paren() {
  check(
    &[diverges("width", ")/1 )*.5", ") / 1) * .5", ") / 1 )*.5")],
    &default_options(),
  );
}

// ── A hex colour after a value token ─────────────────────────────────

/// `#` starts a hex colour, and the minifier will happily park it against
/// whatever came before. Every token class that can precede it is covered: a
/// dimension, a keyword, a percentage, a bare digit, a capital, and a non-ASCII
/// word.
#[test]
fn separates_a_hex_colour_from_the_token_before_it() {
  check(
    &[
      diverges("boxShadow", "1px#000", "1px #000", "1px#000"),
      diverges("color", "solid#abc", "solid #abc", "solid#abc"),
      diverges("backgroundPosition", "50%#fff", "50% #fff", "50%#fff"),
      diverges("color", "A#fff", "A #fff", "A#fff"),
      diverges("color", "9#fff", "9 #fff", "9#fff"),
      diverges("color", "%#fff", "% #fff", "%#fff"),
      diverges("color", "日本語#fff", "日本語 #fff", "日本語#fff"),
      diverges(
        "color",
        "a#fff Z#fff 1#fff %#fff",
        "a #fff Z #fff 1 #fff % #fff",
        "a#fff Z#fff 1#fff %#fff",
      ),
      // A `#` with nothing before it has nothing to be separated from.
      unchanged("color", "#"),
    ],
    &default_options(),
  );
}

// ── A number after a percentage ──────────────────────────────────────

/// `40%.1147` is two components, not a percentage followed by a fraction of
/// one. The repair separates a percentage from a digit or a `.` that follows
/// it.
#[test]
fn separates_a_number_from_a_percentage() {
  check(
    &[
      diverges("color", "40%.1147", "40% .1147", "40%.1147"),
      diverges("backgroundPosition", "50%10", "50% 10", "50%10"),
      diverges("backgroundPosition", "100%.5", "100% .5", "100%.5"),
      diverges(
        "backgroundPosition",
        "50%10 40%.5",
        "50% 10 40% .5",
        "50%10 40%.5",
      ),
    ],
    &default_options(),
  );
}

/// The same shape inside a colour function takes the allowlist path instead,
/// which emits the author's value verbatim and so never inserts the space —
/// which is what the reference compiler does everywhere. This is the one
/// percentage case the two agree on, and it agrees by not being repaired.
#[test]
fn leaves_a_number_after_a_percentage_alone_inside_a_colour_function() {
  check(
    &[unchanged("color", "oklab(40.101%.1147 .0453)")],
    &default_options(),
  );
}

// ── The `/` and `*` operators ────────────────────────────────────────

/// `/` is both a division operator and the CSS slash separator, and the
/// minifier removes the spaces around it in either role. The repair puts a
/// space on both sides — and so does the reference compiler, which is why these
/// mostly agree.
///
/// They part company at the start of a value: the reference compiler emits a
/// leading space before a `/` that opens one (` / 7`), and this compiler does
/// not. Same rule, different edge.
#[test]
fn spaces_the_slash_operator() {
  check(
    &[
      same("width", "size/2", "size / 2"),
      diverges("width", "/ 7", "/ 7", " / 7"),
      diverges("width", "/.5", "/ .5", " / .5"),
      diverges("width", "/-1", "/ -1", " / -1"),
      diverges("width", "/-2", "/ -2", " / -2"),
      // A function SWC's grammar does not know still gets its slash spaced,
      // because the unknown-syntax fallback runs the same repair.
      same(
        "width",
        "calc-size(fit-content,size/2)",
        "calc-size(fit-content,size / 2)",
      ),
    ],
    &default_options(),
  );
}

/// `*` is a multiplication operator, so an operand parked against it is
/// separated: a digit, a `.`, a `(`, or a sign. Unlike `/`, the reference
/// compiler leaves it as written.
#[test]
fn spaces_the_multiplication_operator() {
  check(
    &[
      unchanged("width", "* 3"),
      diverges("width", "*(100%)", "* (100%)", "*(100%)"),
    ],
    &default_options(),
  );
}

/// A `*` followed by an unclosed `(` never reaches the repair: the structural
/// guard rejects the unclosed function first. Harness verdict `both reject`.
#[test]
fn rejects_an_unclosed_operand_after_a_multiplication_operator() {
  rejects(
    "width",
    &["*(", "/.5 /-1 *( *3"],
    UNCLOSED_FUNCTION,
    &default_options(),
  );
}

// ── Adjacent quoted strings ──────────────────────────────────────────

/// Two strings written flush against each other are two components, and `""`
/// must not be split into `" "` on the way — which is why an empty string, the
/// one input where the closing quote is also the opening one, is here.
#[test]
fn separates_adjacent_quoted_strings() {
  check(
    &[
      diverges(
        "gridTemplateAreas",
        r#""content""sidebar""#,
        r#""content" "sidebar""#,
        r#""content""sidebar""#,
      ),
      diverges("quotes", r#""a""b""#, r#""a" "b""#, r#""a""b""#),
      diverges("quotes", r#""""""#, r#""" """#, r#""""""#),
      // Three in a row, so a separator has to be inserted twice — the case
      // that catches a scan which only ever looks one quote back.
      diverges("quotes", r#""""""""#, r#""" "" """#, r#""""""""#),
      // A lone empty string keeps its quotes and gains nothing.
      unchanged("quotes", r#""""#),
    ],
    &default_options(),
  );
}

/// The same separation with single quotes, or with the two quote characters
/// mixed. Two divergences compound here: the inserted space, and the quote
/// character being rewritten to a double quote.
#[test]
fn diverges_twice_on_adjacent_single_quoted_strings() {
  check(
    &[
      diverges("quotes", r#"'a''b'"#, r#""a" "b""#, r#"'a''b'"#),
      diverges("quotes", r#""a"'b'"#, r#""a" "b""#, r#""a"'b'"#),
    ],
    &default_options(),
  );
}

/// A string already separated from its neighbours by a space is a different
/// path through the quote tracking — the flag that remembers a closing quote
/// has to be cleared by the character in between — and the space itself is
/// dropped, because only *missing* spaces are repaired, never surviving ones.
///
/// This is the reported whitespace loss in its plainest form: the space is not
/// displaced or collapsed, it is gone, and the class name changes with it.
#[test]
fn diverges_on_dropping_the_space_around_a_separated_string() {
  check(
    &[
      diverges("quotes", r#""a" x "b""#, r#""a"x"b""#, r#""a" x "b""#),
      diverges("fontFamily", r#""a" 1px"#, r#""a"1px"#, r#""a" 1px"#),
    ],
    &default_options(),
  );
}

// ── Inside a quoted string ───────────────────────────────────────────

/// String contents are value data, not CSS syntax. Every character the repair
/// reacts to outside a string is inert inside one, and both compilers agree
/// byte for byte.
#[test]
fn applies_no_spacing_rule_inside_a_string() {
  check(
    &[
      unchanged("fontFamily", r#""hello)world""#),
      unchanged("fontFamily", r##""a)b#c%d.5/1*2(""##),
      // A doubled backslash is an escaped backslash, not an escaped quote, so
      // the string closes where it looks like it closes.
      unchanged("fontFamily", r#""a\\b""#),
    ],
    &default_options(),
  );
}

/// The same contents in single quotes, where the rewritten quote character is
/// the only difference either compiler makes.
#[test]
fn diverges_on_the_quote_character_around_inert_contents() {
  check(
    &[diverges(
      "fontFamily",
      r##"'a)b#c%d.5/1*2('"##,
      r##""a)b#c%d.5/1*2(""##,
      r##"'a)b#c%d.5/1*2('"##,
    )],
    &default_options(),
  );
}

/// **A defect, recorded rather than fixed.** An escaped quote inside a string
/// does not survive the round trip through SWC's minifying codegen: a
/// double-quoted string comes back with its quotes *removed*
/// (`"a\"b#c"` → `a"b#c`), and a single-quoted one comes back with the escaped
/// quote silently dropped (`'a\'b#c'` → `"ab#c"`).
///
/// The unquoted form is not merely a different spelling — it is no longer a
/// string, so the value extraction that follows never recognises the generated
/// rule's closing brace as a terminator and the `}` ends up inside the
/// declaration. That escapes the rule being generated, which is what the
/// structural guard exists to prevent; the guard reads the author's value, and
/// this `}` is manufactured downstream of it.
///
/// Filed as issue 15 in this effort's tracker. Asserted here as it behaves
/// today, so the pipeline replacement has to move it deliberately.
#[test]
fn diverges_on_an_escaped_quote_inside_a_string() {
  check(
    &[
      diverges(
        "fontFamily",
        r##""a\"b#c""##,
        r##"a"b#c}"##,
        r##""a\"b#c""##,
      ),
      diverges("fontFamily", r##"'a\'b#c'"##, r#""ab#c""#, r##"'a\'b#c'"##),
    ],
    &default_options(),
  );
}

// ── `url()` bodies ───────────────────────────────────────────────────

/// A `url()` body is not CSS-tokenized. Slashes, colons, semicolons, `#`, `*`
/// and query strings inside one are URL characters, and the repair copies the
/// body through byte for byte rather than spacing it as if it were a value.
/// Every one of these agrees with the reference compiler, which is what
/// "verbatim" has to mean.
#[test]
fn copies_url_bodies_verbatim() {
  check(
    &[
      unchanged("backgroundImage", "url(image.png)"),
      unchanged("backgroundImage", "url(http://example.com/img.png)"),
      unchanged(
        "backgroundImage",
        "url(https://fonts.googleapis.com/css2?family=Roboto)",
      ),
      // A `/*` inside a quoted body is part of the URL, not a comment opener.
      unchanged("backgroundImage", r#"url("a/*b.png")"#),
      // A data URL carries both a `;` and a `/`, either of which would end the
      // value if the body were read as CSS. The payload is broken with Rust
      // line continuations rather than bound to a `const` or joined with
      // `concat!`: the harness harvests string literals, and either of those
      // would take the case out of the corpus and leave it with no verdict.
      // A continuation is still one literal to the scanner, which decodes it
      // the same way rustc does.
      unchanged("backgroundImage", "url(data:image/svg+xml;utf8,<svg/>)"),
      unchanged(
        "backgroundImage",
        "url(\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAMUlEQVQ4T2NkYG\
          AQYcAP3uCTZhw1gGGYhAGBZIA/nYDCgBDAm9BGDWAAJyRCgLaBCAAgXwixzAS0pgAAAABJRU5ErkJggg==\")",
      ),
      // One `url()` among other components, so the body has to be recognised
      // partway through the value rather than at offset zero.
      unchanged(
        "backgroundImage",
        "linear-gradient(red,blue),url(http://example.com/a.png)",
      ),
    ],
    &default_options(),
  );
}

/// An unquoted `(` inside a body nests, so the *matching* `)` ends the URL, not
/// the first one. Without that the body would terminate early and its remainder
/// would be spaced as if it were CSS.
#[test]
fn balances_nested_parens_inside_a_url_body() {
  check(
    &[
      unchanged("backgroundImage", "url(a(b).png)"),
      unchanged("backgroundImage", "url(a(b(c)).png) 10px"),
    ],
    &default_options(),
  );
}

/// The body is copied verbatim, but the value *after* it is still a value. The
/// fast path used to skip the rest of the declaration, leaving a later
/// component with the minifier's spacing — so these repairs are real, and being
/// repairs, they diverge.
#[test]
fn still_repairs_the_value_after_a_url() {
  check(
    &[
      diverges(
        "backgroundImage",
        "url(a(b).png)calc(1px)",
        "url(a(b).png) calc(1px)",
        "url(a(b).png)calc(1px)",
      ),
      diverges(
        "backgroundImage",
        "url(a.png)calc(1px)",
        "url(a.png) calc(1px)",
        "url(a.png)calc(1px)",
      ),
      diverges(
        "backgroundImage",
        r#"url("/icons/•#hash.svg")calc(1px)rgb(0,0,0)"#,
        r#"url("/icons/•#hash.svg") calc(1px) rgb(0,0,0)"#,
        r#"url("/icons/•#hash.svg")calc(1px)rgb(0,0,0)"#,
      ),
    ],
    &default_options(),
  );
}

/// A `)` inside a *quoted* body does not close the function. Reading it as one
/// re-entered the spacing rules mid-URL, which both injected a space into the
/// URL and swallowed the separator after it.
#[test]
fn does_not_close_a_url_on_a_quoted_paren() {
  check(
    &[unchanged("backgroundImage", r#"url("a)b.png") 10px"#)],
    &default_options(),
  );
}

/// The slash separator of a shorthand sits right after a `url()` body, so the
/// two rules meet: the body is copied through and the slash is still spaced.
#[test]
fn spaces_a_slash_separator_after_a_url() {
  check(
    &[
      same(
        "backgroundImage",
        "url(a.png) no-repeat center/cover",
        "url(a.png) no-repeat center / cover",
      ),
      same(
        "backgroundImage",
        r#"url("asset.png") no-repeat center/cover"#,
        r#"url("asset.png") no-repeat center / cover"#,
      ),
    ],
    &default_options(),
  );
}

/// The same body in single quotes, where the rewritten quote character
/// compounds with the repaired space.
#[test]
fn diverges_on_the_quote_character_inside_a_url_body() {
  check(
    &[diverges(
      "backgroundImage",
      r#"url('a)b.png')calc(1px)"#,
      r#"url("a)b.png") calc(1px)"#,
      r#"url('a)b.png')calc(1px)"#,
    )],
    &default_options(),
  );
}

/// The escaped-quote defect reaches `url()` bodies too: the codegen drops the
/// quotes, the body stops being a string, and the generated rule's closing
/// brace lands in the declaration. Same finding as
/// [`diverges_on_an_escaped_quote_inside_a_string`], recorded at the URL shape
/// because that is where it was originally pinned.
#[test]
fn diverges_on_an_escaped_quote_inside_a_url_body() {
  check(
    &[diverges(
      "backgroundImage",
      r#"url("a\")b.png") 10px"#,
      r#"url(a")b.png)10px}"#,
      r#"url("a\")b.png") 10px"#,
    )],
    &default_options(),
  );
}

/// An unquoted `'` or `/*` inside a body is an ordinary URL character to the
/// repair, but the structural guard ahead of it reads the value as text and
/// does not know `url()` from any other function: the `'` opens a string that
/// never closes and the `/*` opens a comment that never closes, so both values
/// are rejected before the repair is reached.
///
/// The reference compiler accepts both, so the harness verdict is
/// `acceptance divergent`. Asserted on the diagnostic rather than through the
/// case table, since a rejection has no spelling to compare.
#[test]
fn rejects_a_url_body_that_reads_as_an_unclosed_construct() {
  rejects(
    "backgroundImage",
    &["url(it's-fine.png)", "url(a/*b.png)"],
    UNCLOSED_FUNCTION,
    &default_options(),
  );
}

/// The `url(` lookbehind must not fire for an identifier that merely *ends* in
/// `url`, and it must walk characters rather than bytes: `open_paren - 3` is not
/// necessarily a UTF-8 boundary, and slicing there used to panic.
#[test]
fn does_not_treat_a_url_suffixed_identifier_as_a_url() {
  check(
    &[
      // `éurl(` is not `url(`, so the body is spaced as CSS — and the reference
      // compiler spaces the `/` there too.
      same("width", "éurl(foo/2)", "éurl(foo / 2)"),
      same("width", "日本(1/2)", "日本(1 / 2)"),
      // Fewer than three characters before the `(`, which is where the
      // byte-indexed lookbehind used to go out of bounds.
      unchanged("width", "éab(1)"),
      unchanged("width", "é(1)"),
    ],
    &default_options(),
  );
}

// ── Comments ─────────────────────────────────────────────────────────

/// The repair copies a comment through untouched — it has to, since spacing a
/// `/*` would turn it into `/ *` and destroy the declaration — but the comment
/// never survives that far: SWC's codegen drops it, so what the seam returns is
/// the value with its comments already gone.
///
/// That difference is the point of migrating these cases. The repair's own
/// contract is "a comment is copied verbatim"; the compiler's is "a comment is
/// dropped", and only the second is observable. The reference compiler keeps
/// the comment in the declaration, so all of these diverge — and for
/// `/*/ x */`, which it mangles into `/**/ x * /`, they diverge twice over.
#[test]
fn drops_comments_from_the_value() {
  check(
    &[
      diverges("width", "/* a */1px", "1px", "/* a */1px"),
      diverges("width", "/*/ x */ 1px", "1px", "/**/ x * / 1px"),
      diverges("width", "1px /*/ y */", "1px", "1px /**/ y * / "),
      // A `}` inside a comment is comment text: the comment is dropped whole
      // rather than terminating the rule, which is why this is not rejected.
      diverges("width", "/* a }b */ 1px", "1px", "/* a }b */ 1px"),
    ],
    &default_options(),
  );
}

/// A value that is nothing but a comment normalizes to nothing at all.
///
/// The harness calls this `structurally divergent` rather than `divergent`: an
/// empty value makes this compiler drop the declaration outright, so the two
/// emit a different number of them. The reference spelling below is still the
/// declaration text the reference compiler produces, which is what a pipeline
/// change would have to start producing to close the gap.
#[test]
fn diverges_on_a_value_that_is_only_a_comment() {
  check(
    &[
      diverges("width", "/* a */", "", "/* a */"),
      diverges("width", "/**/", "", "/**/"),
    ],
    &default_options(),
  );
}

// ── Values the repair must leave alone ───────────────────────────────

/// The pass only ever inserts a space where one is missing. A value already
/// spelled the way the compiler would spell it comes back byte for byte, which
/// is what makes running it a second time a no-op — and what makes every case
/// here agree with the reference compiler.
#[test]
fn leaves_an_already_spaced_value_alone() {
  check(
    &[
      unchanged("boxShadow", "1px solid red"),
      unchanged("boxShadow", "1px solid #000"),
      unchanged("width", "calc(100% - 20px)"),
      unchanged("width", "calc(100% - 2px)"),
      unchanged("color", "var(--color) var(--bg)"),
      unchanged("color", "var(--my-color)"),
      unchanged("margin", "10px 20px"),
      unchanged("width", "10px"),
      unchanged("color", "red"),
      // A single character has no pair to place a space between.
      unchanged("color", "a"),
    ],
    &default_options(),
  );
}

/// An empty value, and one that is nothing but whitespace, both normalize to
/// empty rather than to a space or a panic.
///
/// The harness verdict is `acceptance divergent`: the reference compiler
/// rejects both outright. There is no reference spelling to compare against, so
/// this is asserted directly rather than through the case table.
#[test]
fn normalizes_an_empty_value_to_nothing() {
  let options = default_options();

  assert_eq!(normalize_css_property_value("color", "", &options), "");
  assert_eq!(normalize_css_property_value("color", "   ", &options), "");
}

// ── Non-ASCII content ────────────────────────────────────────────────

/// Text that needs no escape survives byte for byte, bare and inside a string,
/// across scripts and across UTF-8 lengths up to four bytes. The pass scans
/// characters rather than bytes, and every one of these used to be a way to
/// slice one in half.
#[test]
fn preserves_non_ascii_content() {
  check(
    &[
      unchanged("color", "•"),
      unchanged("fontFamily", "✓"),
      unchanged("fontFamily", "日本語"),
      unchanged("fontFamily", "привет"),
      unchanged("fontFamily", "שלום"),
      unchanged("fontFamily", "مرحبا"),
      unchanged("fontFamily", "😀"),
      unchanged("fontFamily", "🎉"),
      unchanged("fontFamily", r#""•""#),
      unchanged("fontFamily", r#""•✓日本語😀""#),
    ],
    &default_options(),
  );
}

/// The same content in single quotes, where the rewritten quote character is
/// the only divergence.
#[test]
fn diverges_on_the_quote_character_around_non_ascii_content() {
  check(
    &[diverges(
      "fontFamily",
      r#"'•✓日本語😀'"#,
      r#""•✓日本語😀""#,
      r#"'•✓日本語😀'"#,
    )],
    &default_options(),
  );
}

// ── The leading zero on a negative decimal ───────────────────────────

/// The minifier strips the `0` from `-0.5px`, and CSS reads what is left as a
/// subtraction rather than a negative number. The repair puts it back — but
/// only where the `-` is a sign, never where it is an operator.
///
/// The reference compiler keeps the author's `-.5px`, so restoring the zero is
/// a divergence in every one of these. It is also the only thing standing
/// between this compiler and invalid CSS, which is why the repair exists.
#[test]
fn restores_the_leading_zero_on_a_negative_decimal() {
  check(
    &[
      diverges("marginTop", "-.24px", "-0.24px", "-.24px"),
      diverges("transitionDuration", "-.9s", "-0.9s", "-.9s"),
      diverges("opacity", "-.5", "-0.5", "-.5"),
      diverges(
        "width",
        "calc(-.5px + 1px)",
        "calc(-0.5px + 1px)",
        "calc(-.5px + 1px)",
      ),
      // Inside a function argument list, and after a multi-byte character.
      diverges(
        "transform",
        "translate(-.5px,-.25px)",
        "translate(-0.5px,-0.25px)",
        "translate(-.5px,-.25px)",
      ),
      diverges(
        "transform",
        "translate(🎉,-.5px)",
        "translate(🎉,-0.5px)",
        "translate(🎉,-.5px)",
      ),
      // A `-` in sign position that is not followed by `.<digit>` is left
      // alone, and a later one is still restored.
      diverges("margin", "-5px -.5px", "-5px -0.5px", "-5px -.5px"),
    ],
    &default_options(),
  );
}

/// A positive decimal keeps the stripped zero — that spelling is the
/// compiler's, not an accident — and a `-` that follows a complete token is a
/// subtraction operator whatever the token was: a dimension, or a closing
/// paren. Both compilers agree on all three, which is what makes them the
/// boundary of the rule above.
#[test]
fn leaves_subtraction_operators_alone() {
  check(
    &[
      unchanged("opacity", ".5px"),
      unchanged("width", "calc(1px-.5px)"),
      unchanged("width", "calc(var(--x)-.5px)"),
    ],
    &default_options(),
  );
}

/// A percentage followed by `-.5px` reads as subtraction to the repair, so the
/// zero would not be restored there — except this value takes the
/// unknown-syntax fallback, which runs the restoration over the author's text
/// before the repair ever sees it, and the zero comes back anyway.
///
/// Recorded as it behaves. The disagreement between the two paths is a
/// symptom of the same problem the pipeline replacement addresses: a spelling
/// decided by which branch a value happens to take.
#[test]
fn diverges_on_a_negative_decimal_after_a_percentage() {
  check(
    &[diverges("width", "10%-.5px", "10%-0.5px", "10%-.5px")],
    &default_options(),
  );
}

/// The restoration never reaches inside a string, so a value that happens to
/// contain the pattern is left as written — including when the string carries
/// multi-byte content, and when a restoration in the surrounding value has to
/// step over it.
#[test]
fn keeps_the_restoration_out_of_strings() {
  check(
    &[
      unchanged("fontFamily", r#""-.5""#),
      unchanged("fontFamily", r#""🎉 -.5""#),
      diverges(
        "transform",
        r#"translate(-.5px,"-.25px")"#,
        r#"translate(-0.5px,"-.25px")"#,
        r#"translate(-.5px,"-.25px")"#,
      ),
    ],
    &default_options(),
  );
}

/// The same string in single quotes, where the rewritten quote character is the
/// only divergence.
#[test]
fn diverges_on_the_quote_character_around_a_protected_decimal() {
  check(
    &[diverges("fontFamily", r#"'-.5'"#, r#""-.5""#, r#"'-.5'"#)],
    &default_options(),
  );
}

/// The escaped-quote defect again, on a string whose contents are the
/// restoration pattern: the quotes are dropped by the codegen and the generated
/// rule's closing brace lands in the declaration.
#[test]
fn diverges_on_an_escaped_quote_around_a_protected_decimal() {
  check(
    &[diverges(
      "fontFamily",
      r#""a\"-.5""#,
      r#"a"-.5}"#,
      r#""a\"-.5""#,
    )],
    &default_options(),
  );
}
