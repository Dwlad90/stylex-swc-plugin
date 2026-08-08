/**
 * Which `package.json` files the repository treats as source.
 *
 * The answer already exists: `.syncpackrc`'s `source` list, whose negations
 * spell out the four families that are deliberately not source -- fixture
 * manifests that must resolve as if they were real user projects, generated
 * platform manifests, virtual test apps and build output. Re-stating that list
 * in a second file is how the two copies drift, and a drifted copy here means
 * a release rewrites a fixture or skips a published package.
 *
 * So this module reads `.syncpackrc` and nothing else decides the scope.
 *
 * Not a `*.test.mjs` file, so `pnpm test:scripts` does not try to run it as a
 * suite; it is covered through the scripts that use it.
 */

import fs from 'node:fs';
import path from 'node:path';

/**
 * `.syncpackrc`'s `source` list, split into the patterns that select and the
 * patterns that exclude. Syncpack spells an exclusion as a leading `!`;
 * `fs.globSync` takes the two apart instead, so the `!` is stripped here.
 *
 * @param {string} root repository root
 * @returns {{include: string[], exclude: string[]}}
 */
export function readManifestScope(root) {
  const file = path.join(root, '.syncpackrc');
  const source = JSON.parse(fs.readFileSync(file, 'utf8')).source;

  if (!Array.isArray(source) || source.length === 0) {
    throw new Error(`${file} declares no \`source\` patterns -- cannot determine manifest scope`);
  }

  return {
    include: source.filter(pattern => !pattern.startsWith('!')),
    exclude: source.filter(pattern => pattern.startsWith('!')).map(pattern => pattern.slice(1)),
  };
}

/**
 * Every source manifest, as paths relative to `root`, sorted so that callers
 * report in a stable order.
 *
 * @param {string} root repository root
 * @returns {string[]}
 */
export function findSourceManifests(root) {
  const { include, exclude } = readManifestScope(root);

  return fs.globSync(include, { cwd: root, exclude }).toSorted();
}

/**
 * The generated platform manifests for the NAPI binding.
 *
 * They are excluded from the source scope above -- nothing hand-edits them --
 * but they are published to npm under their own names, so their `version` is
 * part of a release even though their contents are not source. The bumper owns
 * them for that reason; the catalog check in the migration's contract step
 * does not.
 *
 * @param {string} root repository root
 * @returns {string[]}
 */
export function findPublishedPlatformManifests(root) {
  return fs.globSync(['crates/stylex-rs-compiler/npm/*/package.json'], { cwd: root }).toSorted();
}
