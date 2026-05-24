import { test, expect } from '@stylexswc/playwright';

const ROUTES = [
  { path: '/', screenshot: 'full-page.png' },
  { path: '/theming-demos', screenshot: 'theming-demos.png' },
  { path: '/nested-demo', screenshot: 'nested-demo.png' },
  { path: '/ds-demo', screenshot: 'ds-demo.png' },
] as const;

test.describe('StyleX Visual Regression', () => {
  test.setTimeout(15_000);

  for (const route of ROUTES) {
    test(`should render ${route.path} styling correctly`, async ({ page, screenshotOptions }) => {
      await page.goto(route.path);

      await page.waitForSelector('body', { state: 'visible' });

      await expect(page).toHaveScreenshot(route.screenshot, screenshotOptions);
    });
  }
});
