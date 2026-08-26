import { test, expect } from '@stylexswc/playwright';

test.describe('StyleX Visual Regression', () => {
  test('should render styling correctly', async ({ page, screenshotOptions }) => {
    await page.goto('/');

    await page.waitForSelector('body', { state: 'visible' });

    // The lazily loaded card is what proves rules collected after the
    // placeholder was processed still reach the stylesheet.
    await page.waitForSelector('[data-testid="late-card"]', { state: 'visible' });

    await expect(page).toHaveScreenshot('full-page.png', screenshotOptions);
  });
});
