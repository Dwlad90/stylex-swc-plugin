import type { JsPlugin } from '@farmfe/core';
import { createFarmPlugin } from 'unplugin';

import { unpluginFactory } from './index';
import type { UnpluginStylexRSOptions } from './index';

const plugin: (options?: UnpluginStylexRSOptions) => JsPlugin = createFarmPlugin(unpluginFactory);
export default plugin;
