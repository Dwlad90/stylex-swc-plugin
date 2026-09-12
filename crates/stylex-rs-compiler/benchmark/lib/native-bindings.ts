/**
 * Rules about how many native bindings one process can hold.
 *
 * The paired benchmark loads a base subject and a candidate subject together,
 * so it can measure both on one runner in one process. This works on Linux,
 * which is where CI runs the gate. On macOS it works only while at most one of
 * the two bindings links mimalloc. Two mimalloc bindings stop the process with
 * SIGSEGV.
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
import { realPathOf, settledPathOf } from './paths.js';

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
 * Platforms that hold two native bindings together only under a condition.
 *
 * macOS is the only one that is known to. Measured on `darwin` arm64 with Node
 * 24, one fixture and one round for each pair: a base of `0.18.6`, which links
 * no mimalloc, measured both subjects and left with exit 0, while a base of
 * `0.19.0-rc.1` and one of `0.19.0-rc.2`, which both link it, each stopped the
 * process with SIGSEGV. All three version strings differ from the candidate, so
 * the cause is not a shared version: it is the second mimalloc. What the second
 * copy does to the first is not known -- the addon references no macOS zone
 * call, so it holds the allocator as a plain Rust global one -- and the guard
 * does not need to know. `require`, dynamic `import`, and `process.dlopen` with
 * `RTLD_LOCAL` all fail the same way.
 *
 * The name says restricted and not unsafe because the platform alone does not
 * decide. It is one of the two conditions, and `assertBindingCanLoad` reads the
 * allocator for the other, so only the pair that was measured to fail is
 * refused.
 */
const DUAL_LOAD_RESTRICTED_PLATFORMS: ReadonlySet<NodeJS.Platform> = new Set(['darwin']);

export function isDualLoadRestricted(platform: NodeJS.Platform = process.platform): boolean {
  return DUAL_LOAD_RESTRICTED_PLATFORMS.has(platform);
}

/** The long-path prefix `realpath` may put in front of a Windows path. */
const WINDOWS_LONG_PATH = /^\\\\\?\\(UNC\\)?/;

/**
 * One spelling of a path, so two readers of the same file agree about it.
 *
 * The guard compares the bindings a subject would load against the bindings the
 * process holds, and the two come from different readers: one from the file
 * system, one from the diagnostic report the runtime writes. On Linux and macOS
 * both answer the same string. On Windows they need not: the file system is
 * case-insensitive, the loader records the case it was handed, `realpath` may
 * answer with a `\\?\` long-path prefix, and either may spell a separator the
 * other way. Compared as plain strings, the process's own binding then reads as
 * a second one -- which is exactly the reading the guard exists to prevent.
 *
 * Case is folded on Windows only. A POSIX file system holds `A.node` and
 * `a.node` apart, so folding there would merge two real files.
 */
export function bindingPathKey(file: string, platform: NodeJS.Platform = process.platform): string {
  if (platform !== 'win32') return file;

  // `\\?\UNC\host\share` and `\\host\share` name one file, so the UNC form
  // keeps the two separators the plain form spells it with.
  return file
    .replace(WINDOWS_LONG_PATH, (_, unc: string | undefined) => (unc === undefined ? '' : '\\\\'))
    .replaceAll('/', '\\')
    .toLowerCase();
}

/**
 * Whether a file is an addon that this compiler builds.
 *
 * Reads the name, because the loaded list holds every addon in the process and
 * most of them belong to other packages. A watcher such as `fsevents` must not
 * count as a second compiler binding, or the guard stops a run that is safe.
 *
 * Named from the one spelling rather than from the path as written, so the rule
 * that folds a Windows name lives in `bindingPathKey` alone. A name this reader
 * dropped for its case would leave the guard blind to a binding the process
 * holds. The basename is taken with the parser of the platform named rather
 * than of the host, because the platform is an argument here: a backslash is a
 * separator on Windows and an ordinary character everywhere else.
 */
export function isCompilerBinding(
  file: string,
  platform: NodeJS.Platform = process.platform
): boolean {
  const parser = platform === 'win32' ? path.win32 : path.posix;
  const name = parser.basename(bindingPathKey(file, platform));

  return name.startsWith(`${NATIVE_BINARY_NAME}.`) && name.endsWith(NATIVE_EXTENSION);
}

/**
 * Whether `loaded` already holds `binding`, whatever either one calls it.
 *
 * The one place a binding path is compared, so no caller has to remember that
 * two readers spell a Windows path differently -- which is the mistake that put
 * the process's own addon in neither set.
 */
export function holdsBinding(
  loaded: ReadonlySet<string>,
  binding: string,
  platform: NodeJS.Platform = process.platform
): boolean {
  const wanted = bindingPathKey(binding, platform);

  for (const held of loaded) {
    if (bindingPathKey(held, platform) === wanted) return true;
  }

  return false;
}

/** Names in one directory, or nothing when there is no such directory. */
function entriesIn(dir: string): string[] {
  try {
    return fs.readdirSync(dir);
  } catch {
    return [];
  }
}

/** Real paths of the addons that lie directly in one directory. */
function addonsIn(dir: string): string[] {
  const found: string[] = [];
  for (const entry of entriesIn(dir)) {
    if (!entry.endsWith(NATIVE_EXTENSION)) continue;
    try {
      found.push(realPathOf(path.join(dir, entry)));
    } catch {
      // A broken link names no file. Nothing can load it, so skip it.
    }
  }

  return found;
}

/**
 * The directories that Node searches for a package name required from `from`.
 *
 * `dist/transform.js` asks for the platform package by name, so where that name
 * resolves is Node's rule and not this module's. The rule is every ancestor of
 * the asking file with `node_modules` appended, nearest first, and an ancestor
 * that is itself a `node_modules` directory contributes nothing.
 */
function nodeModulesChain(from: string): string[] {
  const chain: string[] = [];

  let dir = from;
  for (;;) {
    if (path.basename(dir) !== 'node_modules') chain.push(path.join(dir, 'node_modules'));
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }

  return chain;
}

/**
 * Lists the native bindings that a subject package can load.
 *
 * `dist/transform.js` looks for the addon in three places, and this function
 * reads all three. A published package is the reason: `files` in the manifest
 * ships `dist/index.js` and `dist/transform.js` but no addon, so a subject
 * unpacked from the registry keeps its addon in a platform package.
 *
 * The three places, in the order that `transform.js` tries them:
 *   1. the file that `NAPI_RS_NATIVE_LIBRARY_PATH` names;
 *   2. `<packageDir>/dist`;
 *   3. the `@stylexswc/rs-compiler-*` package that the name resolves to.
 *
 * The third one is a search and not a single directory. An installer is free to
 * put the platform package where it likes, and both of the ones this repository
 * uses put it somewhere other than under the compiler package: npm hoists it to
 * a sibling, and pnpm links it beside the copy under `node_modules/.pnpm`. A
 * search of `<packageDir>/node_modules` alone found nothing for either layout,
 * so the guard read an installed base as holding no binding and permitted the
 * load that stops the process. The whole chain is read instead, from the real
 * path, which is where Node resolves a linked module from.
 *
 * The answer is a superset of what the subject loads: it holds every platform
 * package the chain reaches, and the subject loads one addon. That is the side
 * to err on. A binding too many makes the guard refuse a pair that would have
 * run, which costs a run and says why; a binding too few makes it permit the
 * pair that ends the process without a word.
 *
 * Returns an empty list when the package holds no addon, and when it holds one
 * in a layout nobody has read yet. This function cannot tell those two apart,
 * so it fails for neither. `assertBindingIsVisible` decides what an empty list
 * means for the caller that asked.
 */
export function findNativeBindings(packageDir: string): string[] {
  const found = new Set<string>();

  const override = process.env.NAPI_RS_NATIVE_LIBRARY_PATH;
  if (override) {
    try {
      found.add(realPathOf(override));
    } catch {
      // The variable names a file that is not there. Nothing can load it.
    }
  }

  for (const addon of addonsIn(path.join(packageDir, 'dist'))) found.add(addon);

  for (const modulesDir of nodeModulesChain(settledPathOf(packageDir))) {
    const scopeDir = path.join(modulesDir, PLATFORM_PACKAGE_SCOPE);
    for (const name of entriesIn(scopeDir)) {
      if (!name.startsWith(`${NATIVE_BINARY_NAME}-`)) continue;
      for (const addon of addonsIn(path.join(scopeDir, name))) found.add(addon);
    }
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

  // Settled before it is keyed, because the report and the variable are not
  // written by the same hand. The runtime reports a path the operating system
  // resolved; the variable holds whatever the caller typed. `bindingPathKey`
  // folds case and separators but cannot follow a link or expand a short name,
  // so an override under a symlinked temp directory -- which is every temp
  // directory on macOS -- would key differently from the same file in the
  // report. The binding would then be dropped, the set would read empty, and
  // the guard would permit the dual load it exists to stop, on the one platform
  // where that load ends the process.
  const override = process.env.NAPI_RS_NATIVE_LIBRARY_PATH;
  const overrideKey = override === undefined ? undefined : bindingPathKey(settledPathOf(override));
  const loaded = new Set<string>();
  for (const object of sharedObjects) {
    if (typeof object !== 'string') continue;
    // The override can name a file that the compiler naming rule does not
    // match, so accept it as well as a file with the standard name.
    if (!isCompilerBinding(object) && bindingPathKey(settledPathOf(object)) !== overrideKey)
      continue;
    try {
      loaded.add(realPathOf(object));
    } catch {
      // The file is gone. It cannot conflict with a load that comes now.
    }
  }

  return loaded;
}

/**
 * Bytes that a build with mimalloc holds and a build without it does not.
 *
 * The allocator writes its own name into the messages it can print, so the name
 * is in the binary whenever the allocator is linked into it. Read from the
 * shipped files rather than from the build settings: `0.18.6` holds the bytes
 * nowhere, and `0.19.0-rc.1`, `0.19.0-rc.2` and the current build each hold
 * them six times. Every target this package publishes holds them, musl
 * included: `swc_malloc` leaves musl on the system allocator, and this package
 * names mimalloc for `x86_64-unknown-linux-musl` itself, in its own manifest.
 *
 * The rule holds for the two artifacts a paired run compares -- a build of this
 * package and a published release of it -- because `napi build` writes both and
 * strips neither. It is not a rule about any binary: a build that links the
 * allocator and keeps none of its messages would read as one that does not.
 * `native-bindings.test.ts` reads the build on disk, so a pipeline that begins
 * to strip the name fails there rather than at a release.
 */
const MIMALLOC_MARKER = Buffer.from('mimalloc', 'utf8');

/**
 * Whether an addon file brings its own copy of mimalloc.
 *
 * A file this function cannot read counts as one that does, because a guard
 * must not permit a load it has no evidence about. That answer is the older and
 * stricter one, which refused every second binding on the platform, so nothing
 * that ran before can start failing for this reason.
 */
export function linksMimalloc(file: string): boolean {
  try {
    return fs.readFileSync(file).includes(MIMALLOC_MARKER);
  } catch {
    return true;
  }
}

/** The bindings of a list that bring their own copy of mimalloc. */
function mimallocBindings(bindings: Iterable<string>): string[] {
  return [...bindings].filter(binding => linksMimalloc(binding));
}

/**
 * Whether any binding of a list brings mimalloc.
 *
 * Stops at the first one that does, because each answer is read off a file and
 * the question is only whether there is one.
 */
function anyLinksMimalloc(bindings: Iterable<string>): boolean {
  for (const binding of bindings) {
    if (linksMimalloc(binding)) return true;
  }

  return false;
}

/** The bindings a subject brings that the process does not hold already. */
function arrivingBindings(
  bindings: readonly string[],
  loaded: ReadonlySet<string>,
  platform?: NodeJS.Platform
): string[] {
  return bindings.filter(binding => !holdsBinding(loaded, binding, platform));
}

/**
 * Whether two subjects can be timed in one process.
 *
 * The question `assertBindingCanLoad` answers after the fact, asked before
 * either subject is loaded. A caller that gets `false` must time each subject
 * in a process of its own; a caller that ignores the answer gets the SIGSEGV
 * the guard describes.
 *
 * Answers `false` for everything the two guards refuse, and for the case they
 * refuse without naming a binding: a subject whose binding was not found.
 * `findNativeBindings` reads the layouts npm and pnpm write, and a third layout
 * would give an empty list, which cannot be read as "brings nothing" here any
 * more than it can there. A pair this function cannot clear is measured in two
 * processes, which is always safe and only costs the start-up of a second one.
 */
export function subjectsCanShareProcess(
  first: readonly string[],
  second: readonly string[],
  platform: NodeJS.Platform = process.platform
): boolean {
  if (!isDualLoadRestricted(platform)) return true;
  if (first.length === 0 || second.length === 0) return false;

  const arriving = arrivingBindings(second, new Set(first), platform);
  if (arriving.length === 0) return true;

  return !anyLinksMimalloc(arriving) || !anyLinksMimalloc(first);
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
 * Stops a subject whose binding this module could not find.
 *
 * `findNativeBindings` reads the layouts that npm and pnpm write. An installer
 * that writes another one gives an empty list, and an empty list is what the
 * rest of this module reads as "brings nothing to conflict with" -- which is
 * how a base installed for a release passed a guard that exists to stop it.
 *
 * A subject that `loadSubject` loads is a compiler package, so it has an addon
 * whether or not this module found it. Where the process already holds a
 * mimalloc binding on a restricted platform, an unfound one cannot be ruled
 * out, and a sentence is a better answer than a SIGSEGV. Everywhere else the
 * question does not arise and the subject loads.
 *
 * @throws Error when the subject must hold a binding and none was found.
 */
export function assertBindingIsVisible(request: BindingLoadRequest): void {
  if (!isDualLoadRestricted(request.platform)) return;
  if (request.bindings.length > 0) return;
  const held = mimallocBindings(request.loaded);
  if (held.length === 0) return;

  throw new Error(
    `Cannot load subject "${request.label}": the process holds ${held.join(', ')}, ` +
      'which links mimalloc, and no native binding was found for this subject. ' +
      'A second mimalloc binding stops the process with SIGSEGV and reports no ' +
      'result, and this one cannot be read to say whether it is one. Measure ' +
      'each revision in its own process, which `bench:revisions` does on this ' +
      'platform and `--separate-processes` asks for anywhere.'
  );
}

/**
 * Stops a load that would put a second mimalloc binding in the process.
 *
 * A subject that brings in a binding which is already loaded is safe, because
 * the runtime gives back the same instance. Only a new and different binding is
 * a risk, and only while both it and one the process holds link mimalloc.
 *
 * The allocator is read last, because it is the only step that reads a file and
 * the two cheap steps answer most calls: a subject whose bindings are all held
 * already, and every platform but the one.
 *
 * @throws Error when the platform cannot hold the new binding.
 */
export function assertBindingCanLoad(request: BindingLoadRequest): void {
  if (!isDualLoadRestricted(request.platform)) return;
  if (request.loaded.size === 0) return;

  const conflicting = arrivingBindings(request.bindings, request.loaded, request.platform);
  if (conflicting.length === 0) return;

  const arriving = mimallocBindings(conflicting);
  if (arriving.length === 0) return;
  const held = mimallocBindings(request.loaded);
  if (held.length === 0) return;

  const platform = request.platform ?? process.platform;
  throw new Error(
    `Cannot load subject "${request.label}": ${platform} cannot hold two ` +
      'native bindings that both link mimalloc, and the process already holds ' +
      `${held.join(', ')}. Loading ${arriving.join(', ')} stops the process ` +
      'with SIGSEGV and reports no result. Run the paired benchmark on Linux, ' +
      'or measure each revision in its own process, which `bench:revisions` ' +
      'does on this platform and `--separate-processes` asks for anywhere.'
  );
}
