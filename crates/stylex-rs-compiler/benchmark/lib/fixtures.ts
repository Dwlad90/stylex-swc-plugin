/**
 * Single registry of every benchmarkable fixture.
 *
 * The versioned JSON manifest is shared by measurement and the trusted PR
 * reporter, so fixture names, weight classes, categories, and calibrated
 * batch sizes cannot drift across the trust boundary.
 *
 * Batch sizes are all `1` until Phase 0 calibration lands. Fast fixtures
 * will move above sub-millisecond noise once calibrated; do not add an
 * absolute-delta floor as a shortcut.
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
      return {
        name: fixture.name,
        filePath,
        code: fs.readFileSync(filePath, 'utf-8'),
        weight: fixture.weight,
        category: fixture.category,
        batchSize: fixture.batchSize,
      };
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
  return {
    name: input.name,
    file: input.file,
    category: input.category,
    weight: input.weight,
    batchSize: Number(input.batchSize),
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}
