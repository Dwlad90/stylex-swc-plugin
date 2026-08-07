import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

function createHarness(releases = '') {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-release-scripts-'));
  const bin = path.join(directory, 'bin');
  const log = path.join(directory, 'commands.log');
  const pathVariable = ['PA', 'TH'].join('');
  fs.mkdirSync(bin);

  writeExecutable(
    path.join(bin, 'git'),
    `#!/usr/bin/env bash
set -u
printf 'git %s\n' "$*" >> "$FAKE_COMMAND_LOG"
case "$1 $2" in
  "remote get-url") printf '%s\n' 'git@github.com:Dwlad90/stylex-swc-plugin.git' ;;
  "symbolic-ref --quiet") printf '%s\n' 'feature' ;;
  "tag --list") printf '%s\n' '0.18.3' ;;
  "merge-base --is-ancestor"|"fetch --quiet") exit 0 ;;
  "ls-remote --exit-code") exit "\${FAKE_REMOTE_TAG_STATUS:-2}" ;;
  "rev-parse --verify") printf '%s\n' 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' ;;
  "rev-parse -q") exit "\${FAKE_LOCAL_TAG_STATUS:-1}" ;;
  "push origin"|"tag --delete") exit 0 ;;
  *) exit 0 ;;
esac
`
  );
  writeExecutable(
    path.join(bin, 'gh'),
    `#!/usr/bin/env bash
set -u
printf 'gh %s\n' "$*" >> "$FAKE_COMMAND_LOG"
if [ "\${1:-}" = release ] && [ "\${2:-}" = view ]; then exit 1; fi
if [ "\${1:-}" = api ] && [ "\${2:-}" = --paginate ]; then
  printf '%s\n' "$FAKE_RELEASES"
fi
`
  );

  return {
    directory,
    log,
    env: {
      ...process.env,
      PATH: `${bin}:${process.env[pathVariable] ?? ''}`,
      FAKE_COMMAND_LOG: log,
      FAKE_RELEASES: releases,
    },
  };
}

function writeExecutable(file, contents) {
  fs.writeFileSync(file, contents);
  fs.chmodSync(file, 0o755);
}

function run(script, args, env) {
  return spawnSync('bash', [path.join(repoRoot, script), ...args], {
    cwd: repoRoot,
    env,
    encoding: 'utf8',
  });
}

void test('start-release preview validates and prints the plan without dispatching', () => {
  const harness = createHarness();
  const result = run(
    'scripts/git/start-release.sh',
    ['--preview', '--no-fetch', '--ref', 'feature'],
    harness.env
  );

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /Next version:\s+0\.18\.4/);
  assert.match(result.stdout, /Preview only, nothing dispatched/);
  assert.doesNotMatch(fs.readFileSync(harness.log, 'utf8'), /gh workflow run/);
});

void test('start-release dispatches the validated workflow inputs non-interactively', () => {
  const harness = createHarness();
  const result = run(
    'scripts/git/start-release.sh',
    ['--yes', '--no-fetch', '--ref', 'feature', '--pre', 'rc', '--npm-dry-run'],
    harness.env
  );

  assert.equal(result.status, 0, result.stderr);
  const commands = fs.readFileSync(harness.log, 'utf8');
  assert.match(commands, /gh workflow run release\.yml/);
  assert.match(commands, /--raw-field prerelease=true/);
  assert.match(commands, /--raw-field dry-run=true/);
});

void test('delete-draft-release refuses a published release', () => {
  const harness = createHarness(
    '{"id":1,"tagName":"0.18.4","isDraft":false,"createdAt":"2026-01-01"}'
  );
  const result = run('scripts/git/delete-draft-release.sh', ['0.18.4', '--yes'], harness.env);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /it is a published release/);
  assert.doesNotMatch(fs.readFileSync(harness.log, 'utf8'), /-X DELETE/);
});

void test('delete-draft-release dry-run reports every deletion without executing it', () => {
  const harness = createHarness(
    '{"id":2,"tagName":"0.18.4-dev.1","isDraft":true,"createdAt":"2026-01-01"}'
  );
  const env = { ...harness.env, FAKE_REMOTE_TAG_STATUS: '0', FAKE_LOCAL_TAG_STATUS: '0' };
  const result = run('scripts/git/delete-draft-release.sh', ['0.18.4-dev.1', '--dry-run'], env);

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /\[dry-run\] gh api -X DELETE/);
  assert.match(result.stdout, /\[dry-run\] git push origin --delete/);
  assert.match(result.stdout, /\[dry-run\] git tag --delete/);
  assert.doesNotMatch(fs.readFileSync(harness.log, 'utf8'), /gh api -X DELETE/);
});

void test('delete-draft-release treats a remote query failure as fatal', () => {
  const harness = createHarness(
    '{"id":2,"tagName":"0.18.4-dev.1","isDraft":true,"createdAt":"2026-01-01"}'
  );
  const result = run('scripts/git/delete-draft-release.sh', ['0.18.4-dev.1', '--yes'], {
    ...harness.env,
    FAKE_REMOTE_TAG_STATUS: '128',
  });

  assert.equal(result.status, 1);
  assert.match(result.stderr, /Could not query 'origin'/);
});
