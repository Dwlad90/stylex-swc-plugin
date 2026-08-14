# 06 — Migrate the published packages and the root manifest

**What to build:** The 13 package manifests and the root manifest declare their
dependencies by catalog reference, and the published artefacts are unchanged —
consumers installing from npm see exactly the ranges they saw before.

This is the batch where getting it wrong reaches users. `packages/unplugin` is
the hard one: it declares the widest peer ranges in the repo (`vite >=4.0.0`,
`webpack >=5.0.0`, `rollup >=3.0.0`, `next >=15.0.0`, `@nuxt/* >=3.0.0`,
`@farmfe/core`, `esbuild`) alongside narrow development ranges for the same
packages, plus `peerDependenciesMeta`. Every one of those pairs must stay a pair:
the narrow range points at its semantic catalog, the wide one at `peers`.
Collapsing a pair silently narrows what consumers may install.

`packages/unplugin/playground` sits at depth 3 and was invisible to syncpack
until ticket 01 — it is the manifest whose `vite` range had drifted.

The safety net worth verifying explicitly: pnpm rewrites `catalog:` specifiers
to real ranges during `pnpm publish` and `pnpm pack`, so published packages stay
installable by npm and yarn users. That is the property the whole migration
rests on for the seven public packages, and this ticket is where it should be
proven rather than trusted — pack each public package and diff the resulting
manifest's dependency blocks against the current published shape.

The root manifest is included here. It is one file, and grouping it with the
packages keeps the batch count down without making this ticket meaningfully
larger.

This batch is independently green; unmigrated manifests keep literal ranges.

**Blocked by:** 03

**Status:** resolved

- [x] All 13 package manifests plus the root manifest reference catalogs
- [x] Every narrow/wide range pair survives as two distinct catalog references,
      with `peerDependenciesMeta` untouched
- [x] Packing each public package yields dependency ranges identical to the
      current published ones, with no `catalog:` specifier surviving
- [x] The lockfile resolves to the same versions as before the change
- [x] Build, typecheck, full test suite and artifact checks pass
- [x] `syncpack lint` passes at all three call sites

## Comments

**132 ranges across 14 manifests; `packages/typescript-config` declares no
dependencies, so 13 files changed.** Every one of the 132 literal ranges already
matched its catalogued range exactly, so no assignment was a judgement call —
each dependency name resolved to exactly one semantic catalog, and the migration
was rejected outright on any name that matched zero or more than one.

**The pack property holds.** All nine non-private packages — `jest`,
`nextjs-plugin`, `plugin-shared`, `postcss-plugin`, `rollup-plugin`,
`rspack-plugin`, `turbopack-plugin`, `unplugin`, `webpack-plugin` — were packed
before and after. No `catalog:` specifier survives packing, and every packed
`dependencies`, `devDependencies`, `peerDependencies` and
`peerDependenciesMeta` block is byte-identical to `HEAD`'s except for one line:
`nextjs-plugin` gains `"@babel/core": "^8.0.1"` in `devDependencies`, which is
the pin below. Consumers installing from npm are unaffected either way —
`devDependencies` of an installed package are inert — and the consumer-facing
`dependencies` and `peerDependencies` blocks are unchanged everywhere. That the
pin packs as its literal range is itself the property under test. The ticket
says "seven public packages"; there are nine.

**The fifth `@babel/core` pin, carried over from 04.** 04 pinned
`@babel/core` on the four Next apps and left a note: "There is a fifth importer
of `next`, `packages/nextjs-plugin` ... the same pin should go on it when the
packages batch runs, ticket 06." Done here. It is resolution-neutral — the
manifest already bound `8.0.1` through `next`'s optional peer, and the only
lockfile movement is the declaration itself appearing against that same version.
As in 04, the point is to make the single-copy property structural rather than
dependent on which way a resolution pass happens to fall.

**Five narrow/wide pairs survive as two references**: `next` in
`nextjs-plugin`, and `@nuxt/kit`, `@nuxt/schema`, `rollup` and `vite` in
`unplugin`. Three more (`@farmfe/core`, `esbuild`, `webpack`) are wide-only and
point at `peers` alone. `next-rspack` reads `catalog:frameworks` on both sides,
which is correct — it is the package the workspace comment names as declared
twice at the *same* range and so deliberately absent from `peers`.

**Resolution-neutral.** All 53 non-empty lockfile importers resolve to identical
versions before and after; the 54th, `packages/typescript-config`, has no
dependencies. The lockfile's only other movement is growth in its `catalogs:`
snapshot as entries gain their first referent — every hunk is an addition, none
a removal or a version change. `pnpm peers check` reports the same five unmet
peers as the parent commit.

**The taxonomy questions 04 raised are still open, and this batch entrenched
them.** The root manifest now reads `"@stylexjs/eslint-plugin":
"catalog:runtime"` and `"oxlint-tsgolint": "catalog:types"` — an ESLint plugin
in the catalog defined as "libraries our own shipped code imports at run time",
and a lint backend in the catalog that otherwise holds only `@types/*` and
`typescript`. `loader-utils` sits in `bundlers` while being a shipped
`dependency` of `plugin-shared` and `turbopack-plugin`, which fits `runtime`'s
definition verbatim. None of this was decided here: the placements come from 03,
every literal range matched its catalogued range exactly, and re-cutting the
taxonomy would move entries that all three batches now reference. Recorded so
the next person reads it as inherited rather than chosen.

**A trap for 07 was found and recorded there:**
`packages/unplugin/playground` is migrated as this ticket asks, but it is not a
pnpm workspace member, so its references are never resolved and
`bundlers.vite-plugin-inspect` — declared only there — is absent from the
lockfile snapshot. `cleanupUnusedCatalogs: true` will therefore delete an entry
that does have a referent. See the comment on 07.
