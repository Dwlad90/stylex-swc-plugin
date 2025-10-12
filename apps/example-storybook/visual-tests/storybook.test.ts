import { test, expect } from '@stylexswc/playwright';

test.describe('Storybook Visual Regression', () => {
  test.describe('Button Component', () => {
    test('should render Primary button correctly', async ({ page, screenshotOptions }) => {
      await page.goto('/iframe?id=example-button--primary&viewMode=story');
      await expect(page).toHaveScreenshot('button-primary.png', screenshotOptions);
    });

    test('should render Secondary button correctly', async ({ page, screenshotOptions }) => {
      await page.goto('/iframe?id=example-button--secondary&viewMode=story');
      await expect(page).toHaveScreenshot('button-secondary.png', screenshotOptions);
    });

    test('should render Danger button correctly', async ({ page, screenshotOptions }) => {
      await page.goto('/iframe?id=example-button--danger&viewMode=story');
      await expect(page).toHaveScreenshot('button-danger.png', screenshotOptions);
    });

    test('should render Small button correctly', async ({ page, screenshotOptions }) => {
      await page.goto('/iframe?id=example-button--small&viewMode=story');
      await expect(page).toHaveScreenshot('button-small.png', screenshotOptions);
    });

    test('should render Medium button correctly', async ({ page, screenshotOptions }) => {
      await page.goto('/iframe?id=example-button--medium&viewMode=story');
      await expect(page).toHaveScreenshot('button-medium.png', screenshotOptions);
    });

    test('should render Large button correctly', async ({ page, screenshotOptions }) => {
      await page.goto('/iframe?id=example-button--large&viewMode=story');
      await expect(page).toHaveScreenshot('button-large.png', screenshotOptions);
    });
  });

  test.describe('Card Component', () => {
    test('should render Default card correctly', async ({ page, screenshotOptions }) => {
      await page.goto('/iframe?id=example-card--default&viewMode=story');
      await expect(page).toHaveScreenshot('card-default.png', screenshotOptions);
    });

    test('should render Elevated card correctly', async ({ page, screenshotOptions }) => {
      await page.goto('/iframe?id=example-card--elevated&viewMode=story');
      await expect(page).toHaveScreenshot('card-elevated.png', screenshotOptions);
    });
  });
});
