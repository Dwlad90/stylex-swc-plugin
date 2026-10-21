/**
 * Stylesheet parity harness: `@stylexswc/rs-compiler`'s `processStylexRules`
 * vs `@stylexjs/babel-plugin`'s.
 *
 * `processStylexRules` assembles every rule a build emitted into the one
 * stylesheet the bundler plugins write, so its output is a contract between
 * the two implementations the way a class name is: a sheet assembled by one
 * for markup compiled by the other has to be the same bytes, or a rule lands
 * in the wrong layer or the wrong order and nothing errors.
 *
 * The harness compiles the whole parity corpus through this compiler (the
 * declarations and the module entries, under the options every subject is
 * compiled with), adds a synthetic rule per at-rule chain of the width-sort-key
 * matrix `stylex-stylesheet` is tested against, and hands the collected rules
 * to both implementations under each configuration a consumer can pick. A
 * sheet that differs by one byte is a divergence, and a divergence fails the
 * run: unlike a value's spelling there is no "both are valid CSS" reading of
 * two sheets in two orders.
 *
 * It lives beside `parity-values.ts` for the same reasons that one does: it
 * needs the built `dist/` and a Node toolchain, so it runs in CI's `checks`
 * matrix rather than in `cargo test` or a hook.
 *
 * Usage:
 *   pnpm parity:sheets                        # every configuration, human report
 *   pnpm parity:sheets --json parity/results/sheets.json
 */

import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import type { ProcessStylexRulesConfig, StyleXRule } from '../dist/index.js';
import {
  baseStyleXOptions,
  loadBabelPlugin,
  loadRustCompiler,
  messageOf,
  resolveVersions,
  subjectBlock,
} from './lib/compilers.js';
import { loadCorpus } from './lib/corpus.js';
import { WIDTH_SORT_KEY_MATRIX, stylesheetFixturesDir } from './lib/fixtures.js';
import { isRecord } from './lib/guards.js';
import { writeJsonReport } from './lib/harness-cli.js';
import { moduleFor } from './lib/subject.js';

const parityDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(parityDir, '..');
const matrixFixture = path.join(stylesheetFixturesDir(packageDir), WIDTH_SORT_KEY_MATRIX);

/** Every configuration the reference's signature admits, each in its own shape. */
const CONFIGURATIONS: Record<string, boolean | ProcessStylexRulesConfig | null> = {
  'layers (boolean)': true,
  'layers, prefixed': { useLayers: { prefix: 'stylex' } },
  'layers, before and after': {
    useLayers: { before: ['reset', 'typography'], after: ['theme'] },
  },
  'no layers, specificity polyfill': { useLayers: false },
  'no layers, legacy': { useLayers: false, legacyDisableLayers: true },
  'no layers, LTR/RTL comments': { useLayers: false, enableLTRRTLComments: true },
  'legacy class-name sort': { useLayers: true, useLegacyClassnamesSort: true },
  'no config': null,
};

interface SheetVerdict {
  configuration: string;
  rules: number;
  verdict: 'identical' | 'divergent' | 'rust refused' | 'babel refused' | 'both refused';
  bytes: number;
  detail?: string;
}

/** The at-rule chains of the matrix, each wrapped around one rule of its own. */
function matrixRules(): StyleXRule[] {
  const parsed: unknown = JSON.parse(fs.readFileSync(matrixFixture, 'utf8'));
  const chains = isRecord(parsed) && Array.isArray(parsed.chains) ? parsed.chains : [];

  return chains.flatMap((row, index): StyleXRule[] => {
    if (!isRecord(row) || !Array.isArray(row.preludes)) return [];
    const preludes = row.preludes.filter(
      (prelude): prelude is string => typeof prelude === 'string'
    );
    const className = `xmatrix${index}`;
    const open = preludes.map(prelude => `${prelude}{`).join('');
    const close = '}'.repeat(preludes.length);

    return [
      [className, { ltr: `${open}.${className}.${className}{color:red}${close}`, rtl: null }, 3000],
    ];
  });
}

/** The first line the two sheets differ on, for the report. */
function firstDifference(rust: string, babel: string): string {
  const rustLines = rust.split('\n');
  const babelLines = babel.split('\n');
  const count = Math.max(rustLines.length, babelLines.length);
  for (let index = 0; index < count; index++) {
    if (rustLines[index] !== babelLines[index]) {
      return `line ${index + 1}: rust ${JSON.stringify(rustLines[index] ?? '<end>')} / babel ${JSON.stringify(babelLines[index] ?? '<end>')}`;
    }
  }

  return `same lines, ${rust.length} vs ${babel.length} bytes`;
}

function attempt(run: () => string): { ok: true; sheet: string } | { ok: false; message: string } {
  try {
    return { ok: true, sheet: run() };
  } catch (error) {
    return { ok: false, message: messageOf(error) };
  }
}

async function main(): Promise<void> {
  const { values } = parseArgs({
    options: { json: { type: 'string' } },
    strict: true,
  });

  const {
    transform,
    processStylexRules: processWithRust,
    distEntry,
  } = await loadRustCompiler(packageDir);
  const { processStylexRules: processWithBabel, pluginEntry } = loadBabelPlugin();
  const versions = resolveVersions(packageDir, distEntry, pluginEntry);

  // The corpus, compiled by this compiler: the sheet step reads rules, and the
  // value harness already holds the two compilers to the same rules.
  const options = baseStyleXOptions(packageDir);
  const filename = path.join(packageDir, 'parity/__fixture__/value.js');
  const rules: StyleXRule[] = [];
  let compiled = 0;
  let refused = 0;
  for (const entry of loadCorpus(path.join(parityDir, 'corpus'))) {
    try {
      const result = transform(filename, moduleFor(entry), options);
      rules.push(...(result.metadata.stylex as StyleXRule[]));
      compiled++;
    } catch {
      refused++;
    }
  }
  const matrix = matrixRules();
  rules.push(...matrix);

  const verdicts: SheetVerdict[] = Object.entries(CONFIGURATIONS).map(([configuration, config]) => {
    const rust = attempt(() => processWithRust(rules, config));
    const babel = attempt(() => processWithBabel(rules, config));
    if (rust.ok && babel.ok) {
      return rust.sheet === babel.sheet
        ? { configuration, rules: rules.length, verdict: 'identical', bytes: rust.sheet.length }
        : {
            configuration,
            rules: rules.length,
            verdict: 'divergent',
            bytes: rust.sheet.length,
            detail: firstDifference(rust.sheet, babel.sheet),
          };
    }
    const verdict =
      !rust.ok && !babel.ok ? 'both refused' : rust.ok ? 'babel refused' : 'rust refused';
    const detail = [
      rust.ok ? null : `rust: ${rust.message}`,
      babel.ok ? null : `babel: ${babel.message}`,
    ]
      .filter((part): part is string => part !== null)
      .join(' / ');

    return { configuration, rules: rules.length, verdict, bytes: 0, detail };
  });

  console.log(chalk.bold('Stylesheet parity: processStylexRules\n'));
  console.log(
    subjectBlock(versions, [
      ['corpus entries', `${compiled} compiled, ${refused} refused by this compiler`],
      ['matrix chains', `${matrix.length}`],
      ['rules', `${rules.length}`],
    ])
  );
  console.log('');
  for (const verdict of verdicts) {
    const mark = verdict.verdict === 'identical' ? chalk.green('✓') : chalk.red('✗');
    const size = verdict.verdict === 'identical' ? chalk.dim(`${verdict.bytes} bytes`) : '';
    console.log(`  ${mark} ${verdict.configuration.padEnd(34)} ${verdict.verdict} ${size}`);
    if (verdict.detail) console.log(`      ${verdict.detail}`);
  }

  const failed = verdicts.filter(verdict => verdict.verdict !== 'identical');
  console.log('');
  console.log(
    failed.length === 0
      ? chalk.green(`Every configuration assembles the same sheet (${verdicts.length}).`)
      : chalk.red(`${failed.length} of ${verdicts.length} configurations differ.`)
  );

  if (values.json) {
    const written = writeJsonReport(packageDir, values.json, {
      generatedAt: new Date().toISOString(),
      subjects: versions,
      rules: rules.length,
      verdicts,
    });
    console.log(chalk.dim(`\nReport written to ${written}`));
  }

  if (failed.length > 0) process.exitCode = 1;
}

await main();
