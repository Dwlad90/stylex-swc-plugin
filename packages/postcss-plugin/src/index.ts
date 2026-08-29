import { exportAsCommonJs } from '@stylexswc/plugin-shared/cjs-interop';

import createPlugin from './plugin';

const plugin = createPlugin();

export default plugin;

// PostCSS reads a plugin with `require`, so `module.exports` must be the plugin
// itself. A bare write breaks the file when it is read as an ES module.
exportAsCommonJs(typeof module === 'undefined' ? undefined : module, plugin);
