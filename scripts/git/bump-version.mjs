#!/usr/bin/env node

/**
 * Moves the release version to every place it lives, and fails loudly when any
 * one of them does not move.
 *
 * Usage: `node scripts/git/bump-version.mjs <version> [--root <dir>]`
 *
 * The version lives in four places, and they have to agree or a release ships
 * a package whose dependency ranges point at a version that was never
 * published:
 *
 *   1. the Cargo workspace package version -- crates inherit it, so no crate
 *      manifest carries a literal
 *   2. the `version` field and every internal `@stylexswc/*` dependency range
 *      across the source manifests, plus the generated platform manifests that
 *      are published under their own names
 *   3. the release badge in the root README
 *   4. the `internal` catalog block in `pnpm-workspace.yaml`
 *
 * This replaces a `find`-and-`sed`/`jq` shell script with two defects worth
 * naming, because they are what the shape below is built to prevent:
 *
 *   - `sed` matched `^version = "..."` in *every* `Cargo.toml`. It happened to
 *     hit exactly one line per file only because `[workspace.dependencies]`
 *     uses inline-table syntax; an entry written as a plain `version =` at
 *     column zero would have been silently rewritten to the release version.
 *     Here the Cargo rewrite is scoped to the `[workspace.package]` section and
 *     asserts it hit exactly one line.
 *   - A substitution that matches nothing exits zero, so a bump that quietly
 *     did nothing looked identical to one that worked. Here every location is
 *     re-read and must report the new version before anything reaches disk, and
 *     a run that changes no file at all is an error.
 *
 * Fixture, virtual and other non-source manifests are deliberately left alone
 * -- see `lib/manifests.mjs`. The shell script rewrote them because its `find`
 * could not tell them apart; nothing consumes the version they carry.
 */

import fs from 'node:fs';
import path from 'node:path';

import {
  DEPENDENCY_FIELDS,
  findPublishedPlatformManifests,
  findSourceManifests,
  isLiteralRange,
} from './lib/manifests.mjs';

/** `<major>.<minor>.<patch>` with an optional prerelease tail. */
const VERSION = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/;

/** A prerelease is not what the release badge points at -- see `bumpReadme`. */
const STABLE_VERSION = /^\d+\.\d+\.\d+$/;

/** `[section.name]`, with whatever whitespace and trailing comment. */
const TOML_SECTION = /^\s*\[([^\]]+)]/;

/** A literal `version = "..."` assignment; `version.workspace = true` is not one. */
const TOML_VERSION = /^(\s*version\s*=\s*")([^"]*)(".*)$/;

/**
 * The release-status badge in the root README, which points at the git tag of
 * the current release. Anchored on the repository path so that the StyleX
 * compatibility badge on the neighbouring line -- a different version, owned by
 * `update-stylex-compatibility.sh` -- is out of reach.
 */
const README_BADGE_PREFIX = 'stylex-swc-plugin/';
const README_BADGE = /stylex-swc-plugin\/\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?/g;

/**
 * An internal dependency whose range this script owns.
 *
 * A specifier carrying a scheme -- `workspace:`, `file:`, `link:`, `catalog:`
 * and anything else of that shape -- is a reference, not a version range, and
 * is not ours to rewrite. `catalog:` matters most: overwriting it with a
 * literal would quietly undo the catalog migration on the next release.
 */
function isInternalRange(name, range) {
  return name.startsWith('@stylexswc/') && isLiteralRange(range);
}

/** One `<key>: <value>` line of a YAML block, keeping the quoting intact. */
const YAML_ENTRY = /^(\s+)('[^']*'|"[^"]*"|[^\s#][^:]*)(:\s*)('[^']*'|"[^"]*"|\S+)(\s*(?:#.*)?)$/;

function fail(message) {
  process.stderr.write(`bump-version: ${message}\n`);
  process.exit(1);
}

function parseArguments(argv) {
  let version;
  let root = process.cwd();

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];

    if (argument === '--root') {
      index += 1;
      root = argv[index] ?? fail('--root needs a directory');
    } else if (argument.startsWith('-')) {
      fail(`unknown option \`${argument}\``);
    } else if (version === undefined) {
      version = argument;
    } else {
      fail(`unexpected argument \`${argument}\``);
    }
  }

  if (version === undefined) {
    fail('usage: bump-version.mjs <version> [--root <dir>]');
  }

  if (!VERSION.test(version)) {
    fail(`\`${version}\` is not a version of the form <major>.<minor>.<patch>[-prerelease]`);
  }

  return { version, root: path.resolve(root) };
}

/**
 * The rewrites this run intends, and the reasons it cannot finish.
 *
 * Rewrites are staged in memory and flushed only once every location has been
 * rewritten and verified. A bump that fails halfway would otherwise leave the
 * tree carrying two versions at once, which is a worse state to recover from
 * than not having run at all.
 */
function createRun(root) {
  return {
    root,
    staged: new Map(),
    /** @type {string[]} */
    problems: [],
    get changed() {
      return [...this.staged.keys()];
    },
    read(file) {
      return this.staged.get(file) ?? fs.readFileSync(path.join(root, file), 'utf8');
    },
    write(file, before, after) {
      if (before !== after) {
        this.staged.set(file, after);
      }
    },
    problem(message) {
      this.problems.push(message);
    },
    flush() {
      for (const [file, contents] of this.staged) {
        fs.writeFileSync(path.join(root, file), contents);
      }
    },
  };
}

/**
 * The `[workspace.package]` version in the root `Cargo.toml`, and nothing else
 * in that file: `[workspace.dependencies]` entries carry versions too, and the
 * whole point of scoping by section is that they stay untouched however they
 * are written.
 */
function rewriteCargoWorkspaceVersion(text, version) {
  let section = '';
  const hits = [];

  const lines = text.split('\n').map(line => {
    const header = line.match(TOML_SECTION);

    if (header) {
      section = header[1].trim();
      return line;
    }

    const assignment = section === 'workspace.package' ? line.match(TOML_VERSION) : null;

    if (!assignment) {
      return line;
    }

    hits.push(assignment[2]);
    return `${assignment[1]}${version}${assignment[3]}`;
  });

  return { text: lines.join('\n'), hits };
}

function bumpCargo(run, version) {
  const file = 'Cargo.toml';
  const before = run.read(file);
  const { text, hits } = rewriteCargoWorkspaceVersion(before, version);

  if (hits.length !== 1) {
    run.problem(
      `${file} has ${hits.length} \`version = "..."\` lines under [workspace.package]; expected exactly one`
    );
    return;
  }

  run.write(file, before, text);
}

/**
 * Crates inherit the workspace version, so a crate manifest carrying a literal
 * one is a crate the bump would silently skip. Cheaper to refuse than to
 * discover in a published artefact.
 *
 * Scoped to `crates/*`, the workspace members: a Cargo manifest deeper in the
 * tree is a fixture or an example, and pinning a literal version is a
 * legitimate thing for one of those to do.
 */
function assertCratesInheritVersion(run) {
  const manifests = fs.globSync(['crates/*/Cargo.toml'], { cwd: run.root }).toSorted();

  for (const file of manifests) {
    let section = '';

    for (const line of run.read(file).split('\n')) {
      const header = line.match(TOML_SECTION);

      if (header) {
        section = header[1].trim();
      } else if (section === 'package' && TOML_VERSION.test(line)) {
        run.problem(
          `${file} declares a literal \`version\`; crates must inherit it with \`version.workspace = true\``
        );
        break;
      }
    }
  }
}

/** The manifests whose version is part of a release. */
function releaseManifests(root) {
  return [...findSourceManifests(root), ...findPublishedPlatformManifests(root)];
}

function rewriteManifest(manifest, version) {
  if (typeof manifest.version === 'string') {
    manifest.version = version;
  }

  for (const field of DEPENDENCY_FIELDS) {
    const dependencies = manifest[field];

    if (!dependencies) {
      continue;
    }

    for (const [name, range] of Object.entries(dependencies)) {
      if (isInternalRange(name, range)) {
        dependencies[name] = version;
      }
    }
  }

  return manifest;
}

function bumpManifests(run, version) {
  const files = releaseManifests(run.root);

  if (files.length === 0) {
    run.problem('found no manifests to bump -- check `source` in .syncpackrc');
    return;
  }

  for (const file of files) {
    const before = run.read(file);
    const manifest = rewriteManifest(JSON.parse(before), version);

    run.write(file, before, `${JSON.stringify(manifest, null, 2)}\n`);
  }
}

/**
 * The badge points at the tag of the current *release*, so a prerelease leaves
 * it alone: `0.19.0-rc.1` has no release to link to, and pointing the badge at
 * it would break the link until the release lands.
 */
function bumpReadme(run, version) {
  const file = 'README.md';

  if (!STABLE_VERSION.test(version)) {
    return;
  }

  const before = run.read(file);

  if (before.match(README_BADGE) === null) {
    run.problem(`${file} no longer contains the release badge this script owns`);
    return;
  }

  run.write(file, before, before.replaceAll(README_BADGE, `${README_BADGE_PREFIX}${version}`));
}

/** A YAML line's indentation, which is what delimits a block. */
function indentOf(line) {
  return line.length - line.trimStart().length;
}

/** Blank lines and comments belong to no block and end none. */
function ignorable(line) {
  return line.trim() === '' || line.trimStart().startsWith('#');
}

/**
 * The `internal` catalog block of `pnpm-workspace.yaml`, rewritten line by line
 * rather than through a YAML round-trip: the file is mostly comments explaining
 * overrides and approved build scripts, and serialising it back would discard
 * every one of them.
 *
 * Returns the block's entry lines with their indices, `null` when the file
 * declares no catalogs at all -- the state before they exist, and not an error
 * -- or `{ missing: true }` when it declares catalogs but no `internal` among
 * them.
 *
 * That last case is the one worth spending code on. Once the catalogs land, a
 * renamed or deleted `internal` block would otherwise leave this location
 * quietly unbumped while the other three moved and the run still exited zero --
 * precisely the silent no-op this script replaced a `sed` to avoid.
 */
function findInternalCatalogEntries(text) {
  const lines = text.split('\n');
  const catalogs = lines.findIndex(line => line.startsWith('catalogs:'));

  if (catalogs === -1) {
    return null;
  }

  let internal = -1;
  let internalIndent = 0;

  for (let index = catalogs + 1; index < lines.length; index += 1) {
    const line = lines[index];

    if (ignorable(line)) {
      continue;
    }

    if (indentOf(line) === 0) {
      break;
    }

    if (/^\s+internal:\s*(?:#.*)?$/.test(line)) {
      internal = index;
      internalIndent = indentOf(line);
      break;
    }
  }

  if (internal === -1) {
    return { missing: true, entries: [] };
  }

  const entries = [];

  for (let index = internal + 1; index < lines.length; index += 1) {
    const line = lines[index];

    if (ignorable(line)) {
      continue;
    }

    if (indentOf(line) <= internalIndent) {
      break;
    }

    entries.push({ index, match: line.match(YAML_ENTRY) });
  }

  return { lines, entries };
}

function bumpInternalCatalog(run, version) {
  const file = 'pnpm-workspace.yaml';
  const before = run.read(file);
  const block = findInternalCatalogEntries(before);

  if (block === null) {
    return;
  }

  if (block.missing) {
    run.problem(`${file} declares catalogs but no \`internal\` one for this script to move`);
    return;
  }

  if (block.entries.length === 0) {
    run.problem(`${file} declares an \`internal\` catalog with no entries`);
    return;
  }

  const { lines, entries } = block;

  for (const { index, match } of entries) {
    if (!match) {
      run.problem(`${file}: cannot read \`internal\` catalog entry on line ${index + 1}`);
      continue;
    }

    const [, indent, key, separator, value, trailing] = match;
    const quote = value.startsWith("'") || value.startsWith('"') ? value[0] : '';

    lines[index] = `${indent}${key}${separator}${quote}${version}${quote}${trailing}`;
  }

  run.write(file, before, lines.join('\n'));
}

/**
 * Re-reads every location from the bytes this run is about to write and reports
 * the ones that do not say the new version. This is the check the shell script
 * could not make: it is what separates "the rewrite worked" from "the pattern
 * matched nothing".
 */
function verify(run, version) {
  const stale = [];

  const { hits } = rewriteCargoWorkspaceVersion(run.read('Cargo.toml'), version);

  if (hits.some(found => found !== version)) {
    stale.push('Cargo.toml [workspace.package] version');
  }

  for (const file of releaseManifests(run.root)) {
    const manifest = JSON.parse(run.read(file));

    if (typeof manifest.version === 'string' && manifest.version !== version) {
      stale.push(`${file} version`);
    }

    for (const field of DEPENDENCY_FIELDS) {
      for (const [name, range] of Object.entries(manifest[field] ?? {})) {
        if (isInternalRange(name, range) && range !== version) {
          stale.push(`${file} ${field}.${name}`);
        }
      }
    }
  }

  if (STABLE_VERSION.test(version)) {
    for (const badge of run.read('README.md').match(README_BADGE) ?? []) {
      if (badge !== `${README_BADGE_PREFIX}${version}`) {
        stale.push(`README.md ${badge}`);
      }
    }
  }

  const block = findInternalCatalogEntries(run.read('pnpm-workspace.yaml'));

  for (const { index, match } of block?.entries ?? []) {
    if (match?.[4].replaceAll(/['"]/g, '') !== version) {
      stale.push(`pnpm-workspace.yaml internal catalog, line ${index + 1}`);
    }
  }

  return stale;
}

const { version, root } = parseArguments(process.argv.slice(2));
const run = createRun(root);

assertCratesInheritVersion(run);
bumpCargo(run, version);
bumpManifests(run, version);
bumpReadme(run, version);
bumpInternalCatalog(run, version);

if (run.problems.length === 0) {
  for (const location of verify(run, version)) {
    run.problem(`${location} did not move to ${version}`);
  }
}

if (run.problems.length > 0) {
  for (const problem of run.problems) {
    process.stderr.write(`bump-version: ${problem}\n`);
  }

  process.exit(1);
}

const changed = run.changed;

if (changed.length === 0) {
  fail(`nothing to do -- every version location already reads ${version}`);
}

run.flush();

for (const file of changed) {
  process.stdout.write(`${file}\n`);
}

process.stdout.write(`bump-version: ${changed.length} file(s) moved to ${version}\n`);
