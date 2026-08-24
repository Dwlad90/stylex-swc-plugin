/**
 * What a run of the value corpus says, decided apart from how it is printed.
 *
 * The two halves separate for one reason: the printing half is a script, so
 * nothing in it can be asserted, and the deciding half is what the harness's
 * two failure conditions rest on. A gate nothing can exercise is a gate that
 * has been assumed rather than demonstrated — which is the same argument the
 * `expected` field on a corpus entry is built on, applied to the code that
 * reads it.
 *
 * So this module answers, over a list of compared entries: where each row
 * stands, how many of each there are, which families accounted for what, and
 * whether the run should fail. `parity-values.ts` reads those answers and turns
 * them into lines.
 */

import { groupByFamily, familyOf, unreachedFamilies } from './refusal-families.js';
import type { RefusalFamily } from './refusal-families.js';
import type { Report, ReportEntry, Verdict } from './types.js';

/**
 * The verdicts where the two compilers agreed, whatever they agreed about.
 *
 * One set rather than two spellings of the same list: `stanceOf` decides what is
 * agreement from it and the report skips its side-by-side detail on it, and a
 * verdict added to one place but not the other is either hidden from
 * `--only-mismatches` or printed as two empty lines.
 *
 * `identical-empty` belongs here. Both compilers accepted and emitted nothing,
 * which is agreement — that it measures nothing is a fact about the corpus
 * rather than about parity, and the summary reports it on its own line where a
 * count is the useful form. Listing it as a mismatch would overload the word
 * for the one verdict that is not a disagreement.
 *
 * `both-reject-divergent` does not. Two refusals worded differently are the
 * only thing that separates it from `both-reject`, and that difference is the
 * whole of what an author whose build stopped is handed — so it is a mismatch
 * to chase, and its two sentences are what the entry prints.
 */
export const AGREED: ReadonlySet<Verdict> = new Set<Verdict>([
  'identical',
  'identical-empty',
  'both-reject',
]);

/**
 * Where one entry's verdict stands: agreement, something already accounted for,
 * or news.
 *
 * `agreed` and `unexpected` are the two a reader cares about, and the three in
 * between are how a row leaves the second group. `expected` and `changed` come
 * from the entry's own recorded verdict — a changed one is loud in both
 * directions, since a divergence that has gone away is a corpus row that has
 * stopped measuring what it was written for, exactly as a new one is a
 * regression. `pinned` comes from a refusal family instead, for the permanent
 * divergences no entry can carry an expectation on; see `refusal-families.ts`.
 *
 * The entry's own expectation is consulted first. It is the more specific of the
 * two — written for one value, with a note saying why — and a family that also
 * claimed it would relabel a row someone wrote by hand.
 */
export type Stance =
  | { kind: 'agreed' }
  | { kind: 'expected' }
  | { kind: 'changed' }
  | { kind: 'pinned'; family: RefusalFamily }
  | { kind: 'unexpected' };

export function stanceOf(entry: ReportEntry): Stance {
  if (entry.expected !== undefined) {
    return entry.verdict === entry.expected ? { kind: 'expected' } : { kind: 'changed' };
  }
  if (AGREED.has(entry.verdict)) return { kind: 'agreed' };

  const family = familyOf(entry);
  return family === undefined ? { kind: 'unexpected' } : { kind: 'pinned', family };
}

/** Everything a run concluded, before any of it is printed. */
export interface Verdicts {
  summary: Report['summary'];
  /** The stance of each entry, decided once so no two readers can disagree. */
  stances: ReadonlyMap<ReportEntry, Stance>;
  /** The pinned rows, grouped by the family that accounted for them. */
  byFamily: ReadonlyMap<RefusalFamily, ReportEntry[]>;
  /** Entries whose recorded expectation no longer holds, in corpus order. */
  changed: ReportEntry[];
  /**
   * Families no row reached, or empty where the caller said not to ask.
   *
   * A filtered run reaches a handful of families by construction, so asking
   * there would train a reader to ignore the answer.
   */
  unreached: RefusalFamily[];
}

export interface ConcludeOptions {
  /**
   * Whether the corpus handed in is the whole of it. `false` suppresses the
   * unreached-family check, which is the only conclusion that depends on the
   * corpus being complete.
   */
  whole: boolean;
}

export function conclude(entries: readonly ReportEntry[], options: ConcludeOptions): Verdicts {
  const stances = new Map<ReportEntry, Stance>(entries.map(entry => [entry, stanceOf(entry)]));
  const summary = {
    total: entries.length,
    expected: 0,
    changed: 0,
    pinned: 0,
    unexpected: 0,
    identical: 0,
    'identical-empty': 0,
    divergent: 0,
    'structurally-divergent': 0,
    'both-reject': 0,
    'both-reject-divergent': 0,
    'acceptance-divergent': 0,
  } satisfies Report['summary'];

  for (const entry of entries) {
    summary[entry.verdict]++;
    // Read back with `!` rather than a default: the map was built from this same
    // array, and a default would invent a stance for an entry that cannot exist.
    const stance = stances.get(entry)!;
    if (stance.kind === 'expected') summary.expected++;
    if (stance.kind === 'changed') summary.changed++;
    if (stance.kind === 'pinned') summary.pinned++;
    if (stance.kind === 'unexpected') summary.unexpected++;
  }

  return {
    summary,
    stances,
    byFamily: groupByFamily(entries),
    changed: entries.filter(entry => stances.get(entry)!.kind === 'changed'),
    unreached: options.whole ? unreachedFamilies(entries) : [],
  };
}

/**
 * Whether a run should exit non-zero.
 *
 * Three conditions, and all three are a report that has stopped being read: a
 * row whose recorded verdict moved, a family nothing reached, and a divergence
 * nothing accounts for.
 *
 * The third was excluded on the argument that reading a divergence is a person's
 * job and a corpus of degenerate values would otherwise fail every run. That
 * argument stopped describing this corpus: `unexpected` is **0** across all of
 * it, and across the generated sweep — every divergent row is either an
 * `expected` verdict or a row a refusal family accounts for. So the clean-run
 * invariant already holds, and leaving the count out of the gate meant a new
 * value divergence landed green in the leg that runs per pull request, which is
 * the one failure this harness exists to catch.
 *
 * `fuzz-shorthand-split.ts` next door already exits non-zero on the same number.
 * Two harnesses disagreeing about whether an unaccounted divergence is a failure
 * meant one of them was wrong, whichever way it was settled.
 *
 * A divergence that should not fail a run has two ways to say so, and both are
 * durable: record its verdict as `expected` on the corpus entry, or write the
 * family that accounts for it. Neither is a suppression — each is a statement a
 * later reader can check.
 */
export function fails(verdicts: Verdicts): boolean {
  return (
    verdicts.changed.length > 0 || verdicts.unreached.length > 0 || verdicts.summary.unexpected > 0
  );
}
