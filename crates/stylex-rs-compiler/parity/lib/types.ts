import type { StyleXOptions } from '../../dist/index.js';

/**
 * Shared shapes for the CSS value parity harness.
 *
 * A corpus entry asks both compilers one question and a report entry is their
 * two answers. Almost every entry asks it as a single CSS declaration — one
 * property, one authored value — because that is what a class name hashes and
 * therefore what the compatibility contract is written in.
 *
 * A few cannot be asked that way. Whether an expression the evaluator cannot
 * fold is refused or aborts the build is a fact about a whole module: the
 * expression sits in a component, not in a declaration, and the evidence is
 * that both compilers emitted the same rules at all rather than that they
 * spelled one the same way. Those entries carry their module source instead,
 * and the two subject kinds are distinguished by `kind` everywhere they are
 * read.
 */

/** The fields every corpus entry carries, whatever it asks about. */
interface CorpusEntryBase {
  /**
   * Stable identifier. Harvested entries derive it from the declaration text,
   * so re-harvesting after a test file moves does not renumber the corpus;
   * hand-written entries use a readable slug instead.
   */
  id: string;
  /** Where the entry came from: `<path>:<line>` for harvested, prose otherwise. */
  origin: string;
  /** Optional note carried into the report, for hand-written entries. */
  note?: string;
}

/** One CSS declaration, as written in a corpus file. */
export interface DeclarationEntry extends CorpusEntryBase {
  /** The CSS property, spelled as a StyleX author writes it (camelCase). */
  property: string;
  /** The authored value, exactly as it appears in the source it came from. */
  value: string;
}

/**
 * One whole module, for a question a declaration cannot express.
 *
 * The comparison is class names, rule text, and the shape of the style objects
 * — never the emitted JavaScript as text. The two compilers print code
 * differently (parameter lists, JSX spacing, how a `const` array is wrapped,
 * which consumed declarations they leave standing), so comparing their output
 * would report a divergence on every entry and say nothing about StyleX. What a
 * module subject adds is the ability to ask whether a compiler *reached* the
 * rules at all.
 */
export interface ModuleEntry extends CorpusEntryBase {
  /** Short name for the report, since there is no `property: value` to print. */
  label: string;
  /** The module handed to both compilers verbatim. May contain JSX. */
  source: string;
}

/**
 * A corpus entry, tagged with which kind of question it asks.
 *
 * The tag is added when the corpus is loaded rather than stored: a declaration
 * entry is the overwhelming majority and `harvested.json` is generated, so
 * writing `"kind": "declaration"` onto 700-odd generated rows would churn a
 * checked-in fixture — and the file that is generated from it — to record
 * something already implied by the fields present.
 */
export type CorpusEntry =
  | ({ kind: 'declaration' } & DeclarationEntry)
  | ({ kind: 'module' } & ModuleEntry);

/**
 * A corpus file as it is written on disk, where `kind` is implied rather than
 * stored.
 */
export interface CorpusFile {
  /** Name of the set, echoed in the report so a verdict can be attributed. */
  set: string;
  /** One-line description of what the set covers. */
  description: string;
  entries: (DeclarationEntry | ModuleEntry)[];
}

/** A corpus file once loaded, with every entry tagged with its kind. */
export type LoadedCorpusFile = Omit<CorpusFile, 'entries'> & { entries: CorpusEntry[] };

/** A corpus entry tagged with the set it was loaded from. */
export type LoadedCorpusEntry = CorpusEntry & { set: string };

/** What one compiler did with one subject. */
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
      /**
       * The style objects the compiled module carries, as canonical text — one
       * per `$$css`-marked object literal, in source order.
       *
       * This is the half of the answer the rule text cannot carry: a property
       * whose value is `null` emits no CSS, so two compilers that disagree
       * about whether the property exists at all agree on every rule. See
       * `lib/style-object.ts`.
       */
      styleObjects: string[];
    }
  | { status: 'error'; message: string };

export type Verdict =
  /** Both compilers accepted the value and agreed byte for byte. */
  | 'identical'
  /**
   * Both accepted and emitted *nothing* — agreement about no declaration at
   * all.
   *
   * Counted apart from `identical` because it is the one verdict that can be
   * reached without either compiler having an opinion about the value: a
   * property both of them drop, or a fixture that failed to carry the value in,
   * agrees just as loudly as a value they both spell the same way. A corpus
   * that quietly slid into this verdict would report perfect parity while
   * measuring nothing.
   */
  | 'identical-empty'
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

/** One corpus entry, plus what each compiler did with it. */
export type ReportEntry = LoadedCorpusEntry & {
  verdict: Verdict;
  rust: CompilerOutcome;
  babel: CompilerOutcome;
};

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
