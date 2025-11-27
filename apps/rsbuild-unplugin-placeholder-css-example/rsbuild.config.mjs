import { defineConfig } from '@rsbuild/core'
import { pluginReact } from '@rsbuild/plugin-react'
import styleXRSPlugin from '@stylexswc/unplugin/rspack'
import rspack from '@rspack/core'

export default defineConfig({
  plugins: [pluginReact()],
  html: {
    template: './public/index.html',
  },
  optimization: {
    minimize: true, // Ensure minification is enabled
    minimizer: [
      // If using a specific CSS minifier plugin
      new rspack.LightningCssMinimizerRspackPlugin({
        // options for cssnano or Lightning CSS
      }),
    ],
  },
  tools: {
    rspack: {
      plugins: [
        styleXRSPlugin({
          useCSSLayers: true,
          useCssPlaceholder: true,
          rsOptions: {
            dev: true,
            treeshakeCompensation: true,
          },
        }),
      ],
    },
  },
})