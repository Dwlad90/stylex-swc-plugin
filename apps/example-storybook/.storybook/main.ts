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
    /* infer property docs from typescript types  */
    reactDocgen: 'react-docgen-typescript',
    // @ts-expect-error - its a valid type
    reactDocgenTypescriptOptions: {
      shouldExtractLiteralValuesFromEnum: true,
      shouldRemoveUndefinedFromOptional: true,
      // @ts-expect-error - its a valid type
      propFilter: prop => {
        /* does property have documentation? */
        const hasDoc = prop.description !== '';

        /* is property defined in external dependency package? */
        const isExternal = prop.parent && /node_modules/.test(prop.parent.fileName);

        return hasDoc && !isExternal;
      },
    },
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
