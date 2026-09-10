import fs from 'node:fs';
import path from 'node:path';

import { afterEach, describe, expect, test, vi } from 'vitest';

import {
  assertBindingCanLoad,
  assertBindingIsVisible,
  bindingPathKey,
  findNativeBindings,
  holdsBinding,
  isCompilerBinding,
  isDualLoadRestricted,
  linksMimalloc,
  loadedNativeBindings,
  NATIVE_BINARY_NAME,
} from '../lib/native-bindings.js';
import { realPathOf, settledPathOf } from '../lib/paths.js';
import { createTempDirs } from './helpers/temp-dirs.js';

const temp = createTempDirs();

afterEach(() => {
  vi.unstubAllEnvs();
  // A case that stands in for the runtime report must not leave it stood in
  // for: the cases that read the real report run after it.
  vi.restoreAllMocks();
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

function visible(bindings: string[], loaded: string[], platform: NodeJS.Platform) {
  return () =>
    assertBindingIsVisible({ label: 'base', bindings, loaded: new Set(loaded), platform });
}

describe('isDualLoadRestricted', () => {
  test('reports macOS as restricted', () => {
    expect(isDualLoadRestricted('darwin')).toBe(true);
  });

  // CI runs the paired gate on Linux. A guard that blocked there would stop
  // the only comparison that can gate.
  test('reports the CI platforms as unrestricted', () => {
    expect(isDualLoadRestricted('linux')).toBe(false);
    expect(isDualLoadRestricted('win32')).toBe(false);
  });

  test('reports an unknown platform as unrestricted', () => {
    expect(isDualLoadRestricted('haiku')).toBe(false);
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

  // Measured on darwin arm64: a published base without mimalloc and a
  // candidate with it ran the paired benchmark to the end. Refusing that pair
  // would stop a release that works.
  test('allows a second binding when only one of the two links mimalloc', () => {
    const dir = temp.make('bench-allocator-');
    const held = writeAddon(dir, 'held.node', 'system');
    const incoming = writeAddon(dir, 'incoming.node', 'mimalloc');

    expect(attempt('candidate', [incoming], [held], 'darwin')).not.toThrow();
  });

  test('allows a second binding when neither links mimalloc', () => {
    const dir = temp.make('bench-allocator-');
    const held = writeAddon(dir, 'held.node', 'system');
    const incoming = writeAddon(dir, 'incoming.node', 'system');

    expect(attempt('candidate', [incoming], [held], 'darwin')).not.toThrow();
  });

  test('stops a second binding when both link mimalloc', () => {
    const dir = temp.make('bench-allocator-');
    const held = writeAddon(dir, 'held.node', 'mimalloc');
    const incoming = writeAddon(dir, 'incoming.node', 'mimalloc');

    expect(attempt('candidate', [incoming], [held], 'darwin')).toThrow(/mimalloc/);
  });

  // One safe binding among the held ones does not make the pair safe: the
  // process still holds a second mimalloc heap once the new one arrives.
  test('stops a load when one held binding of several links mimalloc', () => {
    const dir = temp.make('bench-allocator-');
    const safe = writeAddon(dir, 'safe.node', 'system');
    const held = writeAddon(dir, 'held.node', 'mimalloc');
    const incoming = writeAddon(dir, 'incoming.node', 'mimalloc');

    expect(attempt('candidate', [incoming], [safe, held], 'darwin')).toThrow(/mimalloc/);
  });

  test('allows a mimalloc binding on Linux', () => {
    const dir = temp.make('bench-allocator-');
    const held = writeAddon(dir, 'held.node', 'mimalloc');
    const incoming = writeAddon(dir, 'incoming.node', 'mimalloc');

    expect(attempt('candidate', [incoming], [held], 'linux')).not.toThrow();
  });
});

describe('assertBindingIsVisible', () => {
  test('allows a subject whose binding was found', () => {
    const dir = temp.make('bench-allocator-');
    const held = writeAddon(dir, 'held.node', 'mimalloc');
    const found = writeAddon(dir, 'found.node', 'system');

    expect(visible([found], [held], 'darwin')).not.toThrow();
  });

  test('allows the first subject, which meets no held binding', () => {
    expect(visible([], [], 'darwin')).not.toThrow();
  });

  test('allows a subject when nothing held links mimalloc', () => {
    const dir = temp.make('bench-allocator-');
    const held = writeAddon(dir, 'held.node', 'system');

    expect(visible([], [held], 'darwin')).not.toThrow();
  });

  test('allows an unfound binding on Linux', () => {
    const dir = temp.make('bench-allocator-');
    const held = writeAddon(dir, 'held.node', 'mimalloc');

    expect(visible([], [held], 'linux')).not.toThrow();
  });

  // The layout an installer writes is the thing that changed under this guard
  // once already. A layout nobody listed must stop the run with a sentence
  // rather than let the process die without one.
  test('stops a subject whose binding was not found', () => {
    const dir = temp.make('bench-allocator-');
    const held = writeAddon(dir, 'held.node', 'mimalloc');

    expect(visible([], [held], 'darwin')).toThrow(/no native binding was found/);
  });

  test('names the subject and what to do', () => {
    const dir = temp.make('bench-allocator-');
    const held = writeAddon(dir, 'held.node', 'mimalloc');

    expect(visible([], [held], 'darwin')).toThrow(/base[\s\S]*NAPI_RS_NATIVE_LIBRARY_PATH/);
  });
});

describe('linksMimalloc', () => {
  // The rule is about the shipped binaries, so the shipped binary is the case
  // that proves the marker is the right one to look for. A clean checkout has
  // no build, and a missing build is not a fault in this module.
  const ownBindings = addonsInDist(path.resolve(import.meta.dirname, '..', '..'));

  test('reads the marker that a build with mimalloc holds', () => {
    const dir = temp.make('bench-allocator-');

    expect(linksMimalloc(writeAddon(dir, 'with.node', 'mimalloc'))).toBe(true);
  });

  test('reads a build without mimalloc', () => {
    const dir = temp.make('bench-allocator-');

    expect(linksMimalloc(writeAddon(dir, 'without.node', 'system'))).toBe(false);
  });

  test('reads an empty file as a build without mimalloc', () => {
    const dir = temp.make('bench-allocator-');
    const addon = path.join(dir, 'empty.node');
    fs.writeFileSync(addon, '');

    expect(linksMimalloc(addon)).toBe(false);
  });

  // A file nobody can read gives no evidence, and the guard must not permit
  // the load it cannot rule out. Reading it as a mimalloc build keeps the
  // older, stricter answer for a binding this module cannot measure.
  test('reads a file that is not there as a mimalloc build', () => {
    expect(linksMimalloc('/no/such/addon.node')).toBe(true);
  });

  test('reads a directory as a mimalloc build', () => {
    expect(linksMimalloc(temp.make('bench-allocator-'))).toBe(true);
  });

  test.runIf(ownBindings.length > 0)('reads the build of this package', () => {
    for (const binding of ownBindings) expect(linksMimalloc(binding)).toBe(true);
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

    // Both sets are named in the message: a bare `false` here says only that
    // the two readers disagree, and which of them moved is the whole question.
    expect(
      own.some(binding => holdsBinding(loaded, binding)),
      `own: ${own.join(', ')} | loaded: ${[...loaded].join(', ')}`
    ).toBe(true);
  });

  // The report and the variable are written by different hands, and only one of
  // them is settled. A temp directory is reached through a symbolic link on
  // macOS -- `/var` names `/private/var` -- which is the platform where a
  // second binding ends the process, so the two spellings must key alike. When
  // they did not, the override was dropped, the set read empty, and the guard
  // returned early on a load it exists to stop. The addon is given a name the
  // compiler rule does not match, so the override branch is the only way in.
  test.skipIf(!canSymlink)('reads an override that names the file through a link', () => {
    const real = temp.make('bench-override-real-');
    const addon = path.join(real, 'custom-name.node');
    fs.writeFileSync(addon, '');

    const link = path.join(temp.make('bench-override-link-'), 'to-real');
    fs.symlinkSync(real, link);
    const throughLink = path.join(link, 'custom-name.node');

    expect(throughLink).not.toBe(realPathOf(throughLink));
    vi.stubEnv('NAPI_RS_NATIVE_LIBRARY_PATH', throughLink);
    vi.spyOn(process.report, 'getReport').mockReturnValue({
      sharedObjects: [realPathOf(addon)],
    });

    expect([...loadedNativeBindings()]).toEqual([realPathOf(addon)]);
  });

  test('drops an override that names nothing rather than matching everything', () => {
    const absent = path.join(temp.make('bench-override-absent-'), 'gone.node');
    vi.stubEnv('NAPI_RS_NATIVE_LIBRARY_PATH', absent);
    vi.spyOn(process.report, 'getReport').mockReturnValue({
      sharedObjects: ['/n/fsevents/fsevents.node'],
    });

    expect([...loadedNativeBindings()]).toEqual([]);
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
  return realPathOf(addon);
}

/**
 * Writes `<root>/node_modules/@stylexswc/rs-compiler` the way an install does.
 *
 * The platform package is hoisted beside it rather than nested under it, which
 * is what npm does and what pnpm does through a link. Answers both directories,
 * because the addon of such an install is found from the root and not from the
 * package.
 */
function makeInstalledPackage(): { root: string; packageDir: string } {
  const root = temp.make('bench-install-');
  const packageDir = path.join(root, 'node_modules', '@stylexswc', NATIVE_BINARY_NAME);
  fs.mkdirSync(path.join(packageDir, 'dist'), { recursive: true });
  return { root, packageDir };
}

/**
 * The addons that this package built, and none that an ancestor directory
 * holds. A case about the build on disk must read the build on disk.
 */
function addonsInDist(packageDir: string): string[] {
  const dist = path.join(packageDir, 'dist');
  if (!fs.existsSync(dist)) return [];

  return fs
    .readdirSync(dist)
    .filter(entry => entry.endsWith('.node'))
    .map(entry => path.join(dist, entry));
}

/** An addon file that holds the marker of the allocator, or one that does not. */
function writeAddon(dir: string, name: string, allocator: 'mimalloc' | 'system'): string {
  const addon = path.join(dir, name);
  const body =
    allocator === 'mimalloc' ? 'ELF\u0000mimalloc: option\u0000' : 'ELF\u0000malloc\u0000';
  fs.writeFileSync(addon, body, 'binary');
  return realPathOf(addon);
}

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

  // The name is taken from `bindingPathKey`, so every spelling that function
  // settles reaches this rule too. Windows names the module in whatever case
  // the loader recorded, and a name dropped for its case leaves the guard blind
  // to a binding the process holds.
  test('accepts a Windows addon whatever case the loader recorded', () => {
    expect(isCompilerBinding(`D:\\Build\\dist\\RS-Compiler.WIN32-X64-MSVC.NODE`, 'win32')).toBe(
      true
    );
  });

  test('accepts a Windows addon behind a long-path prefix', () => {
    expect(isCompilerBinding(`\\\\?\\D:\\a\\dist\\rs-compiler.win32-x64-msvc.node`, 'win32')).toBe(
      true
    );
    expect(
      isCompilerBinding(`\\\\?\\UNC\\host\\share\\rs-compiler.win32-x64-msvc.node`, 'win32')
    ).toBe(true);
  });

  test('accepts a Windows addon written with forward separators', () => {
    expect(isCompilerBinding('D:/a/dist/rs-compiler.win32-x64-msvc.node', 'win32')).toBe(true);
  });

  test('still rejects another package on Windows', () => {
    expect(isCompilerBinding(`D:\\a\\node_modules\\FSEvents\\FSEvents.node`, 'win32')).toBe(false);
  });

  // A POSIX file system holds `A.node` and `a.node` apart, so folding case
  // there would accept a file that is not this compiler's addon.
  test('keeps case on a POSIX platform', () => {
    expect(isCompilerBinding('/pkg/dist/RS-Compiler.linux-x64-gnu.node', 'linux')).toBe(false);
    expect(isCompilerBinding('/pkg/dist/rs-compiler.linux-x64-gnu.node', 'linux')).toBe(true);
  });

  // A backslash is an ordinary character on POSIX, so the whole string is one
  // name and it does not start with the binary name.
  test('reads a backslash as an ordinary character on a POSIX platform', () => {
    expect(isCompilerBinding('/a/b\\rs-compiler.linux-x64-gnu.node', 'linux')).toBe(false);
  });

  // A trailing separator names the same entry, which is what `basename`
  // answers. An empty path names nothing.
  test('reads a path that ends in a separator as the name before it', () => {
    expect(isCompilerBinding('/pkg/rs-compiler.linux-x64-gnu.node/', 'linux')).toBe(true);
  });

  test('answers no for a path that names nothing', () => {
    expect(isCompilerBinding('', 'linux')).toBe(false);
    expect(isCompilerBinding('/', 'linux')).toBe(false);
    expect(isCompilerBinding('', 'win32')).toBe(false);
  });

  test('reads a path far deeper than any this repository builds', () => {
    const deep = `${'/level'.repeat(2_000)}/dist/rs-compiler.linux-x64-gnu.node`;

    expect(isCompilerBinding(deep, 'linux')).toBe(true);
  });
});

/**
 * Whether this runner can make a symbolic link. Windows refuses one without a
 * privilege the CI account does not hold, and a case about a resolver has
 * nothing to say about a platform that cannot set one up.
 */
const canSymlink = (() => {
  const dir = temp.make('bench-symlink-probe-');
  try {
    fs.symlinkSync(path.join(dir, 'target'), path.join(dir, 'link'));
    return true;
  } catch {
    return false;
  }
})();

describe('settling a path', () => {
  // The rule the Windows runner broke: a test settles a path with the same
  // function the code does. `fs.realpathSync` is the other one, and it differs
  // from this one only on Windows, where it leaves an 8.3 short name as it
  // found it. A machine with no short names cannot show that, so what is
  // asserted here is the agreement itself.
  test('a directory the temp helper made needs no further settling', () => {
    const dir = temp.make('bench-realpath-');

    expect(realPathOf(dir)).toBe(dir);
    expect(realPathOf(path.join(dir, '..'))).toBe(path.dirname(dir));
  });

  test('refuses a path that names nothing', () => {
    expect(() => realPathOf(path.join(temp.make('bench-realpath-gone-'), 'absent.node'))).toThrow();
  });

  test('the settled form answers the path itself when it names nothing', () => {
    const absent = path.join(temp.make('bench-settled-gone-'), 'absent.node');

    expect(settledPathOf(absent)).toBe(absent);
  });

  test.skipIf(!canSymlink)('follows a link to the file it names', () => {
    const dir = makePackage('rs-compiler.darwin-arm64.node');
    const target = path.join(dir, 'dist', 'rs-compiler.darwin-arm64.node');
    const link = path.join(dir, 'dist', 'rs-compiler.linux-x64-gnu.node');
    fs.symlinkSync(target, link);

    expect(realPathOf(link)).toBe(realPathOf(target));
    expect(settledPathOf(link)).toBe(realPathOf(target));
  });
});

describe('findNativeBindings resolution paths', () => {
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

    expect(findNativeBindings(dir)).toEqual([realPathOf(addon)]);
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

    expect(findNativeBindings(dir)).toEqual([realPathOf(addon)]);
  });

  test('reads a package that holds very many platform packages', () => {
    const dir = makePackage();
    for (let index = 0; index < 300; index += 1) addPlatformPackage(dir, `target-${index}`);

    expect(findNativeBindings(dir)).toHaveLength(300);
  });

  // An install hoists the platform package beside the compiler package instead
  // of nesting it under it, and the installed package ships no addon of its
  // own. A search that reads only the package finds nothing there, and the
  // guard then permits the load it exists to stop.
  test('finds the addon of a package whose platform package is hoisted', () => {
    const { root, packageDir } = makeInstalledPackage();
    const addon = addPlatformPackage(root, 'darwin-arm64');

    expect(findNativeBindings(packageDir)).toEqual([addon]);
  });

  // pnpm puts the compiler package deep under `node_modules/.pnpm` and links
  // the platform package beside it there. Node resolves a module through its
  // real path, so the search must start from the real path too.
  test.skipIf(!canSymlink)('finds the addon of a package reached through a link', () => {
    const { root, packageDir } = makeInstalledPackage();
    const addon = addPlatformPackage(root, 'darwin-arm64');
    const link = path.join(temp.make('bench-link-'), 'rs-compiler');
    fs.symlinkSync(packageDir, link);

    expect(findNativeBindings(link)).toEqual([addon]);
  });

  // Node appends `node_modules` to every ancestor except one already named
  // that, so `node_modules/node_modules` is a path it never asks about. A
  // search that asked would read a directory the subject cannot load from.
  test('ignores a scope directory under a second node_modules', () => {
    const { root, packageDir } = makeInstalledPackage();
    addPlatformPackage(path.join(root, 'node_modules'), 'darwin-arm64');

    expect(findNativeBindings(packageDir)).toEqual([]);
  });

  test('ignores a hoisted package of another scope', () => {
    const { root, packageDir } = makeInstalledPackage();
    const other = path.join(root, 'node_modules', '@other', `${NATIVE_BINARY_NAME}-darwin-arm64`);
    fs.mkdirSync(other, { recursive: true });
    fs.writeFileSync(path.join(other, `${NATIVE_BINARY_NAME}.darwin-arm64.node`), '');

    expect(findNativeBindings(packageDir)).toEqual([]);
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

/**
 * One spelling of a path, which is what lets the two readers agree.
 *
 * The guard reads the bindings a subject holds from the file system and the
 * bindings the process holds from the runtime's report. On Windows those two
 * need not spell the same file the same way, and a Windows job read its own
 * addon as a second binding for exactly that reason. The platform is an
 * argument, so every case runs on every host.
 */
describe('bindingPathKey', () => {
  const addon = 'rs-compiler.win32-x64-msvc.node';

  test('folds the case a Windows file system does not keep', () => {
    expect(bindingPathKey(`D:\\Build\\dist\\${addon}`, 'win32')).toBe(`d:\\build\\dist\\${addon}`);
  });

  test('reads the two spellings of a drive letter as one path', () => {
    // The difference a module loader and a file URL actually produce.
    expect(bindingPathKey(`D:\\a\\dist\\${addon}`, 'win32')).toBe(
      bindingPathKey(`d:\\a\\dist\\${addon}`, 'win32')
    );
  });

  test('reads a forward slash as the separator it is on Windows', () => {
    expect(bindingPathKey(`D:/a/dist/${addon}`, 'win32')).toBe(
      bindingPathKey(`D:\\a\\dist\\${addon}`, 'win32')
    );
  });

  test('drops a long-path prefix that realpath may answer with', () => {
    expect(bindingPathKey(`\\\\?\\D:\\a\\${addon}`, 'win32')).toBe(`d:\\a\\${addon}`);
  });

  test('drops the UNC form of that prefix and keeps the path it names', () => {
    // `\\?\UNC\host\share` is the long-path spelling of `\\host\share`, so the
    // key it answers is a path that can be opened rather than a bare share.
    expect(bindingPathKey(`\\\\?\\UNC\\host\\share\\${addon}`, 'win32')).toBe(
      `\\\\host\\share\\${addon}`
    );
  });

  test('a plain UNC path keeps both of its leading separators', () => {
    expect(bindingPathKey(`\\\\host\\share\\${addon}`, 'win32')).toBe(`\\\\host\\share\\${addon}`);
  });

  test('a path that is already one spelling is answered unchanged', () => {
    expect(bindingPathKey(`d:\\a\\${addon}`, 'win32')).toBe(`d:\\a\\${addon}`);
  });

  test('reading a key twice says what reading it once said', () => {
    const once = bindingPathKey(`\\\\?\\D:/A/${addon}`, 'win32');

    expect(bindingPathKey(once, 'win32')).toBe(once);
  });

  test('a POSIX path is answered verbatim, case and all', () => {
    // Two files, not one: a POSIX file system holds them apart, and folding
    // the case would merge a real pair.
    expect(bindingPathKey('/A/rs-compiler.linux-x64-gnu.node', 'linux')).toBe(
      '/A/rs-compiler.linux-x64-gnu.node'
    );
    expect(bindingPathKey('/a/x.node', 'linux')).not.toBe(bindingPathKey('/A/x.node', 'linux'));
  });

  test('a backslash in a POSIX name is a character, not a separator', () => {
    expect(bindingPathKey('/a/od\\d.node', 'darwin')).toBe('/a/od\\d.node');
  });

  test('nothing at all is nothing at all, on either platform', () => {
    expect(bindingPathKey('', 'win32')).toBe('');
    expect(bindingPathKey('', 'linux')).toBe('');
  });

  test('a unicode path keeps every character it came with', () => {
    expect(bindingPathKey(`D:\\стиль\\🎉\\${addon}`, 'win32')).toBe(`d:\\стиль\\🎉\\${addon}`);
  });

  test('a path far longer than any file system takes is still keyed', () => {
    const deep = `D:\\${'Directory\\'.repeat(4000)}${addon}`;

    expect(bindingPathKey(deep, 'win32')).toBe(deep.toLowerCase());
  });

  test('a path of nothing but separators is left as it is', () => {
    expect(bindingPathKey('\\\\\\\\', 'win32')).toBe('\\\\\\\\');
  });

  test('the UNC form of the prefix keeps the separators the plain form has', () => {
    expect(bindingPathKey(`\\\\?\\UNC\\host\\share\\${addon}`, 'win32')).toBe(
      bindingPathKey(`\\\\host\\share\\${addon}`, 'win32')
    );
  });
});

/**
 * The one comparison of two binding paths, which is why no caller keys its own.
 *
 * A Windows job read the process's own addon as a second binding because the
 * file system reader and the report reader spelled it differently and the
 * comparison was a plain `Set.has`. What that broke was the reader, not the
 * guard -- Windows holds two bindings safely, so the guard returns before it
 * compares anything there. The predicate is where the spelling is settled, so
 * both readers and the guard answer one question one way.
 */
describe('holdsBinding', () => {
  const addon = 'rs-compiler.win32-x64-msvc.node';

  test('answers for a path the set spells another way', () => {
    const loaded = new Set([`\\\\?\\D:\\A\\dist\\${addon}`]);

    expect(holdsBinding(loaded, `d:/a/dist/${addon}`, 'win32')).toBe(true);
  });

  test('answers for a path the set does not hold at all', () => {
    const loaded = new Set([`D:\\a\\dist\\${addon}`]);

    expect(holdsBinding(loaded, `D:\\b\\dist\\${addon}`, 'win32')).toBe(false);
  });

  test('holds a POSIX path to its own case', () => {
    const loaded = new Set(['/a/rs-compiler.linux-x64-gnu.node']);

    expect(holdsBinding(loaded, '/a/rs-compiler.linux-x64-gnu.node', 'linux')).toBe(true);
    expect(holdsBinding(loaded, '/A/rs-compiler.linux-x64-gnu.node', 'linux')).toBe(false);
  });

  test('an empty set holds nothing', () => {
    expect(holdsBinding(new Set(), `D:\\a\\${addon}`, 'win32')).toBe(false);
  });

  test('a very large set is searched to its end', () => {
    const loaded = new Set(Array.from({ length: 5000 }, (_, index) => `D:\\a\\b-${index}.node`));

    expect(holdsBinding(loaded, 'd:/a/b-4999.node', 'win32')).toBe(true);
    expect(holdsBinding(loaded, 'd:/a/b-5000.node', 'win32')).toBe(false);
  });

  test('the guard asks this predicate rather than the set itself', () => {
    // Read on the platform the guard actually refuses on, where a key is the
    // path: what the case pins is that the guard answers through the predicate,
    // so a platform whose spellings differ is compared the same way.
    const held = '/subject/dist/rs-compiler.darwin-arm64.node';

    expect(attempt('self', [held], [held], 'darwin')).not.toThrow();
    expect(attempt('candidate', ['/other/dist/x.node'], [held], 'darwin')).toThrow(/SIGSEGV/);
  });
});

describe('isCompilerBinding across the platforms', () => {
  test('accepts the addon however Windows spells its name', () => {
    expect(isCompilerBinding('D:\\a\\RS-COMPILER.WIN32-X64-MSVC.NODE', 'win32')).toBe(true);
  });

  test('keeps a POSIX name exact, because the file system does', () => {
    expect(isCompilerBinding('/a/RS-COMPILER.linux-x64-gnu.NODE', 'linux')).toBe(false);
    expect(isCompilerBinding('/a/rs-compiler.linux-x64-gnu.node', 'linux')).toBe(true);
  });

  test('another package addon is not this compiler on either platform', () => {
    expect(isCompilerBinding('D:\\a\\FSEVENTS.NODE', 'win32')).toBe(false);
    expect(isCompilerBinding('/a/fsevents.node', 'linux')).toBe(false);
  });
});
