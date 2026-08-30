/**
 * Rules about how many native bindings one process can hold.
 *
 * The paired benchmark loads a base subject and a candidate subject together,
 * so it can measure both on one runner in one process. This works on Linux,
 * which is where CI runs the gate. It does not work on macOS: the second
 * binding stops the process with SIGSEGV.
 *
 * A SIGSEGV gives no message and no exit code that names a cause. A benchmark
 * that dies without a message is the failure that the performance policy warns
 * about, because a run that stopped and a run that was fast look the same. The
 * checks here find the unsafe load before it happens and stop the run with a
 * message that says what to do.
 *
 * `assertBindingCanLoad` takes the platform and the loaded set as arguments. It
 * reads no global state, so a test can supply any platform.
 */

import fs from 'node:fs';
import path from 'node:path';

import { isRecord } from './json.js';

/** File extension of a Node native addon. */
const NATIVE_EXTENSION = '.node';

/**
 * Name that NAPI gives the addon file, from `napi.binaryName` in the package
 * manifest. Every build writes `rs-compiler.<target>.node`, in the package
 * `dist` and in each platform package. `native-bindings.test.ts` compares this
 * against the manifest, so the two cannot drift apart.
 */
export const NATIVE_BINARY_NAME = 'rs-compiler';

/** Scope that holds the per-platform packages, such as `@stylexswc/rs-compiler-darwin-arm64`. */
const PLATFORM_PACKAGE_SCOPE = '@stylexswc';

/**
 * Platforms that cannot hold two different native bindings in one process.
 *
 * macOS is the only one that is known to fail. Measured on `darwin` arm64 with
 * Node 24: one binding does 4,000 transforms and survives, and two bindings
 * stop the process. `require`, dynamic `import`, and `process.dlopen` with
 * `RTLD_LOCAL` all fail.
 */
const DUAL_LOAD_UNSAFE_PLATFORMS: ReadonlySet<NodeJS.Platform> = new Set(['darwin']);

export function isDualLoadUnsafe(platform: NodeJS.Platform = process.platform): boolean {
  return DUAL_LOAD_UNSAFE_PLATFORMS.has(platform);
}

/**
 * Whether a file is an addon that this compiler builds.
 *
 * Reads the name, because the loaded list holds every addon in the process and
 * most of them belong to other packages. A watcher such as `fsevents` must not
 * count as a second compiler binding, or the guard stops a run that is safe.
 */
export function isCompilerBinding(file: string): boolean {
  const name = path.basename(file);
  return name.startsWith(`${NATIVE_BINARY_NAME}.`) && name.endsWith(NATIVE_EXTENSION);
}

/** Real paths of the addons that lie directly in one directory. */
function addonsIn(dir: string): string[] {
  let entries: string[];
  try {
    entries = fs.readdirSync(dir);
  } catch {
    return [];
  }

  const found: string[] = [];
  for (const entry of entries) {
    if (!entry.endsWith(NATIVE_EXTENSION)) continue;
    try {
      found.push(fs.realpathSync(path.join(dir, entry)));
    } catch {
      // A broken link names no file. Nothing can load it, so skip it.
    }
  }

  return found;
}

/**
 * Lists the native bindings that a subject package can load.
 *
 * `dist/transform.js` looks for the addon in three places, and this function
 * reads all three. A published package is the reason: `files` in the manifest
 * ships `dist/index.js` and `dist/transform.js` but no addon, so a subject
 * unpacked from the registry keeps its addon in a platform package under
 * `node_modules`. A search of `dist` alone finds nothing there, and the guard
 * would then permit the load that stops the process.
 *
 * The three places, in the order that `transform.js` tries them:
 *   1. the file that `NAPI_RS_NATIVE_LIBRARY_PATH` names;
 *   2. `<packageDir>/dist`;
 *   3. each `<packageDir>/node_modules/@stylexswc/rs-compiler-*` package.
 *
 * Returns an empty list when the package holds no addon. A caller must not fail
 * for that reason: the entry point decides whether the subject can run, and it
 * gives a better message than this function can.
 */
export function findNativeBindings(packageDir: string): string[] {
  const found = new Set<string>();

  const override = process.env.NAPI_RS_NATIVE_LIBRARY_PATH;
  if (override) {
    try {
      found.add(fs.realpathSync(override));
    } catch {
      // The variable names a file that is not there. Nothing can load it.
    }
  }

  for (const addon of addonsIn(path.join(packageDir, 'dist'))) found.add(addon);

  const scopeDir = path.join(packageDir, 'node_modules', PLATFORM_PACKAGE_SCOPE);
  let platformPackages: string[];
  try {
    platformPackages = fs.readdirSync(scopeDir);
  } catch {
    platformPackages = [];
  }

  for (const name of platformPackages) {
    if (!name.startsWith(`${NATIVE_BINARY_NAME}-`)) continue;
    for (const addon of addonsIn(path.join(scopeDir, name))) found.add(addon);
  }

  return [...found].toSorted();
}

/**
 * Lists the compiler bindings that the process holds now.
 *
 * Reads the Node diagnostic report, which names every shared object that the
 * process loaded. This finds a binding that any module pulled in, not only one
 * that `loadSubject` asked for. `benchmark/lib/types.ts` reads an enum off the
 * package's own build, so the harness holds a binding before the first subject
 * arrives, and a list that counted only subjects would miss it.
 *
 * Keeps the addons of this compiler and drops the rest, because the process
 * holds addons of other packages that cannot conflict with a subject.
 *
 * Returns an empty set when the runtime gives no report. The guard then permits
 * the load, because a guard must not stop a run on a fact it cannot read.
 */
export function loadedNativeBindings(): Set<string> {
  let sharedObjects: readonly unknown[];
  try {
    const report: unknown = process.report?.getReport();
    sharedObjects =
      isRecord(report) && Array.isArray(report.sharedObjects) ? report.sharedObjects : [];
  } catch {
    return new Set();
  }

  const override = process.env.NAPI_RS_NATIVE_LIBRARY_PATH;
  const loaded = new Set<string>();
  for (const object of sharedObjects) {
    if (typeof object !== 'string') continue;
    // The override can name a file that the compiler naming rule does not
    // match, so accept it as well as a file with the standard name.
    if (!isCompilerBinding(object) && object !== override) continue;
    try {
      loaded.add(fs.realpathSync(object));
    } catch {
      // The file is gone. It cannot conflict with a load that comes now.
    }
  }

  return loaded;
}

export interface BindingLoadRequest {
  /** Name of the subject, for the message. */
  label: string;
  /** Bindings that this subject brings in. */
  bindings: readonly string[];
  /** Bindings that the process already holds. */
  loaded: ReadonlySet<string>;
  platform?: NodeJS.Platform;
}

/**
 * Stops a load that would put a second native binding in the process.
 *
 * A subject that brings in a binding which is already loaded is safe, because
 * the runtime gives back the same instance. Only a new and different binding
 * is a risk.
 *
 * @throws Error when the platform cannot hold the new binding.
 */
export function assertBindingCanLoad(request: BindingLoadRequest): void {
  if (!isDualLoadUnsafe(request.platform)) return;
  if (request.loaded.size === 0) return;

  const conflicting = request.bindings.filter(binding => !request.loaded.has(binding));
  if (conflicting.length === 0) return;

  const platform = request.platform ?? process.platform;
  throw new Error(
    `Cannot load subject "${request.label}": ${platform} cannot hold two ` +
      'different native bindings in one process, and the process already ' +
      `holds ${[...request.loaded].join(', ')}. Loading ${conflicting.join(', ')} ` +
      'stops the process with SIGSEGV and reports no result. Run the paired ' +
      'benchmark on Linux, or measure each revision in its own process and ' +
      'compare the two reports.'
  );
}
