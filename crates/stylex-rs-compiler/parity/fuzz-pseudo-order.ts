/**
 * Condition-key ordering parity, over random pairs rather than a corpus.
 *
 * `pseudo_comparator` reproduces the ordering upstream reaches through
 * `String.prototype.localeCompare`, and the sorted key path is hashed into the
 * class name — so a disagreement here costs a class name, which is the one thing
 * that cannot be caught by reading either compiler's output alone. The curated
 * corpus pins the shapes someone thought of. This asks the same question over
 * pairs nobody chose.
 *
 * It checks two things, and they are not the same check:
 *
 * 1. **Against the reference compiler.** Both compilers are handed the same two
 *    keys nested, and the class names and rule text have to match. That is the
 *    contract, and it is the only oracle that covers the whole path from key to
 *    hash rather than the comparator alone.
 * 2. **Against the ordering algorithm.** The order this compiler chose, read out
 *    of the emitted selector, has to match `Intl.Collator` at the **root**
 *    locale. This is what tells a failure in (1) apart: a comparator that moved,
 *    against a reference whose own answer moved under it.
 *
 * And it measures the remainder `pre_rule.rs` names. Upstream calls
 * `localeCompare` *bare*, so its answer follows the build machine's default
 * locale rather than root — a Swedish or Danish machine sorts `ö` after `z`. The
 * run reports how many pairs the default locale and root disagree on, so the
 * remainder is a number in a report rather than a sentence someone has to
 * remember.
 *
 * Keys are attribute selectors, and that is not for convenience. Every
 * pseudo-class and pseudo-element name CSS defines is ASCII, so an attribute
 * selector is the only way a non-ASCII key reaches the comparator at all —
 * `crates/stylex-css/docs/adr/0001-root-collation-orders-a-non-ascii-condition-key.md`
 * argues that at length. It also keeps the
 * generated key clear of the selector syntax a random character would otherwise
 * spell, which would make the subject about parsing rather than about order.
 *
 * Usage:
 *   pnpm run --filter=@stylexswc/rs-compiler build       # reads dist/
 *   pnpm run --filter=@stylexswc/rs-compiler fuzz:pseudo-order
 *   pnpm run --filter=@stylexswc/rs-compiler fuzz:pseudo-order -- --pairs 5000
 */

import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import { createComparer } from './lib/compare.js';
import { subjectBlock } from './lib/compilers.js';
import type { CompilerOutcome } from './lib/types.js';

const parityDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(parityDir, '..');

const { values: cliOptions } = parseArgs({
  args: process.argv.slice(2).filter(argument => argument !== '--'),
  options: {
    pairs: { type: 'string', default: '1000' },
    show: { type: 'string', default: '20' },
    help: { type: 'boolean', short: 'h', default: false },
  },
});

if (cliOptions.help) {
  console.log(`
${chalk.bold('Condition-key ordering parity over random pairs')}

Options:
      --pairs <n>   how many key pairs to generate (default 1000)
      --show <n>    how many disagreeing pairs to print (default 20)
  -h, --help        show this help
`);
  process.exit(0);
}

/**
 * The characters a generated key is built from: printable ASCII, Latin-1
 * Supplement, Latin Extended-A, and the combining diacritics.
 *
 * The range that ADR settled on, and the reason it is a floor rather
 * than a claim: a quoted attribute value can carry anything at all, so no
 * generated range bounds what an author can write. What this range does bound is
 * the part where a wrong answer is *likely* — the accented letters a Western
 * European author writes, and the combining marks that decompose into them.
 *
 * `]` and `\\` are excluded: one closes the attribute selector early and the
 * other escapes the character after it, and either would make the subject about
 * how a selector parses rather than about how two keys order.
 */
const ALPHABET: readonly string[] = (() => {
  const characters: string[] = [];
  const ranges: [number, number][] = [
    [0x20, 0x7e],
    [0xa0, 0xff],
    [0x100, 0x17f],
    [0x300, 0x36f],
  ];
  for (const [low, high] of ranges) {
    for (let point = low; point <= high; point += 1) {
      const character = String.fromCodePoint(point);
      if (character === ']' || character === '\\') continue;
      characters.push(character);
    }
  }
  return characters;
})();

/**
 * A 64-bit xorshift, so a failing run is reproducible from the seed printed
 * above it rather than from the clock. A property check nobody can re-run is a
 * report rather than a check.
 */
function randoms(seed: bigint): () => number {
  let state = seed;
  const mask = (1n << 64n) - 1n;
  return () => {
    state = (state ^ (state << 13n)) & mask;
    state = state ^ (state >> 7n);
    state = (state ^ (state << 17n)) & mask;
    return Number(state & 0xffff_ffffn) / 0x1_0000_0000;
  };
}

/** The order the reference algorithm puts two keys in, at the root locale. */
const rootCollator = new Intl.Collator('und');

/**
 * What `localeCompare` with no argument reaches: the build machine's default
 * locale, which is what upstream actually calls. Compared against root so the
 * documented remainder is measured rather than asserted.
 */
const defaultLocaleCollator = new Intl.Collator(undefined);

interface Disagreement {
  left: string;
  right: string;
  kind: 'class names' | 'chosen order';
  here: string;
  expected: string;
}

/**
 * The selector a rule carries, which is the sorted keys spelled out.
 *
 * Cut at the **last** `{` rather than the first. A generated key can contain a
 * brace — `[data-{÷]` is a key this alphabet produces — and cutting at the first
 * one truncated the selector mid-key, which read as eight ordering divergences
 * that were this function's fault rather than the comparator's. The brace that
 * opens the rule body is the last one, since these subjects declare `color:red`
 * and nothing in the value can carry another.
 */
function selectorOf(outcome: CompilerOutcome): string {
  if (outcome.status === 'error') return `refused: ${outcome.sentence}`;
  const rule = outcome.rules.find(text => text !== '') ?? '';
  const open = rule.lastIndexOf('{');
  return open === -1 ? rule : rule.slice(0, open);
}

async function run(): Promise<void> {
  const pairs = Number.parseInt(cliOptions.pairs ?? '1000', 10);
  const show = Number.parseInt(cliOptions.show ?? '20', 10);
  if (!Number.isFinite(pairs) || pairs < 1) {
    console.error(chalk.red(`--pairs must be a positive integer, got ${String(cliOptions.pairs)}`));
    process.exit(1);
  }

  const comparer = await createComparer({ packageDir, enableFontSizePxToRem: false });
  const seed = 0x2545_f491_4f6c_dd1dn;
  const next = randoms(seed);

  console.log(
    `${chalk.bold('Subjects')}\n${subjectBlock(comparer.versions, [
      ['pairs', String(pairs)],
      ['seed', `0x${seed.toString(16)}`],
      ['alphabet', `${ALPHABET.length} characters, ASCII through U+036F`],
      ['reference ordering', `Intl.Collator('und'), root`],
    ])}\n`
  );

  const key = (): string => {
    let built = '[data-';
    const length = 1 + Math.floor(next() * 4);
    for (let index = 0; index < length; index += 1) {
      built += ALPHABET[Math.floor(next() * ALPHABET.length)];
    }
    return `${built}]`;
  };

  const disagreements: Disagreement[] = [];
  let compared = 0;
  let refused = 0;
  let localeShifted = 0;

  for (let index = 0; index < pairs; index += 1) {
    const left = key();
    const right = key();
    // Two keys that collate equal have no order to check, and a repeated key is
    // refused before the sort in both compilers.
    if (left === right || rootCollator.compare(left, right) === 0) continue;

    // Measured on the pair itself rather than on the alphabet, since it is the
    // pair that decides a class name.
    if (
      Math.sign(defaultLocaleCollator.compare(left, right)) !==
      Math.sign(rootCollator.compare(left, right))
    ) {
      localeShifted += 1;
    }

    const source =
      "import * as stylex from '@stylexjs/stylex';\n" +
      `export const styles = stylex.create({\n  w: { color: { ${JSON.stringify(left)}: { ${JSON.stringify(right)}: 'red' } } },\n});\n`;

    const entry = comparer.compare({
      kind: 'module',
      set: 'reported',
      id: `pseudo-order-${index}`,
      label: `${left} beside ${right}`,
      source,
      origin: 'fuzz-pseudo-order.ts',
    });

    // A pair either compiler refuses says nothing about order: neither reached
    // the sort. Counted so a run that refused everything cannot read as a pass.
    if (entry.rust.status === 'error' || entry.babel.status === 'error') {
      refused += 1;
      continue;
    }

    compared += 1;

    if (entry.verdict !== 'identical') {
      disagreements.push({
        left,
        right,
        kind: 'class names',
        here: selectorOf(entry.rust),
        expected: selectorOf(entry.babel),
      });
      continue;
    }

    // The second check: the order this compiler chose against the algorithm
    // upstream reaches. Read off the selector, which is the sorted keys spelled
    // out, so this asks the comparator rather than the hash.
    const selector = selectorOf(entry.rust);
    const leadsHere = selector.indexOf(left) < selector.indexOf(right);
    const leadsInRoot = rootCollator.compare(left, right) < 0;
    if (leadsHere !== leadsInRoot) {
      disagreements.push({
        left,
        right,
        kind: 'chosen order',
        here: selector,
        expected: leadsInRoot ? `${left} before ${right}` : `${right} before ${left}`,
      });
    }
  }

  console.log(
    `${chalk.bold('Results')}\n` +
      `  compared               ${compared}\n` +
      `  skipped (refused)      ${refused}   ${chalk.gray('(neither reached the sort)')}\n` +
      `  ${chalk.bold('disagreements')}          ${disagreements.length}\n` +
      `  default locale differs ${localeShifted}   ${chalk.gray("(pairs where a bare localeCompare would not answer root's order)")}`
  );

  if (compared === 0) {
    console.error(
      chalk.red('\nNo pair reached the sort, so this run measured nothing. That is a failure.')
    );
    process.exitCode = 1;
    return;
  }

  if (localeShifted > 0) {
    console.log(
      chalk.gray(
        '\nThe default-locale count is the remainder `pre_rule.rs` names, not a defect: upstream\n' +
          'calls `localeCompare` bare, so its answer follows the build machine. A non-zero count\n' +
          'here means this machine tailors some of the characters in play, and the two compilers\n' +
          'would name different classes for those pairs however correct this comparator is.'
      )
    );
  }

  if (disagreements.length > 0) {
    console.log(chalk.red.bold(`\nDisagreeing pairs`));
    for (const found of disagreements.slice(0, show)) {
      console.log(
        `  ${chalk.bold(found.kind)}  ${JSON.stringify(found.left)} beside ${JSON.stringify(found.right)}`
      );
      console.log(`    here      ${found.here}`);
      console.log(`    expected  ${found.expected}`);
    }
    console.log(
      chalk.gray(
        `\nA \`class names\` row is a divergence from the reference compiler; a \`chosen order\` row is\n` +
          'a divergence from root collation itself. If only the first kind appears, read the\n' +
          "default-locale count above: the reference's own answer may have moved rather than this\n" +
          'comparator. `pseudo_comparator` in `crates/stylex-css/src/utils/pre_rule.rs` is the code.'
      )
    );
    process.exitCode = 1;
  }

  console.log('');
}

run().catch((error: unknown) => {
  console.error(chalk.red('Pseudo-order parity failed:'), error);
  process.exit(1);
});
