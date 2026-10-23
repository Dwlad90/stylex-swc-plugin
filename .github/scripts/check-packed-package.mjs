#!/usr/bin/env node
/**
 * Fails when the tarball `pnpm pack` writes for a package cannot be loaded as
 * published: a `files` entry of its package.json matches nothing in the
 * tarball, or a relative `require` or `import` in a packed JavaScript file
 * names a target the tarball does not carry.
 *
 * Usage: node check-packed-package.mjs <package directory>
 *
 * npm says nothing about a `files` entry that matches nothing, so a module
 * the entry point requires can be absent while the publish succeeds. The
 * test jobs run on the whole build artifact and never see the `dist` the
 * publish job assembles; this check reads the tarball that job would ship.
 */

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { fail, failWithErrors } from './lib/ci.mjs';

/** A relative specifier in `require('./x')`, `from './x'` or `import('./x')`. */
const RELATIVE_SPECIFIER = /(?:\brequire\s*\(|\bfrom|\bimport\s*\()\s*(['"])(\.\.?\/[^'"\n]+)\1/g;

/** The files a relative specifier can name, in Node's resolution order. */
const RESOLUTION_SUFFIXES = ['', '.js', '.cjs', '.mjs', '.json', '/index.js', '/index.cjs'];

/**
 * A native binding or the wasi shim: the loader NAPI-RS generates probes one
 * per platform inside `try`, and the platform packages carry them.
 */
const OPTIONAL_BINDING = /\.(?:node|wasi\.cjs)$/;

const PACKED_SCRIPT = /\.(?:[cm]?js)$/;

/**
 * The regular expression npm's `files` glob stands for: `**` crosses
 * directories, `*` and `?` stay inside one.
 */
export function filesPatternToRegExp(pattern) {
  let source = '';
  for (let i = 0; i < pattern.length; i++) {
    const char = pattern[i];
    if (char === '*') {
      if (pattern[i + 1] === '*') {
        i++;
        if (pattern[i + 1] === '/') {
          i++;
          source += '(?:.*/)?';
        } else {
          source += '.*';
        }
      } else {
        source += '[^/]*';
      }
    } else if (char === '?') {
      source += '[^/]';
    } else {
      source += char.replace(/[.+^${}()|[\]\\]/g, '\\$&');
    }
  }
  return new RegExp(`^${source}$`);
}

/**
 * The `files` entries that match no path in the tarball. An entry names a
 * file, a directory (everything beneath it ships), or a glob.
 */
export function unmatchedFilesEntries(files, entries) {
  return files.filter(entry => {
    const clean = entry.replace(/^\.\//, '').replace(/\/$/, '');
    const pattern = filesPatternToRegExp(clean);
    return !entries.some(
      packed => packed === clean || packed.startsWith(`${clean}/`) || pattern.test(packed)
    );
  });
}

/**
 * Every relative specifier in a packed script whose target is not in the
 * tarball, as `{ file, specifier }`, the optional binding probes aside.
 * `readText(entry)` gives a packed file's text.
 */
export function unresolvedRelativeSpecifiers(entries, readText) {
  const packed = new Set(entries);
  const missing = [];
  for (const file of entries) {
    if (!PACKED_SCRIPT.test(file)) continue;
    const text = readText(file);
    for (const match of text.matchAll(RELATIVE_SPECIFIER)) {
      const specifier = match[2];
      if (OPTIONAL_BINDING.test(specifier)) continue;
      const target = path.posix.normalize(path.posix.join(path.posix.dirname(file), specifier));
      const resolves = RESOLUTION_SUFFIXES.some(suffix => packed.has(`${target}${suffix}`));
      if (!resolves) missing.push({ file, specifier });
    }
  }
  return missing;
}

/** Runs a command, failing the check when it does not exit 0. */
function run(command, args, options) {
  const result = spawnSync(command, args, { encoding: 'utf8', ...options });
  if (result.error) fail(`${command} ${args.join(' ')}: ${result.error.message}`);
  if (result.status !== 0) {
    fail(`${command} ${args.join(' ')} exited ${result.status}:\n${result.stderr}${result.stdout}`);
  }
  return result.stdout;
}

/** Packs the package into a fresh directory and returns the tarball's path. */
export function packInto(packageDir, destination) {
  run('pnpm', ['pack', '--pack-destination', destination], { cwd: packageDir });
  const tarballs = fs.readdirSync(destination).filter(name => name.endsWith('.tgz'));
  if (tarballs.length !== 1) {
    fail(`expected one tarball in ${destination}, found ${tarballs.length}`);
  }
  return path.join(destination, tarballs[0]);
}

/** Extracts the tarball and returns its paths without the `package/` root. */
export function extractTarball(tarball, destination) {
  fs.mkdirSync(destination, { recursive: true });
  run('tar', ['-xzf', tarball, '-C', destination]);
  const root = path.join(destination, 'package');
  const entries = [];
  const walk = dir => {
    for (const dirent of fs.readdirSync(dir, { withFileTypes: true })) {
      const file = path.join(dir, dirent.name);
      if (dirent.isDirectory()) walk(file);
      else entries.push(path.relative(root, file).split(path.sep).join('/'));
    }
  };
  walk(root);
  return { root, entries: entries.toSorted((a, b) => a.localeCompare(b, 'en')) };
}

function isMainModule() {
  return (
    process.argv[1] !== undefined &&
    path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
  );
}

if (isMainModule()) {
  const packageDir = process.argv[2];
  if (packageDir === undefined) fail('Usage: check-packed-package.mjs <package directory>');

  const manifest = JSON.parse(fs.readFileSync(path.join(packageDir, 'package.json'), 'utf8'));
  const workDir = fs.mkdtempSync(path.join(os.tmpdir(), 'check-packed-package-'));
  try {
    const tarball = packInto(packageDir, workDir);
    const { root, entries } = extractTarball(tarball, path.join(workDir, 'extracted'));
    const errors = [];
    for (const entry of unmatchedFilesEntries(manifest.files ?? [], entries)) {
      errors.push(`package.json "files" entry ${entry} matches nothing in the tarball`);
    }
    const readText = entry => fs.readFileSync(path.join(root, entry), 'utf8');
    for (const { file, specifier } of unresolvedRelativeSpecifiers(entries, readText)) {
      errors.push(`${file} requires ${specifier}, which the tarball does not carry`);
    }
    if (errors.length > 0) {
      failWithErrors(`${path.basename(tarball)} cannot be loaded as published:`, errors);
    }
    console.log(
      `ok  ${path.basename(tarball)}: ${entries.length} files, every relative require resolves`
    );
  } finally {
    fs.rmSync(workDir, { recursive: true, force: true });
  }
}
