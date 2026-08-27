import { describe, expect, test } from 'vitest';

import { REFUSAL_FAMILIES } from '../lib/refusal-families.js';
import { conclude, fails, stanceOf } from '../lib/report.js';
import type { ReportEntry, Verdict } from '../lib/types.js';
import { ACCEPTED, TERMINATOR_REFUSAL, accepted, refused, subject } from './support.js';

/**
 * Every condition `fails` names, exercised. Asserted here rather than trusted,
 * because the printing half of the harness is a script and a gate nothing
 * exercises is a gate that has been assumed — the same argument `expected` on a
 * corpus entry is built on, turned on the code that reads it.
 *
 * What each condition is and why it is one belongs beside `fails` in
 * `lib/report.ts`; a second copy of that list here is one that goes stale.
 */

/** One row of every family, so a corpus can be complete without being a corpus. */
function everyFamily(): ReportEntry[] {
  return [
    // The value carries a bare `;`, because the family claims the refusal plus
    // the evidence for it rather than the refusal alone.
    subject('acceptance-divergent', refused(TERMINATOR_REFUSAL), ACCEPTED, {
      value: 'red;blue',
    }),
    subject('acceptance-divergent', refused('Rule contains an unclosed comment'), ACCEPTED),
    subject('acceptance-divergent', refused('Unprefixed custom properties: var(x)'), ACCEPTED),
    subject(
      'acceptance-divergent',
      refused('Rule contains a value nested more deeply than the compiler supports (limit 64)'),
      ACCEPTED
    ),
    subject(
      'acceptance-divergent',
      ACCEPTED,
      refused("Cannot read properties of undefined (reading 'type')")
    ),
    subject(
      'both-reject-divergent',
      refused('String value contains invalid UTF-8 encoding.'),
      refused('Invalid pseudo or at-rule.')
    ),
    subject('structurally-divergent', ACCEPTED, accepted(['[:', 'o:'])),
  ];
}

// The `structurally-divergent` row above needs the inherited property name to be
// claimed, which `subject` does not take. Patched in rather than widening the
// helper for one row.
function completeCorpus(): ReportEntry[] {
  const rows = everyFamily();
  const inherited = rows[6];
  if (inherited?.kind === 'declaration') inherited.property = 'toString';
  return rows;
}

/** Recorded, refused here, and silent about why. */
function silent(): ReportEntry {
  return subject('acceptance-divergent', refused('Cannot fold this at compile time.'), ACCEPTED, {
    expected: 'acceptance-divergent',
  });
}

/** A refusal a named option decides, which is not a divergence between the two. */
function ceiling(verdict: Verdict = 'acceptance-divergent'): ReportEntry {
  return subject(verdict, refused('Folded value exceeds maxFoldedCharacters'), ACCEPTED, {
    expected: 'acceptance-divergent',
    configuration: 'maxFoldedCharacters',
    note: 'Raise the option past the count and the same source folds.',
  });
}

describe('where a row stands', () => {
  test('agreement is agreement, and asks no family', () => {
    expect(stanceOf(subject('identical', ACCEPTED, ACCEPTED))).toEqual({ kind: 'agreed' });
    expect(stanceOf(subject('both-reject', refused('x'), refused('x')))).toEqual({
      kind: 'agreed',
    });
  });

  test("a row's own expectation wins over any family that would claim it", () => {
    const stance = stanceOf(
      subject('acceptance-divergent', refused('Rule contains an unclosed comment'), ACCEPTED, {
        expected: 'acceptance-divergent',
      })
    );

    expect(stance).toEqual({ kind: 'expected' });
  });

  test('a recorded expectation that no longer holds is changed, not agreement', () => {
    // The direction that is easy to get wrong: the row now *agrees*, and that is
    // the loud case — a corpus row recording a divergence that stopped happening
    // has stopped measuring what it was written for.
    const stance = stanceOf(subject('identical', ACCEPTED, ACCEPTED, { expected: 'divergent' }));

    expect(stance).toEqual({ kind: 'changed' });
  });

  test('a divergence no family accounts for is news', () => {
    expect(stanceOf(subject('divergent', ACCEPTED, accepted(['color:#f00'])))).toEqual({
      kind: 'unexpected',
    });
  });
});

describe('what a run concludes', () => {
  test('a complete corpus of pinned rows fails on nothing', () => {
    const verdicts = conclude(completeCorpus(), { whole: true });

    expect(verdicts.summary.pinned).toBe(REFUSAL_FAMILIES.length);
    expect(verdicts.summary.unexpected).toBe(0);
    expect(verdicts.unreached).toEqual([]);
    expect(fails(verdicts)).toBe(false);
  });

  test('a changed expectation fails the run', () => {
    // The gate, demonstrated: one row whose recorded verdict moved is enough,
    // and it is listed rather than only counted.
    const moved = subject('identical', ACCEPTED, ACCEPTED, { expected: 'divergent' });
    const verdicts = conclude([...completeCorpus(), moved], { whole: true });

    expect(verdicts.summary.changed).toBe(1);
    expect(verdicts.changed).toEqual([moved]);
    expect(fails(verdicts)).toBe(true);
  });

  test('a family no row reaches fails the run', () => {
    const missing = completeCorpus().slice(1);
    const verdicts = conclude(missing, { whole: true });

    expect(verdicts.unreached.map(family => family.name)).toEqual([
      'declaration-terminating token',
    ]);
    expect(fails(verdicts)).toBe(true);
  });

  test('a reworded diagnostic un-pins its rows and they resurface as news', () => {
    // What a rewording costs, end to end: the row leaves the family, the pinned
    // count drops, and the unexpected count is what a reader sees move. The run
    // fails too — but on the emptied family rather than on the loose row, which
    // is the honest reason.
    const reworded = completeCorpus();
    for (const entry of reworded) {
      if (
        entry.rust.status === 'error' &&
        entry.rust.sentence.startsWith('Rule contains an unclosed comment')
      ) {
        entry.rust = refused('Rule contains a comment that is never closed');
      }
    }

    const verdicts = conclude(reworded, { whole: true });

    expect(verdicts.summary.unexpected).toBe(1);
    expect(verdicts.summary.pinned).toBe(REFUSAL_FAMILIES.length - 1);
    expect(verdicts.unreached.map(family => family.name)).toEqual(['unclosed comment']);
    expect(fails(verdicts)).toBe(true);
  });

  test('a divergence nobody has accounted for fails the run', () => {
    // This used to pass, on the argument that reading a divergence is a person's
    // job and a corpus of degenerate values would otherwise fail every run. The
    // corpus carries no such divergence — `unexpected` is 0 over all 1085
    // subjects and over the generated sweep — so the only thing the exclusion
    // bought was a new divergence landing green in the leg that runs per pull
    // request.
    //
    // A divergence that should not fail still has two ways to say so, and this
    // test's own corpus is built from both: an `expected` verdict on the entry,
    // or a refusal family that accounts for it.
    const news = subject('divergent', ACCEPTED, accepted(['color:#f00']));
    const verdicts = conclude([...completeCorpus(), news], { whole: true });

    expect(verdicts.summary.unexpected).toBe(1);
    expect(fails(verdicts)).toBe(true);
  });

  test('the same divergence, recorded, does not fail the run', () => {
    // The other half of the gate: `expected` is what turns a divergence from
    // news into a measurement, and it has to keep working or the tightening
    // above would leave no way to say "looked at, still true".
    const looked = subject('divergent', ACCEPTED, accepted(['color:#f00']), {
      expected: 'divergent',
    });
    const verdicts = conclude([...completeCorpus(), looked], { whole: true });

    expect(verdicts.summary.unexpected).toBe(0);
    expect(verdicts.summary.expected).toBeGreaterThan(0);
    expect(fails(verdicts)).toBe(false);
  });

  test('a partial corpus is not asked which families it missed', () => {
    // What `--set` and `--filter` hand in. Asking there would report all but
    // one family as unreached on every filtered run, which teaches a reader
    // to skip the line that matters on the unfiltered one.
    const verdicts = conclude([subject('identical', ACCEPTED, ACCEPTED)], { whole: false });

    expect(verdicts.unreached).toEqual([]);
    expect(fails(verdicts)).toBe(false);
  });

  test('the summary counts every verdict as well as every stance', () => {
    const verdicts = conclude(
      [
        subject('identical', ACCEPTED, ACCEPTED),
        subject('identical', ACCEPTED, ACCEPTED),
        subject('identical-empty', ACCEPTED, ACCEPTED),
        ...completeCorpus(),
      ],
      { whole: true }
    );

    expect(verdicts.summary.total).toBe(REFUSAL_FAMILIES.length + 3);
    expect(verdicts.summary.identical).toBe(2);
    expect(verdicts.summary['identical-empty']).toBe(1);
    expect(verdicts.summary['acceptance-divergent']).toBe(5);
    expect(verdicts.summary['both-reject-divergent']).toBe(1);
    expect(
      verdicts.summary.expected +
        verdicts.summary.changed +
        verdicts.summary.pinned +
        verdicts.summary.configured +
        verdicts.summary.unexpected
    ).toBe(REFUSAL_FAMILIES.length);
  });

  test('every pinned row appears under exactly one family', () => {
    const verdicts = conclude(completeCorpus(), { whole: true });
    const grouped = [...verdicts.byFamily.values()].flat();

    expect(grouped).toHaveLength(verdicts.summary.pinned);
    expect(new Set(grouped).size).toBe(grouped.length);
  });
});

/**
 * A refusal the reference compiler does not make is the one an author feels, so
 * a row recording one has to say why it is wanted. Two forms count — a `note` on
 * the entry, or a refusal family — and a row with neither fails the run.
 *
 * The gap this closes is not a divergence nobody looked at: those already fail
 * as `unexpected`. It is a row someone recorded an expectation on and wrote
 * nothing beside, which reads as looked-at while saying nothing about what was
 * concluded.
 */
describe('a refusal of a build the reference compiler completes', () => {
  test('fails the run when nothing says why', () => {
    const verdicts = conclude([...completeCorpus(), silent()], { whole: true });

    expect(verdicts.unreasoned).toHaveLength(1);
    // Not caught by any of the other three: the row's expectation holds, no
    // family was asked, and it is not counted as unexpected.
    expect(verdicts.summary.changed).toBe(0);
    expect(verdicts.summary.unexpected).toBe(0);
    expect(fails(verdicts)).toBe(true);
  });

  test('a note on the entry is a reason, and the run passes', () => {
    const reasoned = subject(
      'acceptance-divergent',
      refused('Cannot fold this at compile time.'),
      ACCEPTED,
      { expected: 'acceptance-divergent', note: 'The engine carries no locale data.' }
    );
    const verdicts = conclude([...completeCorpus(), reasoned], { whole: true });

    expect(verdicts.unreasoned).toEqual([]);
    expect(fails(verdicts)).toBe(false);
  });

  test('a whitespace-only note is not a reason', () => {
    // A note is prose a later reader checks, so the empty forms of it have to
    // fail the same way an absent one does — otherwise the cheapest way past
    // the gate is a space.
    const blank = subject(
      'acceptance-divergent',
      refused('Cannot fold this at compile time.'),
      ACCEPTED,
      { expected: 'acceptance-divergent', note: '   \n  ' }
    );

    expect(conclude([...completeCorpus(), blank], { whole: true }).unreasoned).toHaveLength(1);
  });

  test('a refusal family is a reason, so the generated corpora need no note', () => {
    // Every row in `completeCorpus` is claimed by a family and none carries a
    // note. `harvested.json` is regenerated wholesale, so a note written there
    // is lost on the next harvest — a family is the only durable form it has.
    const verdicts = conclude(completeCorpus(), { whole: true });

    expect(verdicts.summary.pinned).toBe(REFUSAL_FAMILIES.length);
    expect(verdicts.unreasoned).toEqual([]);
  });

  test('the other direction is not asked for one', () => {
    // This compiler compiling where the reference refuses costs an author
    // nothing, so it carries no such obligation. Only the direction that stops
    // a build the reference completes does.
    const ours = subject('acceptance-divergent', ACCEPTED, refused('Unsupported expression'), {
      expected: 'acceptance-divergent',
    });
    const verdicts = conclude([...completeCorpus(), ours], { whole: true });

    expect(verdicts.unreasoned).toEqual([]);
    expect(fails(verdicts)).toBe(false);
  });
});

/**
 * A ceiling an author can raise is not a divergence between the two compilers:
 * the same source folds to the same value on both once the option passes the
 * number the input needs. Those rows are counted apart so the divergence columns
 * a reader acts on do not carry them.
 */
describe('a configured ceiling', () => {
  test('is read as configuration, naming the option', () => {
    expect(stanceOf(ceiling())).toEqual({
      kind: 'configured',
      option: 'maxFoldedCharacters',
    });
  });

  test('is counted apart from a recorded divergence, and fails nothing', () => {
    const verdicts = conclude([...completeCorpus(), ceiling()], { whole: true });

    expect(verdicts.summary.configured).toBe(1);
    expect(verdicts.summary.expected).toBe(0);
    expect(fails(verdicts)).toBe(false);
  });

  test('is still an expectation, so a verdict that moved is changed', () => {
    // The direction that would otherwise go quiet: the ceiling stopped
    // refusing — the guard moved, or the default rose past the input — and a
    // row read as configuration would have gone on reading as accounted for.
    const moved = ceiling('identical');
    const verdicts = conclude([...completeCorpus(), moved], { whole: true });

    expect(verdicts.summary.configured).toBe(0);
    expect(verdicts.changed).toEqual([moved]);
    expect(fails(verdicts)).toBe(true);
  });
});
