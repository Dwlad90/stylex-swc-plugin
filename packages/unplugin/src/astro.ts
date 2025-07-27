import type { UnpluginStylexRSOptions } from './types';

import unplugin from './index';

type VitePlugin = ReturnType<typeof unplugin.vite>;

export default (options: UnpluginStylexRSOptions) => ({
  name: '@stylexswc/unplugin/astro',
  hooks: {
    'astro:config:setup': async (astro: { config: { vite: { plugins: VitePlugin[] } } }) => {
      astro.config.vite.plugins ||= [];
      astro.config.vite.plugins.push(unplugin.vite(options));
    },
  },
});
