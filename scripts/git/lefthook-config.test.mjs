import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import { repoRoot } from './lib/test-harness.mjs';

const lefthook = path.join(repoRoot, 'node_modules/.bin/lefthook');
const golden = path.join(repoRoot, 'scripts/git/__snapshots__/lefthook-dump.yml');

/**
 * `lefthook validate` is pure JSON-schema: it accepts a config whose *meaning*
 * is wrong. The two checks here cover what it structurally cannot.
 *
 * The golden file is the merged, resolved config -- what lefthook will actually
 * run, including anything an untracked `lefthook-local.yml` contributes. Its
 * job is not to know which configs are good but to make every change to them
 * visible and deliberate, in a file whose diff a reviewer reads.
 *
 * The `parallel`/`piped` check is the one case worth asserting outright,
 * because its failure is silent: at hook level lefthook errors on the pair, but
 * on a `group:` it accepts both and discards `piped`. This config leans on
 * `group: {piped: true}` for ordering that is load-bearing -- lint before
 * format, oxfmt before the Markdown alert check -- so losing it silently
 * reintroduces races that were already fixed once.
 */
const NEEDS_LEFTHOOK = fs.existsSync(lefthook) ? false : 'requires node_modules/.bin/lefthook';

function dump() {
  const result = spawnSync(lefthook, ['dump'], { cwd: repoRoot, encoding: 'utf8' });
  assert.equal(result.status, 0, `lefthook dump failed: ${result.stderr}`);
  return result.stdout;
}

/**
 * The keys directly under each `group:` mapping. `lefthook dump` emits fixed
 * two-space indentation, so scoping by indent is enough and avoids taking on a
 * YAML parser for a hundred lines of tool output.
 */
function groupKeys(yaml) {
  const lines = yaml.split('\n');
  const groups = [];

  for (const [index, line] of lines.entries()) {
    const opening = line.match(/^(\s*)group:\s*$/);
    if (!opening) continue;

    const indent = opening[1].length;
    const keys = [];

    for (const following of lines.slice(index + 1)) {
      if (following.trim() === '') continue;

      const followingIndent = following.length - following.trimStart().length;
      if (followingIndent <= indent) break;

      const key = following.match(/^\s+([a-z_]+):/);
      if (followingIndent === indent + 2 && key) keys.push(key[1]);
    }

    groups.push(keys);
  }

  return groups;
}

void test('lefthook config', { skip: NEEDS_LEFTHOOK }, async t => {
  await t.test('the resolved config matches its golden file', () => {
    const expected = fs.readFileSync(golden, 'utf8');

    assert.equal(
      dump(),
      expected,
      'the resolved lefthook config changed. Review the diff, then run `pnpm hooks:dump` to accept it'
    );
  });

  await t.test('no group sets both parallel and piped', () => {
    const offenders = groupKeys(dump()).filter(
      keys => keys.includes('parallel') && keys.includes('piped')
    );

    assert.deepEqual(
      offenders,
      [],
      'lefthook accepts this pair on a group and silently discards `piped`, dropping the ordering the group exists to express'
    );
  });

  // A guard on the guard: if the indent-scoping above ever stopped finding
  // anything, the check would pass vacuously for the rest of time.
  await t.test('the group scan finds the groups that are there', () => {
    const groups = groupKeys(dump());

    assert.ok(groups.length > 0, 'expected at least one group: in the resolved config');
    assert.ok(
      groups.every(keys => keys.includes('jobs')),
      'every group has a jobs: key, so finding one without it means the scan is wrong'
    );
  });
});
