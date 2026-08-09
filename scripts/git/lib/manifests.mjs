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
 * The manifest fields a dependency specifier can appear in.
 *
 * Here rather than in each caller for the reason the scope below is here: the
 * release bumper and the catalog check walk the same four fields, and a fifth
 * one added to a single copy is a field one of them silently stops seeing.
 */
export const DEPENDENCY_FIELDS = [
  'dependencies',
  'devDependencies',
  'peerDependencies',
  'optionalDependencies',
];

/** A specifier carrying a scheme: `catalog:`, `workspace:`, `link:`, `npm:`. */
const SPECIFIER_SCHEME = /^[a-z][a-z0-9+.-]*:/i;

/**
 * Whether `specifier` is a literal version range rather than a reference to
 * one declared elsewhere.
 *
 * The two callers ask this question for opposite reasons -- the bumper to find
 * the ranges it owns, the catalog check to find the ranges nobody should be
 * writing -- but it is one question, and two copies of the answer can disagree
 * about a specifier form neither was thinking of.
 *
 * @param {unknown} specifier
 * @returns {boolean}
 */
export function isLiteralRange(specifier) {
  return typeof specifier === 'string' && !SPECIFIER_SCHEME.test(specifier);
}

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
