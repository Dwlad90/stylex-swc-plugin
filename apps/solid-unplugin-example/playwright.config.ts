import defaultConfig from '@stylexswc/playwright';
import { defineConfig } from '@playwright/test';

const PORT = +(process.env.PORT || 3007);

export default defineConfig({
  ...defaultConfig.default,
  use: {
    ...defaultConfig.default.use,
    baseURL: `http://localhost:${PORT}`,
  },
  webServer: defaultConfig.default.webServer
    ? {
        ...(defaultConfig.default.webServer || {}),
        port: PORT,
      }
    : undefined,
});
