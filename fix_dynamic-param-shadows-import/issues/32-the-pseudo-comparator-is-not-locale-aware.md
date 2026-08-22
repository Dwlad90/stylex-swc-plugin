# 32 — Two pseudo names differing only in case hash a different class name

Status: `resolved`
Blocked by: None

**What was found:** Two nested pseudo-classes whose names differ only in letter
case sort into a different order than the reference implementation puts them in,
so the two compilers hash a different class name for one authored style.

```js
export const styles = stylex.create({
  w: { color: { ':HOVER': { ':active': 'red' } } },
});
```

| | leaf selector | class |
| --- | --- | --- |
| `@stylexjs/babel-plugin` 0.19.0 | `:active:HOVER` | `.xyhlusd` |
| rs-compiler HEAD | `:HOVER:active` | `.xi7zcr1` |

Same declaration, same rule count: the verdict is `divergent`.

Measured as `modules-pseudo-names-differing-only-in-case-hash-differently`,
which records `divergent` so it reports as a changed verdict the day it moves.
Pinned in the suite as
`transform_stylex_create_test::nested_pseudo_ordering::two_pseudo_names_differing_only_in_case`
and, at three keys, as `an_uppercase_pseudo_name_sorts_by_its_bytes`.

## The mechanism

Both compilers group the pseudo list into the same runs — that is
[19](./19-three-nested-pseudo-classes-hash-differently.md)'s question, and it
agrees. What differs is the **comparator** each run is sorted with:

| | comparator | orders `:HOVER` / `:active` |
| --- | --- | --- |
| upstream | `String.prototype.localeCompare` | `:active` first |
| this compiler | `str::cmp`, i.e. byte order | `:HOVER` first |

`localeCompare` under the root collation orders a letter's lowercase form below
its uppercase form and sorts a base letter ahead of its accented variants; a byte
comparison puts every uppercase ASCII letter below every lowercase one and every
non-ASCII byte above all of them. So the two disagree wherever the first
differing position is a case pair or a non-ASCII character:

| pair | `localeCompare` | byte order |
| --- | --- | --- |
| `:a` / `:A` | `:a` first | `:A` first |
| `:HOVER` / `:hover` | `:hover` first | `:HOVER` first |
| `:ä` / `:z` | `:ä` first | `:z` first |

Two keys is enough, so this is not the run-length shape [19](./19-three-nested-pseudo-classes-hash-differently.md)
fixed, and it did not arrive with that fix — it was found by that fix's edge
cases. It has been there as long as `sort_pseudos` has.

Reachable by an author: neither compiler lowercases a condition key, and CSS
pseudo-class names are case-insensitive, so `:HOVER` compiles as a working
selector on both sides — just under two different class names.

The same comparator sorts at-rules (`sort_at_rules`), where upstream's plain
`.sort()` is code-unit order rather than `localeCompare` — a *third* comparator,
and one this compiler's `default`-first `string_comparator` also does not match.
`default` never reaches `sort_at_rules` from a key-path filter that keeps only
`@`-prefixed keys, so the branch is unreachable; whether the at-rule side
diverges on a non-ASCII media query is not measured yet.

## What the fix costs, as far as it is scoped

`localeCompare` is ICU root-collation ordering. Reproducing it is not a
one-line comparator swap — it is either a collation dependency or a deliberate
subset of it, and either is a decision for a person rather than an obvious
correction. The subset that would settle every case measured here is narrower
than full collation, which is worth pinning before reaching for a crate.

- [x] The comparator's required behaviour is pinned as a table of pairs, not as
      "match `localeCompare`" — with the non-ASCII and case pairs above in it
- [x] The cost of matching it is decided: a collation dependency, a subset, or
      `wontfix` with the divergence documented where an author would look
- [x] If fixed: `sort_at_rules` is measured the same way, since it shares the
      comparator and upstream sorts at-rules with a *different* one again
- [x] The corpus row's `expected` becomes `identical`, and the two suite cases
      are re-recorded

## Answer

Fixed for ASCII, documented for the rest, and the at-rule side deliberately left
where it was.

**The comparator is now the ASCII half of root collation.**
`stylex_css::utils::pre_rule::pseudo_comparator` compares ASCII-case-folded
bytes, and remembers the first position where two keys differed only in case as
a tiebreak that the length check gets to settle first. That is the shape root
collation has: a letter's *primary* weight ignores its case, case is a
*tertiary* difference read only on a tie, and lowercase ranks below uppercase.
The fold is also what lifts every letter above the ``[ \ ] ^ _ ` `` block that
sits between the two ASCII cases, which is where a byte comparison had put an
uppercase letter.

**Pinned as a table of pairs**, which was the first checkbox and the reason the
rest could be decided. `pre_rule_test.rs` asserts each ordering in both
directions, plus transitivity across the whole chain
`: < :! < :0 < :9 < :a < :A < :b < :B < :z < :Z` and a sort over a shuffle of
it. Every ordering in it -- including the divergent ones -- was read out of
`@stylexjs/babel-plugin` 0.19.0 through `@babel/core` under the parity
harness's options, not inferred from a reading of the collation spec.

**What moved, measured against upstream:**

| input | before | after, and upstream |
| --- | --- | --- |
| `:HOVER` `:active` | `:HOVER:active`, `xi7zcr1` | `:active:HOVER`, `xyhlusd` |
| `:HOVER` `:focus` `:active` | `:HOVER:active:focus`, `x17ymi95` | `:active:focus:HOVER`, `xnnn07p` |

Nothing else in the suite moved. Every previously-agreeing case still agrees,
which is what makes this a contained change: the fold only reorders a pair whose
first difference is a letter's case, or a letter against the six characters
between the cases.

**Eight new ASCII cases, all measured as agreeing:** the tiebreak alone
(`:hover` before `:HOVER`), the smallest input it has (`:a` before `:A`), the
first-position rule (`:aBc` before `:AbC`), length settling ahead of case (`:a`
before `:aB`), a letter against the between-cases block
(`:_leading:Z[data-x]`), a digit below an uppercase letter, a mixed-case
functional pseudo-class (`:Is(.c):not(.a):NOT(.b)`), and a mixed-case attribute
selector.

**What is still divergent: anything outside ASCII** -- one rule, four
disagreements, all measured rather than reasoned:

| pair | upstream | here |
| --- | --- | --- |
| `:ä` / `:z` | `:ä:z`, `x1enrlzn` | `:z:ä`, `x143q076` |
| `:ä` / `:Ä` | `:ä:Ä`, `xgvn8d` | `:Ä:ä`, `x1th3k6m` |
| emoji / `:hover` | emoji first, `x1jqz5xw` | `:hover` first, `x17d4qyr` |
| lone combining acute / `:hover` | the mark first, `xcdw69q` | `:hover` first |

Every byte at or above `0x80` sorts above every ASCII character here; upstream
gives `ä` the primary weight of `a`, weighs a symbol or a lone combining mark
below every letter, and separates a non-ASCII letter's two cases on the case
rather than on the byte. Closing it needs decomposition and a weight table --
`icu_collator` or a hand-rolled subset of DUCET -- to serve an author who writes
an accented pseudo name *and* nests a second key beside it. That is a dependency
decision with no reported instance behind it, so it is left, named, and
measured. `modules-an-accented-pseudo-name-sorts-above-ascii` is the new corpus
row that reports the day it moves; a non-ASCII key the two *do* agree on
(`U+FFFD`) is pinned beside it, so the boundary is not read as "all non-ASCII
disagrees".

**`sort_at_rules` was measured and deliberately not changed.** Upstream sorts
pseudo keys with `localeCompare` and at-rules with a bare `.sort()` -- three
comparators across the two compilers, not two. So the at-rule side keeps the
plain byte comparison, and making it locale-aware would have been a *new*
divergence rather than a fix. Two cases now say so, one of them asserting both
comparators' answers to the same pair in a single test so the split cannot be
quietly undone. The encoding question the ticket left open is answered: a
`String` compares by UTF-8 bytes and a JavaScript string by UTF-16 code units,
and both are code-point order through the basic multilingual plane, so a
non-ASCII media query cannot reach a disagreement -- only a supplementary
character weighed against a private-use or specials character could, and no
at-rule has a use for either. Measured on `@supports (--ü: 1)` against
`@supports (--z: 1)` and agreeing.

The `default`-first branch in `string_comparator` is confirmed unreachable from
a key path and kept, with a case that says what it did so a reader deleting it
has to delete the statement too.

### Correction: the first pass of this fix under-covered ASCII

The account above says the comparator is "the ASCII half of root collation" and
that everything still divergent is non-ASCII. **That was wrong**, and a review
caught it before the work was called done. Case folding fixes the letters and
nothing else: root collation ranks symbols below digits below letters, and a
folded byte comparison only agrees with that for symbols whose byte is below
`0x30`. Measured against Babel 0.19.0, nine of twelve probe pairs diverged --
`:z` / `:~`, `:1` / `:@`, `:_a` / `:-a`, `[data-x]` / `[data_x]`,
`[a~=b]` / `[ay]` among them. Attribute selectors join the sortable run, so
`[data-x]` beside `[data_x]` is reachable without trying.

**The fix now carries a weight table**, `ASCII_PRIMARY_ORDER`, read out of
`localeCompare` itself by sorting the 95 printable ASCII characters with it:

```
" _-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~$0123456789" + letters, a letter's two cases
sharing one rank
```

Not byte order anywhere: `_` leads `-`, `$` trails every other symbol, and
`{ | } ~` weigh below every letter although their bytes are above every one.
Case stays a tiebreak on top of it, and length still settles ahead of case.

**Validated rather than reasoned.** The model was checked against
`localeCompare` on **200 000 random ASCII pairs** and on every pair drawn from a
list of realistic condition keys: **zero disagreements**. The four transform
cases added for it -- `:~`/`:z`, `:@`/`:1`, `[data_x]`/`[data-x]`, and three
attribute matchers -- were each read back out of Babel and agree byte for byte
on the class name.

**What is left is smaller and stated correctly**: every character the table does
not name -- a control character, `DEL`, and every byte of a non-ASCII character
-- ranks above every character it does. Five measured divergences, all one rule.
`nothing_outside_printable_ascii_is_weighed` and the cases at the end of
`nested_pseudo_ordering` pin them.

The lesson worth keeping: "the ASCII half" was a plausible-sounding boundary
that had not been probed at its edges. The pairs that broke it took one script
to find, and the reason none of the existing suite caught it is that every
degenerate key already pinned there differs from its neighbour at a character
below `0x30`, where byte order and root collation happen to agree.
