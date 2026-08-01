import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'node',
    // The suites exercise the built NAPI binding in `dist`, so `dist` is
    // excluded from test discovery but deliberately still importable.
    exclude: ['**/node_modules/**', '**/dist/**', '**/perf_fixtures/**'],
    // `memoryLeak.spec.ts` spawns child processes and repeatedly reloads the
    // native module, which is slower than a normal unit test.
    testTimeout: 120_000,
  },
});
