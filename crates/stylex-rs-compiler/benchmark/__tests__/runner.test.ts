/**
 * Producer-side guarantees of `runRounds`.
 *
 * The budget check resolves which subject its ceilings describe from
 * `fixtures[].paired`, and the release benchmark records roles without
 * bootstrap statistics. That combination was untested: the budget suite
 * hand-built raw stats that always carried a full `paired` block, so a
 * producer that emitted none still passed every test and failed the
 * release job instead. These assertions run the real producer.
 */

import { describe, expect, test } from 'vitest';

import { parseRawStats } from '../lib/raw-stats.js';
import { runRounds } from '../lib/runner.js';
import { createSubject, type SubjectRun } from '../lib/subjects.js';
import {
  RAW_STATS_SCHEMA_VERSION,
  type FixtureDescriptor,
  type RawStatsEnvironment,
} from '../lib/types.js';

const FIXTURE: FixtureDescriptor = {
  name: 'card',
  filePath: '/fixtures/card.js',
  code: 'const styles = 1;',
  weight: 'standard',
  category: 'transform',
  batchSize: 1,
};

/**
 * A second fixture, so a run that loses one still has something to measure.
 *
 * A run whose every fixture leaves it is its own failure, and a test that
 * cannot tell the two apart would pass on either.
 */
const OTHER_FIXTURE: FixtureDescriptor = { ...FIXTURE, name: 'counter' };

/**
 * Throws `value`, whatever it is.
 *
 * A native binding can reject with something that is not an `Error`, and the
 * report has to read as a sentence either way. Written as a call because a
 * thrown literal is a lint error, and what is under test is the shape the value
 * has when it arrives rather than how the test spelled it.
 */
function raise(value: unknown): never {
  throw value;
}

/** A subject run that refuses every fixture with `message`. */
function refusingWith(message: string): SubjectRun {
  return () => raise(new Error(message));
}

/** A subject that answers `rules` for one named fixture and 1 for the rest. */
function subjectRefusing(label: string, name: string, rules: SubjectRun) {
  return createSubject(
    { label, version: '1.0.0', resolvedFrom: `/${label}` },
    (fixture, options) => (fixture.name === name ? rules(fixture, options) : 1)
  );
}

// One iteration per task keeps the suite fast; the assertions are about
// the shape of the emitted stats, not the timings.
const BENCH = { retainSamples: true, warmup: false, time: 0, iterations: 1 } as const;

const ENVIRONMENT: RawStatsEnvironment = {
  timestamp: '2026-01-01T00:00:00.000Z',
  node: 'v24.18.0',
  os: { type: 'Linux', release: '6.0', arch: 'x64', platform: 'linux' },
  cpu: { model: 'test', cores: 2 },
  memoryGB: 16,
  packageVersion: '0.0.0',
  target: 'x86_64-unknown-linux-gnu',
  toolchain: {},
};

function subject(label: string) {
  return createSubject({ label, version: '1.0.0', resolvedFrom: `/${label}` }, () => 1);
}

async function run(
  subjects: ReturnType<typeof subject>[],
  rounds = 1,
  seed = 1,
  extra: { fixtures?: FixtureDescriptor[]; requiredSubject?: string } = {}
) {
  return runRounds({
    subjects,
    fixtures: extra.fixtures ?? [FIXTURE],
    stylexOptions: {},
    rounds,
    seed,
    standardBench: BENCH,
    heavyBench: BENCH,
    ...(extra.requiredSubject === undefined ? {} : { requiredSubject: extra.requiredSubject }),
  });
}

/**
 * A run of both fixtures where only the candidate is a gate.
 *
 * Every exclusion test is that run, so the fixture pair and the gate label are
 * stated once. `subjects` defaults to one healthy candidate behind the base
 * given, which is the paired shape; a test that needs a third subject passes
 * the whole list.
 */
async function pairedRun(...subjects: ReturnType<typeof subject>[]) {
  const named = subjects.some(entry => entry.descriptor.label === 'candidate')
    ? subjects
    : [...subjects, subject('candidate')];

  return run(named, 1, 1, {
    fixtures: [FIXTURE, OTHER_FIXTURE],
    requiredSubject: 'candidate',
  });
}

/**
 * The error a paired run rejects with when its `base` subject refuses the
 * fixture and its `candidate` subject is healthy.
 *
 * Returns the `Error` rather than asserting on it, so a caller states the whole
 * message with `toBe`: `rejects.toThrow` matches a substring, and these
 * messages *are* the diagnosis, so a stray suffix must not pass. A run that
 * resolves, or rejects with something that is not an `Error`, fails here rather
 * than turning into a cast at the call site.
 *
 * The healthy second subject is what makes this a paired run, and it is what
 * pins *which* subject gets named: a refusal reported against the last subject
 * in the list rather than the one that threw passes every single-subject test
 * and is caught here. Naming the wrong revision is the one way this message can
 * mislead while still looking like a diagnosis.
 */
async function refused(subjectRun: SubjectRun): Promise<Error> {
  const failure = await run([
    createSubject({ label: 'base', version: '1.0.0', resolvedFrom: '/base' }, subjectRun),
    subject('candidate'),
  ]).catch((error: unknown) => error);

  if (!(failure instanceof Error)) throw new Error('the run was expected to reject with an Error');

  return failure;
}

describe('runRounds paired roles', () => {
  test('a two-subject run records roles even without bootstrap statistics', async () => {
    const { fixtures } = await run([subject('base'), subject('candidate')]);
    const paired = fixtures[0]?.paired;

    expect(paired).toBeDefined();
    expect(paired?.base).toBe('base');
    expect(paired?.candidate).toBe('candidate');
    expect(paired?.ratios).toBeUndefined();
    expect(paired?.confidence).toBeUndefined();
  });

  test('counterbalances each subject position across paired rounds', async () => {
    const { fixtures } = await run([subject('base'), subject('candidate')], 10);
    const orders = fixtures[0]?.rounds.map(round => round.subjectOrder);

    expect(orders).toHaveLength(10);
    expect(orders?.filter(order => order[0] === 'base')).toHaveLength(5);
    expect(orders?.filter(order => order[0] === 'candidate')).toHaveLength(5);
  });

  test('reproduces the counterbalanced order from the same seed', async () => {
    const first = await run([subject('base'), subject('candidate')], 4, 7);
    const second = await run([subject('base'), subject('candidate')], 4, 7);

    expect(first.fixtures[0]?.rounds.map(round => round.subjectOrder)).toEqual(
      second.fixtures[0]?.rounds.map(round => round.subjectOrder)
    );
  });

  test('a single-subject run records no paired block', async () => {
    const { fixtures } = await run([subject('current')]);
    expect(fixtures[0]?.paired).toBeUndefined();
  });

  // Which subject refused which fixture is the whole of a paired run's
  // diagnosis, and the run reports it in no other way, so both messages are
  // pinned exactly rather than by substring.
  test('names the fixture and the subject when a subject cannot compile it', async () => {
    const failure = await refused(() => {
      throw new Error('[StyleX] Style value must evaluate to a static expression.');
    });

    expect(failure.message).toBe(
      'Sanity check failed: subject "base" could not compile fixture "card"'
    );
    // The compiler's own message is what says *why*, so it must survive as the
    // cause. Narrowed, never asserted: `cause` is `unknown`.
    const cause = failure.cause;
    if (!(cause instanceof Error)) throw new Error('the refusal lost its cause');

    expect(cause.message).toMatch(/static expression/);
  });

  // `guidelines/PERFORMANCE.md` makes a zero-rule fixture a gate rather than a
  // curiosity: a subject that emits nothing is fast, and a fixture that stopped
  // producing rules would otherwise report as an improvement.
  test('refuses a subject that produces no rules, naming both', async () => {
    const failure = await refused(() => 0);

    expect(failure.message).toBe(
      'Sanity check failed: subject "base" produced 0 StyleX rules for fixture "card"'
    );
  });

  test('roles-only output survives a raw-stats round trip', async () => {
    const { fixtures } = await run([subject('base'), subject('candidate')]);
    const file = {
      schemaVersion: RAW_STATS_SCHEMA_VERSION,
      environment: ENVIRONMENT,
      subjects: [
        { label: 'base', version: '1.0.0', resolvedFrom: '/base' },
        { label: 'candidate', version: '1.0.0', resolvedFrom: '/candidate' },
      ],
      fixtures,
    };

    const parsed = parseRawStats(JSON.parse(JSON.stringify(file)), 'raw', { subjects: 'any' });
    expect(parsed.fixtures[0]?.paired?.candidate).toBe('candidate');
  });
});

/**
 * What a paired run does with a fixture only one of its subjects can measure.
 *
 * The release leg compares this build against the last published version, so
 * the base is behind by every feature landed since. A fixture that prices such
 * a feature has no second side, and stopping the leg for it threw away every
 * other measurement -- which is what one `.trim()` in `engine-fold.js` did to
 * the whole publish benchmark. The gate that matters is the candidate: a
 * fixture *it* refuses is a regression in the code under measurement.
 */
describe('runRounds fixture exclusion', () => {
  test('a healthy run excludes nothing', async () => {
    const { fixtures, excluded } = await pairedRun(subject('base'));

    expect(excluded).toEqual([]);
    expect(fixtures.map(fixture => fixture.name)).toEqual(['card', 'counter']);
  });

  test('drops a fixture the base cannot compile and keeps the rest', async () => {
    const base = subjectRefusing(
      'base',
      'card',
      refusingWith("[StyleX] The method 'trim' is not yet supported in static evaluation.")
    );

    const { fixtures, excluded } = await pairedRun(base);

    expect(fixtures.map(fixture => fixture.name)).toEqual(['counter']);
    expect(excluded).toEqual([
      {
        fixture: 'card',
        subject: 'base',
        reason: "[StyleX] The method 'trim' is not yet supported in static evaluation.",
      },
    ]);
  });

  // A compiler refusal carries a code frame under its sentence. Beside a
  // dropped fixture the sentence is the whole of what a reader needs, and the
  // frame would bury the fixture names the report is made of.
  test('reports only the first line of what the base said', async () => {
    const base = subjectRefusing(
      'base',
      'card',
      refusingWith('  refused\n  --> file.js:1:1\n   |\n 1 | code\n')
    );

    const { excluded } = await pairedRun(base);

    expect(excluded[0]?.reason).toBe('refused');
  });

  test('names the refusal when the base threw something that is not an Error', async () => {
    const base = subjectRefusing('base', 'card', () => raise('plain string'));

    const { excluded } = await pairedRun(base);

    expect(excluded[0]?.reason).toBe('plain string');
  });

  test('stands in for a refusal that carries no message at all', async () => {
    const base = subjectRefusing('base', 'card', refusingWith(''));

    const { excluded } = await pairedRun(base);

    expect(excluded[0]?.reason).toBe('refused without a message');
  });

  // A base that compiles a fixture to nothing cannot measure it either: the
  // zero-rule guard reads the same either way, and reporting the count keeps
  // the two apart for whoever has to read the line.
  test('drops a fixture the base compiles to no rules', async () => {
    const { fixtures, excluded } = await pairedRun(subjectRefusing('base', 'card', () => 0));

    expect(fixtures.map(fixture => fixture.name)).toEqual(['counter']);
    expect(excluded[0]?.reason).toBe('emitted 0 StyleX rules');
  });

  test('drops a fixture whose base rule count is not a number at all', async () => {
    const { excluded } = await pairedRun(subjectRefusing('base', 'card', () => Number.NaN));

    expect(excluded[0]?.reason).toBe('emitted NaN StyleX rules');
  });

  // The one refusal that must still stop the run. Reported against the
  // candidate even though the base refused the same fixture first, because the
  // subject under measurement is the one a reader has to act on.
  test('fails when the subject under measurement cannot compile a fixture', async () => {
    const refuse = refusingWith('[StyleX] Style value must evaluate to a static expression.');
    const failure = await pairedRun(
      subjectRefusing('base', 'card', refuse),
      subjectRefusing('candidate', 'card', refuse)
    ).catch((error: unknown) => error);

    expect(failure).toBeInstanceOf(Error);
    expect((failure as Error).message).toBe(
      'Sanity check failed: subject "candidate" could not compile fixture "card"'
    );
  });

  test('fails when the subject under measurement emits no rules', async () => {
    const failure = await pairedRun(
      subject('base'),
      subjectRefusing('candidate', 'card', () => 0)
    ).catch((error: unknown) => error);

    expect((failure as Error).message).toBe(
      'Sanity check failed: subject "candidate" produced 0 StyleX rules for fixture "card"'
    );
  });

  // A base that refuses everything is a broken base, not a manifest question,
  // and a run that measured nothing must not report as a clean comparison.
  test('fails when no fixture survives, naming every one that left', async () => {
    const base = createSubject(
      { label: 'base', version: '1.0.0', resolvedFrom: '/base' },
      refusingWith('dist/index.js is not a StyleX compiler')
    );

    const failure = await pairedRun(base).catch((error: unknown) => error);

    expect((failure as Error).message).toBe(
      'Sanity check failed: no fixture is measurable by every subject — ' +
        '"card" (base: dist/index.js is not a StyleX compiler), ' +
        '"counter" (base: dist/index.js is not a StyleX compiler)'
    );
  });

  // Without a required subject nothing is privileged, so the older behaviour
  // holds and any refusal stops the run. Both legs that pair subjects name
  // one; a caller that does not gets the stricter reading.
  test('keeps stopping the run when no subject is named as the gate', async () => {
    const base = subjectRefusing('base', 'card', refusingWith('refused'));
    const failure = await run([base, subject('candidate')], 1, 1, {
      fixtures: [FIXTURE, OTHER_FIXTURE],
    }).catch((error: unknown) => error);

    expect((failure as Error).message).toBe(
      'Sanity check failed: subject "base" could not compile fixture "card"'
    );
  });

  // A large manifest where one entry leaves: the report must name that entry
  // and nothing else, and the surviving fixtures must keep manifest order.
  test('keeps manifest order across a large manifest with one exclusion', async () => {
    const many = Array.from({ length: 60 }, (_index, position) => ({
      ...FIXTURE,
      name: `fixture-${String(position).padStart(2, '0')}`,
    }));
    const base = subjectRefusing('base', 'fixture-31', refusingWith('refused'));

    const { fixtures, excluded } = await run([base, subject('candidate')], 1, 1, {
      fixtures: many,
      requiredSubject: 'candidate',
    });

    expect(excluded).toHaveLength(1);
    expect(excluded[0]?.fixture).toBe('fixture-31');
    expect(fixtures).toHaveLength(59);
    expect(fixtures.map(fixture => fixture.name)).toEqual(
      many.map(fixture => fixture.name).filter(name => name !== 'fixture-31')
    );
  });

  // Two subjects refusing the same fixture is one exclusion. A second line
  // saying the same thing adds no information, and the fixture is gone either
  // way -- so the count is what a reader can trust.
  test('reports one exclusion when every non-gate subject refuses the same fixture', async () => {
    const refuse = refusingWith('refused');
    const { excluded } = await pairedRun(
      subjectRefusing('base-a', 'card', refuse),
      subjectRefusing('base-b', 'card', refuse)
    );

    expect(excluded).toEqual([{ fixture: 'card', subject: 'base-a', reason: 'refused' }]);
  });
});
