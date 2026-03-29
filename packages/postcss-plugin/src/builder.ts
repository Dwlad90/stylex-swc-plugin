import postcss, { AtRule } from 'postcss';
import path from 'node:path';
import fs from 'node:fs';
import { normalize, resolve } from 'path';
import { globSync } from 'fast-glob';
import isGlob from 'is-glob';
import globParent from 'glob-parent';
import createBundler from './bundler';
import { shouldTransformFile, TransformedOptions } from '@stylexswc/rs-compiler';

import type { StyleXPluginOption, TransformOptions } from './types';

const NODE_MODULES_CATCH_ALL_EXCLUDE_PATTERNS = new Set([
  'node_modules/**',
  '**/node_modules/**',
]);

// Parses a glob pattern and extracts its base directory and pattern.
// Returns an object with `base` and `glob` properties.
function parseGlob(pattern: string) {
  // License: MIT
  // Based on:
  // https://github.com/chakra-ui/panda/blob/6ab003795c0b076efe6879a2e6a2a548cb96580e/packages/node/src/parse-glob.ts
  let glob = pattern;
  const base = globParent(pattern);

  if (base !== '.') {
    glob = pattern.substring(base.length);
    if (glob.charAt(0) === '/') {
      glob = glob.substring(1);
    }
  }

  if (glob.substring(0, 2) === './') {
    glob = glob.substring(2);
  }
  if (glob.charAt(0) === '/') {
    glob = glob.substring(1);
  }

  return { base, glob };
}

// Parses a file path or glob pattern into a PostCSS dependency message.
function parseDependency(fileOrGlob: string, cwd: string) {
  // License: MIT
  // Based on:
  // https://github.com/chakra-ui/panda/blob/6ab003795c0b076efe6879a2e6a2a548cb96580e/packages/node/src/parse-dependency.ts
  if (fileOrGlob.startsWith('!')) {
    return null;
  }

  let message:
    | { type: 'dir-dependency'; dir: string; glob: string }
    | { type: 'dependency'; file: string };

  if (isGlob(fileOrGlob)) {
    const { base, glob } = parseGlob(fileOrGlob);
    message = {
      type: 'dir-dependency',
      dir: normalize(resolve(cwd, base)),
      glob,
    };
  } else {
    message = { type: 'dependency', file: normalize(resolve(cwd, fileOrGlob)) };
  }

  return message;
}

function normalizeGlobPattern(pattern: string | RegExp): string {
  return String(pattern).replace(/\\/g, '/').replace(/^\.\//, '');
}

function isNodeModulesCatchAllExcludePattern(pattern: string | RegExp): boolean {
  return NODE_MODULES_CATCH_ALL_EXCLUDE_PATTERNS.has(normalizeGlobPattern(pattern));
}

function toCanonicalFilePath(file: string, cwd: string): string {
  return normalize(resolve(cwd, file));
}

/**
 * When an absolute include pattern points into node_modules, we remove the broad
 * catch-all excludes (node_modules/** and **\/node_modules/**) so the specific
 * package path can be matched. However that also allows deeply nested
 * node_modules within the discovered package (e.g. transitive deps that
 * themselves have their own node_modules) to be scanned.
 *
 * This helper computes a *specific* absolute exclude pattern that prevents
 * scanning the nested node_modules inside the matched package's base directory,
 * while still letting the package's own source files through.
 *
 * The pattern must be absolute because fast-glob does not apply relative ignore
 * patterns to absolute match results (which occur when the include pattern is
 * itself absolute).
 *
 * Example:
 *   includePattern = '/app/node_modules/@acme/ui/**\/*.{ts,tsx}'
 *   → '/app/node_modules/@acme/ui/node_modules/**'
 */
function nestedNodeModulesExcludeFor(includePattern: string): string | null {
  if (!path.isAbsolute(includePattern)) {
    return null;
  }

  // globParent gives the base directory before the glob wildcards begin.
  // e.g. '/app/node_modules/@acme/ui/**/*.ts' → '/app/node_modules/@acme/ui'
  const baseDir = globParent(includePattern);

  // Return an absolute ignore pattern so fast-glob matches it correctly
  // when the include pattern is absolute (relative ignore patterns are not
  // applied to absolute match results).
  return `${baseDir.split(path.sep).join('/')}/node_modules/**`;
}

// Creates a builder for transforming files and bundling StyleX CSS.
function createBuilder() {
  let config: StyleXPluginOption | null = null;

  const bundler = createBundler();

  const fileModifiedMap = new Map();

  // Configures the builder with the provided options.
  function configure(options: StyleXPluginOption) {
    config = options;
  }

  /// Retrieves the current configuration.
  function getConfig() {
    if (config == null) {
      throw new Error('Builder not configured');
    }

    return config;
  }

  // Finds the `@stylex;` at-rule in the provided PostCSS root.
  function findStyleXAtRule(root: postcss.Root): AtRule | null {
    let styleXAtRule: AtRule | null = null;

    root.walkAtRules(atRule => {
      if (atRule.name === 'stylex' && !atRule.params) {
        styleXAtRule = atRule;
      }
    });

    return styleXAtRule;
  }

  // Retrieves all files that match the include and exclude patterns.
  function getFiles() {
    const { cwd, include, exclude } = getConfig();

    // Separate glob patterns from regex patterns
    // String patterns that don't look like regex are treated as globs
    const isRegexPattern = (p: string | RegExp) =>
      p instanceof RegExp || (typeof p === 'string' && /^\/.*\/[gimsuy]*$/.test(p));
    const isGlobPattern = (p: string | RegExp) => !isRegexPattern(p);

    const globPatterns = (include || []).filter(isGlobPattern).map(p => String(p));
    const hasRegexPatterns =
      (include || []).some(isRegexPattern) || (exclude || []).some(isRegexPattern);

    if (globPatterns.length === 0 && !hasRegexPatterns) {
      return [];
    }

    // Use fast-glob with glob patterns for initial discovery
    const globExclude = (exclude || []).filter(isGlobPattern).map(p => String(p));

    const ignoreWithoutNodeModulesCatchAll = globExclude.filter(
      (pattern) => !isNodeModulesCatchAllExcludePattern(pattern),
    );

    const files = new Set<string>();
    for (const includePattern of globPatterns) {
      const isAbsolutePattern = path.isAbsolute(includePattern);
      const pointsToNodeModules = /(^|[/\\])node_modules([/\\]|$)/.test(
        includePattern,
      );

      let ignore: string[];
      if (isAbsolutePattern && pointsToNodeModules) {
        // Remove broad catch-all patterns so this specific node_modules path
        // can be matched, but add back a targeted exclude for any nested
        // node_modules *within* the matched package to avoid scanning
        // transitive dependencies' source files.
        const nestedExclude = nestedNodeModulesExcludeFor(includePattern);
        ignore = [
          ...ignoreWithoutNodeModulesCatchAll,
          ...(nestedExclude != null ? [nestedExclude] : []),
        ];
      } else {
        ignore = globExclude;
      }

      const matchedFiles = globSync(includePattern, {
        onlyFiles: true,
        ignore,
        cwd,
      });
      for (const file of matchedFiles) {
        files.add(toCanonicalFilePath(file, cwd || '/'));
      }
    }

    let result = Array.from(files);

    // If there are regex patterns, filter using shouldTransformFile
    if (hasRegexPatterns) {
      result = result.filter(file => shouldTransformFile(file, include, exclude));
    }

    return result;
  }

  // Transforms the included files, bundles the CSS, and returns the result.
  function build({ shouldSkipTransformError }: TransformOptions) {
    const { cwd, rsOptions, useCSSLayers, isDev } = getConfig();

    const transformedOptions: TransformedOptions = {
      useLayers: useCSSLayers,
      enableLTRRTLComments: rsOptions?.enableLTRRTLComments,
      legacyDisableLayers: rsOptions?.legacyDisableLayers,
    };

    const files = getFiles();
    const filesToTransform = [];

    // Remove deleted files since the last build
    for (const file of fileModifiedMap.keys()) {
      if (!files.includes(file)) {
        fileModifiedMap.delete(file);
        bundler.remove(file);
      }
    }

    for (const file of files) {
      const filePath = path.resolve(cwd || '/', file);
      const mtimeMs = fs.existsSync(filePath) ? fs.statSync(filePath).mtimeMs : -Infinity;

      // Skip files that have not been modified since the last build
      // On first run, all files will be transformed
      const shouldSkip = fileModifiedMap.has(file) && mtimeMs === fileModifiedMap.get(file);

      if (shouldSkip) {
        continue;
      }

      fileModifiedMap.set(file, mtimeMs);
      filesToTransform.push(file);
    }

    filesToTransform.forEach(file => {
      const filePath = path.resolve(cwd || '/', file);
      const contents = fs.readFileSync(filePath, 'utf-8');
      if (!bundler.shouldTransform(contents, rsOptions)) {
        return;
      }

      if (rsOptions) {
        // @ts-expect-error - this field is omitted from the type for postcss plugin
        rsOptions.include = undefined;
        // @ts-expect-error - this field is omitted from the type for postcss plugin
        rsOptions.exclude = undefined;
      }

      const transformedResult = bundler.transform(filePath, contents, rsOptions || {}, {
        isDev,
        shouldSkipTransformError,
      });

      return transformedResult;
    });

    const css = bundler.bundle(transformedOptions);
    return css;
  }

  // Retrieves the dependencies that PostCSS should watch.
  function getDependencies() {
    const { include, cwd } = getConfig();
    const dependencies: Awaited<ReturnType<typeof parseDependency>>[] = [];

    for (const fileOrGlob of include || []) {
      const fileOrGlobString = fileOrGlob.toString();

      const dependency = parseDependency(fileOrGlobString, cwd || process.cwd());
      if (dependency != null) {
        dependencies.push(dependency);
      }
    }

    return dependencies;
  }

  return {
    findStyleXAtRule,
    configure,
    build,
    getDependencies,
  };
}

export default createBuilder;
