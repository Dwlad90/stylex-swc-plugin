#!/usr/bin/env node

/**
 * Drops the resolved version of a split catalogued package from
 * `pnpm-lock.yaml`, so that the next install resolves it again.
 *
 * Usage: `node scripts/git/dedupe-catalog-pins.mjs [--root <dir>]`
 *
 * A package catalogued twice must resolve to one version. A catalog entry in
 * the lockfile is a pin, so `pnpm dedupe` cannot collapse a split: to obey the
 * pin is what a catalog is for. To delete the pin is what lets the install
 * resolve the entry again, inside the range `pnpm-workspace.yaml` declares.
 * `guidelines/SCRIPTS.md` explains why the two ranges differ and what the
 * split costs.
 *
 * Every pin for the package goes, not only the stale one. To tell which is
 * stale needs a version comparison this repository has no semver dependency to
 * make, and a range resolved again inside its own bounds cannot leave them.
 *
 * This repairs; it does not assert. `catalog-integrity.mjs duplicates` is the
 * gate, and `version-mismatch-check.sh` runs it in pre-commit and in
 * `pr-validation`. `sync-deps.yml` runs that mode, and `lockfile` mode,
 * straight after this repair -- not a check confirming its own repair, because
 * this script selects no version and puts nothing back, so what those two read
 * is the result of the install.
 */

import fs from 'node:fs';
import path from 'node:path';

import {
  blockKey,
  conflictingPins,
  describePins,
  ignorable,
  indentOf,
  LOCKFILE,
  parseLockfileCatalogs,
  readTextFile,
} from './lib/catalogs.mjs';

/** @returns {never} */
function fail(message) {
  process.stderr.write(`dedupe-catalog-pins: ${message}\n`);
  process.exit(1);
}

function parseArguments(argv) {
  let root = process.cwd();

  for (let index = 0; index < argv.length; index += 1) {
    if (argv[index] === '--root') {
      index += 1;
      root = argv[index] ?? fail('--root needs a directory');
    } else {
      fail(`unknown argument \`${argv[index]}\` -- see the file header`);
    }
  }

  return path.resolve(root);
}

/**
 * The indentation of a package entry: `catalogs:` at column zero, the catalog
 * two in, the package four. Named because the depth is what tells a catalog
 * name from a package name -- both are a bare `<key>:`.
 */
const ENTRY_INDENT = 4;

/**
 * The lockfile with every `catalogs:` entry for `names` removed.
 *
 * Line-based, like the reader in `lib/catalogs.mjs`, because a YAML round-trip
 * would reformat far more of the file than it changed.
 *
 * @param {string} text lockfile contents
 * @param {Set<string>} names packages whose entries to drop
 * @returns {{text: string, dropped: number}}
 */
function withoutPins(text, names) {
  const lines = text.split('\n');
  // The caller found conflicts in this file, so it has a `catalogs:` block.
  const start = lines.findIndex(line => line.startsWith('catalogs:'));
  const kept = lines.slice(0, start + 1);
  let dropped = 0;
  let index = start + 1;

  while (index < lines.length) {
    const line = lines[index];

    // Column zero ends the block: the next top-level key of the file.
    if (!ignorable(line) && indentOf(line) === 0) {
      break;
    }

    const name = ignorable(line) ? null : blockKey(line);

    if (name !== null && indentOf(line) === ENTRY_INDENT && names.has(name)) {
      index += 1;

      // The entry owns the deeper-indented lines under it and nothing else. A
      // blank line ends it. pnpm writes one between catalogs, and to swallow it
      // would join two catalogs in a file whose only review is a diff.
      while (
        index < lines.length &&
        lines[index].trim() !== '' &&
        indentOf(lines[index]) > ENTRY_INDENT
      ) {
        index += 1;
      }

      dropped += 1;
      continue;
    }

    kept.push(line);
    index += 1;
  }

  return { text: [...kept, ...lines.slice(index)].join('\n'), dropped };
}

/**
 * The lockfile, or a named failure when it cannot be read.
 *
 * @param {string} file
 * @returns {string}
 */
function readLockfile(file) {
  try {
    return readTextFile(file);
  } catch (error) {
    return fail(error instanceof Error ? error.message : String(error));
  }
}

const root = parseArguments(process.argv.slice(2));
const file = path.join(root, LOCKFILE);

// One read serves both the report and the rewrite. A second read is a second
// file, if an install or another copy of this script writes between them, and
// the rewrite would then drop pins from a lockfile nobody examined. The write
// that follows is still not atomic; what this removes is the window inside
// this script.
const lockfile = readLockfile(file);
const conflicts = conflictingPins(parseLockfileCatalogs(lockfile, LOCKFILE));

if (conflicts.length === 0) {
  process.stdout.write('dedupe-catalog-pins: every catalogued package resolves to one version\n');
  process.exit(0);
}

for (const { name, pins } of conflicts) {
  process.stdout.write(`dedupe-catalog-pins: \`${name}\` resolves to ${describePins(pins)}\n`);
}

const { text, dropped } = withoutPins(lockfile, new Set(conflicts.map(conflict => conflict.name)));

fs.writeFileSync(file, text);

process.stdout.write(
  `dedupe-catalog-pins: dropped ${dropped} pin(s) from ${LOCKFILE} -- ` +
    `run \`pnpm install --no-frozen-lockfile\` to resolve them again\n`
);
