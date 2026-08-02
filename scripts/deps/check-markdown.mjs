// Structural validation for GitHub alerts in Markdown.
//
// GitHub renders `> [!NOTE]` and friends as a coloured callout, but only when
// the marker sits alone on the first line of the blockquote and names one of
// the five supported types in uppercase. Break any of those and GitHub silently
// falls back to a plain grey quote with a literal `[!NOTE]` in the body.
//
// Nothing else catches this. `oxfmt --check` is happy either way — a merged
// marker was its own output, back when `proseWrap` was `always` for Markdown
// (see the `*.md` override in `.oxfmtrc.json`, which is now `preserve` for
// exactly this reason). The only other signal is a human noticing the rendered
// README looks wrong, which is how 29 of these accumulated unnoticed.

import { execSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

/** The complete set GitHub recognises; anything else renders as a plain quote. */
const ALERT_TYPES = new Set(['NOTE', 'TIP', 'IMPORTANT', 'WARNING', 'CAUTION']);

/** A blockquote line opening with a `[!...]` marker, whatever the spelling. */
const MARKER_LINE = /^(\s*)>\s*\[!([^\]]*)\](.*)$/;

/** Any blockquote line, used to tell whether a marker opens its quote. */
const QUOTE_LINE = /^\s*>/;

/** Opens or closes a fenced code block; alerts inside one are documentation. */
const FENCE = /^\s*(```|~~~)/;

const MARKDOWN_EXTENSIONS = new Set(['.md', '.mdx']);

/**
 * Every tracked Markdown file.
 *
 * `-z` for the same reason as `check-json.mjs`: without it `git ls-files`
 * C-quotes unusual paths, which then fail to open and get skipped in silence.
 */
const trackedMarkdown = () =>
  execSync('git ls-files -z -- "*.md" "*.mdx"', {
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
  })
    .split('\0')
    .filter(Boolean);

// Paths may be passed explicitly, which is how the pre-commit hook runs this
// against just the staged files instead of re-reading every README in the
// repository. With no arguments — the CI path, via `pnpm lint:markdown` — the
// whole tree is checked, so a file that regresses without being touched is
// still caught. Non-Markdown arguments are dropped rather than parsed as prose.
const explicit = process.argv.slice(2);
const files =
  explicit.length > 0
    ? explicit.filter(file => MARKDOWN_EXTENSIONS.has(path.extname(file).toLowerCase()))
    : trackedMarkdown();

const problems = [];
let checked = 0;

for (const file of files) {
  // `git ls-files` also reports intent-to-add entries whose file may be gone.
  if (!existsSync(file)) continue;

  const lines = readFileSync(file, 'utf8').split('\n');
  checked++;

  let inFence = false;
  let previousWasQuote = false;

  for (const [index, line] of lines.entries()) {
    if (FENCE.test(line)) {
      inFence = !inFence;
      previousWasQuote = false;
      continue;
    }

    if (inFence) continue;

    const match = line.match(MARKER_LINE);
    if (!match) {
      previousWasQuote = QUOTE_LINE.test(line);
      continue;
    }

    const [, , type, rest] = match;
    // Relative, because the hook passes absolute paths and an absolute prefix
    // on every line makes the report hard to scan.
    const where = `${path.relative(process.cwd(), file) || file}:${index + 1}`;

    if (!ALERT_TYPES.has(type)) {
      problems.push(
        `${where}: unknown alert type "[!${type}]" — GitHub renders a plain quote. ` +
          `Expected one of ${[...ALERT_TYPES].join(', ')}.`
      );
    } else if (rest.trim() !== '') {
      problems.push(
        `${where}: "[!${type}]" must be alone on its line — GitHub renders a plain ` +
          `quote otherwise. Move the text to the next line, prefixed with "> ".`
      );
    } else if (previousWasQuote) {
      problems.push(
        `${where}: "[!${type}]" must open its blockquote — GitHub ignores a marker ` +
          `that follows other quoted lines. Add a blank line above it.`
      );
    }

    previousWasQuote = true;
  }
}

if (problems.length > 0) {
  console.error(`Markdown check failed with ${problems.length} problem(s):`);
  for (const problem of problems) console.error(`  - ${problem}`);
  // `process.exitCode`, not `process.exit`: under CI the streams are pipes and
  // written asynchronously, so exiting outright can truncate the list above.
  process.exitCode = 1;
} else {
  console.log(`Markdown check passed: ${checked} files, all GitHub alerts well-formed.`);
}
