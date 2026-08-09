#!/usr/bin/env node

/**
 * Asserts that every dependency version in this workspace is declared once, by
 * name, in `pnpm-workspace.yaml`.
 *
 * Usage: `node scripts/git/catalog-integrity.mjs <mode> [--root <dir>]`
 *
 * Modes:
 *
 *   - `manifests` -- no source manifest carries a literal external range, and
 *     every `catalog:` reference it does carry resolves to a declared entry.
 *
 * The catalogs made drift impossible to *express*; this is what stops a
 * manifest opting back out of them. `catalogMode: prefer` was chosen over
 * `strict` precisely so that this check, rather than a `pnpm add` failure, is
 * what teaches the convention -- `strict` fails an add on an uncatalogued
 * dependency without being able to say which of the eight catalogs it belongs
 * in, and a contributor who cannot act on an error routes around it. So the
 * failure output here is the feature: it names the file, the dependency, the
 * range it found and the catalog to use.
 *
 * Scope comes from `.syncpackrc` via `lib/manifests.mjs` and is not restated
 * here. Fixture manifests are resolved as if they were real user projects,
 * generated platform manifests are not source, and virtual test apps and build
 * output are neither -- all four families legitimately carry literal ranges,
 * and a check that flags them is a check that gets disabled by the first
 * person it inconveniences.
 */

import fs from 'node:fs';
import path from 'node:path';

import { catalogsDeclaring, readCatalogs, WORKSPACE_FILE } from './lib/catalogs.mjs';
import { DEPENDENCY_FIELDS, findSourceManifests, isLiteralRange } from './lib/manifests.mjs';

/**
 * The catalog a `peerDependencies` range belongs in when the package is one of
 * the nine declared twice. The pairing is the whole reason the catalog exists:
 * a narrow range we develop against, a wide one we accept from consumers.
 * Suggesting the semantic catalog for a peer range would silently narrow what
 * consumers may install, which is the mistake tickets 05 and 06 were written
 * to avoid -- so it is worth encoding in the suggestion rather than leaving to
 * the reader.
 */
const PEER_CATALOG = 'peers';

const REFERENCE = 'catalog:';

function fail(message) {
  process.stderr.write(`catalog-integrity: ${message}\n`);
  process.exit(1);
}

function parseArguments(argv, modes) {
  let mode;
  let root = process.cwd();

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];

    if (argument === '--root') {
      index += 1;
      root = argv[index] ?? fail('--root needs a directory');
    } else if (argument.startsWith('-')) {
      fail(`unknown option \`${argument}\``);
    } else if (!modes.includes(argument)) {
      fail(`unknown mode \`${argument}\` -- expected one of ${modes.join(', ')}`);
    } else if (mode === undefined) {
      mode = argument;
    } else {
      fail(`unexpected argument \`${argument}\``);
    }
  }

  if (mode === undefined) {
    fail(`usage: catalog-integrity.mjs <${modes.join('|')}> [--root <dir>]`);
  }

  return { mode, root: path.resolve(root) };
}

/**
 * One place a specifier was found. The three reporters below all need the same
 * four values to say anything useful, so they travel together rather than as
 * four parallel parameters.
 *
 * @typedef {{file: string, field: string, name: string, specifier: string}} Site
 */

/**
 * Which catalog to point the contributor at, or `null` when no catalog
 * declares the package and the choice is genuinely theirs to make.
 *
 * @param {Record<string, Record<string, string>>} catalogs
 * @param {Site} site
 */
function suggestCatalog(catalogs, site) {
  const candidates = catalogsDeclaring(catalogs, site.name);

  if (candidates.length === 0) {
    return null;
  }

  if (site.field === 'peerDependencies' && candidates.includes(PEER_CATALOG)) {
    return PEER_CATALOG;
  }

  return candidates.find(catalog => catalog !== PEER_CATALOG) ?? candidates[0];
}

/**
 * @param {Record<string, Record<string, string>>} catalogs
 * @param {Site} site
 */
function literalRangeProblem(catalogs, site) {
  const { file, field, name, specifier } = site;
  const suggestion = suggestCatalog(catalogs, site);
  const advice = suggestion
    ? `use \`${REFERENCE}${suggestion}\``
    : [
        `no catalog declares \`${name}\` -- add it to one of`,
        `${Object.keys(catalogs).join(', ')} in ${WORKSPACE_FILE}, then reference it`,
      ].join(' ');

  return `${file}: ${field}.${name} is the literal range \`${specifier}\` -- ${advice}`;
}

/**
 * A `catalog:<name>` reference nothing resolves. Worth reporting alongside the
 * literal ranges because it is the same invariant read the other way round,
 * and because it is the failure the playground actually hit: a manifest
 * outside the workspace globs kept a reference to an entry
 * `cleanupUnusedCatalogs` had removed for having no referent.
 *
 * @param {Record<string, Record<string, string>>} catalogs
 * @param {Site} site
 */
function danglingReferenceProblem(catalogs, site) {
  const { file, field, name, specifier } = site;
  const catalog = specifier.slice(REFERENCE.length);
  const at = `${file}: ${field}.${name}`;
  const declared = Object.keys(catalogs).join(', ');

  if (catalog === '') {
    return [
      `${at} references the default catalog, which this workspace`,
      `does not declare -- name one of ${declared}`,
    ].join(' ');
  }

  if (!(catalog in catalogs)) {
    return `${at} references \`${REFERENCE}${catalog}\`, which ${WORKSPACE_FILE} does not declare`;
  }

  if (!(name in catalogs[catalog])) {
    const elsewhere = catalogsDeclaring(catalogs, name);
    const hint =
      elsewhere.length > 0
        ? `it is declared in ${elsewhere.join(', ')}`
        : `add \`${name}\` to that catalog in ${WORKSPACE_FILE}`;

    return `${at} references \`${REFERENCE}${catalog}\`, which declares no \`${name}\` -- ${hint}`;
  }

  return null;
}

function checkManifests(root) {
  const catalogs = readCatalogs(root);
  const files = findSourceManifests(root);
  const problems = [];

  if (files.length === 0) {
    return ['found no manifests to check -- check `source` in .syncpackrc'];
  }

  for (const file of files) {
    const manifest = JSON.parse(fs.readFileSync(path.join(root, file), 'utf8'));

    for (const field of DEPENDENCY_FIELDS) {
      for (const [name, specifier] of Object.entries(manifest[field] ?? {})) {
        const site = { file, field, name, specifier };

        if (isLiteralRange(specifier)) {
          problems.push(literalRangeProblem(catalogs, site));
        } else if (typeof specifier === 'string' && specifier.startsWith(REFERENCE)) {
          const problem = danglingReferenceProblem(catalogs, site);

          if (problem) {
            problems.push(problem);
          }
        }
      }
    }
  }

  return problems;
}

const MODES = {
  manifests: checkManifests,
};

const { mode, root } = parseArguments(process.argv.slice(2), Object.keys(MODES));
let problems;

try {
  problems = MODES[mode](root);
} catch (error) {
  fail(error.message);
}

if (problems.length > 0) {
  for (const problem of problems) {
    process.stderr.write(`catalog-integrity: ${problem}\n`);
  }

  process.stderr.write(
    `\nEvery dependency version in this workspace is declared once, by name,\n` +
      `in ${WORKSPACE_FILE}. Reference it with \`${REFERENCE}<name>\`\n` +
      `instead of repeating the range.\n`
  );
  process.exit(1);
}

process.stdout.write(`catalog-integrity: ${mode} ok\n`);
