import { defineConfig } from '@playwright/test';
import defaultConfig from '@toss/stylexswc-playwright';

const PORT = +(process.env.PORT || 3002);

export default defineConfig({
  ...defaultConfig,
  use: {
    ...defaultConfig.use,
    baseURL: `http://localhost:${PORT}`,
  },
  webServer: defaultConfig.webServer
    ? {
        ...(defaultConfig.webServer || {}),
        port: PORT,
      }
    : undefined,
});
