import autoprefixer from 'autoprefixer'
import nesting from 'postcss-nesting'
import stylexSWCPlugin from '@stylexswc/postcss-plugin'

/** @type {import('postcss-load-config').Config} */
const config = {
  plugins: [
    nesting,
    stylexSWCPlugin({
      include: ['stories/**/*.{ts,tsx}'],
      useCSSLayers: process.env.NODE_ENV !== 'production'
    }),
    autoprefixer
  ]
}

export default config
