import fs from 'node:fs';
import path from 'node:path';

import { afterEach, describe, expect, test, vi } from 'vitest';

import {
  assertBindingCanLoad,
  findNativeBindings,
  isCompilerBinding,
  isDualLoadUnsafe,
  loadedNativeBindings,
  NATIVE_BINARY_NAME,
} from '../lib/native-bindings.js';
import { createTempDirs } from './helpers/temp-dirs.js';

const temp = createTempDirs();

afterEach(() => {
  vi.unstubAllEnvs();
  temp.removeAll();
});

/** Makes a package directory and registers it for removal. */
function makePackage(...bindings: string[]): string {
  const dir = temp.make('bench-bindings-');
  fs.mkdirSync(path.join(dir, 'dist'));
  for (const name of bindings) fs.writeFileSync(path.join(dir, 'dist', name), '');
  return dir;
}

function attempt(label: string, bindings: string[], loaded: string[], platform: NodeJS.Platform) {
  return () => assertBindingCanLoad({ label, bindings, loaded: new Set(loaded), platform });
}

describe('isDualLoadUnsafe', () => {
  test('reports macOS as unsafe', () => {
    expect(isDualLoadUnsafe('darwin')).toBe(true);
  });

  // CI runs the paired gate on Linux. A guard that blocked there would stop
  // the only comparison that can gate.
  test('reports the CI platforms as safe', () => {
    expect(isDualLoadUnsafe('linux')).toBe(false);
    expect(isDualLoadUnsafe('win32')).toBe(false);
  });

  test('reports an unknown platform as safe', () => {
    expect(isDualLoadUnsafe('haiku')).toBe(false);
  });
});

describe('findNativeBindings', () => {
  test('finds one addon', () => {
    const dir = makePackage('rs-compiler.darwin-arm64.node');

    expect(findNativeBindings(dir)).toHaveLength(1);
  });

  test('finds every addon and sorts them', () => {
    const dir = makePackage('b.node', 'a.node');
    const found = findNativeBindings(dir).map(file => path.basename(file));

    expect(found).toEqual(['a.node', 'b.node']);
  });

  test('ignores files that are not addons', () => {
    const dir = makePackage('index.js', 'index.d.ts', 'real.node', 'notes.node.txt');
    const found = findNativeBindings(dir).map(file => path.basename(file));

    expect(found).toEqual(['real.node']);
  });

  test('returns nothing for a package with no dist directory', () => {
    const dir = temp.make('bench-bindings-bare-');

    expect(findNativeBindings(dir)).toEqual([]);
  });

  test('returns nothing for a path that does not exist', () => {
    expect(findNativeBindings('/no/such/package/anywhere')).toEqual([]);
  });

  test('returns nothing for an empty dist directory', () => {
    expect(findNativeBindings(makePackage())).toEqual([]);
  });

  // Two names for one file are one binding. Real paths make that visible, so a
  // linked copy does not look like a second binding.
  test('counts two links to one file as one binding', () => {
    const dir = makePackage('real.node');
    fs.symlinkSync(path.join(dir, 'dist', 'real.node'), path.join(dir, 'dist', 'alias.node'));

    expect(findNativeBindings(dir)).toHaveLength(1);
  });

  test('skips a link that points at nothing', () => {
    const dir = makePackage('real.node');
    fs.symlinkSync(path.join(dir, 'dist', 'gone.node'), path.join(dir, 'dist', 'broken.node'));
    const found = findNativeBindings(dir).map(file => path.basename(file));

    expect(found).toEqual(['real.node']);
  });

  test('reads a directory that holds many files', () => {
    const names = Array.from({ length: 600 }, (_, index) => `addon-${index}.node`);
    const dir = makePackage(...names, 'index.js');

    expect(findNativeBindings(dir)).toHaveLength(600);
  });

  test('reads a name that holds spaces and unicode', () => {
    const dir = makePackage('rs compiler — arm64.node');
    const found = findNativeBindings(dir).map(file => path.basename(file));

    expect(found).toEqual(['rs compiler — arm64.node']);
  });
});

describe('assertBindingCanLoad', () => {
  test('allows the first subject', () => {
    expect(attempt('base', ['/a.node'], [], 'darwin')).not.toThrow();
  });

  test('allows a subject that brings in no binding', () => {
    expect(attempt('babel', [], ['/a.node'], 'darwin')).not.toThrow();
  });

  // The runtime gives back one instance for one file, so the same subject
  // twice is safe.
  test('allows a binding that the process already holds', () => {
    expect(attempt('candidate', ['/a.node'], ['/a.node'], 'darwin')).not.toThrow();
  });

  test('stops a second and different binding on macOS', () => {
    expect(attempt('candidate', ['/b.node'], ['/a.node'], 'darwin')).toThrow(/SIGSEGV/);
  });

  test('names the subject and both bindings', () => {
    expect(attempt('candidate', ['/b.node'], ['/a.node'], 'darwin')).toThrow(
      /candidate[\s\S]*\/a\.node[\s\S]*\/b\.node/
    );
  });

  test('tells the reader what to do instead', () => {
    expect(attempt('candidate', ['/b.node'], ['/a.node'], 'darwin')).toThrow(/own process/);
  });

  test('allows a second binding on Linux', () => {
    expect(attempt('candidate', ['/b.node'], ['/a.node'], 'linux')).not.toThrow();
  });

  // A package can ship one addon per architecture. One new file among known
  // ones is still a new binding.
  test('stops a set that holds one new binding among known ones', () => {
    expect(attempt('candidate', ['/a.node', '/b.node'], ['/a.node'], 'darwin')).toThrow();
  });

  test('allows a set that repeats known bindings only', () => {
    expect(
      attempt('candidate', ['/a.node', '/b.node'], ['/a.node', '/b.node'], 'darwin')
    ).not.toThrow();
  });

  test('stops a load against a very large loaded set', () => {
    const loaded = Array.from({ length: 5000 }, (_, index) => `/binding-${index}.node`);

    expect(attempt('candidate', ['/new.node'], loaded, 'darwin')).toThrow(/SIGSEGV/);
  });

  test('allows a very large set that is fully known', () => {
    const loaded = Array.from({ length: 5000 }, (_, index) => `/binding-${index}.node`);

    expect(attempt('candidate', loaded, loaded, 'darwin')).not.toThrow();
  });
});

describe('loadedNativeBindings', () => {
  const packageDir = path.resolve(import.meta.dirname, '..', '..');
  // The two cases below need a build on disk. A clean checkout has none, and a
  // missing build is not a fault in this module, so skip instead of fail.
  const built = findNativeBindings(packageDir).length > 0;

  test('gives a set of absolute paths that end in .node', () => {
    for (const binding of loadedNativeBindings()) {
      expect(path.isAbsolute(binding)).toBe(true);
      expect(binding.endsWith('.node')).toBe(true);
    }
  });

  // `benchmark/lib/types.ts` reads an enum off the package build, so importing
  // the harness loads that addon. The reader must see it. If it does not, the
  // guard permits a second binding and the process dies without a message.
  test.runIf(built)('sees the binding that the harness itself loaded', async () => {
    // Importing the shared types reads an enum off the package build, which
    // loads the addon. This is what the paired runner does before it asks for
    // the first subject.
    await import('../lib/types.js');
    const own = findNativeBindings(packageDir);
    const loaded = loadedNativeBindings();

    expect(own.some(binding => loaded.has(binding))).toBe(true);
  });

  test('gives the same answer when it is called again', () => {
    expect([...loadedNativeBindings()].toSorted()).toEqual([...loadedNativeBindings()].toSorted());
  });

  // The package's own binding is already in the process, so asking to load it
  // again must stay allowed on every platform.
  test.runIf(built)('permits the binding that is already loaded', async () => {
    await import('../lib/types.js');

    expect(() =>
      assertBindingCanLoad({
        label: 'self',
        bindings: findNativeBindings(packageDir),
        loaded: loadedNativeBindings(),
        platform: 'darwin',
      })
    ).not.toThrow();
  });
});

describe('isCompilerBinding', () => {
  test('accepts an addon that a build of this compiler writes', () => {
    expect(isCompilerBinding('/pkg/dist/rs-compiler.darwin-arm64.node')).toBe(true);
    expect(isCompilerBinding('/pkg/dist/rs-compiler.linux-x64-gnu.node')).toBe(true);
  });

  // The loaded list holds every addon in the process. A watcher must not read
  // as a second compiler binding, or the guard stops a run that is safe.
  test('rejects an addon of another package', () => {
    expect(isCompilerBinding('/n/fsevents/fsevents.node')).toBe(false);
    expect(isCompilerBinding('/n/better-sqlite3/build/Release/better_sqlite3.node')).toBe(false);
  });

  test('rejects a name that only starts with the binary name', () => {
    expect(isCompilerBinding('/pkg/rs-compiler-extra.node')).toBe(false);
  });

  test('rejects a file that is not an addon', () => {
    expect(isCompilerBinding('/pkg/dist/rs-compiler.darwin-arm64.js')).toBe(false);
  });
});

describe('findNativeBindings resolution paths', () => {
  /** Writes a platform package the way a published install holds one. */
  function addPlatformPackage(packageDir: string, target: string): string {
    const dir = path.join(
      packageDir,
      'node_modules',
      '@stylexswc',
      `${NATIVE_BINARY_NAME}-${target}`
    );
    fs.mkdirSync(dir, { recursive: true });
    const addon = path.join(dir, `${NATIVE_BINARY_NAME}.${target}.node`);
    fs.writeFileSync(addon, '');
    return fs.realpathSync(addon);
  }

  // `files` in the manifest ships no addon, so a subject unpacked from the
  // registry keeps its addon in a platform package. A search of `dist` alone
  // finds nothing, and the guard would permit the load that stops the process.
  test('finds the addon of a published package that ships none in dist', () => {
    const dir = makePackage('index.js', 'transform.js');
    const addon = addPlatformPackage(dir, 'darwin-arm64');

    expect(findNativeBindings(dir)).toEqual([addon]);
  });

  test('finds the addons of several platform packages', () => {
    const dir = makePackage();
    const first = addPlatformPackage(dir, 'darwin-arm64');
    const second = addPlatformPackage(dir, 'linux-x64-gnu');

    expect(findNativeBindings(dir)).toEqual([first, second].toSorted());
  });

  test('finds a dist addon and a platform addon together', () => {
    const dir = makePackage(`${NATIVE_BINARY_NAME}.darwin-arm64.node`);
    const platform = addPlatformPackage(dir, 'linux-x64-gnu');

    expect(findNativeBindings(dir)).toHaveLength(2);
    expect(findNativeBindings(dir)).toContain(platform);
  });

  test('ignores a scoped package that is not a platform package', () => {
    const dir = makePackage();
    const other = path.join(dir, 'node_modules', '@stylexswc', 'helper');
    fs.mkdirSync(other, { recursive: true });
    fs.writeFileSync(path.join(other, 'helper.node'), '');

    expect(findNativeBindings(dir)).toEqual([]);
  });

  test('reads the file that NAPI_RS_NATIVE_LIBRARY_PATH names', () => {
    const dir = makePackage();
    const elsewhere = temp.make('bench-override-');
    const addon = path.join(elsewhere, 'custom-name.node');
    fs.writeFileSync(addon, '');
    vi.stubEnv('NAPI_RS_NATIVE_LIBRARY_PATH', addon);

    expect(findNativeBindings(dir)).toEqual([fs.realpathSync(addon)]);
  });

  test('ignores an override that names a file which is not there', () => {
    const dir = makePackage(`${NATIVE_BINARY_NAME}.darwin-arm64.node`);
    vi.stubEnv('NAPI_RS_NATIVE_LIBRARY_PATH', '/no/such/addon.node');

    expect(findNativeBindings(dir)).toHaveLength(1);
  });

  test('reports one addon once when two paths reach it', () => {
    const dir = makePackage(`${NATIVE_BINARY_NAME}.darwin-arm64.node`);
    const addon = path.join(dir, 'dist', `${NATIVE_BINARY_NAME}.darwin-arm64.node`);
    vi.stubEnv('NAPI_RS_NATIVE_LIBRARY_PATH', addon);

    expect(findNativeBindings(dir)).toEqual([fs.realpathSync(addon)]);
  });

  test('reads a package that holds very many platform packages', () => {
    const dir = makePackage();
    for (let index = 0; index < 300; index += 1) addPlatformPackage(dir, `target-${index}`);

    expect(findNativeBindings(dir)).toHaveLength(300);
  });
});

// The name must match `napi.binaryName` in the manifest. If a rename lands
// without this constant, the guard stops seeing the addons it must compare.
describe('NATIVE_BINARY_NAME', () => {
  test('matches the binary name that the package manifest declares', () => {
    const packageDir = path.resolve(import.meta.dirname, '..', '..');
    const manifest: unknown = JSON.parse(
      fs.readFileSync(path.join(packageDir, 'package.json'), 'utf-8')
    );
    const napi = (manifest as { napi?: { binaryName?: string } }).napi;

    expect(napi?.binaryName).toBe(NATIVE_BINARY_NAME);
  });
});
