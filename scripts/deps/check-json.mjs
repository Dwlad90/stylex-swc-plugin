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

// `-z` rather than the default output: without it `git ls-files` C-quotes any
// path containing non-ASCII or special characters, which `existsSync` below
// then rejects — so the file would be skipped silently and the gate would
// report success over a file it never opened. `maxBuffer` is raised because
// the default 1 MB is a hard throw, not a truncation, on a large repo.
const files = execSync('git ls-files -z -- "*.json" "*.jsonc"', {
  encoding: 'utf8',
  maxBuffer: 64 * 1024 * 1024,
})
  .split('\0')
  .filter(Boolean);

/** Decoded values of the single-character JSON escapes; others decode to themselves. */
const JSON_ESCAPES = { b: '\b', f: '\f', n: '\n', r: '\r', t: '\t' };

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
      if (escaped) {
        escaped = false;
        // Decode rather than drop. Skipping the escaped character entirely
        // made `"a\nb"` and `"ab"` both capture as `ab`, so a file holding
        // both keys was reported as a duplicate — a false failure in a gate
        // that blocks every commit. Decoding also keeps `"b"` and `"b"`
        // comparing equal, which is what JSON says they are.
        if (capturing) {
          const hex = char === 'u' ? source.slice(i + 1, i + 5) : null;
          if (hex !== null && /^[0-9a-fA-F]{4}$/.test(hex)) {
            // A `\uXXXX` escape is a single UTF-16 code unit, so this is
            // always < 0x10000; a surrogate pair arrives as two escapes and
            // reassembles correctly through concatenation.
            current += String.fromCodePoint(parseInt(hex, 16));
            i += 4;
          } else {
            current += JSON_ESCAPES[char] ?? char;
          }
        }
      } else if (char === '\\') escaped = true;
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
      // Only count a newline that is actually there: a trailing `// comment`
      // on the last line has none, and counting it shifted every subsequent
      // reported line number by one.
      if (i < source.length) line++;
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
let checked = 0;

for (const file of files) {
  // `git ls-files` also reports intent-to-add entries whose file may be gone.
  if (!existsSync(file)) continue;
  const source = readFileSync(file, 'utf8');
  checked++;

  // Strict JSON files must parse; JSONC may carry comments and trailing commas.
  // `.jsonc` needs no second test: a path ending in `.jsonc` cannot also end
  // in `.json`.
  if (file.endsWith('.json')) {
    try {
      JSON.parse(source);
    } catch (error) {
      // tsconfig and VS Code files are JSONC in practice despite the extension.
      const jsoncByConvention = /(^|\/)(tsconfig[^/]*\.json|\.vscode\/[^/]+\.json)$/.test(file);
      const reason = error instanceof Error ? error.message : String(error);
      if (!jsoncByConvention) problems.push(`${file}: invalid JSON — ${reason}`);
    }
  }

  for (const { key, line } of findDuplicateKeys(source)) {
    problems.push(`${file}:${line}: duplicate key "${key}" — the later value silently wins`);
  }
}

if (problems.length > 0) {
  console.error(`JSON check failed with ${problems.length} problem(s):`);
  for (const problem of problems) console.error(`  - ${problem}`);
  // `process.exitCode`, not `process.exit`. When stderr is a pipe — which it
  // is under every CI runner and under `2>&1 | tee` — writes are asynchronous,
  // and `process.exit` tears the process down without flushing them. The gate
  // would then fail with its reasons truncated or missing entirely.
  process.exitCode = 1;
} else {
  console.log(`JSON check passed: ${checked} files, no duplicate keys or parse errors.`);
}
