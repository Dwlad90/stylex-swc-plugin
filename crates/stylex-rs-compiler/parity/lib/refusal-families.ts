/**
 * The divergences this compiler produces on purpose, named once.
 *
 * Both harnesses print rows that are neither agreement nor a regression, and
 * every one of them is a refusal this compiler makes deliberately — a value it
 * declines that the reference compiler emits, or a crash on the reference side
 * that this compiler declines to reproduce. Printed undistinguished, they cost
 * a reader the same attention a real regression does, which is the failure mode
 * `expected` on a corpus entry exists to prevent: without it a permanent
 * divergence and a new one print the same, so the report can only be read by
 * someone who already knows which is which.
 *
 * `expected` cannot carry them all. Half the curated corpus's permanent rows
 * live in `corpus/harvested.json`, which is generated from the Rust sources and
 * rewritten wholesale by `pnpm parity:harvest` — a value written there is lost
 * on the next harvest. And the generated corpus next door has no entries at all
 * to write on: its rows are produced by crossing an alphabet, so hundreds of
 * distinct values reach the same refusal and an expectation per row would be a
 * fixture nobody can read.
 *
 * So a permanent divergence is pinned by *family* rather than by row. A family
 * is a reason, a verdict, and the test for whether a row is an instance of it;
 * a row no family claims is news, whatever its verdict, and that count is the
 * number a reader acts on. Both harnesses read this list, so neither can come
 * to disagree with the other about which refusal is deliberate.
 *
 * A family stops claiming a row the moment the row's verdict or the compiler's
 * wording moves, which is the loud direction: the row reappears as news and
 * someone looks at it. What is deliberately *not* pinned is a count — the
 * generated corpus's row count moves whenever the alphabet grows, and pinning
 * it would make every alphabet addition an expectation edit. What is checked
 * instead is that every family still claims something: see `unreachedFamilies`.
 */

import type { ReportEntry, Verdict } from './types.js';

/** One reason this compiler diverges on purpose, and the rows it accounts for. */
export interface RefusalFamily {
  /**
   * The family's name, as both reports print it. Short enough to sit in a
   * summary line, since that is where a reader meets it first.
   */
  readonly name: string;
  /**
   * Why the divergence is permanent, stated as what agreement would cost.
   *
   * "Known difference" is the sentence this field exists to refuse: it tells a
   * reader the row has been seen and nothing about whether seeing it again
   * should change their mind. What agreement would cost is the argument, and it
   * is the thing that stops being true if the trade-off ever changes.
   */
  readonly reason: string;
  /**
   * The verdicts a member of this family reads.
   *
   * Usually one. More where the same reason survives this compiler's own
   * behaviour changing around it: a reference crash is a reference crash whether
   * this compiler accepted the value or refused it for a fault of its own, and
   * the two read different verdicts.
   */
  readonly verdicts: readonly Verdict[];
  /**
   * Whether `entry` is an instance of this family.
   *
   * Called only for entries whose verdict is one of `verdicts`, so a test here
   * asks about the refusal and never about the verdict again.
   */
  readonly claims: (entry: ReportEntry) => boolean;
}

/**
 * The complaints this compiler writes for the guards below, matched as
 * prefixes because two of them interpolate the offending value.
 *
 * Matching the diagnostic text is what makes a family recognizable without an
 * entry to write an expectation on, and it means a reworded diagnostic stops a
 * family from claiming its rows. That is the intended direction: the wording is
 * what a refused build hands the author, so a change to it is a change someone
 * should read the report over.
 */
const REFUSALS = {
  ruleBreakingToken: 'Rule contains a `{`, `}` or `;` outside of a string or comment',
  unclosedComment: 'Rule contains an unclosed comment',
  unprefixedCustomProperty: 'Unprefixed custom properties:',
  nestedTooDeeply: 'Rule contains a value nested more deeply than the compiler supports',
} as const;

/**
 * How the reference compiler fails when it reads a node that is not there.
 *
 * Its own `TypeError`, not a diagnostic: several degenerate values reach a
 * branch that indexes a node list without checking it, and the property read
 * off `undefined` is what comes back out.
 */
const REFERENCE_TYPE_ERROR = "Cannot read properties of undefined (reading 'type')";

/**
 * Names every JavaScript object carries whether or not anything wrote them.
 *
 * An authored style key that collides with one of these is a CSS property name
 * to this compiler and an inherited method to the reference compiler, which is
 * a divergence about JavaScript rather than about CSS.
 */
const OBJECT_PROTOTYPE_NAMES: ReadonlySet<string> = new Set(
  Object.getOwnPropertyNames(Object.prototype)
);

/** The sentence a side wrote, or `undefined` where it did not refuse. */
function sentenceOf(entry: ReportEntry, side: 'rust' | 'babel'): string | undefined {
  const outcome = entry[side];
  return outcome.status === 'error' ? outcome.sentence : undefined;
}

/** Whether this compiler refused with the complaint `refusal` names. */
function refusedWith(entry: ReportEntry, refusal: string): boolean {
  return sentenceOf(entry, 'rust')?.startsWith(refusal) === true;
}

/**
 * Every deliberate divergence, in the order the reports group them.
 *
 * Ordered most-populated first, because the order is what a reader scans: the
 * two guards at the top account for the overwhelming majority of rows in both
 * harnesses, and the three below them are single-digit in the curated corpus.
 *
 * The order is also precedence — the first family to claim a row keeps it — and
 * one pair overlaps. A value that is rule-breaking here *and* crashes the
 * reference compiler could read as either `reference TypeError` or
 * `declaration-terminating token`, and the crash wins by sitting above it:
 * agreement on a crash would mean reproducing it, which is the stronger reason
 * of the two and the one a reader should be handed.
 *
 * A family is gone from this list once nothing reaches it. The refusal a value
 * carrying two faults used to earn here was one such: this compiler now runs
 * its declaration-terminating token guard after the two rejections the
 * reference compiler also makes, so those rows read agreement and there is
 * nothing left for a family to claim.
 */
export const REFUSAL_FAMILIES: readonly RefusalFamily[] = [
  {
    name: 'declaration-terminating token',
    reason:
      'Agreement would mean emitting a `;`, `{` or `}` into the stylesheet, where it closes ' +
      'the declaration being generated and turns the rest of the authored value into rules of ' +
      'its own. The reference compiler has no equivalent guard and emits the token; this is ' +
      'one of the two families agreement is not wanted on.',
    verdicts: ['acceptance-divergent'],
    claims: entry => refusedWith(entry, REFUSALS.ruleBreakingToken),
  },
  {
    name: 'reference TypeError',
    reason:
      'Agreement would mean reproducing a crash. The reference compiler reads `.type` off a ' +
      'node it never checked for, so a value that leaves it nothing to read — empty, ' +
      'whitespace-only, a lone control character, or a comment contributing no text — comes ' +
      'back as its own `TypeError`. This compiler drops the declaration instead — ' +
      'or refuses it for a fault of its own, which is the same divergence read under a ' +
      'both-reject verdict rather than an acceptance one.',
    verdicts: ['acceptance-divergent', 'both-reject-divergent'],
    claims: entry => sentenceOf(entry, 'babel') === REFERENCE_TYPE_ERROR,
  },
  {
    name: 'unclosed comment',
    reason:
      'Agreement would mean emitting an unclosed `/*` into the stylesheet, which comments out ' +
      'every rule injected after it. The reference compiler emits it.',
    verdicts: ['acceptance-divergent'],
    claims: entry => refusedWith(entry, REFUSALS.unclosedComment),
  },
  {
    name: 'unprefixed custom property',
    reason:
      'Agreement would mean accepting `var(x)` as a custom property reference. The `--` prefix ' +
      'is a StyleX rule rather than a CSS one, so what the reference compiler emits here is a ' +
      'value StyleX does not define — there is no CSS behaviour for the two to agree about.',
    verdicts: ['acceptance-divergent'],
    claims: entry => refusedWith(entry, REFUSALS.unprefixedCustomProperty),
  },
  {
    name: 'nesting past the recursion budget',
    reason:
      'Agreement would mean recursing until the stack runs out. Scanning a value builds a tree ' +
      'whose destructor recurses once per level, and a stack overflow aborts the process ' +
      'without a diagnostic, so the budget is refused with a message instead. The reference ' +
      'compiler recurses.',
    verdicts: ['acceptance-divergent'],
    claims: entry => refusedWith(entry, REFUSALS.nestedTooDeeply),
  },
  {
    name: 'style key off Object.prototype',
    reason:
      'Agreement would mean emitting one declaration per character of `[object Undefined]`. A ' +
      'style key spelled like a name every object inherits — `toString` — reaches the ' +
      "reference compiler's own inherited method rather than a CSS property. This compiler " +
      'treats the key as the property name it was written as.',
    verdicts: ['structurally-divergent'],
    // The key name alone is not the family: `toString` reaching a divergence for
    // some other reason is a row nobody has read. What is claimed is the shape
    // the inherited method produces — this compiler emitting the one declaration
    // the key was written as, and the reference compiler emitting a declaration
    // per character of the string its method returned.
    claims: entry =>
      entry.kind === 'declaration' &&
      OBJECT_PROTOTYPE_NAMES.has(entry.property) &&
      entry.rust.status === 'ok' &&
      entry.babel.status === 'ok' &&
      entry.rust.declarations.length === 1 &&
      entry.babel.declarations.length > 1,
  },
];

/**
 * The family that accounts for `entry`, or `undefined` where none does.
 *
 * An entry carrying its own `expected` verdict is not one a family may claim: a
 * hand-written expectation is more specific, and says why in its own note. That
 * is checked here rather than left to the caller, because a caller that forgot
 * would produce the silent expectation this whole mechanism exists to catch — a
 * family whose only rows are hand-pinned would count as reached while the report
 * printed no pinned rows for it.
 */
export function familyOf(entry: ReportEntry): RefusalFamily | undefined {
  if (entry.expected !== undefined) return undefined;

  return REFUSAL_FAMILIES.find(
    family => family.verdicts.includes(entry.verdict) && family.claims(entry)
  );
}

/**
 * The rows of `entries` each family accounts for, grouped.
 *
 * Both harnesses group by family — one to print a breakdown under its summary,
 * the other a count per family — and both then walk `REFUSAL_FAMILIES` to print
 * the groups in declaration order rather than in whichever order the corpus
 * reached them. Grouped here so the two cannot come to disagree about what a
 * group is, which is the argument the family list itself is built on.
 *
 * A family that claimed nothing is absent from the map rather than present and
 * empty, so a caller printing groups needs no emptiness test.
 */
export function groupByFamily(
  entries: readonly ReportEntry[]
): ReadonlyMap<RefusalFamily, ReportEntry[]> {
  const grouped = new Map<RefusalFamily, ReportEntry[]>();
  for (const entry of entries) {
    const family = familyOf(entry);
    if (family === undefined) continue;
    const claimed = grouped.get(family);
    if (claimed === undefined) grouped.set(family, [entry]);
    else claimed.push(entry);
  }

  return grouped;
}

/**
 * The families no row in `entries` reached.
 *
 * A family claiming nothing is the family-level form of an expectation that
 * silently started passing: it measures nothing, and either the refusal it
 * names is gone — which someone should read — or the corpus stopped reaching
 * it. Reported rather than counted per family, because a count moves whenever
 * an alphabet grows and this does not.
 *
 * Only meaningful over a whole corpus: a filtered run reaches a handful of
 * families by construction, so the caller decides when to ask.
 */
export function unreachedFamilies(entries: readonly ReportEntry[]): RefusalFamily[] {
  const reached = groupByFamily(entries);

  return REFUSAL_FAMILIES.filter(family => !reached.has(family));
}
