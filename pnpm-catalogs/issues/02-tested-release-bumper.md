# 02 — Replace the release bumper with a tested script

**What to build:** Running the release bump moves the version in every place it
lives, and fails loudly rather than silently if any one of them does not move.

Today `scripts/git/bump-version.sh` does this with `find`-and-`sed` over
`Cargo.toml` and `jq` over `package.json`. Two problems. The Cargo half matches
`^version = "..."` in every `Cargo.toml`; it happens to hit exactly one line per
file only because `[workspace.dependencies]` uses inline-table syntax, so a
future entry written as a plain `version =` at column zero would be silently
rewritten to the release version. And a `sed` that matches nothing exits zero,
so a bump that quietly does nothing looks identical to a bump that worked.

Replace it with a Node script under `scripts/git/`, tested the way the existing
`scripts/git/*.test.mjs` suites are, owning all four locations the version
lives in:

- the Cargo workspace package version (crates inherit it, so no crate manifest
  carries a literal)
- the `version` field and every internal `@stylexswc/*` dependency range across
  the in-scope manifests
- the version reference in the root README
- the `internal` catalog block in `pnpm-workspace.yaml`

The last one has no referents yet — ticket 03 creates the catalog. Build and
test it here anyway, against a fixture. This is the piece that can fail
expensively and silently, and this ticket is the only point in the sequence
where it lands with tests but nothing depending on it. If it is wrong, the
blast radius is a test file rather than a botched publish.

The internal-dependency rewrite must skip `catalog:` specifiers the same way it
already skips `workspace:`, `file:` and `link:` — otherwise it overwrites the
catalog reference with a literal and undoes ticket 03 on the next release.

Delete the shell script and have the release workflow call the new one
directly. A wrapper that only execs the real script is a file future readers
must open to learn it does nothing.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] One script owns all four version locations; `bump-version.sh` is deleted
      and the release workflow invokes the replacement directly
- [x] Internal dependency ranges are rewritten, and `workspace:`, `file:`,
      `link:` and `catalog:` specifiers are all left untouched
- [x] A bump that changes nothing is an error, not a silent success
- [x] Tests assert all four locations move together, and cover the catalog
      rewrite against a fixture
- [x] `pnpm test:scripts` passes

## Comments

**The manifest set is `.syncpackrc`'s `source`, read at runtime, plus the
generated platform manifests.** Ticket 01 settled which manifests are source, so
`scripts/git/lib/manifests.mjs` reads that list rather than restating it — a
second copy is how a release starts rewriting a fixture or skipping a published
package. The platform manifests under `crates/stylex-rs-compiler/npm/*` are the
one addition: 01 excludes them because nothing hand-edits them, but they are
published under their own names, so their `version` is part of a release. That
module is also what 07 should read for its exemption list.

**Fixture manifests stop moving.** The shell bumper rewrote all 75 manifests
because `find` could not tell them apart; the replacement moves 66 and leaves
the five fixture/virtual manifests frozen at whatever they last read. Nothing
consumes the version they carry — checked: no Rust test, snapshot or resolver
assertion mentions it — and standalone is what 01 says they are for.

**The old `sed` also clobbered the StyleX compatibility badge.** Its README
pattern was `\(](.*/\)[0-9]\+\.[0-9]\+\.[0-9]\+\(.*)\)`, and the greedy `.*/`
reaches `…/badge/StyleX%20compatibility-v0.19.0-blue` on the following line, so
every stable release rewrote the compatibility version to the release version
until `update-stylex-compatibility.sh` happened to run afterwards. The
replacement anchors on the repository path, and a test asserts the neighbouring
badge is untouched.

**Two guards beyond the ticket, both cheap.** Crate manifests are refused if any
carries a literal `version` rather than inheriting the workspace one — that is
the assumption "no crate manifest carries a literal" made load-bearing instead of
assumed. And rewrites are staged in memory and flushed only after every location
verifies, so a failure leaves the tree at one version rather than two.

**The workflow's bump step moved after `🛠️ Setup environment`.** The bumper is
Node and uses `fs.globSync`'s array `exclude`; running it before setup would have
it on whatever the runner image ships rather than the pinned 24.18.0. The job
still runs two installs, as it did before — what changes is that setup's install
now sees manifests that match the lockfile instead of freshly bumped ones.

**From review — the catalog location could still have failed silently.** The
first cut returned quietly whenever `pnpm-workspace.yaml` declared no `internal`
catalog, which is correct today but would have become the exact silent no-op
this ticket exists to prevent the moment 03 lands: rename or drop that block and
the other three locations still move, so the run exits zero. Now the file
declaring `catalogs:` without an `internal:` among them is an error, while a file
declaring no catalogs at all stays fine. Both states have a test.

**From review — the crate guard is scoped to `crates/*`.** Globbing every
`Cargo.toml` would have hard-failed a release over a future fixture or example
crate legitimately pinning a literal version. The workspace members are what the
inheritance claim is about.

**Deliberately broader than the ticket, both inert today:**
`optionalDependencies` joins the three fields the old `jq` rewrote (napi injects
the platform optional deps at prepublish, so nothing matches yet), and the
specifier skip-list is any `scheme:` rather than the four named — a plain range
never contains a colon, and enumerating four invites a fifth being forgotten.
