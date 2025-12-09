// import path from 'node:path';
// import { fileURLToPath } from 'node:url';
import styleXRSPlugin from '@stylexswc/unplugin/vite';
import react from '@vitejs/plugin-react-swc';

import { defineConfig } from 'vitest/config';

// import { storybookTest } from '@storybook/addon-vitest/vitest-plugin';

// const dirname =
//   // @ts-expect-error - its a valid type
//   typeof __dirname !== 'undefined' ? __dirname : path.dirname(fileURLToPath(import.meta.url));

// More info at: https://storybook.js.org/docs/next/writing-tests/integrations/vitest-addon
export default defineConfig({
  test: {
    projects: [
      {
        extends: true,
        plugins: [
          // The plugin will run tests for the stories defined in your Storybook config
          // See options at: https://storybook.js.org/docs/next/writing-tests/integrations/vitest-addon#storybooktest
          react({}),
          // NOTE: Disabled due to issues with Storybook,
          // Dynamic require of "path" is not supported
          // storybookTest({ configDir: path.join(dirname, '.storybook') }),
          styleXRSPlugin({
            pageExtensions: ['tsx', 'jsx', 'js', 'ts', 'vue'],
            useCSSLayers: true,
            rsOptions: {
              dev: true,
              treeshakeCompensation: true,
              enableDebugClassNames: true,
            },
          }),
        ],
        test: {
          name: 'storybook',
          browser: {
            enabled: true,
            headless: true,
            provider: 'playwright',
            instances: [{ browser: 'chromium' }],
          },
          setupFiles: ['.storybook/vitest.setup.ts'],
        },
      },
      {
        extends: true,
        plugins: [
          react({}),
          styleXRSPlugin({
            pageExtensions: ['tsx', 'jsx', 'js', 'ts', 'vue'],
            useCSSLayers: true,
            rsOptions: {
              dev: true,
              treeshakeCompensation: true,
              enableDebugClassNames: true,
            },
          }),
        ],
        test: {
          name: 'snapshots',
          include: ['stories/**/*.test.tsx'],
          environment: 'jsdom',
          setupFiles: ['.storybook/vitest.setup.ts'],
        },
      },
    ],
  },
});
