import type { UnpluginStylexRSOptions } from './types';

import unplugin from './index';

type VitePlugin = ReturnType<typeof unplugin.vite>;

interface AstroIntegration {
  name: string;
  hooks: {
    'astro:config:setup': (astro: {
      config: { vite: { plugins: VitePlugin[] } };
    }) => Promise<void>;
  };
}

export default (options: UnpluginStylexRSOptions): AstroIntegration => ({
  name: '@stylexswc/unplugin/astro',
  hooks: {
    'astro:config:setup': async (astro: { config: { vite: { plugins: VitePlugin[] } } }) => {
      astro.config.vite.plugins ||= [];
      astro.config.vite.plugins.push(unplugin.vite(options));
    },
  },
});
