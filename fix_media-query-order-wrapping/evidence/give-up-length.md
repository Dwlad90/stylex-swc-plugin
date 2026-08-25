# Ticket 03 — the ladder length at which the reference implementation gives up

## The answer

**There is no give-up length.** Within the reach of a breakpoint ladder, the
reference implementation never stops merging ranges and never emits the
authored queries unmerged. It degrades — super-exponentially — and then the
process dies. The recovery its `try`/`catch` provides is not reachable this
way.

That contradicts the premise ticket 03 was written on, and it changes what
ticket 08 has to do. The correction is stated at the end.

## The ladder used

Exclusive `min-width`/`max-width` rungs from widest to narrowest, the last one
`max-width` only — the shape reported in the issue, generalized to `n` rungs.
For `n = 6`:

```json
{
  "default": "black",
  "@media (min-width: 1000px)": "c0",
  "@media (min-width: 950px) and (max-width: 999px)": "c1",
  "@media (min-width: 900px) and (max-width: 949px)": "c2",
  "@media (min-width: 850px) and (max-width: 899px)": "c3",
  "@media (min-width: 800px) and (max-width: 849px)": "c4",
  "@media (max-width: 799px)": "c5"
}
```

Widths are spaced so no two rungs touch, which is what makes every distributed
branch a contradiction and therefore what maximizes the `not all` expansion.
The measurement is the query text the *first* rung compiles to, because it
carries the whole accumulated negation chain.

## The measured curve

Wall clock and the length of the first rung's emitted query, per ladder
length. Apple M1 Max, 64 GB, Node v24.11.0, `@stylexjs/babel-plugin` 0.19.0,
warm module cache, one process per length.

| Rungs | Wall clock | First rung's query |
| ----- | ---------- | ------------------ |
| 2     | 29 ms      | 26 chars           |
| 3     | 31 ms      | 26 chars           |
| 4     | 39 ms      | 26 chars           |
| 5     | 41 ms      | 41 chars           |
| 6     | 46 ms      | 71 chars           |
| 8     | 62 ms      | 251 chars          |
| 10    | 74 ms      | 971 chars          |
| 12    | 93 ms      | 3 851 chars        |
| 14    | 122 ms     | 15 371 chars       |
| 16    | 184 ms     | 61 451 chars       |
| 18    | 387 ms     | 245 771 chars      |
| 20    | 1.21 s     | 983 051 chars      |
| 21    | 2.37 s     | 1 966 091 chars    |
| 22    | 4.75 s     | 3 932 171 chars    |
| 23    | 9.67 s     | 7 864 331 chars    |
| 24    | 20.4 s     | 15 728 651 chars   |
| 26    | 88.3 s     | 62 914 571 chars   |
| 28    | 435 s      | 251 658 251 chars  |

Every one of these merged. The query text doubles per rung and the wall clock
doubles with it: at 26 rungs one media query is 63 MB of CSS, at 28 rungs it is
252 MB and takes seven and a quarter minutes. A 30-rung run was abandoned after
projecting past thirty minutes and thirty gigabytes; the failure mode is
measured directly below instead, which does not need it.

The output either side of the first rung that expands at all:

- **4 rungs** — `@media (min-width: 1000px)`. Contradictory branches are
  produced here too, but only one survives, and serialization unwraps a lone
  survivor back to the bare query.
- **5 rungs** — `@media (not all) or ((min-width: 1000px))`. The first length
  at which a retained contradiction is visible in the output.

## Where it fails, and how

Not by giving up, and **not** by exhausting the call stack. The recursion peels
one `not (A and B)` clause per level, so its depth is linear in ladder length
while the number of branches is `2^n`. A stack overflow would need something on
the order of ten thousand rungs, and the output at ten thousand rungs is not a
number of bytes that exists. Two other limits bind long before it, and which of
them arrives first is **not established here**:

- **The string-length limit.** V8's `MAX_STRING_LENGTH` on this machine is
  536 870 888 chars. The measured curve quadruples per two rungs, so a 30-rung
  first-rung query lands somewhere near twice that — an *extrapolation from* the
  28-rung measurement, not a measurement. Exceeding it raises
  `RangeError: Invalid string length` while the query text is being built.
- **The heap.** `2^n` query-tree nodes; the 28-rung run reached about 7.4 GB
  resident before completing.

Either way the recovery is unreachable, which is what this ticket needs, and
each route reaches that conclusion differently:

- A heap exhaustion is a fatal abort rather than a thrown value, so no `catch`
  can see it. Measured rather than argued — the same 28-rung ladder under
  `--max-old-space-size=2048`:

  ```text
  FATAL ERROR: Ineffective mark-compacts near heap limit
  Allocation failed - JavaScript heap out of memory
  ----- Native stack trace -----
  ```

  No JavaScript frame ever runs again.

- A string-length `RangeError` *is* catchable, but it is not raised anywhere the
  recovery can catch it. `mergeAndSimplifyRanges` wraps only the merge call
  (`lib/index.js:3309`), while the text is built by `combinedQuery.toString()`
  at `lib/index.js:3536` — outside that `try`. So it propagates as a build
  failure instead of returning the input rules.

There is therefore no ladder length at which the reference implementation
returns its input rules unmerged: it merges, or the compile dies. Settling
*which* limit binds first would take a 30-rung run, projected at roughly half an
hour and thirty gigabytes, and it is not run here because no conclusion in this
document turns on the answer.

For contrast, the same ladders through this compiler today, which still carries
the shortcut ticket 05 deletes — 6 rungs 2 ms, 24 rungs 4 ms, 60 rungs 32 ms,
100 rungs 135 ms, every one emitting the authored queries unchanged. That is
the fast path being measured, not the expansion; ticket 08 records the cost
once the expansion is restored.

## What this means for ticket 08

Ticket 08 was written to place a Rust recursion bound at or above "the length
at which the reference implementation gives up". That length does not exist,
so the instruction cannot be followed literally, and the two options it framed
— degrade or fail — resolve to *fail*.

Three consequences, for ticket 08 to decide on rather than for this ticket to
settle:

1. A bound placed anywhere finite is *above* the reference implementation's
   own behaviour in the only sense that matters: past it we emit the queries
   unmerged where the reference implementation would still be merging. Byte
   parity past the bound is unattainable, which the spec already accepts —
   but the bound is now the point where we deliberately stop matching, not
   the point where we match its recovery.
2. A depth bound alone does not protect this compiler. The cost is `2^n`
   branches at depth `n`, so a bound that permits 26 rungs permits a 63 MB
   query. Whatever number ticket 08 picks has to be justified against output
   size, not against stack depth.
3. Failing loudly is not on the table for the bound: the spec is explicit that
   exceeding it returns the input rules and does not raise the
   invalid-media-query-syntax refusal.

## Where the numbers came from

- `@stylexjs/babel-plugin` **0.19.0**, resolved from
  `node_modules/.pnpm/@stylexjs+babel-plugin@0.19.0_supports-color@8.1.1/node_modules/@stylexjs/babel-plugin/lib/index.js`
- the version is held by `pnpm-lock.yaml`, not by an exact range in the
  dependency catalog
- `@babel/core` 8.0.1
- reproduced by `give-up-length.cjs` in this directory, one process per length
- nothing in the repository was modified to take these measurements
