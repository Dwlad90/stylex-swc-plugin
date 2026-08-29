import { execFileSync } from 'child_process';
import { existsSync, readFileSync } from 'fs';
import path from 'path';

import { describe, expect, it } from 'vitest';

const PACKAGE_ROOT = path.join(__dirname, '..');

// `constants.ts` is the only module of this package that a bundler plugin can
// read from its config file. The plugins that build a glob need the extension
// list, and nothing else. The package index pulls in the plugin core, which
// loads the native compiler, so the list gets its own entry point. These tests
// keep that entry point cheap.
describe('the ./constants entry point', () => {
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

  it('has a source module that imports no other module', () => {
    const source = readFileSync(path.join(PACKAGE_ROOT, 'src', 'constants.ts'), 'utf8');

    // An import here would travel to every config file that reads the list.
    // The module resolves two CSS paths, and it must stay at that.
    expect(source).not.toMatch(/^\s*import\s/m);
    expect(source.match(/\brequire\.resolve\(/g) ?? []).toHaveLength(2);
    expect(source).not.toMatch(/\brequire\s*\(/);
  });

  it.runIf(existsSync(path.join(PACKAGE_ROOT, 'dist', 'constants.js')))(
    'loads no native compiler binary',
    () => {
      const script = [
        "require('./dist/constants.js');",
        'const native = Object.keys(require.cache).filter(id => id.endsWith(".node"));',
        'process.stdout.write(JSON.stringify(native));',
      ].join('\n');

      const loaded = execFileSync(process.execPath, ['-e', script], {
        cwd: PACKAGE_ROOT,
        encoding: 'utf8',
      });

      expect(JSON.parse(loaded)).toEqual([]);
    }
  );
});
