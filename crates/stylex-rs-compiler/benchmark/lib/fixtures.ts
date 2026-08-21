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
 * A fixture may opt into a `dev` build with `"dev": true`. That is a
 * different measurement rather than a louder one -- see
 * `FixtureDescriptor.dev` -- so it is per fixture, never a switch in the
 * shared options.
 *
 * Only fixtures that actually produce StyleX rules belong here. The
 * transform test corpus also contains negative fixtures that compile to
 * zero rules (`button-props`, which never imports `stylex`); registering
 * one trips the runner's sanity check and fails every benchmark job.
 * `fixtures.test.ts` enforces this against the real binding.
 */

import fs from 'node:fs';
import path from 'node:path';

import type { FixtureCategory, FixtureDescriptor, FixtureWeight } from './types.js';

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
      // particular" must not read the same to a consumer.
      if (fixture.dev !== undefined) descriptor.dev = fixture.dev;
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
  return {
    name: input.name,
    file: input.file,
    category: input.category,
    weight: input.weight,
    batchSize: Number(input.batchSize),
    ...(input.dev === undefined ? {} : { dev: input.dev }),
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}
