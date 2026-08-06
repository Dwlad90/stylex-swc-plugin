/**
 * Single registry of every benchmarkable fixture.
 *
 * `bench.ts` previously walked the transform-fixtures directory at import
 * time, `bench-compare.ts` maintained its own hard-coded list, and
 * `perf_fixtures` / rollup fixtures were duplicated across both. Both now
 * read from this module so fixture names, weight classes, categories, and
 * (later) calibrated batch sizes stay consistent across entry points.
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

interface ListedFixture {
  file: string;
  displayName: string;
}

const PERF_FIXTURES: readonly ListedFixture[] = [
  { file: 'colors.stylex.js', displayName: 'Performance - Colors StyleX transformation' },
  { file: 'createTheme-basic.js', displayName: 'Performance - Basic theme transformation' },
  { file: 'createTheme-complex.js', displayName: 'Performance - Complex theme transformation' },
  { file: 'create-basic.js', displayName: 'Performance - Basic create transformation' },
  { file: 'create-complex.js', displayName: 'Performance - Complex create transformation' },
];

const ROLLUP_FIXTURES: readonly ListedFixture[] = [
  { file: 'lotsOfStyles.js', displayName: 'Rollup plugin - lotsOfStyles.js' },
  { file: 'lotsOfStylesDynamic.js', displayName: 'Rollup plugin - lotsOfStylesDynamic.js' },
];

export function loadAllFixtures(options: LoadFixturesOptions): FixtureDescriptor[] {
  const requested = new Set<FixtureCategory>(
    options.categories && options.categories.length > 0
      ? options.categories
      : (['transform', 'perf', 'rollup'] as const)
  );

  const all: FixtureDescriptor[] = [];
  if (requested.has('transform')) all.push(...loadTransformFixtures(options.workspaceRoot));
  if (requested.has('perf')) {
    all.push(
      ...loadListedFixtures({
        baseDir: path.join(options.packageDir, 'benchmark', 'perf_fixtures'),
        entries: PERF_FIXTURES,
        weight: 'standard',
        category: 'perf',
      })
    );
  }
  if (requested.has('rollup')) {
    all.push(
      ...loadListedFixtures({
        baseDir: path.join(options.workspaceRoot, 'apps/rollup-large-example'),
        entries: ROLLUP_FIXTURES,
        weight: 'heavy',
        category: 'rollup',
      })
    );
  }

  if (!options.filter || options.filter.length === 0) return all;
  return all.filter(fixture => options.filter!.some(needle => fixture.name.includes(needle)));
}

/**
 * Transform fixtures come from `crates/stylex-transform/tests/fixture`
 * and are discovered by walking for `input.stylex.js`. Stable name is the
 * containing directory — renaming a directory therefore breaks trend
 * history. Accepted risk, flagged here for fixture authors.
 */
function loadTransformFixtures(workspaceRoot: string): FixtureDescriptor[] {
  const root = path.join(workspaceRoot, 'crates/stylex-transform/tests/fixture');
  const filePaths: string[] = [];
  walkForInputs(root, filePaths);
  filePaths.sort();

  return filePaths.map(filePath => ({
    name: path.basename(path.dirname(filePath)),
    filePath,
    code: fs.readFileSync(filePath, 'utf-8'),
    weight: 'standard' as const,
    category: 'transform' as const,
    batchSize: 1,
  }));
}

function loadListedFixtures(args: {
  baseDir: string;
  entries: readonly ListedFixture[];
  weight: FixtureWeight;
  category: FixtureCategory;
}): FixtureDescriptor[] {
  return args.entries.map(entry => {
    const filePath = path.join(args.baseDir, entry.file);
    return {
      name: entry.displayName,
      filePath,
      code: fs.readFileSync(filePath, 'utf-8'),
      weight: args.weight,
      category: args.category,
      batchSize: 1,
    };
  });
}

function walkForInputs(dir: string, out: string[]): void {
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      walkForInputs(full, out);
    } else if (entry.name === 'input.stylex.js') {
      out.push(full);
    }
  }
}
