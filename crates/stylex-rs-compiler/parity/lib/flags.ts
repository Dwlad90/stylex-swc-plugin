/**
 * Reading the numeric command-line flags the generated harnesses take.
 *
 * Shared because both of them take a count and both got it wrong the same way,
 * and because a flag that silently means something other than what was typed is
 * worse in a fuzzer than anywhere else: the whole claim a sweep makes is how
 * much it covered.
 */

import chalk from 'chalk';

/**
 * A positive-integer flag, refused rather than defaulted.
 *
 * `Number.parseInt` is not enough on its own. It stops at the first character it
 * cannot read, so `--pairs 1e9` parses as **1** — which passes any `>= 1` bound
 * check and runs a single pair, so a reader who asked for a large sweep gets a
 * green run over one subject. `--show abc` parses as `NaN`, and `slice(0, NaN)`
 * is empty, so a failing run printed its count and none of the evidence.
 *
 * The whole string therefore has to be digits for the number the reader typed to
 * be the number that runs, and anything else exits rather than falling back to a
 * default the reader did not ask for.
 */
export function countFlag(
  flag: string,
  raw: string | undefined,
  fallback: number,
  most: number
): number {
  if (raw === undefined) return fallback;

  if (!/^\d+$/.test(raw)) {
    console.error(chalk.red(`${flag} must be a whole number, got ${JSON.stringify(raw)}.`));
    process.exit(1);
  }

  const parsed = Number(raw);

  if (parsed < 1 || parsed > most) {
    console.error(chalk.red(`${flag} must be between 1 and ${most}, got ${raw}.`));
    process.exit(1);
  }

  return parsed;
}
