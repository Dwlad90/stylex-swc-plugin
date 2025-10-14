This is a [Storybook](https://storybook.js.org/) project demonstrating StyleX
integration with component documentation and visual testing.

## Getting Started

First, run the development server:

```bash
npm run storybook
# or
yarn storybook
# or
pnpm storybook
```

Open [http://localhost:6006](http://localhost:6006) with your browser to see the
Storybook interface.

You can start editing the components by modifying files in the `stories/`
directory. Storybook will automatically reload as you edit the files.

## Testing

This project includes both snapshot tests and visual regression tests:

### Snapshot Tests

Run unit tests with snapshot testing:

```bash
npm run test
# or
pnpm test
```

Update snapshots after intentional changes:

```bash
npm run test:update
# or
pnpm test:update
```

### Visual Regression Tests

Run Playwright visual tests to capture and compare component screenshots:

```bash
# First, build Storybook
npm run build

# Run visual tests
npm run test:visual
# or
pnpm test:visual
```

Update visual baselines after intentional UI changes:

```bash
npm run test:visual -- --update-snapshots
# or
pnpm test:visual --update-snapshots
```

Visual test results are available at `visual-tests/playwright-report/`.

## Building for Production

To build the static Storybook for deployment:

```bash
npm run build
# or
pnpm build
```

The static build will be output to `storybook-static/`.

## Learn More

To learn more about Storybook and StyleX, take a look at the following
resources:

- [Storybook Documentation](https://storybook.js.org/docs) - learn about
  Storybook features and API.
- [StyleX Documentation](https://stylexjs.com/docs) - learn about StyleX
  features and usage.
- [Learn Storybook](https://storybook.js.org/tutorials) - interactive Storybook
  tutorials.
- [Playwright Documentation](https://playwright.dev/) - learn about visual
  testing with Playwright.

You can check out
[the Storybook GitHub repository](https://github.com/storybookjs/storybook) and
[the StyleX GitHub repository](https://github.com/facebook/stylex) - your
feedback and contributions are welcome!
