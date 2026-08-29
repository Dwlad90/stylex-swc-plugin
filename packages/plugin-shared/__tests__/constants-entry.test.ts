import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

import { describe, expect, it } from 'vitest';

import { INCLUDE_EXTENSIONS } from '../src/constants';

const PACKAGE_ROOT = path.join(__dirname, '..');
const BUILT_ENTRY = path.join(PACKAGE_ROOT, 'dist', 'constants.js');

interface EntryReading {
  extensions: string[];
  loaded: string[];
}

let reading: EntryReading | undefined;

/**
 * Loads the built entry point in a new Node process and reports what that
 * process ended up with. A new process is the only place where the cost of the
 * entry point can be read, because the test process has already loaded the
 * package for the tests above.
 *
 * One process answers every question below, so the result is kept and given
 * again to each caller.
 */
function readBuiltEntry(): EntryReading {
  if (reading !== undefined) {
    return reading;
  }

  const script = [
    "const entry = require('@stylexswc/plugin-shared/constants');",
    'const loaded = Object.keys(require.cache);',
    'process.stdout.write(JSON.stringify({ extensions: entry.INCLUDE_EXTENSIONS, loaded }));',
  ].join('\n');

  const output = execFileSync(process.execPath, ['-e', script], {
    cwd: PACKAGE_ROOT,
    encoding: 'utf8',
  });

  reading = JSON.parse(output) as EntryReading;

  return reading;
}

// `constants.ts` is the only module of this package that a bundler plugin can
// read from its config file. The plugins that build a glob need the extension
// list, and nothing else. The package index pulls in the plugin core, which
// loads the native compiler, so the list gets its own entry point. These tests
// keep that entry point cheap.
describe('the ./constants entry point', () => {
  // This one reads the manifest, so it holds the contract even before a build.
  // The tests that load the entry point cannot run then.
  it('is declared in the package exports', () => {
    const manifest = JSON.parse(readFileSync(path.join(PACKAGE_ROOT, 'package.json'), 'utf8')) as {
      exports: Record<string, Record<string, string>>;
    };

    expect(manifest.exports['./constants']).toEqual({
      types: './dist/constants.d.ts',
      import: './dist/constants.js',
      require: './dist/constants.js',
    });
  });

  describe.runIf(existsSync(BUILT_ENTRY))('once it is built', () => {
    it('resolves under its subpath and gives the list', () => {
      expect(readBuiltEntry().extensions).toEqual([...INCLUDE_EXTENSIONS]);
    });

    it('loads no native compiler binary', () => {
      const native = readBuiltEntry().loaded.filter(id => id.endsWith('.node'));

      expect(native).toEqual([]);
    });

    it('loads no compiler package', () => {
      // These two carry the compiler. A single import of either would travel to
      // every config file that reads the list.
      const compiler = readBuiltEntry().loaded.filter(
        id => id.includes('rs-compiler') || id.includes('babel-plugin')
      );

      expect(compiler).toEqual([]);
    });

    it('loads the entry point alone', () => {
      // The entry point resolves two stylesheet paths and holds no import, so
      // the process ends up with exactly one module.
      expect(readBuiltEntry().loaded).toHaveLength(1);
    });
  });
});
