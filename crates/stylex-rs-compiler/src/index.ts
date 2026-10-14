import * as path from 'path';

import picomatch from 'picomatch';

import nativeBinding from '../dist/transform';

// ── Re-exports from native binding ──────────────────────────────────

export type {
  ImportSourceInput,
  StyleXMetadata,
  StyleXModuleResolution,
  StyleXTransformResult,
} from '../dist/transform';

import type {
  SourceMaps as NativeSourceMaps,
  StyleXMetadata,
  StyleXOptions as NativeStyleXOptions,
  StyleXTransformResult,
} from '../dist/transform';

// const enums are erased by TypeScript — provide runtime values
// so ESM consumers can import them. Typed as the native enum rather than as
// string literals: without that, `{ sourceMap: SourceMaps.False }` fails to
// typecheck, which defeats the point of exporting the values at all.
export const SourceMaps = Object.freeze({
  True: 'True',
  False: 'False',
  Inline: 'Inline',
}) as Readonly<{
  True: NativeSourceMaps;
  False: NativeSourceMaps;
  Inline: NativeSourceMaps;
}>;

export const PropertyValidationMode = Object.freeze({
  Throw: 'throw',
  Warn: 'warn',
  Silent: 'silent',
} as const);

// ── The stylesheet step ─────────────────────────────────────────────

/** One emitted rule, a row of `StyleXMetadata['stylex']`: the reference's `Rule`. */
export type StyleXRule = StyleXMetadata['stylex'][number];

/**
 * The `useLayers` object form: layers around the priority layers, and a prefix
 * for their names. One shape for `processStylexRules` and for the plugins'
 * `useCSSLayers` option (`UseLayersType` below).
 */
export interface ProcessStylexRulesLayerOptions {
  readonly before?: ReadonlyArray<string>;
  readonly after?: ReadonlyArray<string>;
  readonly prefix?: string;
}

/**
 * The options of `processStylexRules`, as `@stylexjs/babel-plugin` spells
 * them. A superset of the type that package publishes in its `index.d.ts`
 * (0.19.1): `useLayers` also takes `null` (read as absent, as the runtime
 * reads it), and `useLegacyClassnamesSort` is declared, which the reference's
 * Flow source reads but its `.d.ts` omits.
 */
export interface ProcessStylexRulesConfig {
  readonly useLayers?: boolean | ProcessStylexRulesLayerOptions | null;
  readonly enableLTRRTLComments?: boolean;
  readonly legacyDisableLayers?: boolean;
  readonly useLegacyClassnamesSort?: boolean;
}

/**
 * The stylesheet for the rules a build's `transform` calls emitted, as
 * `@stylexjs/babel-plugin`'s `processStylexRules` writes it, byte for byte:
 * constants resolved and substituted, the rules sorted and grouped by priority,
 * each group a `@layer` or carrying the specificity polyfill, RTL twins under
 * `html[dir='rtl']` and its negation or between `@ltr`/`@rtl` comments.
 *
 * `rules` is the concatenation of every file's `metadata.stylex`, in the order
 * the files were compiled; `config` is the reference's, a boolean standing for
 * `useLayers` or the options object. A circular constant throws with the
 * reference's message; the README lists where a value outside the published
 * types is read differently from the reference.
 *
 * The signature is a superset of the one `@stylexjs/babel-plugin` 0.19.1
 * publishes: `rules` is `readonly` where the reference says `Array<Rule>`,
 * `config` also takes `null` (read as absent, as the runtime reads it), and
 * the tuple type is named `StyleXRule` here, `Rule` there. Every call that
 * typechecks against the reference typechecks here.
 */
export const processStylexRules: (
  rules: readonly StyleXRule[],
  config?: boolean | ProcessStylexRulesConfig | null
) => string = nativeBinding.processStylexRules;

// ── Extended types ──────────────────────────────────────────────────

/** StyleX compiler options (native options + TS-only fields). */
export interface StyleXOptions extends NativeStyleXOptions {
  include?: Array<string | RegExp>;
  exclude?: Array<string | RegExp>;
  inputSourceMap?: string;
  swcPlugins?: Array<[string, Record<string, unknown>]>;
}

export type UseLayersType = boolean | ProcessStylexRulesLayerOptions;

export type TransformedOptions = Partial<
  Pick<
    StyleXOptions,
    'legacyDisableLayers' | 'enableLTRRTLComments' | 'useLegacyClassnamesSort'
  > & {
    useLayers: UseLayersType;
  }
>;

/**
 * Default values for StyleX options.
 * Every field that has a sensible default is listed here.
 *
 * `maxEvaluationDepth`, `maxFoldedCharacters` and `maxFoldedEntries` are
 * deliberately absent. Each default lives in the compiler, which falls back to
 * that ceiling's environment variable before reaching for it -- so a value
 * listed here would be sent on every call and make the variable unreachable.
 */
const defaultOptions: Partial<StyleXOptions> = {
  dev: false,
  test: false,
  debug: false,
  enableFontSizePxToRem: false,
  runtimeInjection: false,
  treeshakeCompensation: false,
  enableInlinedConditionalMerge: true,
  enableLogicalStylesPolyfill: false,
  enableMinifiedKeys: true,
  enableLegacyValueFlipping: false,
  enableLTRRTLComments: false,
  legacyDisableLayers: false,
  useLegacyClassnamesSort: false,
  useRealFileForSource: true,
  inlineSourcesContent: true,
  emitSourceMapColumns: true,
  enableMediaQueryOrder: true,
  propertyValidationMode: 'silent',
  styleResolution: 'property-specificity',
  importSources: ['stylex', '@stylexjs/stylex'],
};

// ── normalizeRsOptions ──────────────────────────────────────────────

/** Strip keys whose value is `undefined` so they don't clobber defaults. */
function definedEntries(obj: Record<string, unknown>): Record<string, unknown> {
  return Object.fromEntries(Object.entries(obj).filter(([, v]) => v !== undefined));
}

/**
 * Normalize StyleX compiler options by applying defaults and merging
 * user-provided values. Uses a spread/defaults pattern: defaults are
 * applied first, then user-provided values overlay them
 * (undefined keys skipped).
 */
export function normalizeRsOptions(options?: StyleXOptions | null): StyleXOptions {
  if (options == null) {
    throw new TypeError('Options must be an object, received null/undefined');
  }

  // Non-object input (string, number, etc.) — treat as empty options
  const inputOptions: StyleXOptions = typeof options === 'object' ? options : {};
  const definedOptions: Partial<StyleXOptions> = definedEntries(
    inputOptions as unknown as Record<string, unknown>
  );

  // Spread defaults then user values (undefined keys already stripped)
  const result: StyleXOptions = {
    ...defaultOptions,
    ...definedOptions,

    include: definedOptions.include ?? [],
    exclude: definedOptions.exclude ?? [],
    swcPlugins: definedOptions.swcPlugins ?? [],
  };

  return result;
}

// ── shouldTransformFile ─────────────────────────────────────────────

/**
 * Determine whether a file should be transformed based on include/exclude
 * patterns (glob strings or RegExp).
 */
export function shouldTransformFile(
  filePath: string,
  include?: Array<string | RegExp> | null,
  exclude?: Array<string | RegExp> | null
): boolean {
  const relativePath = path.relative(process.cwd(), filePath).split(path.sep).join('/');

  if (include && include.length > 0) {
    if (!include.some(p => matchPattern(relativePath, p))) {
      return false;
    }
  }

  if (exclude && exclude.length > 0) {
    if (exclude.some(p => matchPattern(relativePath, p))) {
      return false;
    }
  }

  return true;
}

/** Match a file path against a single pattern (glob string or RegExp). */
function matchPattern(filePath: string, pattern: string | RegExp): boolean {
  if (pattern instanceof RegExp) {
    // Reset lastIndex to avoid nondeterministic results with /g or /y flags
    pattern.lastIndex = 0;
    return pattern.test(filePath);
  }
  if (typeof pattern !== 'string' || pattern === '') {
    return false;
  }
  return picomatch.isMatch(filePath, pattern, { dot: true });
}

/**
 * Whether an angle bracket in `filename` may open a JSX element: in every
 * module but a `.ts`, `.mts` or `.cts` one, where it opens a type parameter
 * list or a type assertion instead. The same answer the native transform gives
 * (`stylex_utils::swc::syntax_for_file`), so a plugin pre-pass parses exactly
 * what the native pass then parses: the extension is read without regard to
 * case, and a name that is nothing but the extension (`.ts`, `/dir/.ts`) has
 * no extension, as `Path::extension` reads it, so it keeps JSX. Exported for
 * the test that holds the two rules together.
 */
const TYPESCRIPT_WITHOUT_JSX = /[^/\\]\.[mc]?ts$/i;

export function admitsJsx(filename: string): boolean {
  return !TYPESCRIPT_WITHOUT_JSX.test(filename);
}

// ── transform ───────────────────────────────────────────────────────

/**
 * Transform source code with StyleX. When `options.swcPlugins` is set,
 * SWC plugins are applied first, then the native StyleX transform runs.
 */
export function transform(
  filename: string,
  code: string,
  options: StyleXOptions
): StyleXTransformResult {
  // Apply include/exclude filter before transforming
  if (!shouldTransformFile(filename, options.include, options.exclude)) {
    return {
      code,
      metadata: { stylex: [] },
      map: undefined,
    };
  }

  let transformedCode = code;
  let inputSourceMap = options.inputSourceMap;

  if (options.swcPlugins?.length) {
    // oxlint-disable-next-line typescript/no-require-imports
    const swc = require('@swc/core');

    // Always request an external map: it chains `inputSourceMap` and the
    // plugins' own edits, so the positions handed to the native transform
    // keep describing the code it actually receives. The native transform
    // owns emission of the final map per `options.sourceMap` (incl. inline).
    // The intermediate map becomes the native transform's `orig`, and a
    // chained map keeps whatever `sourcesContent` it arrives with — so it has
    // to carry the authored text from here.
    const result = swc.transformSync(transformedCode, {
      filename,
      sourceMaps: true,
      inlineSourcesContent: options.inlineSourcesContent ?? true,
      inputSourceMap,
      jsc: {
        parser: { syntax: 'typescript', tsx: admitsJsx(filename) },
        target: 'es2022',
        experimental: { plugins: options.swcPlugins },
      },
    });
    transformedCode = result.code;
    if (result.map) {
      inputSourceMap = result.map;
    }
  }

  // Strip TS-only fields before passing to native transform
  const {
    swcPlugins: _swcPlugins,
    include: _include,
    exclude: _exclude,
    ...nativeOptions
  } = options;

  return nativeBinding.transform(filename, transformedCode, {
    ...nativeOptions,
    inputSourceMap,
  });
}
