# 53 — Two mimalloc addons in one process stop the paired benchmark on macOS

**What to build:** The paired benchmark stops with `Segmentation fault: 11`
(exit 139) on darwin when the base and the candidate both link mimalloc.
The `publish` job needs `benchmark`, so the release stops.

A second defect makes the first one silent: the guard that exists to catch
this cannot see a binding that npm installed, so the run gives a bare
SIGSEGV instead of the message the guard holds.

## What causes it

Reproduced on darwin arm64, one fixture and one round for each case. The
base is an installed published package; the candidate is the local build.

| Case | Base | Base has mimalloc | Same version string | Result |
| --- | --- | --- | --- | --- |
| A | `0.18.6` | no | no | exit 0, both subjects measured |
| B | `0.19.0-rc.1` | yes | no | **SIGSEGV (139)** |
| C | `0.19.0-rc.2` | yes | yes | **SIGSEGV (139)** |

Case B is the one that answers the question. The version strings differ
there, so the cause is not that the base and the candidate carry the same
version. **Two mimalloc addons in one process is the cause.**

Case A shows that darwin holds two *different* bindings without a problem
when only one of them links mimalloc.

The allocator comes from `7142ccfb1`, which gave the addon `swc_malloc`.
Read from the shipped binaries, not from the history:

```
0.18.6        mimalloc strings: 0
0.19.0-rc.1   mimalloc strings: 6
0.19.0-rc.2   mimalloc strings: 6
local HEAD    mimalloc strings: 6
```

## Why no release has failed yet

`release.yml` takes the base from the newest **stable** tag:

```sh
regex='^[0-9]+\.[0-9]+\.[0-9]+$'
previousTag=$(git tag --list | grep -E "$regex" | sort -V | tail -n 1)
```

The pattern excludes every `rc`, so the base is `0.18.6` today, and
`0.18.6` has no mimalloc. Run `34265091652`, which published
`0.19.0-rc.2`, measured `npm@0.18.6` against `candidate@0.19.0-rc.2` — case
A — and passed on both macOS legs.

**So the next release still passes.** The first release that stops is the
one after `0.19.0` becomes the newest stable tag, because the base then
carries mimalloc.

The three dispatches that found this used `previous-version=0.19.0-rc.2`
to measure the four fold fixtures for
[ticket 20](./20-reprice-the-addon-ci-and-budget.md). They were the first
runs ever to pair two mimalloc addons.

## The guard does not see the binding

`assertBindingCanLoad` in `benchmark/lib/native-bindings.ts` is called from
`loadSubject`, and it is supposed to stop this load and explain it. It
never runs its check, because `findNativeBindings` finds nothing for an
installed base:

```
findNativeBindings(base) -> []
loadedNativeBindings()   -> ['.../rs-compiler.darwin-arm64.node']
```

`findNativeBindings` reads `<packageDir>/dist` and
`<packageDir>/node_modules/@stylexswc/rs-compiler-*`. npm **hoists** the
platform package instead: it goes to
`node_modules/@stylexswc/rs-compiler-darwin-arm64`, a *sibling* of
`rs-compiler`, and the installed package has no `dist/*.node` of its own.
So `bindings` is empty, `conflicting` is empty, and the guard returns and
says nothing.

The guard only works when the platform package is nested, which is the
pnpm layout of this workspace, and never for the npm layout the release
uses. Case A also shows the guard is silent when it should refuse.

## Notes for whoever fixes this

The two defects are independent and the second is the cheaper one.

`DUAL_LOAD_UNSAFE_PLATFORMS` and the module comment say darwin cannot hold
two bindings at all. Case A shows that is too strong: it can, unless both
link mimalloc. Whatever the guard becomes, it should say what the
measurement shows.

**Blocked by:** None.

**Status:** done

- [x] `findNativeBindings` finds the binding of a package that npm
      installed, with the platform package hoisted to a sibling
- [x] A test covers the hoisted layout, so the guard cannot go blind again
- [x] The paired benchmark gives a message, not a SIGSEGV, when it cannot
      load the second subject
- [x] A release passes end to end with a base that links mimalloc, or the
      gate measures each subject in its own process on darwin. The second
      half is done:
      [ticket 56](./56-measure-each-subject-in-its-own-process-on-darwin.md)
      gave darwin a process for each subject, and the pair that gave
      SIGSEGV now measures
- [x] The comment on the restricted-platform set says what case A shows

## Comments

**An earlier reading in this ticket was wrong and is removed.** It said the
break started with `0.19.0-rc.2` and stopped every release from the next
one onward. Two things were wrong: `0.19.0-rc.1` also carries mimalloc, and
the base of a release is the newest stable tag, not the newest published
version. The maintainer asked how run `34265091652` passed, and the answer
is that its base was `0.18.6`.

**The guard reads the allocator now, not the platform alone.** Case A is
measured behaviour and the guard follows it: on darwin it refuses a second
binding only where that binding and one the process holds both link
mimalloc. `linksMimalloc` reads the bytes of the addon, because the ticket
found the answer in the shipped binaries and not in the build settings. A
file it cannot read counts as a mimalloc build, so a binding this module
cannot measure keeps the older and stricter answer.

`findNativeBindings` now walks the whole Node resolution chain from the
real path of the package. The hoisted npm layout the ticket names is one
of two that the old single-directory search missed: pnpm, which is what
`npm.yml` installs the base with, links the platform package beside a copy
under `node_modules/.pnpm`, and that is not under the package either.

Re-measured on darwin arm64 after the change, one fixture and one round:

| Case | Base | Before | After |
| --- | --- | --- | --- |
| A | `0.18.6` | exit 0 | exit 0, both subjects measured |
| C | `0.19.0-rc.2` | SIGSEGV (139) | exit 1, the guard's sentence |

So the release that runs today keeps passing, and the first one whose base
carries mimalloc says why it stopped. Making that one pass is ticket 56.

**Closed by ticket 56.** The guard work here stopped the run with a
sentence instead of a SIGSEGV, which is not the same as making the run
work. Ticket 56 gave each subject a process of its own on darwin, and the
pair that ended the process now measures. Both halves are needed: the
guard is what tells a reader why a pair cannot share a process, and it is
what `subjectsCanShareProcess` reads to decide which path a run takes.

**Two more things the review of this work changed.**

`DUAL_LOAD_UNSAFE_PLATFORMS` and `isDualLoadUnsafe` are now
`DUAL_LOAD_RESTRICTED_PLATFORMS` and `isDualLoadRestricted`. The platform
alone no longer decides, so a name that calls it unsafe says more than the
measurement does.

`assertBindingIsVisible` closes the shape of the defect rather than the
one layout. `findNativeBindings` reads what npm and pnpm write, and an
installer that writes a third layout would give an empty list again --
which is what the guard read as "brings nothing to conflict with". So an
empty list is now a refusal in the one place it cannot be ruled out: a
restricted platform where the process already holds a mimalloc binding.
Everywhere else it still loads.

**What each CI leg does now.** `npm.yml` runs the guard on six benchmark
legs and the tests on six more.

| Leg | Reading |
| --- | --- |
| `benchmark`, windows and the three linux targets | `isDualLoadRestricted` is false, so both guards return at the first line. Nothing changes |
| `benchmark`, the two darwin targets | The base is `pnpm add`-ed into `$RUNNER_TEMP`, which is the layout the fix reads. With today's base of `0.18.6`, which holds no mimalloc, the guard permits and the leg passes as before |
| `test-*`, all six targets | The case that reads the build on disk passes on each, because every published target links mimalloc |

The last row was the one to check. `swc_malloc` leaves musl on the system
allocator, so a marker that reads mimalloc out of the binary might have
been true of five targets and false of one. It is not:
[ticket 40](./40-give-the-musl-target-an-allocator.md) named the allocator
for `x86_64-unknown-linux-musl` in the workspace manifest, and the shipped
`0.19.0-rc.2` musl binary holds the same six mimalloc strings darwin does.

What the second copy of the allocator does to the first is still not
known. The reading that it is a macOS memory zone was tested and is
wrong: the darwin addon references no zone call, so it holds mimalloc as
a plain Rust global allocator. The guard does not rest on a mechanism --
it rests on the three pairs measured above.
