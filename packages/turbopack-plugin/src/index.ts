import { exportAsCommonJs } from '@stylexswc/plugin-shared/cjs-interop';

function StyleXTurbopackPlugin() {
  throw new Error(
    `
    Turbopack does not support webpack plugins. This affects third-party tools that rely on webpack's plugin system for integration. We do support webpack loaders. If you depend on webpack plugins, you'll need to find Turbopack-compatible alternatives or continue using webpack until equivalent functionality is available.

    https://nextjs.org/docs/app/api-reference/turbopack#webpack-plugins
`
  );
}

export default StyleXTurbopackPlugin;

// Next.js reads the plugin with `require`, so `module.exports` must be the
// function itself. A bare write breaks the file when it is read as an ES
// module.
exportAsCommonJs(typeof module === 'undefined' ? undefined : module, StyleXTurbopackPlugin);
