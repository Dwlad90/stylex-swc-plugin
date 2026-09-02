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

**Status:** backlog

- [ ] The release workflow is observed once end to end, and the per-target
      build times are recorded.
- [ ] `npm.yml` carries a `timeout-minutes` that matches the measured cost, or
      a note saying why the default is enough.
- [ ] `budget.json` carries calibrated ceilings taken from an LTO build.
