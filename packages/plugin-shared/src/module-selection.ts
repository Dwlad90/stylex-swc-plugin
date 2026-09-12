/**
 * Which modules a bundler plugin hands to the compiler.
 *
 * Every plugin runs this scan over the module source before it pays for a
 * compile. The answer must be the same in all of them, so it lives here
 * rather than in each plugin.
 *
 * Kept in its own module — and out of the package entry — so a plugin can pull
 * it in without also loading the plugin core and its webpack dependencies.
 */

import { escapeRegExp } from './regexp';

/**
 * An import source as any plugin may carry it: a bare specifier, or a pair of
 * the source it comes `from` and the name it is imported `as`.
 *
 * Both fields are optional here because the plugins disagree: the compiler
 * options declare both, while the PostCSS plugin option lets `as` be left out.
 */
export type ModuleImportSource = string | { from?: string; as?: string };

/**
 * The `sxPropName` option: the name of the `sx` shorthand prop, or `false` to
 * disable the feature. Absent means the compiler default, `sx`.
 */
export type SxPropNameOption = string | false;

/** What the scan needs to know about the resolved plugin options. */
export interface ModuleSelectionOptions {
  /** The import sources the compiler is configured to recognise. */
  importSources?: readonly ModuleImportSource[];
  /** The `sx` shorthand prop name, as the user configured it. */
  sxPropName?: SxPropNameOption;
}

/** The prop name the compiler uses when the option is left out. */
const DEFAULT_SX_PROP_NAME = 'sx';

/**
 * True when the source text mentions `name`.
 *
 * A missing or blank name is never a mention. It would otherwise match every
 * module and turn one misconfigured entry into a full-project compile. The
 * webpack and Turbopack loaders used to search the module for the text
 * `undefined` when an import source left `as` out, which had the same effect
 * on a smaller scale.
 */
function mentions(sourceCode: string, name: string | undefined): boolean {
  return typeof name === 'string' && name.trim() !== '' && sourceCode.includes(name);
}

/**
 * Reads the `from` field out of an import source that arrived as JSON text.
 *
 * Some hosts pass the options through a serialising layer, which turns an
 * object import source into its own JSON text. Searching the module for that
 * text would never match, so the specifier is read out of it instead.
 *
 * Returns `undefined` when the text is not JSON carrying a string `from`;
 * the caller then falls back to the plain substring scan.
 */
function readSerializedFrom(text: string): string | undefined {
  try {
    const parsed: unknown = JSON.parse(text);

    if (typeof parsed === 'object' && parsed !== null) {
      const { from } = parsed as { from?: unknown };

      if (typeof from === 'string') {
        return from.trim();
      }
    }
  } catch {
    // Not valid JSON. The plain substring scan still applies.
  }

  return undefined;
}

/** True when the source text mentions either half of one import source. */
function mentionsImportSource(sourceCode: string, importSource: ModuleImportSource): boolean {
  if (typeof importSource !== 'string') {
    return mentions(sourceCode, importSource.as) || mentions(sourceCode, importSource.from);
  }

  const trimmed = importSource.trimStart();

  if (trimmed.startsWith('{')) {
    const from = readSerializedFrom(trimmed);

    if (from !== undefined) {
      return mentions(sourceCode, from);
    }
  }

  return mentions(sourceCode, importSource);
}

/**
 * Patterns already built, keyed by prop name.
 *
 * The scan runs once per module, so a project with thousands of modules would
 * otherwise recompile the same pattern thousands of times. The keys come from
 * the plugin options, so the map holds one entry per configured name.
 */
const sxPropPatterns = new Map<string, RegExp>();

/**
 * A pattern matching the prop name in a prop-like position: followed by `=`,
 * `:`, `,` or `}`, with optional whitespace between the two. That covers the
 * attribute `sx={...}`, the property `sx: ...`, the shorthand `{ sx }` and the
 * shorthand before further properties, `{ sx, ... }`.
 *
 * The name must also start a word. Without that, every identifier that ends
 * with the name matches too: with the default name, `import { jsx } from
 * "react/jsx-runtime"` matches on `jsx }`, which selects almost every module
 * a build step has already compiled.
 *
 * The name is escaped, so a name carrying pattern metacharacters matches
 * itself instead of corrupting the pattern.
 */
function sxPropPattern(name: string): RegExp {
  const cached = sxPropPatterns.get(name);

  if (cached) {
    return cached;
  }

  const pattern = new RegExp(`(?<![\\p{ID_Continue}$])${escapeRegExp(name)}\\s*[=:,}]`, 'u');

  sxPropPatterns.set(name, pattern);

  return pattern;
}

/**
 * True when the source text uses the configured `sx` prop.
 *
 * A module that only forwards the prop has no StyleX import to find, so the
 * import scan alone would drop it and leave the element unstyled. The prop
 * transform does not read the import sources, so this half stands on its own.
 *
 * Returns false when the prop is disabled, or when the name is blank and so
 * cannot be searched for; module selection is then what it was before this
 * half existed.
 */
function mentionsSxProp(sourceCode: string, sxPropName: SxPropNameOption | undefined): boolean {
  if (sxPropName === false) {
    return false;
  }

  const name = sxPropName ?? DEFAULT_SX_PROP_NAME;

  if (name.trim() === '') {
    return false;
  }

  return sxPropPattern(name).test(sourceCode);
}

/**
 * Decide whether a module source is worth handing to the compiler.
 *
 * This is a text scan, not a parse: a mention inside a string or a comment
 * counts. The trade is deliberate. A false positive costs one compile that
 * produces unchanged output, while a false negative costs a silently
 * unstyled element.
 *
 * @param sourceCode the module source, as text
 * @param options the resolved plugin options
 */
export function shouldProcessSource(sourceCode: string, options: ModuleSelectionOptions): boolean {
  const { importSources, sxPropName } = options;

  // The import half answers first, because a substring sweep is the cheaper of
  // the two. An empty list simply matches nothing; it no longer decides the
  // whole answer, because the prop transform does not read the import sources.
  const mentionsAnyImportSource =
    importSources?.some(importSource => mentionsImportSource(sourceCode, importSource)) ?? false;

  return mentionsAnyImportSource || mentionsSxProp(sourceCode, sxPropName);
}
