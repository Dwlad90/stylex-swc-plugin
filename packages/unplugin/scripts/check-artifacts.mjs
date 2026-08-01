// Validates the published contract of every entry in the `exports` map.
//
// The shared `scripty` artifact check only asserts that `dist/index.js` exists,
// which cannot catch a subpath that is missing, unloadable, or that silently
// changes its CommonJS shape. Publint and ATTW check the package statically;
// this executes it.

import { existsSync } from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const packageDir = path.resolve(import.meta.dirname, '..');
const require = createRequire(path.join(packageDir, 'package.json'));
const pkg = require('./package.json');

/** Subpaths whose CommonJS entry must stay directly callable by `require()`. */
const CALLABLE_SUBPATHS = new Set([
  './astro',
  './rspack',
  './vite',
  './webpack',
  './rollup',
  './esbuild',
  './nuxt',
  './farm',
]);

const failures = [];
const fail = message => failures.push(message);

const subpaths = Object.keys(pkg.exports);
let fileCount = 0;

for (const subpath of subpaths) {
  const conditions = pkg.exports[subpath];

  for (const condition of ['import', 'require']) {
    for (const key of ['types', 'default']) {
      const target = conditions[condition]?.[key];
      if (!target) {
        fail(`${subpath}: missing exports["${subpath}"].${condition}.${key}`);
        continue;
      }
      fileCount++;
      if (!existsSync(path.join(packageDir, target))) {
        fail(`${subpath}: ${condition}.${key} points at missing file ${target}`);
      }
    }
  }
}

// CommonJS: loadable, and still shaped the way consumers depend on.
for (const subpath of subpaths) {
  const target = pkg.exports[subpath]?.require?.default;
  if (!target || !existsSync(path.join(packageDir, target))) continue;

  let loaded;
  try {
    loaded = require(target);
  } catch (error) {
    fail(`${subpath}: require() threw ${error.message.split('\n')[0]}`);
    continue;
  }

  if (CALLABLE_SUBPATHS.has(subpath) && typeof loaded !== 'function') {
    fail(`${subpath}: require() must return a callable plugin factory, got ${typeof loaded}`);
  }

  if (subpath === '.') {
    for (const name of ['unplugin', 'unpluginFactory', 'default']) {
      if (!(name in loaded)) fail(`.: require() lost the "${name}" named export`);
    }
  }
}

// ESM: every subpath resolves and, where applicable, exposes a callable default.
for (const subpath of subpaths) {
  const target = pkg.exports[subpath]?.import?.default;
  if (!target || !existsSync(path.join(packageDir, target))) continue;

  try {
    const loaded = await import(pathToFileURL(path.join(packageDir, target)).href);
    if (CALLABLE_SUBPATHS.has(subpath) && typeof loaded.default !== 'function') {
      fail(`${subpath}: import() default must be callable, got ${typeof loaded.default}`);
    }
  } catch (error) {
    fail(`${subpath}: import() threw ${error.message.split('\n')[0]}`);
  }
}

if (failures.length > 0) {
  console.error(`Artifact check failed with ${failures.length} problem(s):`);
  for (const failure of failures) console.error(`  - ${failure}`);
  process.exit(1);
}

console.log(
  `Artifact check passed: ${subpaths.length} subpaths, ${fileCount} files, ` +
    `CommonJS and ESM both load with the expected shapes.`
);
