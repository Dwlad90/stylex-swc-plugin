# 08 — Reshape Dependabot around the catalogs and guard the lockfile

**What to build:** A dependency update PR announces which catalog it moves, and
a PR that corrupts the lockfile fails visibly instead of being repaired by
accident.

**Grouping.** Reshape the npm groups so they mirror the eight catalogs. The
payoff is that "which catalog does this live in" and "which PR will bump it"
have one answer, and a PR title names a bucket a reader recognises.

Two groups must survive as narrow overrides listed *ahead* of the catalog
groups, because a group is a lockstep assertion — "these move together or CI
breaks" — and the catalog partition does not encode either coupling:

- `typescript` and `oxlint-tsgolint` are version-locked to each other. Folding
  them into a ten-package `types` group means a routine `@types/node` bump drags
  a TypeScript upgrade along, or blocks on one.
- `@stylexjs/*` and `postcss` are paired in today's config and land in different
  catalogs (`runtime` covers both, but the pairing is about release cadence, not
  taxonomy — confirm which grouping the current config actually intends before
  preserving it).

Review the remaining groups while reshaping: several in the current config
overlap, with the same bundler listed under two group names. Overlapping groups
were harmless before; named after catalogs they become misleading.

**Lockfile integrity.** Dependabot has supported pnpm catalogs since early 2025,
but there is a live upstream bug where a catalog entry is dropped from the
lockfile during an update. The dependency-sync workflow currently reinstalls
after syncing, which would probably restore a dropped entry — but "probably, by
accident" is not what should be guarding the lockfile of a package that ships
native bindings.

Add the assertion as a second mode of the catalog-integrity script ticket 07
built, not as inline workflow YAML: every catalog entry present before the sync
is still present after it, failing the run loudly if not. The workflow becomes a
caller. Inline bash in a workflow has no seam and cannot be tested, which is not
an acceptable property for the check guarding this lockfile.

Cover it with tests using the shared script-test harness, the way the existing
script suites do — plant a fixture lockfile with an entry removed, run the real
script, assert it fails.

A second upstream bug reports all catalog dependencies as production
dependencies regardless of their real type. That one is harmless here because the
config groups by name pattern rather than by dependency type — confirm that
remains true after the reshape rather than assuming it.

**Blocked by:** 07

**Status:** resolved

- [x] npm groups mirror the eight catalogs, with no package matched by two
      groups -- seven of the eight. See the note below: the two clauses of this
      criterion are in conflict and the second one wins.
- [x] The TypeScript lockstep pairing and the StyleX core pairing survive as
      narrow groups taking precedence over the catalog groups
- [x] No group relies on dependency-type filtering
- [x] The lockfile assertion is a mode of the catalog-integrity script from
      ticket 07, with the dependency-sync workflow calling it rather than
      inlining bash
- [x] The dependency-sync workflow fails if any catalog entry disappears from
      the lockfile, with tests driving the real script end-to-end
- [x] `pnpm test:scripts` passes

## Comments

**`peers` gets no group, and cannot.** A dependabot group matches a package
*name*; a catalog entry is a (catalog, name) pair. Every one of the nine
packages in `peers` also carries a development range in a semantic catalog, so
a `peers` group would match nine packages that `bundlers` or `frameworks`
already match -- exactly the duplicate this ticket's first criterion forbids.
Seven groups mirror the seven semantic catalogs, `peers` is documented as
deliberately absent, and the two entries for one package move in the same PR,
which is the behaviour they should have had anyway.

**Both sides of the comparison come out of git, and the guard runs before the
sync.** The ticket says "every catalog entry present before the sync is still
present after it". Taken as "after" meaning after the reinstall, the check is
close to vacuous, and a first pass built it that way: the install regenerates the
`catalogs:` block from `pnpm-workspace.yaml`, so a dropped entry is back before
the check reads the file and all it confirms is that the accidental repair
worked -- the "probably, by accident" this ticket set out to remove, now with a
green tick on it. Verified rather than reasoned: with an entry cut from the real
lockfile, the guard reading the reinstalled copy passes and the guard reading the
delivered copy names `runtime.lightningcss`.

So `--current` was added, the step moved ahead of the sync, and both sides are
read with `git show` -- `HEAD:pnpm-lock.yaml` as dependabot wrote it against
`<base>:pnpm-lock.yaml` from before the update. Off disk, any later step could
move the file; out of git, none can. `pnpm install --no-frozen-lockfile` stays
where it was, in the commit step. `Setup pnpm` installs with `--frozen-lockfile`
on a pull request, so it neither rewrites the lockfile before the guard reads it
nor hides the problem.

**Two groups were dropped rather than renamed.** `version-management` matched
`@changesets/*` and `nextjs` matched `@next/*`; neither is a dependency of this
workspace any more, in a catalog or out of one.

**The shared catalog reader gained a shape check.** `lib/catalogs.mjs` now reads
the `catalogs:` block indentation-driven rather than fixed at two levels, so one
reader serves both the declaration (leaf: the range) and the lockfile (leaf: a
`specifier`/`version` pair). Sharing it that way opened a hole the module header
had promised was closed -- a scalar where a mapping belongs parses cleanly and
means something else entirely, so a lockfile leaf written as a bare version would
have enumerated one "entry" per character index and passed. Both readers now
declare their leaf depth and reject anything else, which also subsumes the
default-catalog shape the workspace does not have.

**Follow-up worth considering, not done here.** Nothing enforces that the groups
stay a partition of the catalogs -- a package added to a catalog and not to a
group lands in an ungrouped PR, silently. The invariant was verified by hand
during this ticket (155 catalogued packages, nine groups, exactly one match
each, no pattern matching nothing). A test would need to read
`.github/dependabot.yml`, whose nested sequences the hand-rolled YAML reader in
`lib/catalogs.mjs` does not handle.
