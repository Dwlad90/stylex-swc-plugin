import { pluginReact } from '@rsbuild/plugin-react';
import { defineConfig } from '@rslib/core';
import styleXRSPlugin from '@stylexswc/unplugin/rspack';

export default defineConfig({
  source: {
    entry: {
      index: ['./src/**'],
    },
  },
  lib: [
    {
      bundle: false,
      dts: true,
      format: 'esm',
    },
  ],
  output: {
    target: 'web',
  },
  plugins: [pluginReact()],
  tools: {
    rspack: {
      plugins: [
        styleXRSPlugin({
          rsOptions: {
            dev: true,
            treeshakeCompensation: true,
            runtimeInjection: true,
          },
          extractCSS: false,
        }),
      ],
    },
  },
});
