# 20 — Re-price the addon's CI budget and release matrix after the LTO change

**What to build:** [Ticket 16](./16-measure-the-crate-type-change.md) dropped the
unused `rlib` from `stylex-rs-compiler`, which lets the fat LTO of
`profile.release` run for the first time. Two consequences need attention, and
neither can be measured on a laptop.

**1. Release build cost, and whether the runners survive it.**
`.github/workflows/npm.yml` builds seven targets, three of them in Docker
(musl, aarch64-gnu). Locally the release build went from 144.79 s to 248.52 s,
and fat LTO with `codegen-units = 1` raises peak link memory as well as time.
The hosted runners have about 7 GB. The matrix carries no `timeout-minutes`.
Watch the first release run, and record what the aarch64 and musl legs cost.

**2. The benchmark budget is uncalibrated, and now is the moment.**
`crates/stylex-rs-compiler/benchmark/budget.json` is
`"state": "pending-calibration"` with no entries, so nothing gates on the old,
slower numbers. Seed the ceilings from the LTO build before the figures drift,
rather than from a build that never had LTO.

**Blocked by:** ticket 16, which is what makes this necessary.

**Status:** ready-for-human

- [x] The release workflow is observed end to end, and the per-target build
      times are recorded. Seven complete runs, 2026-09-04 to 2026-09-08, in
      [`../bench/ticket-20.md`](../bench/ticket-20.md). The slowest leg is
      `aarch64-pc-windows-msvc` at 18.3 min; the Docker legs are not the slow
      ones. Every leg of every run passed.
- [x] The runner headroom question is answered, and the premise was stale.
      The runners report 16 GB, not about 7 GB, and no leg ran out of memory
      or was retried under fat LTO with `codegen-units = 1`.
- [x] `npm.yml` has a `timeout-minutes` on each job, and each value comes
      from the measured cost of that job. `release.yml` and
      `pr-validation.yml` had the same gap and now have limits too. Only
      `upload-npm` has none, because GitHub does not permit the key on a
      `uses:` call, and the workflow that it calls sets its own limits.
- [x] `budget.json` carries calibrated ceilings taken from an LTO build. It
      is `enforced`, with one ceiling for each of the 65 fixtures, on the
      canonical environment. Sixty-one ceilings come from ten clean runs and
      four from three, because the four fold fixtures could not be measured
      until the published base could fold. Each ceiling is the largest
      median-of-round p95 that any run gave, times a headroom of 1.25. Every
      run was replayed against the file and none breaches a ceiling.

## Comments

**One measurement is recorded, from the run that
[ticket 40](./40-give-the-musl-target-an-allocator.md) was filed on.** The
paired release gate compared the LTO candidate against the previous published
release, per artifact:

- Every glibc artifact read 0.33-0.82x of the previous release, so faster.
- The musl artifact read 1.13-1.53x, so slower — which is what led to the
  allocator work in ticket 40, and what the numbers above no longer describe.

This prices the *artifacts*, not the build. The two open halves are the
per-target build times and the runner headroom, both of which need the release
workflow watched while it runs. `budget.json` should be seeded from a run taken
after ticket 40, because the musl figure above is from before it.

**The two open halves are now closed, and the budget half moved rather than
finished.** Seven complete release runs are recorded, so the build cost and
the runner headroom are both settled — and the headroom worry rested on a
stale figure, because the runners carry 16 GB.

The budget could not be seeded, for a reason the ticket could not have known.
`parseBudget` refuses entries while the state is `pending-calibration`, so
there is no way to commit a measured ceiling without also turning the gate
on. Turning it on today fails the next release twice, and neither failure
would say anything about performance: four fixtures the published base cannot
compile are dropped from every run and would read as missing entries once a
folding base ships, and the pinned runner image is rebuilt about every eight
days. The ceilings, the corpus and both blockers are in
[`../bench/ticket-20.md`](../bench/ticket-20.md), so the LTO figures are
captured before they drift, which is what this ticket asked for.

**One reading in the older comment is superseded.** The musl artifact read
1.13-1.53x slower there. That was before ticket 40 gave musl an allocator,
and every run in the new corpus is after it.

**The budget is calibrated.** Three dispatches with
`previous-version=0.19.0-rc.2` removed the reason 61 of 65 fixtures could be
measured: the published base can fold `trim` and `Math.sign` now, so no
fixture leaves the run. The corpus is ten clean runs, three CPU models and
two image builds.

The rule about the image build was the other half, and it changed rather
than being satisfied. An unknown build is a diagnostic now, not a failure,
which is the treatment the CPU model already had and for the same cause:
GitHub chooses the machine and rebuilds the image more often than the
project releases. The image family stays a failure.

**One finding came out of the seeding runs and is not in this ticket.** The
two macOS legs of the paired benchmark stop with a segmentation fault when
the base and the candidate both link mimalloc, which the three dispatches
were the first runs to do. A release takes its base from the newest stable
tag, and that is `0.18.6`, which has no mimalloc, so the next release still
passes; the first one to stop is the one after `0.19.0` ships —
[ticket 53](./53-macos-paired-benchmark-segfaults.md). It does not affect
the ceilings, which come from `x86_64-unknown-linux-gnu`.

**The review round is recorded.** Two agents read the calibration. Both
reproduced every number independently. They found one defect in behaviour —
an absent runner image build passed when it should fail — and six
statements that the code did not support, all corrected in `91475d8d9`.
Section 4 of [`../bench/ticket-20.md`](../bench/ticket-20.md) lists them.

Two more findings became tickets:
[ticket 54](./54-a-new-fixture-stops-the-next-release.md), because an
enforced budget and `--allow-base-refusals` together stop one release
whenever a fixture is added, and
[ticket 55](./55-unplugin-concurrent-css-test-is-flaky.md) for a flaky test
seen while the gates ran.
