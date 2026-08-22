# 26 — `NaN` and `Infinity` as a style array element

Status: `resolved`
Blocked by: None

**What was found:** A fallback array holding `NaN` or `Infinity` is refused
here and compiled upstream, in both style-value positions.

```js
export const styles = stylex.create({ s: { height: [NaN, '2px'] } });
```

| input | Babel 0.19.0 | this compiler |
| --- | --- | --- |
| `height: [NaN, '2px']` | `.x…{height:2px}` — the element is dropped | `A style array value can only contain strings or numbers.` |
| `height: [Infinity, '2px']` | `.x…{height:Infinitypx;height:2px}` | the same refusal |

The two upstream answers differ from each other, which is the interesting half:
`NaN` is dropped as an absent value and `Infinity` is spelled into the
declaration with the property's unit appended. Neither is a number to the array
check here, because both reach it as identifiers rather than as numeric
literals — the same representation fact ticket 05 settled for the value
position, one level further in.

`height: Infinitypx` is not a declaration any browser accepts, so upstream's
answer for the second row is not obviously the one to adopt. A verdict is
wanted before the check moves.

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration, in a static namespace and inside a dynamic style's body — both
positions read the same, so this is about the array check and not about the
position. Found while measuring ticket 14, whose fold is what made the dynamic
half reachable.

- [x] A verdict per row, `NaN` and `Infinity` separately
- [x] Either the fold, or a recorded reason to keep refusing
- [x] `modules-1266-an-array-with-a-nan-element-in-a-dynamic-style` and a static
      counterpart carry the decided verdict

## Answer

Both rows fold, and the verdict the ticket asked for turned out not to be a
choice. Landed as one commit.

### The measurement that settled it

The ticket framed this as "is upstream's answer worth adopting", with
`height:Infinitypx` as the reason to hesitate. Two rows measured while checking
the family answer that, and neither involves a global:

| input | Babel 0.19.0 | here, *before* this change |
| --- | --- | --- |
| `height: [0 / 0, '2px']` | `height:2px` | `height:2px` |
| `height: [1 / 0, '2px']` | `height:Infinitypx;height:2px` | the same |
| `height: -Infinity` | `height:-Infinitypx` | the same |

This compiler already emitted every one of upstream's answers, byte for byte,
including the one that reads like nonsense -- and `-Infinity` already did it for
the global itself, because the unary minus computes a number. So there was
never a rule to decide: refusing `Infinity` while emitting `Infinitypx` for
`1 / 0` and for `-Infinity` is incoherent whichever way one likes the CSS.

For the record, the CSS is defensible too. A fallback array *is* a chain of
declarations, and an unusable one ahead of a usable one is how a CSS fallback
works -- the browser discards what it cannot parse and reads the next.

### It was never the array check

The array check and upstream's are the same rule: upstream's
`validateNamespace` continues on `typeof v === 'number'`, and
`is_style_value_literal` admits `Lit::Num`. What differed was what arrived. The
resolution chain's globals step answered the *name*, so `NaN` and `Infinity`
reached a consumer that reads an expression's shape as identifiers, and were
refused as unresolved references.

The fix is at that step: it answers the value now.
`global_spelled_as_an_identifier_as_a_value` sits beside the set predicate in
`stylex-js::coercions`, which already owned the three names, and gives `NaN` and
`Infinity` numeric literals while `undefined` answers itself -- nothing else
spells it. Every consumer downstream agreed at once; no consumer was taught
anything.

### This closes what ticket 05 left open

05 recorded `width: NaN` with nothing bound as a remaining divergence
(`modules-1266-the-unshadowed-globals-in-a-style-value`, `acceptance
divergent`), correctly calling it a CSS-value question rather than a resolution
one. It was a resolution question after all -- the same one, one level out. That
entry and `modules-1266-a-global-beside-an-unrelated-binding-of-its-name` both
read `identical` now.

The shadowing half of 05 is untouched: a binding still wins, and a parameter
named `NaN` still falls through to the inline-style path.

### One thing the emitter needed telling

A `Number` node holding `NaN` prints as `0 / 0`, and `Infinity` as a numeral no
author wrote. Both evaluate correctly, but the text is what a reader diffs, so
the literals carry their authored spelling as `raw`.

### A defect found by the edge cases, one line of it fixed here

`NaN ? '1px' : '2px'` chose the wrong branch. A ternary reads its test through
`convert_expr_to_bool`, a second truthiness table beside
`coercions::to_js_boolean` whose numeric arm asked `n.value != 0.0` -- the one
comparison `NaN` answers true. `NaN || '2px'` reads the coercion and was always
right.

**This was first written down as pre-existing and deferred, and that was wrong.**
Code review caught it. `(0 / 0) ? '1px' : '2px'` does show the table was already
broken, but the *named* row was not reachable before: while `NaN` resolved to an
identifier, `convert_expr_to_bool`'s identifier arm ran
`convert_ident_to_expr`, which for an unbound name aborts with `A style value
can only contain an array, string or number.` -- verified by building at the
parent commit and compiling the input. So answering the global as a number
turned a **build refusal into silently wrong CSS**, which is the exact failure
class this whole effort exists to remove. A loud wrong answer is survivable; a
quiet one is not.

The numeric arm is corrected here, one line, mirroring the coercion. Both rows
now emit `height:2px` as upstream does, and `-1`, `0` and `Infinity` are pinned
beside them. The rest of that table -- the three unary arms that negate their
operand's truthiness, and the `_` arm that aborts where the coercion refuses --
is genuinely pre-existing and stays with
[39](./39-a-second-truthiness-table-calls-nan-true.md). Pinned as
`globals_as_style_values::a_nan_test_in_a_ternary_takes_the_falsy_branch` and as
corpus `modules-a-nan-test-in-a-ternary`, both now reading agreement.

### Verification

`cargo test --workspace --all-features` 0 failed, `cargo clippy --workspace
--all-features --all-targets` clean, `cargo fmt` clean, `pnpm typecheck &&
pnpm lint:check && pnpm test` green. `parity` 0 changed verdicts over 1019
subjects; three entries moved to `identical` and five were added.
