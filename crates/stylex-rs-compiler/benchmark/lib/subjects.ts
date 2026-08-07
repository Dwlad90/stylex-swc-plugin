/**
 * Subject loading.
 *
 * A subject is anything that turns a fixture into a StyleX rule count.
 * `loadSubject` handles the primary case — an on-disk `@stylexswc/rs-compiler`
 * dist bundle imported by absolute file URL so each subject keeps its own
 * module resolution scope. `createSubject` covers everything else (Babel,
 * shim runners, tests) without pretending to load a NAPI addon.
 */

import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import type { StyleXOptions } from '../../dist/index.js';
import type { FixtureDescriptor, SubjectDescriptor } from './types.js';

/** Runs a fixture and returns the number of StyleX rules produced. */
export type SubjectRun = (fixture: FixtureDescriptor, options: StyleXOptions) => number;

export interface LoadedSubject {
  descriptor: SubjectDescriptor;
  run: SubjectRun;
}

export interface LoadSubjectOptions {
  label: string;
  /**
   * Absolute path to the package directory whose `dist/index.js` is the
   * subject's entry and whose `package.json` supplies the version.
   */
  packageDir: string;
}

type TransformFn = (
  filename: string,
  code: string,
  options: StyleXOptions
) => { metadata: { stylex: unknown[] }; code: string };

/**
 * Load an `@stylexswc/rs-compiler`-shaped subject from an on-disk package.
 *
 * The loader imports the entry via a `file://` URL so the runtime resolves
 * it exactly like `import`, giving each subject its own resolution scope so
 * two independently built NAPI bindings can coexist in one process.
 */
export async function loadSubject(options: LoadSubjectOptions): Promise<LoadedSubject> {
  const entry = path.join(options.packageDir, 'dist/index.js');
  if (!fs.existsSync(entry)) {
    throw new Error(`Subject "${options.label}" entry does not exist: ${entry}`);
  }

  const loaded = (await import(pathToFileURL(entry).href)) as { transform?: TransformFn };
  const transform = loaded.transform;
  if (typeof transform !== 'function') {
    throw new Error(`Subject "${options.label}" does not export a transform function`);
  }

  return {
    descriptor: {
      label: options.label,
      version: readPackageVersion(options.packageDir),
      resolvedFrom: entry,
    },
    run(fixture, stylexOptions) {
      const { metadata } = transform(fixture.filePath, fixture.code, stylexOptions);
      return metadata.stylex.length;
    },
  };
}

/**
 * Construct a subject manually. Used by `bench-compare.ts` for the Babel
 * runner (not a NAPI addon) and by tests.
 */
export function createSubject(descriptor: SubjectDescriptor, run: SubjectRun): LoadedSubject {
  return { descriptor, run };
}

/**
 * Read the version string from a package.json. Tolerant on purpose: a
 * missing or malformed manifest is legitimate for temporary base builds
 * and should not crash the harness before the benchmark runs.
 */
export function readPackageVersion(packageDir: string): string {
  const manifest = path.join(packageDir, 'package.json');
  try {
    const raw = JSON.parse(fs.readFileSync(manifest, 'utf-8')) as { version?: string };
    return raw.version ?? 'unknown';
  } catch {
    return 'unknown';
  }
}
