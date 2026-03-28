import path from 'node:path';
import fs from 'node:fs';
import { createRequire } from 'node:module';

import type { StyleXPluginOption } from './types';

type ImportSource = string | { from: string; as?: string };

export const DEFAULT_IMPORT_SOURCES: string[] = ['@stylexjs/stylex', 'stylex'];

const DEFAULT_IMPORT_SOURCE_PACKAGES = new Set(
  DEFAULT_IMPORT_SOURCES.map((source) => {
    if (source.startsWith('@')) {
      const parts = source.split('/');
      const scope = parts[0];
      const name = parts[1];
      return scope != null && name != null ? `${scope}/${name}` : null;
    }
    const packageName = source.split('/')[0];
    return packageName ?? null;
  }).filter(Boolean) as string[]
);

export const DEFAULT_INCLUDE_GLOB = '**/*.{js,jsx,mjs,cjs,ts,tsx,mts,cts}';

// Keep auto-discovery focused on source files.
// Explicit include values from users are always respected.
export const AUTO_DISCOVERY_EXCLUDES = [
  'node_modules/**',
  '**/node_modules/**',
  '.git/**',
  '.next/**',
  '.nuxt/**',
  '.svelte-kit/**',
  '.turbo/**',
  '.cache/**',
  'dist/**',
  'build/**',
  'coverage/**',
  'tmp/**',
  'temp/**',
];

function toArray<T>(value: T | T[] | undefined | null): T[] {
  if (value == null) {
    return [];
  }
  return Array.isArray(value) ? value : [value];
}

function dedupe<T>(items: T[]): T[] {
  return Array.from(new Set(items));
}

function readJSON(file: string): Record<string, unknown> | null {
  try {
    const content = fs.readFileSync(file, 'utf8');
    return JSON.parse(content) as Record<string, unknown>;
  } catch {
    return null;
  }
}

function toPackageName(importSource: ImportSource): string | null {
  const source =
    typeof importSource === 'string'
      ? importSource
      : (importSource as { from?: string })?.from;

  if (source == null || source.startsWith('.') || source.startsWith('/')) {
    return null;
  }

  if (source.startsWith('@')) {
    const parts = source.split('/');
    const scope = parts[0];
    const name = parts[1];
    if (scope != null && name != null) {
      return `${scope}/${name}`;
    }
    return null;
  }

  const packageName = source.split('/')[0];
  return packageName ?? null;
}

function hasStylexDependency(
  manifest: Record<string, unknown> | null,
  targetPackages: Set<string>
): boolean {
  if (manifest == null || typeof manifest !== 'object') {
    return false;
  }

  const dependencyFields = ['dependencies', 'peerDependencies', 'optionalDependencies'];

  return dependencyFields.some((field) => {
    const deps = manifest[field];
    if (deps == null || typeof deps !== 'object') {
      return false;
    }
    return Object.keys(deps as Record<string, unknown>).some((depName) =>
      targetPackages.has(depName)
    );
  });
}

function findDependencyManifestPathFromEntry(
  entryPath: string,
  dependencyName: string
): string | null {
  let dir = path.dirname(entryPath);

  for (;;) {
    const candidate = path.join(dir, 'package.json');
    const manifest = readJSON(candidate);
    if (manifest != null && manifest.name === dependencyName) {
      return candidate;
    }

    const parent = path.dirname(dir);
    if (parent === dir) {
      return null;
    }
    dir = parent;
  }
}

function resolveDependencyManifestPath(
  requireFromRoot: NodeRequire,
  dependencyName: string
): string | null {
  try {
    return requireFromRoot.resolve(`${dependencyName}/package.json`);
  } catch {
    // fall through
  }

  try {
    const entryPath = requireFromRoot.resolve(dependencyName);
    return findDependencyManifestPathFromEntry(entryPath, dependencyName);
  } catch {
    // fall through
  }

  return null;
}

function includePackageFromImportSource({
  importSource,
  cwd,
  requireFromRoot,
  discoveredDirectories,
}: {
  importSource: ImportSource;
  cwd: string;
  requireFromRoot: NodeRequire;
  discoveredDirectories: Set<string>;
}) {
  const source =
    typeof importSource === 'string'
      ? importSource
      : (importSource as { from?: string })?.from;
  if (typeof source !== 'string') {
    return;
  }
  if (source.startsWith('.') || source.startsWith('/')) {
    return;
  }

  const packageName = toPackageName(source);
  if (packageName == null) {
    return;
  }
  if (DEFAULT_IMPORT_SOURCE_PACKAGES.has(packageName)) {
    return;
  }

  const manifestPath = resolveDependencyManifestPath(requireFromRoot, packageName);
  if (manifestPath == null) {
    return;
  }

  const directory = path.dirname(manifestPath);
  if (directory !== path.resolve(cwd)) {
    discoveredDirectories.add(directory);
  }
}

function getDirectDependencies(manifest: Record<string, unknown> | null): string[] {
  if (manifest == null || typeof manifest !== 'object') {
    return [];
  }

  const dependencyFields = [
    'dependencies',
    'devDependencies',
    'peerDependencies',
    'optionalDependencies',
  ];

  const dependencies = new Set<string>();
  for (const field of dependencyFields) {
    const deps = manifest[field];
    if (deps == null || typeof deps !== 'object') {
      continue;
    }
    for (const name of Object.keys(deps as Record<string, unknown>)) {
      dependencies.add(name);
    }
  }

  return Array.from(dependencies);
}

function toAbsoluteGlob(directory: string, globPattern: string): string {
  const normalizedDir = path.resolve(directory).split(path.sep).join('/');
  return `${normalizedDir}/${globPattern}`;
}

function discoverStylexPackageDirectories({
  cwd,
  importSources,
}: {
  cwd: string;
  importSources: ImportSource[];
}): string[] {
  const rootPackageJsonPath = path.join(path.resolve(cwd), 'package.json');
  if (!fs.existsSync(rootPackageJsonPath)) {
    return [];
  }

  const rootPackageDir = path.dirname(rootPackageJsonPath);
  const requireFromRoot = createRequire(rootPackageJsonPath);
  const rootManifest = readJSON(rootPackageJsonPath);
  const dependencyNames = getDirectDependencies(rootManifest);

  const targetPackages = new Set(
    (importSources.map(toPackageName).filter(Boolean) as string[]).concat(
      DEFAULT_IMPORT_SOURCES
    )
  );

  const discoveredDirectories = new Set<string>();

  for (const dependencyName of dependencyNames) {
    const manifestPath = resolveDependencyManifestPath(requireFromRoot, dependencyName);
    if (manifestPath == null) {
      continue;
    }

    const manifest = readJSON(manifestPath);
    if (!hasStylexDependency(manifest, targetPackages)) {
      continue;
    }

    const dependencyDir = path.dirname(manifestPath);
    // Avoid accidentally re-scanning the project root in monorepo edge cases.
    if (dependencyDir !== rootPackageDir) {
      discoveredDirectories.add(dependencyDir);
    }
  }

  for (const importSource of importSources) {
    includePackageFromImportSource({
      importSource,
      cwd,
      requireFromRoot,
      discoveredDirectories,
    });
  }

  return Array.from(discoveredDirectories);
}

export interface ImportSourcesResolution {
  importSources: ImportSource[];
  source: 'postcss-option' | 'rs-options' | 'defaults';
}

/**
 * Resolves the effective importSources with metadata about where they came from.
 *
 * Precedence:
 * 1. Explicit PostCSS `importSources` option
 * 2. Inferred from `rsOptions.importSources`
 * 3. Built-in defaults (`@stylexjs/stylex`, `stylex`)
 */
export function resolveImportSourcesWithMetadata({
  importSources,
  rsOptions,
}: {
  importSources?: ImportSource[];
  rsOptions?: StyleXPluginOption['rsOptions'];
}): ImportSourcesResolution {
  // 1. Explicit importSources from PostCSS plugin options
  if (Array.isArray(importSources) && importSources.length > 0) {
    return {
      importSources: dedupe(importSources),
      source: 'postcss-option',
    };
  }

  // 2. Inferred from rsOptions.importSources
  const rsImportSources = rsOptions?.importSources;
  if (Array.isArray(rsImportSources) && rsImportSources.length > 0) {
    return {
      importSources: dedupe([...DEFAULT_IMPORT_SOURCES, ...rsImportSources]),
      source: 'rs-options',
    };
  }

  // 3. Built-in defaults
  return {
    importSources: DEFAULT_IMPORT_SOURCES,
    source: 'defaults',
  };
}

export function resolveImportSources({
  importSources,
  rsOptions,
}: {
  importSources?: ImportSource[];
  rsOptions?: StyleXPluginOption['rsOptions'];
}): ImportSource[] {
  return resolveImportSourcesWithMetadata({ importSources, rsOptions }).importSources;
}

export interface IncludeResolution {
  include: Array<string | RegExp>;
  discoveredDependencyDirectories: string[];
  hasExplicitInclude: boolean;
}

export function resolveIncludeWithMetadata({
  cwd,
  include,
  importSources,
}: {
  cwd: string;
  include?: StyleXPluginOption['include'];
  importSources: ImportSource[];
}): IncludeResolution {
  const normalizedInclude = toArray(include);
  const hasExplicitInclude = normalizedInclude.length > 0;

  if (hasExplicitInclude) {
    return {
      include: dedupe(normalizedInclude),
      discoveredDependencyDirectories: [],
      hasExplicitInclude,
    };
  }

  const discoveredDependencyDirectories = discoverStylexPackageDirectories({
    cwd,
    importSources: importSources.filter(
      (s): s is string | { from: string } => typeof s === 'string' || typeof s === 'object'
    ),
  });

  const discoveredDependencyGlobs = discoveredDependencyDirectories.map((dir) =>
    toAbsoluteGlob(dir, DEFAULT_INCLUDE_GLOB)
  );

  return {
    include: dedupe([DEFAULT_INCLUDE_GLOB, ...discoveredDependencyGlobs]),
    discoveredDependencyDirectories,
    hasExplicitInclude,
  };
}

export function resolveInclude({
  cwd,
  include,
  importSources,
}: {
  cwd: string;
  include?: StyleXPluginOption['include'];
  importSources: ImportSource[];
}): Array<string | RegExp> {
  return resolveIncludeWithMetadata({ cwd, include, importSources }).include;
}

export function resolveExclude({
  include,
  exclude,
}: {
  include?: StyleXPluginOption['include'];
  exclude?: StyleXPluginOption['exclude'];
}): Array<string | RegExp> {
  const normalizedExclude = toArray(exclude);
  const hasExplicitInclude = toArray(include).length > 0;

  if (hasExplicitInclude) {
    return normalizedExclude;
  }

  return dedupe([...AUTO_DISCOVERY_EXCLUDES, ...normalizedExclude]);
}
