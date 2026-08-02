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

/**
 * Loaders reject with whatever the module threw, which is not necessarily an
 * `Error`. Reading `.message` off a thrown string or `null` would turn a real
 * finding into a crash inside the checker.
 *
 * @param {unknown} error
 * @returns {string}
 */
const firstLineOf = error => {
  const message = error instanceof Error ? error.message : String(error);
  return message.split('\n', 1)[0];
};

const hasExportsMap = typeof pkg.exports === 'object' && pkg.exports !== null;
if (!hasExportsMap) {
  fail('package.json has no `exports` map to validate.');
}

/**
 * Resolves one condition of a subpath to its `{ types, default }` targets.
 *
 * Node accepts both the nested form and the string shorthand
 * (`"import": "./dist/x.js"`, or an entire subpath as a bare string, as with
 * `"./package.json": "./package.json"`). Treating the shorthand as malformed
 * would report a valid package as broken.
 *
 * @param {unknown} entry the value of `exports[subpath]`
 * @param {'import' | 'require'} condition
 * @returns {{ types?: string, default?: string, typesRequired: boolean } | null}
 */
const resolveCondition = (entry, condition) => {
  if (typeof entry === 'string') {
    // A bare subpath applies to every condition and carries no declarations.
    return { default: entry, typesRequired: false };
  }
  if (typeof entry !== 'object' || entry === null) return null;

  const value = entry[condition];
  if (value === undefined) return null;
  if (typeof value === 'string') return { default: value, typesRequired: true };
  if (typeof value !== 'object' || value === null) return null;

  return { types: value.types, default: value.default, typesRequired: true };
};

const subpaths = hasExportsMap ? Object.keys(pkg.exports) : [];
let fileCount = 0;

for (const subpath of subpaths) {
  const entry = pkg.exports[subpath];

  // A bare subpath string applies to every condition and carries no
  // declarations, so it is a single target. Running it through the loop below
  // checked the same file twice and counted it twice in the summary.
  if (typeof entry === 'string') {
    fileCount++;
    if (!existsSync(path.join(packageDir, entry))) {
      fail(`${subpath}: points at missing file ${entry}`);
    }
    continue;
  }

  for (const condition of ['import', 'require']) {
    const resolved = resolveCondition(entry, condition);
    if (!resolved) {
      fail(`${subpath}: missing exports["${subpath}"].${condition}`);
      continue;
    }

    const keys = resolved.typesRequired ? ['types', 'default'] : ['default'];
    for (const key of keys) {
      const target = resolved[key];
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

/** Extensions the loader probes below can meaningfully evaluate. */
const JS_EXTENSIONS = new Set(['.js', '.cjs', '.mjs']);

/**
 * @param {string} subpath
 * @param {'import' | 'require'} condition
 * @returns {string | null} an existing on-disk JavaScript target, or null when
 *   there is nothing to probe. Non-JS targets (an exported `./package.json`,
 *   say) are skipped rather than loaded: `import()` on JSON requires an import
 *   attribute, so probing one would report a spurious failure. Their existence
 *   is already covered above.
 */
const loadableTarget = (subpath, condition) => {
  const target = resolveCondition(pkg.exports[subpath], condition)?.default;
  if (!target || !JS_EXTENSIONS.has(path.extname(target))) return null;

  const absolute = path.join(packageDir, target);
  return existsSync(absolute) ? absolute : null;
};

// CommonJS: loadable, and still shaped the way consumers depend on. `require`
// is synchronous by nature, so this stays a plain loop.
for (const subpath of subpaths) {
  const target = loadableTarget(subpath, 'require');
  if (!target) continue;

  let loaded;
  try {
    loaded = require(target);
  } catch (error) {
    fail(`${subpath}: require() threw ${firstLineOf(error)}`);
    continue;
  }

  if (CALLABLE_SUBPATHS.has(subpath) && typeof loaded !== 'function') {
    fail(`${subpath}: require() must return a callable plugin factory, got ${typeof loaded}`);
  }

  // `in` throws a TypeError on a primitive, so a module that regressed to
  // `module.exports = <string>` would crash the checker instead of being
  // reported by it. Same reason `./nuxt`'s property read is guarded below.
  const isIndexable = typeof loaded === 'object' ? loaded !== null : typeof loaded === 'function';

  if (subpath === '.') {
    if (!isIndexable) {
      fail(`.: require() must return an object or function, got ${typeof loaded}`);
    } else {
      for (const name of ['unplugin', 'unpluginFactory', 'default']) {
        if (!(name in loaded)) fail(`.: require() lost the "${name}" named export`);
      }
    }
  }

  // `./nuxt` is the one entry whose declaration exports a `default` while
  // `cjsDefault` writes `module.exports =` directly, so `tsdown.config.ts`
  // appends `module.exports.default = module.exports` to reconcile them.
  // Without this assertion nothing notices if that footer stops being applied,
  // and `import mod from '@stylexswc/unplugin/nuxt'` silently yields undefined
  // for CommonJS consumers.
  if (subpath === './nuxt' && (!isIndexable || loaded.default !== loaded)) {
    fail('./nuxt: require() must expose `default` pointing back at the module itself');
  }
}

// ESM: every subpath resolves and, where applicable, exposes a callable
// default. The probes are independent, so they run concurrently rather than
// paying one module-evaluation round trip per subpath in series; results are
// collected in `subpaths` order so failure output stays deterministic.
const esmResults = await Promise.all(
  subpaths.map(async subpath => {
    const target = loadableTarget(subpath, 'import');
    if (!target) return null;

    try {
      const loaded = await import(pathToFileURL(target).href);
      if (CALLABLE_SUBPATHS.has(subpath) && typeof loaded.default !== 'function') {
        return `${subpath}: import() default must be callable, got ${typeof loaded.default}`;
      }
      return null;
    } catch (error) {
      return `${subpath}: import() threw ${firstLineOf(error)}`;
    }
  })
);

for (const result of esmResults) {
  if (result) fail(result);
}

if (failures.length > 0) {
  console.error(`Artifact check failed with ${failures.length} problem(s):`);
  for (const failure of failures) console.error(`  - ${failure}`);
  // `process.exitCode`, not `process.exit`. Under CI the streams are pipes and
  // therefore written asynchronously; `process.exit` would tear the process
  // down mid-flush and the failure list — the entire point of this script —
  // could reach the log truncated or not at all.
  process.exitCode = 1;
} else {
  console.log(
    `Artifact check passed: ${subpaths.length} subpaths, ${fileCount} files, ` +
      `CommonJS and ESM both load with the expected shapes.`
  );
}
