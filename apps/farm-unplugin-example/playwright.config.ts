import { defineConfig } from '@playwright/test';
import defaultConfig from '@stylexswc/playwright';

const PORT = +(process.env.PORT || 3003);

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
