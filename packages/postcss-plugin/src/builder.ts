import postcss, { AtRule } from 'postcss';
import path from 'node:path';
import fs from 'node:fs';
import { normalize, resolve } from 'path';
import { globSync } from 'fast-glob';
import isGlob from 'is-glob';
import globParent from 'glob-parent';
import createBundler from './bundler';

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
    let files = globSync(include || [], {
      onlyFiles: true,
      ignore: exclude,
      cwd,
    });

    // Normalize file paths
    files = files.map(file => (file.includes(cwd || '/') ? file : path.join(cwd || '/', file)));

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
      if (!bundler.shouldTransform(contents)) {
        console.log('contents: ', contents);
        return;
      }
      const transformedResult = bundler.transform(file, contents, rsOptions || {}, {
        isDev,
        shouldSkipTransformError,
      });

      // if (transformedResult.code !== contents) {
      //   fs.writeFileSync(filePath, transformedResult.code, 'utf-8');
      // }

      return transformedResult;
    });

    const css = bundler.bundle({ useCSSLayers });
    return css;
  }

  // Retrieves the dependencies that PostCSS should watch.
  function getDependencies() {
    const { include } = getConfig();
    const dependencies = [];

    for (const fileOrGlob of include || []) {
      const dependency = parseDependency(fileOrGlob);
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
