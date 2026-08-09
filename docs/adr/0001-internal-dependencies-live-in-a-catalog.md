# Internal `@stylexswc/*` dependencies live in a catalog

**Status:** accepted

Our own `@stylexswc/*` packages are declared in an `internal` catalog in
`pnpm-workspace.yaml` and referenced as `catalog:internal`, like every external
dependency. The bumper rewrites that catalog on release.

They are workspace members, so pnpm already has a protocol for them and the
bumper already rewrote their literal ranges. A reader will wonder why neither
was enough.

## Considered options

**Literal ranges, rewritten by the bumper.** Simpler, and already working.
Rejected because two mechanisms would own versions, and the boundary between
them is invisible at the call site: `"0.18.3"` does not say who owns it,
`catalog:internal` does.

**The `workspace:` protocol.** The obvious answer, and the one most likely to be
proposed again. It is not rejected because it "changes what gets published" —
that reason gets offered, and it is wrong. pnpm rewrites `workspace:` on pack
exactly as it rewrites `catalog:`. Packing `@stylexswc/rollup-plugin`:

| specifier          | published |
| ------------------ | --------- |
| `catalog:internal` | `0.18.3`  |
| `workspace:^`      | `^0.18.3` |

What changes is the range _shape_. `saveWorkspaceProtocol: rolling` makes
`workspace:^` what gets written, and the whole set ships in lockstep around a
native binding — so a consumer of `@stylexswc/rollup-plugin@0.18.3` should get
exactly `@stylexswc/rs-compiler@0.18.3`, not whichever `0.18.x` is newest.
`workspace:*` would hold that pin, but only by hand in every manifest, against
the configured default, with nothing checking it — and it would still be the
second mechanism the first option was rejected for.

## Consequences

**The bumper is a single point of failure for four version locations that must
agree.** Hence it is tested, treats a location that does not move as an error,
and refuses to rewrite any specifier carrying a scheme. A `sed` that matches
nothing exits zero; this must not.

**`catalogMode` is `prefer`.** Enforcement lives in
`scripts/git/catalog-integrity.mjs`, which can name the file and suggest a
catalog; a strict mode can only refuse. Reasoning sits at the setting in
`pnpm-workspace.yaml`. Cheap to reverse, hence a consequence rather than its own
ADR.
