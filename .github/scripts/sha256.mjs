#!/usr/bin/env node
/**
 * Prints the hex SHA-256 of each file argument, one digest per line.
 *
 * Exists because the release matrix hashes artifacts on Windows hosts, where
 * Git Bash has no `shasum` (exit 127). Digest only, no filename column, so
 * callers do not need `cut`.
 */

import { fail, sha256File } from './lib/ci.mjs';

const files = process.argv.slice(2);
if (files.length === 0) fail('Usage: sha256.mjs <file> [...]');

for (const file of files) {
  try {
    process.stdout.write(`${sha256File(file)}\n`);
  } catch (error) {
    fail(`Cannot hash ${file}: ${error instanceof Error ? error.message : String(error)}`);
  }
}
