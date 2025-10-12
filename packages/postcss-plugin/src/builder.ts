import postcss, { AtRule } from 'postcss';
import path from 'node:path';
import fs from 'node:fs';
import { normalize, resolve } from 'path';
import { globSync } from 'fast-glob';
import isGlob from 'is-glob';
import globParent from 'glob-parent';
import createBundler from './bundler';
import { shouldTransformFile } from '@stylexswc/rs-compiler';

import type { StyleXPluginOption, TransformOptions } from './types';

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
function parseDependency(fileOrGlob: string) {
  // License: MIT
  // Based on:
  // https://github.com/chakra-ui/panda/blob/6ab003795c0b076efe6879a2e6a2a548cb96580e/packages/node/src/parse-dependency.ts
  if (fileOrGlob.startsWith('!')) {
    return null;
  }

  let message = null;

  if (isGlob(fileOrGlob)) {
    const { base, glob } = parseGlob(fileOrGlob);
    message = { type: 'dir-dependency', dir: normalize(resolve(base)), glob };
  } else {
    message = { type: 'dependency', file: normalize(resolve(fileOrGlob)) };
  }

  return message;
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

    // Use fast-glob with glob patterns for initial discovery
    const globExclude = (exclude || []).filter(isGlobPattern).map(p => String(p));
    let files = globSync(globPatterns.length > 0 ? globPatterns : [], {
      onlyFiles: true,
      ignore: globExclude,
      cwd,
    });

    // Normalize file paths
    files = files.map(file => (file.includes(cwd || '/') ? file : path.resolve(cwd || '/', file)));

    // If there are regex patterns, filter using shouldTransformFile
    if (hasRegexPatterns) {
      files = files.filter(file => shouldTransformFile(file, include, exclude));
    }

    return files;
  }

  // Transforms the included files, bundles the CSS, and returns the result.
  function build({ shouldSkipTransformError }: TransformOptions) {
    const { cwd, rsOptions, useCSSLayers, isDev } = getConfig();

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

    const css = bundler.bundle({
      useCSSLayers,
      enableLTRRTLComments: rsOptions?.enableLTRRTLComments,
    });
    return css;
  }

  // Retrieves the dependencies that PostCSS should watch.
  function getDependencies() {
    const { include } = getConfig();
    const dependencies: Awaited<ReturnType<typeof parseDependency>>[] = [];

    for (const fileOrGlob of include || []) {
      const fileOrGlobString = fileOrGlob.toString();

      const dependency = parseDependency(fileOrGlobString);
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
