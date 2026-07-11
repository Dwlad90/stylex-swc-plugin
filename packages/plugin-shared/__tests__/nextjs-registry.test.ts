import { describe, expect, test } from 'vitest';

import { mergeStyleXRulesInto, publishStyleXRules } from '../src/nextjs-registry';

import type { StyleXRulesMap } from '../src/nextjs-registry';

describe('Next.js cross-compiler registry', () => {
  test('isolates rules by compiler context', () => {
    const appOneServer: StyleXRulesMap = new Map([
      ['/app-one/server.tsx', [['app-one', { ltr: '.app-one{color:red}', rtl: null }, 3000]]],
    ]);
    const appTwoServer: StyleXRulesMap = new Map([
      ['/app-two/server.tsx', [['app-two', { ltr: '.app-two{color:blue}', rtl: null }, 3000]]],
    ]);
    const appOneClient: StyleXRulesMap = new Map();

    publishStyleXRules('/app-one', 'server', appOneServer);
    publishStyleXRules('/app-two', 'server', appTwoServer);
    mergeStyleXRulesInto('/app-one', 'client', appOneClient);

    expect(appOneClient.has('/app-one/server.tsx')).toBe(true);
    expect(appOneClient.has('/app-two/server.tsx')).toBe(false);
  });
});
