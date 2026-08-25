# 07 — Port JavaScript object semantics to the transform's output

**What to build:** The rule count and ordering an author gets from the reference
implementation when rewritten query keys collide. The last-media-query-wins
transform currently accumulates rewritten keys in a sequence, so it keeps both
colliding entries; the reference implementation writes them into a plain object,
deleting the old key and assigning the new one, so assigning a key already
present keeps that key's original position and replaces only its value, while a
key not yet present is appended.

Reproduce that exactly. One authored declaration disappears as a result, and
that loss is the intended, faithful outcome. No diagnostic, warning or refusal
accompanies it: the reference implementation emits none, and a warning it does
not print would itself be a divergence in observable behaviour. Non-media
properties keep their relative order, and rewritten media keys follow them in
declaration order, as they do today.

**Blocked by:** 06.

**Status:** done

- [x] Ticket 06's seams pass, including which declaration survives and where.
- [x] A collision keeps the earlier key's position and the later entry's value —
      asserted, not merely implemented.
- [x] No diagnostic, warning, or error is emitted for a dropped declaration.
- [x] Relative order of non-media properties, and of media keys among
      themselves, is unchanged from before this ticket.
- [x] The Rust and JS suites pass, the JS suites against a fresh build, and the
      parity harness reports no unexpected rows.
