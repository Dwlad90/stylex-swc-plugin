import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, test } from 'vitest';

import { createStylexOptions } from '../lib/config.js';
import { loadAllFixtures } from '../lib/fixtures.js';
import { loadSubject } from '../lib/subjects.js';

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(benchmarkDir, '../..');
const workspaceRoot = path.resolve(packageDir, '../..');

describe('loadAllFixtures', () => {
  const fixtures = loadAllFixtures({ packageDir, workspaceRoot });

  test('loads the complete versioned registry', () => {
    expect(fixtures).toHaveLength(22);
    expect(new Set(fixtures.map(fixture => fixture.name)).size).toBe(22);
  });

  // The runner refuses to time a subject that produces no rules, so a
  // registered zero-rule fixture fails every benchmark job in CI rather
  // than here. Keep this assertion against the real binding.
  test('every registered fixture produces at least one StyleX rule', async () => {
    const subject = await loadSubject({ label: 'current', packageDir });
    const stylexOptions = createStylexOptions(packageDir);

    for (const fixture of fixtures) {
      expect(subject.run(fixture, stylexOptions), fixture.name).toBeGreaterThan(0);
    }
  });

  test('produces at least one fixture of each expected group', () => {
    expect(fixtures.some(fixture => fixture.name.startsWith('Performance -'))).toBe(true);
    expect(fixtures.some(fixture => fixture.name.startsWith('Rollup plugin -'))).toBe(true);
    expect(fixtures.some(fixture => !fixture.name.includes(' - '))).toBe(true);
  });

  test('assigns heavy weight only to rollup fixtures', () => {
    for (const fixture of fixtures) {
      if (fixture.weight === 'heavy') {
        expect(fixture.name.startsWith('Rollup plugin -')).toBe(true);
      }
    }
  });

  test('every fixture has non-empty code and batchSize >= 1', () => {
    for (const fixture of fixtures) {
      expect(fixture.code.length).toBeGreaterThan(0);
      expect(fixture.batchSize).toBeGreaterThanOrEqual(1);
    }
  });

  test('filter narrows the returned set', () => {
    const filtered = loadAllFixtures({
      packageDir,
      workspaceRoot,
      filter: ['Rollup plugin'],
    });
    expect(filtered.length).toBeGreaterThan(0);
    expect(filtered.every(fixture => fixture.name.startsWith('Rollup plugin'))).toBe(true);
  });
});
