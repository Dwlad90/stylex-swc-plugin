#!/usr/bin/env node

/**
 * Asserts that every dependency version in this workspace is declared once, by
 * name, in `pnpm-workspace.yaml`.
 *
 * Usage: `node scripts/git/catalog-integrity.mjs <mode> [options]`
 *
 * Modes:
 *
 *   - `manifests [--root <dir>]` -- no source manifest carries a literal
 *     external range, and every `catalog:` reference it does carry resolves to a
 *     declared entry.
 *   - `lockfile --baseline <file> [--current <file>]` -- every catalog entry the
 *     baseline lockfile resolved is still resolved by the current one, which
 *     defaults to `<root>/pnpm-lock.yaml`.
 *
 * Two assertions over the same data, so one script with one suite rather than
 * two scripts with two sets of wiring -- and the lockfile half is testable at
 * all only because it is here: inline workflow YAML has no seam.
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

import {
  catalogEntries,
  catalogsDeclaring,
  LOCKFILE,
  readCatalogs,
  readLockfileCatalogs,
  WORKSPACE_FILE,
} from './lib/catalogs.mjs';
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

/** The options only `lockfile` mode takes, so `manifests` can reject them. */
const LOCKFILE_OPTIONS = ['--baseline', '--current'];

function parseArguments(argv, modes) {
  let mode;
  let root = process.cwd();
  /** @type {Record<string, string | undefined>} */
  const options = {};

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];

    if (argument === '--root') {
      index += 1;
      root = argv[index] ?? fail('--root needs a directory');
    } else if (LOCKFILE_OPTIONS.includes(argument)) {
      index += 1;
      options[argument] = argv[index] ?? fail(`${argument} needs a lockfile`);
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
    fail(`usage: catalog-integrity.mjs <${modes.join('|')}> [options] -- see the file header`);
  }

  if (mode === 'lockfile' && options['--baseline'] === undefined) {
    fail('lockfile mode needs `--baseline <file>` -- the lockfile to compare against');
  }

  for (const option of mode === 'lockfile' ? [] : LOCKFILE_OPTIONS) {
    if (options[option] !== undefined) {
      fail(`\`${option}\` means nothing to ${mode} mode`);
    }
  }

  return {
    mode,
    root: path.resolve(root),
    baseline: options['--baseline'],
    current: options['--current'],
  };
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

/** @param {{root: string}} options */
function checkManifests({ root }) {
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

/**
 * Every catalog entry the baseline lockfile resolved is still resolved now.
 *
 * Dependabot has understood catalogs since early 2025, but an update can drop
 * an entry from `pnpm-lock.yaml` -- the entry the workspace still declares and
 * a manifest still references, silently unresolved. A reinstall would most
 * likely put it back, and that is the problem: "most likely, as a side effect"
 * is not a guard for the lockfile of a repository that ships native bindings.
 *
 * Which two files those are is the caller's business, and it matters: comparing
 * a *reinstalled* lockfile against anything mostly asserts that the accidental
 * repair worked. `--current` exists so the caller can name the lockfile as it
 * arrived rather than as some later step left it.
 *
 * The comparison is presence only. A specifier that moved is what a dependency
 * update is *for*, and a version that moved with it is the point; an entry that
 * stopped existing is not something any update legitimately does here, because
 * the only caller is a bot that bumps ranges and never removes a dependency.
 *
 * @param {{root: string, baseline: string, current?: string}} options
 */
function checkLockfile({ root, baseline, current }) {
  const resolved = current ?? path.join(root, LOCKFILE);
  const before = catalogEntries(readLockfileCatalogs(baseline));

  if (before.length === 0) {
    return [
      [
        `the baseline ${path.basename(baseline)} records no catalog entries, so this`,
        `check would assert nothing -- is it the right file?`,
      ].join(' '),
    ];
  }

  const after = new Set(catalogEntries(readLockfileCatalogs(resolved)));
  const name = path.basename(resolved);

  return before
    .filter(entry => !after.has(entry))
    .map(entry => `${name} no longer records \`${entry}\`, which the baseline resolved`);
}

/**
 * Each mode's check, and what to say after its problems. The closing paragraph
 * is per mode because the two failures ask for different things: one is a
 * manifest to edit, the other a lockfile to regenerate.
 */
const MODES = {
  manifests: {
    check: checkManifests,
    epilogue:
      `Every dependency version in this workspace is declared once, by name,\n` +
      `in ${WORKSPACE_FILE}. Reference it with \`${REFERENCE}<name>\`\n` +
      `instead of repeating the range.\n`,
  },
  lockfile: {
    check: checkLockfile,
    epilogue:
      `An entry a manifest still references but ${LOCKFILE} no longer resolves\n` +
      `is an unresolved dependency in a repository that ships native bindings.\n` +
      `Run \`pnpm install --no-frozen-lockfile\` and commit the result.\n`,
  },
};

const options = parseArguments(process.argv.slice(2), Object.keys(MODES));
const { check, epilogue } = MODES[options.mode];
let problems;

try {
  problems = check(options);
} catch (error) {
  fail(error.message);
}

if (problems.length > 0) {
  for (const problem of problems) {
    process.stderr.write(`catalog-integrity: ${problem}\n`);
  }

  process.stderr.write(`\n${epilogue}`);
  process.exit(1);
}

process.stdout.write(`catalog-integrity: ${options.mode} ok\n`);
