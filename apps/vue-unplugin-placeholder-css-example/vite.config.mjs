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
      // Keep Vue SFC processing on virtual script requests (`lang.ts`), not raw `.vue` documents.
      pageExtensions: ['tsx', 'jsx', 'js', 'ts'],
      useCssPlaceholder: true,
      rsOptions: {
        dev: true,
        useCSSLayers: true,
        treeshakeCompensation: true,
        env: {
          tokens: {
            layout: {
              fullWidth: '100vw',
              fullHeight: '100vh',
            },
          },
          wrapper: (value) => `${value}`,
        },
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
