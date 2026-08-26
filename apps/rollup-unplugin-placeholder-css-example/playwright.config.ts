import { defineConfig } from '@playwright/test';
import defaultConfig, { resolvePort, webServer } from '@stylexswc/playwright';

const PORT = resolvePort(3021);

export default defineConfig({
  ...defaultConfig,
  use: {
    ...defaultConfig.use,
    baseURL: `http://localhost:${PORT}`,
  },
  webServer: {
    ...webServer,
    port: PORT,
  },
});
