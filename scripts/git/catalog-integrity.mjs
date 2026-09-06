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
 *   - `duplicates [--root <dir>]` -- every package catalogued more than once
 *     resolves to a single version in `<root>/pnpm-lock.yaml`.
 *
 * Three assertions over the same data, so one script with one suite rather
 * than three scripts with three sets of wiring -- and the lockfile half is
 * testable at all only because it is here: inline workflow YAML has no seam.
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
  conflictingPins,
  describePins,
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

/**
 * The fields whose entries an install must resolve. `peerDependencies` is not
 * one of them by itself: `autoInstallPeers` installs a peer only when nothing
 * else in the same manifest already provides the package, so a peer beside a
 * `devDependencies` entry for the same name needs no catalog entry of its own.
 * That pairing is what stops the wide range resolving on its own, which is the
 * split `duplicates` mode exists to catch.
 *
 * Known limit: pnpm skips the peer when the sibling *satisfies* the peer
 * range, and this compares names only. Evaluating a range needs a semver
 * library, which these scripts deliberately do without -- they parse YAML by
 * hand for the same reason. The gap opens only for a `peers` range with an
 * upper bound the narrow twin can outgrow, which today is `@farmfe/core`
 * (`<2.0.0`) and `@swc/core` (`^1`); every other `peers` range is an open
 * `>=`, which no bump of the twin can fall outside. If a twin does outgrow
 * one, pnpm writes the peer pin back and `duplicates` reads it again; what
 * this misses is a later silent drop of that pin.
 */
const INSTALL_FIELDS = ['dependencies', 'devDependencies', 'optionalDependencies'];

/**
 * The `<catalog>.<package>` entries an install still has to resolve.
 *
 * @param {string} root repository root
 * @returns {Set<string>}
 */
function requiredEntries(root) {
  const required = new Set();

  for (const file of findSourceManifests(root)) {
    const manifest = JSON.parse(fs.readFileSync(path.join(root, file), 'utf8'));
    const provided = new Set(INSTALL_FIELDS.flatMap(field => Object.keys(manifest[field] ?? {})));

    for (const field of DEPENDENCY_FIELDS) {
      for (const [name, specifier] of Object.entries(manifest[field] ?? {})) {
        if (typeof specifier !== 'string' || !specifier.startsWith(REFERENCE)) {
          continue;
        }

        if (field !== 'peerDependencies' || !provided.has(name)) {
          required.add(`${specifier.slice(REFERENCE.length)}.${name}`);
        }
      }
    }
  }

  return required;
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
 * Which two files those are is the caller's business, and it matters.
 * `sync-deps.yml` asks two questions with this one mode. Before the sync it
 * names both lockfiles out of git, so that a reinstall cannot repair the thing
 * it is asking about; `--current` exists for that call, to name the lockfile as
 * it arrived rather than as a later step left it. After the sync it reads the
 * repaired file on disk, where running after a reinstall is the point:
 * `dedupe-catalog-pins.mjs` deletes entries deliberately, and this is the only
 * check that sees one stay deleted.
 *
 * The comparison is presence only, which is what lets one mode serve both. A
 * specifier that moved is what a dependency update is *for*, and a version the
 * repair moved is the repair working.
 *
 * An entry that stopped existing is reported only when an install still needs
 * it, which `requiredEntries` decides. A catalog entry exists because something
 * installs from it, so one that nothing installs from any more is a manifest
 * edit finishing, not a lockfile losing a resolution -- and reporting it would
 * make the honest half of that edit impossible to commit.
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
  const required = requiredEntries(root);
  const name = path.basename(resolved);

  return before
    .filter(entry => !after.has(entry) && required.has(entry))
    .map(entry => `${name} no longer records \`${entry}\`, which the baseline resolved`);
}

/**
 * Every package catalogued more than once resolves to a single version.
 *
 * The two ranges differ on purpose, so the declaration cannot show a drift.
 * Only what they resolved to can, and that is in the lockfile.
 *
 * Left alone, the drift reads as a type error in a file nobody touched.
 * Reported here it is one line naming the package and both pins.
 * `guidelines/SCRIPTS.md` explains the mechanism.
 *
 * @param {{root: string}} options
 */
function checkDuplicates({ root }) {
  // A lockfile this check cannot read is a check that asserts nothing, which
  // the reader's header calls worse than one that fails -- so the read is left
  // to throw.
  const conflicts = conflictingPins(readLockfileCatalogs(path.join(root, LOCKFILE)));

  return conflicts.map(({ name, pins }) => `\`${name}\` resolves to ${describePins(pins)}`);
}

/**
 * Each mode's check, and what to say after its problems. The closing paragraph
 * is per mode because the three failures ask for different things: a manifest
 * to edit, a lockfile to regenerate, and a pin to drop.
 */
const MODES = {
  manifests: {
    check: checkManifests,
    epilogue:
      `Every dependency version in this workspace is declared once, by name,\n` +
      `in ${WORKSPACE_FILE}. Reference it with \`${REFERENCE}<name>\`\n` +
      `instead of repeating the range.\n`,
  },
  duplicates: {
    check: checkDuplicates,
    epilogue:
      `A package this workspace catalogues twice must resolve to one version.\n` +
      `Two versions mean two copies of everything that depends on them, whose\n` +
      `types are then nominally unrelated. To repair it, run\n` +
      `\`node scripts/git/dedupe-catalog-pins.mjs\`, which drops the pins from\n` +
      `the \`catalogs:\` block of ${LOCKFILE}, and then\n` +
      `\`pnpm install --no-frozen-lockfile\`, which resolves them again in step\n` +
      `with each other. Do not narrow the \`peers\` range to force it -- that\n` +
      `range is published to consumers.\n`,
  },
  lockfile: {
    check: checkLockfile,
    epilogue:
      `An entry an install still needs but ${LOCKFILE} no longer resolves is an\n` +
      `unresolved dependency in a repository that ships native bindings.\n` +
      `Run \`pnpm install --no-frozen-lockfile\` and commit the result. An entry\n` +
      `nothing installs from any more is not reported, so a manifest edit that\n` +
      `retires one does not have to fight this check.\n`,
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
