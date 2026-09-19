# 58 — A bench named for work it does not do

**What to build:** `InlineStyleToCssString/rtl_flippable_pairs` says it prices
right-to-left flipping. Nothing in it flips. Make the name and the work agree.

The bench is in `crates/stylex-css/benches/css_generation_bench.rs:50`:

```rust
group.bench_function("rtl_flippable_pairs", |b| {
  b.iter(|| black_box(inline_style_to_css_string(&rtl_pairs)))
});
```

`inline_style_to_css_string` is declared at
`crates/stylex-css/src/css/common.rs:790` as
`pub fn inline_style_to_css_string(pairs: &[Pair]) -> String`. It takes no
options. The `rtl_options()` helper in the same bench file is only ever handed
to `generate_css_rule`, so the right-to-left path is never entered here.

**The number is the wrong way round, and that is the tell.** The re-baseline
reads:

| Benchmark | Reads |
| --- | ---: |
| `InlineStyleToCssString/rtl_flippable_pairs` | 91.61 ns |
| `InlineStyleToCssString/ltr_only_pairs` | 154.61 ns |

Flipping is extra work, so the flippable set should be the slower one. It is
faster, and the reason has nothing to do with direction: its keys are already in
kebab case, so `normalize_css_property_name` returns a borrowed value and skips
the five conversions the other set pays. The bench advertises the expensive path
and prices the cheap one.

**Two ways to close it.**

1. **Measure the flipping.** Call the entry point that takes the options, so the
   right-to-left case does the work its name claims.
2. **Rename both cases.** If the point is the property-name conversion, say so:
   `borrowed_keys` and `converted_keys` would describe what the two rows
   actually separate.

Either is fine. What must not stay is a row a reader would quote as the price of
right-to-left support.

**This closes the two rows' series.** Both numbers change under either fix, so
note it in the re-baseline record when it lands.

**Where the numbers are:**
[`../bench/ticket-17.md`](../bench/ticket-17.md), "Four things this run found".

**Found by:** the re-baseline run of
[ticket 17](./17-link-mimalloc-in-every-bench.md).

**Blocked by:** None.

**Status:** done

- [x] Neither case claims to flip. `InlineStyleToCssString` now holds
      `converted_keys` and `borrowed_keys`, and flipping is priced where it
      really happens, by `GenerateCssRule/rtl_flippable`
- [x] The two rows separate one variable and the names say which: the same five
      declarations, spelled camel case in one set and kebab case in the other,
      so the only difference is whether `normalize_css_property_name` has to
      build a new string
- [x] The re-baseline record says the rows changed --
      [`../bench/ticket-57-58-59.md`](../bench/ticket-57-58-59.md)

## Comments

**Option 2, because option 1 has nowhere to go.**
`inline_style_to_css_string` takes no options and reads no direction, so no
call of it can flip. The entry point that does take the options is
`generate_css_rule`, and the bench file already prices it under
`GenerateCssRule/rtl_flippable`. Adding a second flipping case to the
inline-style group would have measured the same path twice under two names.

**So the right-to-left case did not move: it was already there, and it does
flip.** The checks added under
[ticket 59](./59-six-bench-files-assert-nothing.md) prove it. Each
`GenerateCssRule` case now states whether the key and the options make a
right-to-left rule, and only `rtl_flippable` states `true`:

| Case | Key | `rtl` rule |
| --- | --- | --- |
| `ltr_only` | `background-color` | none |
| `rtl_flippable` | `margin-inline-start` | **yes** |
| `hover_pseudo` | `text-decoration` | none |
| `media_at_rule` | `transform` | none |

That is the row a reader may quote as the price of right-to-left support.

**The old sets differed in more than one thing.** The left-to-right set was
five camel case keys, the right-to-left set five kebab case keys, and the two
sets also held different properties and different values. Matching them leaves
one difference, which is what makes the pair worth reading. Both sets now
serialize to the same text, and a check states it.

**Both rows are a new series** under new names, so nothing reads across the
change.
