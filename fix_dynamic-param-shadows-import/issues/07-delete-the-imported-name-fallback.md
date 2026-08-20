# 07 — Delete the imported-name fallback

Status: `resolved`
Blocked by: 04

**What to build:** An import stops resolving through the name it was aliased
*away from*.

The import lookup has a second half with no counterpart in the reference
implementation: after failing to match a reference against a specifier's local
binding, it tries the specifier's *imported* name. So a reference to `zIndex`
could resolve to `import { zIndex as zi }` — a binding that does not exist under
that name in any scope.

One arm of it is unreachable by construction: the imported identifier carries
the unresolved syntax context and never matches a reference's. The other arm,
for string-named specifiers, matches on symbol alone and remains live.

It is also the only route to a latent abort. Had the branch matched, the caller
re-searches by *local* name, finds nothing, and aborts with `Could not resolve
the import specifier` — a panic reachable through this branch and nowhere else.
Deleting the branch deletes the panic's only route.

Delete both arms and the now-unreachable abort. The suite is the check: if
something depended on resolving an import by its aliased-away name, that is the
finding, and it belongs in this ticket's comments rather than being worked
around.

- [x] Both arms of the fallback are gone
- [x] The abort reachable only through them is gone
- [x] Full suite green, or the dependency it exposed is recorded
- [x] A corpus guard: a reference whose name matches an import's aliased-away
      name resolves the way the reference implementation resolves it

## Comments

Both arms deleted from `get_import_from` in `shared/utils/common.rs`, which
leaves one comparison for all three specifier kinds: the local binding, by
`eq_ignore_span`. Nothing depended on the fallback -- the suite was green on the
deletion alone, before any test was rewritten.

The abort is gone by construction rather than by deletion. `get_import_by_ident`
now answers with the declaration *and* the specifier that bound the name, out of
the one scan that found it, so the caller in `js/evaluate/binding.rs` no longer
re-searches the specifier list by local name and has no empty case to abort on.
The lookup and the caller cannot disagree about which specifier matched, because
there is only one comparison left.

### The abort was reachable, not latent

The ticket called the panic latent. Measured before the change, it fires on
every module that reads a string-named specifier's imported name:
`import { "spacing" as sp } from 'tokens.stylex.js'` with `padding: spacing`
aborted with `Could not resolve the import specifier. Ensure the import is
correct.` -- the fallback matched by symbol, the caller re-searched by local
name, found nothing, and panicked. Nine cases in
`validation_stylex_create_test::invalid_values` reproduce it; each panicked at
`binding.rs:80` with the src change stashed and refuses with the reference
implementation's answer with it applied.

The identifier arm was unreachable as described: an imported identifier carries
the syntax context the parser gave it and a reference carries the resolver's, so
the two never compared equal on real source. It is asserted anyway in
`resolution_order.rs`, which assembles module state directly and shares one
context between an import and a reference -- the only place a fallback coming
back would be visible.

### Parity

Three corpus entries, all reading their recorded verdict against
`@stylexjs/babel-plugin` 0.19.0, over a 70-subject `--set modules` run with 0
changed verdicts:

| entry | verdict |
| --- | --- |
| `modules-1266-read-by-a-string-named-imports-imported-name` | `both reject` |
| `modules-1266-read-by-an-aliased-imports-imported-name` | `both reject` |
| `modules-1266-a-constant-named-after-an-aliased-away-import` | `identical` |

The third is what the deletion buys, and the reason it is worth a corpus entry
rather than only a unit test: a module that declares a constant under the name an
import was aliased away from now folds that declaration, where the import step
used to answer first and the declaration was never read. Both references sit in
one style object, so the alias's own binding is measured beside it -- `sp.small`
still resolves to the theme it names, byte for byte with upstream.

### Tests

- `common_tests`: the three cases that pinned the fallback now pin its absence,
  each asserting the local binding still resolves so a lookup that answered
  `None` for everything could not pass them. One new case fixes that
  `get_import_by_ident` answers with the specifier that bound the name, over a
  declaration carrying a default and a named specifier at once.
- `resolution_order.rs`: three `ImportedAs` shapes for the aliased spellings,
  and five cases -- the two aliased-away spellings are not the import, the local
  binding still is, a declaration of the aliased-away name is what the reference
  reads, and a global of that name folds to itself rather than to the import.
- `validation_stylex_create_test::invalid_values`: nine refusals, reaching the
  chain through a bare read, a member read, shorthand expansion, four levels of
  nested conditions, a computed key, an operand, a non-ASCII imported name and a
  unicode-escaped reference.
- `transform_stylex_create_test::static_styles`: three snapshots for the folding
  half, including one declaration carrying two aliased-away names read beside
  both aliases.

Verification: `cargo test --workspace --all-features` 6260 passed / 0 failed,
`cargo clippy --workspace --all-features --all-targets` clean, `cargo fmt` clean,
`pnpm typecheck && pnpm lint:check && pnpm format:check && pnpm test` green,
`parity --set modules` 0 changed verdicts over 70 subjects.

### For 12

The silence 12 owns is not this fallback. Read by its *local* name --
`import { "color-lg" as colorLg }` with `color: colorLg` -- the module is
unchanged by this ticket: the local binding matched before and matches now, and
the emit path still answers with nothing. What did change is the aliased-away
half: it aborted on the specifier re-search before and refuses like upstream now,
so a re-measurement of 12 will not be reading this ticket's panic by mistake.
