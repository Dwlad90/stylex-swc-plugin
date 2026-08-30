import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import { afterEach, describe, expect, test } from 'vitest';

import {
  assertBindingCanLoad,
  findNativeBindings,
  isDualLoadUnsafe,
  loadedNativeBindings,
} from '../lib/native-bindings.js';

const created: string[] = [];

afterEach(() => {
  for (const dir of created.splice(0)) fs.rmSync(dir, { force: true, recursive: true });
});

/** Makes a package directory and registers it for removal. */
function makePackage(...bindings: string[]): string {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'bench-bindings-'));
  created.push(dir);
  fs.mkdirSync(path.join(dir, 'dist'));
  for (const name of bindings) fs.writeFileSync(path.join(dir, 'dist', name), '');
  return dir;
}

function load(label: string, bindings: string[], loaded: string[], platform: NodeJS.Platform) {
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
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'bench-bindings-bare-'));
    created.push(dir);

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
    expect(load('base', ['/a.node'], [], 'darwin')).not.toThrow();
  });

  test('allows a subject that brings in no binding', () => {
    expect(load('babel', [], ['/a.node'], 'darwin')).not.toThrow();
  });

  // The runtime gives back one instance for one file, so the same subject
  // twice is safe.
  test('allows a binding that the process already holds', () => {
    expect(load('candidate', ['/a.node'], ['/a.node'], 'darwin')).not.toThrow();
  });

  test('stops a second and different binding on macOS', () => {
    expect(load('candidate', ['/b.node'], ['/a.node'], 'darwin')).toThrow(/SIGSEGV/);
  });

  test('names the subject and both bindings', () => {
    expect(load('candidate', ['/b.node'], ['/a.node'], 'darwin')).toThrow(
      /candidate[\s\S]*\/a\.node[\s\S]*\/b\.node/
    );
  });

  test('tells the reader what to do instead', () => {
    expect(load('candidate', ['/b.node'], ['/a.node'], 'darwin')).toThrow(/own process/);
  });

  test('allows a second binding on Linux', () => {
    expect(load('candidate', ['/b.node'], ['/a.node'], 'linux')).not.toThrow();
  });

  // A package can ship one addon per architecture. One new file among known
  // ones is still a new binding.
  test('stops a set that holds one new binding among known ones', () => {
    expect(load('candidate', ['/a.node', '/b.node'], ['/a.node'], 'darwin')).toThrow();
  });

  test('allows a set that repeats known bindings only', () => {
    expect(
      load('candidate', ['/a.node', '/b.node'], ['/a.node', '/b.node'], 'darwin')
    ).not.toThrow();
  });

  test('stops a load against a very large loaded set', () => {
    const loaded = Array.from({ length: 5000 }, (_, index) => `/binding-${index}.node`);

    expect(load('candidate', ['/new.node'], loaded, 'darwin')).toThrow(/SIGSEGV/);
  });

  test('allows a very large set that is fully known', () => {
    const loaded = Array.from({ length: 5000 }, (_, index) => `/binding-${index}.node`);

    expect(load('candidate', loaded, loaded, 'darwin')).not.toThrow();
  });
});

describe('loadedNativeBindings', () => {
  const packageDir = path.resolve(import.meta.dirname, '..', '..');

  test('gives a set of absolute paths that end in .node', () => {
    for (const binding of loadedNativeBindings()) {
      expect(path.isAbsolute(binding)).toBe(true);
      expect(binding.endsWith('.node')).toBe(true);
    }
  });

  // `benchmark/lib/types.ts` reads an enum off the package build, so importing
  // the harness loads that addon. The reader must see it. If it does not, the
  // guard permits a second binding and the process dies without a message.
  test('sees the binding that the harness itself loaded', async () => {
    // Importing the shared types reads an enum off the package build, which
    // loads the addon. This is what the paired runner does before it asks for
    // the first subject.
    await import('../lib/types.js');
    const own = findNativeBindings(packageDir);
    const loaded = loadedNativeBindings();

    expect(own.length).toBeGreaterThan(0);
    expect(own.some(binding => loaded.has(binding))).toBe(true);
  });

  test('gives the same answer when it is called again', () => {
    expect([...loadedNativeBindings()].toSorted()).toEqual([...loadedNativeBindings()].toSorted());
  });

  // The package's own binding is already in the process, so asking to load it
  // again must stay allowed on every platform.
  test('permits the binding that is already loaded', async () => {
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
