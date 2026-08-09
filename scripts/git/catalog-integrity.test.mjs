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
 * @param {{workspaceYaml?: string, manifests?: Record<string, object>}} [overrides]
 */
function createFixture(overrides = {}) {
  const root = makeTemporaryDirectory('stylex-catalog-integrity-');
  const file = relative => path.join(root, relative);

  writeText(file('pnpm-workspace.yaml'), overrides.workspaceYaml ?? WORKSPACE_YAML);
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

function check(root, ...extra) {
  return spawnSync('node', [SCRIPT, 'manifests', '--root', root, ...extra], {
    encoding: 'utf8',
    env: hermeticEnvironment(),
  });
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
  const result = check(root, 'lockfile');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /unknown mode `lockfile`/);
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
