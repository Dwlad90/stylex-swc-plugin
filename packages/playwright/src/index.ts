import { defineConfig, devices } from '@playwright/test';
import { test as base, expect } from '@playwright/test';
import type { PageAssertionsToHaveScreenshotOptions } from '@playwright/test';

const snapshotDir = process.env.SNAPSHOT_DIR || 'visual-tests/.playwright-snapshots';

const PORT = +(process.env.PORT || 3000);

const isCI = !!process.env.CI;

export default defineConfig({
  testDir: './visual-tests',
  outputDir: 'visual-tests/test-results',
  timeout: 5000,
  fullyParallel: true,
  forbidOnly: isCI,
  retries: isCI ? 1 : 0,
  workers: isCI ? 1 : undefined,
  reporter: [['html', { outputFolder: 'visual-tests/playwright-report' }], ['list']],
  use: {
    baseURL: `http://localhost:${PORT}`,
    trace: isCI ? 'on-first-retry' : 'on',
    screenshot: isCI ? 'only-on-failure' : 'on',
  },
  projects: [
    {
      name: 'Chrome Desktop',
      use: {
        ...devices['Desktop Chrome'],
        viewport: { width: 1920, height: 1080 },
      },
    },
    {
      name: 'Chrome Mobile',
      use: {
        ...devices['iPhone 15 Pro Max'],
      },
    },
  ],
  webServer: {
    command: 'pnpm run serve',
    port: PORT,
    reuseExistingServer: !isCI,
    timeout: 30000, // 30 seconds
  },
  expect: {
    toHaveScreenshot: {
      maxDiffPixelRatio: 0.03,
      threshold: 0.2,
      pathTemplate: `${snapshotDir}/{testFilePath}_{projectName}_{arg}{ext}`,
    },
  },
  updateSnapshots: isCI ? 'none' : 'missing',
});

export const test = base.extend<{
  screenshotOptions: PageAssertionsToHaveScreenshotOptions;
}>({
  screenshotOptions: async ({ browser }, use) => {
    const isMobile = browser.browserType().name() === 'webkit';

    const options: PageAssertionsToHaveScreenshotOptions = {
      fullPage: true,
      animations: 'disabled',
      threshold: 0.2,
      ...(isMobile
        ? {
            maxDiffPixelRatio: 0.06,
          }
        : {
            maxDiffPixelRatio: 0.03,
          }),
    };

    await use(options);
  },
});

export { expect };
