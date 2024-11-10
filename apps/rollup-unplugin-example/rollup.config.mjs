import { readFileSync } from 'node:fs'
import swc from '@rollup/plugin-swc'
import commonjs from '@rollup/plugin-commonjs'
import html, { makeHtmlAttributes } from '@rollup/plugin-html'
import nodeResolve from '@rollup/plugin-node-resolve'
import replace from '@rollup/plugin-replace'
import serve from 'rollup-plugin-serve'
import styleXRSRollupPlugin from '@stylexswc/unplugin/rollup'

export default {
  input: 'src/index.jsx',
  output: {
    file: 'dist/bundle.js',
    format: 'iife',
  },
  plugins: [
    nodeResolve({
      extensions: ['.js', '.jsx'],
    }),
    commonjs(),
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
    replace({
      preventAssignment: false,
      'process.env.NODE_ENV': '"development"',
    }),
    styleXRSRollupPlugin({
      dev: true,
      rsOptions: {
        useCSSLayers: true,
        genConditionalClasses: true,
        treeshakeCompensation: true,
      },
    }),
    serve({
      contentBase: ['dist'],
      host: '127.0.0.1',
      port: 8081,
    }),
    html({
      publicPath: '/',
      title: 'Stylex With Rollup',
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