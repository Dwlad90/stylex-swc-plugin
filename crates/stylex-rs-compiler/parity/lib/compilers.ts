/**
 * Loading the two compilers, and the options both are handed.
 *
 * Shared because there are two harnesses now — the value comparison and the
 * refusal-position comparison — and every line here is a place where they must
 * agree. A second copy of "how the plugin is resolved" or of the option object
 * would let one harness measure a differently configured compiler than the
 * other, and report the difference as a divergence between the compilers.
 */

import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import * as babel from '@babel/core';
import stylexBabelPluginModule from '@stylexjs/babel-plugin';

import type { StyleXOptions } from '../../dist/index.js';
import { isRecord, stringAt } from './guards.js';

const require = createRequire(import.meta.url);

/** This compiler's entry point: `transform(filename, code, options)`. */
export type TransformFn = (
  filename: string,
  code: string,
  options: StyleXOptions
) => { metadata: { stylex: unknown[] }; code: string };

/** This compiler, loaded from `dist/` rather than from the Rust sources. */
export interface LoadedRustCompiler {
  transform: TransformFn;
  /** The file it was resolved from, for a report to be attributable. */
  distEntry: string;
}

export async function loadRustCompiler(packageDir: string): Promise<LoadedRustCompiler> {
  const distEntry = path.join(packageDir, 'dist/index.js');
  const loaded: unknown = await import(pathToFileURL(distEntry).href);
  const transform = isRecord(loaded) ? loaded.transform : undefined;
  if (!isTransform(transform)) {
    throw new Error(
      `${distEntry} does not export a transform function — run \`pnpm build\` in this package first.`
    );
  }

  return { transform, distEntry };
}

/** The reference implementation's plugin, and the file it was resolved from. */
export interface LoadedBabelPlugin {
  plugin: babel.PluginTarget;
  pluginEntry: string;
}

export function loadBabelPlugin(): LoadedBabelPlugin {
  // The plugin is published both as a default export and as the module object
  // itself, depending on how the consumer resolves it; either is accepted.
  const pluginModule: unknown = stylexBabelPluginModule;
  const plugin = (isRecord(pluginModule) ? pluginModule.default : undefined) ?? pluginModule;
  if (!isPluginTarget(plugin)) {
    throw new Error('@stylexjs/babel-plugin did not export a Babel plugin function');
  }

  return { plugin, pluginEntry: require.resolve('@stylexjs/babel-plugin') };
}

/**
 * The options every subject is compiled with, before a harness adds its own.
 *
 * `haste` module resolution keeps both compilers from needing a real
 * `node_modules` layout beside the fixture, and `dev: false` keeps debug class
 * names — which encode a file path — out of the comparison.
 */
export function baseStyleXOptions(packageDir: string): StyleXOptions {
  return {
    dev: false,
    unstable_moduleResolution: { type: 'haste', rootDir: packageDir },
  };
}

/**
 * Which build of each compiler a run measured.
 *
 * Every harness prints it, because a report that does not name its subjects
 * cannot be compared with an older one -- and the upstream version is held by
 * the lockfile rather than by an exact range in the catalog, so it moves under a
 * `pnpm update` without anything in this directory changing. That is also what
 * makes it the first thing to read when a CI run starts failing on a corpus
 * nobody touched.
 */
export interface SubjectVersions {
  rust: { version: string; resolvedFrom: string };
  babel: { version: string; resolvedFrom: string };
  babelCore: string;
}

/**
 * Read once here rather than per harness, for the reason the rest of this file
 * exists: three harnesses each resolving their own version strings is three
 * places for a report to become unattributable one at a time.
 */
export function resolveVersions(
  packageDir: string,
  distEntry: string,
  babelPluginEntry: string
): SubjectVersions {
  return {
    rust: {
      version: readVersion(path.join(packageDir, 'package.json')),
      resolvedFrom: distEntry,
    },
    babel: {
      version: readVersion(resolveManifest('@stylexjs/babel-plugin')),
      resolvedFrom: babelPluginEntry,
    },
    babelCore: babel.version,
  };
}

/** The subject block every harness prints above its report. */
export function subjectBlock(versions: SubjectVersions, extra: [string, string][] = []): string {
  const rows: [string, string][] = [
    ['@stylexswc/rs-compiler', `v${versions.rust.version}`],
    ['@stylexjs/babel-plugin', `v${versions.babel.version}`],
    ['@babel/core', `v${versions.babelCore}`],
    ...extra,
  ];

  return rows.map(([name, value]) => `  ${name.padEnd(24)} ${value}`).join('\n');
}

/**
 * Where a package's manifest is, resolved as a package export rather than
 * guessed at as `dirname(entry)/../package.json` — that guess is right only
 * while the entry point sits exactly one directory below the manifest, and
 * `readVersion` answers `unknown` rather than complaining when it is wrong, so
 * a report would quietly stop naming which upstream it was measured against.
 *
 * Falls back to that guess rather than propagating. A package whose `exports`
 * map omits `./package.json` raises `ERR_PACKAGE_PATH_NOT_EXPORTED` here, and
 * a version string the report prints for the reader is not worth failing a
 * measurement run over — `readVersion` degrades a wrong path to `unknown`,
 * which is the outcome this is trying to make rare, not one it must prevent.
 */
function resolveManifest(packageName: string): string {
  try {
    return require.resolve(`${packageName}/package.json`);
  } catch {
    return path.join(path.dirname(require.resolve(packageName)), '../package.json');
  }
}

function readVersion(manifestPath: string): string {
  try {
    return stringAt(JSON.parse(fs.readFileSync(manifestPath, 'utf8')), 'version') ?? 'unknown';
  } catch {
    return 'unknown';
  }
}

/** The message a thrown value carries, however it was thrown. */
export function messageOf(error: unknown): string {
  if (error instanceof Error) return error.message;

  return String(error);
}

function isTransform(value: unknown): value is TransformFn {
  return typeof value === 'function';
}

function isPluginTarget(value: unknown): value is babel.PluginTarget {
  return typeof value === 'function';
}
