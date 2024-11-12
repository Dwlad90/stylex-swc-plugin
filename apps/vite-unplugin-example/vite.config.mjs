import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import styleXRSPlugin from '@stylexswc/unplugin/vite'

export default defineConfig({
  build: {
    outDir: 'dist',
  },
  plugins: [
    react(),
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