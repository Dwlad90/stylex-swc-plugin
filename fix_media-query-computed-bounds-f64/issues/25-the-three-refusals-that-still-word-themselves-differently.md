# 25 — The three refusals that still word themselves differently

**What to build:** of the three subjects where both compilers refuse and word it
differently, the one that can agree does, and the two that cannot say so as a
reason rather than sitting as rows.

They are not the same kind of thing, which is why they read the same today and
should not.

A dynamic parameter spread into a style object is refused here for the binding
(`Referenced constant is not defined.`) and upstream for the call it sits in
(`Only static values are allowed inside of a create() call.`). Both are true;
upstream's names the position, which is the more useful of the two to an author
who wrote a spread, and nothing stops this compiler from naming it too.

The other two are a lone surrogate — in a theme export name, and in a condition
key. This compiler refuses the encoding at the point the name is decoded, before
it can reach the pass upstream refuses it in, because no Rust string holds a
lone surrogate at all. That is not an ordering that can be swapped; it is the
absence of a representation. Agreement would mean carrying a string the language
cannot spell.

So the second pair belongs in the refusal-family list, which is the mechanism
for a divergence stated as what agreement would cost, rather than in two
`expected` rows that read like unfinished work.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] The spread reads `both-reject` rather than `both-reject-divergent`, and
      the complaint it now writes names the position, matching what an author is
      handed upstream
- [x] Every other refusal that names a binding still names it: the spread is one
      position, not a rule about all of them
- [x] The two lone-surrogate subjects are claimed by a refusal family whose
      reason is the absence of a representation, and their `expected` rows go —
      a hand-written expectation and a family must not both claim a row
- [x] `pnpm parity` reports no unexpected row and no unreached family
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Closing note

Delivered. The spread refusal now reads `both-reject`, having been given the
reference compiler's wording for the position a spread cannot admit a value in.
Every other refusal that named a binding still names it -- the spread was the
one case where naming the binding was the divergence, not the information.

The two lone-surrogate subjects are claimed by the `lone surrogate in a name`
family, whose reason is that agreement would require carrying a string the
language cannot spell. `pnpm parity` reports no unexpected row and no unreached
family.
