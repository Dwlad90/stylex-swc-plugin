import { defineConfig } from '@rsbuild/core'
import { pluginReact } from '@rsbuild/plugin-react'
import styleXRSPlugin from '@toss/stylexswc-unplugin/rspack'

export default defineConfig({
  plugins: [pluginReact()],
  html: {
    template: './public/index.html',
  },
  tools: {
    rspack: {
      plugins: [
        styleXRSPlugin({
          rsOptions: {
            dev: true,
            useCSSLayers: true,
            treeshakeCompensation: true,
          },
        }),
      ],
    },
  },
})