/**
 * Write `src/process-stylex-rules/reference.ts` again from the installed
 * `@stylexjs/babel-plugin`.
 *
 * Run this after a version bump of the plugin. `vendoredReference.spec.ts`
 * fails until the file is written again, so that the fallback of
 * `processStylexRules` always runs the version that the parity suites compare
 * with. `parity/lib/vendored-reference.ts` says what the copy holds.
 *
 * Usage:
 *   pnpm vendor:reference
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { renderVendoredReference, VENDORED_REFERENCE_FILE } from './lib/vendored-reference.js';

const packageDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const outputPath = path.join(packageDir, VENDORED_REFERENCE_FILE);

fs.mkdirSync(path.dirname(outputPath), { recursive: true });
fs.writeFileSync(outputPath, renderVendoredReference(), 'utf8');
console.log(`Wrote ${path.relative(packageDir, outputPath)}`);
