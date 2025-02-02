import { addVitePlugin, addWebpackPlugin, defineNuxtModule } from '@nuxt/kit';
import vite from './vite';
import webpack from './webpack';
import type { UnpluginStylexRSOptions } from './types';
import '@nuxt/schema';

export interface ModuleOptions extends UnpluginStylexRSOptions {}

export default defineNuxtModule<ModuleOptions>({
  meta: {
    name: 'nuxt-unplugin-starter',
    configKey: 'unpluginStarter',
  },
  defaults: {
    // ...default options
  },
  setup(options, _nuxt) {
    addVitePlugin(() => vite(options));
    addWebpackPlugin(() => webpack(options));

    // ...
  },
});
