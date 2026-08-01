import { addVitePlugin, addWebpackPlugin, defineNuxtModule } from '@nuxt/kit';
import type { NuxtModule } from '@nuxt/schema';

import type { UnpluginStylexRSOptions } from './types';
import vite from './vite';
import webpack from './webpack';
import '@nuxt/schema';

export type ModuleOptions = UnpluginStylexRSOptions;

// Annotated explicitly for `isolatedDeclarations`. `defineNuxtModule` is
// overloaded, so `ReturnType` would resolve to the builder form returned by
// the no-argument overload rather than the module returned here.
const module: NuxtModule<ModuleOptions, ModuleOptions> = defineNuxtModule<ModuleOptions>({
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

export default module;
