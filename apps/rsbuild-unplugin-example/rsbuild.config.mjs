import { defineConfig } from '@rsbuild/core'
import { pluginReact } from '@rsbuild/plugin-react'
import styleXRSPackPlugin from '@stylexswc/unplugin/rspack'

export default defineConfig({
  plugins: [pluginReact()],
  html: {
    template: './public/index.html',
  },
  tools: {
    rspack: {
      plugins: [
        styleXRSPackPlugin({
          rsOptions: {
            dev: true,
            useCSSLayers: true,
            genConditionalClasses: true,
            treeshakeCompensation: true,
          },
        }),
      ],
    },
  },
})