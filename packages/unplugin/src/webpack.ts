import { createWebpackPlugin } from 'unplugin';
import type { UnpluginInstance } from 'unplugin';
import { unpluginFactory } from './index';
import type { UnpluginStylexRSOptions } from './types';

// Annotated explicitly so the declaration can be emitted without type
// inference, which `isolatedDeclarations` requires. The type is taken from the
// shared `UnpluginInstance`, so it stays identical to the inferred one.
const plugin: UnpluginInstance<UnpluginStylexRSOptions | undefined, boolean>['webpack'] =
  createWebpackPlugin(unpluginFactory);

export default plugin;
