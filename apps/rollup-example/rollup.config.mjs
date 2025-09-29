import { readFileSync } from 'node:fs'
import swc from '@rollup/plugin-swc'
import commonjs from '@rollup/plugin-commonjs'
import html, { makeHtmlAttributes } from '@rollup/plugin-html'
import nodeResolve from '@rollup/plugin-node-resolve'
import replace from '@rollup/plugin-replace'
import serve from 'rollup-plugin-serve'
import styleXRSPlugin from '@toss/stylexswc-rollup-plugin'

const isDev = process.env.NODE_ENV !== 'production';


export default {
  input: 'src/index.jsx',
  output: {
    file: 'dist/bundle.js',
    format: 'cjs',
  },
  // See all options in the babel plugin configuration docs:
  // https://stylexjs.com/docs/api/configuration/babel-plugin/
  plugins: [
    nodeResolve({
      extensions: ['.js', '.jsx'],
    }),
    commonjs(),
    replace({
      'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV || 'development'),
      preventAssignment: true,
    }),
    swc({
      swc: {
        jsc: {
          parser: {
            syntax: 'ecmascript',
            jsx: true,
          },
          transform: {
            react: {
              runtime: 'automatic',
            },
          },
        },
      }
    }),
    styleXRSPlugin.default({
      dev: isDev,
      fileName: 'stylex.css',
      rsOptions: {
        useCSSLayers: true,
        treeshakeCompensation: true,
      },
    }),
    isDev &&
    serve({
      contentBase: ['dist'],
      host: '127.0.0.1',
      port: 8081,
    }),
    html({
      publicPath: '/',
      title: 'StyleX With Rollup',
      template: ({ attributes, files, publicPath, title }) => {
        const htmlTemplate = 'public/index.html'
        const { css, js } = files
        const scripts = (js || [])
          .map(
            ({ fileName }) =>
              `<script src="${publicPath}${fileName}" ${makeHtmlAttributes(attributes.script)}></script>`,
          )
          .join('\n')
        const links = (css || [])
          .map(({ fileName }) => `<link rel="stylesheet" href="${publicPath}${fileName}" />`)
          .join('\n')
        const template = readFileSync(htmlTemplate, 'utf-8')
        return template
          .replace(/{title}/g, title)
          .replace(/{links}/g, links)
          .replace(/{scripts}/g, scripts)
      },
    }),
  ],
}