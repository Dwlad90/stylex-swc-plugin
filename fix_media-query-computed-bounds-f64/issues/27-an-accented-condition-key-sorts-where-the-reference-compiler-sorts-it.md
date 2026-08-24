# 27 — An accented condition key sorts where the reference compiler sorts it

**What to build:** an author who nests a condition key carrying an accented
letter beside another key gets the same class name from both compilers.

Today they do not. Root collation places an accented letter beside its base
letter, so `[data-état]` sorts between `[data-e]` and `[data-f]` upstream; here
every byte outside printable ASCII ranks above the entire table, so it sorts
after both. The sorted key path is hashed into the class name, so the two
compilers name different classes for the same source — which is the
mixed-toolchain hazard the rest of this effort has been closing everywhere else.

This is the last such divergence the harness reports. Everything else it prints
is either agreement, a refusal both compilers make, or a divergence recorded
with the argument for why agreement is not wanted.

Carry out whatever ticket 26 decided, including the part about what stays
uncovered: the deliverable is the behaviour and the honesty about its edge, not
a table.

The comparator is one of three and must stay so. At-rules sort by code units
through a different comparator, and the two were one function once — which is
how the pseudo side came to be sorted by bytes in the first place.

**Blocked by:** 26 — the decision it records is what this builds.

**Status:** done

- [x] The reported pair sorts as the reference compiler sorts it, and the class
      name it hashes to is the reference compiler's, read from a run
- [x] The at-rule comparator is untouched and still sorts by code units, since
      upstream passes no comparator there at all
- [x] Existing pinned pairs — the printable-ASCII order, case as a tertiary
      difference, length settling a tie before case — all still hold, so
      whatever replaces the table does not lose what the table got right
- [x] The property the comparator is checked against is still checked against
      the reference compiler over random pairs, extended past ASCII to whatever
      range ticket 26 settled on
- [x] `pnpm parity` reports no row reading `divergent`
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Closing note

Delivered. The reported pair sorts as the reference compiler sorts it, and the
class name matches. The at-rule comparator is untouched and still sorts by code
units, which is correct: upstream sorts at-rules with a bare `.sort()` and only
pseudo keys with `localeCompare`, so the two comparators are two different
algorithms rather than one applied twice.

Every previously pinned pair still holds -- the printable-ASCII order and the
case tertiary among them -- because the ASCII fast path answers them unchanged
and is asserted to agree with root collation over every printable-ASCII pair.
`pnpm parity` reports no row reading `divergent`.
