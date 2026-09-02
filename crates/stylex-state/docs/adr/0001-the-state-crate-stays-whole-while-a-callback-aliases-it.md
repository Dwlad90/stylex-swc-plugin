# The state crate stays whole while a callback aliases it

**Status:** accepted

`stylex-state` is one crate holding the state manager and the seven value types
it composes. It must not be split further until
[`types::EvaluationCallback`](../../src/types.rs) stops taking
`&mut StateManager`.

The reason to say this out loud is that the crate reads like a split waiting to
happen. It has seven modules, one concern each by name -- an evaluated value, a
function config, a theme reference, a seen value, the writers -- and a reader
who counts modules concludes that the state manager could keep `state_manager`
and the value vocabulary could go below it. The module count is not what holds
the crate together.

What holds it together is one type alias:

```rust
pub type EvaluationCallback =
  Rc<dyn Fn(Vec<EvaluateResultValue>, &mut StateManager) -> Option<Expr> + 'static>;
```

An author's own arrow is stored **as a value** --
`EvaluateResultValue::Callback` holds one -- and applying it needs the whole
state manager, mutably, because the
body it stands for can inject a style, memoize a module or record a refusal. So
a value names the state manager, and the state manager holds values. Put the two
in separate crates and each `[dependencies]` table names the other, which Cargo
rejects.

The other edges in the knot are shared reads and would survive a split on their
own: `ThemeRef::of` and `ThemeRef::get` take `&StateManager`, and a function
config carries a `ThemeRef`. It is the mutable alias that cannot be pointed
across a crate boundary, because it is the one edge that runs _upward_ -- a
value asking for the thing that owns it.

## What would unlock a split

The callback taking, instead of the state manager, the narrow set of writes an
applied arrow actually performs. That set is a trait the crate below could own,
the way `stylex-diagnostics` already takes the reads a code frame needs as
a trait the state manager implements. Once the alias names a trait rather than
the struct, the value vocabulary can sit under the state manager and the split
becomes an ordinary move.

Until then, a split proposal has to answer the alias first. Reviewing the module
list will not find the problem: every module is a plausible crate, and the
compile error arrives only after the files have moved.

## Considered options

**Split the value types out now and re-export them.** Rejected: the cycle is in
the types, not in the module paths, so a re-export moves the compile error
without removing it -- and a facade is against
[the re-export rule](../../../../guidelines/stack/RUST.md#re-exports).

**Box the state manager behind a trait object in the alias.** Rejected as the
first step. It would break the cycle, but it puts a virtual call on the path an
applied arrow takes per call, and no measurement says what that costs. The
narrow-write trait above is the same fix, with a bound the compiler can see
through.

**Say nothing and let the next reader find out.** Rejected: that is what this
record exists to prevent. The knot is invisible in the module list and shows up
only as a Cargo error after a day of moving files.

## Consequences

- A split proposal for this crate is reviewed against the alias first. The
  module list will not show the problem: every module is a plausible crate, and
  the compile error arrives only after the files have moved.
- The crate keeps its temporary coverage exclusion until its own ticket closes
  it, and that exclusion is not an argument for splitting the crate to make the
  coverage smaller.
- `EvaluationCallback` is the one signature in the crate that must not be copied
  as a pattern. A new value type that needs to write to the state should take
  the writes it performs, not the state manager.
