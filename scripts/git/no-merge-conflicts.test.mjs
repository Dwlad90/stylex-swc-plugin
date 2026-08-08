import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import { git, makeTemporaryDirectory, missing } from './lib/test-harness.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const script = path.join(repoRoot, 'scripts/git/no-merge-conflicts.sh');

const NEEDS_GIT = missing('git', 'bash');

/**
 * Markers are assembled rather than written literally so that no line of this
 * file starts with one. `git diff --check` -- the very thing under test -- would
 * otherwise flag this file when it is committed, and unlike the Haaretz original
 * this port has no self-exemption pathspec to hide behind.
 */
const MARKERS = {
  opening: `${'<'.repeat(7)} HEAD`,
  base: `${'|'.repeat(7)} merged common ancestors`,
  separator: '='.repeat(7),
  closing: `${'>'.repeat(7)} feature`,
};

const CONFLICTED_FILE = [
  MARKERS.opening,
  'ours',
  MARKERS.separator,
  'theirs',
  MARKERS.closing,
  '',
].join('\n');

const ZERO_OID = '0'.repeat(40);

function tryGit(cwd, ...args) {
  return spawnSync('git', args, { cwd, encoding: 'utf8' });
}

function write(cwd, file, contents) {
  const target = path.join(cwd, file);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, contents);
}

function configure(directory) {
  git(directory, 'config', 'user.email', 'test@example.com');
  git(directory, 'config', 'user.name', 'Test');
  // The repository's own hooks must not fire inside the fixtures. Pointing at a
  // directory that does not exist is more thorough than `--no-verify`, which
  // only covers the commands that accept it.
  git(directory, 'config', 'core.hooksPath', path.join(directory, '.git/no-hooks'));
}

function createEmptyRepository(branch = 'main') {
  const directory = makeTemporaryDirectory('no-merge-conflicts-');

  git(directory, 'init', `--initial-branch=${branch}`, '--quiet');
  configure(directory);

  return directory;
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

/**
 * A real clone, so `origin/<branch>` exists. Pushed mode resolves the base for a
 * never-pushed branch through `origin/develop`, which a bare fixture repository
 * cannot exercise at all.
 */
function createClonedRepository(remoteBranch = 'main') {
  const parent = makeTemporaryDirectory('no-merge-conflicts-clone-');
  const remote = path.join(parent, 'remote');
  const local = path.join(parent, 'local');

  fs.mkdirSync(remote);
  git(remote, 'init', `--initial-branch=${remoteBranch}`, '--quiet');
  configure(remote);
  write(remote, 'file.txt', 'original\n');
  git(remote, 'add', '.');
  git(remote, 'commit', '--quiet', '-m', 'initial');

  git(parent, 'clone', '--quiet', remote, local);
  configure(local);

  return local;
}

/** A repository sitting in a genuine conflicted-merge state. */
function createConflictedRepository(file = 'file.txt') {
  const directory = createRepository({ [file]: 'base\n' });

  git(directory, 'checkout', '--quiet', '-b', 'other');
  write(directory, file, 'theirs\n');
  git(directory, 'commit', '--quiet', '-am', 'theirs');

  git(directory, 'checkout', '--quiet', 'main');
  write(directory, file, 'ours\n');
  git(directory, 'commit', '--quiet', '-am', 'ours');

  const merge = tryGit(directory, 'merge', 'other');
  assert.notEqual(merge.status, 0, 'the fixture has to actually conflict');

  return directory;
}

/** The `<local ref> <local oid> <remote ref> <remote oid>` line git feeds pre-push. */
function pushedInput(directory) {
  const localRef = git(directory, 'symbolic-ref', 'HEAD');
  const localOid = git(directory, 'rev-parse', 'HEAD');
  const remote = tryGit(directory, 'ls-remote', 'origin', localRef);
  const remoteOid =
    remote.status === 0 && remote.stdout.trim() ? remote.stdout.trim().split(/\s+/)[0] : ZERO_OID;

  return `${localRef} ${localOid} ${localRef} ${remoteOid}\n`;
}

function run(cwd, { mode, stdin, from } = {}) {
  return spawnSync('bash', mode ? [script, mode] : [script], {
    cwd: from ? path.join(cwd, from) : cwd,
    encoding: 'utf8',
    input: stdin ?? (mode === 'pushed' ? pushedInput(cwd) : ''),
  });
}

function assertPasses(cwd, options) {
  const result = run(cwd, options);
  assert.equal(result.status, 0, `${result.stdout}${result.stderr}`);
}

/** How many times `text` appears, so "reported once" is a real assertion. */
function occurrences(value, text) {
  return value.split(text).length - 1;
}

void test('no-merge-conflicts.sh staged', { skip: NEEDS_GIT }, async t => {
  await t.test('passes when nothing is staged', () => {
    assertPasses(createRepository());
  });

  await t.test('passes for ordinary staged changes', () => {
    const repository = createRepository();
    write(repository, 'file.txt', 'changed\n');
    git(repository, 'add', 'file.txt');

    assertPasses(repository);
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

  await t.test('reports each path once, including paths containing spaces', () => {
    const repository = createRepository({ 'a dir/first.txt': 'x\n', 'second.txt': 'x\n' });
    write(repository, 'a dir/first.txt', `${MARKERS.opening}\n`);
    write(repository, 'second.txt', `${MARKERS.closing}\n`);
    git(repository, 'add', '.');

    const result = run(repository);

    assert.equal(result.status, 1);
    assert.equal(occurrences(result.stdout, 'a dir/first.txt'), 1);
    assert.equal(occurrences(result.stdout, 'second.txt'), 1);
  });

  // A whole conflict block is four marker lines in one file. Reporting the path
  // once is what keeps the output readable on a large botched merge.
  await t.test('reports a file holding a whole conflict block once', () => {
    const repository = createRepository();
    write(repository, 'file.txt', CONFLICTED_FILE);
    git(repository, 'add', 'file.txt');

    const result = run(repository);

    assert.equal(result.status, 1);
    assert.equal(occurrences(result.stdout, 'file.txt'), 1);
  });

  await t.test('rejects an unknown mode', () => {
    const result = run(createRepository(), { mode: 'bogus' });

    assert.equal(result.status, 2);
    assert.match(result.stderr, /Unknown mode/);
  });

  await t.test('staged mode may be named explicitly', () => {
    const repository = createRepository();
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'add', 'file.txt');

    assert.equal(run(repository, { mode: 'staged' }).status, 1);
  });

  // `git diff` is not limited by the working directory, so the hook catches a
  // marker anywhere in the repository no matter where it was invoked from.
  await t.test('checks the whole repository when run from a subdirectory', () => {
    const repository = createRepository({ 'nested/deep/file.txt': 'x\n', 'root.txt': 'x\n' });
    write(repository, 'root.txt', `${MARKERS.opening}\n`);
    git(repository, 'add', '.');

    const result = run(repository, { from: 'nested/deep' });

    assert.equal(result.status, 1);
    assert.match(result.stdout, /root\.txt/);
  });
});

void test('no-merge-conflicts.sh staged, what it must not flag', { skip: NEEDS_GIT }, async t => {
  // The staged/unstaged distinction is the whole contract: the hook judges what
  // you are about to commit, not what happens to be on disk.
  await t.test('ignores a marker that is only in unstaged content', () => {
    const repository = createRepository();
    write(repository, 'file.txt', 'staged change\n');
    git(repository, 'add', 'file.txt');
    write(repository, 'file.txt', `staged change\n${MARKERS.opening}\n`);

    assertPasses(repository);
  });

  await t.test('still rejects staged content the working copy has since fixed', () => {
    const repository = createRepository();
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'add', 'file.txt');
    write(repository, 'file.txt', 'fixed in the working tree\n');

    assert.equal(run(repository).status, 1);
  });

  await t.test('ignores markers in untracked files', () => {
    const repository = createRepository();
    write(repository, 'untracked.txt', `${MARKERS.opening}\n`);

    assertPasses(repository);
  });

  // git skips binary content, so a `.node` addon or a fixture that happens to
  // contain the byte sequence cannot fail the hook.
  await t.test('ignores markers inside staged binary content', () => {
    const repository = createRepository();
    write(
      repository,
      'blob.bin',
      Buffer.concat([Buffer.from(`${MARKERS.opening}\n`), Buffer.from([0])])
    );
    git(repository, 'add', 'blob.bin');

    assertPasses(repository);
  });

  // git requires exactly seven characters at the start of the line. Six is a
  // horizontal rule, and an indented or quoted one is documentation -- this file
  // and HOOKS.md both contain them.
  await t.test('ignores lookalikes: six characters, indented, and quoted', () => {
    const repository = createRepository();
    write(
      repository,
      'file.txt',
      [
        '<'.repeat(6),
        '='.repeat(6),
        '>'.repeat(6),
        `  ${MARKERS.opening}`,
        `const marker = "${MARKERS.closing}";`,
        '',
      ].join('\n')
    );
    git(repository, 'add', 'file.txt');

    assertPasses(repository);
  });

  // `git diff --check` also reports trailing whitespace and space-before-tab.
  // The script filters on the marker diagnostic alone; if that filter ever
  // broke, every whitespace nit in the repository would become a failed commit.
  await t.test('does not turn whitespace diagnostics into conflict failures', () => {
    const repository = createRepository();
    write(repository, 'file.txt', 'trailing whitespace   \n \tspace before tab\n');
    git(repository, 'add', 'file.txt');

    assertPasses(repository);
  });

  // Only *added* lines are checked, which is what makes resolving a conflict
  // possible at all -- otherwise the commit that removes the markers would be
  // the one the hook rejects.
  await t.test('allows staged removal of a marker', () => {
    const repository = createRepository({ 'file.txt': `${MARKERS.opening}\nresolved\n` });
    write(repository, 'file.txt', 'resolved\n');
    git(repository, 'add', 'file.txt');

    assertPasses(repository);
  });

  await t.test('allows deleting a file that contains a marker', () => {
    const repository = createRepository({ 'file.txt': `${MARKERS.opening}\n` });
    fs.rmSync(path.join(repository, 'file.txt'));
    git(repository, 'add', 'file.txt');

    assertPasses(repository);
  });

  await t.test('allows editing a file around an untouched marker-like line', () => {
    const repository = createRepository({ 'file.txt': `${MARKERS.opening}\noriginal\n` });
    write(repository, 'file.txt', `${MARKERS.opening}\nchanged\n`);
    git(repository, 'add', 'file.txt');

    assertPasses(repository);
  });

  await t.test('allows renaming a file that already contains marker-like content', () => {
    const repository = createRepository({ 'old name.txt': `${MARKERS.opening}\n` });
    git(repository, 'mv', 'old name.txt', 'new name.txt');

    assertPasses(repository);
  });
});

void test('no-merge-conflicts.sh unmerged index', { skip: NEEDS_GIT }, async t => {
  // A conflicted index is a separate failure from marker text: the file on disk
  // may read perfectly while the index still holds three stages.
  await t.test('rejects an unmerged entry whose file was resolved on disk', () => {
    const repository = createConflictedRepository();
    write(repository, 'file.txt', 'resolved\n');

    const result = run(repository);

    assert.equal(result.status, 1);
    assert.match(result.stdout, /file\.txt/);
  });

  await t.test('reports an unmerged entry once, not once per stage', () => {
    const repository = createConflictedRepository();

    const result = run(repository);

    assert.equal(result.status, 1);
    assert.equal(occurrences(result.stdout, 'file.txt'), 1);
  });

  // The Haaretz original excludes its own path from the marker scan. This port
  // deliberately does not, so a marker committed into the checker is caught like
  // any other -- pinned here because dropping an exemption is easy to undo by
  // accident.
  await t.test('does not exempt the checker script from its own scan', () => {
    const self = 'scripts/git/no-merge-conflicts.sh';
    const repository = createRepository({ [self]: '#!/usr/bin/env bash\nexit 0\n' });
    write(repository, self, `#!/usr/bin/env bash\n${MARKERS.opening}\n`);
    git(repository, 'add', self);

    const result = run(repository);

    assert.equal(result.status, 1);
    assert.match(result.stdout, /no-merge-conflicts\.sh/);
  });
});

void test('no-merge-conflicts.sh pushed', { skip: NEEDS_GIT }, async t => {
  await t.test('passes when there is nothing new to push', () => {
    assertPasses(createClonedRepository(), { mode: 'pushed' });
  });

  await t.test('passes for clean commits in the pushed range', () => {
    const repository = createClonedRepository();
    write(repository, 'file.txt', 'a perfectly fine change\n');
    git(repository, 'commit', '--quiet', '-am', 'clean');

    assertPasses(repository, { mode: 'pushed' });
  });

  // The case the pre-commit hook cannot cover: `--no-verify`, or a commit made
  // before the hooks were installed.
  await t.test('rejects a marker committed past the pre-commit hook', () => {
    const repository = createClonedRepository();
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'commit', '--quiet', '-am', 'conflict');

    const result = run(repository, { mode: 'pushed' });

    assert.equal(result.status, 1);
    assert.match(result.stdout, /in the commits being pushed/);
    assert.match(result.stdout, /file\.txt/);
  });

  await t.test('rejects a marker arriving through a merge commit', () => {
    const repository = createClonedRepository();
    git(repository, 'checkout', '--quiet', '-b', 'topic');
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'commit', '--quiet', '-am', 'conflict');
    git(repository, 'checkout', '--quiet', 'main');
    git(repository, 'merge', '--quiet', 'topic');

    assert.equal(run(repository, { mode: 'pushed' }).status, 1);
  });

  await t.test('ignores markers that are staged but not committed', () => {
    const repository = createClonedRepository();
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'add', 'file.txt');

    assertPasses(repository, { mode: 'pushed' });
  });

  // The range is diffed end to end, not commit by commit, so a mess that was
  // cleaned up before the push is not the push's problem.
  await t.test('allows a marker that a later commit in the range removes', () => {
    const repository = createClonedRepository();
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'commit', '--quiet', '-am', 'conflict');
    write(repository, 'file.txt', 'resolved\n');
    git(repository, 'commit', '--quiet', '-am', 'resolution');

    assertPasses(repository, { mode: 'pushed' });
  });

  // A branch the remote has never seen has no base. The script falls back to
  // `origin/develop` so the scan covers the branch rather than all of history.
  await t.test('checks a new branch with no upstream, via origin/develop', () => {
    const repository = createClonedRepository('develop');
    git(repository, 'checkout', '--quiet', '-b', 'feature');
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'commit', '--quiet', '-am', 'conflict');

    assert.equal(run(repository, { mode: 'pushed' }).status, 1);
  });

  // ...and when there is no `origin/develop` either, the empty tree is the base,
  // so a first push of a fresh repository is still scanned rather than skipped.
  await t.test('checks a new ref with no remote configured at all', () => {
    const repository = createRepository();
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'commit', '--quiet', '-am', 'conflict');

    assert.equal(run(repository, { mode: 'pushed' }).status, 1);
  });

  // git names the ref being pushed; the hook must scan that, not whatever
  // happens to be checked out.
  await t.test('checks the ref git supplied rather than HEAD', () => {
    const repository = createClonedRepository();
    git(repository, 'checkout', '--quiet', '-b', 'topic');
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'commit', '--quiet', '-am', 'conflict');
    const topic = git(repository, 'rev-parse', 'HEAD');
    git(repository, 'checkout', '--quiet', 'main');

    const result = run(repository, {
      mode: 'pushed',
      stdin: `refs/heads/topic ${topic} refs/heads/topic ${ZERO_OID}\n`,
    });

    assert.equal(result.status, 1);
    assert.match(result.stdout, /file\.txt/);
  });

  await t.test('scans every ref in a multi-ref push', () => {
    const repository = createClonedRepository();
    const base = git(repository, 'rev-parse', 'HEAD');

    git(repository, 'checkout', '--quiet', '-b', 'clean-topic');
    write(repository, 'clean.txt', 'fine\n');
    git(repository, 'add', '.');
    git(repository, 'commit', '--quiet', '-m', 'clean');
    const clean = git(repository, 'rev-parse', 'HEAD');

    git(repository, 'checkout', '--quiet', base);
    git(repository, 'checkout', '--quiet', '-b', 'dirty-topic');
    write(repository, 'dirty.txt', `${MARKERS.closing}\n`);
    git(repository, 'add', '.');
    git(repository, 'commit', '--quiet', '-m', 'dirty');
    const dirty = git(repository, 'rev-parse', 'HEAD');

    const result = run(repository, {
      mode: 'pushed',
      stdin:
        `refs/heads/clean-topic ${clean} refs/heads/clean-topic ${base}\n` +
        `refs/heads/dirty-topic ${dirty} refs/heads/dirty-topic ${base}\n`,
    });

    assert.equal(result.status, 1, 'a clean first ref must not mask a dirty second one');
    assert.match(result.stdout, /dirty\.txt/);
  });

  await t.test('deduplicates a path that is dirty on more than one ref', () => {
    const repository = createClonedRepository();
    const base = git(repository, 'rev-parse', 'HEAD');
    write(repository, 'file.txt', `${MARKERS.opening}\n`);
    git(repository, 'commit', '--quiet', '-am', 'conflict');
    const head = git(repository, 'rev-parse', 'HEAD');

    const result = run(repository, {
      mode: 'pushed',
      stdin:
        `refs/heads/one ${head} refs/heads/one ${base}\n` +
        `refs/heads/two ${head} refs/heads/two ${base}\n`,
    });

    assert.equal(result.status, 1);
    assert.equal(occurrences(result.stdout, 'file.txt'), 1);
  });

  await t.test('ignores a ref deletion', () => {
    const repository = createClonedRepository();
    const head = git(repository, 'rev-parse', 'HEAD');

    assertPasses(repository, {
      mode: 'pushed',
      stdin: `(delete) ${ZERO_OID} refs/heads/old ${head}\n`,
    });
  });

  await t.test('passes when git sends no ref updates', () => {
    assertPasses(createClonedRepository(), { mode: 'pushed', stdin: '' });
  });

  // Tags can point at trees or blobs, which have no commit to diff. Skipping
  // them is correct; treating them as an error would break `git push --tags`.
  await t.test('skips a tag object that does not point at a commit', () => {
    const repository = createRepository();
    const blob = git(repository, 'hash-object', '-w', '--stdin');
    const tag = spawnSync('git', ['mktag'], {
      cwd: repository,
      encoding: 'utf8',
      input: `object ${blob}\ntype blob\ntag blob-tag\ntagger Test <test@example.com> 0 +0000\n\nmessage\n`,
    });

    assert.equal(tag.status, 0, `git mktag failed: ${tag.stderr}`);

    assertPasses(repository, {
      mode: 'pushed',
      stdin: `refs/tags/blob-tag ${tag.stdout.trim()} refs/tags/blob-tag ${ZERO_OID}\n`,
    });
  });
});

// Failing closed matters most here. A ref git cannot resolve makes
// `git diff --check` exit non-zero with no findings, which reads exactly like a
// clean tree -- so the one bug this whole mode could hide is "silently passed".
void test('no-merge-conflicts.sh fails closed', { skip: NEEDS_GIT }, async t => {
  await t.test('on an unresolvable remote oid', () => {
    const repository = createClonedRepository();
    const head = git(repository, 'rev-parse', 'HEAD');

    const result = run(repository, {
      mode: 'pushed',
      stdin: `refs/heads/main ${head} refs/heads/main ${'f'.repeat(40)}\n`,
    });

    assert.equal(result.status, 2);
    assert.match(result.stderr, /Unable to inspect remote object/);
  });

  await t.test('on a local oid that is not in the object database', () => {
    const repository = createClonedRepository();

    const result = run(repository, {
      mode: 'pushed',
      stdin: `refs/heads/missing ${'1'.repeat(40)} refs/heads/missing ${ZERO_OID}\n`,
    });

    assert.equal(result.status, 2, 'a missing object is an error, not a clean tree');
    assert.match(result.stderr, /Unable to inspect local object/);
  });

  await t.test('when neither side of the range resolves', () => {
    const result = run(createRepository(), {
      mode: 'pushed',
      stdin: `refs/heads/main ${'e'.repeat(40)} refs/heads/main ${'d'.repeat(40)}\n`,
    });

    assert.equal(result.status, 2);
  });

  // The `checked_diff` guard proper: git's own diagnostics are preserved, but a
  // `fatal:` from git is escalated instead of being read as "found nothing".
  await t.test('when git itself refuses the comparison', () => {
    const repository = createConflictedRepository();

    // `git diff --check --cached` is fatal in a conflicted index only for some
    // git versions, so drive the guard directly with a ref that cannot resolve
    // while the local object is genuinely present.
    const head = git(repository, 'rev-parse', 'HEAD');
    const result = run(repository, {
      mode: 'pushed',
      stdin: `refs/heads/main ${head} refs/heads/main ${'a'.repeat(40)}\n`,
    });

    assert.equal(result.status, 2);
    assert.match(result.stderr, /Unable to inspect remote object/);
  });
});
