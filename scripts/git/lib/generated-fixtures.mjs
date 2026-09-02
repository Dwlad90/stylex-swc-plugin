/**
 * Rules for the crates that commit a Rust test file a Node script writes.
 *
 * Each such crate has a `generate:<name>` script that writes the file and a
 * `generate:<name>:check` twin that diffs a fresh run against what is
 * committed. The pair is only a gate while something calls the twin, and the
 * repository has lost that caller twice: once when every crate's `test` script
 * became a skip line, and once when Turbo replayed a cached pass because the
 * task did not hash the file the generator reads.
 *
 * These two failures look identical from outside -- a green run over a stale
 * fixture -- so the rules below assert the wiring rather than the fixture.
 *
 * Not a `*.test.mjs` file, so `pnpm test:scripts` does not try to run it as a
 * suite.
 */

import fs from 'node:fs';
import path from 'node:path';

/** Marks a path that climbs out of the package holding it. */
const CLIMBS_OUT = '../../';

/** A generator whose output a `:check` twin must guard. */
const GENERATOR = /^generate:(?!.*:check$).+$/;

/**
 * Reads one manifest, or explains which file could not be read.
 *
 * @param {string} file absolute path to a `package.json`
 * @returns {{name: string, scripts: Record<string, string>}}
 */
function readManifest(file) {
  let parsed;

  try {
    parsed = JSON.parse(fs.readFileSync(file, 'utf8'));
  } catch (cause) {
    throw new Error(`cannot read ${file}: ${cause.message}`, { cause });
  }

  return { name: parsed.name ?? path.basename(path.dirname(file)), scripts: parsed.scripts ?? {} };
}

/**
 * The scripts a package reaches when it runs `entry`, `entry` included.
 *
 * A script reaches another by naming it in a `pnpm run <name>` call, and a
 * chain can be any length, so this walks until it finds nothing new. The seen
 * set also stops a script that names itself from looping.
 *
 * @param {Record<string, string>} scripts the manifest's `scripts` object
 * @param {string[]} entries script names to start from
 * @returns {Set<string>}
 */
function reachableScripts(scripts, entries) {
  const seen = new Set();
  const pending = entries.filter(name => name in scripts);

  while (pending.length > 0) {
    const name = pending.pop();

    if (seen.has(name)) continue;
    seen.add(name);

    for (const match of scripts[name].matchAll(/pnpm (?:run )?([\w:@./-]+)/g)) {
      if (match[1] in scripts && !seen.has(match[1])) pending.push(match[1]);
    }
  }

  return seen;
}

/**
 * True when the generator behind `command` names a path outside its package.
 *
 * The generator for the value-parser cases reads the parity corpus of another
 * crate, so a Turbo task that hashes only its own package replays a cached
 * pass over a stale fixture. Nothing in the manifest says so -- the fact is in
 * the generator's source -- so this reads the file the command runs.
 *
 * @param {string} packageDirectory absolute path to the package
 * @param {string} command the `generate:<name>` script body
 * @returns {boolean}
 */
function readsAnotherPackage(packageDirectory, command) {
  const source = command.match(/(?:node )([\w:@./-]+\.mjs)/)?.[1];

  if (source === undefined) return false;

  const file = path.join(packageDirectory, source);

  return fs.existsSync(file) && fs.readFileSync(file, 'utf8').includes(CLIMBS_OUT);
}

/**
 * The Turbo inputs declared for `<package>#test` at the repository root.
 *
 * The crates share one `turbo.json` through a symlink, so a task that needs an
 * input of its own is declared at the root instead.
 *
 * @param {object} rootTurbo the parsed root `turbo.json`
 * @param {string} packageName the npm name of the package
 * @returns {string[]}
 */
function declaredInputs(rootTurbo, packageName) {
  return rootTurbo?.tasks?.[`${packageName}#test`]?.inputs ?? [];
}

/**
 * Every way the generated-fixture gate is broken in the tree at `root`.
 *
 * @param {string} root repository root
 * @param {string[]} manifestFiles package manifests to check, relative to root
 * @returns {string[]} one sentence per fault, empty when the gate holds
 */
export function findGateFaults(root, manifestFiles) {
  const turboFile = path.join(root, 'turbo.json');
  const rootTurbo = fs.existsSync(turboFile) ? JSON.parse(fs.readFileSync(turboFile, 'utf8')) : {};
  const faults = [];

  for (const relative of manifestFiles) {
    const directory = path.dirname(path.join(root, relative));
    const { name, scripts } = readManifest(path.join(root, relative));
    const reached = reachableScripts(scripts, ['test', 'pretest']);

    for (const generator of Object.keys(scripts).filter(script => GENERATOR.test(script))) {
      const check = `${generator}:check`;

      if (!(check in scripts)) {
        faults.push(`${relative}: \`${generator}\` has no \`${check}\` twin`);
        continue;
      }

      if (!reached.has(check)) {
        faults.push(`${relative}: nothing in \`test\` or \`pretest\` runs \`${check}\``);
      }

      if (
        readsAnotherPackage(directory, scripts[generator]) &&
        !declaredInputs(rootTurbo, name).some(input => input.includes('$TURBO_ROOT$'))
      ) {
        faults.push(
          `turbo.json: \`${name}#test\` needs an input outside the package, ` +
            `because \`${generator}\` reads one`
        );
      }
    }
  }

  return faults;
}
