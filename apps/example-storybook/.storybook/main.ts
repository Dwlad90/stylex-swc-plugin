import type { UserConfig } from 'vite';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { defineMain } from '@storybook/react-vite/node';

// @ts-expect-error - its a valid type
const __dirname = dirname(fileURLToPath(import.meta.url));

export default defineMain({
  stories: ['../stories/**/*.mdx', '../stories/**/*.stories.@(js|jsx|mjs|ts|tsx)'],
  addons: ['@storybook/addon-links', '@storybook/addon-docs', '@chromatic-com/storybook'],
  framework: {
    name: '@storybook/react-vite',
    options: {},
  },

  typescript: {
    /* Infer property docs by parsing the source rather than by type-checking
    it. `react-docgen-typescript` drives the JavaScript TypeScript compiler
    API, which TypeScript 7 no longer ships. `react-docgen` is Babel-based and
    needs no compiler API, at the cost of resolving fewer inherited and
    computed prop types. */
    reactDocgen: 'react-docgen',
  },

  async viteFinal(config) {
    /* use a different config for static build for self-contained setup to
    include external deps (like react) into the served package */
    const { mergeConfig } = await import('vite');
    const configPath = join(__dirname, '../vite-storybook.config.ts');
    const viteConfig = await import(configPath);

    return mergeConfig(config, {
      plugins: viteConfig.plugins,
    } as UserConfig);
  },

  core: {
    disableTelemetry: true,
  },
});
