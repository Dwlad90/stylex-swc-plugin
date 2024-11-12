import path from 'node:path'
import { fileURLToPath } from 'node:url'
import styleXRSPlugin from '@stylexswc/unplugin/esbuild'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

export const config = {
  entryPoints: [path.resolve(__dirname, '..', 'src/index.tsx')],
  bundle: true,
  minify: true,
  outfile: 'dist/output.js',
  /* HACK: for @stylexjs/open-props to work, you need to inject the following paths
  * because after the transformation external props imports are not used anymore,
  * so EsBuild not transpile them, so you need to inject them manually.
  * Description: https://esbuild.github.io/api/#non-analyzable-imports
  */
  inject: [
    '@stylexjs/open-props/lib/colors.stylex',
    '@stylexjs/open-props/lib/fonts.stylex',
    '@stylexjs/open-props/lib/sizes.stylex',
  ],
  plugins: [
    styleXRSPlugin({
      fileName: 'dist/stylex.css',
      rsOptions: {
        useCSSLayers: true,
        genConditionalClasses: true,
        treeshakeCompensation: true,
        stylexImports: ['@stylexjs/stylex'],
      },
    })
  ],
}