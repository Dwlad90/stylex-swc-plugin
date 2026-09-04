/**
 * Rules for the crates that commit a file a script writes.
 *
 * Each such crate has a script that writes the file and a `<name>:check` twin
 * that runs the same script to compare a fresh result against what is
 * committed. The pair is only a gate while something calls the twin, and the
 * repository has lost that caller twice: once when every crate's `test` script
 * became a skip line, and once when Turbo replayed a cached pass because the
 * task did not hash the files the generator reads.
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

/** The suffix of the script that verifies a generator. */
const CHECK_SUFFIX = ':check';

/** One of the two signals of a generator: the naming convention. */
const GENERATOR_NAME = /^generate:.+/;

/** Extension of a script file the repository runs. */
const SCRIPT_FILE = /\.(?:mjs|cjs|js|ts)$/;

/** A command-line option, with or without its value attached. */
const OPTION = /^-/;

/**
 * A `diff` call at the head of a command or of one of its pipeline stages.
 *
 * A stage starts at the beginning of the command, after a pipe, after `&&` or
 * `||`, after a semicolon, on a new line, or inside a subshell. Anchored to one
 * of those, so `git diff` and a file whose name ends in `diff` are not read as
 * the program.
 */
const DIFF_CALL = /(?:^|[|&;(\n])\s*diff\s/;

/**
 * The option that makes `diff` read a checkout as its content rather than as
 * its bytes.
 *
 * Git for Windows checks text out as CRLF by default, so a generator writing LF
 * and a `diff` reading bytes disagree about a file the repository holds exactly
 * as the generator writes it. Two of the three checks here reported every line
 * as changed for that reason. `.gitattributes` asks for LF in every working
 * tree, and this is the second half: a tree that arrived before the rule, or a
 * file the rule does not cover, must not be able to turn the gate red.
 */
const STRIP_TRAILING_CR = '--strip-trailing-cr';

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
 * The script file a command runs, as the command spells it.
 *
 * The first argument that looks like a script file wins. Options and the value
 * behind an option are stepped over, because `--import ./register.mjs` names a
 * file that is not the script. Which runner starts the file is not read: the
 * repository runs one through `node`, `tsx` and `--import tsx/esm`, and a gate
 * that knew only about `node` would go quiet when the command changed.
 *
 * @param {string} command a script body
 * @returns {string|undefined} the path as written, or nothing when the command
 *   runs no script file
 */
function scriptFile(command) {
  const words = command.split(/\s+/);

  return words.find(
    (word, index) => SCRIPT_FILE.test(word) && !OPTION.test(word) && !OPTION.test(words[index - 1])
  );
}

/**
 * The scripts of one package that write a committed file.
 *
 * What makes a script a generator is not its name. Two signals say so: the
 * `generate:*` convention, which is reported even with no twin so that a
 * half-wired pair cannot hide; and a `:check` twin that runs the same script
 * file, which is the pair the parity harvester forms under a name of its own.
 * A script that writes a report rather than a fixture -- a benchmark, a parity
 * run -- has no twin, and so is neither.
 *
 * @param {Record<string, string>} scripts the manifest's `scripts` object
 * @returns {string[]}
 */
function generatorNames(scripts) {
  return Object.keys(scripts).filter(name => {
    if (name.endsWith(CHECK_SUFFIX)) return false;
    if (GENERATOR_NAME.test(name)) return true;

    const source = scriptFile(scripts[name]);

    return source !== undefined && scriptFile(scripts[`${name}${CHECK_SUFFIX}`] ?? '') === source;
  });
}

/**
 * True when the generator behind `command` names a path outside its package.
 *
 * The harvester reads the Rust test sources of every crate, so a Turbo task
 * that hashes only its own package replays a cached pass over a stale corpus.
 * Nothing in the manifest says so -- the fact is in the generator's source --
 * so this reads the file the command runs.
 *
 * @param {string} packageDirectory absolute path to the package
 * @param {string} command the generator's script body
 * @returns {boolean}
 */
function readsAnotherPackage(packageDirectory, command) {
  const source = scriptFile(command);

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

    for (const generator of generatorNames(scripts)) {
      const check = `${generator}${CHECK_SUFFIX}`;

      if (!(check in scripts)) {
        faults.push(`${relative}: \`${generator}\` has no \`${check}\` twin`);
        continue;
      }

      if (!reached.has(check)) {
        faults.push(`${relative}: nothing in \`test\` or \`pretest\` runs \`${check}\``);
      }

      if (DIFF_CALL.test(scripts[check]) && !scripts[check].includes(STRIP_TRAILING_CR)) {
        faults.push(
          `${relative}: \`${check}\` compares with \`diff\` and needs ` +
            `\`${STRIP_TRAILING_CR}\`, or a CRLF checkout reports every line as changed`
        );
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
