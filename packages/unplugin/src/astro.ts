import type { UnpluginStylexRSOptions } from './types';

import unplugin from './index';
import { Plugin } from 'vite';

export default (options: UnpluginStylexRSOptions) => ({
  name: 'unplugin-starter',
  hooks: {
    'astro:config:setup': async (astro: {
      config: { vite: { plugins: (Plugin<unknown> | Plugin<unknown>[])[] } };
    }) => {
      astro.config.vite.plugins ||= [];
      astro.config.vite.plugins.push(unplugin.vite(options));
    },
  },
});
