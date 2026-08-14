# Spec: Migrate to pnpm catalogs

**Status:** resolved

## Problem Statement

Dependency versions in this workspace are declared once per manifest, and there
are 57 manifests. 154 distinct external packages are declared across them, 84 of
them in more than one place: the Playwright test runner appears in 22 manifests,
React and React DOM in 20 each, the TOML formatter in 20, the script runner in
19, the static file server in 17. Changing any one of them means changing it
everywhere, correctly, by hand.

Nothing structurally prevents those copies from disagreeing. What prevents it
today is a linter whose configured source glob matches manifests exactly two
directories deep. Manifests deeper than that have never been linted or
formatted by it, and the workspace already carries the consequence: the unplugin
playground declares a Vite range one minor behind the nine other declarations
that were all bumped together. Nobody noticed, because nothing was looking.

The cost lands on several people. A contributor adding a dependency has to
discover what range the rest of the workspace already uses, and there is no
canonical place to look. A reviewer reading a dependency bump sees the same
version repeated across dozens of files and cannot tell a deliberate exception
from an omission. A maintainer resolving a merge conflict resolves it once per
manifest. And for peer-dependent packages, two copies of one version produce
nominally distinct types — the failure mode the dependency-sync workflow already
documents at length.

## Solution

Every npm dependency version becomes a **named constant** declared once in the
workspace configuration, and manifests reference the constant by name using
pnpm's `catalog:` protocol. Drift stops being something to detect and becomes
something you cannot express: you cannot diverge from a constant you reference
by name.

The constants are grouped into eight **catalogs**, named so the name answers a
question a reader has at the point of use — is this a bundler, a framework, a
type package, a piece of tooling, one of our own packages, or the deliberately
wide range we accept from consumers. There is no default catalog; every
dependency belongs to exactly one named group.

From a contributor's point of view: adding a dependency that the workspace
already uses picks up the existing range automatically. Adding one it does not
use produces a check failure that names the file, the dependency, and the
catalog it should live in. From a consumer's point of view nothing changes at
all — pnpm rewrites catalog references into real version ranges when packages
are packed and published, so what lands on npm is what lands there today.

The Rust side needs no migration. It already works this way: the Cargo workspace
declares shared dependency versions centrally and crates inherit them by
reference. This brings the npm half of the repository up to the pattern the Rust
half has used all along.

## User Stories

1. As a contributor adding a dependency the workspace already uses, I want the
   existing range applied automatically, so that I do not have to grep 57
   manifests to find out what everyone else settled on.
2. As a contributor adding a dependency nobody uses yet, I want a check that
   names the file, the dependency and a suggested catalog, so that I can fix it
   without having to learn the taxonomy first.
3. As a contributor, I want that check to run before my commit lands rather than
   twenty minutes into CI, so that the feedback arrives while the change is
   still in my head.
4. As a contributor, I want the check to stay silent about fixture and generated
   manifests, so that I am never told to "fix" a file that is deliberately
   literal.
5. As a contributor upgrading a shared dependency, I want to edit one line, so
   that the upgrade is an edit rather than a sweep.
6. As a contributor, I want to know which catalog a new dependency belongs in
   from the catalog names alone, so that the decision is not a judgement call I
   have to escalate.
7. As a reviewer of a dependency bump, I want the version change to appear as
   one line in one file, so that I can see what changed instead of scrolling
   through repetition.
8. As a reviewer, I want a deliberate exception to look different from an
   accident, so that a dependency pinned differently on purpose is visibly
   intentional.
9. As a maintainer resolving a merge conflict on a dependency bump, I want to
   resolve it once, so that a routine upgrade does not turn into dozens of
   identical conflict resolutions.
10. As a maintainer, I want the workspace configuration to be the single place
    versions are declared, so that it sits alongside the overrides, patches and
    build-approval policy already recorded there.
11. As a maintainer, I want manifests at any depth covered by the consistency
    tooling, so that a package does not escape review by being nested one level
    deeper than the glob expects.
12. As a maintainer, I want the drift that the current blind spot hides to be
    fixed before the migration, so that a stale range is not silently
    canonicalised into a catalog.
13. As a maintainer, I want the peer ranges we accept from consumers kept
    distinct from the ranges we develop against, so that a migration does not
    quietly narrow what users are allowed to install.
14. As a maintainer, I want unused catalog entries removed automatically, so
    that the configuration does not accumulate declarations nothing references.
15. As a maintainer, I want that automatic removal enabled only once every
    manifest is migrated, so that it does not delete the entire catalog set the
    moment it is introduced.
16. As a release manager, I want one command to move the version everywhere it
    is recorded, so that a release is not an exercise in remembering the fourth
    place.
17. As a release manager, I want a bump that changes nothing to fail, so that a
    silently ineffective release is impossible.
18. As a release manager, I want the bumper covered by tests before anything
    depends on it, so that its first real exercise is not a live publish.
19. As a release manager, I want internal package references rewritten to the
    new version while workspace, file, link and catalog references are left
    alone, so that the bumper never overwrites a reference it does not own.
20. As a release manager, I want the Cargo workspace version handled by the same
    tested code as the npm side, so that no single test can pass while the two
    halves disagree.
21. As a consumer installing a published package with npm or yarn, I want the
    dependency ranges I have always seen, so that the migration is invisible to
    me.
22. As a consumer, I want the wide peer ranges preserved exactly, so that the
    bundler version I already use keeps working.
23. As a consumer reading a published manifest, I want no catalog references in
    it, so that my package manager can install it.
24. As an AFK agent picking up a migration batch, I want each batch to be
    independently verifiable, so that I can finish and prove one slice without
    the rest of the migration existing.
25. As an AFK agent, I want the batch I am given to fit in one context, so that
    I do not have to hand off mid-file.
26. As an AFK agent, I want the enforcement check to already exist and be tested
    when I start migrating, so that I find out immediately if I miss a manifest.
27. As a maintainer watching CI during the migration, I want every intermediate
    state to be green, so that a red build means a real problem rather than an
    expected transitional one.
28. As a maintainer, I want the version linter stood down from version duties at
    the moment the migration starts rather than when it ends, so that it does
    not fail on the mixed state it is guaranteed to see.
29. As a maintainer, I want the reason that linter no longer asserts versions
    recorded in its own configuration, so that a future reader does not
    "restore" the checks and break the build.
30. As a maintainer, I want that linter to keep formatting and sorting
    manifests, so that a capability unrelated to versions is not lost as
    collateral.
31. As a maintainer reviewing an automated dependency PR, I want its title to
    name the catalog it moves, so that I know what area is affected before
    opening it.
32. As a maintainer, I want dependencies that must upgrade in lockstep to stay
    grouped together regardless of which catalog they land in, so that a routine
    type-package bump does not drag a compiler upgrade along or block on one.
33. As a maintainer, I want no package matched by two update groups, so that
    group membership is unambiguous.
34. As a maintainer, I want an automated update that drops a catalog entry from
    the lockfile to fail visibly, so that a known upstream bug cannot corrupt the
    lockfile of a package shipping native bindings.
35. As a maintainer, I want that guard covered by tests, so that it is not an
    untested check guarding the most consequential file in the repository.
36. As a maintainer, I want to know that automated updates do not depend on
    dependency-type classification, so that a second upstream bug that
    misreports every catalog dependency as production remains harmless.
37. As a future reader finding an internal package referenced through a catalog,
    I want an architecture decision record explaining why, so that I do not
    assume it was an oversight.
38. As a future reader, I want the alternatives that were rejected recorded
    alongside the decision, so that I do not re-propose one that was already
    considered.
39. As a future reader, I want the vocabulary this migration introduced defined
    once, so that "catalog", "peers catalog" and "bumper" mean the same thing to
    everyone.
40. As a maintainer, I want the glossary to stay a glossary, so that it does not
    silently become a second copy of this spec.
41. As a maintainer, I want fixture manifests untouched, so that the tests that
    resolve them as if they were real user projects keep testing what users
    actually have.
42. As a maintainer, I want generated manifests untouched, so that the tool that
    regenerates them does not fight the migration.
43. As a security-conscious maintainer, I want the existing overrides, patches
    and build-approval policy unaffected, so that the supply-chain controls
    already in place survive the change.
44. As a maintainer, I want the lockfile to resolve to identical versions after
    each batch, so that the migration is provably a refactor rather than an
    upgrade.

## Implementation Decisions

**Catalog topology.** Eight named catalogs, no default catalog: `bundlers` (~40
entries), `internal` (~29), `tooling` (~24), `runtime` (~17), `testing` (~17),
`frameworks` (~17), `types` (~10), `peers` (~9). Roughly 163 entries over 154
distinct packages. A semantic taxonomy was chosen over a single default catalog
so that the name carries meaning at the reference site; a purpose-built set was
chosen over reusing the automated-update grouping vocabulary because that
vocabulary overlaps (the same bundler appears under two group names) and leaves
roughly a third of dependencies unclassified.

**The `peers` catalog exists because a catalog key is a package name.** Nine
packages are declared twice with genuinely different intent — a narrow range the
workspace develops against, and a deliberately wide range accepted from
consumers. One package needing two ranges therefore needs two catalogs. This is
not confined to example apps: the NAPI compiler package declares the SWC core
both ways.

**Internal packages are catalogued too.** They are not on the workspace
protocol; they carry literal published versions that the release pipeline
rewrites. Catalogueing them means the release version now lives in four places
that must agree — the Cargo workspace package version, the manifests, the
`internal` catalog, and the README reference. This is the decision that earns an
ADR.

**One tested script owns all four version locations.** The existing bumper is
shell — `find`-and-`sed` over Cargo manifests, `jq` over npm manifests. Two
defects motivate the replacement: the Cargo pattern matches a version assignment
at the start of any line, and only happens to hit exactly one line per file
because the shared-dependency block uses inline-table syntax, so a future entry
written differently would be silently rewritten; and a substitution that matches
nothing exits successfully, making an ineffective bump indistinguishable from a
working one. The replacement is Node, tested, treats a no-op as an error, and is
invoked directly by the release workflow rather than through a wrapper. Its
internal-dependency rewrite must skip `catalog:` specifiers the way it already
skips workspace, file and link specifiers.

**Catalog mode is `prefer`, not `strict`.** Strict mode makes the most common
contributor command fail, and with named catalogs the fix requires choosing
which catalog — a judgement call the error cannot make. `prefer` makes the good
path automatic for anything already catalogued and leaves teaching to a check
that can name the file and suggest a catalog.

**Automatic cleanup of unused entries starts disabled and is enabled last.** The
expand step introduces ~163 entries with zero references; with cleanup enabled,
the next install deletes all of them. It is turned on only once every manifest
references a catalog, at which point anything it would remove indicates a
manifest the migration failed to wire up.

**The version linter stands down from version duties in the same step that
introduces the catalogs, not at the end.** Removing its version groups does not
disable version checking — it restores the tool's default, which flags any
dependency whose specifier differs across manifests. Mid-migration, a bundler
reads as a catalog reference in migrated manifests and a literal range in
unmigrated ones; the tool compares specifier strings and fails. It would also
fail permanently on the nine dual-declared packages, which legitimately
reference two different catalogs. The replacement is a single group that ignores
everything, labelled with the reason. Sorting, ordering and indent
responsibilities are untouched. There are three call sites of this lint, not
two: the pre-commit hook, the pull-request validation matrix, and the
documentation validation workflow.

**Manifest coverage is widened before anything else.** The linter's source glob
is broadened to match manifests at any depth, with explicit negations for four
families that must stay literal: fixture trees resolved as if they were real
user projects, the virtual test application, manifests generated by the NAPI
tooling, and Next.js build output. This settles the in-scope set at 57
manifests — 22 example apps, 21 crates, 13 packages, and the root — and exposes
the Vite drift, which is corrected in the same step so a stale range is not
canonicalised.

**Migration proceeds expand–contract, in three batches.** A single edit across 57
manifests cannot land green. Declaring the catalogs changes nothing on its own;
each batch is independently green because unmigrated manifests keep working
literal ranges; the contract step makes the invariant enforceable. Batches are
example apps, crates, and packages-plus-root.

**Example apps are treated as internal test surface first.** Their
integration-demonstrating dependencies are catalogued along with their
infrastructure dependencies. Documenting integration for users is a separate
concern from making the test bed consistent, and no documentation in the
repository quotes a manifest dependency block, so nothing user-facing regresses.

**Two integrity checks share one command-line interface.** "No literal range
outside the workspace configuration" and "no catalog entry vanished from the
lockfile" are both catalog-integrity assertions over the same data, so they are
two modes of one script rather than two scripts. The existing version-mismatch
check and the dependency-sync workflow become callers of it. This is what lets
the pre-commit job and both CI matrix entries pick up the new behaviour with no
change to their wiring, and it is what makes the lockfile guard testable at all
— inline workflow YAML has no seam.

**Automated update groups are reshaped to mirror the catalogs**, with two
narrow override groups listed ahead of them for couplings the taxonomy does not
encode: the compiler and its type-aware lint backend are version-locked to each
other, and the StyleX core packages move on a shared cadence. A group is a
lockstep assertion, and folding the compiler pin into a ten-package type group
would make a routine type-package bump drag a compiler upgrade along.

**Publishing is unaffected**, because pnpm rewrites catalog references into real
ranges during pack and publish. This is the property the whole migration rests
on for the public packages, and it is verified rather than assumed.

## Testing Decisions

**What a good test looks like here.** Every script suite in this repository
drives the *real* script end-to-end and asserts on what a caller can observe —
exit status, the message printed, the files left on disk. None of them reach
inside a script to test a function. That is the standard to hold: a test that
asserts a catalog entry was rewritten is good; a test that asserts a particular
parsing helper was called is not. Tests skip rather than fail when a
prerequisite is absent, so the suite stays runnable on a workstation that lacks
a tool CI has.

**Seams.** Two, and only one of them is new.

The first is the release bumper's command-line interface. This is not a new
seam — it replaces a shell script already driven end-to-end by the existing
release-script suite, and the replacement inherits that seam and its suite.
Tests stand up a throwaway workspace containing a miniature Cargo workspace, a
workspace configuration with an `internal` catalog, a handful of manifests and a
README; run the real script; and assert that all four version locations moved
together, that workspace, file, link and catalog specifiers were left alone, and
that a bump changing nothing exits non-zero.

The second is the catalog-integrity command-line interface, which is new. Its
manifest mode is tested by planting a literal range in a throwaway workspace and
asserting the check fails and names the file, the dependency and a suggested
catalog — the message is behaviour, not decoration, because `prefer` mode was
chosen specifically so that this message is what teaches the convention. Its
exemption behaviour is tested by planting literal ranges in fixture, generated,
virtual and build-output manifests and asserting silence; an exemption that
does not hold will be disabled by the first contributor it inconveniences. Its
lockfile mode is tested by removing a catalog entry from a fixture lockfile and
asserting a loud failure.

The 57 manifest migrations get **no new seam**. They are verified through gates
that already exist: the lockfile resolving to identical versions, the build,
the full test suite including visual regression, the version lint at all three
call sites, and — for public packages — packing each one and comparing the
resulting dependency ranges against the current published shape, confirming no
catalog reference survives.

**Prior art to follow.** The shared script-test harness already provides
everything these suites need: throwaway workspace creation, executable stubs
that record their own argument vectors, a search-path overlay so stubs win while
genuine tools stay reachable, a skip guard for absent prerequisites, and an
environment scrubbed of inherited git variables. That last one is not optional —
these suites run from a pre-push hook, and inherited git environment variables
aim every git call at the repository being pushed rather than the throwaway one
under test, which makes a suite pass from a terminal and fail from the hook.

**A golden snapshot of the catalog assignment was considered and rejected.** The
existing resolved-configuration golden earns its place because the resolved form
is merged from several sources and cannot be read off any single file. The
workspace configuration already *is* the resolved form, so a snapshot would
duplicate a file that appears in the same diff.

## Out of Scope

- **Cargo.** The Rust workspace already declares shared dependency versions
  centrally with crates inheriting them by reference. Nothing migrates. Only the
  release bumper's handling of the Cargo workspace version is hardened.
- **Fixture manifests.** They exist to be resolved as if they were real user
  projects; a catalog reference in one makes it test something users never have.
- **Generated manifests** — those produced by the NAPI tooling for platform
  binaries, and Next.js build output.
- **Dependency upgrades.** This is a refactor. Every batch must resolve to
  identical versions. The one exception is the drifted Vite range, corrected
  first and deliberately.
- **The context map and per-crate context files** the repository documentation
  promises. Only a root glossary seeded with the terms this migration settled is
  in scope; writing a context file for every crate as a side effect of a
  dependency migration is how both jobs end up half-done.
- **Replacing automated dependency updates** with a scheduled native update job.
  Considered as a response to the upstream catalog bugs and rejected as a much
  larger change that discards existing grouping configuration.
- **Strict catalog mode**, and any change that makes adding an uncatalogued
  dependency impossible rather than merely flagged.
- **Retiring the version linter.** It keeps its formatting and sorting duties.

## Further Notes

**Known upstream issues.** Automated dependency updates have supported pnpm
catalogs since early 2025, but two bugs are live. One drops catalog entries from
the lockfile during an update — this is what the lockfile-integrity mode exists
to catch. The dependency-sync workflow's existing reinstall would probably
repair a dropped entry, but "probably, by accident" is not an acceptable guard
for the lockfile of a package that ships native bindings. The second reports
every catalog dependency as a production dependency regardless of its real type;
this is harmless here only because the update groups match by name pattern
rather than by dependency type, which should be confirmed after the reshape
rather than assumed.

**Operational sequencing.** The migration batches rewrite manifests in bulk and
will conflict with any open automated dependency PR, and the dependency-sync
workflow runs the version linter's fixer against whatever configuration is
current. Close the open npm dependency PRs before the first batch starts; the
next scheduled run regenerates them against the new configuration.

**Why the bumper lands first with nothing depending on it.** It is the one piece
whose failure is both silent and expensive. Sequencing it ahead of the catalogs
is the only ordering in which a defect surfaces in a test file rather than in a
release.

**Evidence the current arrangement is insufficient.** The drifted Vite range is
not hypothetical. It sits in a manifest one directory deeper than the linter's
glob reaches, and it drifted across a bump that touched nine sibling
declarations. That is the strongest available argument that the boundary is
wrong and that detection is the wrong mechanism.
