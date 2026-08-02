import { context } from './core.mjs';

const port = 8000;

await context.watch();

await context
  .serve({
    servedir: './',
    port,
  })
  .then(() => {
    console.log(`[info]: server start at http://127.0.0.1:${port}.`);
  })
  .catch((/** @type {unknown} */ error) => {
    console.error(error);
    // Without this the process still exits 0, so a dev server that never came
    // up reads as a successful run to whatever invoked it.
    process.exitCode = 1;
  });
