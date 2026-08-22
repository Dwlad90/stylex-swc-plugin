/**
 * Single registry of every benchmarkable fixture.
 *
 * The versioned JSON manifest is shared by measurement and the trusted PR
 * reporter, so fixture names, weight classes, categories, and calibrated
 * batch sizes cannot drift across the trust boundary.
 *
 * Batch sizes are all `1` until they are calibrated. Fast fixtures are
 * lifted above sub-millisecond noise by batching; do not add an
 * absolute-delta floor as a shortcut.
 *
 * A fixture may opt into a `dev` build with `"dev": true`. That is per
 * fixture, never a switch in the shared options -- see `FixtureDescriptor.dev`.
 *
 * Only fixtures that actually produce StyleX rules belong here. The
 * transform test corpus also contains negative fixtures that compile to
 * zero rules (`button-props`, which never imports `stylex`); registering
 * one trips the runner's sanity check and fails every benchmark job.
 * `fixtures.test.ts` enforces this against the real binding.
 */

import fs from 'node:fs';
import path from 'node:path';

import {
  BOOLEAN_OPTION_KEYS,
  SOURCE_MAP_SETTINGS,
  STYLE_RESOLUTIONS,
  type BooleanOptionKey,
  type FixtureCategory,
  type FixtureDescriptor,
  type FixtureOptionOverrides,
  type FixtureWeight,
} from './types.js';

export interface FixtureRegistryPaths {
  packageDir: string;
  workspaceRoot: string;
}

export interface LoadFixturesOptions extends FixtureRegistryPaths {
  /** Optional category allowlist. Empty/absent = all categories. */
  categories?: readonly FixtureCategory[];
  /** Optional substring filter applied to fixture names. */
  filter?: readonly string[];
}

/** The manifest key that names a file whose contents become `inputSourceMap`. */
const INPUT_SOURCE_MAP_KEY = 'inputSourceMapFrom';

interface FixtureManifestEntry {
  name: string;
  file: string;
  category: FixtureCategory;
  weight: FixtureWeight;
  batchSize: number;
  dev?: boolean;
  options?: FixtureOptionOverrides;
}

export function loadAllFixtures(options: LoadFixturesOptions): FixtureDescriptor[] {
  const requested = new Set<FixtureCategory>(
    options.categories && options.categories.length > 0
      ? options.categories
      : (['transform', 'perf', 'rollup'] as const)
  );

  const all = loadManifest(options.packageDir, options.workspaceRoot)
    .filter(fixture => requested.has(fixture.category))
    .map(fixture => {
      const filePath = path.join(options.workspaceRoot, fixture.file);
      const descriptor: FixtureDescriptor = {
        name: fixture.name,
        filePath,
        code: fs.readFileSync(filePath, 'utf-8'),
        weight: fixture.weight,
        category: fixture.category,
        batchSize: fixture.batchSize,
      };
      // Assigned only when the manifest declared it, so an undeclared `dev`
      // stays absent rather than becoming an own `dev: undefined` property:
      // "the manifest did not say" and "the manifest said nothing in
      // particular" must not read the same to a consumer. Same shape as
      // `parseManifestEntry`.
      if (fixture.dev !== undefined) descriptor.dev = fixture.dev;
      if (fixture.options !== undefined) descriptor.options = fixture.options;
      return descriptor;
    });

  if (!options.filter || options.filter.length === 0) return all;
  return all.filter(fixture => options.filter!.some(needle => fixture.name.includes(needle)));
}

function loadManifest(packageDir: string, workspaceRoot: string): readonly FixtureManifestEntry[] {
  const manifestPath = path.join(packageDir, 'benchmark', 'fixtures.v1.json');
  const input = JSON.parse(fs.readFileSync(manifestPath, 'utf8')) as unknown;
  if (!isRecord(input) || input.schemaVersion !== 1 || !Array.isArray(input.fixtures)) {
    throw new Error('Benchmark fixture manifest must use schema version 1');
  }

  const fixtures = input.fixtures.map((fixture, index) =>
    parseManifestEntry(fixture, index, workspaceRoot)
  );
  if (
    fixtures.length === 0 ||
    new Set(fixtures.map(fixture => fixture.name)).size !== fixtures.length
  ) {
    throw new Error('Benchmark fixture manifest names must be non-empty and unique');
  }
  return fixtures;
}

function parseManifestEntry(
  input: unknown,
  index: number,
  workspaceRoot: string
): FixtureManifestEntry {
  const context = `Benchmark fixture manifest entry ${String(index)}`;
  if (!isRecord(input)) throw new Error(`${context} must be an object`);
  if (typeof input.name !== 'string' || input.name.length === 0) {
    throw new Error(`${context}.name must be a non-empty string`);
  }
  if (
    typeof input.file !== 'string' ||
    input.file.length === 0 ||
    path.isAbsolute(input.file) ||
    input.file.split(/[\\/]/).includes('..')
  ) {
    throw new Error(`${context}.file must be a relative path`);
  }
  if (input.category !== 'transform' && input.category !== 'perf' && input.category !== 'rollup') {
    throw new Error(`${context}.category is unsupported`);
  }
  if (input.weight !== 'standard' && input.weight !== 'heavy') {
    throw new Error(`${context}.weight is unsupported`);
  }
  if (!Number.isSafeInteger(input.batchSize) || Number(input.batchSize) <= 0) {
    throw new Error(`${context}.batchSize must be a positive integer`);
  }
  // Rejected rather than coerced: `"false"` and `0` are both truthy-adjacent
  // mistakes that would silently benchmark the wrong configuration, and a
  // fixture's shape is not something to guess at.
  if (input.dev !== undefined && typeof input.dev !== 'boolean') {
    throw new Error(`${context}.dev must be a boolean when present`);
  }
  const entry: FixtureManifestEntry = {
    name: input.name,
    file: input.file,
    category: input.category,
    weight: input.weight,
    batchSize: Number(input.batchSize),
  };

  // Copied the same way `loadAllFixtures` copies it onto the descriptor, and
  // for the same reason: an undeclared `dev` must stay absent rather than
  // become an own `dev: undefined` property.
  if (input.dev !== undefined) entry.dev = input.dev;
  if (input.options !== undefined) {
    entry.options = parseOptionOverrides(input.options, `${context}.options`, workspaceRoot);
  }

  return entry;
}

/**
 * One fixture's option overrides, narrowed key by key.
 *
 * An unknown key is an error rather than a key dropped: a manifest that names
 * `enableDebugDataProps` would otherwise be measured under the production shape
 * while claiming to price the debug one, and the number it reports would look
 * entirely reasonable.
 */
function parseOptionOverrides(
  input: unknown,
  context: string,
  workspaceRoot: string
): FixtureOptionOverrides {
  if (!isRecord(input)) throw new Error(`${context} must be an object`);

  const overrides: FixtureOptionOverrides = {};

  for (const [key, value] of Object.entries(input)) {
    if (isBooleanOptionKey(key)) {
      if (typeof value !== 'boolean') {
        throw new Error(`${context}.${key} must be a boolean`);
      }
      overrides[key] = value;
      continue;
    }

    // The keys whose value is not a boolean are spelled out rather than
    // tabulated, because a table would have to hold each one's accepted values
    // against a key whose value type differs from every other key's.
    if (key === 'styleResolution') {
      overrides.styleResolution = requireOneOf(value, STYLE_RESOLUTIONS, `${context}.${key}`);
      continue;
    }
    if (key === 'sourceMap') {
      overrides.sourceMap = requireSourceMapSetting(value, `${context}.${key}`);
      continue;
    }
    // Named for the file it reads, not for the option it fills, because that is
    // what the manifest actually holds -- and a reader who greps for
    // `inputSourceMap` should land on this branch rather than wonder where a
    // whole source map came from.
    if (key === INPUT_SOURCE_MAP_KEY) {
      if (typeof value !== 'string' || value.length === 0) {
        throw new Error(`${context}.${key} must be a relative path to a source map`);
      }
      overrides.inputSourceMap = readInputSourceMap(value, workspaceRoot, `${context}.${key}`);
      continue;
    }
    if (key === 'classNamePrefix') {
      if (typeof value !== 'string' || value.length === 0) {
        throw new Error(`${context}.${key} must be a non-empty string`);
      }
      overrides.classNamePrefix = value;
      continue;
    }

    throw new Error(
      `${context}.${key} is not a benchmarkable option — the accepted keys are ` +
        `${BOOLEAN_OPTION_KEYS.join(', ')}, styleResolution, sourceMap, ` +
        `classNamePrefix, ${INPUT_SOURCE_MAP_KEY}`
    );
  }

  return overrides;
}

function isBooleanOptionKey(key: string): key is BooleanOptionKey {
  // Widened to compare, not asserted: the predicate is what narrows, and the
  // caller only ever indexes with a key this returned true for.
  const keys: readonly string[] = BOOLEAN_OPTION_KEYS;
  return keys.includes(key);
}

/**
 * `value` narrowed onto the type `sourceMap` takes, or an error.
 *
 * A predicate rather than an assertion, for the reason `SOURCE_MAP_SETTINGS`
 * gives: the type is an ambient `const enum` this module cannot name a member
 * of, and the strings below are what the napi boundary actually receives.
 */
function requireSourceMapSetting(
  value: unknown,
  context: string
): NonNullable<FixtureOptionOverrides['sourceMap']> {
  if (!isSourceMapSetting(value)) {
    throw new Error(`${context} must be one of ${SOURCE_MAP_SETTINGS.join(', ')}`);
  }

  return value;
}

/**
 * The contents of a committed source map, read as the string the transform
 * expects. Relative to the workspace root, like every other path a fixture
 * entry names, and refused if it is absolute or climbs out.
 */
function readInputSourceMap(file: string, workspaceRoot: string, context: string): string {
  if (path.isAbsolute(file) || file.split(/[\\/]/).includes('..')) {
    throw new Error(`${context} must be a relative path inside the workspace`);
  }

  return fs.readFileSync(path.join(workspaceRoot, file), 'utf8');
}

function isSourceMapSetting(
  value: unknown
): value is NonNullable<FixtureOptionOverrides['sourceMap']> {
  const settings: readonly string[] = SOURCE_MAP_SETTINGS;
  return typeof value === 'string' && settings.includes(value);
}

/** `value` when it is one of `accepted`, else an error naming what was allowed. */
function requireOneOf<T extends string>(
  value: unknown,
  accepted: readonly T[],
  context: string
): T {
  const found = accepted.find(candidate => candidate === value);
  if (found === undefined) {
    throw new Error(`${context} must be one of ${accepted.join(', ')}`);
  }

  return found;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}
