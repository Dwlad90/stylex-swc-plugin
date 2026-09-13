# 57 — Two benches measure nothing

**What to build:** Two benches time an empty value, not the work their names
claim. Make them measure something, or delete them.

Both are in `crates/stylex-css-parser/benches/properties_bench.rs`, and both
start from `Transform::new(vec![])`:

```rust
group.bench_function("transform_create", |b| {
  b.iter(|| black_box(Transform::new(vec![])))
});
```

`Transform` holds one field, `value: Vec<TransformFunction>`, and `new` moves
its argument into it. An empty `Vec` allocates nothing, so nothing is built,
nothing is parsed and nothing is asked of the allocator.

`PropertyDisplay/transform_display` then calls `to_string()` on that same empty
value. Every step of the `Display` impl is empty: the map over an empty `Vec`
allocates nothing, the collect returns an empty `Vec<String>`, `join(" ")` over
zero elements returns an empty `String`, and the write writes nothing.

| Benchmark | Reads | Sibling that builds a real value |
| --- | ---: | --- |
| `PropertyCreation/transform_create` | **801.74 ps** | 13.9 ns, 24.8 ns |
| `PropertyDisplay/transform_display` | **7.32 ns** | 170.7 ns, 416.9 ns |

801.74 ps is about two cycles.

**Why it matters more after ticket 17.** Every bench now links `swc_malloc`,
so each one states that it measures the allocator the published addon runs.
These two make that statement and then never allocate, so the claim is false for
one case in each of two groups.

**It is also the failure `guidelines/PERFORMANCE.md` warns about.** "Assert what
the bench is measuring, in the bench", because "a curve that flattens because
the work stopped happening is indistinguishable from a win". A sub-nanosecond
figure cannot regress in any way a reader would see, so the row costs 8 seconds
of every re-baseline and reports nothing.

**Two ways to close each, and they are not equal.**

1. **Give them a transform to build.** A vector holding a few
   `TransformFunction` values makes `new` allocate and makes `to_string()`
   write, which is what the other cases in both groups do. This keeps each
   group's three cases comparable.
2. **Delete the two cases.** Defensible if `Transform` is not on a path anyone
   cares about, but then say so in the file, because the next person will add
   them back.

Prefer the first. The groups are called `PropertyCreation` and
`PropertyDisplay`, and their other cases create and display a property; these
two should too.

**Keep it to these two.** They are the two fastest measurements in the
workspace, and the third-fastest is 13.9 ns, which is a real allocation. So this
is a finding about one type used in two benches, not a class.

**Where the numbers are:**
[`../bench/ticket-17.md`](../bench/ticket-17.md), "Four things this run
found".

**Found by:** the re-baseline run of
[ticket 17](./17-link-mimalloc-in-every-bench.md).

**Blocked by:** None.

**Status:** ready-for-agent

- [ ] `PropertyCreation/transform_create` measures work, or the case is gone
- [ ] `PropertyDisplay/transform_display` measures work, or the case is gone
- [ ] Neither reads faster than the slowest real case in its own group
- [ ] The re-baseline record says both rows changed, so the series is not read
      across the change
