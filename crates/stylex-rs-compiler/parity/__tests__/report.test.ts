import { describe, expect, test } from 'vitest';

import { REFUSAL_FAMILIES } from '../lib/refusal-families.js';
import { conclude, fails, stanceOf } from '../lib/report.js';
import type { CompilerOutcome, ReportEntry, Verdict } from '../lib/types.js';

/**
 * The harness fails on exactly two things, and both are an expectation that has
 * stopped measuring anything. Asserted here rather than trusted, because the
 * printing half of the harness is a script and a gate nothing exercises is a
 * gate that has been assumed — the same argument `expected` on a corpus entry is
 * built on, turned on the code that reads it.
 */

/** An acceptance emitting `declarations`, which is the half a verdict reads. */
function accepted(declarations: string[] = ['color:red']): CompilerOutcome {
  return {
    status: 'ok',
    classNames: declarations.map((_, index) => `x${index}`),
    rules: declarations.map(declaration => `.x{${declaration}}`),
    rtlRules: declarations.map(() => ''),
    declarations,
    styleObjects: ['{"k":class}'],
  };
}

const ACCEPTED = accepted();

function refused(sentence: string): CompilerOutcome {
  return { status: 'error', message: `[StyleX] ${sentence}`, sentence };
}

let counter = 0;

function subject(
  verdict: Verdict,
  rust: CompilerOutcome,
  babel: CompilerOutcome,
  expected?: Verdict
): ReportEntry {
  // A distinct id per subject because the stances are keyed by entry identity,
  // and two structurally equal rows are two rows.
  counter += 1;
  return {
    kind: 'declaration',
    set: 'test',
    id: `test-${counter}`,
    origin: 'report.test.ts',
    property: 'color',
    value: 'red',
    verdict,
    rust,
    babel,
    ...(expected === undefined ? {} : { expected }),
  };
}

/** One row of every family, so a corpus can be complete without being a corpus. */
function everyFamily(): ReportEntry[] {
  return [
    subject(
      'acceptance-divergent',
      refused('Rule contains a `{`, `}` or `;` outside of a string or comment'),
      ACCEPTED
    ),
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
    subject('structurally-divergent', ACCEPTED, accepted(['[:', 'o:']), undefined),
  ];
}

// The `structurally-divergent` row above needs the inherited property name to be
// claimed, which `subject` does not take. Patched in rather than widening the
// helper for one row.
function completeCorpus(): ReportEntry[] {
  const rows = everyFamily();
  const inherited = rows[5];
  if (inherited?.kind === 'declaration') inherited.property = 'toString';
  return rows;
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
      subject(
        'acceptance-divergent',
        refused('Rule contains an unclosed comment'),
        ACCEPTED,
        'acceptance-divergent'
      )
    );

    expect(stance).toEqual({ kind: 'expected' });
  });

  test('a recorded expectation that no longer holds is changed, not agreement', () => {
    // The direction that is easy to get wrong: the row now *agrees*, and that is
    // the loud case — a corpus row recording a divergence that stopped happening
    // has stopped measuring what it was written for.
    const stance = stanceOf(subject('identical', ACCEPTED, ACCEPTED, 'divergent'));

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
    const moved = subject('identical', ACCEPTED, ACCEPTED, 'divergent');
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

  test('a divergence nobody has looked at is reported, not failed on', () => {
    // Deliberate, and the line between the two conditions: reading a divergence
    // is a person's job, and a corpus of degenerate values would otherwise fail
    // every run.
    const news = subject('divergent', ACCEPTED, accepted(['color:#f00']));
    const verdicts = conclude([...completeCorpus(), news], { whole: true });

    expect(verdicts.summary.unexpected).toBe(1);
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

    expect(verdicts.summary.total).toBe(9);
    expect(verdicts.summary.identical).toBe(2);
    expect(verdicts.summary['identical-empty']).toBe(1);
    expect(verdicts.summary['acceptance-divergent']).toBe(5);
    expect(
      verdicts.summary.expected +
        verdicts.summary.changed +
        verdicts.summary.pinned +
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
