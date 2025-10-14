import vue from '@vitejs/plugin-vue'
import styleXRSPlugin from '@stylexswc/unplugin/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  build: {
    outDir: 'dist',
  },
  plugins: [
    vue(),
    styleXRSPlugin({
      pageExtensions: ['tsx', 'jsx', 'js', 'ts', 'vue'],
      useCSSLayers: true,
      rsOptions: {
        dev: true,
        treeshakeCompensation: true,
      },
    }),
  ],
})