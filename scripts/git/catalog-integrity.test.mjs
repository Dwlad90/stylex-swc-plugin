/**
 * Drives the real `catalog-integrity.mjs` against a throwaway workspace shaped
 * like this repository: a `pnpm-workspace.yaml` whose catalogs carry the
 * comments a YAML round-trip would drop and the one package declared twice
 * that motivates `peers`, a `.syncpackrc` with the same four families of
 * exclusion, and one manifest per family that must stay exempt.
 *
 * Every assertion is on exit status and on the message a contributor reads.
 * The message is the deliverable here -- a check that only says "literal range
 * found" pushes them into guessing at the taxonomy, which is the failure mode
 * `catalogMode: prefer` was chosen to avoid -- so the tests treat its content
 * as behaviour rather than as incidental output.
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import {
  hermeticEnvironment,
  makeTemporaryDirectory,
  repoRoot,
  writeJson,
  writeText,
} from './lib/test-harness.mjs';

/**
 * The real `.syncpackrc`'s `source` list, verbatim. Copied rather than
 * paraphrased so that the exemption test exercises every one of the four
 * families ticket 01 enumerated -- both transform fixture trees, the singular
 * `tests/fixture/`, the postcss auto-discovery tree, the generated platform
 * manifests and build output -- rather than a representative sample of them.
 */
const SYNCPACK = {
  source: [
    '**/package.json',
    '!**/node_modules/**',
    '!crates/stylex-path-resolver/fixtures/**',
    '!crates/stylex-transform/src/shared/structures/tests/fixtures/**',
    '!crates/stylex-transform/tests/__virtual__/**',
    '!crates/stylex-transform/tests/fixture/**',
    '!packages/postcss-plugin/__tests__/__auto_discovery_fixtures__/**',
    '!crates/stylex-rs-compiler/npm/**',
    '!**/.next/**',
  ],
};

/**
 * Comments between and inside the catalog blocks, because the reader parses
 * this file by hand and a comment line is what a naive block parser trips on.
 * `@swc/core` appears in two catalogs -- the narrow development range and the
 * wide consumer one -- which is the pairing every `peers` assertion below
 * rests on.
 */
const WORKSPACE_YAML = `packages:
  - packages/*

catalogMode: prefer
cleanupUnusedCatalogs: true

# Load-bearing prose that a YAML round-trip would discard.
catalogs:
  bundlers:
    # Why this range and not another.
    '@swc/core': '^1.15.43'
    webpack: '^5.109.2'

  peers:
    '@swc/core': '^1'
    webpack: '>=5.0.0'

  testing:
    vitest: '^4.1.10'
`;

/**
 * The lockfile's record of the catalogs above -- three levels rather than two,
 * because pnpm writes the range it was given beside the version it resolved.
 * Only the block the check reads is here; nothing in `importers:` or
 * `snapshots:` is load-bearing for it.
 */
const LOCK_YAML = `lockfileVersion: '9.0'

settings:
  autoInstallPeers: true

catalogs:
  bundlers:
    '@swc/core':
      specifier: ^1.15.43
      version: 1.15.43
    webpack:
      specifier: ^5.109.2
      version: 5.109.2

  peers:
    webpack:
      specifier: '>=5.0.0'
      version: 5.109.2

  testing:
    vitest:
      specifier: ^4.1.10
      version: 4.1.10

importers:
  .: {}
`;

/**
 * @param {{workspaceYaml?: string, lockYaml?: string, manifests?: Record<string, object>}} [overrides]
 */
function createFixture(overrides = {}) {
  const root = makeTemporaryDirectory('stylex-catalog-integrity-');
  const file = relative => path.join(root, relative);

  writeText(file('pnpm-workspace.yaml'), overrides.workspaceYaml ?? WORKSPACE_YAML);
  writeText(file('pnpm-lock.yaml'), overrides.lockYaml ?? LOCK_YAML);
  writeJson(file('.syncpackrc'), SYNCPACK);

  writeJson(file('package.json'), {
    name: 'root',
    devDependencies: { vitest: 'catalog:testing' },
  });

  writeJson(file('packages/plugin/package.json'), {
    name: '@stylexswc/plugin',
    dependencies: {
      '@stylexswc/sibling': 'workspace:*',
      '@stylexswc/linked': 'link:../linked',
    },
    devDependencies: { '@swc/core': 'catalog:bundlers', webpack: 'catalog:bundlers' },
    peerDependencies: { '@swc/core': 'catalog:peers', webpack: 'catalog:peers' },
  });

  for (const [relative, manifest] of Object.entries(overrides.manifests ?? {})) {
    writeJson(file(relative), manifest);
  }

  return { root, file };
}

const SCRIPT = path.join(repoRoot, 'scripts/git/catalog-integrity.mjs');

function run(root, ...args) {
  return spawnSync('node', [SCRIPT, ...args, '--root', root], {
    encoding: 'utf8',
    env: hermeticEnvironment(),
  });
}

function check(root, ...extra) {
  return run(root, 'manifests', ...extra);
}

/**
 * `lockfile` mode against a baseline written from `lockYaml`. `current` names
 * the lockfile to compare; omitted, the check reads `<root>/pnpm-lock.yaml`.
 */
function checkLockfile(root, lockYaml, current) {
  const baseline = path.join(root, 'baseline-lock.yaml');

  writeText(baseline, lockYaml);

  return run(root, 'lockfile', '--baseline', baseline, ...(current ? ['--current', current] : []));
}

void test('a fully catalogued workspace passes', () => {
  const { root } = createFixture();
  const result = check(root);

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /manifests ok/);
});

void test('a literal range names the file, the dependency and a suggested catalog', () => {
  const { root } = createFixture({
    manifests: {
      'packages/app/package.json': {
        name: '@stylexswc/app',
        devDependencies: { webpack: '^5.109.2' },
      },
    },
  });

  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /packages\/app\/package\.json/);
  assert.match(result.stderr, /devDependencies\.webpack/);
  assert.match(result.stderr, /\^5\.109\.2/);
  assert.match(result.stderr, /catalog:bundlers/);
});

void test('a literal peer range is pointed at `peers`, not at the semantic catalog', () => {
  const { root } = createFixture({
    manifests: {
      'packages/app/package.json': {
        name: '@stylexswc/app',
        peerDependencies: { '@swc/core': '>=1.0.0' },
      },
    },
  });

  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /peerDependencies\.@swc\/core.*catalog:peers/);
  assert.doesNotMatch(result.stderr, /peerDependencies\.@swc\/core.*catalog:bundlers/);
});

void test('an uncatalogued dependency is told to pick a catalog, and which ones exist', () => {
  const { root } = createFixture({
    manifests: {
      'packages/app/package.json': {
        name: '@stylexswc/app',
        dependencies: { picomatch: '^4.0.4' },
      },
    },
  });

  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /no catalog declares `picomatch`/);
  assert.match(result.stderr, /bundlers, peers, testing/);
});

void test('every problem is reported, not just the first', () => {
  const { root } = createFixture({
    manifests: {
      'packages/app/package.json': {
        name: '@stylexswc/app',
        dependencies: { webpack: '^5.109.2' },
        devDependencies: { vitest: '^4.1.10' },
      },
    },
  });

  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /dependencies\.webpack/);
  assert.match(result.stderr, /devDependencies\.vitest/);
});

void test('`workspace:`, `link:` and other schemes are references, not literal ranges', () => {
  const { root } = createFixture();

  assert.equal(check(root).status, 0);
});

void test('a reference to a catalog that does not exist fails', () => {
  const { root } = createFixture({
    manifests: {
      'packages/app/package.json': {
        name: '@stylexswc/app',
        dependencies: { webpack: 'catalog:frameworks' },
      },
    },
  });

  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /`catalog:frameworks`, which pnpm-workspace\.yaml does not declare/);
});

void test('a reference to a catalog with no such entry says where the entry does live', () => {
  const { root } = createFixture({
    manifests: {
      'packages/app/package.json': {
        name: '@stylexswc/app',
        devDependencies: { vitest: 'catalog:bundlers' },
      },
    },
  });

  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /declares no `vitest` -- it is declared in testing/);
});

void test('the default catalog is rejected -- this workspace declares none', () => {
  const { root } = createFixture({
    manifests: {
      'packages/app/package.json': {
        name: '@stylexswc/app',
        devDependencies: { vitest: 'catalog:' },
      },
    },
  });

  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /references the default catalog/);
});

/**
 * A literal range in each of the excluded families, all of them the kind this
 * check flags anywhere else. A check that flags a fixture gets disabled by the
 * first person it inconveniences, and a fixture that stops resolving as a real
 * user project stops testing what users actually have.
 */
const EXEMPT = {
  'crates/stylex-path-resolver/fixtures/app/package.json': '^5.0.0',
  'crates/stylex-transform/src/shared/structures/tests/fixtures/app/package.json': '^5.1.0',
  'crates/stylex-transform/tests/__virtual__/app/package.json': '^4.0.0',
  'crates/stylex-transform/tests/fixture/app/package.json': '^4.1.0',
  'packages/postcss-plugin/__tests__/__auto_discovery_fixtures__/app/package.json': '^3.0.0',
  'crates/stylex-rs-compiler/npm/darwin-arm64/package.json': '^2.0.0',
  'packages/plugin/node_modules/vendored/package.json': '^1.0.0',
};

void test('fixture, virtual, generated and build-output manifests are exempt', () => {
  const manifests = Object.fromEntries(
    Object.entries(EXEMPT).map(([file, range]) => [
      file,
      { name: path.basename(path.dirname(file)), dependencies: { webpack: range } },
    ])
  );

  // Generated by Next.js, gitignored, and this is its entire content.
  manifests['apps/next-example/.next/package.json'] = { type: 'commonjs' };

  const { root } = createFixture({ manifests });
  const result = check(root);

  assert.equal(result.status, 0, result.stderr);
});

/**
 * The exemption above is only meaningful if the same manifests would otherwise
 * fail. Without this, a typo in the fixture paths would leave the test passing
 * for the wrong reason -- nothing to exempt, so nothing to flag.
 */
void test('each exempt family would fail the check if it were in scope', () => {
  for (const [file, range] of Object.entries(EXEMPT)) {
    const { root } = createFixture({
      workspaceYaml: WORKSPACE_YAML,
      manifests: { [file]: { name: 'in-scope', dependencies: { webpack: range } } },
    });

    fs.writeFileSync(
      path.join(root, '.syncpackrc'),
      `${JSON.stringify({ source: ['**/package.json'] }, null, 2)}\n`
    );

    const result = check(root);

    assert.equal(result.status, 1, `${file} was expected to fail without its exclusion`);
    assert.match(result.stderr, new RegExp(range.replaceAll(/[.^]/g, '\\$&')));
  }
});

void test('an unknown mode fails rather than checking nothing', () => {
  const { root } = createFixture();
  const result = run(root, 'everything');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /unknown mode `everything`/);
});

void test('no mode at all is a usage error, not a default', () => {
  const { root } = createFixture();
  const result = run(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /usage: catalog-integrity\.mjs <manifests\|lockfile>/);
});

void test('a workspace file with no catalogs fails loudly', () => {
  const { root } = createFixture({ workspaceYaml: 'packages:\n  - packages/*\n' });
  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /declares no `catalogs:` block/);
});

void test('a catalogs block with no catalogs in it fails loudly', () => {
  const { root } = createFixture({ workspaceYaml: 'catalogs:\n\n# nothing here\nother: 1\n' });
  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /`catalogs:` block with no catalogs/);
});

/**
 * The reader accepts one shape and rejects everything else by design. Without
 * this, a catalog written as a flow mapping would parse as zero entries and the
 * check would pass over a file it did not understand -- silence being the one
 * outcome a check must never produce.
 */
void test('a catalog entry the reader cannot parse fails loudly, naming the line', () => {
  const { root } = createFixture({
    workspaceYaml: "catalogs:\n  bundlers:\n    { webpack: '^5.0.0' }\n",
  });

  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /pnpm-workspace\.yaml:3: cannot read catalog entry/);
});

void test('the real workspace passes its own check', () => {
  const result = check(repoRoot);

  assert.equal(result.status, 0, result.stderr);
});

/**
 * `lockfile` mode -- a dependabot update dropping a catalog entry from
 * `pnpm-lock.yaml`. Why that is worth a check of its own rather than left to
 * the reinstall that would probably repair it is in `checkLockfile`'s docblock.
 */
void test('a lockfile still recording every baseline entry passes', () => {
  const { root } = createFixture();
  const result = checkLockfile(root, LOCK_YAML);

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /lockfile ok/);
});

void test('an entry dropped from the lockfile fails, naming the catalog and the package', () => {
  const { root } = createFixture({
    lockYaml: LOCK_YAML.replace(
      '    webpack:\n      specifier: ^5.109.2\n      version: 5.109.2\n',
      ''
    ),
  });

  const result = checkLockfile(root, LOCK_YAML);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /no longer records `bundlers\.webpack`/);
  // The `peers` entry for the same package is a different entry and survived.
  assert.doesNotMatch(result.stderr, /peers\.webpack/);
});

void test('a whole catalog dropped reports every entry it carried, not just one', () => {
  const { root } = createFixture({
    lockYaml: LOCK_YAML.replace(/ {2}bundlers:\n(?: {4}.+\n| {6}.+\n)+/, ''),
  });

  const result = checkLockfile(root, LOCK_YAML);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /no longer records `bundlers\.@swc\/core`/);
  assert.match(result.stderr, /no longer records `bundlers\.webpack`/);
});

void test('a lockfile with no catalogs block at all names every baseline entry', () => {
  const { root } = createFixture({ lockYaml: "lockfileVersion: '9.0'\n\nimporters:\n  .: {}\n" });
  const result = checkLockfile(root, LOCK_YAML);

  assert.equal(result.status, 1);

  for (const entry of [
    'bundlers.@swc/core',
    'bundlers.webpack',
    'peers.webpack',
    'testing.vitest',
  ]) {
    assert.ok(result.stderr.includes(`\`${entry}\``), `expected \`${entry}\` in: ${result.stderr}`);
  }
});

/**
 * A moved range is what a dependency update *is*. Only an entry that stopped
 * existing is a failure, so the check compares presence and nothing else --
 * otherwise the guard would fire on every PR it is meant to let through.
 */
void test('a specifier and version that moved is an update, not a dropped entry', () => {
  const { root } = createFixture({
    lockYaml: LOCK_YAML.replaceAll('5.109.2', '5.110.0'),
  });

  const result = checkLockfile(root, LOCK_YAML);

  assert.equal(result.status, 0, result.stderr);
});

void test('lockfile mode without a baseline fails rather than comparing nothing', () => {
  const { root } = createFixture();
  const result = run(root, 'lockfile');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /lockfile mode needs `--baseline <file>`/);
});

/**
 * A baseline pointed at the wrong file would make this check pass on any
 * lockfile at all. Silence is the one outcome it must never produce, so an
 * empty baseline is a failure rather than a vacuous success.
 */
void test('a baseline recording no catalog entries fails loudly', () => {
  const { root } = createFixture();
  const result = checkLockfile(root, "lockfileVersion: '9.0'\n");

  assert.equal(result.status, 1);
  assert.match(result.stderr, /records no catalog entries, so this check would assert nothing/);
});

void test('a baseline that does not exist fails loudly', () => {
  const { root } = createFixture();
  const result = run(root, 'lockfile', '--baseline', path.join(root, 'absent-lock.yaml'));

  assert.equal(result.status, 1);
  assert.match(result.stderr, /absent-lock\.yaml/);
});

for (const option of ['--baseline', '--current']) {
  void test(`\`${option}\` is rejected by manifests mode rather than ignored`, () => {
    const { root } = createFixture();
    const result = check(root, option, path.join(root, 'pnpm-lock.yaml'));

    assert.equal(result.status, 1);
    assert.match(result.stderr, new RegExp(`\`\\${option}\` means nothing to manifests mode`));
  });
}

/**
 * `--current` is the whole reason this check can catch anything. Compare a
 * *reinstalled* lockfile against the base commit's and the install has already
 * put back whatever the update dropped, so the guard confirms the accidental
 * repair instead of reporting the corruption -- which is why the workflow reads
 * both sides out of git and names them both.
 */
void test('`--current` compares the named lockfile, not the one on disk', () => {
  const { root, file } = createFixture();
  const dropped = path.join(root, 'as-delivered-lock.yaml');

  // On disk: whole. Named: missing an entry, the way the update left it.
  writeText(dropped, LOCK_YAML.replace(/ {4}vitest:\n(?: {6}.+\n)+/, ''));

  const result = checkLockfile(root, LOCK_YAML, dropped);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /as-delivered-lock\.yaml no longer records `testing\.vitest`/);
  // The repaired lockfile beside it is not what was asked about.
  assert.equal(checkLockfile(root, LOCK_YAML, file('pnpm-lock.yaml')).status, 0);
});

void test('the real lockfile records every entry it records', () => {
  const result = run(repoRoot, 'lockfile', '--baseline', path.join(repoRoot, 'pnpm-lock.yaml'));

  assert.equal(result.status, 0, result.stderr);
});

/**
 * The reader accepts one shape per file and rejects everything else. A scalar
 * where a mapping belongs parses cleanly and means something else entirely, so
 * without these it would be read as an entry -- and a check that misreads a
 * file it then passes is the one outcome silence must never come from.
 */
void test('a range where a catalog belongs is rejected, not read as a catalog', () => {
  const { root } = createFixture({ workspaceYaml: "catalogs:\n  webpack: '^5.0.0'\n" });
  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /`catalogs\.webpack` is a value where a mapping belongs/);
});

void test('a lockfile leaf written as a bare version is rejected, not enumerated', () => {
  const { root } = createFixture();
  const result = checkLockfile(root, 'catalogs:\n  testing:\n    vitest: 4.1.10\n');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /`catalogs\.testing\.vitest` is a value where a mapping belongs/);
});

void test('a catalog nested deeper than an entry goes is rejected', () => {
  const { root } = createFixture({
    workspaceYaml: "catalogs:\n  bundlers:\n    webpack:\n      specifier: '^5.0.0'\n",
  });

  const result = check(root);

  assert.equal(result.status, 1);
  assert.match(
    result.stderr,
    /`catalogs\.bundlers\.webpack` nests deeper than a catalog entry goes/
  );
});
