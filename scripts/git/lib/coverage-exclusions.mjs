/**
 * Reads the five lists of crates excluded from coverage, and says where they
 * disagree.
 *
 * The lists are hand-kept copies of one decision, in two spellings. Three of
 * them hold Cargo package names -- the root `test:coverage:workspace` script,
 * `EXCLUDED_CRATES` in `scripts/coverage-missing.sh`, because both hand the
 * names to cargo, and the rows under "Excluded from Coverage" in
 * `guidelines/STRUCTURE.md`, which is where a reader is sent to find out why a
 * crate is off the gate. The other two hold crate directory names -- the `case`
 * in `scripts/packages/test/coverage.sh`, which reads the name off `PWD`, and
 * `EXCLUDED` in the suite that asserts that `case`. The two spellings are not
 * the same string: `stylex-rs-compiler` is the crate `stylex_compiler_rs`.
 *
 * The docs count as a list because they drift the same way: this module's own
 * commit edited those rows by hand, and nothing would have said so.
 *
 * Nothing compared them until now. Taking one row off three of the four left
 * every check green and failed in the pre-push hook, which is the shape this
 * module exists to turn into a named fault.
 *
 * Not a `*.test.mjs` file, so `pnpm test:scripts` does not run it as a suite.
 */

import fs from 'node:fs';
import path from 'node:path';

/** Where each list lives, and how it spells a crate. */
export const SOURCES = [
  {
    name: 'package.json',
    file: 'package.json',
    spelling: 'package',
    read: readWorkspaceScript,
  },
  {
    name: 'scripts/coverage-missing.sh',
    file: 'scripts/coverage-missing.sh',
    spelling: 'package',
    read: readShellArray,
  },
  {
    name: 'scripts/packages/test/coverage.sh',
    file: 'scripts/packages/test/coverage.sh',
    spelling: 'directory',
    read: readShellCase,
  },
  {
    name: 'scripts/git/crate-coverage-runner.test.mjs',
    file: 'scripts/git/crate-coverage-runner.test.mjs',
    spelling: 'directory',
    read: readJsArray,
  },
  {
    name: 'guidelines/STRUCTURE.md',
    file: 'guidelines/STRUCTURE.md',
    spelling: 'package',
    read: readGuidelineRows,
  },
];

/**
 * Raised when a list cannot be found at all. A parser that answered an empty
 * list instead would make every other list agree with it, which is the one
 * answer this module must never give.
 */
class UnreadableList extends Error {}

function firstMatch(contents, pattern, description) {
  const match = contents.match(pattern);

  if (match === null) {
    throw new UnreadableList(`has no ${description} to read`);
  }

  return match[1];
}

/** Every `--exclude <name>` the root coverage script passes to cargo. */
function readWorkspaceScript(contents) {
  let manifest;

  // A manifest that will not parse is a list that cannot be read, not a crash:
  // `readExclusionLists` rethrows anything that is not an `UnreadableList`, so
  // without this the suite dies with a parser stack instead of a named fault.
  try {
    manifest = JSON.parse(contents);
  } catch {
    throw new UnreadableList('is not readable JSON');
  }

  const script = manifest.scripts?.['test:coverage:workspace'];

  if (typeof script !== 'string') {
    throw new UnreadableList('has no `test:coverage:workspace` script to read');
  }

  // `--exclude name` and `--exclude=name` are the same flag to cargo, so a row
  // written the second way has to count as a row here too.
  const names = [...script.matchAll(/--exclude[\s=]+(\S+)/g)].map(match => match[1]);

  if (names.length === 0) {
    throw new UnreadableList('has a `test:coverage:workspace` that excludes nothing');
  }

  return names;
}

/**
 * The `EXCLUDED_CRATES=( … )` array, without the comment on each row.
 *
 * Split on whitespace rather than on newlines, because the shell does: two
 * names on one row are two rows to bash, and reading them as one produced a
 * fault naming a crate spelled `stylex_logs stylex_evaluator`. The body is
 * taken up to the first `)` that stands at the end of a line, so a `(` inside a
 * row comment cannot end the array early.
 */
function readShellArray(contents) {
  const body = firstMatch(
    contents,
    /EXCLUDED_CRATES=\(([\s\S]*?)\n?\)\s*$/m,
    '`EXCLUDED_CRATES` array'
  );

  return body
    .split('\n')
    .flatMap(line => line.replace(/#.*$/, '').trim().split(/\s+/))
    .map(name => name.replace(/^["']|["']$/g, ''))
    .filter(name => name.length > 0);
}

/**
 * Every alternation of the `case` that exits before measuring.
 *
 * Every arm, not the first one: reading only the first made a name in a second
 * arm invisible, so a crate genuinely off the per-crate gate could sit on the
 * other three lists and the comparison would still report agreement. That is
 * the one answer this module must never give, and a wrap of the arm past the
 * 100-column line width is the likeliest way to reach it.
 */
function readShellCase(contents) {
  const body = firstMatch(
    contents,
    /case\s+"\$\{?crate_name\}?"\s+in\s*\n([\s\S]*?)\nesac/,
    '`case` over the crate name'
  );

  // A row continued with a trailing `\` is one row to the shell, so it is
  // joined back before the arms are read -- otherwise the half in front of the
  // break is dropped, which is the same invisible row by another spelling.
  const names = [...body.replace(/\\\n\s*/g, '').matchAll(/^([^\n()]+?)\)\s*$/gm)]
    .flatMap(match => match[1].split('|'))
    .map(name => name.trim())
    .filter(name => name.length > 0 && name !== '*');

  if (names.length === 0) {
    throw new UnreadableList('has a `case` over the crate name that names no crate');
  }

  return names;
}

/** The `const EXCLUDED = [ … ]` array the runner suite asserts that `case` with. */
function readJsArray(contents) {
  const body = firstMatch(contents, /const EXCLUDED = \[([^\]]*)\]/, '`EXCLUDED` array');

  // Either quote style, and never through a comment: a `//` note holding an
  // apostrophe used to be read back as a crate name.
  return [...body.replace(/\/\/[^\n]*/g, '').matchAll(/['"]([^'"]+)['"]/g)].map(match => match[1]);
}

/**
 * The `` - `name` -- reason `` rows under "Excluded from Coverage", across both
 * the permanent and the temporary heading.
 *
 * Anchored at the start of a line, so the crate names spelled inline in the
 * prose above the rows -- which are examples, not decisions -- are not read as
 * rows.
 */
function readGuidelineRows(contents) {
  const section = firstMatch(
    contents,
    /### Excluded from Coverage\n([\s\S]*?)\n## /,
    '`Excluded from Coverage` section'
  );

  const names = [...section.matchAll(/^- `([A-Za-z0-9_]+)`/gm)].map(match => match[1]);

  if (names.length === 0) {
    throw new UnreadableList('has an `Excluded from Coverage` section that names no crate');
  }

  return names;
}

/**
 * Every crate directory under `crates/`, paired with the Cargo package name it
 * declares. Read rather than derived, because the two spellings differ by more
 * than the hyphens.
 *
 * @param {string} root
 * @returns {Map<string, string>} directory name -> package name
 */
export function readCratePackageNames(root) {
  const crates = path.join(root, 'crates');

  if (!fs.existsSync(crates)) {
    return new Map();
  }

  const names = new Map();

  for (const directory of fs.readdirSync(crates)) {
    const manifest = path.join(crates, directory, 'Cargo.toml');

    if (!fs.existsSync(manifest)) {
      continue;
    }

    const declared = fs.readFileSync(manifest, 'utf8').match(/^name\s*=\s*"([^"]+)"/m);

    if (declared !== null) {
      names.set(directory, declared[1]);
    }
  }

  return names;
}

/**
 * The five lists as they are written, each with the faults reading it raised.
 *
 * @param {string} root
 * @returns {{name: string, spelling: string, names: string[], fault: string|null}[]}
 */
export function readExclusionLists(root) {
  return SOURCES.map(source => {
    const file = path.join(root, source.file);

    if (!fs.existsSync(file)) {
      return { name: source.name, spelling: source.spelling, names: [], fault: 'is missing' };
    }

    try {
      return {
        name: source.name,
        spelling: source.spelling,
        names: source.read(fs.readFileSync(file, 'utf8')),
        fault: null,
      };
    } catch (error) {
      if (error instanceof UnreadableList) {
        return { name: source.name, spelling: source.spelling, names: [], fault: error.message };
      }

      throw error;
    }
  });
}

/**
 * Everything wrong with the five lists, one sentence each: a list that cannot
 * be read, a name that no crate answers to, and any pair that disagrees once
 * both are read as package names.
 *
 * @param {string} root
 * @returns {string[]}
 */
export function findExclusionFaults(root) {
  const lists = readExclusionLists(root);
  const packageNames = readCratePackageNames(root);
  const declared = new Set(packageNames.values());
  const faults = [];

  const asPackageNames = [];

  for (const list of lists) {
    if (list.fault !== null) {
      faults.push(`${list.name} ${list.fault}`);
      continue;
    }

    const resolved = new Set();

    for (const name of list.names) {
      const packageName = list.spelling === 'directory' ? packageNames.get(name) : name;

      if (packageName === undefined) {
        faults.push(`${list.name} excludes \`${name}\`, which is no crate directory`);
        continue;
      }

      if (!declared.has(packageName)) {
        faults.push(`${list.name} excludes \`${name}\`, which is no crate in this workspace`);
        continue;
      }

      resolved.add(packageName);
    }

    asPackageNames.push({ name: list.name, packages: resolved });
  }

  // Compared against the first readable list rather than pairwise, so a row
  // taken off four of five reads as four faults naming the one that kept it,
  // instead of twenty naming each other.
  const [reference, ...rest] = asPackageNames;

  if (reference === undefined) {
    return faults;
  }

  for (const list of rest) {
    for (const missing of difference(reference.packages, list.packages)) {
      faults.push(`${list.name} does not exclude \`${missing}\`, which ${reference.name} does`);
    }

    for (const extra of difference(list.packages, reference.packages)) {
      faults.push(`${list.name} excludes \`${extra}\`, which ${reference.name} does not`);
    }
  }

  return faults;
}

/** The names `left` holds and `right` does not, ordered so a fault list is stable. */
function difference(left, right) {
  return [...left]
    .filter(name => !right.has(name))
    .toSorted((one, other) => one.localeCompare(other));
}
