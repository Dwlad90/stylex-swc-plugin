/**
 * Small helpers shared by release-time CI scripts. Kept as .mjs alongside the
 * scripts that need them; introducing a package boundary just for two
 * functions would be worse than the duplication it removes.
 */

import crypto from 'node:crypto';
import fs from 'node:fs';

/**
 * Hex SHA-256 of a file.
 *
 * Release jobs hash on every host in the build matrix, and `shasum` is not
 * present in Git Bash on the Windows runners -- it exited 127 there. Node is
 * set up on all of them, so hashing goes through this instead of a shell tool.
 */
export function sha256File(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}

export function requireEnv(name) {
  const value = process.env[name];
  if (!value) fail(`Environment variable ${name} is required`);
  return value;
}

export function fail(message) {
  console.error(message);
  process.exit(1);
}

export function failWithErrors(header, errors) {
  console.error(header);
  for (const message of errors) console.error(`  - ${message}`);
  process.exit(1);
}
