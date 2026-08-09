# @stylexswc/playwright

The shared Playwright configuration behind every example app's visual
regression suite. Each app extends it and overrides the parts that are its own.

## Language

**Visual snapshot**:
A committed reference screenshot under the snapshot directory, compared against
on every run. Regenerated deliberately (`pnpm test:visual:update`), never as a
side effect of a failing run.
_Avoid_: baseline, golden, screenshot, fixture

**Port resolution**:
`resolvePort(defaultPort)` — reading `PORT` with validation. Exported rather
than applied once here because every app config reads `PORT` itself, each with a
different default, so a guard on this module's own constant would never run for
a real consumer. A bare `+process.env.PORT` would silently yield
`http://localhost:NaN`, and port `0` is rejected too: the OS resolves it inside
the web server while `baseURL` stays pinned to `0`.
_Avoid_: port parsing, env lookup, config

**Web server block**:
The `webServer` options, exported separately from the config. `defineConfig`
widens it to an object-or-array, so spreading it off the default export yields
numeric index keys instead of the server options — consumers must spread this
value.
_Avoid_: server config, dev server
