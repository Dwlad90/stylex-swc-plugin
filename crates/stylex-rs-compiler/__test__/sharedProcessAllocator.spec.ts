// A build loads this binding and `@swc/core` into one Node process. The
// rollup, webpack and Vite pipelines all do this. Each binding links its own
// copy of mimalloc. On macOS, two copies of mimalloc v3 share one TLS slot and
// stop the process with SIGSEGV. The crate manifest gives the cause and the
// fix.
//
// The check runs in a child process, because a crash stops the process that
// runs it. It runs on every platform. Only macOS can fail it today.
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { expect, test } from 'vitest';

import { runNodeScript } from './nodeScript';

const require = createRequire(import.meta.url);

const distEntry = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../dist/index.js');

const swcCore = require.resolve('@swc/core');

/**
 * The first `@swc/core` release that links mimalloc v3. An older release
 * cannot cause the crash, so a test against it proves nothing.
 */
const FIRST_SWC_CORE_WITH_MIMALLOC_V3 = '1.16.17';

/**
 * A number that sorts `major.minor.patch` versions in release order. A
 * pre-release tag is ignored. Each part must be less than 1000.
 */
function releaseRank(version: string): number {
  const [major = 0, minor = 0, patch = 0] = version
    .split('.')
    .map(part => Number.parseInt(part, 10));

  return major * 1e6 + minor * 1e3 + patch;
}

/** A script that loads both bindings in the given order and uses each one. */
function loadBoth(order: 'swc first' | 'stylex first'): string {
  const swc = `const swc = require(${JSON.stringify(swcCore)});
    swc.transformSync('const a = 1;', {});`;
  const stylex = `const stylex = require(${JSON.stringify(distEntry)});
    stylex.transform('file.js', 'export const x = 1;', stylex.normalizeRsOptions({}));`;
  const [first, second] = order === 'swc first' ? [swc, stylex] : [stylex, swc];

  return `${first}\n${second}\nconsole.log('loaded');`;
}

test('the resolved @swc/core links mimalloc v3, so the checks below can fail', () => {
  const { version } = require('@swc/core/package.json') as { version: string };

  expect(releaseRank(version)).toBeGreaterThanOrEqual(releaseRank(FIRST_SWC_CORE_WITH_MIMALLOC_V3));
});

test.each(['swc first', 'stylex first'] as const)(
  'the binding and @swc/core load into one process, %s',
  order => {
    // The diff of a failure prints the whole outcome, `stderr` included.
    expect(runNodeScript(loadBoth(order))).toMatchObject({
      signal: null,
      status: 0,
      stdout: 'loaded\n',
    });
  }
);
