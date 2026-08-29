# 26 — The fold module splits at the seams it names

**What to build:** `engine_fold` becomes the four modules its own doc comment
already enumerates, so the guard — the module's stated subject — is readable on
its own, and the tickets that follow are small edits rather than surgery inside
a three-thousand-line file.

The seams are not invented here; the module names them and `b5e80d0a3` already
proved one of them works by lifting `engine.rs` out:

- **transport** — `Transport`, `Inward`, `Carried`, and the inward conversions
- **guard** — the `admit_*` walk, which is what the module is called after
- **amplification** — the rendered-length arithmetic, `EntryAmplifier`, `Declared`
- **outward** — `Outward`, `to_value`, `to_object_value`, the own-key ordering

Two threading fixes travel with the move, because they are what makes the split
land instead of reproducing the same signatures across four files. `guard: Guard,
reader: &mut Reader` appears in eighteen signatures and `depth: Depth` in ten
more — one `Walk<'a> { guard, reader }` receiver turns most of those free
functions into methods. `Depth` is already a field of `Guard`, so the separate
parameter is a second copy of one value, and the two can disagree.

**Why first.** 27, 28, 29 and 32 each edit one of the four concerns. Done after
them, this is a rebase of four branches through a file move; done before, each of
them is a diff a reviewer can read. The cost is one large mechanical commit and a
rebase for anything in flight — that is the trade, and it was taken deliberately.

**Behaviour does not change.** No refusal is added, removed or reworded, and no
test is edited except where a `use` path moves. A diff that changes an expected
value is a mistake in this ticket, not a finding.

**Blocked by:** none — can start immediately.

**Status:** resolved

- [x] `engine_fold/` holds `transport`, `guard`, `amplification` and `outward`
      beside the existing `engine`, and `mod.rs` is the fold's entry point rather
      than its whole implementation
- [x] `Walk` carries the guard and the reader; `Depth` is read from `Guard` and
      is not also a parameter
- [x] The glob import at `engine_fold/engine.rs` names what it takes. The two in
      `nodes/` were left: this ticket read them as globbing `engine_fold`, but
      both spell `use super::super::*`, which is `evaluate` — a module the split
      does not touch and which still exists as one thing. Both already reach the
      fold by explicit path, and seventeen sibling `nodes/*.rs` files carry the
      same glob, so naming two of them would leave the directory less consistent
      than it started. The premise, not the code, was wrong.
- [x] `cargo test` passes with no expected value edited, and the transform
      fixture corpus is byte-identical
