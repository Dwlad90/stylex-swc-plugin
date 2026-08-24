# 18 — What an empty part means to an expansion

**What to build:** one stated rule for what an expansion does with a part that
is empty, applied everywhere parts are consumed.

An empty part is reachable: an unterminated comment contributes one, because a
comment node's text is what it contributes and there is none. So
`padding: '1px /*'` splits into two parts, the second empty, and the expansion
emits `padding-top` and `padding-bottom` while dropping both inline sides — a
two-sided padding from a four-sided shorthand.

There is no reference answer. The reference compiler throws
`Cannot read properties of undefined (reading 'type')` on this input, so
parity cannot arbitrate it and the decision is this compiler's to make and to
write down.

The decision has already been made once, narrowly, without the general rule
being stated: ticket 14's fold joins an empty part onto a preceding `auto`,
because the reference compiler's guard there asks whether a part is *absent*
rather than empty, and no part of a split value is absent. That reasoning
either generalises to every consumer or it does not, and which is not currently
recorded anywhere.

A defensible outcome is "current behaviour is correct, now documented". What is
not defensible is leaving two consumers disagreeing about the same shape for
reasons neither states.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] The rule is stated once, where a reader consuming parts will find it
- [x] Every consumer of a part list is checked against the rule, and any that
      disagreed either changes or records why it differs
- [x] `padding: '1px /*'` has an asserted, explained outcome, whether or not
      that outcome is today's
- [x] The absence of a reference answer is recorded, so the next reader does not
      re-derive it from a crash
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Outcome

Current behaviour is correct and is now stated. The rule — **an empty part is a
part**: it occupies its position, counts toward the arity, and is never read as
absent — is written once, in the module documentation of
`crates/stylex-css/src/values/parser.rs`, which is the only producer of a part
and so the place a consumer already reads.

All four consumers were checked against it and all four already agreed:

- the four-sided view assigns it to a side, whose declaration emits nothing
  later because its value is empty;
- the importance fold qualifies it like any other part;
- `contain-intrinsic-size`'s fold joins it onto a preceding `auto`;
- `list-style` lets it take the slot it landed in, and refuses `url(a.png) /**/`
  for two images.

`padding: '1px /*'` is asserted: `padding-top` and `padding-bottom` carry `1px`
and both inline sides are present and empty. The absence of a reference answer
is recorded beside the rule — the reference compiler throws
`Cannot read properties of undefined (reading 'type')` — so the next reader does
not re-derive it from a crash.

Seven splitter tests and nine expansion tests, including the second way to reach
an empty part (`/**/`, a terminated comment with nothing in it), an empty part
that is not last, importance landing on one, and the near miss of a quoted empty
string, which is a two-character part rather than an empty one.
