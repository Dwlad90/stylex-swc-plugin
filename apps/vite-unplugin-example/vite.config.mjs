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
      useCSSLayers: true,
      rsOptions: {
        dev: true,
        treeshakeCompensation: true,
        unstable_moduleResolution: {
          type: "commonJS",
        },
      },
    }),
  ],
})