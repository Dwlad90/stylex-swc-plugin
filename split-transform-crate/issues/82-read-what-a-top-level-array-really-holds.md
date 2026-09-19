# 82 — Read what a top-level array really holds

**What to fix:** a `stylex.create` call written anywhere inside a top-level
array is treated as program level, even when the array does not hold its
result. Three shapes compile their class names and inject none of their rules,
and one of the three is also built again on every render.

Measured against `@stylexjs/babel-plugin` 0.19.0, `runtimeInjection: true`:

| Source                                   | Here                                   | Upstream                                |
| ---------------------------------------- | -------------------------------------- | --------------------------------------- |
| `[wrap(stylex.create({ ... }))]`         | object inline, no `_inject2`           | object inline, `_inject2` before it     |
| `[true ? stylex.create({ ... }) : null]` | object inline, no `_inject2`           | object inline, `_inject2` before it     |
| `[() => stylex.create({ ... })]`         | object inside the arrow, no `_inject2` | hoisted to `_styles`, `_inject2` before |

Each ships an element carrying a class name that no stylesheet defines. The
third also rebuilds the object on every call of the arrow, which is what
hoisting exists to stop.

**Where it starts.** `holds_call_in_top_level_array` in
`crates/stylex-state/src/state_manager.rs` answers by span containment, and
`is_bound_create_expr` in `crates/stylex-transform/src/shared/utils/validators.rs`
answers the validator's half the same way. Containment is O(1) on purpose: a
single top-level array holding every style in a module is idiomatic, and
walking its elements once per call would be quadratic in the styles it holds.
The cost of that choice is that containment cannot tell an element from
anything nested inside one.

So a call inside an arrow, a conditional or another call reads as program
level, and then two things follow. It is not hoisted, because hoisting is for
calls that are not program level. And the walk that places the injection looks
through arrays, objects and parentheses only, so it never reaches the object
and the rules are dropped.

[79](./79-inject-the-rules-an-object-in-an-array-declares.md) closed the object
shape by widening the walk. These three cannot be closed the same way: the
arrow shape needs the call to be hoisted, which is a decision made before the
walk runs.

**What to settle:** how to ask "is this call an element of the array" without
paying the walk that containment was chosen to avoid. The elements of an array
are in source order, so their spans are sorted, and the element holding a span
can be found by binary search rather than by a scan -- `O(log k)` per call
instead of `O(k)`. Descending into a found element that is itself an array or
an object keeps the same bound per level.

**Blocked by:** None.

**Status:** in-review

- [x] A call a top-level array holds only through an arrow, a conditional or
      another call is not read as program level. Answered by
      [80](./80-decide-what-binds-a-create-call.md), which replaced span
      containment with the position the discovery pass records. The arrow shape
      is hoisted, and the other two keep their place, which is what upstream
      prints for each.
- [x] Each of the three shapes above agrees with upstream. So do eighteen more,
      in the table under `## Answer`.
- [x] The question stays cheaper than a scan of the array's elements. It is
      cheaper than before: the walk that places the injection costs 63% less on
      the shape that pays the most.

## What else landed under this ticket

Recorded rather than filed, because each grew out of the question this ticket
asks and none of them is worth a ticket of its own:

- **ADR 0003**, `the-injection-queue-is-deduped-by-a-pair-of-hashes`. Reading
  what an array really holds is what forced the queue to be keyed by a pair, so
  the decision belongs beside the answer below.
- **The `with_nested_rules` seam across six producers.** Widening the walk meant
  each producer had to say which rules it carries, and writing that out six
  times is what the seam replaced.
- **The probe's `--inject` flag.** The shapes in the table below differ only in
  where the `_inject2(...)` statements are placed, and nothing could print that
  before.

## Comments

Found by the correctness review of
[79](./79-inject-the-rules-an-object-in-an-array-declares.md), read off the
code, and confirmed by running all three through both compilers. Pre-existing:
none of the three is a regression from that work.

## Answer

The position question was already answered by
[80](./80-decide-what-binds-a-create-call.md). What was left was the other half
of this ticket: the rules were compiled and injected nowhere. That was not
about the array either.

### What the defect really was

`flush_pending_insertions` places each `_inject2(...)` statement in front of
the statement that holds the styles, and it found that statement by reading
arrays, objects and parentheses only. A statement can hold the object anywhere,
so every other spelling dropped its rules without a word. Ticket 80 widened
the validator to upstream's rule and opened seven such spellings at once; the
walk was never widened with it.

Two changes:

- `RegisteredObjects` in `crates/stylex-state/src/state_manager.rs` replaces
  the four `push_*_hashes` functions. It reads every expression the item holds,
  stops at the object it matched, and does not enter a function body.
- `visit_mut_export_default_expr` reads the children of the export. It read
  only the whole expression, so `export default wrap(stylex.create({ … }))`
  printed a call to an API that is not there at runtime.

### What was measured

`pnpm run parity:probe '<sources>' probe.js --inject` from
`crates/stylex-rs-compiler` against `@stylexjs/babel-plugin` 0.19.0. The
`--inject` flag is new: the probe could not turn runtime injection on, so it
could not show *where* either compiler places the rules a call declares, which
is the whole subject of this ticket. "Before" is this ticket's starting point,
after 80 landed.

| Source                                   | Before                     | Upstream, and now       |
| ---------------------------------------- | -------------------------- | ----------------------- |
| `[wrap(stylex.create({ … }))]`            | no `_inject2`              | `_inject2` before it    |
| `[true ? stylex.create({ … }) : null]`    | no `_inject2`              | `_inject2` before it    |
| `[() => stylex.create({ … })]`            | hoisted, `_inject2` placed | the same                |
| `Object.freeze(stylex.create({ … }))`     | no `_inject2`              | `_inject2` before it    |
| `true ? stylex.create({ … }) : null`      | no `_inject2`              | `_inject2` before it    |
| `(0, stylex.create({ … }))`               | no `_inject2`              | `_inject2` before it    |
| `stylex.create({ … }).a`                  | no `_inject2`              | `_inject2` before it    |
| `stylex.create({ … }) \|\| null`          | no `_inject2`              | `_inject2` before it    |
| `{ s: stylex.create({ … }) }`             | no `_inject2`              | `_inject2` before it    |
| `s = stylex.create({ … })`                | no `_inject2`              | `_inject2` before it    |
| `class C { p = stylex.create({ … }) }`    | no `_inject2`              | `_inject2` before it    |
| `const C = class { p = create({ … }) }`   | no `_inject2`              | `_inject2` before it    |
| `export default wrap(create({ … }))`      | **not compiled at all**    | compiled and injected   |
| `export default [create({ … })]`          | **not compiled at all**    | compiled and injected   |
| `{ k: [wrap({ inner: create({ … }) })] }` | no `_inject2`              | `_inject2` before it    |

Nine more shapes were measured to hold what already worked: a name-bound
`create`, one inside a function, two in one array, a dynamic style, an inline
`keyframes` inside a `create` argument, and a `defineVars` in a module that
cannot name it. All twenty-one agree with upstream, printer style aside.

### What it costs

The walk reads more of the module and is cheaper, because it stops at the
object it matched. Criterion medians, `InjectionWalk` in `stylex-state`,
`aarch64-apple-darwin`, the old code re-measured first on the same machine:

| `create` calls in an array | before   | after    |
| -------------------------- | -------- | -------- |
| 128                        | 123.9 us | 43.5 us  |
| 512                        | 503.6 us | 168.0 us |
| 2048                       | 2.011 ms | 706 us   |
| 21,733 (`lotsOfStyles.js`) | 22.29 ms | 8.31 ms  |

63% less at every size. The object legs are unchanged within what this machine
resolves (0.3% to 3.5%), and the injection-off legs do not reach the walk.

One shape costs more, and it is now measured rather than argued: an authored
object that is *not* the styles is read level by level, where it used to be
hashed once. `array-with-data` is the new leg for it -- eight levels, 200 names
each, so every level takes the copying hash arm -- and it costs about 0.45 ms
more whatever the styles beside it are: 252 us to 710 us at 128 calls, and
2.232 ms to 1.381 ms at 2,048, where the saving on the styles has overtaken it.
Kept rather than guarded, for the reasons in the ADR.

Three narrowings that
`crates/stylex-state/docs/adr/0002-the-injection-walk-over-a-large-array-is-measured-and-kept.md`
had rejected are taken with the widening, and that record says why the trade
changed, what pins the rule they rest on, and what the widening costs.

### What the reviews found, and what was done

Two reviews read the change, one for correctness and standards and one for
performance. Neither found a shape the walk misplaces. Eight findings were
taken:

- A glossary entry in `crates/stylex-state/CONTEXT.md` said the opposite of the
  code. **Registered object** replaces **Held candidate**.
- Three load-bearing comments claimed more than the code did: that the map
  always empties, that hoisting is why a function body holds nothing, and that
  the object that matched is read once however often it is held. All three now
  say what is true, and the invariant the widened walk rests on -- that a
  written string cannot answer for a built one, because the hash carries the
  written form -- is back beside the arm that needs it.
- A namespace was read for nothing. It is skipped like a function body, which
  is what the sibling walk in `state_writers.rs` already answered.
- The queueing path held an O(rules squared) scan over a copy of every
  injecting statement, and queued one key per hoisted call that nothing could
  match -- which is what kept the walk from ever knowing it was done. Both are
  closed, measured by a new `InjectionQueue` group, and recorded as
  `crates/stylex-state/docs/adr/0003-the-injection-queue-is-deduped-by-a-pair-of-hashes.md`:
  68% less at 128 rules per call, +1.6% at the corpus average of eight.
- The evidence in this ticket was not reproducible from the repo, because the
  probe could not turn injection on. It can now.
- Tests were missing for the shape production really hands over (a hoisted
  reference with the object as the fallback), for two statements holding the
  same object, for a 500-deep wrapper chain, for an object wide enough to take
  the copying hash arm, for a class expression, and for the consumer calls a
  default export holds. All are in.

One divergence from upstream is recorded rather than followed: two statements
holding the same compiled object take one injection here and one each upstream,
because upstream places the rules of a call against that call and this compiler
places them against the shape the call left. The stylesheet is the same either
way.

### The third review, and the defect it found

A review of the work above found one defect in it and one older defect beside
it, both about the rules a nested producer call declares. A `keyframes` written
inside another producer's argument is folded to its name where it stands and
registers nothing of its own, so the producer that holds it must carry the rule
it left.

- `createTheme` carried nothing, so a theme named a `@keyframes` block that no
  stylesheet defined. `viewTransitionClass` carried it behind its own rule
  rather than in front, which is the other order from upstream.
- The rules were read with a clone off a file-scoped map that nothing cleared,
  so **every later producer of the module carried them again**. One module with
  two themes wrote the same block twice, the second time in front of a
  declaration that named no animation. That was true of `create` before this
  ticket as well: a module with a nested `keyframes` and two more `create`
  calls wrote the block three times where upstream writes it once.

`with_nested_rules` is the one way in for all six producers that can hold such
a rule. It puts the nested rules in front of the producer's own, and it *takes*
them off the state, because they belong to the one call that folded them. The
six are the six upstream declares a local `otherInjectedCSSRules` in;
`keyframes`, `positionTry`, `defineConsts` and `defineConstsNested` hold none,
and both compilers refuse a producer call inside a `defineConsts` argument.

Measured with `--inject` against `@stylexjs/babel-plugin` 0.19.0, counting the
`@keyframes` blocks and the `_inject2` calls of the printed module:

| Source                                              | Before   | Upstream, and now |
| --------------------------------------------------- | -------- | ----------------- |
| a `keyframes` in a `createTheme` value               | 0 blocks | 1 block, first    |
| a `keyframes` in a `viewTransitionClass` step        | behind   | in front          |
| two themes, the first holding a `keyframes`          | 2 blocks | 1 block           |
| a `create` with a `keyframes`, then another `create` | 2 blocks | 1 block           |

Four more findings were taken: the warning that says a producer registered a
shape the walk cannot find reads its kind through the parentheses and is two
short sentences rather than one long one; the two nested producers that no
snapshot held are pinned; and the two naming nits.

### Landed

`b16351b45` -- the walk, the queueing, both ADRs, the benchmark legs and the
glossary entry.
`84a538ac9` -- the default export, the shape tests against upstream, and the
probe's `--inject` flag.
`f305c209d` -- the rules a nested call declares, carried once and in front, by
the one seam all six producers now read them through.

The whole workspace suite, `cargo clippy --workspace --all-features
--all-targets`, `cargo fmt`, `pnpm test`, `pnpm typecheck`, `pnpm lint:check`,
`pnpm lint:type-aware`, `pnpm format:check` and
`pnpm run test:coverage:workspace` are green, the last at 100.00% of regions,
functions and lines.
