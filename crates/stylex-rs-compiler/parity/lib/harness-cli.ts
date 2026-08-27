/**
 * The three things every generated harness does around its comparison.
 *
 * Shared for the reason `flags.ts` next door is shared, and on the same
 * evidence: the second copy of each of these was written by hand, and by the
 * third they had begun to disagree — one harness's refusal cell dropped the
 * second line of a two-line complaint, which is the line that carries the rule.
 * A count flag lives in `flags.ts`; these three are about a *selection*, a
 * *report file*, and an *answer cell*, and none of them is about a number.
 */

import fs from 'node:fs';
import path from 'node:path';

import chalk from 'chalk';

import type { CompilerOutcome } from './types.js';

/**
 * The members `selected` names, or an exit.
 *
 * An empty selection is refused rather than run. A `--property` or `--surface`
 * nobody spells correctly selects nothing, a sweep over nothing finds no
 * unexpected row, and the run exits 0 — so a typo reads as a pass, which in a
 * harness whose whole claim is how much it covered is the worst possible
 * failure mode.
 */
export function selectedOrExit<Member>(
  flag: string,
  selected: string[] | undefined,
  members: readonly Member[],
  nameOf: (member: Member) => string
): Member[] {
  if (selected === undefined || selected.length === 0) return [...members];

  const chosen = members.filter(member => selected.includes(nameOf(member)));
  if (chosen.length > 0) return chosen;

  console.error(
    chalk.red(
      `No ${flag} matches ${JSON.stringify(selected)}.\n` +
        `Known: ${members.map(nameOf).join(', ')}`
    )
  );

  // Returned rather than called as a statement: `process.exit` answers `never`,
  // so this is the one spelling under which every path of the function returns.
  return process.exit(1);
}

/**
 * Write a report to `target`, and answer where it went.
 *
 * Resolved against the package rather than the shell's working directory: `pnpm
 * run --filter` leaves the cwd at the repo root, so the same command run from
 * there and from this package would otherwise write two different files — and
 * these are the reports CI archives.
 */
export function writeJsonReport(packageDir: string, target: string, report: unknown): string {
  const resolved = path.resolve(packageDir, target);
  fs.mkdirSync(path.dirname(resolved), { recursive: true });
  fs.writeFileSync(resolved, `${JSON.stringify(report, null, 2)}\n`);

  return resolved;
}

/**
 * What one compiler did with one subject, as a single report cell.
 *
 * A refusal keeps every line of its complaint, joined rather than cut. Several
 * diagnostics are two lines in both compilers and the second is the one that
 * names the rule, so a cell showing only the first says a call could not be
 * folded without saying what declined it.
 */
export function answerOf(outcome: CompilerOutcome): string {
  return outcome.status === 'error'
    ? `refused: ${outcome.sentence.trim().split('\n').join(' / ')}`
    : outcome.declarations.join(' | ');
}
