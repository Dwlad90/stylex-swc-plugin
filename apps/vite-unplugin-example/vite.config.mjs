import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import styleXRSVitePlugin from '@stylexswc/unplugin/vite'

export default defineConfig({
  build: {
    outDir: 'dist',
  },
  plugins: [
    react(),
    styleXRSVitePlugin({
      dev: true,
      rsOptions: {
        useCSSLayers: true,
        genConditionalClasses: true,
        treeshakeCompensation: true,
      },
    }),
  ],
})