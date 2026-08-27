/**
 * The divergences the prototype sweep is not news, and the corpus row that says
 * why each one is standing.
 *
 * A generated row has nothing to write an expectation on — the sweep produces
 * its subjects by reading the language, so there is no file a reader could edit
 * — which is the situation `refusal-families.ts` was built for, and an account
 * here is the same idea: a reason, the verdicts its rows read, and the test for
 * whether a row is one of them. Claimed by reason and never by method name, so
 * that adding a surface costs nothing and a reworded diagnostic un-pins its
 * rows rather than quietly outliving them.
 *
 * What an account adds to a family is the second half of the sentence: **the
 * corpus row that carries the reason**. Every divergence here is already
 * measured by a curated row, with the argument written out at whatever length
 * it took, and copying that argument into a second file is how two statements
 * of one reason come to disagree. So an account names the row instead, and
 * `unrecorded` checks the link — a row that has been deleted, has lost its
 * note, or has stopped recording a verdict this account claims fails the run.
 * The reason is written once, where a reader of either harness will find it.
 *
 * These are deliberately *not* entries in `REFUSAL_FAMILIES`, and the reason is
 * mechanical: a family may not claim a row that carries its own `expected`
 * verdict, and every curated row for these refusals carries one. A family added
 * for them would therefore go unclaimed across the curated corpus and fail
 * `pnpm parity` under the unreached-family check, which is a gate worth keeping
 * rather than loosening. The sweep still reads the family list first, so a
 * folded value that trips a guard the families already name — a `;` in a rule,
 * for instance — is pinned there rather than needing an account of its own.
 */

import type { RefusalFamily } from './refusal-families.js';
import type { LoadedCorpusEntry, ReportEntry, Verdict } from './types.js';

/** One reason a sweep row diverges, and the curated row that argues it. */
export interface Account {
  /** How the report names it, in a summary line. */
  readonly name: string;
  /**
   * The line this compiler's complaint carries, matched anywhere in it.
   *
   * The reason line rather than the whole sentence, because the first line of
   * every one of these interpolates the method — `Cannot fold 'toLowerCase' at
   * compile time.` — and matching that would make an account a method list
   * again. The second line is the rule, and it is what a reworded guard changes.
   */
  readonly complaint: string;
  /** The verdicts a row of this account reads. */
  readonly verdicts: readonly Verdict[];
  /** The id of the corpus entry that records the reason. */
  readonly recordedBy: string;
}

/**
 * Every divergence the sweep expects, in the order it is claimed.
 *
 * The order is precedence, as it is for the families: the first account to
 * claim a row keeps it. Only one pair can overlap — a `constructor` call on a
 * number written into the source is refused for the receiver rather than for
 * the read, so it reaches the numeric-literal complaint — and the two accounts
 * are told apart by the sentence either way, so the order is stated rather than
 * relied on.
 */
export const ACCOUNTS: readonly Account[] = [
  {
    name: 'locale-sensitive method',
    complaint: 'Its answer depends on locale data the compiler does not carry.',
    // Two verdicts, because the same refusal meets a reference compiler that
    // sometimes refuses too: `toLocaleString` on a number written out is
    // refused by both, for the receiver on one side and the locale on the
    // other.
    verdicts: ['acceptance-divergent', 'both-reject-divergent'],
    recordedBy: 'modules-06-locale-sensitive-method',
  },
  {
    name: 'a read that escapes onto the function graph',
    complaint:
      "It leads off the value that was written and onto the language's own function graph.",
    verdicts: ['acceptance-divergent'],
    recordedBy: 'modules-15-a-read-that-escapes-onto-the-function-graph',
  },
  {
    name: 'a receiver written as a number',
    complaint: 'Only a number a fold produced can be a method receiver.',
    // `acceptance-divergent` as well as the refusal both compilers make,
    // because one name on this surface is one the reference compiler folds: its
    // `constructor` is `Number`, which needs no receiver at all.
    verdicts: ['both-reject-divergent', 'acceptance-divergent'],
    recordedBy: 'modules-06-numeric-literal-receiver',
  },
  {
    name: 'a mutation that disqualifies the binding',
    complaint: 'Referenced value is not a constant.',
    verdicts: ['both-reject-divergent'],
    recordedBy: 'modules-mutated-binding-read-through-a-method-call',
  },
  {
    name: 'a static that does not answer from the source',
    complaint: 'A fold has to answer from the source alone, and this call does not.',
    verdicts: ['both-reject-divergent'],
    recordedBy: 'modules-15-an-impure-static',
  },
];

/** The complaint this compiler wrote, or `undefined` where it did not refuse. */
function complaintOf(entry: ReportEntry): string | undefined {
  return entry.rust.status === 'error' ? entry.rust.sentence : undefined;
}

/** The account that claims `entry`, or `undefined` where none does. */
export function accountOf(entry: ReportEntry): Account | undefined {
  const complaint = complaintOf(entry);
  if (complaint === undefined) return undefined;

  return ACCOUNTS.find(
    account => account.verdicts.includes(entry.verdict) && complaint.includes(account.complaint)
  );
}

/** Where a row stands once the families and the accounts have been asked. */
export type Standing =
  | { kind: 'agreed' }
  | { kind: 'pinned'; family: RefusalFamily }
  | { kind: 'accounted'; account: Account }
  | { kind: 'unexpected' };

/**
 * A statement about a corpus row an account depends on and the corpus no longer
 * makes.
 *
 * Three ways for the link to break, and each is the account saying something
 * about a reason nobody can read any more: the row is gone, its note is gone,
 * or the verdict it records is not one this account claims — which means the
 * curated row and the generated rows have stopped measuring the same behaviour.
 */
export interface Unrecorded {
  readonly account: Account;
  readonly problem: string;
}

/**
 * The accounts whose corpus row no longer carries the reason they point at.
 *
 * Checked over the whole curated corpus rather than at the row's own set, since
 * an account cares only that the statement exists somewhere a reader can reach
 * it.
 */
export function unrecorded(corpus: readonly LoadedCorpusEntry[]): Unrecorded[] {
  const byId = new Map(corpus.map(entry => [entry.id, entry]));
  const broken: Unrecorded[] = [];

  for (const account of ACCOUNTS) {
    const row = byId.get(account.recordedBy);
    if (row === undefined) {
      broken.push({ account, problem: 'no corpus entry carries that id' });
      continue;
    }
    if (row.note === undefined || row.note.trim() === '') {
      broken.push({ account, problem: 'the corpus entry carries no note' });
      continue;
    }
    if (row.expected === undefined) {
      broken.push({ account, problem: 'the corpus entry records no verdict' });
      continue;
    }
    if (!account.verdicts.includes(row.expected)) {
      broken.push({
        account,
        problem: `the corpus entry records ${row.expected}, which this account does not claim`,
      });
    }
  }

  return broken;
}
