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
      useCssPlaceholder: true,
      rsOptions: {
        dev: true,
        useCSSLayers: true,
        treeshakeCompensation: true,
      },
    }),
    // add lightning css transformer
  ],
  css: {
    transformer: 'lightningcss',
    lightningcss: {
      targets: {
        // Your browserslist targets
        chrome: 129,
      },
    },
  },
})