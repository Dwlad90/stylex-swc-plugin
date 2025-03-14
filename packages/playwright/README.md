# @stylexswc/playwright

Playwright test configuration that enables visual regression testing for StyleX applications.

## Overview

The `@stylexswc/playwright` package provides a pre-configured setup for Playwright visual testing
that works seamlessly with StyleX. This allows you to write visual regression tests for
your StyleX components, ensuring your UI remains consistent across browsers and devices.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/playwright
```

## Configuration

The package exports a default Playwright configuration that you can use or extend. Here's an example:

```typescript
// playwright.config.ts
import { defineConfig } from '@stylexswc/playwright';

export default defineConfig({
  // You can override any default options here
  testDir: './your-test-directory',
  use: {
    baseURL: 'http://localhost:8080',
  },
});
```

## Usage

The package provides extended test functions with custom screenshot capabilities:

```typescript
import { test, expect } from '@stylexswc/playwright';

test('component renders correctly', async ({ page, screenshotOptions }) => {
  await page.goto('/my-component');
  await expect(page).toHaveScreenshot('my-component.png', screenshotOptions);
});
```

## Default Configuration

The package includes the following default configuration:

- Tests run in Chrome desktop and mobile viewports
- Screenshots are saved to `.playwright-snapshots` directory
- CI-specific settings when the CI environment variable is set
- Configured thresholds for screenshot comparisons
- A dev server started with `pnpm run serve`

## Advanced Configuration

You can customize the configuration further:

```typescript
// playwright.config.ts
import { defineConfig } from '@stylexswc/playwright';

export default defineConfig({
  // Custom snapshot directory
  snapshotDir: 'your-snapshots-directory',

  // Additional browsers
  projects: [
    {
      name: 'Firefox Desktop',
      use: {
        browserName: 'firefox',
        viewport: { width: 1280, height: 720 },
      },
    },
  ],

  // Custom web server
  webServer: {
    command: 'npm run start',
    port: 4000,
  },
});
```

## Example

A complete example of a visual test suite:

```typescript
import { test, expect } from '@stylexswc/playwright';

test.describe('Button component', () => {
  test('renders in default state', async ({ page, screenshotOptions }) => {
    await page.goto('/components/button');
    await expect(page).toHaveScreenshot('button-default.png', screenshotOptions);
  });

  test('renders in hover state', async ({ page, screenshotOptions }) => {
    await page.goto('/components/button');
    await page.hover('button');
    await expect(page).toHaveScreenshot('button-hover.png', screenshotOptions);
  });
});
```

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE)
file for details.
