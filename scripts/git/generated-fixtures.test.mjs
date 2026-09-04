/**
 * Asserts that every committed fixture a generator writes still has something
 * that runs its `:check`, and that a generator reading another package has
 * that package declared as a Turbo input.
 *
 * The first rule failed once already: the crates' `test` scripts became skip
 * lines and both `:check` scripts were left reachable from nothing. The second
 * failed at the same time in a quieter way, because a cached `test` task
 * replays without running its `pretest`.
 *
 * Most cases run against a synthetic tree, so the suite states a rule rather
 * than a snapshot of today's manifests. One case runs against the real
 * repository, which is what makes the rule load-bearing.
 */

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import { findGateFaults } from './lib/generated-fixtures.mjs';
import { makeTemporaryDirectory, repoRoot, writeJson, writeText } from './lib/test-harness.mjs';

const GENERATOR = 'node scripts/generate-cases.mjs > src/tests/cases.rs';
const CHECK = 'node scripts/generate-cases.mjs | diff -u --strip-trailing-cr src/tests/cases.rs -';

/** The same check reading bytes, which a CRLF checkout turns red on its own. */
const CHECK_READING_BYTES = 'node scripts/generate-cases.mjs | diff -u src/tests/cases.rs -';

/**
 * A throwaway tree holding one crate per entry of `crates`.
 *
 * `source` is the generator's own file: a body naming a `../../` path stands
 * for a generator that reads another package, and `null` leaves the file out
 * altogether. `sourceFile` says where that file goes, for a generator that
 * does not sit under `scripts/`.
 *
 * @param {{scripts: object, source?: string|null, sourceFile?: string, name?: string}[]} crates
 * @param {object} [turbo] contents of the root `turbo.json`
 * @returns {{root: string, manifests: string[]}}
 */
function createTree(crates, turbo = { tasks: {} }) {
  const root = makeTemporaryDirectory('stylex-generated-fixtures-');
  const manifests = [];

  writeJson(path.join(root, 'turbo.json'), turbo);

  crates.forEach((crate, index) => {
    const directory = `crates/crate-${index}`;
    const relative = `${directory}/package.json`;

    writeJson(path.join(root, relative), {
      name: crate.name ?? `@stylexswc/crate-${index}`,
      scripts: crate.scripts,
    });

    if (crate.source !== null) {
      const file = crate.sourceFile ?? 'scripts/generate-cases.mjs';
      writeText(path.join(root, directory, file), crate.source ?? '');
    }

    manifests.push(relative);
  });

  return { root, manifests };
}

/** The manifest of a crate wired the way the gate wants. */
const WIRED = {
  scripts: {
    'generate:cases': GENERATOR,
    'generate:cases:check': CHECK,
    pretest: 'pnpm run generate:cases:check',
    test: 'echo "Skip"',
  },
};

/**
 * A generator that follows no naming convention: only the `:check` twin over
 * the same file says what it is -- the shape of the parity harvester.
 */
const HARVESTER = 'node --import tsx/esm parity/harvest.ts';

/** The manifest of a crate whose generator is the harvester shape. */
const HARVESTING = {
  scripts: {
    'parity:harvest': HARVESTER,
    'parity:harvest:check': `${HARVESTER} --check`,
    pretest: 'pnpm run parity:harvest:check',
    test: 'vitest run',
  },
  sourceFile: 'parity/harvest.ts',
  source: "fs.writeFileSync(out, body); path.resolve(here, '../../..')",
};

void test('the real repository keeps every generated fixture behind a check', () => {
  const manifests = fs
    .readdirSync(path.join(repoRoot, 'crates'))
    .map(entry => `crates/${entry}/package.json`)
    .filter(relative => fs.existsSync(path.join(repoRoot, relative)));

  assert.notEqual(manifests.length, 0, 'found no crate manifests to check');
  assert.deepEqual(findGateFaults(repoRoot, manifests), []);
});

void test('a crate with no generator has nothing to guard', () => {
  const { root, manifests } = createTree([{ scripts: { test: 'echo "Skip"' }, source: null }]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a manifest with no scripts at all is not a fault', () => {
  const { root, manifests } = createTree([{ scripts: undefined, source: null }]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a generator with no check twin is reported', () => {
  const { root, manifests } = createTree([
    { scripts: { 'generate:cases': GENERATOR, test: 'echo "Skip"' } },
  ]);

  const [fault] = findGateFaults(root, manifests);

  assert.match(fault, /`generate:cases` has no `generate:cases:check` twin/);
});

void test('a check nothing runs is reported', () => {
  const { root, manifests } = createTree([
    {
      scripts: { 'generate:cases': GENERATOR, 'generate:cases:check': CHECK, test: 'echo "Skip"' },
    },
  ]);

  const [fault] = findGateFaults(root, manifests);

  assert.match(fault, /nothing in `test` or `pretest` runs `generate:cases:check`/);
});

void test('a check the test script runs directly is reachable', () => {
  const { root, manifests } = createTree([
    {
      scripts: {
        'generate:cases': GENERATOR,
        'generate:cases:check': CHECK,
        test: 'pnpm run generate:cases:check',
      },
    },
  ]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a check reached through a chain of scripts is reachable', () => {
  const { root, manifests } = createTree([
    {
      scripts: {
        'generate:cases': GENERATOR,
        'generate:cases:check': CHECK,
        'check:all': 'pnpm run generate:cases:check',
        pretest: 'pnpm run check:all',
        test: 'echo "Skip"',
      },
    },
  ]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a script that names itself does not hang the walk', () => {
  const { root, manifests } = createTree([
    {
      scripts: {
        'generate:cases': GENERATOR,
        'generate:cases:check': CHECK,
        pretest: 'pnpm run pretest && pnpm run generate:cases:check',
        test: 'echo "Skip"',
      },
    },
  ]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a check comparing bytes rather than content is reported', () => {
  // Git for Windows checks text out as CRLF, so a diff reading bytes calls a
  // file stale while the repository holds exactly what the generator writes.
  const { root, manifests } = createTree([
    { ...WIRED, scripts: { ...WIRED.scripts, 'generate:cases:check': CHECK_READING_BYTES } },
  ]);

  const [fault] = findGateFaults(root, manifests);

  assert.match(fault, /`generate:cases:check` compares with `diff` and needs/);
});

void test('the flag is accepted wherever the command spells it', () => {
  const spellings = [
    'node scripts/generate-cases.mjs | diff --strip-trailing-cr -u src/tests/cases.rs -',
    'node scripts/generate-cases.mjs | diff -u src/tests/cases.rs - --strip-trailing-cr',
  ];

  for (const check of spellings) {
    const { root, manifests } = createTree([
      { ...WIRED, scripts: { ...WIRED.scripts, 'generate:cases:check': check } },
    ]);

    assert.deepEqual(findGateFaults(root, manifests), [], check);
  }
});

void test('a check that runs no diff at all is not asked for the flag', () => {
  // The parity harvest compares in the script it runs, not with `diff`. It
  // reads another package, so it is asked for a Turbo input and nothing else.
  const { root, manifests } = createTree([HARVESTING]);

  assert.deepEqual(
    findGateFaults(root, manifests).filter(fault => fault.includes('strip-trailing-cr')),
    []
  );
});

void test('a name that merely ends in diff is not read as the program', () => {
  const { root, manifests } = createTree([
    {
      ...WIRED,
      scripts: {
        ...WIRED.scripts,
        'generate:cases:check': 'node scripts/generate-cases.mjs --out src/cases.diff src/x.rs',
      },
    },
  ]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a diff is found wherever a command may start one', () => {
  const stages = {
    'after a pipe': 'node scripts/generate-cases.mjs | diff -u a.rs -',
    'after and': 'node scripts/generate-cases.mjs > a && diff -u a.rs b.rs',
    'after a semicolon': 'node scripts/generate-cases.mjs > a; diff -u a.rs b.rs',
    'on a new line': 'node scripts/generate-cases.mjs > a\ndiff -u a.rs b.rs',
    'inside a subshell': '(diff -u a.rs b.rs)',
    'at the head': 'diff -u a.rs b.rs',
  };

  for (const [where, check] of Object.entries(stages)) {
    const { root, manifests } = createTree([
      { ...WIRED, scripts: { ...WIRED.scripts, 'generate:cases:check': check } },
    ]);

    assert.match(
      findGateFaults(root, manifests).join('\n'),
      /needs `--strip-trailing-cr`/,
      `a diff ${where} was not read`
    );
  }
});

void test('a word merely containing diff is not read as the program', () => {
  const notDiff = {
    'the git subcommand': 'git diff --exit-code src/tests/cases.rs',
    'a file name': 'node scripts/generate-cases.mjs --out src/cases.diff src/x.rs',
    'a longer program': 'node scripts/generate-cases.mjs | diffstat -u a.rs -',
    'a bare word': 'node scripts/generate-cases.mjs | diff',
  };

  for (const [what, check] of Object.entries(notDiff)) {
    const { root, manifests } = createTree([
      { ...WIRED, scripts: { ...WIRED.scripts, 'generate:cases:check': check } },
    ]);

    assert.deepEqual(findGateFaults(root, manifests), [], what);
  }
});

void test('a diff in a later stage of the pipeline is still read', () => {
  const { root, manifests } = createTree([
    {
      ...WIRED,
      scripts: {
        ...WIRED.scripts,
        'generate:cases:check': 'node scripts/generate-cases.mjs | rustfmt | diff -u a.rs -',
      },
    },
  ]);

  const [fault] = findGateFaults(root, manifests);

  assert.match(fault, /needs `--strip-trailing-cr`/);
});

void test('a generator reading another package needs a Turbo input of its own', () => {
  const { root, manifests } = createTree([
    { ...WIRED, source: "path.resolve(here, '../../other/corpus')" },
  ]);

  const [fault] = findGateFaults(root, manifests);

  assert.match(fault, /needs an input outside the package/);
});

void test('the same generator passes once the input is declared', () => {
  const { root, manifests } = createTree(
    [{ ...WIRED, source: "path.resolve(here, '../../other/corpus')" }],
    {
      tasks: { '@stylexswc/crate-0#test': { inputs: ['$TURBO_ROOT$/crates/other/corpus/*.json'] } },
    }
  );

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a generator that stays inside its package needs no extra input', () => {
  const { root, manifests } = createTree([{ ...WIRED, source: "path.join(here, 'data.json')" }]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a missing generator file is not read as reaching outside', () => {
  const { root, manifests } = createTree([{ ...WIRED, source: null }]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a malformed manifest names the file it could not read', () => {
  const root = makeTemporaryDirectory('stylex-generated-fixtures-');
  writeText(path.join(root, 'crates/broken/package.json'), '{ not json');

  assert.throws(() => findGateFaults(root, ['crates/broken/package.json']), /crates\/broken/);
});

void test('every fault in a large tree is reported, not just the first', () => {
  const size = 500;
  const crates = Array.from({ length: size }, (_, index) =>
    index % 2 === 0 ? WIRED : { scripts: { 'generate:cases': GENERATOR }, source: null }
  );
  const { root, manifests } = createTree(crates);

  const faults = findGateFaults(root, manifests);

  assert.equal(faults.length, size / 2);
  assert.ok(faults.every(fault => fault.includes('has no `generate:cases:check` twin')));
});

void test('a generator named by no convention still needs its outside input', () => {
  const { root, manifests } = createTree([HARVESTING]);

  const [fault] = findGateFaults(root, manifests);

  assert.match(fault, /`parity:harvest` reads one/);
});

void test('the harvester passes once the input is declared', () => {
  const { root, manifests } = createTree([HARVESTING], {
    tasks: { '@stylexswc/crate-0#test': { inputs: ['$TURBO_ROOT$/crates/**/*.rs'] } },
  });

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a script that writes a report is not read as a generator', () => {
  const { root, manifests } = createTree([
    {
      scripts: { bench: 'node --import tsx/esm benchmark/bench.ts', test: 'vitest run' },
      sourceFile: 'benchmark/bench.ts',
      source: "fs.writeFileSync(out, body); path.resolve(here, '../../..')",
    },
  ]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});

void test('a generator started by another runner is read the same way', () => {
  const { root, manifests } = createTree([
    {
      ...HARVESTING,
      scripts: {
        'parity:harvest': 'tsx parity/harvest.ts',
        'parity:harvest:check': 'tsx parity/harvest.ts --check',
        pretest: 'pnpm run parity:harvest:check',
        test: 'vitest run',
      },
    },
  ]);

  const [fault] = findGateFaults(root, manifests);

  assert.match(fault, /`parity:harvest` reads one/);
});

void test('the value behind an option is not read as the script', () => {
  const { root, manifests } = createTree([
    {
      ...HARVESTING,
      scripts: {
        'parity:harvest': 'node --import ./register.mjs parity/harvest.ts',
        'parity:harvest:check': 'node --import ./register.mjs parity/harvest.ts --check',
        pretest: 'pnpm run parity:harvest:check',
        test: 'vitest run',
      },
    },
  ]);

  const [fault] = findGateFaults(root, manifests);

  assert.match(fault, /`parity:harvest` reads one/);
});

void test('a formatting script with a check twin is not a generator', () => {
  const { root, manifests } = createTree([
    {
      scripts: {
        format: 'run-p format:rs format:toml',
        'format:check': 'run-p format:rs:check format:toml:check',
        test: 'echo "Skip"',
      },
      source: null,
    },
  ]);

  assert.deepEqual(findGateFaults(root, manifests), []);
});
