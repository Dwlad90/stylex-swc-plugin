# 18 — A length set by the receiver rather than by an argument

**What to build:** The allocation bound reaches the calls whose result length
comes from their *receiver*, not from an argument of their own.

```js
content: Array(100000000).fill(0).length
```

Measured on this compiler: 34 seconds and 421 MB of resident memory, and *then*
a refusal — the outward entry ceiling catches the array once it exists. Nothing
is wrong with that refusal; what is wrong is everything before it. A build that
takes half a minute per mistyped declaration is the failure the ceilings were
put in to prevent, arriving late instead of not at all.

`LENGTH_AMPLIFYING_METHODS` is `repeat`, `padStart` and `padEnd` — the three
whose result length is set by an argument, so a guard reading arguments can
bound them. `fill` is the same shape read from the other end: its result length
is its receiver's, and `Array(n)` is what sets that from an argument, one call
earlier. The two links are each innocent — `Array(n)` is sparse and costs
nothing, `fill` is bounded by whatever it was handed — which is exactly the
per-link reasoning ticket 12 replaced with a product for `repeat`.

So this is 12's rule reaching one link further, not a new kind of bound: the
guard already knows how to work out a length and compare it to
`maxFoldedCharacters`/`maxFoldedEntries`. What it does not yet do is read a
length off a receiver whose own length an argument set.

Whether other names belong beside `fill` — `copyWithin`, and a `flat` whose
depth argument multiplies — is part of the work rather than settled here. The
answer should come from what each one's result length is a function of, since
that is the question the existing three are on the list for.

**Found while building ticket 12**, by asking whether an input could still fold
and allocate past the ceiling. Not a regression: `Array` became foldable in
ticket 09 and `fill` in 06, and 12 changed neither. Left out of 12 deliberately
— its checklist is about the two ceilings becoming configuration and about the
count a written argument used to have to be, and widening the method list is a
separate claim about which calls amplify.

**A second residual, and a different subsystem.** `const b = a + a; const c = b
+ b;` doubles a string per line in the *evaluator*, which reaches
`bound_value_too_large` only when the value crosses into a fold — so the growth
that matters happens before the ceilings can see it. Recorded here because it is
the same question (an allocation the fold's ceilings do not reach) but it is not
the same fix: nothing in that chain is a call the fold guard is asked about.
Whoever takes this should decide whether the two belong in one ticket or two.

**Blocked by:** none — 12 landed the arithmetic this builds on.

**Status:** resolved

- [x] `Array(100000000).fill(0)` refuses before it allocates, not after
- [x] The rule is the product ticket 12 already computes, extended to a length
      the receiver carries, rather than a second bound of its own — **amended,
      see the Comments: there is no product, and the receiver was the wrong end**
- [x] Which names join `fill` is decided by what each one's result length is a
      function of, and written down
- [x] A refusal that arrives late is told from one that arrives on time, by a
      case that measures the wall clock rather than only the sentence
- [x] The corpus records whichever of these upstream folds and this compiler
      refuses

## Answer

**`fill` is not on the list, and neither is any other method.** The criterion
this ticket set — what is each one's result length a function of? — answers
`fill` with *its receiver's own length*, and a receiver's length is not something
`fill` adds to. Applied to every candidate the sweep reaches:

- `fill`, `copyWithin`, `reverse` and `sort` answer their receiver's length.
- `slice`, `splice`, `concat` and `flat` answer a length no larger than the
  elements their receiver and their arguments already hold.
- `map`, `filter` and `join` answer one element, or one element's text, per
  element of a receiver.

Every one of those is a length something already paid for. So the question is
what did *not* pay, and there is exactly one answer: **`Array(n)` declares a
length and allocates nothing for it.** The array is sparse. The declaration is
free, the cost lands on whichever call in the chain first touches it, and the
entry ceiling read the length only where the array crossed *back* — so a call
that never returned an array was never measured at all.

That is why the list this ticket expected to widen did not need widening. The
bound moved one link earlier instead, onto the declaration, and the whole chain
came with it. Measured on this branch, before and after, at the shipped default:

| input                                | before   | after   | upstream    |
| ------------------------------------ | -------- | ------- | ----------- |
| `Array(1e8).fill(0).length`          | 42.8 s ✗ | 0.01 s ✗| ✗ RangeError|
| `Array(1e8).copyWithin(0,1).length`  | 43.8 s ✗ | 0.00 s ✗| folds       |
| `Array(1e8).map(x => x).length`      | 42.1 s ✗ | 0.00 s ✗| folds       |
| `Array(1e8).toReversed().length`     | 31.0 s ✗ | 0.00 s ✗| folds       |
| `Array.from({length: 1e8}).length`   | 25.9 s ✗ | 0.00 s ✗| ✗ RangeError|
| `Array(1e8).join(',').length`        | 21.2 s ✗ | 0.00 s ✗| folds       |
| `Array(1e8).includes(1)`             | 13.4 s **✓**| 0.00 s ✗| folds    |
| `Array(1e8).flat().length`           | 12.4 s **✓**| 0.00 s ✗| folds    |
| `Array(1e8).sort().length`           | 12.8 s ✗ | 0.00 s ✗| folds       |
| `Array(1e8).slice(0).length`         | 12.3 s ✗ | 0.00 s ✗| folds       |

The two rows in bold are the ones that mattered most: they *folded*, in thirteen
seconds each, with no ceiling having measured anything. A refusal arriving late
is at least a refusal; a fold arriving late is a build that has silently accepted
an allocation nobody bounded.

### `Array.from` came with it, and nothing else did

`Array.from(x)` is the same declaration one property along: `{ length: n }` is an
object that declares a length without holding one, which is `Array(n)`'s trick
written as a property. It is bounded with it, and the argument is **resolved**
rather than read as syntax — so a name holding the object, `{ ...{ length: n } }`,
and `{ length: '100000000' }` are all the same declaration. That last one is not
pedantry: the language coerces an array-like's `length`, so a string really does
build a hundred million elements.

`Array(n)` is read the other way, strictly: `Array` declares a length only when
it is handed exactly one argument that *is* a number. `Array('3')` is one element
holding a string, and `Array('a', 'b')` is two elements the source wrote out.
That is the opposite reading from an amplifying count, where `'x'.repeat('3')`
really does repeat three times, and the two are documented against each other so
neither is copied onto the other by mistake.

### What the change costs, and where it is written down

Three rows in `corpus/modules.json`, each with a reason on it:

1. `modules-18-a-length-a-call-declares` — the headline shape, at a length
   `maxFoldedEntries` can reach. **Configured**: raise the option past the
   declared length and it folds to upstream's value.
2. `modules-18-a-declared-length-that-never-crosses` — `String(Array(10001))`.
   This is the capability traded away, and it is worth naming plainly: the entry
   ceiling used to bound only what came *back*, so an array built and joined
   inside the engine was bounded by the string it came to and not by itself.
   `Array(1e8).join(',')` is what that bought. **Configured** as well.
3. `modules-18-a-declared-length-inside-a-callback` — **not** configured, and
   deliberately so: no value of the option folds it. See below.

`a_count_at_the_bound_folds_and_one_past_it_refuses` and
`array_of_an_unmaterialisable_length_is_rejected` both changed their asserted
sentence, and both changes are this ticket rather than churn: the first asserted
that the bound was read on the way out, which is the claim being replaced, and
the second now names the length that has to change rather than reporting a size
after the fact.

### A count the language rejects keeps the language's own sentence

`Array(2.5)`, `Array(-1)`, `Array(NaN)`, `Array(Infinity)` and `Array(2 ** 32)`
declare no length as far as the bound is concerned, and fall through to the
`RangeError` the language raises before allocating anything. A ceiling in front
of those would replace an accurate sentence with a misleading one, and there is
nothing for it to save — the throw is already free. This is the one place the
guard deliberately declines to read a number it could read.

Review caught that this held for `Array(Infinity)` and not for
`Array.from({ length: Infinity })`, which was clamped to `2 ** 53 - 1` and refused
by the ceiling — naming a number the language never reaches, because
`ArrayCreate` refuses the length first. Both spellings now share one range check,
and the fix deleted rather than added: `MAX_SAFE_INTEGER` and the clamp that used
it are gone, because anything outside the language's array-length range is the
language's throw to report. Nothing under `2 ** 32` falls through, which is the
half that matters — `{ length: 4294967295 }` is a length the language accepts and
would really iterate to.

### The callback rule came too, and it is the one judgment call worth arguing

An amplifying string count is refused outright inside a callback, because a
callback body runs once per element of a receiver the guard never measured. A
declared length is refused there for the same reason, and it is not theoretical:
a receiver of ten thousand elements — allowed, and reachable, since the ceiling
is `>` and not `>=` — times a declaration of ten thousand is a hundred million
elements, every link inside the ceiling. `['a','b','c'].map(x =>
Array(9999).fill(x).length)` folded on both compilers before this.

It costs `Array(2).fill(x)` written inside a `map`, which upstream folds. That is
a real cost and it is the reason the check is asked *after* the length rather
than before it: a call that declares no length is `Array('a', 'b')`, whose
elements the source wrote out, and refusing that inside a callback would take
away a fold nothing threatens. What makes the remaining cost acceptable is that
the refused shape has a spelling that folds — `[x, x]` is the same array, and its
length is the source's. The message says so.

The sharper rule would be to carry the enclosing receiver's element count into
the callback's scope and bound the product, which is what the string rule would
want too. That needs `Scope` to hold a count it does not hold, so it is a change
to the guard's shape rather than to its arithmetic. Filed as **21**.

**And review found the rule as first written was not enough.** It refused a
declared length it could *read* and admitted one it could not, on the argument
that an unreadable argument is a value the ceilings already bounded. Inside a
callback that argument is a parameter, and the reasoning does not hold there:
`[{ length: 100000000 }].map(x => Array.from(x).length)` folded in **sixty-eight
seconds** — worse than the input this ticket was filed for — and
`[100000000].map(x => Array(x).fill(0).length)` is thirty-four seconds per
element. Both reach the declaration through a name nothing in front of the engine
can resolve.

So the read answers three things rather than two: a length, no length, or a length
it could not read. Outside a callback the third is still admitted, for the reason
first written. Inside one it is refused, and what is still admitted there is a
call the guard can *see* declares nothing — `Array(x, x)`, whose elements the
source wrote out. That costs `[[1],[2]].map(x => Array(x).length)`, which was a
row of `a_conversion_inside_a_callback_folds` and is now its own case saying why.

### The second residual is two tickets, not one

Filed as **20**. `const a1 = a0 + a0` doubles a string in the *evaluator*, and
`a10.length` folds to 102400000 in three seconds with nothing refusing it —
`maxFoldedCharacters` is a hundred times smaller than the value that came out.
The depth budget caps the chain near `2 ** 14`, so with a base at the character
ceiling the reachable value is about 1.6e10 code units. It is the same *question*
as this ticket — an allocation the ceilings do not reach — and a different fix:
nothing in that chain is a call the fold guard is asked about, so no bound in
front of the engine can see it. Putting it here would have widened this change
into a second subsystem for no shared code.

### What review changed

Worth recording, because two of the four were defects rather than polish:

1. **The unreadable length inside a callback** — a hole this change opened and
   did not close, sixty-eight seconds wide. Above.
2. **`Array.from({ length: Infinity })`** — an inconsistency with the rule this
   ticket had just written down. Above, and it made the code smaller.
3. The two callback messages were a copied sentence; they are one function taking
   the unit's noun, which is the move `cannot_bound` had already made next door.
4. `global == "Array" && method.sym == "from"` spelled the two names a second
   time, beside `EntryAmplifier::name`. One recogniser owns them now.

And one process failure worth its own line: a throwaway probe module was removed
with a shell one-liner that truncated `tests/.../mod.rs` to nothing, which
deregisters thirty-six test modules and leaves the suite green while running
almost none of it. The review agent found it, not the suite — which is the whole
point of the note already in memory about an unregistered `mod`.

### Gates

`cargo test/check/clippy --workspace --all-features` green; `cargo fmt`,
`pnpm test`, `pnpm typecheck`, `pnpm lint:check`, `pnpm lint:type-aware`,
`pnpm format:check` green. `parity` 0 changed / 0 unexpected across 1175
subjects, `parity:positions` 0 unexpected, `fuzz:prototypes` 0 unexpected over
332. Re-run after the review fixes, and `mod.rs` verified against `HEAD` so the
integration suite is known to have run. Not benchmarked: the guard adds one name comparison to the four applied
globals, and for `Array` one literal read or a memoised resolve the admitted call
pays a moment later anyway.

## Comments

**One thing this ticket asked for that turned out to be the wrong shape.** The
second checkbox expected "the product ticket 12 already computes, extended to a
length the receiver carries". There is no product here. `repeat` multiplies
because its result is its receiver repeated; a declared length is one number, and
the receiver-carried framing was the surface reading of a chain whose real
unmeasured link was one call earlier. The bound is 12's *arithmetic on values* in
the other unit, which is the part that generalised — reading a length rather than
matching a shape — and not its multiplication.
