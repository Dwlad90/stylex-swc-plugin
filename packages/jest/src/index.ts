import { createHash } from 'node:crypto';
import { statSync } from 'node:fs';
import path from 'node:path';

import type { TransformerCreator, SyncTransformer } from '@jest/transform';
import type { Config } from '@jest/types';
import { transform, normalizeRsOptions, shouldTransformFile } from '@stylexswc/rs-compiler';
import type { StyleXOptions } from '@stylexswc/rs-compiler';

type TransformerConfig = Config.TransformerConfig[1];

/**
 * Reads `@stylexswc/rs-compiler`'s declared version.
 *
 * Guarded, the way `@stylexswc/plugin-shared` guards the same read: this is a
 * subpath that `@stylexswc/rs-compiler` does not declare in an `exports` map,
 * so it resolves only by legacy lookup. If that package ever gains an `exports`
 * map without `"./package.json"`, an unguarded call here would throw during
 * module evaluation and fail the entire Jest run — not one file — for nothing
 * more than a cache-key ingredient.
 *
 * This package is published as CommonJS and Jest loads it with `require`, so
 * reading the manifest synchronously is the correct thing to do here.
 */
function readCompilerVersion(): string {
  try {
    // oxlint-disable-next-line typescript/no-require-imports
    const manifest = require('@stylexswc/rs-compiler/package.json') as { version?: string };
    return manifest.version ?? 'unknown';
  } catch {
    return 'unknown';
  }
}

/**
 * Identifies the native addon that the compiler actually dlopen'd, by size and
 * mtime.
 *
 * The declared version alone is not enough during development: rebuilding the
 * Rust crate replaces `rs-compiler.<triple>.node` in place without touching
 * `package.json`, so nothing in the key moves and Jest happily replays output
 * from the previous binary. That is the same staleness the version field was
 * added to prevent, displaced from "a consumer upgraded" to "you just rebuilt".
 *
 * Node registers native addons in `require.cache` under their absolute path,
 * and `@stylexswc/rs-compiler` was imported above — its `require` is hoisted
 * ahead of this initializer in the emitted CommonJS — so by now the entry is
 * present and names the exact file in use. Matching on the NAPI-RS basename
 * convention rather than on a directory covers both layouts: the in-repo build
 * writes the addon beside the loader, while published installs get it from a
 * per-platform optional dependency.
 *
 * Returns an empty string when no addon is found, which is the correct answer
 * rather than a failure: a published install pins its platform package to the
 * compiler version, so the version alone already distinguishes those.
 */
function readNativeBindingStamp(): string {
  try {
    const addonPath = Object.keys(require.cache).find(id => {
      const base = path.basename(id);
      return base.startsWith('rs-compiler.') && base.endsWith('.node');
    });
    if (!addonPath) return '';

    const { size, mtimeMs } = statSync(addonPath);
    return `${size}:${mtimeMs}`;
  } catch {
    // A stat failure must not be fatal; the version below still applies.
    return '';
  }
}

/**
 * Transformed output is only valid for the compiler that produced it, so the
 * compiler's identity has to participate in the cache key. Computed once per
 * worker at module load — not per file — so the `statSync` cost is a single
 * syscall per Jest process.
 */
const COMPILER_FINGERPRINT = `${readCompilerVersion()}|${readNativeBindingStamp()}`;

export interface JestTransformerConfig extends TransformerConfig {
  rsOptions?: StyleXOptions;
}

const process: SyncTransformer<JestTransformerConfig>['process'] = function process(
  sourceText,
  sourcePath,
  options
) {
  // Destructure rather than delete: `transformerConfig` is created once per run
  // and shared across every file Jest transforms, so mutating it here would
  // strip the patterns for all later files.
  const { include, exclude, ...compilerOptions } = options.transformerConfig.rsOptions ?? {};

  // Check if file should be transformed based on include/exclude patterns
  if (!shouldTransformFile(sourcePath, include, exclude)) {
    return { code: sourceText };
  }

  const { code } = transform(sourcePath, sourceText, normalizeRsOptions(compilerOptions));

  return { code };
};

const processAsync: SyncTransformer<JestTransformerConfig>['processAsync'] =
  async function processAsync(sourceText, sourcePath, options) {
    return process(sourceText, sourcePath, options);
  };

const getCacheKey: SyncTransformer<JestTransformerConfig>['getCacheKey'] = function getCacheKey(
  sourceText,
  sourcePath,
  options
) {
  // The parts are separated by NUL rather than concatenated: `update` appends
  // raw bytes, so without a delimiter a source ending in the path prefix would
  // hash identically to a different (source, path) pair.
  const hash = createHash('sha256');
  for (const part of [
    sourceText,
    sourcePath,
    JSON.stringify(options.transformerConfig),
    // Without this, upgrading — or locally rebuilding — the compiler leaves
    // Jest replaying output the previous compiler produced, because nothing
    // else in the key moves.
    COMPILER_FINGERPRINT,
  ]) {
    hash.update(part);
    hash.update('\0');
  }

  return hash.digest('hex');
};

/**
 * What this package actually returns: a synchronous transformer that always
 * provides all three members.
 *
 * `SyncTransformer` marks `processAsync` and `getCacheKey` optional, because a
 * transformer in general may omit them. This one never does, and saying so is
 * what lets a consumer call `getCacheKey` without a non-null assertion.
 */
export type StyleXJestTransformer = SyncTransformer<JestTransformerConfig> &
  Required<Pick<SyncTransformer<JestTransformerConfig>, 'processAsync' | 'getCacheKey'>>;

// Annotated with the concrete return type and `satisfies`-checked against
// Jest's contract, rather than typed as `TransformerCreator` directly.
// `TransformerCreator` returns `T | Promise<T>`, so under the direct
// annotation every consumer — including this package's own test suite — saw
// `Promise<SyncTransformer<…>> | SyncTransformer<…>` and could not reach
// `.process` without narrowing a union this factory never actually produces.
// `satisfies` keeps the contract enforced at compile time while publishing the
// precise type.
const createTransformer = ((): StyleXJestTransformer => {
  return {
    process,
    processAsync,
    getCacheKey,
  };
}) satisfies TransformerCreator<SyncTransformer<JestTransformerConfig>, JestTransformerConfig>;

// Both forms, deliberately. Jest loads a transformer through
// `interopRequireDefault(require(path)).default`, so the module must expose a
// `default` carrying `createTransformer`. The previous shape got there by
// accident: `module.exports = { createTransformer }` replaced the exports
// object, which left TypeScript's generated `exports.default = ...` writing to
// an orphaned binding and dropped `__esModule` — so interop fell into its
// `{ default: moduleExports }` branch and happened to produce the right thing.
// It worked, but the emitted `.d.ts` declared only a default export while the
// runtime had only a named one, which `attw` reports as `MissingExportEquals`.
//
// Exporting both makes the declaration and the implementation agree, and is
// strictly more compatible than either half alone: interop-aware loaders take
// `default`, while anything reading the module object directly still finds
// `createTransformer`.
export { createTransformer };
export default { createTransformer };
