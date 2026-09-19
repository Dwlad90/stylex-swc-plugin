# 98 — Close the two remaining blind spots

**What to fix:** two places where a check or a rule was weaker than it reads.

**A comparison blind to the line it filters.**
`assert_spellings_agree_but_for_the_import` filters the import lines out of both
outputs, so a regression confined to an import line passes silently. Its own
rustdoc admitted this. The anchor it carried -- that the agreed body holds
`_inject2(` -- is taken from the filtered body, so it cannot see the import line
either.

Answered by asking each spelling's **whole** output, import line and all, for
`stylex-inject` before the bodies are compared. Both spellings losing the
injector together is now the one thing that cannot pass.

**An inline style the runtime would omit.**
`props` wrote `style` whenever the merge answered `Some`, with no emptiness
test. `packages/@stylexjs/stylex/src/stylex.js:157` writes it only when
`style != null && Object.keys(style).length > 0`, so an empty merge would emit
`style: {}` where the runtime emits no `style` key at all.

No source reaches it: the non-`disable_mix` path
(`stylex-styleq/src/styleq.rs:385-403`) builds `sub_style` only for a non-null
new property, and `disable_mix` is set `true` at exactly one place in the
workspace -- `stylex-styleq/benches/performance_bench.rs:264` -- never in a
shipped options value. Answered anyway, with one `filter`, because it is cheap
to make total and it saves the next reader from having to establish the above.

**Blocked by:** None.

**Status:** resolved

- [x] The anchor reads the whole output, not the filtered body
- [x] An empty merge writes no `style` key
