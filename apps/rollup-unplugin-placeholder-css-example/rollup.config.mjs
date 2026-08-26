import { readFileSync } from 'node:fs';

import commonjs from '@rollup/plugin-commonjs';
import html, { makeHtmlAttributes } from '@rollup/plugin-html';
import nodeResolve from '@rollup/plugin-node-resolve';
import replace from '@rollup/plugin-replace';
import swc from '@rollup/plugin-swc';
import styleXRSPlugin from '@stylexswc/unplugin/rollup';

/**
 * Stands in for a CSS plugin: Rollup has no CSS pipeline of its own, so the
 * stylesheet carrying the `@stylex;` marker is emitted as a plain asset. The
 * StyleX plugin then replaces the marker with the collected rules.
 */
function emitStylesheet() {
  return {
    name: 'emit-stylesheet',
    buildEnd() {
      this.emitFile({
        type: 'asset',
        fileName: 'styles.css',
        source: readFileSync('src/styles.css', 'utf-8'),
      });
    },
  };
}

export default {
  input: 'src/index.jsx',
  output: {
    // A directory and ES modules, because the lazily loaded card is a chunk of
    // its own.
    dir: 'dist',
    format: 'es',
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
      },
    }),
    replace({
      preventAssignment: false,
      'process.env.NODE_ENV': '"development"',
    }),
    emitStylesheet(),
    styleXRSPlugin({
      useCssPlaceholder: true,
      useCSSLayers: true,
      rsOptions: {
        dev: true,
        treeshakeCompensation: true,
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
    html({
      attributes: { script: { type: 'module' } },
      publicPath: '/',
      title: 'StyleX With Rollup',
      template: ({ attributes, files, publicPath, title }) => {
        const htmlTemplate = 'public/index.html';
        const { css, js } = files;
        const scripts = (js || [])
          .map(
            ({ fileName }) =>
              `<script src="${publicPath}${fileName}" ${makeHtmlAttributes(attributes.script)}></script>`
          )
          .join('\n');
        const links = (css || [])
          .map(({ fileName }) => `<link rel="stylesheet" href="${publicPath}${fileName}" />`)
          .join('\n');
        const template = readFileSync(htmlTemplate, 'utf-8');
        return template
          .replace(/{title}/g, title)
          .replace(/{links}/g, links)
          .replace(/{scripts}/g, scripts);
      },
    }),
  ],
};
