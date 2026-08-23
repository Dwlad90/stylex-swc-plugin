# 11 — An authored number is echoed, not reprinted

**What to build:** an author who writes a numeric value the shorthand expansion
path passes through gets back the value they wrote. Today that path re-reads
the digits into a double and re-prints them, so the authored spelling is lost
even though the value survives:

| authored | official compiler | this crate |
| --- | --- | --- |
| `1.50px` | `1.50px` | `1.5px` |
| `1E2px` | `1E2px` | `100px` |
| `1e21px` | `1e21px` | `1000000000000000000000px` |
| `1.7976931348623157e308px` | as authored | 309 digits |
| `-0px` | `0px` | `+-0px` |

The last row is not a spelling difference: the sign-carrying branch tests
`value >= 0.`, which a negative zero satisfies, so a `+` is prepended to a
value that already carries a `-`. `+-0px` is not a CSS value.

Because the printed text feeds the class-name hash, every row is a class-name
divergence, so output from the two compilers cannot be mixed across an SSR and
client boundary for any of them.

**Blocked by:** None — can start immediately.

**The fix is neither of the two already tried on this path.** Ticket 03 widened
it to a double, which fixed the *value* and left the spelling. Ticket 06
considered printing it through the shared JavaScript-number helper and rejected
that, correctly: the helper would write `1e+21px` where the official compiler
writes `1e21px`, trading one divergence for another. The third option is the
one that matches: echo the authored byte slice, and parse the number only for
the decisions that need it. Confirm the slice's extent against the tokenizer's
own bounds rather than re-scanning for one.

Scoped narrowly. The `Display` impls are correct as they stand — they print
values this crate *computed*, where the official compiler also computes and
also prints through `String(Number)`. This ticket is only about the path that
echoes.

**Status:** done

- [x] A numeric token the expansion path passes through is emitted with the
      digits, exponent spelling, and letter case the author wrote
- [x] `-0px` emits what the official compiler emits, and no value is ever
      emitted with two signs
- [x] Each row of the table above is pinned as a test, with the expectation
      confirmed against `@stylexjs/babel-plugin` from `node_modules`
- [x] The leading-zero strip and the `+` on an authored positive sign keep
      their current behaviour, each pinned so the change is visibly scoped
- [x] No `Display` impl changes; the shared JavaScript-number helper is not
      adopted on this path, and the reason is recorded where the next reader
      will look
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Closing note

Delivered. `authored_number` returns the author's bytes rather than a double,
and `leading_f64` is now the value-reading half of a span `leading_number`
hands back unread.

**Reachability, established before the fix rather than assumed.** The first
pass through this concluded the divergence might be unreachable, because a
plain `width: '1E2px'` comes out correct end to end -- `parse_css` is not on
that path. It is reached from shorthand expansion under
`styleResolution: 'legacy-expand-shorthands'`, and there every value in the
table diverged and every class name with it. `padding-top:+-0px` was reaching
a stylesheet.

**Confirmed against Babel, one shorthand at a time.** Five of the six probes
matched byte for byte after the fix, class names included; the sixth is the
divergence below. The new end-to-end snapshot pins twenty rules whose class
names are identical to `@stylexjs/babel-plugin@0.19.0`'s for the same source
and the same `styleResolution`.

**One divergence found and left, pinned rather than fixed.** The unit comes
from the token rather than the source, so an escaped unit is emitted as what it
escapes to: `1\70x` becomes `1px` here where the official compiler echoes the
escape. That is a lost token rather than a lost spelling -- the same shape of
finding as ticket 04's `lch()` percent -- and closing it means echoing the
unit's span as well as the number's, which is a different change.

**A related revert, from the same review.** Ticket 06 had converted
`SimpleToken::extract_value` to the shared formatter. No ticket asked for it,
it is not a `Display` path, and it is a token-text accessor -- the echo shape,
not the reprint shape -- so it was planting the pattern this ticket exists to
remove. Reverted to what it was.

**The `has_sign` branches are gone rather than corrected.** They existed to
prepend an authored `+` that a reparse had dropped; the authored sign is inside
the echoed literal, so prepending one was the second half of `+-0px`. An
authored `+` still survives, and that is pinned.
