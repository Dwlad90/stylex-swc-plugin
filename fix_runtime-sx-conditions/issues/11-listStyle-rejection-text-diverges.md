# 11 — The `listStyle` rejection text diverges from upstream

Status: `resolved`
Phase: Phase 2

**What was built:** `legacy-expand-shorthands`'s four `listStyle` rejections say
what upstream says, byte for byte, and a test holds each one to it.

Found while verifying 07's message text against upstream. Both compilers refuse
the same four cases, so no build outcome differs — only the wording did.

|                                 | text                                    |
| ------------------------------- | --------------------------------------- |
| `@stylexjs/babel-plugin` 0.19.0 | `invalid "listStyle" value of ""none""` |
| this compiler, before           | `Invalid listStyle value: 'none'`       |
| this compiler, after            | `invalid "listStyle" value of ""none""` |

Three differences, all now closed: the leading capital, `value:` versus
`value of`, and the value rendered with single quotes rather than through
`JSON.stringify`.

## The inconsistency was copied, not normalised

Upstream is not self-consistent. The first of its four throws wraps the
stringified value in an extra pair of quotes and the other three do not —

```text
`invalid "listStyle" value of "${JSON.stringify(rawValue)}"`  // line 301
`invalid "listStyle" value of ${JSON.stringify(rawValue)}`    // 308, 317, 338
```

so a `var()` or global-keyword rejection reads `value of ""none inherit""` and a
duplicate-position rejection reads `value of "inside outside"`. Measured, not
inferred: every case below was run through the installed 0.19.0 plugin under
`styleResolution: 'legacy-expand-shorthands'` and `propertyValidationMode:
'throw'`, and the two shapes are what came back.

Copied byte for byte. These messages reach an author through
`propertyValidationMode`, and an author comparing the two compilers on the same
input should read the same sentence from both — the asymmetry is upstream's to
fix, and normalising it here would add a divergence nobody asked for. The two
shapes are named `list_style_rejection` and
`list_style_rejection_with_doubled_quotes` so which one a site uses is visible
at the call, and the doubling is documented at the second rather than left to
look like a typo.

## `JSON.stringify` needed a Rust spelling

The value arrives quoted and JSON-escaped, not printed raw, so
`stylex_utils::string::json_stringify` renders a `&str` the way
`JSON.stringify` renders a JS string: `"` and `\` escaped, the five
single-letter escapes `\b \f \n \r \t`, every remaining code point below U+0020
as a lowercase four-digit `\uXXXX`, and everything from U+0020 up written
through unchanged — U+007F and U+2028 included.

`serde_json` does the escaping, via `Value::String(..).to_string()` so there is
no serializer `Result` whose `Err` arm no input reaches. The escape sets agree
byte for byte, which is not assumed: the eleven tests were written against the
list above and pass unchanged against `serde_json`, so they now hold the
agreement rather than an escape table.

Rust's `{:?}` was the tempting shortcut and is the wrong text: it spells a C0
control `\u{1}` where JSON spells it `\u0001`, and escapes `'`. The one rule of
well-formed `JSON.stringify` left out is unreachable rather than missing — a
lone surrogate escapes to `\ud83c` in JS, and a Rust `str` cannot hold one.

Recorded in `crates/stylex-utils/CONTEXT.md` as **JS string quoting**, beside
the two existing JS-spelling helpers. It is observable for a different reason
than they are: this spelling never reaches a stylesheet, only a message an
author reads.

`TRawValue::Number` never reaches a throw, so the message never has to render a
number the way `JSON.stringify` would: all four sites need at least two tokens
or a `var(`/global keyword, and a rendered number is a single token that is
neither.

## Coverage

The parity harness compares outcomes and not messages, so nothing there holds
this — the assertions do.

- `crates/stylex-utils/src/tests/string_test.rs` — eleven tests on
  `json_stringify`: the plain case, the empty string, an embedded quote, a
  backslash, a backslash immediately before a quote escaping as two escapes and
  not one, the five single-letter escapes, the `\uXXXX` form in lowercase hex,
  U+0020 as the boundary, DEL and both line separators raw, non-ASCII and astral
  scalars raw, and a value of ten thousand quotes.
- `crates/stylex-css/src/order/tests/legacy_expand_shorthands_constants_test.rs`
  — the four error paths now assert the exact text rather than
  `contains("Invalid listStyle")`, plus which site fires for a leading global
  keyword, for each of the four global keywords, for two quoted types, for a
  duplicate position followed by a global keyword (the position site wins, so no
  doubled quotes), and for a value carrying a tab, a C0 control, non-ASCII, an
  astral scalar, or a thousand tokens.
- `crates/stylex-transform/tests/legacy/transform_legacy_shorthands/while_using_standard_logical_properties.rs`
  — the two end-to-end `propertyValidationMode: throw` tests updated to the new
  text, which is where the doubled quotes are pinned as an author reads them.
