import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import { hermeticEnvironment, repoRoot } from './lib/test-harness.mjs';

const lefthook = path.join(repoRoot, 'node_modules/.bin/lefthook');
const golden = path.join(repoRoot, 'scripts/git/__snapshots__/lefthook-dump.yml');

/**
 * `lefthook validate` is pure JSON-schema: it accepts a config whose *meaning*
 * is wrong. The checks here cover what it structurally cannot.
 *
 * The golden file is the merged, resolved config -- what lefthook will actually
 * run. Its job is not to know which configs are good but to make every change
 * to them visible and deliberate, in a file whose diff a reviewer reads.
 *
 * The `parallel`/`piped` check is the one case worth asserting outright,
 * because its failure is silent: at hook level lefthook errors on the pair, but
 * on a `group:` it accepts both and discards `piped`. This config leans on
 * `group: {piped: true}` for ordering that is load-bearing -- lint before
 * format, oxfmt before the Markdown alert check -- so losing it silently
 * reintroduces races that were already fixed once.
 */
const NEEDS_LEFTHOOK = fs.existsSync(lefthook) ? false : 'requires node_modules/.bin/lefthook';

/**
 * `lefthook dump` always merges an untracked `lefthook-local.yml`, and there is
 * no way to ask it not to: the file is keyed off the config's own name, so
 * neither `LEFTHOOK_CONFIG` nor a `dump` flag excludes it.
 *
 * So a developer using the sanctioned override would fail the golden check for
 * their own config -- and the obvious way out, `pnpm hooks:dump`, commits their
 * personal overrides into the shared snapshot. Skipping is the honest outcome:
 * the golden is a claim about the tracked config, which is not what `dump`
 * prints here. CI never has the file, so it still enforces the snapshot in the
 * one place that gates the merge.
 *
 * Only the golden comparison is skipped. Everything else below asserts a
 * property of whatever config lefthook resolved, which is exactly what a local
 * override should still be held to.
 */
const HAS_LOCAL_OVERRIDE = fs.existsSync(path.join(repoRoot, 'lefthook-local.yml'))
  ? 'an untracked lefthook-local.yml is merged into `lefthook dump`, so it cannot match the golden; CI has no such file and enforces it there'
  : false;

/**
 * Memoised: three subtests need the resolved config and `dump` is a process
 * spawn. The config cannot change mid-run, so one call answers all of them.
 */
let dumped;

function dump() {
  if (dumped === undefined) {
    // `hermeticEnvironment` for the reason every other spawn in these suites
    // uses it: `pre-push` runs this suite, and lefthook resolves the repository
    // by shelling out to `git rev-parse`. An inherited `GIT_DIR` aims that at
    // whatever git was operating on -- and if it no longer resolves, as after a
    // moved worktree, `dump` exits 128 and the suite fails for a reason that
    // has nothing to do with the config it is checking.
    const options = { cwd: repoRoot, encoding: 'utf8', env: hermeticEnvironment() };
    const result = spawnSync(lefthook, ['dump'], options);
    assert.equal(result.status, 0, `lefthook dump failed: ${result.stderr}`);
    dumped = result.stdout;
  }

  return dumped;
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
  await t.test('the resolved config matches its golden file', { skip: HAS_LOCAL_OVERRIDE }, () => {
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
