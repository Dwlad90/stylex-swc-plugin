import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  expandBraces,
  exportSubpaths,
  filesPatternToRegExp,
  hostPlatformPackage,
  unmatchedFilesEntries,
} from './check-packed-package.mjs';

const scriptPath = fileURLToPath(new URL('./check-packed-package.mjs', import.meta.url));

const RS_COMPILER_FILES = [
  'dist/index.d.ts',
  'dist/index.js',
  'dist/process-stylex-rules/*.d.ts',
  'dist/process-stylex-rules/*.js',
  'dist/transform.d.ts',
  'dist/transform.js',
  'README.md',
  'LICENSE',
];

void test('expandBraces writes out every alternative of every set', () => {
  assert.deepEqual(expandBraces('dist/{index,other}.js'), ['dist/index.js', 'dist/other.js']);
  assert.deepEqual(expandBraces('dist/{a,b}/{x,y}.js'), [
    'dist/a/x.js',
    'dist/a/y.js',
    'dist/b/x.js',
    'dist/b/y.js',
  ]);
  assert.deepEqual(expandBraces('dist/{a,{b,c}}.js'), ['dist/a.js', 'dist/b.js', 'dist/c.js']);
  assert.deepEqual(expandBraces('dist/index.js'), ['dist/index.js']);
});

void test('filesPatternToRegExp keeps * inside one directory and lets ** cross them', () => {
  assert.equal(filesPatternToRegExp('dist/*.js').test('dist/index.js'), true);
  assert.equal(filesPatternToRegExp('dist/*.js').test('dist/nested/index.js'), false);
  assert.equal(filesPatternToRegExp('dist/**/*.js').test('dist/nested/deep/index.js'), true);
  assert.equal(filesPatternToRegExp('dist/**/*.js').test('dist/index.js'), true);
  assert.equal(filesPatternToRegExp('dist/index.?s').test('dist/index.js'), true);
  assert.equal(filesPatternToRegExp('dist/index.js').test('dist/index_js'), false);
});

void test('unmatchedFilesEntries names the entries a tarball does not satisfy', () => {
  const shipped = [
    'LICENSE',
    'README.md',
    'dist/index.d.ts',
    'dist/index.js',
    'dist/transform.d.ts',
    'dist/transform.js',
    'package.json',
  ];
  assert.deepEqual(unmatchedFilesEntries(RS_COMPILER_FILES, shipped), [
    'dist/process-stylex-rules/*.d.ts',
    'dist/process-stylex-rules/*.js',
  ]);
  const complete = [
    ...shipped,
    'dist/process-stylex-rules/index.d.ts',
    'dist/process-stylex-rules/index.js',
  ];
  assert.deepEqual(unmatchedFilesEntries(RS_COMPILER_FILES, complete), []);
});

void test('unmatchedFilesEntries reads the forms npm accepts: directories, brace sets, negations', () => {
  assert.deepEqual(unmatchedFilesEntries(['dist', './lib/'], ['dist/a/b.js', 'lib/x.js']), []);
  assert.deepEqual(unmatchedFilesEntries(['dist'], ['distribution/a.js']), ['dist']);
  const entries = ['dist/index.js', 'dist/other.js'];
  assert.deepEqual(
    unmatchedFilesEntries(['dist/{index,other}.js', '!dist/secret.js'], entries),
    []
  );
  assert.deepEqual(unmatchedFilesEntries(['dist/{index,gone}.js'], entries), []);
  assert.deepEqual(unmatchedFilesEntries(['dist/{gone,missing}.js'], entries), [
    'dist/{gone,missing}.js',
  ]);
});

void test('exportSubpaths lists the literal keys of an exports map, else the root', () => {
  assert.deepEqual(exportSubpaths({}), ['.']);
  assert.deepEqual(exportSubpaths({ exports: './index.js' }), ['.']);
  assert.deepEqual(exportSubpaths({ exports: { import: './a.mjs', require: './a.cjs' } }), ['.']);
  assert.deepEqual(
    exportSubpaths({ exports: { '.': './a.js', './plugin': './p.js', './rules/*': './r/*.js' } }),
    ['.', './plugin']
  );
});

void test('hostPlatformPackage picks the one package whose os, cpu and libc name the host', () => {
  const npmDir = fs.mkdtempSync(path.join(os.tmpdir(), 'check-packed-package-npm-'));
  try {
    const write = (name, manifest) => {
      fs.mkdirSync(path.join(npmDir, name));
      fs.writeFileSync(path.join(npmDir, name, 'package.json'), JSON.stringify(manifest));
    };
    write('linux-x64-gnu', { os: ['linux'], cpu: ['x64'], libc: ['glibc'] });
    write('linux-x64-musl', { os: ['linux'], cpu: ['x64'], libc: ['musl'] });
    write('darwin-arm64', { os: ['darwin'], cpu: ['arm64'] });
    fs.mkdirSync(path.join(npmDir, 'not-a-package'));
    const pick = host => path.basename(hostPlatformPackage(npmDir, host));
    assert.equal(pick({ platform: 'linux', arch: 'x64', libc: 'glibc' }), 'linux-x64-gnu');
    assert.equal(pick({ platform: 'linux', arch: 'x64', libc: 'musl' }), 'linux-x64-musl');
    assert.equal(pick({ platform: 'darwin', arch: 'arm64', libc: 'musl' }), 'darwin-arm64');
  } finally {
    fs.rmSync(npmDir, { recursive: true, force: true });
  }
});

/** A plain JavaScript package: `main` requires a nested module and ships a declaration file. */
function fixturePackage(dir) {
  fs.mkdirSync(path.join(dir, 'dist', 'nested'), { recursive: true });
  const write = (file, text) => fs.writeFileSync(path.join(dir, file), text);
  write(
    'package.json',
    `${JSON.stringify(
      {
        name: 'check-packed-package-fixture',
        version: '0.0.0',
        main: 'dist/index.js',
        files: ['dist/index.js', 'dist/index.d.ts', 'dist/nested/*.js'],
      },
      null,
      2
    )}\n`
  );
  write('dist/index.js', 'module.exports = { value: require("./nested/index.js") };\n');
  write('dist/index.d.ts', 'export declare const value: number;\n');
  write('dist/nested/index.js', 'module.exports = 1;\n');
  return { write };
}

const check = dir => spawnSync(process.execPath, [scriptPath, dir], { encoding: 'utf8' });

void test('the script loads the packed package as published, and fails on what cannot load', () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'check-packed-package-test-'));
  try {
    const { write } = fixturePackage(dir);

    // The entry point requires a module the `files` list leaves out of the tarball.
    write('dist/index.js', 'module.exports = { value: require("./gone.js") };\n');
    write('dist/gone.js', 'module.exports = 1;\n');
    const missingModule = check(dir);
    assert.equal(missingModule.status, 1, missingModule.stdout + missingModule.stderr);
    assert.match(missingModule.stderr, /cannot be loaded as published/);
    assert.match(missingModule.stderr, /require\('check-packed-package-fixture'\) exited 1/);
    assert.match(missingModule.stderr, /import\('check-packed-package-fixture'\) exited 1/);
    assert.match(missingModule.stderr, /Cannot find module '\.\/gone\.js'/);

    // The code loads, and the declaration file names a module the tarball does not carry.
    write('dist/index.js', 'module.exports = { value: require("./nested/index.js") };\n');
    write('dist/index.d.ts', "export * from './missing.js';\n");
    const missingTypes = check(dir);
    assert.equal(missingTypes.status, 1, missingTypes.stdout + missingTypes.stderr);
    assert.match(missingTypes.stderr, /tsc over dist\/index\.d\.ts exited (?:1|2)/);
    assert.match(missingTypes.stderr, /TS2307: Cannot find module '\.\/missing\.js'/);

    write('dist/index.d.ts', 'export declare const value: number;\n');
    const passing = check(dir);
    assert.equal(passing.status, 0, passing.stdout + passing.stderr);
    assert.match(
      passing.stdout,
      /^ok {2}check-packed-package-fixture-0\.0\.0\.tgz: 4 files; require, import and tsc load it as published/
    );
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

void test('the script names a files entry that matches nothing before loading anything', () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'check-packed-package-test-'));
  try {
    fixturePackage(dir);
    // The nested module is a .cjs file, so `dist/nested/*.js` matches nothing in the tarball.
    fs.renameSync(
      path.join(dir, 'dist', 'nested', 'index.js'),
      path.join(dir, 'dist', 'nested', 'index.cjs')
    );
    const failing = check(dir);
    assert.equal(failing.status, 1, failing.stdout + failing.stderr);
    assert.match(failing.stderr, /cannot be shipped/);
    assert.match(
      failing.stderr,
      /"files" entry dist\/nested\/\*\.js matches nothing in the tarball/
    );
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
