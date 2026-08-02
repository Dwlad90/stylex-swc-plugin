import { defineConfig } from '@playwright/test';
// NOTE: `.default` is load-bearing here. These two apps resolve the shared
// config through CommonJS interop, so the module namespace is the wrapper and
// the config sits on `.default`. Spreading `defaultConfig` directly silently
// yields an empty object: Playwright then runs with its own defaults and the
// Chrome Desktop/Mobile projects disappear.
import defaultConfig, { resolvePort, webServer } from '@stylexswc/playwright';

const PORT = resolvePort(3007);

export default defineConfig({
  ...defaultConfig.default,
  use: {
    ...defaultConfig.default.use,
    baseURL: `http://localhost:${PORT}`,
  },
  webServer: {
    ...webServer,
    port: PORT,
  },
});
