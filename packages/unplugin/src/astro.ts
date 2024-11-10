import type { UnpluginStylexRSOptions } from './types'

import unplugin from './index'

export default (options: UnpluginStylexRSOptions): any => ({
  name: 'unplugin-starter',
  hooks: {
    'astro:config:setup': async (astro: any) => {
      astro.config.vite.plugins ||= []
      astro.config.vite.plugins.push(unplugin.vite(options))
    },
  },
})
