/**
 * Presentation helpers shared by every benchmark entry point.
 *
 * Previously `bench.ts` and `bench-compare.ts` each carried their own copy
 * of the latency formatter with slightly different unit rules (ns/µs/ms vs
 * µs/ms/s). Keeping one formatter — and one Markdown escaper — avoids drift
 * between the human-readable outputs and the CI job summaries.
 */

export function formatLatency(milliseconds: number): string {
  if (!Number.isFinite(milliseconds)) return 'n/a';

  const nanoseconds = milliseconds * 1_000_000;

  if (nanoseconds >= 1_000_000_000) {
    return `${(nanoseconds / 1_000_000_000).toLocaleString('en-US', {
      maximumFractionDigits: 2,
    })} s`;
  }

  if (nanoseconds >= 1_000_000) {
    return `${(nanoseconds / 1_000_000).toLocaleString('en-US', {
      maximumFractionDigits: 2,
    })} ms`;
  }

  if (nanoseconds >= 1_000) {
    return `${(nanoseconds / 1_000).toLocaleString('en-US', {
      maximumFractionDigits: 2,
    })} µs`;
  }

  return `${nanoseconds.toLocaleString('en-US', {
    maximumFractionDigits: 0,
  })} ns`;
}

/**
 * Markdown-safe cell escaping for `GITHUB_STEP_SUMMARY` tables. Pipes and
 * backticks would break the surrounding table; control characters are
 * stripped because they render as visible glyphs on the GitHub UI.
 *
 * Every renderer that puts artifact-derived values into Markdown must use
 * this — fixture names and subject labels cross a trust boundary.
 */
export function escapeMarkdownCell(value: string): string {
  return value
    .replace(/\\/g, '\\\\')
    .replace(/\|/g, '\\|')
    .replace(/`/g, '\\`')
    .replace(/\p{Cc}/gu, ' ');
}

/** Build one Markdown table row from already escaped cells. */
export function markdownTableRow(cells: readonly string[]): string {
  return `| ${cells.join(' | ')} |`;
}
