#!/usr/bin/env node
/**
 * Fails when the package `pnpm publish` would ship cannot be installed and
 * loaded.
 *
 * Usage: node check-packed-package.mjs <package directory>
 *          [--platform-package <directory>]
 *
 * The package is packed as the publish packs it: for a NAPI-RS package,
 * `napi prepublish` first writes the platform packages into
 * `optionalDependencies`, as `prepublishOnly` does at publish time (the
 * package.json is restored afterwards). A consumer project then installs the
 * tarball with pnpm, no hoisting and no peer install, the host's platform
 * package coming from its own tarball since the publish has not happened,
 * and a child Node requires and imports every entry point; `tsc` reads the
 * types. A `files` entry that matches nothing in the tarball fails too: it is
 * the reason a build output stays behind, named before the load reports the
 * missing module.
 *
 * The test jobs run on the whole build artifact and never see the tarball the
 * publish job assembles, which is how 0.20.0-rc.1 shipped a `dist/index.js`
 * whose first `require` had no target (#1352).
 */

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import { createRequire } from 'node:module';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { fail, failWithErrors } from './lib/ci.mjs';

/**
 * Expands `{a,b}` sets, innermost first, so each alternative is a plain glob.
 * A nested set repeats the outer alternatives, so the result is deduplicated.
 */
export function expandBraces(pattern) {
  const match = /\{([^{}]*)\}/.exec(pattern);
  if (match === null) return [pattern];
  const before = pattern.slice(0, match.index);
  const after = pattern.slice(match.index + match[0].length);
  const expanded = match[1]
    .split(',')
    .flatMap(alternative => expandBraces(`${before}${alternative}${after}`));
  return [...new Set(expanded)];
}

/**
 * The regular expression one npm `files` glob stands for: `**` crosses
 * directories, `*` and `?` stay inside one.
 */
export function filesPatternToRegExp(pattern) {
  let source = '';
  for (let i = 0; i < pattern.length; i++) {
    const char = pattern[i];
    if (char === '*') {
      if (pattern[i + 1] === '*') {
        i++;
        if (pattern[i + 1] === '/') {
          i++;
          source += '(?:.*/)?';
        } else {
          source += '.*';
        }
      } else {
        source += '[^/]*';
      }
    } else if (char === '?') {
      source += '[^/]';
    } else {
      source += char.replace(/[.+^${}()|[\]\\]/g, '\\$&');
    }
  }
  return new RegExp(`^${source}$`);
}

/**
 * The `files` entries that match no path in the tarball. An entry names a
 * file, a directory (everything beneath it ships), or a glob with `*`, `?`
 * and `{a,b}` sets. A negation removes files and has nothing to match.
 */
export function unmatchedFilesEntries(files, entries) {
  return files.filter(entry => {
    if (entry.startsWith('!')) return false;
    const clean = entry.replace(/^\.\//, '').replace(/\/$/, '');
    const patterns = expandBraces(clean).map(alternative => filesPatternToRegExp(alternative));
    return !entries.some(
      packed =>
        packed === clean ||
        packed.startsWith(`${clean}/`) ||
        patterns.some(pattern => pattern.test(packed))
    );
  });
}

/**
 * The subpaths a consumer can require or import: `.` and the literal keys
 * of the `exports` map (a key with a `*` stands for many files; it is not
 * loaded).
 */
export function exportSubpaths(manifest) {
  const { exports: exportMap } = manifest;
  if (exportMap === undefined || typeof exportMap === 'string' || Array.isArray(exportMap)) {
    return ['.'];
  }
  const keys = Object.keys(exportMap).filter(key => key.startsWith('.'));
  if (keys.length === 0) return ['.'];
  return keys.filter(key => !key.includes('*'));
}

/** `glibc` or `musl`, read off the running Node. */
function hostLibc() {
  const report =
    typeof process.report?.getReport === 'function' ? process.report.getReport() : null;
  return report?.header?.glibcVersionRuntime ? 'glibc' : 'musl';
}

/**
 * The platform package of `npmDir` whose `os`, `cpu` and `libc` name the
 * running host, as npm picks the one optional dependency it installs.
 */
export function hostPlatformPackage(
  npmDir,
  host = { platform: process.platform, arch: process.arch, libc: hostLibc() }
) {
  const matches = [];
  for (const name of fs.readdirSync(npmDir)) {
    const dir = path.join(npmDir, name);
    const manifestPath = path.join(dir, 'package.json');
    if (!fs.existsSync(manifestPath)) continue;
    const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
    const platforms = manifest.os ?? [];
    const cpus = manifest.cpu ?? [];
    const libcs = manifest.libc ?? [];
    if (!platforms.includes(host.platform) || !cpus.includes(host.arch)) continue;
    if (libcs.length > 0 && !libcs.includes(host.libc)) continue;
    matches.push(dir);
  }
  if (matches.length !== 1) {
    fail(
      `${npmDir}: expected one platform package for ${host.platform}-${host.arch}-${host.libc}, found ${matches.length}` +
        (matches.length > 0 ? ` (${matches.join(', ')})` : '')
    );
  }
  return matches[0];
}

/** Runs a command and returns its output, failing the check when it does not exit 0. */
function run(command, args, options) {
  const result = spawnSync(command, args, { encoding: 'utf8', ...options });
  if (result.error) fail(`${command} ${args.join(' ')}: ${result.error.message}`);
  if (result.status !== 0) {
    fail(
      `${command} ${args.join(' ')} exited ${result.signal ?? result.status}:\n${result.stderr}${result.stdout}`
    );
  }
  return result.stdout;
}

/** Runs a command and returns what went wrong, or null when it exited 0. */
function tryRun(command, args, options) {
  const result = spawnSync(command, args, { encoding: 'utf8', ...options });
  if (result.error) return result.error.message;
  if (result.status === 0) return null;
  return `exited ${result.signal ?? result.status}:\n${result.stderr}${result.stdout}`.trimEnd();
}

/** Packs the package into a fresh directory and returns the tarball's path. */
export function packInto(packageDir, destination) {
  fs.mkdirSync(destination, { recursive: true });
  run('pnpm', ['pack', '--pack-destination', destination], { cwd: packageDir });
  const tarballs = fs.readdirSync(destination).filter(name => name.endsWith('.tgz'));
  if (tarballs.length !== 1) {
    fail(`expected one tarball in ${destination}, found ${tarballs.length}`);
  }
  return path.join(destination, tarballs[0]);
}

/** Extracts the tarball and returns its paths without the `package/` root. */
export function extractTarball(tarball, destination) {
  fs.mkdirSync(destination, { recursive: true });
  run('tar', ['-xzf', tarball, '-C', destination]);
  const root = path.join(destination, 'package');
  const entries = [];
  const walk = dir => {
    for (const dirent of fs.readdirSync(dir, { withFileTypes: true })) {
      const file = path.join(dir, dirent.name);
      if (dirent.isDirectory()) walk(file);
      else entries.push(path.relative(root, file).split(path.sep).join('/'));
    }
  };
  walk(root);
  return { root, entries: entries.toSorted((a, b) => a.localeCompare(b, 'en')) };
}

const readJson = file => JSON.parse(fs.readFileSync(file, 'utf8'));

/**
 * Packs the package as the publish does: a NAPI-RS package first gets its
 * platform packages written into `optionalDependencies` by `napi prepublish`,
 * which `prepublishOnly` runs at publish time. The package.json is restored
 * afterwards. Returns the tarball and the manifest it carries.
 */
export function packAsPublished(packageDir, destination) {
  const manifestPath = path.join(packageDir, 'package.json');
  const manifest = readJson(manifestPath);
  if (manifest.napi === undefined) {
    return { tarball: packInto(packageDir, destination), manifest };
  }
  const original = fs.readFileSync(manifestPath, 'utf8');
  try {
    run('pnpm', ['exec', 'napi', 'prepublish', '--skip-optional-publish', '--no-gh-release'], {
      cwd: packageDir,
    });
    return { tarball: packInto(packageDir, destination), manifest: readJson(manifestPath) };
  } finally {
    fs.writeFileSync(manifestPath, original);
  }
}

/**
 * The settings of the consumer project: no hoisting, so a module the package
 * does not declare is not found; no peer install, so the load needs nothing
 * the package does not ship or depend on; the platform package from its
 * tarball, since the publish that puts its version on the registry has not
 * happened.
 */
function consumerSettings(overrides) {
  const lines = ['autoInstallPeers: false', 'hoist: false'];
  const names = Object.keys(overrides);
  if (names.length > 0) {
    lines.push('overrides:');
    for (const name of names) {
      lines.push(`  ${JSON.stringify(name)}: ${JSON.stringify(overrides[name])}`);
    }
  }
  return `${lines.join('\n')}\n`;
}

const LOAD_CJS = `const [name, subpaths] = [process.argv[2], JSON.parse(process.argv[3])];
for (const subpath of subpaths) require(subpath === '.' ? name : name + subpath.slice(1));
`;

const LOAD_MJS = `const [name, subpaths] = [process.argv[2], JSON.parse(process.argv[3])];
for (const subpath of subpaths) await import(subpath === '.' ? name : name + subpath.slice(1));
`;

/** The declaration file beside `main`, or the one `types` names, as a tarball path. */
function typesEntry(manifest) {
  const declared = manifest.types ?? manifest.typings;
  if (typeof declared === 'string') return declared.replace(/^\.\//, '');
  const main = typeof manifest.main === 'string' ? manifest.main : 'index.js';
  return main.replace(/^\.\//, '').replace(/\.([cm]?)js$/, '.d.$1ts');
}

/**
 * Installs the tarball into a fresh consumer project, requires and imports
 * each entry point in a child Node, and reads the types with `tsc`. Returns
 * the errors found.
 */
export function loadPackedPackage({ tarball, manifest, overrides, typeRoots, workDir }) {
  const consumer = path.join(workDir, 'consumer');
  fs.mkdirSync(consumer, { recursive: true });
  fs.writeFileSync(
    path.join(consumer, 'package.json'),
    `${JSON.stringify({ name: 'packed-package-consumer', private: true }, null, 2)}\n`
  );
  fs.writeFileSync(path.join(consumer, 'pnpm-workspace.yaml'), consumerSettings(overrides));
  const installed = tryRun('pnpm', ['add', tarball], { cwd: consumer });
  if (installed !== null) return [`pnpm add ${path.basename(tarball)} ${installed}`];

  const errors = [];
  const subpaths = JSON.stringify(exportSubpaths(manifest));
  for (const [file, source, verb] of [
    ['load.cjs', LOAD_CJS, 'require'],
    ['load.mjs', LOAD_MJS, 'import'],
  ]) {
    fs.writeFileSync(path.join(consumer, file), source);
    const failure = tryRun(process.execPath, [file, manifest.name, subpaths], { cwd: consumer });
    if (failure !== null) errors.push(`${verb}('${manifest.name}') ${failure}`);
  }

  const types = typesEntry(manifest);
  if (fs.existsSync(path.join(consumer, 'node_modules', manifest.name, types))) {
    const probe = path.join(consumer, 'types-probe.ts');
    fs.writeFileSync(
      probe,
      `import * as api from ${JSON.stringify(manifest.name)};\nexport type Api = typeof api;\n`
    );
    // `lib/tsc.js` is not an exported subpath; the package root is reached through its manifest.
    const tsc = path.join(
      path.dirname(createRequire(import.meta.url).resolve('typescript/package.json')),
      'lib',
      'tsc.js'
    );
    const args = [
      tsc,
      '--noEmit',
      '--strict',
      '--module',
      'nodenext',
      '--moduleResolution',
      'nodenext',
      '--target',
      'es2022',
      '--skipLibCheck',
      'false',
    ];
    if (typeRoots !== undefined && fs.existsSync(path.join(typeRoots, 'node'))) {
      args.push('--types', 'node', '--typeRoots', typeRoots);
    }
    const failure = tryRun(process.execPath, [...args, probe], { cwd: consumer });
    if (failure !== null) errors.push(`tsc over ${types} ${failure}`);
  }
  return errors;
}

function isMainModule() {
  return (
    process.argv[1] !== undefined &&
    path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
  );
}

function parseArgs(argv) {
  const options = { packageDir: undefined, platformPackage: undefined };
  for (let i = 0; i < argv.length; i++) {
    if (argv[i] === '--platform-package') {
      options.platformPackage = argv[++i];
    } else if (options.packageDir === undefined) {
      options.packageDir = argv[i];
    } else {
      fail(`unexpected argument ${argv[i]}`);
    }
  }
  if (options.packageDir === undefined || options.platformPackage === '') {
    fail('Usage: check-packed-package.mjs <package directory> [--platform-package <directory>]');
  }
  return options;
}

if (isMainModule()) {
  const { packageDir, platformPackage } = parseArgs(process.argv.slice(2));
  const sourceManifest = readJson(path.join(packageDir, 'package.json'));
  const workDir = fs.mkdtempSync(path.join(os.tmpdir(), 'check-packed-package-'));
  try {
    const { tarball, manifest } = packAsPublished(packageDir, path.join(workDir, 'package'));
    const { entries } = extractTarball(tarball, path.join(workDir, 'extracted'));
    const errors = [];
    for (const entry of unmatchedFilesEntries(manifest.files ?? [], entries)) {
      errors.push(`package.json "files" entry ${entry} matches nothing in the tarball`);
    }

    const overrides = {};
    if (sourceManifest.napi !== undefined) {
      const platformDir =
        platformPackage ??
        hostPlatformPackage(path.join(packageDir, sourceManifest.napi.npmDir ?? 'npm'));
      const platform = readJson(path.join(platformDir, 'package.json'));
      const declared = manifest.optionalDependencies?.[platform.name];
      if (declared !== platform.version) {
        errors.push(
          `optionalDependencies name ${platform.name}@${declared ?? '(nothing)'}, not ${platform.version}: a consumer would not install the binding`
        );
      }
      overrides[platform.name] = `file:${packInto(platformDir, path.join(workDir, 'platform'))}`;
    }
    if (errors.length > 0) {
      failWithErrors(`${path.basename(tarball)} cannot be shipped:`, errors);
    }

    const loadErrors = loadPackedPackage({
      tarball,
      manifest,
      overrides,
      typeRoots: path.resolve(packageDir, 'node_modules', '@types'),
      workDir,
    });
    if (loadErrors.length > 0) {
      failWithErrors(`${path.basename(tarball)} cannot be loaded as published:`, loadErrors);
    }
    console.log(
      `ok  ${path.basename(tarball)}: ${entries.length} files; require, import and tsc load it as published`
    );
  } finally {
    fs.rmSync(workDir, { recursive: true, force: true });
  }
}
