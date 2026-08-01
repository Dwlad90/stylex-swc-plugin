import { defineConfig } from '@farmfe/core';
import { SourceMaps } from '@stylexswc/rs-compiler';
import styleXRSPlugin from '@stylexswc/unplugin/farm';

export default defineConfig({
  compilation: {
    persistentCache: false,
    sourcemap: 'inline',
    input: {
      index: './index.html',
    },
    output: {
      path: 'dist',
      publicPath: '/',
      targetEnv: 'browser',
    },
  },
  server: {
    hmr: true,
  },
  plugins: [
    [
      '@farmfe/plugin-react',
      {
        refresh: true,
        development: true,
        runtime: 'automatic',
      },
    ],
    styleXRSPlugin({
      useCSSLayers: false,
      rsOptions: {
        sourceMap: SourceMaps.Inline,
        dev: true,
        treeshakeCompensation: true,
        // this must set `true` in farm
        runtimeInjection: true,
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
