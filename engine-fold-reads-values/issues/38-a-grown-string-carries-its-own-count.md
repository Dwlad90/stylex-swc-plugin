# 38 — A grown string carries its own count

**What to build:** A string that has already been measured is not measured again
when it is appended to, so a chain of `+` costs what its length costs rather than
the square of it.

**Where the square comes from.** `binary_expr_to_string` seeds a fresh
`GrownString` at every level and pushes both operands into it — including the
already-accumulated left string, which goes through `units_within` and
`utf16_length`, a scalar-by-scalar `encode_utf16().count()` over the whole text.
That count was known one level down and thrown away when the result was boxed
back into `BinaryExprType::String(String)`. For `a + b + c + …` the left text is
re-encoded once per level: O(n) extra work per level, O(n²) over the chain.

**The merge base did no counting here at all.** The ceiling that requires it is
right — a string grown by `+` is the same string and the same memory as one a
fold builds, and bounding only the fold left the cheapest way to build a huge one
unbounded, which is what tickets 20, 23 and 25 established. What is wrong is
paying for the bound by re-reading what was already read.

**This is the non-fold path.** Every `+` and every template interpolation in
every file pays it, whether or not anything folds — which is why 36 comes first.

The fix is to carry the count with the text rather than recomputing it: a length
alongside the `String` in `BinaryExprType`, and a constructor on `GrownString`
that adopts an already-measured buffer by move and measures only the operand
being added. An `is_ascii` fast path in `units_within` is worth having on its own
— ASCII is SIMD-scanned and covers essentially every CSS value — but it narrows
the constant rather than removing the square, so it is not the fix.

**Blocked by:** 36.

**Status:** resolved

- [x] A measured string carries its unit count, and appending adopts the left
      buffer rather than re-reading it
- [x] A long `+` chain is measured before and after, against the fixture from 36
- [x] Every ceiling refusal the chain can produce still fires at the same length
      and names the same option — the count being carried must be the count that
      was checked
- [x] Template interpolation reaches the same path, since it grows the same string
- [x] A lone surrogate in an operand still counts as it does today

## Answer

**`BinaryExprType::String` became a struct variant carrying `text` and
`units`,** and `GrownString::adopt` takes an already-measured buffer by move.
`into_measured` hands the pair back. Nothing new was added to `stylex-enums` —
the count sits inside the variant that already lived there, because that crate
holds closed sets and a measured buffer is not one.

**The count only survives if the level above folds the level below itself.** A
folded `+` leaves the evaluator as a plain string literal, and a literal has
nowhere to carry a length — so adopting is unreachable while the left operand
arrives through `evaluate_cached`. `binary_expr_to_num_or_str` now folds a left
operand that is itself a `+` through `binary_expression`'s own path, and the
`+` it claims is concatenated in place instead of being handed back to
`binary_expr_to_string`, which used to evaluate both operands a second time.

**The ceiling and the memo are the same ones, which is what `folded_once` is
for.** `evaluate_cached`'s body split into a depth charge and a memoized fold,
and both callers go through them — so a chain refuses at the link it always
refused at, and a subtree written twice still answers from the first reading.
The `Memoized` trait says how a caller's value crosses into the memo and back:
a measured string is remembered as the literal it spells and is measured again
when it comes back, because the tree cannot hold a count. The two ADRs the memo
rests on are untouched.

**The fallback does not descend, and that mattered.** `binary_expr_to_string`
is only reached for a `+` the number-or-string path refused, where the left is
never a folded string — so a descent there could only pay for a measurement
nothing carries. Descending there anyway cost **2.6 %** on 36's fixture
(1.026, CI 1.008–1.043). Reading both operands plainly there brings it to
**1.001 / 0.993** — flat. The whole benchmark suite passes at parity, `engine
fold` included, measured paired against a build of `d01d924d1`.

**The chain itself.** `benches/concatenation_chain_bench.rs` joins the same
900,000 characters through 2, 5, 10 and 20 links, so what varies is only how
many times the accumulated text could be read again:

| links | re-reading | adopting |
| --- | --- | --- |
| 2 | 0.73 ms | 0.72 ms |
| 5 | 2.33 ms | 0.99 ms |
| 10 | 4.55 ms | 1.33 ms |
| 20 | 9.17 ms | 2.00 ms |

**The line still climbs, and is meant to.** What is left is the memo's own key
— a hash of the whole remaining subtree, taken again at every link — and the
one copy per link that boxes a folded chain back into the tree for the memo to
hold. Both are ADR 0005 rather than anything this ticket can remove.

**The `is_ascii` shortcut went into `utf16_length` rather than
`units_within`,** which is the function `units_within` counts through, so every
reading of a length in the compiler gets it rather than this one. It is the
narrowing the ticket calls "worth having on its own"; it is not the fix, and
the table above was measured with it in place on both sides.

**A lone surrogate never reaches the buffer.** There is no `&str` holding one,
so the operand is refused a step earlier, by the reading of the operand rather
than by the ceiling — unchanged, and pinned in
`measured_concatenation_chains.rs` under a raised ceiling so the refusal cannot
be mistaken for the ceiling's. Upstream folds it to a replacement character;
that divergence is older than this ticket.

**Where the coverage is.** `measured_concatenation_chains.rs` — 20 chain shapes
against measured `@stylexjs/babel-plugin` 0.19.0 output, the ceiling walked
link by link in both directions, code units across a link, and templates meeting
chains from both sides. `helpers_tests.rs` — adopt keeps the count, a grown
buffer answers with the count it measured, and an adopted count agrees with a
fresh reading. `binary_expression_tests.rs` — every string case now asserts the
carried count against an independent reading of the text it labels.

**Glossary.** **Measured string** in `crates/stylex-transform/CONTEXT.md`.
