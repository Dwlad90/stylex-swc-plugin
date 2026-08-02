// Semantic validation for JSON and JSONC files.
//
// Oxlint does not discover `.json` files at all ("No files found to lint"), so
// `eslint-plugin-jsonc` has nothing to run against and cannot be bridged. Oxfmt
// parses these files and therefore catches syntax errors, but it accepts
// duplicate keys silently — a formatted `{ "a": 1, "a": 2 }` passes
// `oxfmt --check`. Duplicate keys are the failure that actually bites: the last
// value wins, so a stale entry can override a corrected one with no signal.
//
// This covers the two checks worth having until Oxlint grows JSON support.

import { execSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';

const files = execSync("git ls-files '*.json' '*.jsonc'", { encoding: 'utf8' })
  .split('\n')
  .filter(f => f && !f.endsWith('pnpm-lock.yaml'));

/** Walk the raw text, because JSON.parse silently keeps only the last duplicate. */
function findDuplicateKeys(source) {
  const duplicates = [];
  const stack = [new Set()];
  let inString = false;
  let escaped = false;
  let current = '';
  let capturing = false;
  let line = 1;

  for (let i = 0; i < source.length; i++) {
    const char = source[i];
    if (char === '\n') line++;

    if (inString) {
      if (escaped) escaped = false;
      else if (char === '\\') escaped = true;
      else if (char === '"') {
        inString = false;
        if (capturing) {
          // Only a key if the next non-space character is a colon.
          const rest = source.slice(i + 1).match(/^\s*:/);
          if (rest) {
            const scope = stack[stack.length - 1];
            if (scope.has(current)) duplicates.push({ key: current, line });
            else scope.add(current);
          }
        }
        capturing = false;
        current = '';
      } else if (capturing) current += char;
      continue;
    }

    if (char === '"') {
      inString = true;
      capturing = true;
      current = '';
    } else if (char === '{') stack.push(new Set());
    else if (char === '}') {
      if (stack.length > 1) stack.pop();
    } else if (char === '/' && source[i + 1] === '/') {
      while (i < source.length && source[i] !== '\n') i++;
      line++;
    } else if (char === '/' && source[i + 1] === '*') {
      i += 2;
      while (i < source.length && !(source[i] === '*' && source[i + 1] === '/')) {
        if (source[i] === '\n') line++;
        i++;
      }
      i++;
    }
  }
  return duplicates;
}

const problems = [];

for (const file of files) {
  // `git ls-files` also reports intent-to-add entries whose file may be gone.
  if (!existsSync(file)) continue;
  const source = readFileSync(file, 'utf8');

  // Strict JSON files must parse; JSONC may carry comments and trailing commas.
  if (file.endsWith('.json') && !/\.(jsonc)$/.test(file)) {
    try {
      JSON.parse(source);
    } catch (error) {
      // tsconfig and VS Code files are JSONC in practice despite the extension.
      const jsoncByConvention = /(^|\/)(tsconfig[^/]*\.json|\.vscode\/[^/]+\.json)$/.test(file);
      if (!jsoncByConvention) problems.push(`${file}: invalid JSON — ${error.message}`);
    }
  }

  for (const { key, line } of findDuplicateKeys(source)) {
    problems.push(`${file}:${line}: duplicate key "${key}" — the later value silently wins`);
  }
}

if (problems.length > 0) {
  console.error(`JSON check failed with ${problems.length} problem(s):`);
  for (const problem of problems) console.error(`  - ${problem}`);
  process.exit(1);
}

console.log(`JSON check passed: ${files.length} files, no duplicate keys or parse errors.`);
