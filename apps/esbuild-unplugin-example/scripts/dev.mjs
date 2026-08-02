import { context } from './core.mjs';

const port = 8000;

try {
  await context.watch();
  await context.serve({
    servedir: './',
    port,
  });
  console.log(`[info]: server start at http://127.0.0.1:${port}.`);
} catch (/** @type {unknown} */ error) {
  console.error(error);

  try {
    await context.dispose();
  } catch (/** @type {unknown} */ disposeError) {
    console.error('Failed to dispose the esbuild context:', disposeError);
  }

  // Keep stderr flushable while allowing the process to exit once the watch
  // resources have been released.
  process.exitCode = 1;
}
