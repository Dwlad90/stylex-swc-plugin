# 04 — Migrate the example apps onto the catalogs

**What to build:** All 22 example apps declare their dependencies by catalog
reference instead of by literal range, and every one of them still builds, runs
and passes its tests — including the visual regression suites.

The apps are where the duplication actually lives: `@playwright/test` appears in
22 of them, `react` and `react-dom` in 20, `@taplo/cli` in 20, `npm-run-all2` in
19, `serve` in 17. This batch is the bulk of the migration's value.

These manifests are treated as internal test surface first. That was a deliberate
call — the apps are the integration test bed, and documenting integration for
users is a separate job from making the test bed consistent. So catalogue
everything, including the dependencies that demonstrate the integration
(`vite`, `webpack`, `next`, `react`, `@stylexjs/stylex`), not just the
infrastructure ones.

Exclude the Next.js build output under `.next/` — those manifests are generated,
gitignored, and contain only `{"type": "commonjs"}`. Ticket 01 should already
have them outside the in-scope set; confirm rather than assume.

This batch is independently green. Unmigrated manifests in `crates/` and
`packages/` keep their literal ranges and keep resolving exactly as before.

Verification matters more here than in the other batches: these apps exercise
real bundlers across React, Vue, Solid, Next, Storybook and five plugin
integrations. A catalog assigned to the wrong entry shows up as a build failure
in one framework and nowhere else.

**Blocked by:** 03

**Status:** resolved

- [x] All 22 app manifests reference catalogs; no literal external range remains
      in `apps/`
- [x] Generated `.next/` manifests are untouched
- [x] The lockfile resolves to the same versions as before the change
- [x] Every app builds; the full test suite passes, visual regression included
- [x] `syncpack lint` passes at all three call sites


## Comments

378 references across the 22 manifests became `catalog:` references. Every one
of them already matched its catalogued range exactly, so the assignment was
mechanical and the batch is provably a refactor: with `apps/` migrated, every
importer in `pnpm-lock.yaml` resolves to the same version it did before.

No app declares `peerDependencies`, so the `peers` catalog is not exercised by
this batch -- every reference resolves to exactly one semantic catalog.

**One addition that is not a migration.** The four Next apps gain
`"@babel/core": "catalog:bundlers"` in `devDependencies`. It is needed to keep
the batch green, and the reason is worth recording because it is not obvious.

`next` declares an *optional* peer on `@babel/core`, and the workspace graph
contains two copies -- `8.0.1`, declared at the root and by the compiler crate,
and `7.29.7`, pulled in transitively by jest. Nothing pinned which one each
importer bound to. `HEAD`'s lockfile had every Next app on `8.0.1`, but that was
a fossil: `pnpm install` reported the lockfile up to date and so never
re-derived the peer dedupe. The first full resolution pass -- which migrating
these manifests unavoidably triggers -- flipped `nextjs-example` alone onto
`7.29.7`, producing a *second* physical copy of `next` in the store. That app
then loaded two `next` copies in one webpack build, one through its own
dependency and one through `next-rspack` inside `@stylexswc/nextjs-plugin`, and
the build died on `ObjectMiddleware.register: serializer for
mini-css-extract-plugin/dist/CssModule/null is already registered`.

This was confirmed independent of the catalogs: reverting `apps/` to literal
ranges while keeping the re-resolved lockfile reproduces the same failure. It is
latent breakage the batch surfaced rather than caused. Declaring `@babel/core`
explicitly pins the binding and restores `HEAD`'s resolution. `pnpm dedupe` does
not fix it.

The pin goes on all four Next apps, not just the one that broke. Only
`nextjs-example` flipped, but the other three held `8.0.1` by the luck of the
same resolution pass, and nothing prevents the next one flipping a different app
into the same two-copy state. Pinning one app fixes this resolution; pinning all
four makes the property structural. Adding the other three is
resolution-neutral -- they were already on `8.0.1`.

There is a fifth importer of `next`, `packages/nextjs-plugin`, which declares it
as a `devDependency` and is out of scope here. It binds `8.0.1` today; the same
pin should go on it when the packages batch runs, ticket 06.

**A transitive consolidation the batch also surfaced.** `http-proxy-middleware`
went from two copies to one: `HEAD` carried both `3.0.5` and `4.2.0`, and the
re-resolution collapsed `@farmfe/core`'s subtree from `3.0.5` onto the `4.2.0`
that was already present, dropping `http-proxy`, `follow-redirects`,
`eventemitter3`, `requires-port`, `@types/http-proxy` and `3.0.5` itself.

Farm declares `^3.0.0` and the override `'http-proxy-middleware@>=3 <3.0.5':
'>=3.0.5'` is open-ended, so a full pass floats it to the newest satisfying
version. This is the same fossil effect as the `@babel/core` flip and was
verified the same way -- it reproduces with `apps/` reverted to literal ranges.
No new version enters the lockfile and the override's security intent is
preserved, but it is a version move inside a dependency subtree, so it is
recorded here rather than left for someone to find in the lockfile diff.

**A third fossil, fixed here rather than deferred.** `@napi-rs/cli` in
`crates/stylex-rs-compiler` re-bound its optional `@emnapi/runtime` peer from
`2.0.0-alpha.3` to `1.11.2`. That is not a cosmetic shift: `@napi-rs/cli@3.8.2`
pins the peer to *exactly* `2.0.0-alpha.3`, so `1.11.2` does not satisfy it and
`pnpm peers check` reported it unmet. `HEAD` satisfied it. Declaring
`@emnapi/runtime` at `2.0.0-alpha.3` in that crate's `devDependencies` restores
the binding and clears the warning. The range is literal because `crates/` has
not migrated yet; ticket 05 catalogues it with the rest.

With that pin in place the lockfile carries **no** unintended movement. The only
resolution differences from `HEAD` are the five deliberate additions -- four
`@babel/core` and this one -- and every pre-existing resolution is restored
exactly.

**One pre-existing problem this batch does not fix.** The visual regression
suites are flaky on a workstation under load, failing on a different app each
run and passing when re-run alone. `HEAD` fails the same way, on
`next-rspack-example`, so this is environmental, not a regression.

**One pre-existing problem this batch does fix.** `pnpm lint:type-aware` failed
with one error in `scripts/git/bump-version.mjs` -- the bumper from ticket 02.
`createRun`'s `problems: []` inferred as `never[]`, so the template literal that
prints each problem tripped `restrict-template-expressions`. Annotating it
`/** @type {string[]} */` fixes it; the bumper's 15 tests still pass. The gate
is green, so it now protects the batches that follow.

**A taxonomy strain worth settling before ticket 05/06.** Three assignments the
catalogs already fixed read oddly at their new reference sites: `postcss`,
`postcss-nesting` and `autoprefixer` sit in `runtime`, defined as "libraries our
own shipped code imports at run time", but in `webpack-example`,
`nextjs-postcss-example` and `example-storybook` they are `devDependencies` that
only help produce a build artifact -- the comment's own definition of
`bundlers`. `serve` sits in `testing` but backs the plain preview script in most
of the 17 apps that declare it. Each package belongs to exactly one catalog, so
this batch had no choice; worth deciding before the remaining manifests
entrench the placement.
