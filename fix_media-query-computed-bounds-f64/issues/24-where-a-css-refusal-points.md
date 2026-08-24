# 24 — Where a CSS refusal points

**What to build:** an author whose build a CSS value stopped is pointed at the
same place by both compilers, and that is measured rather than assumed.

A refused build hands an author two things: the complaint, and where it
happened. The value harness compares the first by stripping the second — that is
what makes two messages comparable at all — so the position is left unmeasured
there, and a diagnostic naming a line that is wrong reads as agreement.

The position harness exists for exactly that, and every one of its subjects is a
reference-resolution question: an unresolved binding, a mutated one, a namespace
import, a hoisted declaration. Not one is a CSS value. So the guards a value
runs into — the unclosed function, the unclosed string, the declaration-
terminating token, the unclosed comment, the nesting budget — have had which
complaint fires pinned, and where it points pinned by nothing.

That gap has just been made sharper rather than smaller: the order those guards
speak in was changed to buy agreement on the sentence, and nothing would report
it if the position moved with it.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] One subject per CSS guard, each refused by both compilers with the same
      sentence so the position is all that is left to disagree about — which is
      the discipline the existing subjects already hold to
- [x] A value refused for two faults at once carries a subject, since that is
      the case where the two compilers reach the refusal from different passes
      and are most likely to name different places
- [x] Every divergence found is a defect or an entry recording it with the note
      that says why, and the run's exit code is what says which
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Closing note

Delivered. Each CSS guard carries a subject refused by both compilers, and the
two-fault case carries its own -- which is what turned up the ordering defect
that ticket 20 records: a value carrying both a shared rejection and a
structural guard was handed this compiler's complaint where the reference
compiler names the other one. Fixing that reordered the guard rather than
changing either message.

No divergence was left unaccounted for: every one is either fixed or carried by
a refusal family that states its reason.
