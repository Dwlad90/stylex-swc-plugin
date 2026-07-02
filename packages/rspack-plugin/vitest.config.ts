import path from 'path';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  resolve: {
    alias: {
      '@stylexswc/webpack-plugin/shared': path.resolve(
        __dirname,
        '../webpack-plugin/src/shared.ts'
      ),
    },
  },
  test: {
    environment: 'node',
    exclude: ['**/node_modules/**', '**/dist/**'],
    globals: true,
  },
});
