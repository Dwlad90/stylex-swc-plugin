/**
 * Guards the join between `loadSubject` and the native-binding check.
 *
 * The check itself has its own tests. They pass even if nobody calls it, so
 * they cannot show that a subject load asks the question. These tests replace
 * the check with a double and watch the call, because the real answer on this
 * platform is a process that stops.
 */

import path from 'node:path';

import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';

const assertBindingCanLoad = vi.fn();

vi.mock('../lib/native-bindings.js', async importOriginal => {
  const actual = await importOriginal<typeof import('../lib/native-bindings.js')>();
  return {
    ...actual,
    assertBindingCanLoad: (...args: Parameters<typeof actual.assertBindingCanLoad>) => {
      assertBindingCanLoad(...args);
    },
  };
});

const { loadSubject } = await import('../lib/subjects.js');

const packageDir = path.resolve(import.meta.dirname, '..', '..');

describe('loadSubject asks whether the binding can load', () => {
  beforeEach(() => {
    assertBindingCanLoad.mockReset();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  test('asks once for each subject', async () => {
    await loadSubject({ label: 'candidate', packageDir });

    expect(assertBindingCanLoad).toHaveBeenCalledTimes(1);
  });

  test('gives the label, the bindings of the package, and the loaded set', async () => {
    await loadSubject({ label: 'candidate', packageDir });
    const request = assertBindingCanLoad.mock.calls[0]?.[0];

    expect(request.label).toBe('candidate');
    expect(Array.isArray(request.bindings)).toBe(true);
    expect(request.loaded).toBeInstanceOf(Set);
  });

  // The subject must not load when the check refuses. On a real refusal the
  // load is what stops the process, so the error has to travel to the caller.
  test('passes on the refusal and does not load the subject', async () => {
    assertBindingCanLoad.mockImplementation(() => {
      throw new Error('two bindings');
    });

    await expect(loadSubject({ label: 'base', packageDir })).rejects.toThrow('two bindings');
  });

  // A missing entry is reported before the check, so the reader gets the
  // simpler cause first.
  test('reports a missing entry without asking the check', async () => {
    await expect(loadSubject({ label: 'base', packageDir: '/no/such/package' })).rejects.toThrow(
      /entry does not exist/
    );
    expect(assertBindingCanLoad).not.toHaveBeenCalled();
  });

  test('asks again for a second subject, so each load is checked', async () => {
    await loadSubject({ label: 'base', packageDir });
    await loadSubject({ label: 'candidate', packageDir });

    expect(assertBindingCanLoad).toHaveBeenCalledTimes(2);
  });
});
