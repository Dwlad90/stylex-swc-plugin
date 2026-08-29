import { execFileSync } from 'node:child_process';
import path from 'node:path';

import { describe, expect, it } from 'vitest';

const BUILT_ENTRY = path.join(__dirname, '..', 'dist', 'index.js');

// PostCSS reads a plugin with `require`, so `module.exports` must be the plugin
// itself. A change that turns it back into a namespace object breaks the plugin
// without a sound: the build starts, and no CSS arrives.
//
// The test task of this package builds the package first, so this reads a real
// build every time. It needs no condition that could let it pass with nothing
// done.
interface RequireResult {
  type: string;
  postcss: unknown;
  defaultIsSelf: boolean;
}

/** Reads what a `require` of the built plugin gives back, in a new process. */
function requireBuiltPlugin(): RequireResult {
  const script = [
    `const published = require(${JSON.stringify(BUILT_ENTRY)});`,
    'process.stdout.write(',
    '  JSON.stringify({',
    '    type: typeof published,',
    '    postcss: published.postcss,',
    '    defaultIsSelf: published.default === published,',
    '  })',
    ');',
  ].join('\n');

  const output = execFileSync(process.execPath, ['-e', script], { encoding: 'utf8' });
  const parsed: unknown = JSON.parse(output);

  if (typeof parsed !== 'object' || parsed === null) {
    throw new Error('The built plugin reported no result');
  }

  return parsed as RequireResult;
}

describe('the built plugin under require', () => {
  const published = requireBuiltPlugin();

  it('gives the plugin itself', () => {
    expect(published.type).toBe('function');
  });

  // PostCSS reads this flag to tell a plugin from a plain function.
  it('keeps the flag that PostCSS reads', () => {
    expect(published.postcss).toBe(true);
  });

  it('gives the same plugin under default', () => {
    expect(published.defaultIsSelf).toBe(true);
  });
});
