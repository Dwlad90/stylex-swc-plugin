/**
 * Layout of the `benchmarks` branch and the chart viewer that reads it.
 *
 * The aggregate job appends release measurements to per-target `data.js` files
 * on that branch. `github-action-benchmark` established the layout and writes
 * `index.html` itself, but only for targets it ran on directly -- targets whose
 * history the aggregate script appends would otherwise accumulate data with no
 * page to read it. Both halves live here so the directory shape is defined once
 * and the viewer travels with it.
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

/**
 * Provenance of `templates/benchmark-index.html`, which is a byte-for-byte copy
 * of the page the action generates. It is only interchangeable with an
 * action-written page for this exact release, so bumping the workflow pin means
 * regenerating the template -- `aggregate-release-benchmarks.test.mjs` asserts
 * the two stay in step rather than letting them drift silently.
 */
export const VIEWER_TEMPLATE_SOURCE = Object.freeze({
  action: 'benchmark-action/github-action-benchmark',
  ref: '52576c92bccf6ac60c8223ec7eb2565637cae9ba',
  version: 'v1.22.1',
});

export const VIEWER_TEMPLATE = fileURLToPath(
  new URL('../templates/benchmark-index.html', import.meta.url)
);

/** Directory holding one target's history for one Node version. */
export function historyDataDir(pagesDir, target, nodeVersion) {
  return path.join(pagesDir, 'dev/bench/releases', target, `node-${nodeVersion}`);
}

/**
 * Copy the viewer into a history directory unless a page is already there.
 *
 * Never overwrites: the action owns the pages it wrote. Returns whether a page
 * was added so the caller can report it.
 */
export function ensureViewerPage(dataDir) {
  const page = path.join(dataDir, 'index.html');
  if (fs.existsSync(page)) return false;
  fs.copyFileSync(VIEWER_TEMPLATE, page);
  return true;
}
