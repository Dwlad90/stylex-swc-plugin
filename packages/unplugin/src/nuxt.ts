import { addVitePlugin, addWebpackPlugin, defineNuxtModule } from '@nuxt/kit';
import vite from './vite';
import webpack from './webpack';
import type { UnpluginStylexRSOptions } from './types';
import type { NuxtModule } from '@nuxt/schema';
import '@nuxt/schema';

export type ModuleOptions = UnpluginStylexRSOptions;

// Annotated explicitly for `isolatedDeclarations`. `defineNuxtModule` is
// overloaded, so `ReturnType` would resolve to the builder form returned by
// the no-argument overload rather than the module returned here.
const module: NuxtModule<ModuleOptions, ModuleOptions, false> = defineNuxtModule<ModuleOptions>({
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
