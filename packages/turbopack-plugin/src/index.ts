function StyleXTurbopackPlugin() {
  throw new Error(
    `
    Turbopack does not support webpack plugins. This affects third-party tools that rely on webpack's plugin system for integration. We do support webpack loaders. If you depend on webpack plugins, you'll need to find Turbopack-compatible alternatives or continue using webpack until equivalent functionality is available.

    https://nextjs.org/docs/app/api-reference/turbopack#webpack-plugins
`
  );
}

module.exports = StyleXTurbopackPlugin;
module.exports.default = StyleXTurbopackPlugin;
