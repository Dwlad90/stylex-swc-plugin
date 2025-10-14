import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import styleXRSPlugin from '@stylexswc/unplugin/vite'

export default defineConfig({
  build: {
    outDir: 'dist',
    cssMinify: 'lightningcss', // Use Lightning CSS for minification in production builds
  },
  plugins: [
    react(),
    styleXRSPlugin({
      useViteCssPipeline: true,
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