/**
 * The catalogs, as `pnpm-workspace.yaml` declares them and as `pnpm-lock.yaml`
 * records them resolved.
 *
 * Read by hand rather than through a YAML parser, for the same reason
 * `bump-version.mjs` rewrites the file line by line: `pnpm-workspace.yaml` is
 * mostly comments explaining overrides, approved build scripts and the cut of
 * the catalogs themselves, and this repository has no YAML dependency to parse
 * it with. Reading is the easier half -- nothing here writes either file back,
 * so the comments are never at risk.
 *
 * The shape accepted is the one pnpm documents and the two files use -- the
 * declaration two levels deep, the lockfile's record of it three:
 *
 *     catalogs:                        catalogs:
 *       <catalog>:                       <catalog>:
 *         <package>: <range>               <package>:
 *                                            specifier: <range>
 *                                            version: <resolved>
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
export const LOCKFILE = 'pnpm-lock.yaml';

/** `<key>: <value>`, either side optionally single- or double-quoted. */
const ENTRY = /^(?:'([^']*)'|"([^"]*)"|([^\s#][^:]*?))\s*:\s*(?:'([^']*)'|"([^"]*)"|(\S+))$/;

/** `<key>:` with nothing after it -- a nested block, not a leaf. */
const BLOCK = /^(?:'([^']*)'|"([^"]*)"|([^\s#][^:]*?))\s*:$/;

/**
 * `ENTRY` for a writer: the whole `<key>: <value>` line, with indentation,
 * quoting and trailing comment each captured intact so the value can be
 * replaced and the rest put back exactly as found. Still only grammar --
 * nothing in this module writes; `bump-version.mjs` is the one that does.
 */
export const ENTRY_LINE =
  /^(\s+)('[^']*'|"[^"]*"|[^\s#][^:]*)(:\s*)('[^']*'|"[^"]*"|\S+)(\s*(?:#.*)?)$/;

/** Blank lines and comments belong to no block and end none. */
export function ignorable(line) {
  return line.trim() === '' || line.trimStart().startsWith('#');
}

/** A YAML line's indentation, which is what delimits a block. */
export function indentOf(line) {
  return line.length - line.trimStart().length;
}

function keyOf(match) {
  return match[1] ?? match[2] ?? match[3];
}

function valueOf(match) {
  return match[4] ?? match[5] ?? match[6];
}

/**
 * Every mapping key from the root of the block down to `value`, as a YAML-ish
 * path for an error message.
 *
 * @param {string[]} keys
 */
function pathOf(keys) {
  return keys.length === 0 ? '`catalogs:`' : `\`catalogs.${keys.join('.')}\``;
}

/**
 * Rejects a block whose leaves do not all sit at `depth` mappings down.
 *
 * The module header promises that a shape this reader does not understand is
 * rejected rather than half-understood, and without this that promise held only
 * for a line it could not parse. A scalar where a mapping belongs parses fine
 * and means something else entirely: `catalogs: {webpack: '^5'}` is the default
 * catalog this workspace does not have, read as a catalog named after a
 * package, and a lockfile leaf written as a bare version would enumerate as one
 * "entry" per character index. Both would pass a check silently.
 *
 * @param {unknown} value
 * @param {number} depth mappings between the block and its leaves
 * @param {string} label how to name the file in an error
 * @param {string[]} keys the path walked so far
 */
function assertDepth(value, depth, label, keys = []) {
  const scalar = typeof value !== 'object' || value === null;

  if (depth === 0) {
    if (!scalar) {
      throw new Error(`${label}: ${pathOf(keys)} nests deeper than a catalog entry goes`);
    }

    return;
  }

  if (scalar) {
    throw new Error(`${label}: ${pathOf(keys)} is a value where a mapping belongs`);
  }

  for (const [key, child] of Object.entries(value)) {
    assertDepth(child, depth - 1, label, [...keys, key]);
  }
}

/**
 * The `catalogs:` block of `file`, or `null` when the file has no such block.
 * Keys keep their declaration order.
 *
 * `depth` says how many mappings sit between the block and its leaves, because
 * the two files that carry the block do not agree: the declaration's leaf is
 * the range itself, the lockfile's leaf is a `specifier`/`version` pair. One
 * reader serves both rather than two readers agreeing about pnpm's quoting
 * rules by hand, and `depth` is what keeps that sharing from costing either of
 * them a shape check.
 *
 * @param {string} file
 * @param {number} depth
 * @param {string} label how to name the file in an error
 * @returns {Record<string, unknown> | null}
 */
function readCatalogsBlock(file, depth, label) {
  const lines = fs.readFileSync(file, 'utf8').split('\n');
  const start = lines.findIndex(line => line.startsWith('catalogs:'));

  if (start === -1) {
    return null;
  }

  /** @type {Record<string, unknown>} */
  const catalogs = {};
  // `-1`, so the first line of the block is always deeper than the root.
  const openBlocks = [{ indent: -1, value: catalogs }];

  for (let index = start + 1; index < lines.length; index += 1) {
    const line = lines[index];

    if (ignorable(line)) {
      continue;
    }

    const indent = indentOf(line);

    // Column zero ends the block: the next top-level key of the file.
    if (indent === 0) {
      break;
    }

    while (openBlocks.length > 1 && indent <= openBlocks.at(-1).indent) {
      openBlocks.pop();
    }

    const parent = openBlocks.at(-1).value;
    const text = line.trim();
    const block = text.match(BLOCK);

    if (block) {
      const child = {};
      parent[keyOf(block)] = child;
      openBlocks.push({ indent, value: child });
      continue;
    }

    const entry = text.match(ENTRY);

    if (!entry) {
      throw new Error(`${label}:${index + 1}: cannot read catalog entry \`${text}\``);
    }

    parent[keyOf(entry)] = valueOf(entry);
  }

  assertDepth(catalogs, depth, label);

  return catalogs;
}

/**
 * `{ <catalog>: { <package>: <range> } }`, in declaration order.
 *
 * @param {string} root repository root
 * @returns {Record<string, Record<string, string>>}
 */
export function readCatalogs(root) {
  const catalogs = readCatalogsBlock(path.join(root, WORKSPACE_FILE), 2, WORKSPACE_FILE);

  if (catalogs === null) {
    throw new Error(`${WORKSPACE_FILE} declares no \`catalogs:\` block`);
  }

  if (Object.keys(catalogs).length === 0) {
    throw new Error(`${WORKSPACE_FILE} declares a \`catalogs:\` block with no catalogs`);
  }

  return /** @type {Record<string, Record<string, string>>} */ (catalogs);
}

/**
 * `{ <catalog>: { <package>: {specifier, version} } }` as `pnpm-lock.yaml`
 * records it, or `{}` when the lockfile carries no `catalogs:` block at all.
 *
 * The empty case is not an error here: "every catalog entry was dropped" is
 * exactly the corruption the caller is looking for, and it reports it far
 * better naming the entries than this could naming the block.
 *
 * @param {string} file path to a lockfile
 * @returns {Record<string, Record<string, {specifier?: string, version?: string}>>}
 */
export function readLockfileCatalogs(file) {
  const catalogs = readCatalogsBlock(file, 3, path.basename(file));

  return /** @type {Record<string, Record<string, object>>} */ (catalogs ?? {});
}

/**
 * Every entry of a catalog mapping as `<catalog>.<package>` pairs, sorted, so
 * that two lockfiles can be compared as flat sets.
 *
 * @param {Record<string, Record<string, unknown>>} catalogs
 * @returns {string[]}
 */
export function catalogEntries(catalogs) {
  return Object.entries(catalogs)
    .flatMap(([catalog, entries]) => Object.keys(entries).map(name => `${catalog}.${name}`))
    .toSorted();
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
