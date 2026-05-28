# Playwright test configuration with NAPI-RS StyleX compiler integration

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

Playwright test configuration that enables visual regression testing for StyleX
applications.

## Overview

The `@stylexswc/playwright` package provides a pre-configured setup for
Playwright visual testing that works seamlessly with StyleX. This allows you to
write visual regression tests for your StyleX components, ensuring your UI
remains consistent across browsers and devices.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/playwright
```

## Configuration

The package exports a default Playwright configuration that you can use or
extend. Here's an example:

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

The package provides extended test functions with custom screenshot
capabilities:

```typescript
import { test, expect } from '@stylexswc/playwright';

test('component renders correctly', async ({ page, screenshotOptions }) => {
  await page.goto('/my-component');
  await expect(page).toHaveScreenshot('my-component.png', screenshotOptions);
});
```

## Updating snapshots

In GitHub Actions, run the **Update Visual Snapshots** workflow manually from a
feature or PR branch to update and push Linux snapshots from the same ARM64
Playwright Docker image used for CI snapshot generation.

Locally, update snapshots for your current OS with:

```bash
pnpm test:visual -- --update-snapshots
```

For CI-equivalent Linux snapshots, use the Docker helper. It defaults to
`linux/arm64`, matching the visual CI runners and Apple Silicon Docker:

```bash
pnpm test:visual:update up
pnpm test:visual:update run
pnpm test:visual:update check
```

## Default Configuration

The package includes the following default configuration:

- Tests run in Chrome desktop and mobile viewports
- Screenshots are saved to OS-specific `.playwright-snapshots/<platform>`
  directories
- CI-specific settings when the CI environment variable is set
- Strict screenshot comparisons with no diff tolerance
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
    await expect(page).toHaveScreenshot(
      'button-default.png',
      screenshotOptions
    );
  });

  test('renders in hover state', async ({ page, screenshotOptions }) => {
    await page.goto('/components/button');
    await page.hover('button');
    await expect(page).toHaveScreenshot('button-hover.png', screenshotOptions);
  });
});
```

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
