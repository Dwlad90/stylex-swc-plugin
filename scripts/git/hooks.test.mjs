import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import test from 'node:test';

import {
  createWorkspace,
  missing,
  pathVariable,
  readInvocations,
  readLog,
  repoRoot,
  stubPath,
  writeStubs,
} from './lib/test-harness.mjs';

/**
 * These tests drive the real git hook scripts, stubbing only the commands they
 * shell out to. They cover the branches that decide *whether* something runs --
 * the commitizen guard matrix and the `core.hooksPath` unset -- because those
 * are pure logic over positional arguments and environment, and a wrong branch
 * fails silently: no prompt when you wanted one, or hooks that were never
 * installed at all.
 *
 * The interactive prompt itself needs a real TTY and is not testable here.
 */
const NEEDS_SHELL = missing('sh');

/**
 * Every script under test addresses its npm binaries as
 * `./node_modules/.bin/<tool>`, relative to the directory it is run from, so
 * that is where their stubs go -- not on `PATH`.
 */
const NODE_MODULES_BIN = 'node_modules/.bin';

void test('prepare-commit-msg guard', { skip: NEEDS_SHELL }, async t => {
  const script = path.join(repoRoot, '.lefthook/prepare-commit-msg/commitizen.sh');

  function runGuard(commitSource, sha) {
    const workspace = createWorkspace('stylex-hooks-cz-');
    writeStubs(path.join(workspace.directory, NODE_MODULES_BIN), { cz: {} });

    const result = spawnSync('sh', [script, '/tmp/COMMIT_EDITMSG', commitSource, sha], {
      cwd: workspace.directory,
      env: { ...process.env, FAKE_COMMAND_LOG: workspace.log },
      encoding: 'utf8',
    });

    return { ran: readLog(workspace.log).includes('cz --hook'), status: result.status };
  }

  /**
   * `$2` is git's commit source and `$3` is set only when amending. The table
   * is the contract: anything not listed as a source -- including no source at
   * all, the plain `git commit` case -- gets the prompt.
   */
  const cases = [
    { name: 'plain git commit', source: '', sha: '', expected: true },
    { name: 'template source', source: 'template', sha: '', expected: true },
    { name: 'commit source, no sha', source: 'commit', sha: '', expected: true },
    { name: '-m or -F', source: 'message', sha: '', expected: false },
    { name: 'merge commit', source: 'merge', sha: '', expected: false },
    { name: 'squash', source: 'squash', sha: '', expected: false },
    { name: 'amend', source: '', sha: 'abc1234', expected: false },
    { name: 'amend with source', source: 'commit', sha: 'abc1234', expected: false },
  ];

  for (const testCase of cases) {
    await t.test(testCase.name, () => {
      const { ran, status } = runGuard(testCase.source, testCase.sha);
      assert.equal(ran, testCase.expected);
      assert.equal(status, 0, 'a skipped guard must not fail the hook');
    });
  }
});

void test('pre-commit manifests', { skip: NEEDS_SHELL }, async t => {
  const script = path.join(repoRoot, '.lefthook/pre-commit/manifests.sh');

  /**
   * `perArgument` recording, because what this suite is really asserting is
   * argument *shape*: `--source` interleaved before each path, and a path
   * containing a space arriving as one argument rather than two.
   */
  function runManifests(manifests, { environment = {} } = {}) {
    const workspace = createWorkspace('stylex-hooks-manifests-');
    writeStubs(path.join(workspace.directory, NODE_MODULES_BIN), {
      // `/bin/sh`, not `/usr/bin/env sh`: the empty-PATH case below cannot find
      // `env` either.
      syncpack: { shebang: '#!/bin/sh', perArgument: true },
      oxfmt: { shebang: '#!/bin/sh', perArgument: true },
    });

    const result = spawnSync('/bin/sh', [script, ...manifests], {
      cwd: workspace.directory,
      env: {
        ...process.env,
        FAKE_COMMAND_LOG: workspace.log,
        [pathVariable]: stubPath(workspace.bin),
        ...environment,
      },
      encoding: 'utf8',
    });

    return { invocations: readInvocations(workspace.log), status: result.status };
  }

  await t.test('interleaves --source before every manifest', () => {
    const { invocations, status } = runManifests(['package.json', 'packages/a/package.json']);
    assert.equal(status, 0);

    const [syncpack, oxfmt] = invocations;
    assert.deepEqual(syncpack, [
      'syncpack',
      'format',
      '--config',
      '.syncpackrc',
      '--source',
      'package.json',
      '--source',
      'packages/a/package.json',
    ]);

    // The originals, not the rewritten list -- the subshell must not leak.
    assert.deepEqual(oxfmt, [
      'oxfmt',
      '--no-error-on-unmatched-pattern',
      'package.json',
      'packages/a/package.json',
    ]);
  });

  // The binaries are addressed by path precisely so the hook works in a GUI git
  // client or an IDE commit dialog, where the ambient environment is whatever
  // the desktop session happened to export. An empty PATH is the strongest
  // available statement of that.
  await t.test('resolves its binaries with an empty PATH', () => {
    const { invocations, status } = runManifests(['package.json'], {
      environment: { [pathVariable]: '' },
    });

    assert.equal(status, 0);
    assert.deepEqual(
      invocations.map(([tool]) => tool),
      ['syncpack', 'oxfmt']
    );
  });

  await t.test('keeps a path containing a space as one argument', () => {
    const { invocations, status } = runManifests(['some dir/package.json']);
    assert.equal(status, 0);

    for (const invocation of invocations) {
      assert.ok(
        invocation.includes('some dir/package.json'),
        'the path must survive as a single argument'
      );
    }
  });

  await t.test('does nothing when given no manifests', () => {
    const { invocations, status } = runManifests([]);
    assert.equal(status, 0);
    assert.deepEqual(invocations, []);
  });
});

void test('install-hooks.sh', { skip: NEEDS_SHELL }, async t => {
  const script = path.join(repoRoot, 'scripts/git/install-hooks.sh');

  /**
   * Runs in a throwaway directory rather than the repository, because the
   * script ends in a real `./node_modules/.bin/lefthook install`. Pointed at
   * the checkout it would rewrite this clone's hooks as a side effect of
   * running the suite.
   */
  function runInstall({ hooksPath, env = {} }) {
    const workspace = createWorkspace('stylex-hooks-install-');

    writeStubs(workspace.bin, {
      git: {
        body: `case "$*" in
  "config --get core.hooksPath") [ -n "${hooksPath}" ] && printf '%s\\n' "${hooksPath}" || exit 1 ;;
esac`,
      },
    });
    writeStubs(path.join(workspace.directory, NODE_MODULES_BIN), { lefthook: {} });

    const result = spawnSync('sh', [script], {
      cwd: workspace.directory,
      env: {
        ...process.env,
        CI: '',
        LEFTHOOK: '',
        ...env,
        FAKE_COMMAND_LOG: workspace.log,
        [pathVariable]: stubPath(workspace.bin),
      },
      encoding: 'utf8',
    });

    return { log: readLog(workspace.log), status: result.status };
  }

  await t.test("unsets husky's core.hooksPath before installing", () => {
    const { log, status } = runInstall({ hooksPath: '.husky/_' });
    assert.equal(status, 0);
    assert.match(log, /git config --unset core\.hooksPath/);
    assert.match(log, /lefthook install/);
  });

  await t.test('leaves a custom core.hooksPath alone', () => {
    const { log, status } = runInstall({ hooksPath: '.config/githooks' });
    assert.equal(status, 0);
    assert.doesNotMatch(log, /--unset/, 'a deliberate custom hooks path must survive');
    assert.match(log, /lefthook install/);
  });

  await t.test('installs when no core.hooksPath is set', () => {
    const { log, status } = runInstall({ hooksPath: '' });
    assert.equal(status, 0);
    assert.doesNotMatch(log, /--unset/);
    assert.match(log, /lefthook install/);
  });

  // `--force` would write lefthook's hooks into husky's `.husky/_`, resurrecting
  // the directory this migration deletes -- on exactly the clones the unset
  // above exists to rescue.
  await t.test('never installs with --force', () => {
    const { log } = runInstall({ hooksPath: '.husky/_' });
    assert.doesNotMatch(log, /--force|\s-f\b/);
  });

  /**
   * CI checks out fresh, never commits, and runs the equivalent checks as named
   * jobs. Installing there would mutate the runner's git config for nothing.
   */
  for (const { label, env } of [
    { label: 'CI=true', env: { CI: 'true' } },
    // Providers agree on setting CI, not on its value.
    { label: 'CI=1', env: { CI: '1' } },
    { label: 'LEFTHOOK=0', env: { LEFTHOOK: '0' } },
  ]) {
    await t.test(`skips entirely under ${label}`, () => {
      const { log, status } = runInstall({ hooksPath: '.husky/_', env });
      assert.equal(status, 0);
      assert.equal(log, '', 'nothing should be invoked at all');
    });
  }
});
