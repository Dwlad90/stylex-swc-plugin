//! Class-name parity for the edge shapes around value normalization.
//!
//! Issue #1256 named six symptoms; those are pinned next door in
//! `css_value_normalization`. This module pins the neighbourhood — the inputs
//! that are hostile, ambiguous, or simply unusual enough that a normalizer
//! could plausibly touch what it must leave alone. Each one reaches the same
//! contract the six do: a class name is a hash of the declaration text, so any
//! byte this compiler spells differently from `@stylexjs/babel-plugin` is a
//! class name that silently misses its rule.
//!
//! Every expectation is measured against `@stylexjs/babel-plugin@0.19.0` by the
//! parity harness, whose `edge` corpus set carries these values:
//!
//! ```sh
//! pnpm run --filter=@stylexswc/rs-compiler build
//! pnpm run --filter=@stylexswc/rs-compiler parity -- --set edge
//! ```
//!
//! The measurement is quoted above the test that pins it. The declaration text
//! is written the way the harness writes it — a double-quoted JS string,
//! escapes and all — so the source under test is the source that was measured.
//!
//! A handful of lines run past the repo's 100-column limit, and deliberately: a
//! quoted rule wrapped is a quoted rule falsified, and a value literal broken
//! across lines is no longer the text the harness handed both compilers. Those
//! two shapes are the only exceptions here.
//!
//! A snapshot that stops matching its quoted measurement is a divergence from
//! upstream, not a snapshot to re-record.

use crate::utils::prelude::*;

// The options the measurements were taken under: runtime injection so the rule
// text lands in the snapshot, and font-size conversion left off, which is how
// the harness runs both compilers. No value here is a font size, but a helper
// that quietly enabled it would make the snapshots and the quoted measurements
// agree by luck rather than by construction.
fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
  })
}

// A CSS escape sequence names a character; expanding it to that character
// changes the bytes that reach the hash. Both spellings that carry a trailing
// space — the one the escape consumes — and an astral-plane code point are
// pinned, on a property that reaches normalization rather than `content`, which
// short-circuits ahead of it.
//
// Upstream: .x1nhv5et{font-family:"\2014 A",sans-serif}
//           .x1kfa6ls{font-family:"\1F600",sans-serif}
//           .xki5qqo{font-family:My\ Font,sans-serif}
stylex_test!(
  escape_sequences_are_not_resolved,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      trailingSpace: { fontFamily: "\"\\2014 A\", sans-serif" },
      astralPlane: { fontFamily: "\"\\1F600\", sans-serif" },
      outsideAString: { fontFamily: "My\\ Font, sans-serif" },
    });
  "#
);

// Non-ASCII text survives whole, inside a string and as a bare identifier.
//
// Upstream: .xue0wm0{font-family:"→ Привет 日本語 🙂",sans-serif}
//           .xjmxxf3{font-family:日本語フォント,sans-serif}
//           .xe30er7{content:"→ Привет 日本語 🙂"}
stylex_test!(
  non_ascii_text_survives_whole,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      inAString: { fontFamily: "\"→ Привет 日本語 🙂\", sans-serif" },
      asAnIdentifier: { fontFamily: "日本語フォント, sans-serif" },
      asContent: { content: "\"→ Привет 日本語 🙂\"" },
    });
  "#
);

// A `url()` body is copied verbatim, however much of it looks like CSS syntax:
// a semicolon ends nothing, a brace opens nothing, a colon separates nothing,
// and `/* */` is not a comment.
//
// Upstream: .x14wdgs6{background-image:url("a;b{c}d: e /* f */")}
//           .xb71sbn{background-image:url(image.png?a=1&b=2)}
//           .x19xh8vt{background-image:url("data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg'%3E%3C/svg%3E")}
stylex_test!(
  url_bodies_are_copied_verbatim,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      cssSyntaxInBody: { backgroundImage: "url(\"a;b{c}d: e /* f */\")" },
      unquotedWithQuery: { backgroundImage: "url(image.png?a=1&b=2)" },
      dataUri: {
        backgroundImage:
          "url(\"data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg'%3E%3C/svg%3E\")",
      },
    });
  "#
);

// A comment inside a value is part of the value. Dropping one is a divergence
// even though the CSS means the same thing, because the hash does not.
//
// Upstream: .xlkdqwa{width:calc(100% /* half */ - 20px)}
//           .x16yfguv{color:/* a */ red}
stylex_test!(
  comments_inside_a_value_are_kept,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      betweenTokens: { width: "calc(100% /* half */ - 20px)" },
      leading: { color: "/* a */ red" },
    });
  "#
);

// Malformed values that upstream nonetheless accepts. An unclosed bracket, an
// operator sequence that means nothing, and a string with no closing quote are
// all inert to the normalizers, which is why they pass through rather than
// being repaired into something else.
//
// Upstream: .xlytkhd{grid-template-columns:[full-start 1fr [content-start}
//           .x124hsr6{width:10px ++ 20px}
//           .xbjs7n6{content:""unterminated"}
stylex_test!(
  malformed_but_accepted_values_pass_through,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      unclosedBracket: { gridTemplateColumns: "[full-start 1fr [content-start" },
      invalidTokenSequence: { width: "10px ++ 20px" },
      unclosedString: { content: "\"unterminated" },
    });
  "#
);

// Vendor prefixes: a prefixed keyword, a prefixed function whose body carries
// hex colours that must not be shortened, and both spellings of a prefixed
// value name — the camel-case one is hyphenated to the same declaration the
// already-hyphenated one produces, so the two share a class name.
//
// Upstream: .x104kibb{display:-webkit-box}
//           .x17doe4i{background-image:-webkit-linear-gradient(top,#FFFFFF,#000000)}
//           .x145t2h1{transition-property:-webkit-transform}   (both spellings)
stylex_test!(
  vendor_prefixes_are_carried_through,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      prefixedKeyword: { display: "-webkit-box" },
      prefixedFunction: { backgroundImage: "-webkit-linear-gradient(top, #FFFFFF, #000000)" },
      camelCaseValue: { transitionProperty: "WebkitTransform" },
      hyphenatedValue: { transitionProperty: "-webkit-transform" },
    });
  "#
);

// Depth and length the normalizers walk without disturbing. The nesting here is
// well inside the depth guard; the value past that guard is rejected instead,
// and that rejection is pinned below.
//
// Upstream: .x8dv455{width:calc(calc(calc(calc(calc(calc(calc(calc(1px + 2px) + 3px) + 4px) + 5px) + 6px) + 7px) + 8px) + 9px)}
//           .xebum3e{color:var(--a,var(--b,var(--c,var(--d,var(--e,var(--f,red))))))}
//           .x1b3eisk{box-shadow:0 0 1px #000,0 0 2px #000,0 0 3px #000,0 0 4px #000,0 0 5px #000,0 0 6px #000,0 0 7px #000,0 0 8px #000,0 0 9px #000,0 0 10px #000}
stylex_test!(
  deep_nesting_and_long_lists_are_walked_intact,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      nestedCalc: {
        width: "calc(calc(calc(calc(calc(calc(calc(calc(1px + 2px) + 3px) + 4px) + 5px) + 6px) + 7px) + 8px) + 9px)",
      },
      nestedFallbacks: { color: "var(--a, var(--b, var(--c, var(--d, var(--e, var(--f, red))))))" },
      longShadowList: {
        boxShadow:
          "0 0 1px #000, 0 0 2px #000, 0 0 3px #000, 0 0 4px #000, 0 0 5px #000, 0 0 6px #000, 0 0 7px #000, 0 0 8px #000, 0 0 9px #000, 0 0 10px #000",
      },
    });
  "#
);

// Numbers at the edges of what a normalizer will re-spell: one too large to
// hold exactly, one carrying float noise, and the two leading-zero rules —
// stripped when positive, kept when negative.
//
// Upstream: .xl3gh7a{z-index:10000000000000000000000}
//           .xcqrntm{width:.30000000000000004px}
//           .xbyyjgo{opacity:.5}
//           .x1gu5id8{letter-spacing:-0.24px}
stylex_test!(
  extreme_numbers_keep_their_authored_spelling,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      veryLarge: { zIndex: "10000000000000000000000" },
      highPrecision: { width: "0.30000000000000004px" },
      leadingZeroStripped: { opacity: "0.5" },
      negativeLeadingZeroKept: { letterSpacing: "-0.24px" },
    });
  "#
);

// A custom property is its own normalization regime: the value-name
// hyphenation and the zero-dimension rule both stand down, because the compiler
// cannot know what the property means. Reading one back is ordinary, including
// a `var()` immediately followed by a unit, which must not gain a space.
//
// Upstream: .x1sgzfop{--myVar:backgroundColor}
//           .x3592ib{--myVar:0px}
//           .x19srcev{color:var(--x,var(--y,#ABCDEF))}
//           .x1tjm4ty{width:var(--x)px}
stylex_test!(
  custom_properties_are_exempt_from_value_normalization,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      camelCaseValue: { "--myVar": "backgroundColor" },
      zeroLength: { "--myVar": "0px" },
      nestedFallback: { color: "var(--x, var(--y, #ABCDEF))" },
      flushUnit: { width: "var(--x)px" },
    });
  "#
);

// Letter case is never an input to a normalizer, so a shouted keyword and a
// shouted function name both survive as written.
//
// Upstream: .x1w2zu9n{text-transform:UPPERCASE}
//           .x13hx3ed{width:CALC(100% - 20PX)}
stylex_test!(
  letter_case_is_preserved,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      keyword: { textTransform: "UPPERCASE" },
      functionName: { width: "CALC(100% - 20PX)" },
    });
  "#
);

// Syntax newer than this compiler's knowledge is normalized and emitted rather
// than rejected — there is no allowlist of function names to fall off.
//
// Upstream: .x13s5x7t{color:oklch(from var(--brand) l c h / 50%)}
//           .xwpildb{height:calc-size(fit-content,size / 2)}
stylex_test!(
  unknown_css_syntax_is_emitted_rather_than_rejected,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      relativeColor: { color: "oklch(from var(--brand) l c h / 50%)" },
      futureFunction: { height: "calc-size(fit-content, size / 2)" },
    });
  "#
);

// Separators and trailing punctuation. Any run of whitespace collapses to one
// space, but a semicolon inside the value is text — it is only rejected when
// something follows it that would read as a second declaration.
//
// Upstream: .x3g7hzd{margin:1px 2px 3px 4px}
//           .x18qx21s{transform:rotate(10deg) translate3d(0,0,0)}
//           .x97xg4o{background-color:var(--web-wash);}
//           .x18bhde2{color:red;;}
stylex_test!(
  separators_collapse_and_trailing_semicolons_stay,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      tabsAndNewlines: { margin: "1px\t2px\n3px  4px" },
      leadingAndTrailing: { transform: "  rotate(10deg)  translate3d( 0 , 0 , 0 )  " },
      trailingSemicolon: { backgroundColor: "var(--web-wash);" },
      repeatedSemicolons: { color: "red;;" },
    });
  "#
);

// The importance annotation keeps the spacing it was written with, which is a
// hash input like any other.
//
// Upstream: .xzw3067{color:red!important}
//           .x1rf7pop{color:red ! important}
stylex_test!(
  importance_annotation_keeps_its_spacing,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      tight: { color: "red !important" },
      spaced: { color: "red   !   important" },
    });
  "#
);

// Numbers at the ends of a double's range, where a wrong digit inside a class
// name looks like no defect at all. Four of them do not come back as written;
// which four, and why, is set out at the normalization seam in
// `spells_numbers_at_the_edges_of_a_double_the_way_the_reference_does`. What
// this adds is the class name each one hashes to.
//
// Upstream: .x1g9kooq{width:1e309px}
//           .x1lx9cm8{width:0px}
//           .x1z0jktx{width:1.7976931348623157e308px}
//           .x6pb8ui{width:5e-324px}
//           .x113yxd9{width:9007199254740993px}
//           .xg144zj{width:123456789012345678901234567890px}
//           .xp7cifa{width:1epx}
//           .xd9flah{width:.}
//           .xnalus7{width:0}
//           .x1lx9cm8{width:0px}
//           .x1dwv5jz{opacity:.12345678901234568}
stylex_test!(
  numbers_at_the_edges_of_a_double_keep_the_reference_spelling,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      overflowsToInfinity: { width: "1e309px" },
      underflowsToZero: { width: "1e-324px" },
      largestFinite: { width: "1.7976931348623157e308px" },
      smallestSubnormal: { width: "5e-324px" },
      pastDoublePrecision: { width: "9007199254740993px" },
      thirtyDigits: { width: "123456789012345678901234567890px" },
      exponentWithNoDigits: { width: "1epx" },
      bareDecimalPoint: { width: "." },
      trailingDecimalPoint: { width: "0.px" },
      negativeZero: { width: "-0px" },
      highPrecisionFraction: { opacity: "0.12345678901234567890123456789" },
    });
  "#
);

// Multi-byte characters at the byte offsets the passes cut on; which
// characters, and what each one risks, is set out at the normalization seam in
// `spells_multi_byte_characters_at_a_slicing_boundary_the_way_the_reference_
// does`. What this adds is the class name each one hashes to.
//
// Three of the seven are invisible, and the measurements below carry them as
// the raw characters the harness handed both compilers rather than as escapes —
// so `"a‏b"` holds a right-to-left mark, `"a﻿b"` a byte-order mark, and
// `a b` a non-breaking space, each between the two letters.
//
// Upstream: .x8oru8l{font-family:"é"}
//           .x5l3f0p{font-family:é}
//           .xvgeugp{font-family:"a‏b"}
//           .xz4egqn{font-family:"a﻿b"}
//           .x19sifd9{font-family:a b}
//           .x1iw8uwt{font-family:\😀}
//           .xqrlfat{width:1😀px}
stylex_test!(
  multi_byte_characters_survive_a_slicing_boundary,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      combiningMarkInAString: { fontFamily: "\"é\"" },
      combiningMarkAsAnIdentifier: { fontFamily: "é" },
      rightToLeftMark: { fontFamily: "\"a‏b\"" },
      byteOrderMark: { fontFamily: "\"a﻿b\"" },
      nonBreakingSpace: { fontFamily: "a b" },
      astralPlaneAfterAnEscape: { fontFamily: "\\😀" },
      astralPlaneBetweenNumberAndUnit: { width: "1😀px" },
    });
  "#
);

// The arrangements a totality sweep of the normalization seam found not to
// settle: normalize the output again and it moves, and in the first case it
// grows a space on every run. Nothing normalizes twice today, so what a
// declaration actually gets is the first run's spelling — and the reference
// compiler produces the same one, trailing character and acquired space
// included.
//
// Upstream: .x1nlq1fn{width:() / }
//           .x1prxcy3{width:0)}
//           .x940y1d{width:0\}
//           .xli7qyh{width:0*}
stylex_test!(
  arrangements_that_do_not_settle_keep_the_reference_spelling,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      emptyFunctionThenSeparator: { width: "()/" },
      trailingParen: { width: "00)" },
      trailingBackslash: { width: "00\\" },
      trailingAsterisk: { width: "00*" },
    });
  "#
);
