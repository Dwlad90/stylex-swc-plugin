import autoprefixer from 'autoprefixer'
import nesting from 'postcss-nesting'
import stylexSWCPlugin from '@stylexswc/postcss-plugin'

/** @type {import('postcss-load-config').Config} */
const config = {
  plugins: [
    nesting,
    stylexSWCPlugin({
      include: ['stories/**/*.{ts,tsx}'],
      useCSSLayers: process.env.NODE_ENV !== 'production',
      rsOptions: {
        dev: true,
        debug: true,
        treeshakeCompensation: true,
        enableDebugClassNames: true,
        enableDevClassNames: true,
        enableDebugDataProp: true,
        env: {
          tokens: {
            colors: {
              background: 'white',
              text: '#333',
            },
          },
          wrapper: (value) => `${value}`,
        },
      },
    }),
    autoprefixer
  ]
}

export default config
