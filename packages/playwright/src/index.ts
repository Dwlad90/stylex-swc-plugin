import { defineConfig, devices, test as base, expect } from '@playwright/test';
import type { PageAssertionsToHaveScreenshotOptions } from '@playwright/test';

const snapshotDir = process.env.SNAPSHOT_DIR || 'visual-tests/.playwright-snapshots';

const DEFAULT_PORT = 3000;

/**
 * Reads `PORT`, falling back to `defaultPort`.
 *
 * A bare `+process.env.PORT` turns any non-numeric value into `NaN`, which then
 * propagates silently into `baseURL` as `http://localhost:NaN` and into the web
 * server's port. Every visual test then fails to connect, with nothing pointing
 * at the environment as the cause.
 *
 * Exported because every app config overrides `baseURL` and `webServer.port`
 * with its own `PORT` reading — each app listens on a different default — so a
 * guard applied only to this module's own constant would never run for any real
 * consumer. `defaultPort` is what lets those configs keep their own default
 * while sharing the validation.
 */
export function resolvePort(defaultPort: number = DEFAULT_PORT): number {
  const raw = process.env.PORT;
  if (raw === undefined || raw === '') return defaultPort;

  // Lower bound is 1, not 0: port 0 is "pick any free port", which the OS
  // resolves inside the web server while `baseURL` stays pinned to
  // `http://localhost:0`. That is exactly the silent mismatch this guard
  // exists to prevent.
  const port = Number(raw);
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    throw new Error(`Invalid PORT environment variable: ${JSON.stringify(raw)}`);
  }

  return port;
}

const PORT = resolvePort();

const isCI = !!process.env.CI;
const shouldUpdateSnapshots =
  process.env.PLAYWRIGHT_UPDATE_SNAPSHOTS === 'true' ||
  process.env.PLAYWRIGHT_UPDATE_SNAPSHOTS === '1';

/**
 * Exported separately from the config below because `defineConfig` widens
 * `webServer` to `TestConfigWebServer | TestConfigWebServer[]`. Consumers that
 * spread it off the default export are therefore spreading a possible array,
 * which yields numeric index keys rather than the server options. Spreading
 * this value instead keeps the narrow object type.
 */
export const webServer = {
  command: 'pnpm run serve',
  port: PORT,
  reuseExistingServer: !isCI,
  timeout: 30000, // 30 seconds
};

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
  webServer,
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
  // oxlint-disable-next-line no-empty-pattern
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
