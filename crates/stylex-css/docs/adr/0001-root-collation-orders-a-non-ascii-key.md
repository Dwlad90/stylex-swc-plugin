# Root collation orders a non-ASCII condition key

**Status:** accepted

[`pseudo_comparator`](../../src/utils/pre_rule.rs) orders a run of
[condition keys](../../CONTEXT.md) the way the reference implementation's
`localeCompare` orders them. Its [ASCII fast path](../../CONTEXT.md) answers
every key CSS itself defines, because every pseudo-class and pseudo-element name
is ASCII. This decision is about the other half: what orders a key the fast path
cannot claim.

Every number below was taken from a build on this repository rather than
estimated, because the choice turns on the size of one of them.

## Which characters a condition key can carry

Argued rather than asserted, because the answer decides whether a bounded table
is even an option.

`sort_pseudos` is handed pseudo-classes, pseudo-elements and attribute
selectors. Every pseudo-class and pseudo-element name CSS defines is ASCII, so
nothing non-ASCII can arrive that way. It arrives only through an attribute
selector, in two positions: the attribute _name_, which for a `data-*` attribute
is an XML Name and admits letters from every script; and a quoted attribute
_value_, which is arbitrary text.

The second settles the question. The set is not bounded in principle, so any
generated range leaves a remainder, and the remainder is unbounded. Latin-1
Supplement plus Latin Extended-A would serve a Western-European author; Greek,
Cyrillic, CJK and emoji are each one quoted attribute value away.

## Considered options

**Option A -- `icu_collator`.** Exact: it places `[data-état]` between
`[data-e]` and `[data-f]`, and wiring it in as a probe turned the parity
corpus's one `divergent` row into `identical`, read from a run.

- _Dependency surface_: 33 crates in its tree, of which **26 are already in this
  workspace's lockfile** -- SWC reaches the ICU4X normalizer, properties,
  provider, collections and the zerovec/yoke family through `idna_adapter`. Six
  are new: `icu_collator`, `icu_collator_data`, `icu_locale_fallback`,
  `icu_locale_fallback_data`, `utf16_iter`, `write16`.
- _Compile time_: **2.74s of CPU** across those six, 2.04s of it `icu_collator`
  itself, from `cargo build --timings`. On top of that, once: adding it moves
  twelve already-present crates forward inside their semver ranges and hands
  `icu_normalizer` two features it did not have, which invalidates `idna`, `url`
  and the SWC stack for one rebuild.
- _Binary size_: **+1 222 800 bytes on the `.node` addon**, 9 777 328 to
  11 000 128 -- 12.5%. The same delta to within 16 bytes on a standalone binary,
  so it is the CLDR tables rather than shared code.

**Option B -- a generated weight table.** The table is not the problem. A dense
per-code-point rank derived from `Intl.Collator` the way `ASCII_PRIMARY_ORDER`
was derived is 256 bytes for Latin-1 Supplement, 384 for Latin Extended-A, 880
with combining diacritics and 2 560 with Greek and Cyrillic. Three orders of
magnitude below the ICU data.

The problem is that it does not work. Comparing that table's comparator against
`localeCompare` over 200 000 random pairs drawn from each range: printable
ASCII, which is what ships today, disagrees on **0**; Latin-1 Supplement on
**0.50%**; Latin Extended-A on **0.29%**; add combining diacritics and it is
**9.95%**. Three structural reasons, each observed rather than reasoned about:

1. _Secondary weights._ `é` and `e` share a primary weight, and root collation
   settles the tie on an accent pass that sits between the primary pass and
   case. A dense-rank comparator has no such pass -- primary, then length, then
   case -- so every pair whose primary weights tie on an accented letter sorts
   wrong. Eight of the first ten disagreements.
2. _Completely ignorable characters._ `U+00AD` SOFT HYPHEN carries no weight at
   all in root collation. A dense rank must give it one, which shifts every
   position after it.
3. _Expansions._ `æ` weighs as two primary weights, `a` then `e` -- measured:
   `localeCompare('æ', 'az')` is negative. One character cannot carry two ranks
   in a per-code-point table.

So the generated table has no defensible edge. Reaching the ordering needs
decomposition, a secondary pass, ignorable handling and expansions -- which is
the collation algorithm, and it then has to be fed a weight table anyway.

## Decision

Take the dependency. 1.17 MiB and six crates, four of them from a family the
workspace already carries, against a divergence that costs a class name and has
no cheaper closing.

## Consequences

**What stays uncovered, and how a reader will know.** Root collation is what
upstream reaches when `localeCompare` is called with no locale _on a machine
whose default locale does not tailor the characters involved_. Upstream does not
call it with `'und'`: it calls it bare, so the runtime's default locale decides.
Measured -- `en-US`, `de-DE` and `tr-TR` all agree with root on `ö`, `ä`, `å`,
`ø`, `ü` and `é`, while `sv-SE` and `da-DK` sort `ö` **after** `z`. So upstream
on a Swedish-locale build machine names a different class than upstream on an
American one, from the same source.

This compiler will always sort as root, which is the majority answer and the
only one a compiler can pick without reading the build machine's environment --
but it means the divergence is closed for most build machines rather than all of
them.

That remainder is not left to memory, and not left to prose either.
`crates/stylex-rs-compiler/parity/fuzz-pseudo-order.ts` counts, per run, how
many of its random key pairs the build machine's default locale and root
collation order differently, and prints that count beside its disagreement
count. On a machine tailoring none of the characters in play it is zero; on a
Swedish one it is not, and the run says so. So the remainder is a number in a
report rather than a sentence someone has to remember to re-derive.

**The boundary is printable ASCII, not `is_ascii()`.** The fast path's table
ranks a byte it does not name above every byte it does, and root collation gives
a control character no weight, so admitting a control character to the table
yields `:ä` < `:z` < `:\u{0002}` < `:ä` -- a cycle, and a sort over a cycle may
return anything. The two paths must agree over every printable-ASCII pair, which
is asserted rather than assumed.
