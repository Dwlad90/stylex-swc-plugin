import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, test } from 'vitest';

import {
  REFUSAL_FAMILIES,
  familyOf,
  groupByFamily,
  unreachedFamilies,
} from '../lib/refusal-families.js';
import type { ReportEntry } from '../lib/types.js';
import { ACCEPTED, TERMINATOR_REFUSAL, accepted, refused, subject } from './support.js';

/**
 * A family pins a divergence both harnesses would otherwise print as news, so
 * what has to hold of it is narrowness: it claims the refusal it was written
 * for and nothing else. A family that claimed one row too many would hide a
 * regression behind a reason that does not apply to it, which is worse than
 * printing an extra row — the row at least gets read.
 */

/** The name of the family that claimed `entry`, for a readable expectation. */
function nameOf(entry: ReportEntry): string | undefined {
  return familyOf(entry)?.name;
}

describe('what a family claims', () => {
  test('a value refused for a declaration-terminating token it actually carries', () => {
    expect(
      nameOf(
        subject('acceptance-divergent', refused(TERMINATOR_REFUSAL), ACCEPTED, {
          property: 'color',
          value: 'red;blue',
        })
      )
    ).toBe('declaration-terminating token');
  });

  test('a module subject refused for the token, which has no value to scan', () => {
    // A module carries a whole source rather than one authored value, so the
    // shape check has nothing to read and the diagnostic is the whole claim.
    const entry: ReportEntry = {
      kind: 'module',
      set: 'test',
      id: 'test',
      origin: 'refusal-families.test.ts',
      label: 'a module refused for the token',
      note: 'the module arm of the family',
      source: 'export const x = 1;',
      verdict: 'acceptance-divergent',
      rust: refused(TERMINATOR_REFUSAL),
      babel: ACCEPTED,
    };

    expect(nameOf(entry)).toBe('declaration-terminating token');
  });

  test('a value refused for an unclosed comment', () => {
    expect(
      nameOf(
        subject('acceptance-divergent', refused('Rule contains an unclosed comment'), ACCEPTED)
      )
    ).toBe('unclosed comment');
  });

  test('an unprefixed custom property, whose complaint interpolates the value', () => {
    // Matched as a prefix for exactly this reason: the diagnostic names the
    // reference the author wrote, so no whole sentence could be written down.
    expect(
      nameOf(
        subject(
          'acceptance-divergent',
          refused('Unprefixed custom properties: var(someVariableName)'),
          ACCEPTED
        )
      )
    ).toBe('unprefixed custom property');
  });

  test('a value past the nesting budget, whose complaint interpolates the depth', () => {
    expect(
      nameOf(
        subject(
          'acceptance-divergent',
          refused(
            'Rule contains a value nested more deeply than the compiler supports (limit 64, found 65)'
          ),
          ACCEPTED
        )
      )
    ).toBe('nesting past the recursion budget');
  });

  test("the reference compiler's own TypeError, where this compiler accepted", () => {
    expect(
      nameOf(
        subject(
          'acceptance-divergent',
          ACCEPTED,
          refused("Cannot read properties of undefined (reading 'type')")
        )
      )
    ).toBe('reference TypeError');
  });

  test('a name this compiler cannot decode, refused before the pass upstream refuses it in', () => {
    // The two shapes it accounts for: an export name, which upstream refuses in
    // its parser, and a condition key, which upstream carries as a JavaScript
    // string and refuses at the fold. Both read `both-reject-divergent`, since
    // both compilers refuse and word it differently.
    expect(
      nameOf(
        subject(
          'both-reject-divergent',
          refused('String value contains invalid UTF-8 encoding.'),
          refused("An export name cannot include a lone surrogate, found '\\ud83d'. (2:9)")
        )
      )
    ).toBe('lone surrogate in a name');

    expect(
      nameOf(
        subject(
          'both-reject-divergent',
          refused('String value contains invalid UTF-8 encoding.'),
          refused('Invalid pseudo or at-rule.')
        )
      )
    ).toBe('lone surrogate in a name');
  });

  test('a style key spelled like a name every object inherits', () => {
    // The shape the inherited method produces: one declaration here, and one per
    // character of what the method returned there.
    const perCharacter = accepted(['[:', 'o:', 'b:', 'j:']);

    expect(
      nameOf(subject('structurally-divergent', ACCEPTED, perCharacter, { property: 'toString' }))
    ).toBe('style key off Object.prototype');
    expect(
      nameOf(subject('structurally-divergent', ACCEPTED, perCharacter, { property: 'valueOf' }))
    ).toBe('style key off Object.prototype');
  });
});

describe('what a family leaves as news', () => {
  test('two refusals no family claims, now that neither guard speaks first', () => {
    // The family that used to claim this is gone, because the row is gone: the
    // declaration-terminating token guard now runs after the two rejections the
    // reference compiler also makes, so a value carrying both faults earns the
    // same complaint on both sides and never reaches a divergent verdict. Were
    // the guard reordered back, the row would return as news — which is louder
    // than a family quietly re-claiming it.
    expect(
      nameOf(
        subject(
          'both-reject-divergent',
          refused('Rule contains a `{`, `}` or `;` outside of a string or comment'),
          refused('Rule contains an unclosed function')
        )
      )
    ).toBeUndefined();
  });

  test('a verdict a family produces, reached by a refusal it does not name', () => {
    // The narrowness that matters: `acceptance-divergent` is the verdict of four
    // families, and a row that reads it for a fifth reason is news.
    expect(
      nameOf(subject('acceptance-divergent', refused('Invalid pseudo or at-rule.'), ACCEPTED))
    ).toBeUndefined();
  });

  test('the right refusal read from the wrong side', () => {
    // A refusal family says which compiler refused as well as why. The reference
    // compiler complaining about a rule-breaking token while this one accepts is
    // the opposite divergence, and nothing here has looked at it.
    expect(
      nameOf(
        subject(
          'acceptance-divergent',
          ACCEPTED,
          refused('Rule contains a `{`, `}` or `;` outside of a string or comment')
        )
      )
    ).toBeUndefined();
  });

  test('the right refusal under a verdict the family does not read', () => {
    // The same complaint on a `divergent` row means both compilers accepted and
    // spelled a value differently, which is the one thing this harness exists to
    // find. A family matched on its sentence alone would swallow it.
    expect(
      nameOf(
        subject(
          'divergent',
          refused('Rule contains a `{`, `}` or `;` outside of a string or comment'),
          ACCEPTED
        )
      )
    ).toBeUndefined();
  });

  test('a reference complaint the local guard is not known to preempt', () => {
    // The family says both compilers refused for reasons already understood. A
    // reference refusal nobody here has looked at is news even though this side
    // said what it always says.
    expect(
      nameOf(
        subject(
          'both-reject-divergent',
          refused('Rule contains a `{`, `}` or `;` outside of a string or comment'),
          refused('Invalid pseudo or at-rule.')
        )
      )
    ).toBeUndefined();
  });

  test('an inherited key name where the shape is not the one it produces', () => {
    // The key name alone is not the family. `toString` diverging structurally
    // for some other reason — here both sides emitting one declaration — is a
    // row nobody has read.
    expect(
      nameOf(subject('structurally-divergent', ACCEPTED, ACCEPTED, { property: 'toString' }))
    ).toBeUndefined();
  });

  test('the same undecodable name where only this compiler refused', () => {
    // The reason survives the reference compiler accepting the name — the
    // absence of a representation does not depend on what the other side did
    // with it — and rows now read that verdict: a lone surrogate written or
    // computed as a style key is held upstream and written out as a replacement
    // character, so only this compiler refuses. Both sentences a name is
    // refused with reach the family, since a name is decoded in two places.
    for (const refusal of [
      'String value contains invalid UTF-8 encoding.',
      'The key has no name at compile time.',
    ]) {
      expect(
        nameOf(subject('acceptance-divergent', refused(refusal), ACCEPTED, { value: '\uD800' }))
      ).toBe('lone surrogate in a name');
    }
  });

  test('a lone surrogate spelled as an escape rather than as the code unit', () => {
    // A corpus row is JavaScript source, so the half is written `\uD800` far
    // more often than it is carried. Both spellings name the same string.
    for (const value of ['\\uD800', '\\u{D800}', 'a\\uDC00b', 'a\\uD800', '\\uD800\\n']) {
      expect(
        nameOf(
          subject(
            'acceptance-divergent',
            refused('The key has no name at compile time.'),
            ACCEPTED,
            {
              value,
            }
          )
        )
      ).toBe('lone surrogate in a name');
    }
  });

  test('a character between two halves keeps them apart', () => {
    // The halves are adjacent only if nothing sits between them. An escape that
    // names no code unit still names a character, so `\\uD800\\n\\uDC00` holds
    // two lone halves and not one pair -- which is what JavaScript itself
    // answers for the same text. A reader that passed over the escape read the
    // two halves as adjacent and vouched for a row carrying a surrogate it had
    // not seen.
    for (const value of ['\\uD800\\n\\uDC00', '\\uD800a\\uDC00', '\\u{D800}\\t\\u{DC00}']) {
      expect(
        nameOf(
          subject(
            'acceptance-divergent',
            refused('The key has no name at compile time.'),
            ACCEPTED,
            {
              value,
            }
          )
        )
      ).toBe('lone surrogate in a name');
    }

    // The pair itself, with nothing between, is still a pair.
    expect(
      nameOf(
        subject('acceptance-divergent', refused('The key has no name at compile time.'), ACCEPTED, {
          value: '\\uD800\\uDC00',
        })
      )
    ).toBeUndefined();
  });

  test('a backslash before a half hides it, and the row prints as news', () => {
    // The guard steps two positions past an escape that names no code unit, so
    // a backslash directly before a literal half steps over the half as well
    // and never judges it. The half is there, and the guard does not say so.
    //
    // That is the safe direction of the two: the family declines to vouch, and
    // the row prints as one nobody has read rather than as a divergence
    // produced on purpose. It is pinned because the other direction -- claiming
    // a row on a surrogate the guard has not seen -- is what the family exists
    // to stop, and a change here has to be read as the trade it is.
    expect(
      nameOf(
        subject('acceptance-divergent', refused('The key has no name at compile time.'), ACCEPTED, {
          value: '\\\uD800',
        })
      )
    ).toBeUndefined();

    // Without the backslash in front of it, the same half is judged and the
    // row is the family's.
    expect(
      nameOf(
        subject('acceptance-divergent', refused('The key has no name at compile time.'), ACCEPTED, {
          value: '\uD800',
        })
      )
    ).toBe('lone surrogate in a name');
  });

  test('a key with no name that carries no surrogate is news', () => {
    // The sentence covers every key that has no name — a function, this
    // compiler's own values, and a lone surrogate. Only the last one is this
    // family, so a function used as a computed key has to print as a row
    // nobody has read rather than as a divergence produced on purpose.
    for (const value of ['red', '\\uD800\\uDC00', '\\\\uD800', '\\u{1F600}']) {
      expect(
        nameOf(
          subject(
            'acceptance-divergent',
            refused('The key has no name at compile time.'),
            ACCEPTED,
            {
              value,
            }
          )
        )
      ).toBeUndefined();
    }
  });

  test('the reference compiler refusing an encoding, where this compiler did not', () => {
    // The undecodable-name family says which compiler could not hold the string,
    // and it is this one. The reference compiler complaining about an encoding
    // while this compiler accepts the value is the opposite divergence, and
    // nothing here has looked at it.
    expect(
      nameOf(
        subject(
          'acceptance-divergent',
          ACCEPTED,
          refused('String value contains invalid UTF-8 encoding.')
        )
      )
    ).toBeUndefined();
  });

  test('a reference crash under a both-reject verdict is the same family', () => {
    // The reason survives this compiler's own behaviour changing around it: a
    // reference crash is a reference crash whether this side accepted the value
    // or refused it for a fault of its own, so the family reads both verdicts.
    expect(
      nameOf(
        subject(
          'both-reject-divergent',
          refused('Rule contains a `{`, `}` or `;` outside of a string or comment'),
          refused("Cannot read properties of undefined (reading 'type')")
        )
      )
    ).toBe('reference TypeError');
  });

  test('a complaint that merely contains a pinned one', () => {
    // Prefix-matched, not substring-matched: a future diagnostic that quotes
    // this one inside a longer sentence is a different refusal.
    expect(
      nameOf(
        subject(
          'acceptance-divergent',
          refused('Nested rule: Rule contains an unclosed comment'),
          ACCEPTED
        )
      )
    ).toBeUndefined();
  });

  test('a row that carries its own expectation, even one a family would claim', () => {
    // The hand-written expectation is the more specific of the two and says why
    // in its own note, so a family must not relabel it. Checked inside `familyOf`
    // rather than at each caller: a caller that forgot would leave a family
    // counting as reached while the report printed no pinned rows for it, which
    // is exactly the silent expectation the mechanism exists to catch.
    const pinnedByHand: ReportEntry = {
      ...subject('acceptance-divergent', refused('Rule contains an unclosed comment'), ACCEPTED),
      expected: 'acceptance-divergent',
    };

    expect(nameOf(pinnedByHand)).toBeUndefined();
    expect(unreachedFamilies([pinnedByHand])).toHaveLength(REFUSAL_FAMILIES.length);
  });

  test('agreement is never claimed, whatever either compiler said', () => {
    // A family is asked only about a divergent row by both harnesses, and it
    // must not be the thing that decides that: agreement is decided by the
    // verdict, and no family reads one of the agreeing verdicts.
    for (const verdict of ['identical', 'identical-empty', 'both-reject'] as const) {
      expect(REFUSAL_FAMILIES.some(family => family.verdicts.includes(verdict))).toBe(false);
    }
  });
});

describe('a family will not vouch for a refusal it has no evidence for', () => {
  // The failure mode these guard against is the one the families exist to avoid
  // in reverse: not a family too narrow to claim its row, but one so broad it
  // absorbs a genuine over-refusal into the column a reader is told not to act
  // on. Both of these were live.

  test('an escaped terminator is not a terminator, so the family declines it', () => {
    // `A\;B` is an escaped semicolon: it closes nothing, and the reference
    // compiler emits `font-family:A\;B`. It sat pinned under this family until
    // the guard was fixed, which is the whole reason the shape is checked. Were
    // the over-refusal to return, this row would be news again.
    expect(
      nameOf(
        subject('acceptance-divergent', refused(TERMINATOR_REFUSAL), ACCEPTED, {
          property: 'fontFamily',
          value: 'A\\;B',
        })
      )
    ).toBeUndefined();
  });

  test('a terminator inside a string is not one either', () => {
    expect(
      nameOf(
        subject('acceptance-divergent', refused(TERMINATOR_REFUSAL), ACCEPTED, {
          property: 'content',
          value: '";"',
        })
      )
    ).toBeUndefined();
  });

  test('a reference crash beside a Rust refusal the family does not name', () => {
    // Under `both-reject-divergent` the family reads only the reference side. A
    // Rust refusal it has never been shown — a reworded guard, a new
    // over-refusal, a panic surfaced as a throw — must not be pinned merely
    // because the reference compiler crashed on the same value.
    expect(
      nameOf(
        subject(
          'both-reject-divergent',
          refused('Some complaint nobody has read yet'),
          refused("Cannot read properties of undefined (reading 'type')")
        )
      )
    ).toBeUndefined();
  });

  test('a reference crash beside a Rust refusal the family does name', () => {
    expect(
      nameOf(
        subject(
          'both-reject-divergent',
          refused('Rule contains an unclosed comment'),
          refused("Cannot read properties of undefined (reading 'type')")
        )
      )
    ).toBe('reference TypeError');
  });
});

describe('a broken expectation reports loudly', () => {
  test('a reworded diagnostic stops being pinned and becomes news', () => {
    // The gate, demonstrated rather than assumed. A family is recognized by the
    // complaint this compiler writes, so a rewording of that complaint — which
    // is a change to what a refused build hands the author — un-pins every row
    // it accounted for, and they come back as unexpected.
    const reworded = subject(
      'acceptance-divergent',
      refused('Rule contains a semicolon, brace or bracket outside of a string or comment'),
      ACCEPTED
    );

    expect(nameOf(reworded)).toBeUndefined();
  });

  test('a family no row reaches is reported rather than passing quietly', () => {
    const onlyUnclosedComments = [
      subject('acceptance-divergent', refused('Rule contains an unclosed comment'), ACCEPTED),
    ];

    const unreached = unreachedFamilies(onlyUnclosedComments).map(family => family.name);

    expect(unreached).not.toContain('unclosed comment');
    expect(unreached).toContain('declaration-terminating token');
    expect(unreached).toHaveLength(REFUSAL_FAMILIES.length - 1);
  });

  test('a corpus reaching every family leaves nothing unreached', () => {
    // The state the checked-in corpus is in, asserted here so the case that
    // matters — an empty list — is covered by something cheaper than a full run.
    const everyFamily = [
      subject('acceptance-divergent', refused(TERMINATOR_REFUSAL), ACCEPTED, {
        property: 'color',
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
      subject('structurally-divergent', ACCEPTED, accepted(['[:', 'o:']), { property: 'toString' }),
      subject(
        'both-reject-divergent',
        refused('String value contains invalid UTF-8 encoding.'),
        refused('Invalid pseudo or at-rule.')
      ),
      subject(
        'both-reject-divergent',
        refused('Rule contains a `{`, `}` or `;` outside of a string or comment'),
        refused('Rule contains an unclosed string')
      ),
    ];

    expect(unreachedFamilies(everyFamily)).toEqual([]);
  });
});

describe('grouping', () => {
  test('rows land under the family that claimed them, and news lands nowhere', () => {
    // One grouping for both harnesses, so a breakdown and a per-family count
    // cannot come to disagree about what a group is.
    const rows = [
      subject('acceptance-divergent', refused('Rule contains an unclosed comment'), ACCEPTED),
      subject('acceptance-divergent', refused('Rule contains an unclosed comment'), ACCEPTED),
      subject('acceptance-divergent', refused('Invalid pseudo or at-rule.'), ACCEPTED),
    ];

    const grouped = groupByFamily(rows);
    const unclosed = REFUSAL_FAMILIES.find(family => family.name === 'unclosed comment');

    expect(grouped.size).toBe(1);
    expect(unclosed).toBeDefined();
    expect(grouped.get(unclosed!)).toHaveLength(2);
  });

  test('a family that claimed nothing is absent rather than present and empty', () => {
    // So a caller printing groups needs no emptiness test, and a count read off
    // the map is never a zero standing for a family nobody reached.
    expect(groupByFamily([]).size).toBe(0);
  });
});

describe('the list itself', () => {
  test('no two families share a name', () => {
    // Both harnesses print the name and one of them groups by it, so a
    // duplicate would merge two reasons under one heading.
    const names = REFUSAL_FAMILIES.map(family => family.name);
    expect(new Set(names).size).toBe(names.length);
  });

  test('the README table lists exactly the families the code declares', () => {
    // The reason is written twice — once as prose a reader meets in the README,
    // once as the `reason` a report prints — and the pair is what drifts. The
    // wording is deliberately not compared: pinning prose would fail on an
    // honest rewrite. What is compared is the roster, which is the half that
    // going stale makes the README lie about.
    const readme = fs.readFileSync(
      path.join(path.dirname(fileURLToPath(import.meta.url)), '../README.md'),
      'utf8'
    );
    // Cut to the section first. The file carries several tables and the verdict
    // one above has the same shape, so a pattern loose enough to match rows has
    // to be told which rows.
    const section = readme.split('### Refusal families')[1]?.split('\n### ')[0] ?? '';
    const tabulated = [...section.matchAll(/^\| +(.+?) +\| .+ \|$/gm)]
      // The README writes the one name that is a code identifier in backticks,
      // which is presentation rather than part of the name.
      .map(row => (row[1] ?? '').trim().replaceAll('`', ''))
      .filter(name => name !== 'Family' && !name.startsWith('---'));

    expect(tabulated).toEqual(REFUSAL_FAMILIES.map(family => family.name));
  });
});
