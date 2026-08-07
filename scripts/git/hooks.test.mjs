import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

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
function missing(...commands) {
  const absent = commands.filter(
    command => spawnSync('sh', ['-c', 'command -v "$1"', 'sh', command]).status !== 0
  );
  return absent.length > 0 ? `requires ${absent.join(', ')} on PATH` : false;
}

const NEEDS_SHELL = missing('sh');
const pathVariable = ['PA', 'TH'].join('');

function writeExecutable(file, contents) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, contents);
  fs.chmodSync(file, 0o755);
}

/**
 * A throwaway directory with a logging stub for every command the script under
 * test invokes. Returns the log path so assertions read the recorded argv.
 */
function createHarness(prefix, stubs) {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), prefix));
  const bin = path.join(directory, 'bin');
  const log = path.join(directory, 'commands.log');

  for (const [name, body] of Object.entries(stubs)) {
    writeExecutable(
      path.join(bin, name),
      `#!/usr/bin/env sh\nprintf '${name} %s\\n' "$*" >> "$FAKE_COMMAND_LOG"\n${body ?? ''}\n`
    );
  }

  return { bin, directory, log };
}

function readLog(log) {
  return fs.existsSync(log) ? fs.readFileSync(log, 'utf8') : '';
}

test('prepare-commit-msg guard', { skip: NEEDS_SHELL }, async t => {
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

test('pre-commit manifests', { skip: NEEDS_SHELL }, async t => {
  const script = path.join(repoRoot, '.lefthook/pre-commit/manifests.sh');

  /**
   * `pnpm` is stubbed to record one argument per line, so an argument
   * containing a space is distinguishable from two arguments.
   */
  function runManifests(manifests) {
    const harness = createHarness('stylex-hooks-manifests-', {});
    writeExecutable(
      path.join(harness.bin, 'pnpm'),
      `#!/usr/bin/env sh\nprintf -- '---\\n' >> "$FAKE_COMMAND_LOG"\nfor argument in "$@"; do printf '%s\\n' "$argument" >> "$FAKE_COMMAND_LOG"; done\n`
    );

    const result = spawnSync('sh', [script, ...manifests], {
      cwd: harness.directory,
      env: {
        ...process.env,
        FAKE_COMMAND_LOG: harness.log,
        [pathVariable]: `${harness.bin}:${process.env[pathVariable]}`,
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
      'exec',
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
      'exec',
      'oxfmt',
      '--no-error-on-unmatched-pattern',
      'package.json',
      'packages/a/package.json',
    ]);
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

test('install-hooks.sh', { skip: NEEDS_SHELL }, async t => {
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
