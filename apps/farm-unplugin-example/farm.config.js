import { defineConfig } from '@farmfe/core'
import styleXRSPlugin from '@stylexswc/unplugin/farm'
import { SourceMaps } from '@stylexswc/rs-compiler'

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
      useCSSLayers: true,
      rsOptions: {
        sourceMap: SourceMaps.Inline,
        dev: true,
        // this must set `true` in farm
        treeshakeCompensation: true,
        runtimeInjection: true
      },
    }),
  ],
})