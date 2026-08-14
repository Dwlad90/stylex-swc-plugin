# 03 — Declare the catalogs and stand syncpack down from version duties

**What to build:** Every dependency version in the workspace is declared once,
by name, in `pnpm-workspace.yaml` — and nothing references those declarations
yet, so resolution is byte-for-byte unchanged.

This is the expand half of an expand–contract migration. Adding the catalogs
changes nothing on its own; tickets 04–06 migrate manifests onto them in
batches, each batch green because unmigrated manifests keep working literal
ranges; ticket 07 contracts by making the invariant enforceable.

Eight named catalogs, no default catalog. Every dependency lands in exactly one,
and the name answers a question a reader actually has at the call site:

| catalog      | ~n | what belongs there                                                                 |
| ------------ | -- | ---------------------------------------------------------------------------------- |
| `bundlers`   | 40 | webpack, rollup, vite, esbuild, rspack, rsbuild, farm, swc, unplugin, tsdown, turbo, and the loader/plugin ecosystem around them |
| `internal`   | 29 | `@stylexswc/*`                                                                     |
| `tooling`    | 24 | oxlint, oxfmt, knip, lefthook, syncpack, publint, commitlint, napi-rs cli, taplo, rimraf, scripty, tsx, esno, del-cli, npm-run-all2 |
| `runtime`    | 17 | `@stylexjs/*`, postcss, picomatch, lightningcss, autoprefixer, chalk, prop-types, fast-glob |
| `testing`    | 17 | vitest, jest, playwright, testing-library, jsdom, tinybench, memfs, serve           |
| `frameworks` | 17 | react, next, vue, nuxt, solid, storybook                                           |
| `types`      | 10 | typescript, oxlint-tsgolint, `@types/*`                                            |
| `peers`      |  9 | the deliberately-wide consumer ranges                                              |

`peers` exists because nine dependencies are declared twice with genuinely
different intent — a narrow range we develop against and a wide range we accept
from consumers. A catalog key is the package name, so one package needing two
ranges needs two catalogs. Those nine are `@swc/core`, `webpack`, `vite`,
`rollup`, `next`, `esbuild`, `@nuxt/kit`, `@nuxt/schema`, `@farmfe/core`.
`next-rspack` is also declared twice but at the same range on both sides, so it
needs no `peers` entry — check this rather than assuming it.

Two settings, and one of them is a trap:

- `catalogMode: prefer`. Not `strict`. Strict makes plain `pnpm add <dep>` fail
  with no obvious fix, and with named catalogs the fix requires a judgement call
  about which catalog. `prefer` makes the good path automatic for anything
  already catalogued and leaves teaching to the check in ticket 07, which can
  name the file and suggest a catalog.
- `cleanupUnusedCatalogs: false`. This ticket adds ~163 entries with zero
  referents. With cleanup on, the next install deletes all of them. It flips to
  `true` in ticket 07, once every manifest references a catalog.

**Syncpack has to stand down in this same ticket, not later.** Stripping its
version groups does not disable version checking — it restores syncpack's
default, which flags any dependency whose specifier differs across manifests.
Mid-migration `vite` reads `catalog:bundlers` in migrated manifests and a
literal range in unmigrated ones; syncpack compares specifier strings, sees a
mismatch, and fails. It would also fail on the nine `peers` entries permanently,
since those legitimately read `catalog:peers` in one field and
`catalog:bundlers` in another.

Replace both `versionGroups` and the `semverGroups` entry with a single group
that ignores everything, labelled with why syncpack no longer owns versions —
that label is the comment a future reader needs at the moment they wonder why
the config looks defanged. Keep the sorting and indent configuration untouched;
that job is unaffected by catalogs. Note there are **three** call sites of
`syncpack lint`, not two: the lefthook version-mismatch job, the PR validation
matrix, and the docs validation workflow.

Operational note for whoever picks this up: tickets 04–06 rewrite manifests in
bulk and will conflict with any open npm Dependabot PR, and the dependency-sync
workflow runs `syncpack fix` against whatever config is current. Close the open
npm Dependabot PRs before starting ticket 04; the next scheduled run
regenerates them.

**Blocked by:** 01 (manifest coverage settled), 02 (the bumper must understand
the `internal` catalog before that catalog exists)

**Status:** resolved

- [x] Eight catalogs declared, no default catalog, every in-scope dependency in
      exactly one
- [x] The `peers` catalog holds only ranges that genuinely differ from the
      development range
- [x] `catalogMode: prefer` and `cleanupUnusedCatalogs: false` are set
- [x] Syncpack keeps its formatting and sorting duties and asserts nothing about
      versions, with the reason recorded in the config
- [x] Install produces no lockfile change — nothing references a catalog yet
- [x] `syncpack lint` passes at both live call sites — the lefthook
      version-mismatch job and `docs-validation.yml`. The third the ticket names,
      the PR-validation matrix, is commented out in the workflow and so cannot
      be run; it picks up this config whenever it is re-enabled. Build and full
      test suite green.

## Comments

**From 02.** The bumper treats a `catalogs:` block with no `internal:` among them
as an error, so the moment this ticket adds the catalogs the release bumper
starts requiring that one. Nothing extra to do here beyond declaring it — but if
the block is ever renamed, `scripts/git/bump-version.mjs` is the second place
that has to change.

**The partition was generated from the manifests, not transcribed.** A throwaway
script read the 57 in-scope manifests (`.syncpackrc`'s `source` globs applied to
`git ls-files`), collected every declaration, and refused to emit unless every
one of the 154 distinct packages landed in exactly one catalog and no catalog
named a package nothing declares. That is what makes "every in-scope dependency
in exactly one" a checked property rather than a claim — hand-sorting 154 names
across eight buckets is exactly the job where an omission is invisible.

**Final counts, against the ticket's estimates:** bundlers 41 (~40), internal 29
(29), tooling 23 (~24), testing 19 (~17), frameworks 17 (17), runtime 15 (~17),
types 10 (10), peers 9 (9) — 163 entries over 154 packages, as forecast. The two
that drifted from the estimate are one judgement call: the three `eslint-plugin-*`
packages all went to `tooling` rather than splitting `eslint-plugin-storybook`
into `frameworks` by subject matter, and the `@vitest/*` and `@swc/jest` packages
went to `testing` rather than to the bundler whose name they carry. Both follow
the rule the catalog names promise — what the package *is for* at the reference
site, not who ships it.

**`next-rspack` was checked, not assumed.** It is declared in both `dependencies`
and `peerDependencies`, but at `^16.2.12` on both sides, so it has no `peers`
entry. The nine that do are exactly the nine the ticket names.

**The catalogs were proved to resolve, not just to parse.** Declaring 163 entries
nothing references is unfalsifiable on its own — `pnpm install` is a no-op either
way, which is the acceptance criterion but also means a typo'd catalog name would
have survived to ticket 04. So one manifest was temporarily pointed at
`catalog:types`, `catalog:bundlers` and `catalog:internal`, installed, and the
lockfile diff inspected: the `specifier:` lines changed to the catalog references
and every `version:` line stayed identical (`5.109.2`, `26.1.2`,
`link:../typescript-config`). That is the "provably a refactor" property the spec
asks for, demonstrated once here rather than first discovered in a batch of 22.
The probe was reverted; the lockfile is byte-identical to before.

**The bumper was exercised against the real catalog block.** Ticket 02's comment
warned that adding `catalogs:` makes the bumper start requiring an `internal:`
among them. Running the real `scripts/git/bump-version.mjs --root <copy>` over a
copy of the actual `pnpm-workspace.yaml` moves all 29 `internal` entries to the
new version and leaves `peers` and `bundlers` untouched — so the two tickets are
wired together, not merely compatible.

**Syncpack's split of duties turns on which subcommand runs.** `syncpack lint`
covers versions and semver ranges only; sorting and field order belong to
`syncpack format`, which the `manifests` pre-commit job invokes separately. So
standing `lint` down costs nothing formatting-wise, and this was checked both
ways rather than reasoned about: a planted version mismatch and a loosened
`typescript` range both pass silently, while a manifest with reversed
`devDependencies` is still caught and fixed (`PropertyIsNotSortedAz`,
`PackagePropertiesAreNotSorted`).

**Two things the stand-down gives up until 07, deliberately.** The deleted groups
pinned `typescript` and `oxlint-tsgolint` to exact ranges and required them
identical everywhere. Both are currently unenforced. The catalog already records
them exactly (`typescript: '7.0.2'`, `oxlint-tsgolint: '7.0.2001'`), so once the
root manifest migrates in 06 the exactness is structural — a single entry with no
range prefix — and 07's catalog-integrity check restores enforcement. Between now
and then this is the accepted transitional state, not an oversight. Also
`customTypes.pnpmOverridesLegacy` is now inert: the only group that referenced
`pnpmOverrides` is gone. It was left in place because 07 and 08 may want it.

**Only two of the three `syncpack lint` call sites are live.** The lefthook
`version-mismatch` job and `docs-validation.yml` both run it and both pass. The
PR-validation matrix entry is commented out (the `dependency-graph` check), so it
picks up the new config whenever it is re-enabled — worth knowing before 07 wires
the catalog-integrity check into the same three places.

**Still outstanding for whoever starts 04:** close the open npm Dependabot PRs
first. The ticket flags this and nothing here does it — the migration batches
rewrite manifests in bulk and will conflict, and `sync-deps.yml` runs
`syncpack fix` against whatever config is current.

**From review — the rationale now lives in one place.** The first cut told the
same story twice: a 875-character `label` string in `.syncpackrc` and a comment
header in `pnpm-workspace.yaml`. Two copies to keep true, and the JSON one broke
the 100-character line limit outright. `.syncpackrc` cannot carry comments — it
is `JSON.parse`d by `scripts/git/lib/manifests.mjs` to derive the manifest scope,
so it has to stay strict JSON — which settles which file wins: the labels are now
one-line pointers ("Versions are catalogued, not linted -- do not restore, see
pnpm-workspace.yaml") and the reasoning lives once in the YAML, the file a reader
lands in anyway. Both files are now within the line limit.

**From review — the comments no longer describe a script that does not exist.**
Both files referred to "the catalog-integrity check" in the present tense. It is
ticket 07's work and is not in the tree, so as written they were stale on arrival
and would read as a missing file to anyone who went looking. `catalogMode` now
says plainly that no such check exists yet and that an uncatalogued dependency is
caught in review until it does.

**From review — the two catalog boundaries that the names do not settle are now
stated.** `runtime` holding `chalk`, `fast-glob` and `browserslist` and
`bundlers` holding `turbo` and `tsdown` both read as miscategorised until you
know the rule. Both are what the ticket's own table prescribes, so the fix was to
write the rule down rather than repartition against the spec: `bundlers` is
anything that participates in producing a build artifact, `tooling` is what you
run at the repository rather than through a build, and `runtime` means libraries
our shipped code imports at run time — not "runs in a browser", which is the
misreading, since this repo is a compiler and its run time is somebody else's
build step.

**From review — the `peers` contradiction was wording, not structure.** The
header claimed every dependency belongs to "exactly one named group" while
`peers` exists precisely to give nine of them a second. It now says every
dependency has exactly one *development* range in one of the seven semantic
catalogs, and that `peers` is cut on a different axis rather than being an eighth
sibling — which is what the partition actually does.

**Not fixed, and why.** `customTypes.pnpmOverridesLegacy` is inert: the only
group that referenced `pnpmOverrides` was the deleted one. It is left in place
because 07 and 08 may want it, and it cannot be marked as dormant in the file
itself for the same reason the rationale had to move out — `.syncpackrc` is
strict JSON with no comment syntax. Whoever picks up 07 should either give it a
referent or delete it.

**Also raised in review and rejected: "zero adoption — all 163 entries are
referenced by no manifest".** That is the acceptance criterion, not a defect —
"Install produces no lockfile change — nothing references a catalog yet". The
expand half is supposed to be inert.
