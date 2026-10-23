import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  filesPatternToRegExp,
  unmatchedFilesEntries,
  unresolvedRelativeSpecifiers,
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

void test('unmatchedFilesEntries reads a directory entry as everything beneath it', () => {
  assert.deepEqual(unmatchedFilesEntries(['dist', './lib/'], ['dist/a/b.js', 'lib/x.js']), []);
  assert.deepEqual(unmatchedFilesEntries(['dist'], ['distribution/a.js']), ['dist']);
});

void test('unresolvedRelativeSpecifiers resolves the way Node does and reports the rest', () => {
  const texts = {
    'dist/index.js': [
      'const transform = require("../dist/transform");',
      'var rules = require("./process-stylex-rules/index.js");',
      'const picomatch = require("picomatch");',
      "import { gate } from './gate.js';",
      "const lazy = await import('./lazy');",
      'const data = require("./data.json");',
      "try { return require('./rs-compiler.linux-x64-gnu.node') } catch (e) {}",
      "try { return require('./rs-compiler.wasi.cjs') } catch (e) {}",
    ].join('\n'),
    'dist/transform.js': 'module.exports = {};',
    'dist/gate.js': 'export const gate = 1;',
    'dist/lazy/index.js': 'module.exports = 2;',
    'dist/data.json': '{}',
    'README.md': 'require("./nothing") in prose is not a script',
  };
  const entries = Object.keys(texts);
  const readText = entry => texts[entry];
  assert.deepEqual(unresolvedRelativeSpecifiers(entries, readText), [
    { file: 'dist/index.js', specifier: './process-stylex-rules/index.js' },
  ]);
  entries.push('dist/process-stylex-rules/index.js');
  texts['dist/process-stylex-rules/index.js'] = 'exports.processStylexRules = () => "";';
  assert.deepEqual(unresolvedRelativeSpecifiers(entries, readText), []);
});

void test('the script fails on a package whose entry point requires a file the files list omits', () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'check-packed-package-test-'));
  try {
    fs.mkdirSync(path.join(dir, 'dist', 'nested'), { recursive: true });
    fs.writeFileSync(
      path.join(dir, 'package.json'),
      JSON.stringify({
        name: 'check-packed-package-fixture',
        version: '0.0.0',
        main: 'dist/index.js',
        files: ['dist/index.js', 'dist/nested/*.js'],
      })
    );
    fs.writeFileSync(path.join(dir, 'dist', 'index.js'), 'require("./nested/index.js");\n');
    // The nested module is a .cjs file, so `dist/nested/*.js` matches nothing
    // and the entry point's require has no target in the tarball.
    fs.writeFileSync(path.join(dir, 'dist', 'nested', 'index.cjs'), 'module.exports = 1;\n');

    const failing = spawnSync('node', [scriptPath, dir], { encoding: 'utf8' });
    assert.equal(failing.status, 1, failing.stdout + failing.stderr);
    assert.match(failing.stderr, /"files" entry dist\/nested\/\*\.js matches nothing/);
    assert.match(failing.stderr, /dist\/index\.js requires \.\/nested\/index\.js/);

    fs.renameSync(
      path.join(dir, 'dist', 'nested', 'index.cjs'),
      path.join(dir, 'dist', 'nested', 'index.js')
    );
    const passing = spawnSync('node', [scriptPath, dir], { encoding: 'utf8' });
    assert.equal(passing.status, 0, passing.stdout + passing.stderr);
    assert.match(passing.stdout, /^ok {2}check-packed-package-fixture-0\.0\.0\.tgz: 3 files/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
