#!/usr/bin/env node

/**
 * Post-build script to inject SWC plugin wrapper into dist/index.js
 */

import path from 'node:path'
import { fileURLToPath } from 'node:url'
import fs from 'node:fs'

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const indexJsPath = path.join(__dirname, '../dist/index.js');

console.log('Injecting SWC plugin wrapper into dist/index.js...');

// === Update index.js ===
let jsContent = fs.readFileSync(indexJsPath, 'utf8');

// Find the exports section - napi v3 uses module.exports.EXPORT = nativeBinding.EXPORT format
const transformExportRegex = /module\.exports\.transform\s*=\s*nativeBinding\.transform/;
const exportsMatch = jsContent.match(transformExportRegex);
if (!exportsMatch) {
  console.error('Could not find exports section in index.js');
  process.exit(1);
}

// Check if already injected (after validating file is readable and has expected content)
if (jsContent.includes('transformWithPlugins')) {
  console.log('✓ SWC plugin wrapper already injected, skipping...');
  process.exit(0);
}

// Check that all required exports are present
const requiredExports = ['SourceMaps', 'transform', 'shouldTransformFile', 'normalizeRsOptions'];
const missingExports = requiredExports.filter(exportName => 
  !jsContent.includes(`module.exports.${exportName}`)
);
if (missingExports.length > 0) {
  console.error('Missing required exports in index.js:', missingExports.join(', '));
  process.exit(1);
}

// Wrapper code to inject
const wrapperCode = `
// === SWC Plugin Wrapper (injected by scripts/inject-swc-plugins.mjs) ===

const nativeTransform = nativeBinding.transform;

/**
 * Wrapper around transform that supports SWC plugins
 * If options.swcPlugins is provided, runs SWC plugins first, then StyleX transform
 */
function transformWithPlugins(filename, code, options) {
  let transformedCode = code;

  if (options.swcPlugins && Array.isArray(options.swcPlugins) && options.swcPlugins.length > 0) {
    try {
      // Lazy-load @swc/core only if plugins are used
      const swc = require('@swc/core');

      const swcOptions = {
        filename,
        sourceMaps: options.sourceMap === 'Inline' ? 'inline' : Boolean(options.sourceMap),
        jsc: {
          parser: {
            syntax: 'typescript',
            tsx: true,
          },
          target: 'es2022',
          experimental: {
            plugins: options.swcPlugins,
          },
        },
      };

      const result = swc.transformSync(transformedCode, swcOptions);
      transformedCode = result.code;
    } catch (error) {
      console.error(\`✗ SWC plugin execution failed for \${filename}:\`, error);
      throw error;
    }
  }

  const { swcPlugins: _removed, ...stylexOptions } = options;

  return nativeTransform(filename, transformedCode, stylexOptions);
}

// === End SWC Plugin Wrapper ===
`;

// Replace the module.exports.transform section
// Find the line with module.exports.transform and insert wrapper before it
const transformExportLineRegex = /^module\.exports\.transform\s*=\s*nativeBinding\.transform\s*;?$/m;
if (!transformExportLineRegex.test(jsContent)) {
  console.error('Could not find "module.exports.transform = nativeBinding.transform" in index.js');
  process.exit(1);
}
jsContent = jsContent.replace(transformExportLineRegex, wrapperCode + '\nmodule.exports.transform = transformWithPlugins;');
// Validate that the wrapper was injected
if (!jsContent.includes('transformWithPlugins')) {
  console.error('Failed to inject SWC plugin wrapper into index.js');
  process.exit(1);
}

// Write back
fs.writeFileSync(indexJsPath, jsContent, 'utf8');
console.log('✓ Successfully injected SWC plugin wrapper into dist/index.js');

console.log('\n✅ SWC plugin injection complete!');
