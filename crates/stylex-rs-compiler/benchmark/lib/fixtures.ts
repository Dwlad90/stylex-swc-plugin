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
  PROPERTY_VALIDATION_MODES,
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

  const all = loadManifest(options.packageDir)
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

function loadManifest(packageDir: string): readonly FixtureManifestEntry[] {
  const manifestPath = path.join(packageDir, 'benchmark', 'fixtures.v1.json');
  const input = JSON.parse(fs.readFileSync(manifestPath, 'utf8')) as unknown;
  if (!isRecord(input) || input.schemaVersion !== 1 || !Array.isArray(input.fixtures)) {
    throw new Error('Benchmark fixture manifest must use schema version 1');
  }

  const fixtures = input.fixtures.map((fixture, index) => parseManifestEntry(fixture, index));
  if (
    fixtures.length === 0 ||
    new Set(fixtures.map(fixture => fixture.name)).size !== fixtures.length
  ) {
    throw new Error('Benchmark fixture manifest names must be non-empty and unique');
  }
  return fixtures;
}

function parseManifestEntry(input: unknown, index: number): FixtureManifestEntry {
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
    entry.options = parseOptionOverrides(input.options, `${context}.options`);
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
function parseOptionOverrides(input: unknown, context: string): FixtureOptionOverrides {
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

    // The two enum-valued keys are spelled out, because each has its own set of
    // accepted values and a shared branch could not say which.
    if (key === 'styleResolution') {
      overrides.styleResolution = requireOneOf(value, STYLE_RESOLUTIONS, `${context}.${key}`);
      continue;
    }
    if (key === 'propertyValidationMode') {
      overrides.propertyValidationMode = requireOneOf(
        value,
        PROPERTY_VALIDATION_MODES,
        `${context}.${key}`
      );
      continue;
    }

    throw new Error(
      `${context}.${key} is not a benchmarkable option — add it to BOOLEAN_OPTION_KEYS if it should be`
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
