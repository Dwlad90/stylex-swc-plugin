import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'node',
    exclude: ['**/node_modules/**', '**/dist/**'],
    globals: true,
    // Every case in `integration.test.ts` runs a real webpack build, and the
    // filesystem-cache case runs two. The 5s default leaves that case no
    // headroom on a loaded CI runner, where one production build measured 2.4s
    // and the pair timed out at 5.01s.
    testTimeout: 60_000,
  },
});
