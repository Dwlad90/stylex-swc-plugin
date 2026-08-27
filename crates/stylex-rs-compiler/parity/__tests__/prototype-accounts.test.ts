import path from 'node:path';

import { describe, expect, test } from 'vitest';

import { loadCorpus } from '../lib/corpus.js';
import { ACCOUNTS, accountOf, unrecorded } from '../lib/prototype-accounts.js';
import type { Account } from '../lib/prototype-accounts.js';
import type { LoadedCorpusEntry, ReportEntry, Verdict } from '../lib/types.js';
import { ACCEPTED, refused } from './support.js';

/**
 * The accounting half of the prototype sweep.
 *
 * An account is the sweep's one way to say "this divergence is not news", so
 * every way it could say that about something it should not is a case here: a
 * complaint it half-matches, a verdict it does not claim, a corpus row that has
 * lost the reason it points at. The last is the one the sweep cannot catch on
 * its own — a reason nobody can read still leaves a green run — and it is why
 * `unrecorded` exists.
 */

const corpus = loadCorpus(path.join(import.meta.dirname, '../corpus'));

function accountNamed(name: string): Account {
  const found = ACCOUNTS.find(account => account.name === name);
  if (found === undefined) throw new Error(`no account named ${name}`);

  return found;
}

/** A module row refused here with `sentence` and accepted by the reference. */
function moduleRow(
  verdict: Verdict,
  sentence: string,
  source = "import * as stylex from '@stylexjs/stylex';\n"
): ReportEntry {
  return {
    kind: 'module',
    set: 'prototype-sweep',
    id: 'test',
    label: 'a row',
    origin: '__tests__/prototype-accounts.test.ts',
    source,
    verdict,
    rust: refused(sentence),
    babel: ACCEPTED,
  };
}

describe('the accounts as written', () => {
  test('every account points at a corpus row that carries its reason', () => {
    expect(unrecorded(corpus)).toStrictEqual([]);
  });

  test('no two accounts claim the same complaint', () => {
    const complaints = ACCOUNTS.map(account => account.complaint);

    expect(new Set(complaints).size).toBe(complaints.length);
  });

  test('every account claims at least one verdict, and none claims agreement', () => {
    for (const account of ACCOUNTS) {
      expect(account.verdicts.length, account.name).toBeGreaterThan(0);
      expect(account.verdicts, account.name).not.toContain('identical');
    }
  });
});

describe('claiming a row', () => {
  test('a row whose complaint an account names is claimed', () => {
    const account = accountNamed('locale-sensitive method');
    const row = moduleRow(
      'acceptance-divergent',
      `Cannot fold 'toLocaleUpperCase' at compile time.\n${account.complaint}`
    );

    expect(accountOf(row)).toBe(account);
  });

  test('a row this compiler did not refuse is claimed by nothing', () => {
    const row: ReportEntry = { ...moduleRow('divergent', 'x'), rust: ACCEPTED };

    expect(accountOf(row)).toBeUndefined();
  });

  test('a verdict an account does not claim is not claimed by it', () => {
    const account = accountNamed('a mutation that disqualifies the binding');

    expect(accountOf(moduleRow('both-reject-divergent', account.complaint))).toBe(account);
    // The same complaint under a verdict the account does not list is a row
    // nobody has read: the reference compiler's behaviour moved under it.
    expect(accountOf(moduleRow('acceptance-divergent', account.complaint))).toBeUndefined();
  });

  test('a reworded complaint un-claims its rows rather than outliving them', () => {
    const account = accountNamed('a static that does not answer from the source');
    const reworded = account.complaint.replace('source alone', 'source by itself');

    expect(accountOf(moduleRow('both-reject-divergent', reworded))).toBeUndefined();
  });

  test('the account whose complaint names no rule also reads the evidence', () => {
    const account = accountNamed('a callback reached through a name');
    const withArrow =
      "import * as stylex from '@stylexjs/stylex';\nconst upper = (part) => part;\n";

    expect(accountOf(moduleRow('acceptance-divergent', account.complaint, withArrow))).toBe(
      account
    );
    // The same refusal on a subject that names no function is what the guard
    // says whenever it declines anything, and it is news.
    expect(accountOf(moduleRow('acceptance-divergent', account.complaint))).toBeUndefined();
  });
});

describe('the link to the corpus', () => {
  /**
   * The checked-in corpus, with one row edited to break an account's link.
   *
   * The change is applied field by field rather than by spreading a partial over
   * the row, since a spread over a discriminated union loses the discriminant and
   * would need an assertion to get it back.
   */
  function corpusWith(id: string, change: (row: LoadedCorpusEntry) => void): LoadedCorpusEntry[] {
    return corpus.map(entry => {
      if (entry.id !== id) return entry;
      const copy = structuredClone(entry);
      change(copy);

      return copy;
    });
  }

  const account = accountNamed('a read that escapes onto the function graph');

  test('a row that has been deleted is reported', () => {
    const without = corpus.filter(entry => entry.id !== account.recordedBy);

    expect(unrecorded(without)).toStrictEqual([
      { account, problem: 'no corpus entry carries that id' },
    ]);
  });

  test('a row that has lost its note is reported, since the reason went with it', () => {
    const emptied = corpusWith(account.recordedBy, row => {
      row.note = '   ';
    });

    expect(unrecorded(emptied)).toStrictEqual([
      { account, problem: 'the corpus entry carries no note' },
    ]);
  });

  test('a row that records no verdict is reported', () => {
    const broken = unrecorded(
      corpusWith(account.recordedBy, row => {
        delete row.expected;
      })
    );

    expect(broken).toStrictEqual([{ account, problem: 'the corpus entry records no verdict' }]);
  });

  test('a row whose verdict moved out from under the account is reported', () => {
    const moved = corpusWith(account.recordedBy, row => {
      row.expected = 'divergent';
    });

    expect(unrecorded(moved)).toStrictEqual([
      {
        account,
        problem: 'the corpus entry records divergent, which this account does not claim',
      },
    ]);
  });
});
