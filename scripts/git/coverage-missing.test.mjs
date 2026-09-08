/**
 * The uncovered-region reporter, `scripts/coverage-missing.sh`.
 *
 * The script measures with whatever nightly the machine holds, while continuous
 * integration installs the newest nightly on every run. The region count is a
 * property of that compiler, so an old local nightly makes the script report
 * full coverage against a gate that fails in continuous integration -- which is
 * exactly how fourteen uncovered regions reached a pull request. The script
 * therefore names the compiler it used and warns when a newer one exists, and
 * this suite holds that behaviour still.
 *
 * The real script runs against stubbed `rustc`, `rustup` and `cargo`, so what is
 * asserted is what the script does, not what the machine happens to have.
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import test from 'node:test';

import {
  createWorkspace,
  hermeticEnvironment,
  missing,
  pathVariable,
  readLog,
  repoRoot,
  stubPath,
  writeStubs,
} from './lib/test-harness.mjs';

const script = path.join(repoRoot, 'scripts/coverage-missing.sh');

/** The script is bash, and the report it renders is python. */
const NEEDS_BASH = missing('bash', 'python3');

/** A `rustup check` line for a nightly that has nothing newer behind it. */
const UP_TO_DATE =
  'nightly-aarch64-apple-darwin - up to date: 1.100.0-nightly (cea272fa3 2026-09-07)';

/** The same line for a nightly that is behind the one continuous integration installs. */
const BEHIND =
  'nightly-aarch64-apple-darwin - update available: 1.100.0-nightly (f248f4038 2026-09-05) -> 1.100.0-nightly (cea272fa3 2026-09-07)';

/** A coverage export holding no measured file, which is the shortest clean run. */
const EMPTY_EXPORT = '{"data":[{"files":[],"functions":[]}]}';

/**
 * Runs the real script with stubs for the three commands it starts.
 *
 * `rustcVersion` empty stands for a machine with no nightly installed, so the
 * stub fails the way `rustc +nightly` fails there. `rustupCheck` is the line the
 * script reads to decide whether the nightly is behind.
 */
function runScript({
  rustcVersion = 'rustc 1.100.0-nightly (cea272fa3 2026-09-07)',
  rustupCheck = UP_TO_DATE,
  args = [],
} = {}) {
  const workspace = createWorkspace('stylex-coverage-missing-');

  // The stub writes the export where the script asked for it, so the report
  // renders from a real file rather than from a missing one.
  const cargoBody = [
    'output=""',
    'previous=""',
    'for argument in "$@"; do',
    '  if [ "$previous" = "--output-path" ]; then output="$argument"; fi',
    '  previous="$argument"',
    'done',
    `[ -n "$output" ] && printf '%s' '${EMPTY_EXPORT}' > "$output"`,
    'exit 0',
  ].join('\n');

  writeStubs(workspace.bin, {
    cargo: { body: cargoBody },
    rustc: rustcVersion === '' ? { body: 'exit 1' } : { body: `printf '%s\\n' '${rustcVersion}'` },
    rustup: { body: `printf '%s\\n' '${rustupCheck}'` },
  });

  const result = spawnSync('bash', [script, ...args], {
    cwd: repoRoot,
    encoding: 'utf8',
    env: hermeticEnvironment({
      [pathVariable]: stubPath(workspace.bin),
      FAKE_COMMAND_LOG: workspace.log,
    }),
  });

  return { result, log: readLog(workspace.log) };
}

void test('the run names the compiler that measured it', { skip: NEEDS_BASH }, () => {
  const { result } = runScript();

  assert.match(result.stdout, /[=]=> Toolchain: rustc 1\.100\.0-nightly \(cea272fa3 2026-09-07\)/);
  assert.equal(result.status, 0);
});

void test('a nightly behind the newest one is a warning', { skip: NEEDS_BASH }, () => {
  const { result } = runScript({ rustupCheck: BEHIND });

  assert.match(result.stderr, /a newer nightly is available/);
  assert.match(result.stderr, /rustup update nightly/);
  // A warning only: the report still runs and still answers for the run it made.
  assert.equal(result.status, 0);
});

void test('a nightly that is up to date says nothing', { skip: NEEDS_BASH }, () => {
  const { result } = runScript({ rustupCheck: UP_TO_DATE });

  assert.doesNotMatch(result.stderr, /newer nightly/);
  assert.equal(result.status, 0);
});

void test('--skip-toolchain-check asks the network nothing', { skip: NEEDS_BASH }, () => {
  const { result, log } = runScript({ rustupCheck: BEHIND, args: ['--skip-toolchain-check'] });

  assert.doesNotMatch(log, /^rustup check/m, 'the lookup ran although it was turned off');
  assert.doesNotMatch(result.stderr, /newer nightly/);
  assert.match(result.stdout, /[=]=> Toolchain: /, 'the version is named even so');
  assert.equal(result.status, 0);
});

void test(
  'a machine with no nightly is told which command installs one',
  { skip: NEEDS_BASH },
  () => {
    const { result } = runScript({ rustcVersion: '' });

    assert.match(result.stderr, /no nightly toolchain found/);
    assert.match(result.stderr, /rustup toolchain install nightly/);
  }
);
