import type { StyleXOptions } from '../../dist/index.js';

/**
 * Shared shapes for the CSS value parity harness.
 *
 * A corpus entry is a single CSS declaration — one property, one authored
 * value — that both compilers are asked to normalize. A report entry is the
 * verdict for that declaration.
 */

/** One CSS declaration to run through both compilers. */
export interface CorpusEntry {
  /**
   * Stable identifier. Harvested entries derive it from the declaration text,
   * so re-harvesting after a test file moves does not renumber the corpus;
   * hand-written entries use a readable slug instead.
   */
  id: string;
  /** The CSS property, spelled as a StyleX author writes it (camelCase). */
  property: string;
  /** The authored value, exactly as it appears in the source it came from. */
  value: string;
  /** Where the entry came from: `<path>:<line>` for harvested, prose otherwise. */
  origin: string;
  /** Optional note carried into the report, for hand-written entries. */
  note?: string;
}

/** A corpus file: a named set of entries. */
export interface CorpusFile {
  /** Name of the set, echoed in the report so a verdict can be attributed. */
  set: string;
  /** One-line description of what the set covers. */
  description: string;
  entries: CorpusEntry[];
}

/** A corpus entry tagged with the set it was loaded from. */
export type LoadedCorpusEntry = CorpusEntry & { set: string };

/** What one compiler did with one declaration. */
export type CompilerOutcome =
  | {
      status: 'ok';
      /** Generated class names, in emission order. */
      classNames: string[];
      /** Full LTR rule text, in emission order. */
      rules: string[];
      /**
       * Full RTL rule text, in emission order, `''` where a property has none.
       * Compared but not printed — an RTL-only divergence is still a value
       * divergence, and without this a verdict would call it identical.
       */
      rtlRules: string[];
      /**
       * The declaration bodies of those rules — the `property:value` text
       * inside the braces. This is what value normalization produces, so it
       * is the expectation a later ticket asserts against.
       */
      declarations: string[];
    }
  | { status: 'error'; message: string };

export type Verdict =
  /** Both compilers accepted the value and agreed byte for byte. */
  | 'identical'
  /**
   * Both accepted and emitted the same properties, but spelled a value — and
   * therefore hashed a class name — differently. This is the verdict the
   * harness exists to find.
   */
  | 'divergent'
  /**
   * Both accepted, but emitted different properties or a different number of
   * rules. The divergence is in shorthand expansion, property validation, or
   * RTL generation rather than in value normalization, so it is counted apart
   * to keep it from inflating the divergence set this effort is scoped to.
   */
  | 'structurally-divergent'
  /** Both rejected the value. Messages may differ; only the outcome matters. */
  | 'both-reject'
  /** One accepted and the other rejected. */
  | 'acceptance-divergent';

export interface ReportEntry {
  id: string;
  set: string;
  property: string;
  value: string;
  origin: string;
  note?: string;
  verdict: Verdict;
  rust: CompilerOutcome;
  babel: CompilerOutcome;
}

export interface Report {
  /** ISO timestamp of the run. */
  generatedAt: string;
  /** Versions both verdicts are attributable to. */
  subjects: {
    rust: { version: string; resolvedFrom: string };
    babel: { version: string; resolvedFrom: string };
    babelCore: string;
  };
  /** The exact option object handed to both compilers. */
  options: StyleXOptions;
  summary: Record<Verdict, number> & { total: number };
  entries: ReportEntry[];
}
