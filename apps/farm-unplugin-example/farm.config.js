import { defineConfig } from '@farmfe/core'
import styleXRSFarmPlugin from '@stylexswc/unplugin/farm'
import {} from '@stylexswc/rs-compiler'

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
    styleXRSFarmPlugin({
      rsOptions: {
        sourceMap: "Inline",
        dev: true,
        // debug: true,
        // runtimeInjection: false,

        useCSSLayers: true,
        // this must set `true` in farm
        treeshakeCompensation: true,
      },
    }),
  ],
})