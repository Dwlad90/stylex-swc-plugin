#!/usr/bin/env node

/**
 * Post-build script to inject types into dist/index.d.ts
 */

import path from 'node:path';
import { fileURLToPath } from 'node:url';
import fs from 'node:fs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const indexDtsPath = path.join(__dirname, '../dist/index.d.ts');

console.log('Injecting types into dist/index.d.ts...');

// === Update index.d.ts ===
let dtsContent = fs.readFileSync(indexDtsPath, 'utf8');

// Check if already injected (after validating file is readable and has expected content)
if (dtsContent.includes('UseLayersType')) {
  console.log('✓ Types already injected, skipping...');
  process.exit(0);
}

// Wrapper code to inject
const wrapperCode = `
// === Types (injected by scripts/inject-types.mjs) ===

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

// === End Types ===
`;

dtsContent = `
${dtsContent}

${wrapperCode}
`;

// Validate that the types were injected
if (!dtsContent.includes('UseLayersType')) {
  console.error('Failed to inject types into index.d.ts');
  process.exit(1);
}

// Write back
fs.writeFileSync(indexDtsPath, dtsContent, 'utf8');
console.log('✓ Successfully injected types into dist/index.d.ts');

console.log('\n✅ Types injection complete!');
