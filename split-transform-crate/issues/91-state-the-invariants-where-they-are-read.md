# 91 — State the invariants where they are read

**What to fix:** two functions rested on a guarantee their callers held, and
spelled the guarantee as a default or an index that no call reaches.

**`variable_fallbacks`.** The caller reached it only for a set holding a
`var()` entry, so `first_var` and `last_var` were always `Some` and three
`unwrap_or` defaults could never be taken. `guidelines/stack/RUST.md` names the
shape: a substitution such as `unwrap_or` "charges the region to
`library/core`". It also left `&var_values[0]` -- introduced by this branch's
`compose_vars` reshape -- with no local proof it cannot panic.

Answered by moving the position into the signature. The caller already had to
find out whether there was a `var()` entry; asking `position` instead of `any`
answers the same question and yields the entry, and the two arms it reads are
the two it read before. Inside, the search back is bounded by that position, so
its default is `0` -- the entry itself -- which is the correct answer and not a
fiction. `rposition` also replaces the `.rev().position().map(|i| len - 1 - i)`
arithmetic with the operation it spells.

**`gen_bitwise_or_of_conditions`.** Its rustdoc stated the invariant -- "called
only where there is at least one condition" -- and then indexed `conditions[0]`.
The proof was at the caller.

Answered by splitting where the emptiness is already decided. The caller's
`if conditions.is_empty()` early return became a `let ... else` over
`split_first`, and the join takes `(first, rest)`. The same two arms, and no
index: the signature is now what guarantees there is a first condition.

Neither site could panic before. This is region accounting and readability,
and it adds no arm the gate cannot reach.

**Blocked by:** None.

**Status:** resolved

- [x] No `unwrap_or` default that no call reaches
- [x] No index whose proof lives in another function
- [x] The workspace coverage gate still reports zero uncovered regions
