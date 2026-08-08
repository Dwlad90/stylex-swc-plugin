import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import { makeTemporaryDirectory, missing } from './lib/test-harness.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const script = path.join(repoRoot, 'scripts/git/no-merge-conflicts.sh');

const NEEDS_GIT = missing('git', 'bash');

/**
 * Markers are assembled rather than written literally so that no line of this
 * file starts with one. `git diff --check` -- the very thing under test -- would
 * otherwise flag this file when it is committed.
 */
const MARKERS = {
  opening: `${'<'.repeat(7)} HEAD`,
  base: `${'|'.repeat(7)} merged common ancestors`,
  separator: '='.repeat(7),
  closing: `${'>'.repeat(7)} feature`,
};

function git(cwd, ...args) {
  const result = spawnSync('git', args, { cwd, encoding: 'utf8' });
  assert.equal(result.status, 0, `git ${args.join(' ')} failed: ${result.stderr}`);
  return result.stdout.trim();
}

function write(cwd, file, contents) {
  fs.mkdirSync(path.dirname(path.join(cwd, file)), { recursive: true });
  fs.writeFileSync(path.join(cwd, file), contents);
}

/** A repository with one commit, so `HEAD` resolves and the index has a base. */
function createRepository(files = { 'file.txt': 'original\n' }) {
  const directory = createEmptyRepository();

  for (const [file, contents] of Object.entries(files)) {
    write(directory, file, contents);
  }

  git(directory, 'add', '.');
  git(directory, 'commit', '--quiet', '-m', 'initial');

  return directory;
}

function createEmptyRepository() {
  const directory = makeTemporaryDirectory('no-merge-conflicts-');

  git(directory, 'init', '--initial-branch=main', '--quiet');
  git(directory, 'config', 'user.email', 'test@example.com');
  git(directory, 'config', 'user.name', 'Test');
  // The hooks under test must not fire inside the fixture repositories.
  git(directory, 'config', 'core.hooksPath', path.join(directory, '.git/no-hooks'));

  return directory;
}

function run(cwd, { mode, stdin } = {}) {
  return spawnSync('bash', mode ? [script, mode] : [script], {
    cwd,
    encoding: 'utf8',
    input: stdin ?? '',
  });
}

void test('no-merge-conflicts.sh staged', { skip: NEEDS_GIT }, async t => {
  await t.test('passes when nothing is staged', () => {
    assert.equal(run(createRepository()).status, 0);
  });

  await t.test('passes for ordinary staged changes', () => {
    const repository = createRepository();
    write(repository, 'file.txt', 'changed\n');
    git(repository, 'add', 'file.txt');

    assert.equal(run(repository).status, 0);
  });

  for (const [name, marker] of Object.entries(MARKERS)) {
    await t.test(`rejects a staged ${name} marker`, () => {
      const repository = createRepository();
      write(repository, 'file.txt', `${marker}\n`);
      git(repository, 'add', 'file.txt');

      const result = run(repository);

      assert.equal(result.status, 1);
      assert.match(result.stdout, /There are unresolved merge conflicts/);
      assert.match(result.stdout, /file\.txt/);
    });
  }

  // The very first commit has no HEAD to diff against, which is the case a
  // naive `git diff --check HEAD` would crash on.
  await t.test('rejects a marker in an initial commit with no HEAD', () => {
    const repository = createEmptyRepository();
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'add', 'file.txt');

    assert.equal(run(repository).status, 1);
  });

  // A conflicted index is a separate failure from marker text: the file on disk
  // may be perfectly clean while the index still holds three stages.
  await t.test('rejects an unmerged index entry', () => {
    const repository = createRepository();

    git(repository, 'checkout', '--quiet', '-b', 'other');
    write(repository, 'file.txt', 'theirs\n');
    git(repository, 'commit', '--quiet', '-am', 'theirs');

    git(repository, 'checkout', '--quiet', 'main');
    write(repository, 'file.txt', 'ours\n');
    git(repository, 'commit', '--quiet', '-am', 'ours');

    const merge = spawnSync('git', ['merge', 'other'], { cwd: repository, encoding: 'utf8' });
    assert.notEqual(merge.status, 0, 'the fixture has to actually conflict');

    // Resolve the file on disk but leave the index unmerged.
    write(repository, 'file.txt', 'resolved\n');

    const result = run(repository);

    assert.equal(result.status, 1);
    assert.match(result.stdout, /file\.txt/);
  });

  await t.test('reports each path once, including paths containing spaces', () => {
    const repository = createRepository({ 'a dir/first.txt': 'x\n', 'second.txt': 'x\n' });
    write(repository, 'a dir/first.txt', `${MARKERS.opening}\n${MARKERS.closing}\n`);
    write(repository, 'second.txt', `${MARKERS.separator}\n`);
    git(repository, 'add', '.');

    const result = run(repository);
    const reported = result.stdout.split('\n').filter(line => line.includes('.txt'));

    assert.equal(result.status, 1);
    assert.deepEqual(reported.toSorted(), ['a dir/first.txt', 'second.txt']);
  });

  await t.test('rejects an unknown mode', () => {
    const result = run(createRepository(), { mode: 'bogus' });

    assert.equal(result.status, 2);
    assert.match(result.stderr, /Unknown mode/);
  });
});

void test('no-merge-conflicts.sh pushed', { skip: NEEDS_GIT }, async t => {
  const zero = '0'.repeat(40);

  /** The `<local ref> <local oid> <remote ref> <remote oid>` line git feeds pre-push. */
  function refUpdate(repository, { localOid, remoteOid = zero }) {
    return `refs/heads/main ${localOid} refs/heads/main ${remoteOid}\n`;
  }

  await t.test('passes for clean commits', () => {
    const repository = createRepository();
    write(repository, 'file.txt', 'changed\n');
    git(repository, 'commit', '--quiet', '-am', 'clean');

    const head = git(repository, 'rev-parse', 'HEAD');
    const result = run(repository, {
      mode: 'pushed',
      stdin: refUpdate(repository, { localOid: head }),
    });

    assert.equal(result.status, 0);
  });

  await t.test('rejects a marker committed since the remote ref', () => {
    const repository = createRepository();
    const base = git(repository, 'rev-parse', 'HEAD');

    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'commit', '--quiet', '-am', 'oops');
    const head = git(repository, 'rev-parse', 'HEAD');

    const result = run(repository, {
      mode: 'pushed',
      stdin: refUpdate(repository, { localOid: head, remoteOid: base }),
    });

    assert.equal(result.status, 1);
    assert.match(result.stdout, /in the commits being pushed/);
    assert.match(result.stdout, /file\.txt/);
  });

  // A branch the remote has never seen has no base; the script falls back to
  // the empty tree, so the whole branch is scanned rather than nothing.
  await t.test('scans a brand-new branch with no remote counterpart', () => {
    const repository = createRepository();
    write(repository, 'file.txt', `${MARKERS.closing}\n`);
    git(repository, 'commit', '--quiet', '-am', 'oops');
    const head = git(repository, 'rev-parse', 'HEAD');

    const result = run(repository, {
      mode: 'pushed',
      stdin: refUpdate(repository, { localOid: head }),
    });

    assert.equal(result.status, 1);
  });

  // Deleting a remote branch pushes an all-zero local oid, which has no tree.
  await t.test('ignores a ref deletion', () => {
    const repository = createRepository();
    const head = git(repository, 'rev-parse', 'HEAD');

    const result = run(repository, {
      mode: 'pushed',
      stdin: refUpdate(repository, { localOid: zero, remoteOid: head }),
    });

    assert.equal(result.status, 0);
  });

  await t.test('passes when git sends no ref updates', () => {
    assert.equal(run(createRepository(), { mode: 'pushed', stdin: '' }).status, 0);
  });

  // Failing closed matters most here: a bad ref makes `git diff --check` exit
  // non-zero with no findings, which is indistinguishable from a clean tree.
  await t.test('fails closed on an unresolvable remote oid', () => {
    const repository = createRepository();
    const head = git(repository, 'rev-parse', 'HEAD');

    const result = run(repository, {
      mode: 'pushed',
      stdin: refUpdate(repository, { localOid: head, remoteOid: 'f'.repeat(40) }),
    });

    assert.equal(result.status, 2);
    assert.match(result.stderr, /Unable to inspect remote object/);
  });
});
