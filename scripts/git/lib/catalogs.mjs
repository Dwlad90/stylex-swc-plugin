/**
 * The catalogs declared in `pnpm-workspace.yaml`.
 *
 * Read by hand rather than through a YAML parser, for the same reason
 * `bump-version.mjs` rewrites the file line by line: `pnpm-workspace.yaml` is
 * mostly comments explaining overrides, approved build scripts and the cut of
 * the catalogs themselves, and this repository has no YAML dependency to parse
 * it with. Reading is the easier half -- nothing here writes the file back, so
 * the comments are never at risk.
 *
 * The shape it accepts is the one pnpm documents and the one the file uses:
 *
 *     catalogs:
 *       <catalog>:
 *         <package>: <range>
 *
 * Anything else -- a flow mapping, an anchor, a multi-line scalar -- is
 * rejected outright rather than half-understood. A catalog file this check
 * cannot read is a check that silently passes, which is worse than one that
 * fails.
 *
 * Not a `*.test.mjs` file, so `pnpm test:scripts` does not try to run it as a
 * suite; it is covered through the scripts that use it.
 */

import fs from 'node:fs';
import path from 'node:path';

export const WORKSPACE_FILE = 'pnpm-workspace.yaml';

/** `<key>: <value>`, either side optionally single- or double-quoted. */
const ENTRY = /^(?:'([^']*)'|"([^"]*)"|([^\s#][^:]*?))\s*:\s*(?:'([^']*)'|"([^"]*)"|(\S+))$/;

/** `<key>:` with nothing after it -- a nested block, not a leaf. */
const BLOCK = /^(?:'([^']*)'|"([^"]*)"|([^\s#][^:]*?))\s*:$/;

/** Blank lines and comments belong to no block and end none. */
function ignorable(line) {
  return line.trim() === '' || line.trimStart().startsWith('#');
}

function indentOf(line) {
  return line.length - line.trimStart().length;
}

function keyOf(match) {
  return match[1] ?? match[2] ?? match[3];
}

function valueOf(match) {
  return match[4] ?? match[5] ?? match[6];
}

/**
 * `{ <catalog>: { <package>: <range> } }`, in declaration order.
 *
 * @param {string} root repository root
 * @returns {Record<string, Record<string, string>>}
 */
export function readCatalogs(root) {
  const file = path.join(root, WORKSPACE_FILE);
  const lines = fs.readFileSync(file, 'utf8').split('\n');
  const start = lines.findIndex(line => line.startsWith('catalogs:'));

  if (start === -1) {
    throw new Error(`${WORKSPACE_FILE} declares no \`catalogs:\` block`);
  }

  /** @type {Record<string, Record<string, string>>} */
  const catalogs = {};
  let current = null;

  for (let index = start + 1; index < lines.length; index += 1) {
    const line = lines[index];

    if (ignorable(line)) {
      continue;
    }

    // Column zero ends the block: the next top-level key of the file.
    if (indentOf(line) === 0) {
      break;
    }

    const text = line.trim();
    const block = text.match(BLOCK);

    if (block) {
      current = keyOf(block);
      catalogs[current] = {};
      continue;
    }

    const entry = text.match(ENTRY);

    if (!entry || current === null) {
      throw new Error(`${WORKSPACE_FILE}:${index + 1}: cannot read catalog entry \`${text}\``);
    }

    catalogs[current][keyOf(entry)] = valueOf(entry);
  }

  if (Object.keys(catalogs).length === 0) {
    throw new Error(`${WORKSPACE_FILE} declares a \`catalogs:\` block with no catalogs`);
  }

  return catalogs;
}

/**
 * The catalogs declaring `name`, in declaration order.
 *
 * @param {Record<string, Record<string, string>>} catalogs
 * @param {string} name package name
 * @returns {string[]}
 */
export function catalogsDeclaring(catalogs, name) {
  return Object.keys(catalogs).filter(catalog => name in catalogs[catalog]);
}
