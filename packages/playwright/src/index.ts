import { defineConfig, devices } from '@playwright/test';
import { test as base, expect } from '@playwright/test';
import type { PageAssertionsToHaveScreenshotOptions } from '@playwright/test';

const snapshotDir = process.env.SNAPSHOT_DIR || 'visual-tests/.playwright-snapshots';

const PORT = +(process.env.PORT || 3000);

const isCI = !!process.env.CI;
const shouldUpdateSnapshots =
  process.env.PLAYWRIGHT_UPDATE_SNAPSHOTS === 'true' ||
  process.env.PLAYWRIGHT_UPDATE_SNAPSHOTS === '1';

export default defineConfig({
  testDir: './visual-tests',
  outputDir: 'visual-tests/test-results',
  timeout: 5000,
  fullyParallel: true,
  forbidOnly: isCI,
  retries: isCI ? 1 : 0,
  workers: isCI ? 1 : undefined,
  reporter: [
    ['html', { outputFolder: 'visual-tests/playwright-report', open: 'never' }],
    ['list', { printSteps: true }],
  ],
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
      maxDiffPixelRatio: 0,
      threshold: 0,
      pathTemplate: `${snapshotDir}/{platform}/{testFilePath}_{projectName}_{arg}{ext}`,
    },
  },
  updateSnapshots: shouldUpdateSnapshots ? 'all' : isCI ? 'none' : 'missing',
});

export const test = base.extend<{
  screenshotOptions: PageAssertionsToHaveScreenshotOptions;
}>({
  // eslint-disable-next-line no-empty-pattern
  screenshotOptions: async ({}, use) => {
    const options: PageAssertionsToHaveScreenshotOptions = {
      fullPage: true,
      animations: 'disabled',
      maxDiffPixelRatio: 0,
      threshold: 0,
    };

    await use(options);
  },
});

export { expect };
