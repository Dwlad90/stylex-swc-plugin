/**
 * Drives the real `bump-version.mjs` against a throwaway workspace shaped like
 * this repository: a Cargo workspace whose crates inherit the version, a
 * `.syncpackrc` with the same exclusions, source and generated manifests, a
 * README carrying two badges of which only one is the release badge, and a
 * `pnpm-workspace.yaml` with an `internal` catalog.
 *
 * The assertions are on the four version locations and on what must *not*
 * move, because both defects this script was written to fix -- a Cargo pattern
 * reaching further than intended, and a substitution that matched nothing --
 * are invisible to a test that only checks the happy path.
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

const CURRENT = '0.18.3';

const SYNCPACK = {
  source: ['**/package.json', '!**/node_modules/**', '!fixtures/**', '!crates/*/npm/**'],
};

/**
 * A `[workspace.dependencies]` entry written as a sub-table, so its `version`
 * sits at column zero exactly like the workspace package version. This is the
 * shape the previous shell bumper would have silently rewritten to the release
 * version.
 */
const CARGO = `[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.package]
edition = "2024"
version = "${CURRENT}"

[workspace.dependencies.swc_core]
version = "56.1.0"
features = ["common"]
`;

const README = `# Title

![Release status](https://img.shields.io/github/check-runs/Dwlad90/stylex-swc-plugin/${CURRENT}?label=Release%20status)

<!-- stylex-compatibility:start -->[![StyleX compatibility](https://img.shields.io/badge/StyleX%20compatibility-v0.19.0-blue)](https://stylexjs.com/blog)<!-- stylex-compatibility:end -->

Clone it from <https://github.com/Dwlad90/stylex-swc-plugin.git>.
`;

const WORKSPACE_YAML = `packages:
  - packages/*

# Comments here are load-bearing and a YAML round-trip would drop them.
catalogs:
  internal:
    '@stylexswc/rs-compiler': ${CURRENT}
    '@stylexswc/unplugin': '${CURRENT}'
  runtime:
    picomatch: 0.18.3
`;

function manifest(name, extra = {}) {
  return { name, version: CURRENT, ...extra };
}

/**
 * Named for what it builds rather than `createWorkspace`, which the shared
 * harness exports with a different shape -- a stub directory, not a repository.
 *
 * @param {{workspaceYaml?: string, cargo?: string, readme?: string}} [overrides]
 */
function createFixture(overrides = {}) {
  const root = makeTemporaryDirectory('stylex-bump-version-');
  const file = relative => path.join(root, relative);

  writeText(file('Cargo.toml'), overrides.cargo ?? CARGO);
  writeText(
    file('crates/alpha/Cargo.toml'),
    '[package]\nname = "alpha"\nversion.workspace = true\n'
  );
  writeText(file('README.md'), overrides.readme ?? README);
  writeText(file('pnpm-workspace.yaml'), overrides.workspaceYaml ?? WORKSPACE_YAML);
  writeJson(file('.syncpackrc'), SYNCPACK);

  writeJson(
    file('package.json'),
    manifest('root', { devDependencies: { '@stylexswc/typescript-config': CURRENT } })
  );

  writeJson(
    file('packages/plugin/package.json'),
    manifest('@stylexswc/plugin', {
      dependencies: {
        '@stylexswc/rs-compiler': CURRENT,
        '@stylexswc/catalogued': 'catalog:internal',
        '@stylexswc/linked': 'link:../linked',
        '@stylexswc/local': 'file:../local',
        '@stylexswc/sibling': 'workspace:*',
        picomatch: '^4.0.4',
      },
      peerDependencies: { '@stylexswc/rs-compiler': CURRENT },
    })
  );

  // Generated, so out of `.syncpackrc`'s scope -- but published under its own
  // name, so the bump owns it.
  writeJson(file('crates/stylex-rs-compiler/npm/darwin-arm64/package.json'), {
    name: '@stylexswc/rs-compiler-darwin-arm64',
    version: CURRENT,
  });

  // A fixture resolved as if it were a real user project: standalone, and not
  // the bumper's to touch.
  writeJson(
    file('fixtures/application/package.json'),
    manifest('application', { dependencies: { '@stylexswc/rs-compiler': CURRENT } })
  );

  return { root, file };
}

function bump(root, version, ...extra) {
  return spawnSync(
    'node',
    [path.join(repoRoot, 'scripts/git/bump-version.mjs'), version, '--root', root, ...extra],
    {
      encoding: 'utf8',
      env: hermeticEnvironment(),
    }
  );
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, 'utf8'));
}

function readText(file) {
  return fs.readFileSync(file, 'utf8');
}

void test('a bump moves all four version locations together', () => {
  const { root, file } = createFixture();
  const result = bump(root, '0.19.0');

  assert.equal(result.status, 0, result.stderr);

  assert.match(
    readText(file('Cargo.toml')),
    /\[workspace\.package]\nedition = "2024"\nversion = "0\.19\.0"/
  );
  assert.equal(readJson(file('package.json')).version, '0.19.0');
  assert.equal(readJson(file('packages/plugin/package.json')).version, '0.19.0');
  assert.match(readText(file('README.md')), /stylex-swc-plugin\/0\.19\.0\?label=/);
  assert.match(readText(file('pnpm-workspace.yaml')), /'@stylexswc\/rs-compiler': 0\.19\.0/);
  assert.match(readText(file('pnpm-workspace.yaml')), /'@stylexswc\/unplugin': '0\.19\.0'/);
});

void test('internal dependency ranges move and every other specifier is left alone', () => {
  const { root, file } = createFixture();

  assert.equal(bump(root, '0.19.0').status, 0);

  const plugin = readJson(file('packages/plugin/package.json'));

  assert.deepEqual(plugin.dependencies, {
    '@stylexswc/rs-compiler': '0.19.0',
    '@stylexswc/catalogued': 'catalog:internal',
    '@stylexswc/linked': 'link:../linked',
    '@stylexswc/local': 'file:../local',
    '@stylexswc/sibling': 'workspace:*',
    picomatch: '^4.0.4',
  });
  assert.equal(plugin.peerDependencies['@stylexswc/rs-compiler'], '0.19.0');
  assert.equal(
    readJson(file('package.json')).devDependencies['@stylexswc/typescript-config'],
    '0.19.0'
  );
});

void test('the published platform manifest moves and the fixture manifest does not', () => {
  const { root, file } = createFixture();

  assert.equal(bump(root, '0.19.0').status, 0);

  assert.equal(
    readJson(file('crates/stylex-rs-compiler/npm/darwin-arm64/package.json')).version,
    '0.19.0'
  );

  const fixture = readJson(file('fixtures/application/package.json'));

  assert.equal(fixture.version, CURRENT);
  assert.equal(fixture.dependencies['@stylexswc/rs-compiler'], CURRENT);
});

void test('a shared-dependency version at column zero is not the workspace version', () => {
  const { root, file } = createFixture();

  assert.equal(bump(root, '0.19.0').status, 0);
  assert.match(
    readText(file('Cargo.toml')),
    /\[workspace\.dependencies\.swc_core]\nversion = "56\.1\.0"/
  );
});

void test('only the release badge moves, not the compatibility badge next to it', () => {
  const { root, file } = createFixture();

  assert.equal(bump(root, '0.19.0').status, 0);

  const readme = readText(file('README.md'));

  assert.match(readme, /StyleX%20compatibility-v0\.19\.0-blue/);
  assert.match(readme, /stylex-swc-plugin\.git/);
});

void test('a prerelease moves every location except the release badge', () => {
  const { root, file } = createFixture();

  assert.equal(bump(root, '0.19.0-rc.1').status, 0);

  assert.equal(readJson(file('package.json')).version, '0.19.0-rc.1');
  assert.match(
    readText(file('Cargo.toml')),
    /\[workspace\.package]\nedition = "2024"\nversion = "0\.19\.0-rc\.1"/
  );
  assert.match(readText(file('pnpm-workspace.yaml')), /'@stylexswc\/rs-compiler': 0\.19\.0-rc\.1/);
  assert.match(readText(file('README.md')), /stylex-swc-plugin\/0\.18\.3\?label=/);
});

void test('a bump that changes nothing is an error', () => {
  const { root, file } = createFixture();
  const before = readText(file('package.json'));
  const result = bump(root, CURRENT);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /nothing to do/);
  assert.equal(readText(file('package.json')), before);
});

void test('a workspace with no catalogs yet bumps the other three locations', () => {
  const { root, file } = createFixture({ workspaceYaml: 'packages:\n  - packages/*\n' });

  assert.equal(bump(root, '0.19.0').status, 0);
  assert.equal(readJson(file('package.json')).version, '0.19.0');
});

void test('an empty internal catalog is an error, and nothing is written', () => {
  const { root, file } = createFixture({
    workspaceYaml: 'catalogs:\n  internal:\n  runtime:\n    picomatch: ^4.0.4\n',
  });
  const result = bump(root, '0.19.0');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /`internal` catalog with no entries/);
  assert.equal(readJson(file('package.json')).version, CURRENT);
});

void test('a crate carrying a literal version is refused before anything is written', () => {
  const { root, file } = createFixture();

  writeText(file('crates/alpha/Cargo.toml'), `[package]\nname = "alpha"\nversion = "${CURRENT}"\n`);

  const result = bump(root, '0.19.0');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /crates\/alpha\/Cargo\.toml declares a literal `version`/);
  assert.equal(readJson(file('package.json')).version, CURRENT);
});

void test('a Cargo workspace with no package version is refused', () => {
  const { root } = createFixture({ cargo: '[workspace]\nmembers = ["crates/*"]\n' });
  const result = bump(root, '0.19.0');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /expected exactly one/);
});

void test('catalogs without an `internal` one are an error, not a skipped location', () => {
  const { root } = createFixture({
    workspaceYaml: 'catalogs:\n  runtime:\n    picomatch: ^4.0.4\n',
  });
  const result = bump(root, '0.19.0');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /declares catalogs but no `internal` one/);
});

void test('a README that has lost the release badge is an error', () => {
  const { root } = createFixture({ readme: '# Title\n\nNo badges here.\n' });
  const result = bump(root, '0.19.0');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /no longer contains the release badge/);
});

void test('an unknown option is refused', () => {
  const { root } = createFixture();
  const result = bump(root, '0.19.0', '--dry-run');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /unknown option `--dry-run`/);
});

void test('a version that is not a version is refused', () => {
  const { root } = createFixture();
  const result = bump(root, 'latest');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /is not a version/);
});
