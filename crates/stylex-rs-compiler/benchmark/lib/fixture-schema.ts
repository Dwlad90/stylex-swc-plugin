/**
 * The closed vocabularies a fixture is described with, and the reader for them.
 *
 * Held apart from `lib/types.ts`, which keeps these beside a value it reads off
 * `dist/index.js`. Importing that module loads this package's native binding,
 * and the child process of a split run must hold no binding but its subject's.
 * So the names and the rules live here, where nothing needs the addon, and
 * `lib/types.ts` passes them on to everyone who already asks it for them.
 *
 * One reader, two callers. The manifest reader narrows what a person wrote in
 * `fixtures.v1.json`; the worker reader narrows what arrived from the parent
 * process. Both must refuse the same keys for the same reasons, so the rules
 * are here once and each caller supplies only the one step that differs.
 */

import type { StyleXOptions } from '../../dist/index.js';
import { isRecord, requireOneOf, requireString } from './json.js';
import type { SourceMapSetting } from './types.js';

export type FixtureWeight = 'standard' | 'heavy';
export type FixtureCategory = 'transform' | 'perf' | 'rollup';

export const FIXTURE_WEIGHTS: readonly FixtureWeight[] = ['standard', 'heavy'];
export const FIXTURE_CATEGORIES: readonly FixtureCategory[] = ['transform', 'perf', 'rollup'];

/**
 * Options that take a boolean and change what the compiler emits.
 *
 * Each one is measured because it changes the output on some fixture in the
 * corpus. `fixtures.test.ts` fails an entry whose options leave the emitted
 * module identical to its production run, which is how that was found.
 */
export const BOOLEAN_OPTION_KEYS = [
  'dev',
  'debug',
  'enableDebugClassNames',
  'enableDebugDataProp',
  'enableDevClassNames',
  'enableMinifiedKeys',
  'enableFontSizePxToRem',
  'enableInlinedConditionalMerge',
  'enableLegacyValueFlipping',
  'enableMediaQueryOrder',
  'useRealFileForSource',
  'runtimeInjection',
  'injectStylexSideEffects',
  'test',
  'inlineSourcesContent',
  'emitSourceMapColumns',
] as const;

export type BooleanOptionKey = (typeof BOOLEAN_OPTION_KEYS)[number];

export const STYLE_RESOLUTIONS = [
  'application-order',
  'property-specificity',
  'legacy-expand-shorthands',
] as const;

/**
 * How a manifest spells each `sourceMap` setting.
 *
 * The type ties the list to the settings the package exports, so a spelling
 * that is not one of them does not compile. The list itself carries no value of
 * the addon's enum, which is what lets a process without the addon read it.
 */
export const SOURCE_MAP_SPELLINGS: readonly SourceMapSetting[] = ['True', 'False', 'Inline'];

/** A fixture's own measurement conditions, as the manifest declares them. */
export type FixtureOptionOverrides = Partial<
  Record<BooleanOptionKey, boolean> & {
    styleResolution: (typeof STYLE_RESOLUTIONS)[number];
    sourceMap: NonNullable<StyleXOptions['sourceMap']>;
    classNamePrefix: string;
  }
>;

/** The value `sourceMap` holds once it is read: a member of the addon's enum. */
export type SourceMapValue = NonNullable<FixtureOptionOverrides['sourceMap']>;

/**
 * How a caller turns a spelling of `sourceMap` into the value the addon takes.
 *
 * The one step the two callers cannot share. The manifest reader has the
 * package's own export and looks the value up in it. The worker reader is in a
 * process that must load no addon, so it holds the spelling to the list above
 * and names the value from it.
 *
 * @throws Error when the value is not a setting the compiler accepts.
 */
export type SourceMapReader = (value: unknown, context: string) => SourceMapValue;

/**
 * One fixture's option overrides, narrowed key by key.
 *
 * An unknown key is an error rather than a key dropped: a manifest that names
 * `enableDebugDataProps` would otherwise be measured under the production shape
 * while claiming to price the debug one, and the number it reports would look
 * entirely reasonable.
 */
export function parseOptionOverrides(
  input: unknown,
  context: string,
  readSourceMap: SourceMapReader
): FixtureOptionOverrides {
  if (!isRecord(input)) throw new Error(`${context} must be an object`);

  const overrides: FixtureOptionOverrides = {};

  for (const [key, value] of Object.entries(input)) {
    if (isBooleanOptionKey(key)) {
      if (typeof value !== 'boolean') {
        throw new Error(`${context}.${key} must be a boolean`);
      }
      overrides[key] = value;
      continue;
    }

    // The keys whose value is not a boolean are spelled out rather than
    // tabulated, because a table would have to hold each one's accepted values
    // against a key whose value type differs from every other key's.
    if (key === 'styleResolution') {
      overrides.styleResolution = requireOneOf(value, STYLE_RESOLUTIONS, `${context}.${key}`);
      continue;
    }
    if (key === 'sourceMap') {
      overrides.sourceMap = readSourceMap(value, `${context}.${key}`);
      continue;
    }
    if (key === 'classNamePrefix') {
      overrides.classNamePrefix = requireString(value, `${context}.${key}`);
      continue;
    }

    throw new Error(
      `${context}.${key} is not a benchmarkable option — the accepted keys are ` +
        `${BOOLEAN_OPTION_KEYS.join(', ')}, styleResolution, sourceMap, ` +
        `classNamePrefix`
    );
  }

  return overrides;
}

function isBooleanOptionKey(key: string): key is BooleanOptionKey {
  // Widened to compare, not asserted: the predicate is what narrows, and the
  // caller only ever indexes with a key this returned true for.
  const keys: readonly string[] = BOOLEAN_OPTION_KEYS;
  return keys.includes(key);
}
