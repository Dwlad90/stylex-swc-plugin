/**
 * Rules about how many native bindings one process can hold.
 *
 * The paired benchmark loads a base subject and a candidate subject together,
 * so it can measure both on one runner in one process. This works on Linux,
 * which is where CI runs the gate. It does not work on macOS: the second
 * binding kills the process with SIGSEGV.
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

/** File extension of a Node native addon. */
const NATIVE_EXTENSION = '.node';

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
 * Lists the native bindings that a subject package holds.
 *
 * Reads `<packageDir>/dist` and returns the real path of each `.node` file.
 * Real paths let two directories that link to one file count as one binding.
 *
 * Returns an empty list when the directory is absent or holds no addon. A
 * caller must not fail for that reason: the entry point decides whether the
 * subject can run, and it gives a better message than this function can.
 */
export function findNativeBindings(packageDir: string): string[] {
  const distDir = path.join(packageDir, 'dist');

  let entries: string[];
  try {
    entries = fs.readdirSync(distDir);
  } catch {
    return [];
  }

  const found = new Set<string>();
  for (const entry of entries) {
    if (!entry.endsWith(NATIVE_EXTENSION)) continue;
    const full = path.join(distDir, entry);
    try {
      found.add(fs.realpathSync(full));
    } catch {
      // A broken link names no file. Nothing can load it, so skip it.
    }
  }

  return [...found].toSorted();
}

/**
 * Lists the native bindings that the process holds now.
 *
 * Reads the Node diagnostic report, which names every shared object that the
 * process loaded. This finds a binding that any module pulled in, not only one
 * that `loadSubject` asked for. `benchmark/lib/types.ts` reads an enum off the
 * package's own build, so the harness holds a binding before the first subject
 * arrives, and a list that counted only subjects would miss it.
 *
 * Returns an empty set when the runtime gives no report. The guard then permits
 * the load, because a guard must not stop a run on a fact it cannot read.
 */
export function loadedNativeBindings(): Set<string> {
  let sharedObjects: readonly string[];
  try {
    // `getReport` is typed as `object`. The report names the loaded shared
    // objects under `sharedObjects`, so read that field and check its shape.
    const report = process.report?.getReport() as { sharedObjects?: unknown } | undefined;
    const listed = report?.sharedObjects;
    sharedObjects = Array.isArray(listed) ? listed.filter(item => typeof item === 'string') : [];
  } catch {
    return new Set();
  }

  const loaded = new Set<string>();
  for (const object of sharedObjects) {
    if (!object.endsWith(NATIVE_EXTENSION)) continue;
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
