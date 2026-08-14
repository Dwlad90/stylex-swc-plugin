# 04 — Fix `+` dispatch so a string operand concatenates

**What to build:** `'1' + 2` in a style value produces `"12"`, as JavaScript
says, rather than the number `3`.

An author writing `flexGrow: a + b` with `a` of `'1'` and `b` of `2` gets
`flex-grow: 12` from the reference implementation and `flex-grow: 3` from this
compiler today. That is the worst class of parity bug in this port: not a failed
build an author can see and work around, but a **wrong value** that ships.

The cause is a helper bolted onto the addition arm that has no counterpart
upstream, where `+` is a single line returning `left + right`. Delete it, and
make dispatch ask what the language asks — **either evaluated side is a string,
so concatenate** — rather than what the helper asked, which was whether numeric
coercion happened to fail.

## Why this helper is worth reading before deleting it

It is the missing logical-expression branch, transplanted onto the wrong
operator. It carries the reference implementation's two-sided confidence dance
intact — a separate confidence state per side, each side's deopt reason falling
back to `unknown error` — a structure `+` has no use for at all. It also reads
one side's confidence off the outer state rather than off that side's own
state, which is a copy-paste slip that only makes sense in code that was moved,
and which means its per-side deopt branches cannot fire correctly. Its
right-hand diagnostics interpolate the left operand.

That transplant is the evidence for the root cause in the spec, so understand
it before removing it. Ticket 03 has already put the dance where it belongs.

Several currently-passing cases are passing *by accident* — the fallback
around this helper masks its narrow gate. `'a' + ''`, `'' + 'a'`, `1 + 'px'`,
`1 + 2`, nested `+`, and `+` inside a template literal all agree with upstream
today and must still agree afterwards.

**Blocked by:** 01.

**Status:** resolved

- [x] The transplanted two-sided-confidence helper is gone from the addition arm
- [x] Dispatch concatenates when either evaluated side is a string, rather than
      when numeric coercion failed
- [x] `'1' + 2` produces `"12"`; `2 + '1'` produces `"21"`
- [x] The cases that pass today still pass: `'a' + 'b'`, `'a' + ''`, `'' + 'a'`,
      `1 + 'px'`, `1 + 2`, nested `+`, and `+` inside a template literal
- [x] Numeric addition of two numbers is untouched — it does not become string
      concatenation
- [x] `pnpm run --filter=@stylexswc/rs-compiler build` before any suite that
      reaches the compiler through the Node package
- [x] `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`, `pnpm test` all
      pass
- [x] Lands as `fix(stylex-transform):` with fixtures
