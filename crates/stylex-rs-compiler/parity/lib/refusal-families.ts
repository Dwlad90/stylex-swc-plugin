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
  invalidUtf8: 'String value contains invalid UTF-8 encoding.',
  keyHasNoName: 'The key has no name at compile time.',
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
 * Whether `value` carries a token that would close the declaration being
 * generated: a `;`, `{` or `}` that is neither escaped nor inside a string.
 *
 * The family below is the refusal *plus* this, not the refusal alone. Claiming
 * on the diagnostic by itself absorbs every false positive of the guard into the
 * column a reader is told not to act on — and it did: `A\;B` is an escaped
 * semicolon that closes nothing, the reference compiler emits it, and it sat
 * pinned here as a divergence produced on purpose until the guard was fixed.
 *
 * Deliberately the *shape*, not a second copy of the guard. It does not need to
 * agree with the Rust scan on every input to do its job; it needs to be
 * unwilling to vouch for a refusal it has no evidence for.
 */
function carriesUnescapedTerminator(value: string): boolean {
  let quote: '"' | "'" | undefined;

  for (let index = 0; index < value.length; index += 1) {
    const character = value[index];

    if (character === '\\') {
      index += 1;
      continue;
    }

    if (quote !== undefined) {
      if (character === quote) quote = undefined;
      continue;
    }

    if (character === '"' || character === "'") {
      quote = character;
      continue;
    }

    if (character === ';' || character === '{' || character === '}') return true;
  }

  return false;
}

/**
 * A surrogate half spelled as a JavaScript escape: `\uD800` or `\u{D800}`.
 *
 * Only the halves, and only the spellings that can name one. A code point above
 * `U+FFFF` is written as a whole pair, which is never an unpaired half, so the
 * braced form is read no further than four digits.
 */
const ESCAPED_CODE_UNIT = /u(?:\{0*([0-9a-fA-F]{1,4})\}|([0-9a-fA-F]{4}))/y;

/**
 * The text a row hands both compilers, as the code units a compiler reads.
 *
 * A corpus row is JavaScript source, so a surrogate reaches it in either of two
 * spellings: the code unit itself, or the `\uD800` escape that names it. Both
 * describe the same string, and a guard that knew only one would vouch for
 * whichever half of the corpus it happened to read.
 *
 * Deliberately the shape rather than a second JavaScript lexer. It does not
 * have to agree with the parser on every input; it has to be unwilling to
 * vouch for a refusal it has no evidence for.
 */
function readCodeUnits(text: string): string {
  let units = '';

  for (let index = 0; index < text.length; index += 1) {
    const character = text[index];

    if (character !== '\\') {
      units += character;
      continue;
    }

    // Matched in place with a sticky regex rather than against a slice: a
    // slice per backslash copies the rest of the subject, which is the square
    // of its length on text that is mostly escapes.
    ESCAPED_CODE_UNIT.lastIndex = index + 1;

    const escape = ESCAPED_CODE_UNIT.exec(text);

    if (escape === null) {
      // Any other escape names one character, and no such character is a
      // surrogate half. Stepping over it keeps an escaped backslash from
      // being read as the start of the escape that follows it.
      index += 1;
      continue;
    }

    // One of the two groups holds the digits, since a match is one form or the
    // other. The fallback is for a pattern edited later, and it fails in the
    // safe direction: no digits parses to `NaN`, which is no surrogate half, so
    // the guard declines to vouch rather than claiming a row it cannot read.
    const digits = escape[1] ?? escape[2] ?? '';

    // A code unit is the subject: `fromCodePoint` refuses the very halves
    // this reads.
    // oxlint-disable-next-line unicorn/prefer-code-point
    units += String.fromCharCode(Number.parseInt(digits, 16));
    index += escape[0].length;
  }

  return units;
}

/** The two things a text must hold before it is worth reading unit by unit. */
const SURROGATE_OR_ESCAPE = /[\uD800-\uDFFF]|\\u/;

/**
 * Whether the text a row hands both compilers holds an unpaired surrogate.
 *
 * A surrogate code unit is well-formed UTF-16 only as half of a pair, so a half
 * standing on its own is the thing that has no Rust string to hold it. This is
 * what the family below claims on, rather than the refusal sentence alone: the
 * sentence covers every key that has no name, of which a lone surrogate is one
 * case.
 */
function carriesLoneSurrogate(text: string): boolean {
  const isHighHalf = (unit: number): boolean => unit >= 0xd800 && unit <= 0xdbff;
  const isLowHalf = (unit: number): boolean => unit >= 0xdc00 && unit <= 0xdfff;

  // Almost every row carries neither a surrogate nor an escape that could name
  // one, and those rows need no reading at all.
  if (!SURROGATE_OR_ESCAPE.test(text)) return false;

  const units = readCodeUnits(text);

  for (let index = 0; index < units.length; index += 1) {
    // A code unit is the subject: `codePointAt` joins a pair back together and
    // answers nothing about the half this looks for.
    // oxlint-disable-next-line unicorn/prefer-code-point
    const unit = units.charCodeAt(index);

    // A low half reached here follows no high half, since a paired one is
    // stepped over below.
    if (isLowHalf(unit)) return true;

    if (!isHighHalf(unit)) continue;

    // A high half is paired only when a low half follows it. Past the end
    // `charCodeAt` answers `NaN`, which is no low half either.
    // oxlint-disable-next-line unicorn/prefer-code-point
    if (!isLowHalf(units.charCodeAt(index + 1))) return true;

    index += 1;
  }

  return false;
}

/** The text a row hands both compilers, whichever kind of question it asks. */
function subjectText(entry: ReportEntry): string {
  return entry.kind === 'module' ? entry.source : entry.value;
}

/**
 * The complaints this compiler is known to reach on a value the reference
 * compiler crashes on.
 *
 * Named rather than accepted wholesale. Under `both-reject-divergent` both sides
 * refused, and the family reads only the reference side's sentence — so a Rust
 * refusal it does not name (a reworded guard, a new over-refusal, a panic
 * surfaced as a throw) would be pinned as a crash declined rather than reported
 * as news, the moment the reference compiler happened to hit its `TypeError` on
 * the same value.
 */
const LOCAL_REFUSALS_BESIDE_A_REFERENCE_CRASH: readonly string[] = [
  REFUSALS.ruleBreakingToken,
  REFUSALS.unclosedComment,
];

/**
 * Every deliberate divergence, in the order the reports group them.
 *
 * Ordered most-populated first, because the order is what a reader scans: the
 * two guards at the top account for the overwhelming majority of rows in both
 * harnesses, and the four below them are single-digit in the curated corpus.
 *
 * The order is also precedence — the first family to claim a row keeps it. No
 * pair overlaps as the list stands: the one that did was a value both
 * rule-breaking here and crashing the reference compiler, which `reference
 * TypeError` won by sitting above the family that has since been removed. The
 * precedence is kept stated because a family added below an existing one
 * inherits it silently, and a reader adding the next family needs to know that
 * where they put it is a decision.
 *
 * A family leaves this list once nothing reaches it. The refusal a value
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
    // The refusal *and* the evidence for it, the way `style key off
    // Object.prototype` below is the shape rather than the key name. A module
    // subject has no single authored value to scan, so it is claimed on the
    // diagnostic alone.
    claims: entry =>
      refusedWith(entry, REFUSALS.ruleBreakingToken) &&
      (entry.kind === 'module' || carriesUnescapedTerminator(entry.value)),
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
    // Both sides are read, not just the reference's. See
    // `LOCAL_REFUSALS_BESIDE_A_REFERENCE_CRASH` for why the `both-reject`
    // half cannot be claimed on the reference sentence alone.
    claims: entry =>
      sentenceOf(entry, 'babel') === REFERENCE_TYPE_ERROR &&
      (entry.rust.status === 'ok' ||
        LOCAL_REFUSALS_BESIDE_A_REFERENCE_CRASH.some(refusal => refusedWith(entry, refusal))),
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
    name: 'lone surrogate in a name',
    reason:
      'Agreement would mean carrying a string the language cannot spell. A lone surrogate is ' +
      'well-formed UTF-16 and not well-formed Unicode, so it has no encoding a Rust string ' +
      'holds: this compiler refuses at the point the name is decoded, which is before the pass ' +
      'the reference compiler refuses it in — the parser for an export name, the value fold for ' +
      'a condition key. That is not an ordering that could be swapped to buy the same sentence; ' +
      'it is the absence of a representation. Substituting a replacement character would be ' +
      'worse than refusing, since it writes a name the source does not describe.',
    // Two verdicts, because the rows arrived that the single one was waiting
    // for. A name reached as a condition key refuses in both compilers, in
    // different words. The same name written or computed as a style key is one
    // the reference compiler accepts — it holds the surrogate and writes a
    // replacement character into the selector — so only this compiler refuses,
    // and the row reads as acceptance divergent. The reason above covers both:
    // what is missing is a representation, not an agreement about CSS.
    verdicts: ['both-reject-divergent', 'acceptance-divergent'],
    // Two sentences reach it, because a name is decoded in two places. A name
    // being *read* — an export specifier — refuses where the text is decoded. A
    // name being used as a property key refuses where the key is named, since a
    // key is `String(key)` and a lone surrogate has no string. One family
    // because the reason above is the same for both: there is no
    // representation, not an ordering that could be swapped.
    //
    // The second sentence is not surrogate-specific — a key with no name also
    // covers a function and this compiler's own values — so it is claimed only
    // where the row actually carries the half that has no representation.
    // Claiming on the sentence alone would swallow a future row for, say, a
    // function used as a computed key, and the count a reader acts on is the
    // rows no family claims.
    claims: entry =>
      refusedWith(entry, REFUSALS.invalidUtf8) ||
      (refusedWith(entry, REFUSALS.keyHasNoName) && carriesLoneSurrogate(subjectText(entry))),
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
