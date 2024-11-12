import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import styleXRSVitePlugin from '@stylexswc/unplugin/vite'

export default defineConfig({
  build: {
    outDir: 'dist',
  },
  plugins: [
    solid(),
    styleXRSVitePlugin({
      rsOptions: {
        dev: true,
        useCSSLayers: true,
        genConditionalClasses: true,
        treeshakeCompensation: true,
      },
    }),
  ],
})