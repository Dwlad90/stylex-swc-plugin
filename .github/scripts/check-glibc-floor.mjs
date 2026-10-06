#!/usr/bin/env node
/**
 * Fails when a linux-gnu binding needs a newer glibc than GLIBC_FLOOR.
 *
 * Usage: node check-glibc-floor.mjs <file.node>...
 *
 * A binary records each glibc version it needs as a name such as
 * `GLIBC_2.34` in its ELF string table. The loader refuses the binary when
 * the host glibc is older than the highest of these names, and Node then
 * reports only "Cannot find native binding". This check reads the names from
 * the file bytes, so it works for every architecture on every host.
 *
 * It reads only the `GLIBC_<number>` names. It ignores `GLIBCXX_` and `GCC_`,
 * which a C++ dependency adds, and names without a number such as
 * `GLIBC_ABI_DT_RELR`. Add a check for them when the binding gets a C++
 * dependency.
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { fail, failWithErrors } from './lib/ci.mjs';

/**
 * The highest glibc a binding can need. Amazon Linux 2023, which Vercel and
 * AWS CodeBuild use, ships this version.
 */
const GLIBC_FLOOR = '2.34';

/** The bytes before a version name: the NUL that ends the previous name. */
const GLIBC_PREFIX = Buffer.from('\0GLIBC_', 'latin1');

/** A whole version such as `2.2.5`, which rejects names like `GLIBC_PRIVATE`. */
const VERSION_PATTERN = /^\d+(?:\.\d+)+$/;

/** Longer than any real version, so a decoded name stays a few bytes long. */
const MAX_VERSION_LENGTH = 16;

/** Compares two dotted versions part by part as numbers. */
export function compareVersions(a, b) {
  const left = a.split('.').map(Number);
  const right = b.split('.').map(Number);
  for (let i = 0; i < Math.max(left.length, right.length); i++) {
    const difference = (left[i] ?? 0) - (right[i] ?? 0);
    if (difference !== 0) return difference;
  }
  return 0;
}

/**
 * Returns the highest glibc version that the binary needs.
 *
 * It searches the bytes and decodes only each name, because a debug build is
 * larger than the longest string that Node can make.
 */
export function highestGlibcVersion(bytes) {
  let highest;
  let at = bytes.indexOf(GLIBC_PREFIX);
  while (at !== -1) {
    const start = at + GLIBC_PREFIX.length;
    const end = bytes.indexOf(0, start);
    if (end === -1) break;
    if (end - start <= MAX_VERSION_LENGTH) {
      const version = bytes.toString('latin1', start, end);
      if (
        VERSION_PATTERN.test(version) &&
        (highest === undefined || compareVersions(version, highest) > 0)
      ) {
        highest = version;
      }
    }
    // The NUL that ends this name can start the next one.
    at = bytes.indexOf(GLIBC_PREFIX, end);
  }
  return highest;
}

function isMainModule() {
  return (
    process.argv[1] !== undefined &&
    path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
  );
}

if (isMainModule()) {
  const files = process.argv.slice(2);
  if (files.length === 0) fail('Usage: check-glibc-floor.mjs <file.node>...');

  const errors = [];
  for (const file of files) {
    let bytes;
    try {
      bytes = fs.readFileSync(file);
    } catch (error) {
      errors.push(`${file}: cannot read the file (${error.code ?? error.message})`);
      continue;
    }
    const version = highestGlibcVersion(bytes);
    if (version === undefined) {
      errors.push(`${file} contains no glibc version names. Give a linux-gnu binary.`);
    } else if (compareVersions(version, GLIBC_FLOOR) > 0) {
      errors.push(
        `${file} needs glibc ${version}, but the floor is ${GLIBC_FLOOR}. ` +
          'Hosts with an older glibc cannot load it.'
      );
    } else {
      console.log(`ok  ${file} needs glibc ${version}`);
    }
  }

  if (errors.length > 0) failWithErrors('glibc floor check failed:', errors);
}
