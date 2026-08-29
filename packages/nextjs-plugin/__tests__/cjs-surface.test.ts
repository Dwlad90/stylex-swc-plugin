import { execFileSync } from 'node:child_process';

import { describe, expect, it } from 'vitest';

interface PublishedPlugin {
  /** Package that publishes the plugin. */
  name: string;
  /** Names the package must publish beside the plugin itself. */
  named: string[];
}

// Every plugin a host loads with `require` publishes itself through
// `exportAsCommonJs`. A host reads `module.exports` as the plugin, so a change
// that turns it back into a namespace object breaks the plugin without a
// sound: the build starts, and the styles never arrive.
//
// These three are dependencies of this package, so the test task builds them
// first. The test therefore reads a real build every time, and it needs no
// condition that could let it pass with nothing done.
const PUBLISHED_PLUGINS: PublishedPlugin[] = [
  { name: '@stylexswc/webpack-plugin', named: ['loader', 'virtualLoader', 'StyleXPlugin'] },
  { name: '@stylexswc/rspack-plugin', named: ['loader', 'virtualLoader', 'StyleXPlugin'] },
  { name: '@stylexswc/turbopack-plugin', named: [] },
];

interface RequireResult {
  type: string;
  defaultIsSelf: boolean;
  named: string[];
}

/**
 * Reads what a `require` of one package gives back. A new process is used, so
 * one package cannot change what the next one reports.
 */
function requirePackage(name: string): RequireResult {
  const script = [
    `const published = require(${JSON.stringify(name)});`,
    'process.stdout.write(',
    '  JSON.stringify({',
    '    type: typeof published,',
    '    defaultIsSelf: published.default === published,',
    '    named: Object.keys(published),',
    '  })',
    ');',
  ].join('\n');

  const output = execFileSync(process.execPath, ['-e', script], {
    cwd: __dirname,
    encoding: 'utf8',
  });
  const parsed: unknown = JSON.parse(output);

  if (typeof parsed !== 'object' || parsed === null) {
    throw new Error(`${name} reported no result`);
  }

  return parsed as RequireResult;
}

describe.each(PUBLISHED_PLUGINS)('$name', ({ name, named }) => {
  const published = requirePackage(name);

  it('gives the plugin itself to require', () => {
    expect(published.type).toBe('function');
  });

  it('gives the same plugin under default', () => {
    expect(published.defaultIsSelf).toBe(true);
  });

  it.each(named)('publishes %s beside the plugin', publishedName => {
    expect(published.named).toContain(publishedName);
  });
});
