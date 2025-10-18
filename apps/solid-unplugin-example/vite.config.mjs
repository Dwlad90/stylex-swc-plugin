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
      useCSSLayers: true,
      rsOptions: {
        dev: true,
        treeshakeCompensation: true,
      },
    }),
  ],
})