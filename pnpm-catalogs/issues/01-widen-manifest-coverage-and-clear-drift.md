# 01 — Widen manifest coverage and clear the drift it exposes

**What to build:** Every source manifest in the workspace is subject to the same
consistency tooling, and the one version drift that condition currently hides is
gone.

Syncpack's `source` glob is `*/*/package.json`, which matches only depth-2 paths.
`packages/unplugin/playground/package.json` sits at depth 3 and has never been
linted or formatted by it. That blind spot is real and already cost something:
the playground declares `vite ^8.0.10` while the other nine declarations across
the workspace moved to `^8.2.0`.

Widen the glob to cover every source manifest, then add explicit negations for
the manifests that must stay outside: fixture manifests exist to be resolved as
if they were real user projects and must remain standalone, and generated
manifests are not source at all. Then bring the playground onto the version
everything else uses.

The exclusions are the substance of this ticket — getting them wrong is how a
fixture stops testing what users actually have. There are four families:

- path-resolver and transform fixture trees
- the transform virtual-app and singular `tests/fixture/` tree
- the postcss-plugin auto-discovery fixture
- `@napi-rs/cli`-generated platform manifests, and Next.js build output under
  `.next/` (gitignored, content is `{"type": "commonjs"}`)

After this lands, 57 manifests are in scope: 22 under `apps/`, 21 under
`crates/`, 13 under `packages/`, plus the root.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] Syncpack's `source` covers manifests at any depth, with explicit negations
      for fixture, virtual, generated and build-output manifests
- [x] The in-scope set resolves to exactly 57 manifests
- [x] `packages/unplugin/playground` declares the same `vite` range as the rest
      of the workspace
- [x] `syncpack lint` passes, and now reports on manifests it previously skipped
- [x] Install, build and the full test suite are green

## Comments

**Exclusions are honoured by `syncpack lint`, not by the pre-commit hook.**
`.lefthook/pre-commit/manifests.sh` interleaves one `--source` per staged path,
and a command-line `--source` overrides the config's negations rather than
intersecting with them — verified by mangling
`crates/stylex-path-resolver/fixtures/exports/package.json` and watching
`syncpack format --dry-run --source <that path>` offer to re-sort it while the
same command without `--source` stays silent.

This is pre-existing, not introduced here: the old `*/*/package.json` source
never reached those depth-3 fixtures either, so the old `!*/fixtures/*` negation
was already dead. What changes is that the negations now *look* authoritative.
Nothing is currently broken by it — `syncpack format` only sorts keys and sets
indent, it never touches a version — but the gap is recorded as a trap in
`guidelines/git/HOOKS.md` and belongs to **07**, which owns exemption behaviour
and its tests. Fixing it here would mean a second copy of the exclusion list in
POSIX sh, which is the duplication 07's single command-line interface exists to
avoid.

**The bare `"package.json"` source entry was dropped.** `**/package.json` matches
the root manifest on its own; the in-scope count is 57 either way.

**Pre-existing and untouched:** `syncpack format --check` reports 20
`PackagePropertiesAreNotSorted` issues, the same count before and after this
change. None of the three newly covered manifests contribute to it.
