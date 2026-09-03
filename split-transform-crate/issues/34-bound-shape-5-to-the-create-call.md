# 34 — Bound the fixture scan to the `stylex.create` call

**What to build:** Shape 5 of the parity harvester reads a transform fixture as
one string: if the literal holds `stylex.create(` anywhere, every
`key: 'value'` pair *in the whole literal* is a candidate declaration. A fixture
usually holds more than the call — imports, helper constants, a second module —
and an object among them contributes its keys as if they were CSS.

`logical_operators.rs` shows it: `const color = { primary: 'red' };` sits beside
the call as the receiver a `??` test folds through, and `primary: red` is a
corpus row because of it. Ticket 23 removed the same pair where it came from a
`stylex.env` argument; this is the other way it gets in.

The fix is to bound the scan to the object the call is given, which is the same
matched-brace question the env guard already answers, rather than to name the
keys that are not properties.

**Status:** resolved

- [x] Shape 5 reads only the argument of a `stylex.create` call, and a fixture
      with a second call has both read
- [x] Tests cover an object before the call, an object after it, and a fixture
      with no call at all
- [x] The corpus and `cases.rs` are regenerated once, and every dropped entry
      is named and explained
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code

## Comments

`extractStyleObjects` now bounds itself to the argument list of each
`stylex.create` call. `createCallRanges` makes one forward pass over the
literal and counts parentheses from the callee, over code only, so a
parenthesis in a value — `url(a.png)` is an authored test value — cannot close
the call early. A fixture with two calls gets two ranges; a call nested in
another stays inside the outer one. A call the fixture never closes yields no
range at all: reading to the end of the text would take in every object after
it, which is the behaviour this ticket removed.

Strings, template literals and comments are stepped over by one shared helper,
`skipNonCode`, which `enclosingBraces` now uses too. The two scans ask
different questions but step over the same text for the same reason, and
before this they carried two copies of the quote and escape rules with the
same blind spot: an apostrophe in a comment — `// don't read this` — opened a
string that never closed, and the whole fixture went quiet rather than failing.
A regex literal is still read as code, because telling one from a division
needs the grammar and no fixture writes one.

`isCreateCallee` also asks that the name starts the member chain, so an
`options.stylex.create` reads as the different receiver it is. This is the
distinction the environment guard already draws.

None of the three changes the corpus. Every fixture in the repo today writes
its comments inside string values, closes every call, and calls `create` on
`stylex` directly, so all three are guards against a fixture not yet written.

The match loop walks the ranges with a pointer rather than searching them per
key. `matchAll` yields matches in source order and the ranges are in source
order too, so the two advance together and the filter stays linear.

The corpus went from 833 declarations to 823, and `cases.rs` from 960 parser
cases to 957. Ten entries dropped, none added. Every one is a key that is not a
CSS declaration in a `stylex.create` object:

| Dropped | What it really is |
| --- | --- |
| `a: red` | `const o = { a: 'red' }`, the receiver an `Object(o).a` fold reads |
| `children: Hello World` | a JSX prop in a compiled `_jsx` call |
| `foo: bar` | an argument of `fn(item).arg(…, { foo: 'bar' }, …)` in a scope test |
| `gap: 4px` | `const tokens = { gap: '4px' }`, the binding a refusal test mutates |
| `primary: red` | `const color = { primary: 'red' }`, the receiver a `??` test folds |
| `s: 0.25rem`, `s: 4px` | the same, for the `??`, `||` and `&&` template tests |
| `insetInlineStart: 0`, `left: 10px`, `top: 10px` | the argument of `stylex.positionTry`, not of `stylex.create` |

The three `positionTry` values are real CSS, unlike the other seven. They are
still out of scope: the shape is named for the `create` argument, and reading
a second call means deciding which of its arguments hold declarations. The
values themselves are ordinary — `0`, `10px` — and the corpus already carries
both under other properties, so nothing the harness could test is lost.
