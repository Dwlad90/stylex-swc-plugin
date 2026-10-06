import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import { compareVersions, highestGlibcVersion } from './check-glibc-floor.mjs';

const scriptPath = fileURLToPath(new URL('./check-glibc-floor.mjs', import.meta.url));

/** Builds bytes laid out like an ELF string table: names between NUL bytes. */
function stringTable(...names) {
  return Buffer.from(`\0${names.join('\0')}\0`, 'latin1');
}

void test('highestGlibcVersion compares version parts as numbers', () => {
  const bytes = stringTable('GLIBC_2.2.5', 'GLIBC_2.35', 'GLIBC_2.4', 'GLIBC_2.17', 'GLIBC_2.34');
  assert.equal(highestGlibcVersion(bytes), '2.35');
});

void test('highestGlibcVersion counts only whole version names in a string table', () => {
  const bytes = Buffer.concat([
    stringTable('GLIBC_PRIVATE', 'GLIBCXX_3.4.30', 'GLIBC_2.17', 'XGLIBC_9.9', 'GLIBC_9.9x'),
    stringTable(`GLIBC_9.${'9'.repeat(32)}`),
    // Message text in a data section has no NUL before the name.
    Buffer.from('needs GLIBC_9.99 or newer\0', 'latin1'),
  ]);
  assert.equal(highestGlibcVersion(bytes), '2.17');
});

void test('highestGlibcVersion returns undefined for a binary without glibc names', () => {
  assert.equal(highestGlibcVersion(Buffer.alloc(0)), undefined);
  assert.equal(
    highestGlibcVersion(stringTable('musl', 'GLIBC_', 'GLIBC_2', 'GLIBC_2.')),
    undefined
  );
});

void test('highestGlibcVersion decodes only the bytes of each name', t => {
  // A debug build is larger than the longest string that Node can make, so the
  // search must never decode the whole buffer. The spy records the length of
  // each decode, which stays a few bytes for every name.
  const decode = t.mock.method(Buffer.prototype, 'toString');
  const filler = Buffer.alloc(1 << 20, 0x47);
  const bytes = Buffer.concat([stringTable('GLIBC_2.2.5'), filler, stringTable('GLIBC_2.34')]);

  assert.equal(highestGlibcVersion(bytes), '2.34');
  const lengths = decode.mock.calls.map(({ arguments: [, start, end] }) => end - start);
  assert.deepEqual(lengths, ['2.2.5'.length, '2.34'.length]);
});

void test('highestGlibcVersion ignores a last name that has no closing NUL', () => {
  const bytes = Buffer.from('\0GLIBC_2.17\0GLIBC_2.40', 'latin1');
  assert.equal(highestGlibcVersion(bytes), '2.17');
});

void test('compareVersions treats a missing part as zero', () => {
  assert.equal(compareVersions('2.34', '2.34.0'), 0);
  assert.ok(compareVersions('2.2.5', '2.17') < 0);
  assert.ok(compareVersions('2.35', '2.34.9') > 0);
  assert.ok(compareVersions('10.0', '9.99') > 0);
});

/** Writes each binary to a temporary directory and runs the CLI on them. */
function runCli(t, binaries) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'glibc-floor-'));
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
  const files = Object.entries(binaries).map(([name, bytes]) => {
    const file = path.join(dir, name);
    if (bytes !== undefined) fs.writeFileSync(file, bytes);
    return file;
  });
  const result = spawnSync(process.execPath, [scriptPath, ...files], { encoding: 'utf8' });
  return { ...result, files };
}

void test('CLI passes binaries at or below the floor', t => {
  const { status, stdout, files } = runCli(t, {
    'x64.node': stringTable('GLIBC_2.2.5', 'GLIBC_2.34'),
    'arm64.node': stringTable('GLIBC_2.17'),
  });
  assert.equal(status, 0);
  assert.ok(stdout.includes(`${files[0]} needs glibc 2.34`));
  assert.ok(stdout.includes(`${files[1]} needs glibc 2.17`));
});

void test('CLI fails a binary above the floor and names the version', t => {
  const { status, stderr, files } = runCli(t, {
    'ok.node': stringTable('GLIBC_2.34'),
    'new.node': stringTable('GLIBC_2.34', 'GLIBC_2.35'),
  });
  assert.equal(status, 1);
  assert.ok(stderr.includes(`${files[1]} needs glibc 2.35, but the floor is 2.34`));
  assert.ok(!stderr.includes(files[0]));
});

void test('CLI fails a binary without glibc version names', t => {
  const { status, stderr } = runCli(t, { 'musl.node': stringTable('musl') });
  assert.equal(status, 1);
  assert.match(stderr, /contains no glibc version names/);
});

void test('CLI fails a file that does not exist', t => {
  const { status, stderr } = runCli(t, { 'missing.node': undefined });
  assert.equal(status, 1);
  assert.match(stderr, /cannot read the file/);
});

void test('CLI checks every file when one of them cannot be read', t => {
  const { status, stdout, stderr, files } = runCli(t, {
    'missing.node': undefined,
    'ok.node': stringTable('GLIBC_2.17'),
  });
  assert.equal(status, 1);
  assert.ok(stderr.includes(`${files[0]}: cannot read the file`));
  assert.ok(stdout.includes(`${files[1]} needs glibc 2.17`));
});

void test('CLI fails without arguments', t => {
  const { status, stderr } = runCli(t, {});
  assert.equal(status, 1);
  assert.match(stderr, /^Usage: /);
});
