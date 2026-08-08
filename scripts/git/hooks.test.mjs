import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  makeTemporaryDirectory,
  missing,
  pathVariable,
  readLog,
  writeExecutable,
  writeRecordingStub,
} from './lib/test-harness.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

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
 * A throwaway directory with a logging stub for every command the script under
 * test invokes. Returns the log path so assertions read the recorded argv.
 */
function createHarness(prefix, stubs) {
  const directory = makeTemporaryDirectory(prefix);
  const bin = path.join(directory, 'bin');
  const log = path.join(directory, 'commands.log');

  for (const [name, body] of Object.entries(stubs)) {
    writeRecordingStub(path.join(bin, name), name, body ?? '');
  }

  return { bin, directory, log };
}

void test('prepare-commit-msg guard', { skip: NEEDS_SHELL }, async t => {
  const script = path.join(repoRoot, '.lefthook/prepare-commit-msg/commitizen.sh');

  /**
   * `cz` is resolved as `node_modules/.bin/cz` relative to the working
   * directory, so the stub goes there rather than on PATH.
   */
  function runGuard(commitSource, sha) {
    const harness = createHarness('stylex-hooks-cz-', {});
    const log = harness.log;
    writeExecutable(
      path.join(harness.directory, 'node_modules/.bin/cz'),
      `#!/usr/bin/env sh\nprintf 'cz %s\\n' "$*" >> "$FAKE_COMMAND_LOG"\n`
    );

    const result = spawnSync('sh', [script, '/tmp/COMMIT_EDITMSG', commitSource, sha], {
      cwd: harness.directory,
      env: { ...process.env, FAKE_COMMAND_LOG: log },
      encoding: 'utf8',
    });

    return { ran: readLog(log).includes('cz --hook'), status: result.status };
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
   * The stubs live at `./node_modules/.bin/`, not on `PATH`, because that is
   * where the script looks: lefthook adds nothing to `PATH`, so the hook has to
   * address its binaries by path. Each stub records its name and then one
   * argument per line, so an argument containing a space is distinguishable
   * from two arguments.
   */
  function runManifests(manifests, { environment = {} } = {}) {
    const harness = createHarness('stylex-hooks-manifests-', {});

    for (const tool of ['syncpack', 'oxfmt']) {
      writeExecutable(
        path.join(harness.directory, 'node_modules/.bin', tool),
        // `/bin/sh`, not `/usr/bin/env sh`: the empty-PATH case below cannot
        // find `env` either.
        `#!/bin/sh\nprintf -- '---\\n${tool}\\n' >> "$FAKE_COMMAND_LOG"\nfor argument in "$@"; do printf '%s\\n' "$argument" >> "$FAKE_COMMAND_LOG"; done\n`
      );
    }

    const result = spawnSync('/bin/sh', [script, ...manifests], {
      cwd: harness.directory,
      env: {
        ...process.env,
        FAKE_COMMAND_LOG: harness.log,
        [pathVariable]: harness.bin,
        ...environment,
      },
      encoding: 'utf8',
    });

    const invocations = readLog(harness.log)
      .split('---\n')
      .filter(Boolean)
      .map(block => block.split('\n').filter(Boolean));

    return { invocations, status: result.status };
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

  function runInstall({ hooksPath, env = {} }) {
    const harness = createHarness('stylex-hooks-install-', {
      git: `case "$*" in
  "config --get core.hooksPath") [ -n "${hooksPath}" ] && printf '%s\\n' "${hooksPath}" || exit 1 ;;
esac`,
      pnpm: '',
    });

    const result = spawnSync('sh', [script], {
      cwd: repoRoot,
      env: {
        ...process.env,
        CI: '',
        LEFTHOOK: '',
        ...env,
        FAKE_COMMAND_LOG: harness.log,
        [pathVariable]: `${harness.bin}:${process.env[pathVariable]}`,
      },
      encoding: 'utf8',
    });

    return { log: readLog(harness.log), status: result.status };
  }

  await t.test("unsets husky's core.hooksPath before installing", () => {
    const { log, status } = runInstall({ hooksPath: '.husky/_' });
    assert.equal(status, 0);
    assert.match(log, /git config --unset core\.hooksPath/);
    assert.match(log, /pnpm exec lefthook install/);
  });

  await t.test('leaves a custom core.hooksPath alone', () => {
    const { log, status } = runInstall({ hooksPath: '.config/githooks' });
    assert.equal(status, 0);
    assert.doesNotMatch(log, /--unset/, 'a deliberate custom hooks path must survive');
    assert.match(log, /pnpm exec lefthook install/);
  });

  await t.test('installs when no core.hooksPath is set', () => {
    const { log, status } = runInstall({ hooksPath: '' });
    assert.equal(status, 0);
    assert.doesNotMatch(log, /--unset/);
    assert.match(log, /pnpm exec lefthook install/);
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
