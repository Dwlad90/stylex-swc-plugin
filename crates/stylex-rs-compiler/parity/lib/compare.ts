/**
 * Runs one corpus subject through both compilers and decides a verdict.
 *
 * Both compilers see the same module text and the same option object — option
 * drift would show up as a normalization divergence and send the reader
 * chasing the wrong thing, so the options are constructed once here and shared
 * rather than spelled out per subject.
 */

import path from 'node:path';

import * as babel from '@babel/core';

import type { StyleXOptions } from '../../dist/index.js';
import {
  baseStyleXOptions,
  loadBabelPlugin,
  loadRustCompiler,
  messageOf,
  resolveVersions,
} from './compilers.js';
import type { SubjectVersions } from './compilers.js';
import { arrayAt, stringAt } from './guards.js';
import { refusalSentence } from './refusal.js';
import { SEPARATOR } from './separator.js';
import { styleObjectsOf } from './style-object.js';
import { moduleFor } from './subject.js';
import type { CompilerOutcome, LoadedCorpusEntry, ReportEntry, Verdict } from './types.js';

export interface Comparer {
  options: StyleXOptions;
  versions: SubjectVersions;
  compare: (entry: LoadedCorpusEntry) => ReportEntry;
}

/**
 * What one compiler run produced: the style metadata it collected, and the
 * module it printed. Both halves are needed because they answer different
 * questions -- the metadata carries the CSS, and only the printed module carries
 * the style objects, where an absent value shows.
 */
interface CompilerRun {
  rules: unknown[];
  emitted: string;
}

export interface CreateComparerOptions {
  /** Absolute path to the `@stylexswc/rs-compiler` package directory. */
  packageDir: string;
  /** Passed identically to both compilers. */
  enableFontSizePxToRem: boolean;
  /**
   * Which style resolution both compilers run under. Omitted leaves each
   * compiler on its own default, which both spell `property-specificity`.
   *
   * Both harnesses pass it, for the same reason stated two ways. The generated
   * one pins `legacy-expand-shorthands` because that is the only resolution
   * shorthand value splitting is reached under, and a run left on the default
   * would compare two compilers that both never called it and report agreement.
   * The value harness takes it as a flag, because which longhands a shorthand
   * becomes and what order they land in differ between all three — a class name
   * depends on that, and a report that does not say which resolution it measured
   * cannot be compared with another one.
   */
  styleResolution?: StyleXOptions['styleResolution'];
}

export async function createComparer(options: CreateComparerOptions): Promise<Comparer> {
  const { packageDir } = options;

  const { transform, distEntry } = await loadRustCompiler(packageDir);
  const { plugin: stylexBabelPlugin, pluginEntry: babelPluginEntry } = loadBabelPlugin();

  const stylexOptions: StyleXOptions = {
    ...baseStyleXOptions(packageDir),
    enableFontSizePxToRem: options.enableFontSizePxToRem,
    ...(options.styleResolution != null ? { styleResolution: options.styleResolution } : {}),
  };

  // A fixed filename: `haste` resolution and class hashing both read it, so
  // varying it per entry would vary the output for reasons unrelated to the
  // value under test.
  const filename = path.join(packageDir, 'parity/__fixture__/value.js');

  const runRust = (code: string): CompilerOutcome =>
    outcomeOf(filename, (): CompilerRun => {
      const result = transform(filename, code, stylexOptions);
      return { rules: result.metadata.stylex, emitted: result.code };
    });

  const runBabel = (code: string): CompilerOutcome =>
    outcomeOf(filename, (): CompilerRun => {
      const result = babel.transformSync(code, {
        filename,
        babelrc: false,
        configFile: false,
        parserOpts: { sourceType: 'module', plugins: ['jsx'] },
        plugins: [[stylexBabelPlugin, stylexOptions]],
      });
      return { rules: arrayAt(result?.metadata, 'stylex') ?? [], emitted: result?.code ?? '' };
    });

  return {
    options: stylexOptions,
    versions: resolveVersions(packageDir, distEntry, babelPluginEntry),
    compare(entry) {
      const code = moduleFor(entry);
      const rust = runRust(code);
      const babelOutcome = runBabel(code);
      return { ...entry, verdict: verdictFor(rust, babelOutcome), rust, babel: babelOutcome };
    },
  };
}

/**
 * `filename` is the one both compilers were handed, and it is here because a
 * refusal is normalized where it is caught: `refusalSentence` derives the
 * reference implementation's message prefix from that path, and a caller that
 * normalized later would have to be trusted to pass the same one.
 */
function outcomeOf(filename: string, run: () => CompilerRun): CompilerOutcome {
  let rules: unknown[];
  let emitted: string;
  try {
    ({ rules, emitted } = run());
  } catch (error: unknown) {
    const message = messageOf(error);
    return { status: 'error', message, sentence: refusalSentence(message, filename) };
  }

  const classNames: string[] = [];
  const ruleTexts: string[] = [];
  const rtlRuleTexts: string[] = [];
  const declarations: string[] = [];

  for (const rule of rules) {
    if (!Array.isArray(rule)) continue;
    const [className, payload] = rule;
    classNames.push(String(className));
    const ltr = ruleTextOf(payload, 'ltr');
    ruleTexts.push(ltr);
    // A right-to-left rule is emitted only for properties that have one. It is
    // compared but not reported: a divergence there is a divergence in the
    // value all the same, and the left-to-right spelling shows it more plainly.
    rtlRuleTexts.push(ruleTextOf(payload, 'rtl'));
    declarations.push(declarationOf(ltr));
  }

  let parsedStyleObjects: string[] | undefined;

  return {
    status: 'ok',
    classNames,
    rules: ruleTexts,
    rtlRules: rtlRuleTexts,
    declarations,
    /**
     * Parsed on the first read rather than on the way out, and kept.
     *
     * This is the most expensive thing in the file -- a full `babel.parseSync`
     * plus a traversal of the emitted module -- and `styleObjectsAgree` returns
     * `false` outright whenever either side refused, without reading it. So on
     * every row where one compiler accepted and the other did not, the
     * accepting side was parsed for an answer nothing consulted: 187 of the
     * 1085 curated subjects, and 2631 of 19203 per property in the generated
     * sweep.
     *
     * A getter rather than a changed field type, so nothing that builds an
     * outcome by hand -- the unit tests do, with a literal array -- has to know
     * this is lazy. Memoized because the report reads it again when the shapes
     * are what differ.
     */
    get styleObjects(): string[] {
      parsedStyleObjects ??= styleObjectsOf(emitted);

      return parsedStyleObjects;
    },
  };
}

/** One direction's rule text from a style-metadata payload, or `''` if absent. */
function ruleTextOf(payload: unknown, direction: 'ltr' | 'rtl'): string {
  // `null` is how the metadata spells "this property has no rule in this
  // direction", and it is the common case for `rtl`.
  return stringAt(payload, direction) ?? '';
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

function verdictFor(rust: CompilerOutcome, babelOutcome: CompilerOutcome): Verdict {
  // Two refusals are compared by what they complain about, not only by the
  // fact of refusing: an author whose build stops reads the message, so two
  // compilers stopping it for reasons they word differently have diverged in
  // the half of the behaviour a refused input has. `lib/refusal.ts` carries
  // the normalization the comparison rests on.
  if (rust.status === 'error' && babelOutcome.status === 'error') {
    // A sentence that reduced to nothing is not something two refusals can be
    // agreed to share -- a messageless throw on both sides would otherwise read
    // as agreement about a complaint neither made. There the raw messages are
    // the only evidence left, so they are what is compared.
    const comparable = rust.sentence !== '' && babelOutcome.sentence !== '';
    const agreed = comparable
      ? rust.sentence === babelOutcome.sentence
      : rust.message === babelOutcome.message;

    return agreed ? 'both-reject' : 'both-reject-divergent';
  }
  if (rust.status === 'error' || babelOutcome.status === 'error') return 'acceptance-divergent';
  const sameCss =
    rust.classNames.join(SEPARATOR) === babelOutcome.classNames.join(SEPARATOR) &&
    rust.rules.join(SEPARATOR) === babelOutcome.rules.join(SEPARATOR) &&
    rust.rtlRules.join(SEPARATOR) === babelOutcome.rtlRules.join(SEPARATOR);
  // The style objects are the other half of the answer: a property carrying
  // `null` emits no CSS, so without this two compilers that disagree about
  // whether the property exists at all read as identical. See
  // `lib/style-object.ts`.
  const sameStyleObjects = styleObjectsAgree(rust, babelOutcome);

  if (sameCss && sameStyleObjects) {
    // Agreement about nothing is not evidence of parity — see `identical-empty`
    // in `types.ts`. Reported separately so a corpus that stops carrying its
    // values shows up as a count rather than as a clean run.
    const emitted =
      rust.classNames.length + rust.rules.length + rust.rtlRules.length > 0 ||
      babelOutcome.classNames.length + babelOutcome.rules.length + babelOutcome.rtlRules.length >
        0 ||
      rust.styleObjects.some(object => object !== '{}') ||
      babelOutcome.styleObjects.some(object => object !== '{}');
    return emitted ? 'identical' : 'identical-empty';
  }

  // The same CSS out of a different set of properties is a disagreement about
  // which declarations exist, not about how a value is spelled.
  if (sameCss) return 'structurally-divergent';

  // A declaration that expanded into different properties, or into a different
  // number of them, diverged before value normalization ever saw it —
  // shorthand expansion and property validation both do that. Separating those
  // keeps the divergence count an answer about values.
  return propertyNamesOf(rust) === propertyNamesOf(babelOutcome) && sameStyleObjects
    ? 'divergent'
    : 'structurally-divergent';
}

/**
 * Whether two outcomes emitted the same style objects.
 *
 * Exported because the report needs the same answer the verdict does: it prints
 * the shapes only when they are what differ, and asking that question a second
 * way in the printer is how the two would come to disagree. An outcome that
 * rejected has no shape, so it cannot agree with one that does.
 */
export function styleObjectsAgree(left: CompilerOutcome, right: CompilerOutcome): boolean {
  if (left.status === 'error' || right.status === 'error') return false;
  return left.styleObjects.join(SEPARATOR) === right.styleObjects.join(SEPARATOR);
}

/** The emitted property names, sorted, as a comparable key. */
function propertyNamesOf(outcome: CompilerOutcome): string {
  if (outcome.status === 'error') return '';
  return outcome.declarations
    .map(declaration => declaration.slice(0, declaration.indexOf(':')).trim())
    .toSorted()
    .join(SEPARATOR);
}
