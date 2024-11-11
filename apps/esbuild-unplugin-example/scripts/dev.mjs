import { context } from './core.mjs'

const port = 8000

await context.watch()

await context
  .serve({
    servedir: './',
    port,
  })
  .then(() => {
    console.log(`[info]: server start at http://127.0.0.1:${port}.`)
  })
  .catch((error) => {
    console.error(error)
  })