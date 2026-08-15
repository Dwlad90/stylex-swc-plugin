/**
 * Runs one CSS declaration through both compilers and decides a verdict.
 *
 * Both compilers see the same module text and the same option object — option
 * drift would show up as a normalization divergence and send the reader
 * chasing the wrong thing, so the options are constructed once here and shared
 * rather than spelled out per subject.
 */

import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import * as babel from '@babel/core';
import stylexBabelPluginModule from '@stylexjs/babel-plugin';

import type { StyleXOptions } from '../../dist/index.js';
import { SEPARATOR } from './separator.js';
import type { CompilerOutcome, LoadedCorpusEntry, ReportEntry, Verdict } from './types.js';

const require = createRequire(import.meta.url);

type TransformFn = (
  filename: string,
  code: string,
  options: StyleXOptions
) => { metadata: { stylex: unknown[] }; code: string };

export interface SubjectVersions {
  rust: { version: string; resolvedFrom: string };
  babel: { version: string; resolvedFrom: string };
  babelCore: string;
}

export interface Comparer {
  options: StyleXOptions;
  versions: SubjectVersions;
  compare: (entry: LoadedCorpusEntry) => ReportEntry;
}

export interface CreateComparerOptions {
  /** Absolute path to the `@stylexswc/rs-compiler` package directory. */
  packageDir: string;
  /** Passed identically to both compilers. */
  enableFontSizePxToRem: boolean;
}

export async function createComparer(options: CreateComparerOptions): Promise<Comparer> {
  const { packageDir } = options;

  const distEntry = path.join(packageDir, 'dist/index.js');
  const loaded = (await import(pathToFileURL(distEntry).href)) as { transform?: TransformFn };
  const transform = loaded.transform;
  if (typeof transform !== 'function') {
    throw new Error(
      `${distEntry} does not export a transform function — run \`pnpm build\` in this package first.`
    );
  }

  const pluginCandidate = (stylexBabelPluginModule as { default?: unknown }).default;
  const stylexBabelPlugin = (pluginCandidate ?? stylexBabelPluginModule) as babel.PluginTarget;
  if (typeof stylexBabelPlugin !== 'function') {
    throw new Error('@stylexjs/babel-plugin did not export a Babel plugin function');
  }

  const babelPluginEntry = require.resolve('@stylexjs/babel-plugin');

  // `haste` module resolution keeps both compilers from needing a real
  // node_modules layout for the fixture, and `dev: false` keeps debug class
  // names — which encode a file path — out of the comparison.
  const stylexOptions: StyleXOptions = {
    dev: false,
    enableFontSizePxToRem: options.enableFontSizePxToRem,
    unstable_moduleResolution: { type: 'haste', rootDir: packageDir },
  };

  // A fixed filename: `haste` resolution and class hashing both read it, so
  // varying it per entry would vary the output for reasons unrelated to the
  // value under test.
  const filename = path.join(packageDir, 'parity/__fixture__/value.js');

  const runRust = (code: string): CompilerOutcome =>
    outcomeOf(() => transform(filename, code, stylexOptions).metadata.stylex);

  const runBabel = (code: string): CompilerOutcome =>
    outcomeOf(() => {
      const result = babel.transformSync(code, {
        filename,
        babelrc: false,
        configFile: false,
        parserOpts: { sourceType: 'module', plugins: ['jsx'] },
        plugins: [[stylexBabelPlugin, stylexOptions]],
      });
      const metadata = result?.metadata as { stylex?: unknown[] } | undefined;
      return metadata?.stylex ?? [];
    });

  return {
    options: stylexOptions,
    versions: {
      rust: {
        version: readVersion(path.join(packageDir, 'package.json')),
        resolvedFrom: distEntry,
      },
      babel: {
        version: readVersion(path.join(path.dirname(babelPluginEntry), '../package.json')),
        resolvedFrom: babelPluginEntry,
      },
      babelCore: babel.version,
    },
    compare(entry) {
      const code = moduleFor(entry);
      const rust = runRust(code);
      const babelOutcome = runBabel(code);
      return {
        id: entry.id,
        set: entry.set,
        property: entry.property,
        value: entry.value,
        origin: entry.origin,
        ...(entry.note === undefined ? {} : { note: entry.note }),
        verdict: verdictFor(rust, babelOutcome),
        rust,
        babel: babelOutcome,
      };
    },
  };
}

/** The module both compilers are handed for one declaration. */
export function moduleFor(entry: Pick<LoadedCorpusEntry, 'property' | 'value'>): string {
  return [
    "import * as stylex from '@stylexjs/stylex';",
    `export const styles = stylex.create({ x: { ${JSON.stringify(entry.property)}: ${JSON.stringify(entry.value)} } });`,
    '',
  ].join('\n');
}

function outcomeOf(run: () => unknown[]): CompilerOutcome {
  let rules: unknown[];
  try {
    rules = run();
  } catch (error: unknown) {
    return { status: 'error', message: messageOf(error) };
  }

  const classNames: string[] = [];
  const ruleTexts: string[] = [];
  const rtlRuleTexts: string[] = [];
  const declarations: string[] = [];

  for (const rule of rules) {
    if (!Array.isArray(rule)) continue;
    const [className, payload] = rule as [unknown, unknown];
    classNames.push(String(className));
    const ltr = ruleTextOf(payload, 'ltr');
    ruleTexts.push(ltr);
    // A right-to-left rule is emitted only for properties that have one. It is
    // compared but not reported: a divergence there is a divergence in the
    // value all the same, and the left-to-right spelling shows it more plainly.
    rtlRuleTexts.push(ruleTextOf(payload, 'rtl'));
    declarations.push(declarationOf(ltr));
  }

  return { status: 'ok', classNames, rules: ruleTexts, rtlRules: rtlRuleTexts, declarations };
}

/** One direction's rule text from a style-metadata payload, or `''` if absent. */
function ruleTextOf(payload: unknown, direction: 'ltr' | 'rtl'): string {
  if (typeof payload !== 'object' || payload === null || !(direction in payload)) return '';
  const text = (payload as Record<string, unknown>)[direction];
  // `null` is how the metadata spells "this property has no rule in this
  // direction", and it is the common case for `rtl`.
  return typeof text === 'string' ? text : '';
}

/**
 * The `property:value` text inside a rule's outermost braces — what value
 * normalization produced, stripped of the selector that carries the hash.
 */
function declarationOf(rule: string): string {
  const open = rule.indexOf('{');
  const close = rule.lastIndexOf('}');
  if (open === -1 || close <= open) return rule;
  return rule.slice(open + 1, close);
}

function messageOf(error: unknown): string {
  if (error instanceof Error) return error.message;
  return String(error);
}

function verdictFor(rust: CompilerOutcome, babelOutcome: CompilerOutcome): Verdict {
  if (rust.status === 'error' && babelOutcome.status === 'error') return 'both-reject';
  if (rust.status === 'error' || babelOutcome.status === 'error') return 'acceptance-divergent';
  const same =
    rust.classNames.join(SEPARATOR) === babelOutcome.classNames.join(SEPARATOR) &&
    rust.rules.join(SEPARATOR) === babelOutcome.rules.join(SEPARATOR) &&
    rust.rtlRules.join(SEPARATOR) === babelOutcome.rtlRules.join(SEPARATOR);
  if (same) return 'identical';

  // A declaration that expanded into different properties, or into a different
  // number of them, diverged before value normalization ever saw it —
  // shorthand expansion and property validation both do that. Separating those
  // keeps the divergence count an answer about values.
  return propertyNamesOf(rust) === propertyNamesOf(babelOutcome)
    ? 'divergent'
    : 'structurally-divergent';
}

/** The emitted property names, sorted, as a comparable key. */
function propertyNamesOf(outcome: CompilerOutcome): string {
  if (outcome.status === 'error') return '';
  return outcome.declarations
    .map(declaration => declaration.slice(0, declaration.indexOf(':')).trim())
    .toSorted()
    .join(SEPARATOR);
}

function readVersion(manifestPath: string): string {
  try {
    const raw = JSON.parse(fs.readFileSync(manifestPath, 'utf8')) as { version?: string };
    return raw.version ?? 'unknown';
  } catch {
    return 'unknown';
  }
}
