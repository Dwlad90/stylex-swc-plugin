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

/**
 * An import source as any plugin may carry it: a bare specifier, or a pair of
 * the source it comes `from` and the name it is imported `as`.
 *
 * Both fields are optional here because the plugins disagree: the compiler
 * options declare both, while the PostCSS plugin option lets `as` be left out.
 */
export type ModuleImportSource = string | { from?: string; as?: string };

/** What the scan needs to know about the resolved plugin options. */
export interface ModuleSelectionOptions {
  /** The import sources the compiler is configured to recognise. */
  importSources?: readonly ModuleImportSource[];
}

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
  const { importSources } = options;

  if (!importSources || importSources.length === 0) {
    return false;
  }

  return importSources.some(importSource => mentionsImportSource(sourceCode, importSource));
}
