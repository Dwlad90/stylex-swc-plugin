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

**Status:** ready-for-agent

- [ ] The rule is stated once, where a reader consuming parts will find it
- [ ] Every consumer of a part list is checked against the rule, and any that
      disagreed either changes or records why it differs
- [ ] `padding: '1px /*'` has an asserted, explained outcome, whether or not
      that outcome is today's
- [ ] The absence of a reference answer is recorded, so the next reader does not
      re-derive it from a crash
- [ ] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs
