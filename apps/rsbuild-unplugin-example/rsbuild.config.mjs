import { defineConfig } from '@rsbuild/core'
import { pluginReact } from '@rsbuild/plugin-react'
import styleXRSPlugin from '@stylexswc/unplugin/rspack'

export default defineConfig({
  plugins: [pluginReact()],
  html: {
    template: './public/index.html',
  },
  tools: {
    rspack: {
      plugins: [
        styleXRSPlugin({
          useCSSLayers: true,
          rsOptions: {
            dev: true,
            treeshakeCompensation: true,
          },
        }),
      ],
    },
  },
})