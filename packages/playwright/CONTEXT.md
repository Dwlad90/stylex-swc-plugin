# @stylexswc/playwright

The shared Playwright configuration behind every example app's visual
regression suite.

## Language

**Visual snapshot**:
A committed reference screenshot, compared against on every run. The directory
is `SNAPSHOT_DIR`, default `visual-tests/.playwright-snapshots`, and the path
template is keyed by `{platform}`, so snapshots are per-OS. Comparison is
zero-tolerance. Regenerating is deliberate: `PLAYWRIGHT_UPDATE_SNAPSHOTS`, via
`pnpm test:visual:update`, against `updateSnapshots: 'none'` on CI.
_Avoid_: baseline, golden, screenshot, fixture

**Port resolution**:
`resolvePort(defaultPort)` — reads `PORT` and rejects anything that is not an
integer in 1–65535. Port `0` is rejected too: the OS resolves it inside the web
server while `baseURL` stays pinned to `0`. Exported rather than applied once
here, because every app config reads `PORT` itself with a different default.
_Avoid_: port parsing, env lookup, config

**Web server block**:
The `webServer` options, exported separately from the config. `defineConfig`
widens it to an object-or-array, so spreading it off the default export yields
numeric index keys; consumers must spread this value.
_Avoid_: server config, dev server
