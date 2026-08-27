/**
 * The outcome and subject builders both report suites need.
 *
 * `accepted`, `ACCEPTED`, `refused` and `subject` were defined twice, verbatim
 * apart from which optional field each `subject` exposed — `report.test.ts`
 * wanted `expected`, `refusal-families.test.ts` wanted `property`. Two copies of
 * a fixture builder drift, and a fixture that drifts changes what a test is
 * asserting without the test saying so.
 *
 * The optional fields are an object rather than trailing positionals. With
 * positionals, a case that wanted only the last one had to write `undefined` for
 * the ones before it, which is a placeholder a reader has to decode.
 */

import type { CompilerOutcome, ConfigurationOption, ReportEntry, Verdict } from '../lib/types.js';

/** An acceptance emitting `declarations`, which is the half a verdict reads. */
export function accepted(declarations: string[] = ['color:red']): CompilerOutcome {
  return {
    status: 'ok',
    classNames: declarations.map((_, index) => `x${index}`),
    rules: declarations.map(declaration => `.x{${declaration}}`),
    rtlRules: declarations.map(() => ''),
    declarations,
    styleObjects: ['{"k":class}'],
  };
}

export const ACCEPTED = accepted();

export function refused(sentence: string): CompilerOutcome {
  return { status: 'error', message: `[StyleX] ${sentence}`, sentence };
}

/** The complaint the declaration-terminating token guard writes. */
export const TERMINATOR_REFUSAL = 'Rule contains a `{`, `}` or `;` outside of a string or comment';

/** Everything about a subject that is not its verdict or its two outcomes. */
export interface SubjectOptions {
  /** The verdict the entry records, for a divergence already looked at. */
  expected?: Verdict;
  /** The property name; only the `Object.prototype` family reads it. */
  property?: string;
  /**
   * The authored value. Read by any family that claims on evidence rather than
   * on the diagnostic alone, so a case whose family inspects the value has to
   * set one that carries what the family looks for.
   */
  value?: string;
  /**
   * The reason the row's refusal is wanted, in prose. Read by the gate over
   * rows the reference compiler builds and this one refuses.
   */
  note?: string;
  /** The option whose value decides the refusal, for a configured ceiling. */
  configuration?: ConfigurationOption;
}

let counter = 0;

export function subject(
  verdict: Verdict,
  rust: CompilerOutcome,
  babel: CompilerOutcome,
  options: SubjectOptions = {}
): ReportEntry {
  const { expected, note, configuration, property = 'color', value = 'red' } = options;

  // A distinct id per subject because the stances are keyed by entry identity,
  // and two structurally equal rows are two rows.
  counter += 1;

  return {
    kind: 'declaration',
    set: 'test',
    id: `test-${counter}`,
    origin: '__tests__/support.ts',
    property,
    value,
    verdict,
    rust,
    babel,
    ...(expected === undefined ? {} : { expected }),
    ...(note === undefined ? {} : { note }),
    ...(configuration === undefined ? {} : { configuration }),
  };
}
