import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import styleXRSPlugin from '@toss/stylexswc-unplugin/vite'

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
        treeshakeCompensation: true,
        unstable_moduleResolution: {
          type: "commonJS",
        },
      },
    }),
  ],
})