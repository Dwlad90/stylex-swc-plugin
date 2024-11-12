import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import styleXRSPlugin from '@stylexswc/unplugin/vite'

export default defineConfig({
  build: {
    outDir: 'dist',
  },
  plugins: [
    solid(),
    styleXRSPlugin({
      rsOptions: {
        dev: true,
        useCSSLayers: true,
        genConditionalClasses: true,
        treeshakeCompensation: true,
      },
    }),
  ],
})