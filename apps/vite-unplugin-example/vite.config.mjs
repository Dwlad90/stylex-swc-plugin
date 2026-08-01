import styleXRSPlugin from '@stylexswc/unplugin/vite';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

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
          type: 'commonJS',
        },
        env: {
          tokens: {
            layout: {
              fullWidth: '100vw',
              fullHeight: '100vh',
            },
          },
          wrapper: value => `${value}`,
        },
      },
    }),
  ],
});
