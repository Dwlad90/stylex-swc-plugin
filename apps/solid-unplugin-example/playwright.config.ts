import { defineConfig } from '@playwright/test';
import defaultConfig, { webServer } from '@stylexswc/playwright';

const PORT = +(process.env.PORT || 3007);

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
