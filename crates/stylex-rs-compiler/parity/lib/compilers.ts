/**
 * Loading the two compilers, and the options both are handed.
 *
 * Shared because there are two harnesses now — the value comparison and the
 * refusal-position comparison — and every line here is a place where they must
 * agree. A second copy of "how the plugin is resolved" or of the option object
 * would let one harness measure a differently configured compiler than the
 * other, and report the difference as a divergence between the compilers.
 */

import { createRequire } from 'node:module';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import type * as babel from '@babel/core';
import stylexBabelPluginModule from '@stylexjs/babel-plugin';

import type { StyleXOptions } from '../../dist/index.js';
import { isRecord } from './guards.js';

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
