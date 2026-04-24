import picomatch from 'picomatch';
import * as path from 'path';

import nativeBinding from '../dist/transform';

// ── Re-exports from native binding ──────────────────────────────────

export type {
  ImportSourceInput,
  StyleXMetadata,
  StyleXModuleResolution,
  StyleXTransformResult,
} from '../dist/transform';

import type {
  StyleXOptions as NativeStyleXOptions,
  StyleXTransformResult,
} from '../dist/transform';

// const enums are erased by TypeScript — provide runtime values
// so ESM consumers can import them.
export const SourceMaps = {
  True: 'True',
  False: 'False',
  Inline: 'Inline',
} as const;

export const PropertyValidationMode = {
  Throw: 'throw',
  Warn: 'warn',
  Silent: 'silent',
} as const;

// ── Extended types ──────────────────────────────────────────────────

/** StyleX compiler options (native options + TS-only fields). */
export interface StyleXOptions extends NativeStyleXOptions {
  include?: Array<string | RegExp>;
  exclude?: Array<string | RegExp>;
  swcPlugins?: Array<[string, Record<string, unknown>]>;
}

export type UseLayersType =
  | boolean
  | {
      before?: ReadonlyArray<string>;
      after?: ReadonlyArray<string>;
      prefix?: string;
    };

export type TransformedOptions = Partial<
  Pick<StyleXOptions, 'legacyDisableLayers' | 'enableLTRRTLComments'> & {
    useLayers: UseLayersType;
  }
>;

/**
 * Default values for StyleX options.
 * Every field that has a sensible default is listed here.
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
  useRealFileForSource: true,
  enableMediaQueryOrder: true,
  enableDebugClassNames: false,
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
    } as StyleXTransformResult;
  }

  let transformedCode = code;

  if (options.swcPlugins?.length) {
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    const swc = require('@swc/core');

    const result = swc.transformSync(transformedCode, {
      filename,
      sourceMaps:
        options.sourceMap === 'Inline'
          ? 'inline'
          : options.sourceMap === 'False'
            ? false
            : options.sourceMap !== undefined,
      jsc: {
        parser: { syntax: 'typescript', tsx: true },
        target: 'es2022',
        experimental: { plugins: options.swcPlugins },
      },
    });
    transformedCode = result.code;
  }

  // Strip TS-only fields before passing to native transform
  const {
    swcPlugins: _swcPlugins,
    include: _include,
    exclude: _exclude,
    ...nativeOptions
  } = options;

  return nativeBinding.transform(filename, transformedCode, nativeOptions);
}
