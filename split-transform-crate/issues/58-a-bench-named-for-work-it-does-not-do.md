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

**Status:** ready-for-agent

- [ ] The right-to-left case flips, or neither case claims to
- [ ] The two rows separate one variable, and the name says which
- [ ] The re-baseline record says the rows changed
