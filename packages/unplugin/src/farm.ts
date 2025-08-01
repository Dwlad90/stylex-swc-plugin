import { createFarmPlugin } from 'unplugin';
import { unpluginFactory } from './index';
import type { UnpluginStylexRSOptions } from './index';
import type { JsPlugin } from '@farmfe/core';

const plugin: (options?: UnpluginStylexRSOptions) => JsPlugin = createFarmPlugin(unpluginFactory);
export default plugin;
