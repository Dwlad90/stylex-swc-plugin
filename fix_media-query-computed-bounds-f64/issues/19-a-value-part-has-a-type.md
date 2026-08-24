# 19 — A value part has a type

**What to build:** the concept the glossary names as a value part exists in the
code as a type, so the rules that hold of one live on it instead of being
re-remembered at each call site.

A part is a `String` today, and a part list a `Vec<String>`, travelling through
the splitter, the four-sided view, the importance fold and the
`list-style-type` test. Each of those carries a rule about parts that nothing
enforces:

- a part is echoed, never re-spelled — no formatter, no re-quoting, no escape
  resolution
- a trailing importance annotation belongs to every part, not to one
- an empty part is a part (see ticket 18)

Raised by a review as a judgement call, not as a defect, and it is worth
recording as such: the code is correct today. The argument for the type is that
these rules were each got wrong once already, at a different call site, and a
`String` offers no place to put them.

The argument against is churn, and it is a real one. If the type ends up as a
newtype with no method that a free function could not have been, this ticket
was speculative and should be closed as such rather than landed — that outcome
is an acceptable result, not a failure.

**Blocked by:** None — can start immediately. Worth sequencing *after* 18, so
the empty-part rule is settled before deciding where it lives, though 18 does
not gate it.

**Status:** done

- [x] No behaviour change: the parity and fuzz reports read exactly as before,
      and the class names in every snapshot are unchanged
- [x] At least one of the rules above is enforced by the type rather than by a
      comment, or the ticket is closed with that finding recorded
- [x] The glossary entry for a value part names the type, so the concept and the
      code agree
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Outcome

**Closed as speculative, and the finding is the deliverable.** A part stays a
`String`.

The argument for the type was that its three rules had each been got wrong once
at a call site. The audit found why that is not an argument for a wrapper: all
three are already enforced by construction, because `split_value_parts` is the
only way to get a part. It applies the importance fold before returning, so no
consumer can see an unqualified part list; it spells a part only by echoing what
the scanner read, so there is no formatter on the path to reach; and it never
drops one, which is the empty-part rule of ticket 18. The one path around the
fold — a value nested past the depth budget returns before the split — is stated
beside the rules, and costs nothing: the fold is a no-op on the single part that
path returns. A newtype would hold no
method that the single producer is not already the choke point for — the outcome
the ticket named as an acceptable result rather than a failure.

What was missing was not the type but the statement. The three rules are now
written where a consumer reads them, in that module's documentation, and the
glossary entry for a value part names the producer and says why the concept is
deliberately not a type. No behaviour change: the parity and fuzz reports read
exactly as before, and no snapshot moved.
