import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, test } from 'vitest';

import {
  SHAPES,
  SURFACES,
  SWEEP_SET,
  arityOrder,
  chooseArguments,
  labelFor,
  methodsOf,
  renderingFor,
  shortfalls,
  sweep,
  vectorsOfArity,
} from '../lib/prototype-surface.js';
import type { Surface } from '../lib/prototype-surface.js';
import { phfSetMembers } from '../lib/rust-source.js';

/**
 * The generation half of the prototype sweep, which is where its whole claim
 * lives: what the sweep can say is "every method the language carries here",
 * and every way that sentence can quietly stop being true is a case below —
 * a surface that enumerates nothing, an argument search that answers a value no
 * declaration carries, a shape that stops varying, an answer that differs
 * between runs.
 *
 * The script beside it is not tested, and cannot be: it prints. What it decides
 * is here and in `prototype-accounts.test.ts`.
 */

/** Looked up by name rather than by index, so reordering `SURFACES` is free. */
function surfaceNamed(name: string): Surface {
  const found = SURFACES.find(surface => surface.name === name);
  if (found === undefined) throw new Error(`no surface named ${name}`);

  return found;
}

const stringPrototype = surfaceNamed('String.prototype');
const arrayPrototype = surfaceNamed('Array.prototype');
const mathNamespace = surfaceNamed('Math');
const arrayNamespace = surfaceNamed('Array');

describe('reading the surface off the language', () => {
  test('every declared surface carries methods', () => {
    for (const surface of SURFACES) {
      expect(methodsOf(surface).length, surface.name).toBeGreaterThan(0);
    }
  });

  test('a method nobody listed is enumerated, which is the whole claim', () => {
    // Neither name appears anywhere in the sweep's own sources: one is Annex B
    // and one is ES2024, and both are exactly the kind of method a curated table
    // is written without.
    expect(methodsOf(stringPrototype)).toContain('blink');
    expect(methodsOf(stringPrototype)).toContain('isWellFormed');
  });

  test('a value-carrying property is not a method', () => {
    // `Math.PI` is a member read rather than a call, so it is not this sweep's
    // subject -- and a sweep that took it would generate `Math.PI(...)`.
    expect(methodsOf(mathNamespace)).not.toContain('PI');
  });

  test('a surface whose getter throws loses that name rather than the run', () => {
    const hostile: Surface = {
      kind: 'namespace',
      name: 'Hostile',
      floor: 1,
      target: Object.defineProperties(
        {},
        {
          fine: { value: () => 'a', enumerable: false },
          angry: {
            get: () => {
              throw new Error('read me and see');
            },
          },
        }
      ),
    };

    expect(methodsOf(hostile)).toStrictEqual(['fine']);
  });
});

describe('the argument search', () => {
  test('the declared arity is tried first, then narrower, then wider', () => {
    expect(arityOrder(2)).toStrictEqual([2, 1, 0]);
    expect(arityOrder(1)).toStrictEqual([1, 0, 2]);
    expect(arityOrder(0)).toStrictEqual([0, 1, 2]);
  });

  test('an arity wider than the pool is tried is clamped rather than dropped', () => {
    // `Object.defineProperty` declares three. Clamped to two, which is why it
    // reports as unexercised rather than as covered.
    expect(arityOrder(3)).toStrictEqual([2, 1, 0]);
  });

  test('one vector of no arguments exists, so a nullary method is asked', () => {
    expect(vectorsOfArity(0)).toStrictEqual([[]]);
  });

  test('the vectors of an arity are the pool crossed with itself', () => {
    const single = vectorsOfArity(1);
    const pair = vectorsOfArity(2);

    expect(pair.length).toBe(single.length * single.length);
    // Ordered so the first vector is the first pool member in every position,
    // which is what makes a chosen subject reproducible from the source.
    const first = single[0]?.[0];
    expect(first).toBeTypeOf('string');
    expect(pair[0]).toStrictEqual([first, first]);
  });

  test('a method the pool answers for is asked with the arguments that answer', () => {
    const chosen = chooseArguments(stringPrototype, 'padStart');

    expect(chosen).toMatchObject({ property: 'content' });
    expect('expression' in chosen && chosen.expression).toContain('padStart(');
  });

  test('a method whose answer moves between evaluations is refused as such', () => {
    expect(chooseArguments(mathNamespace, 'random')).toMatchObject({
      rejection: 'nondeterministic',
    });
  });

  test('a method answering nothing usable is refused, and says what it answered', () => {
    const chosen = chooseArguments(arrayPrototype, 'forEach');

    expect(chosen).toMatchObject({ rejection: 'unusable', detail: 'answers undefined' });
  });

  test('a rejected promise does not outlive the search', async () => {
    // `Array.fromAsync()` with no argument rejects. A rejection nothing reads
    // takes the process down on the next tick, so the search settles it -- and
    // this case is only evidence of that if the tick actually passes.
    expect(chooseArguments(arrayNamespace, 'fromAsync')).toMatchObject({ rejection: 'unusable' });
    await new Promise(resolve => {
      setImmediate(resolve);
    });
  });
});

describe('rendering an answer into a declaration', () => {
  test('a string and a finite number go into a declaration as they are', () => {
    expect(renderingFor('a')?.render('call()')).toBe('call()');
    expect(renderingFor('a')?.property).toBe('content');
    expect(renderingFor(1.5)?.property).toBe('zIndex');
  });

  test('a number no declaration can spell has no rendering', () => {
    expect(renderingFor(Number.NaN)).toBeUndefined();
    expect(renderingFor(Number.POSITIVE_INFINITY)).toBeUndefined();
  });

  test('an array, an object and a boolean each take one further call', () => {
    expect(renderingFor([1])?.render('call()')).toBe("call().join(', ')");
    expect(renderingFor({ a: 1 })?.render('call()')).toBe("Object.keys(call()).join(', ')");
    expect(renderingFor(true)?.render('call()')).toBe('String(call())');
  });

  test('a value with no declaration form at all has no rendering', () => {
    for (const value of [undefined, null, () => 'a', Symbol('a'), 1n, new Map()]) {
      expect(renderingFor(value)).toBeUndefined();
    }
  });
});

describe('the two shapes', () => {
  const swept = sweep([stringPrototype]);
  const asked = swept.asked.filter(one => one.method === 'toUpperCase');

  test('a prototype method is asked once per shape', () => {
    expect(asked.map(one => one.shape)).toStrictEqual([...SHAPES]);
  });

  test('the written shape carries the receiver in the style value', () => {
    const written = asked.find(one => one.shape === 'written')!;

    expect(written.subject.source).toContain("'AbC dEf'.toUpperCase()");
    // No prelude at all: the receiver is in the style value, so nothing above
    // the create call carries it.
    expect(written.subject.source).not.toContain('const text');
  });

  test('the named shape moves the same receiver into a declaration', () => {
    const named = asked.find(one => one.shape === 'named')!;

    expect(named.subject.source).toContain("const text = 'AbC dEf';");
    expect(named.subject.source).toContain('text.toUpperCase()');
  });

  test('a namespace call with no arguments is one subject, not the same one twice', () => {
    // Nothing to name, so the two shapes would be one question asked twice.
    const maths = sweep([mathNamespace]);
    for (const one of maths.exercised) {
      const shapes = one.asked.map(each => each.shape);
      expect(new Set(shapes).size, one.method).toBe(shapes.length);
    }
  });

  test('a namespace surface names its arguments instead of its receiver', () => {
    const maths = sweep([mathNamespace]);
    const named = maths.asked.find(one => one.method === 'trunc' && one.shape === 'named')!;

    expect(named.subject.source).toMatch(/const given0 = .+;/);
    expect(named.subject.source).toContain('Math.trunc(given0)');
  });

  test('a rendering survives the receiver being rewritten', () => {
    const arrays = sweep([arrayPrototype]);
    const named = arrays.asked.find(one => one.method === 'concat' && one.shape === 'named')!;

    // The terminal `join` is what makes the array answer a declaration, and it
    // has to still be there once the receiver became a name.
    expect(named.subject.source).toContain('list.concat(');
    expect(named.subject.source).toContain(".join(', ')");
  });

  test('every subject is a module in the sweep set, with a unique id', () => {
    const everything = sweep();
    const ids = everything.asked.map(one => one.subject.id);

    expect(new Set(ids).size).toBe(ids.length);
    for (const one of everything.asked) {
      expect(one.subject.kind).toBe('module');
      expect(one.subject.set).toBe(SWEEP_SET);
      expect(one.subject.source).toContain('stylex.create');
    }
  });

  test('a method is either exercised or reported, never dropped', () => {
    const everything = sweep();
    const seen = new Set([
      ...everything.exercised.map(one => `${one.surface}.${one.method}`),
      ...everything.unexercised.map(one => `${one.surface}.${one.method}`),
    ]);

    for (const surface of SURFACES) {
      for (const method of methodsOf(surface)) {
        expect(seen.has(`${surface.name}.${method}`), `${surface.name}.${method}`).toBe(true);
      }
    }
  });
});

describe('naming a row', () => {
  test('a prototype row names the receiver shape', () => {
    expect(labelFor(stringPrototype, 'trim', 'named')).toBe(
      '`trim` on a receiver named, off String.prototype'
    );
    expect(labelFor(stringPrototype, 'trim', 'written')).toContain('written out');
  });

  test('a namespace row names the arguments instead', () => {
    expect(labelFor(mathNamespace, 'trunc', 'named')).toBe('`Math.trunc` on arguments named');
  });
});

describe('the recorded coverage floor', () => {
  test('every surface still reaches the floor recorded for it', () => {
    // The gate itself, run over the real surfaces. A change to the argument
    // pool, the arities or `renderingFor` that stops a prototype answering
    // fails here rather than printing a smaller number beside a green sweep.
    expect(shortfalls(sweep())).toStrictEqual([]);
  });

  test('a floor is a number a run has to clear, never zero', () => {
    for (const surface of SURFACES) {
      expect(surface.floor, surface.name).toBeGreaterThan(0);
    }
  });

  test('a surface that asks about less than its floor is reported with both counts', () => {
    const raised: Surface = { ...stringPrototype, floor: 10_000 };

    expect(shortfalls(sweep([raised]))).toStrictEqual([
      { surface: 'String.prototype', exercised: methodsOfExercised(raised), floor: 10_000 },
    ]);
  });

  test('a surface that exactly meets its floor is not a shortfall', () => {
    const met: Surface = { ...stringPrototype, floor: methodsOfExercised(stringPrototype) };

    expect(shortfalls(sweep([met]))).toStrictEqual([]);
  });

  test('a surface that answers for nothing at all is a shortfall, not a silent pass', () => {
    // The failure a generated harness is most prone to: the surface changes
    // shape, no candidate answers, and the run agrees about no method at all.
    const empty: Surface = { kind: 'namespace', name: 'Empty', target: {}, floor: 1 };

    expect(shortfalls(sweep([empty]))).toStrictEqual([
      { surface: 'Empty', exercised: 0, floor: 1 },
    ]);
  });

  test('a narrower selection is judged on the surfaces it swept, not on the rest', () => {
    // `--surface Math` covers one surface, and the nine it did not ask about
    // have not fallen below anything.
    expect(shortfalls(sweep([mathNamespace]))).toStrictEqual([]);
  });

  test('every shortfall is named, rather than the first one standing for the rest', () => {
    const raised: Surface[] = [
      { ...stringPrototype, floor: 10_000 },
      { ...mathNamespace, floor: 10_000 },
    ];

    expect(shortfalls(sweep(raised)).map(one => one.surface)).toStrictEqual([
      'String.prototype',
      'Math',
    ]);
  });
});

/** What one surface actually exercises, so a case can state a floor against it. */
function methodsOfExercised(surface: Surface): number {
  return sweep([surface]).exercised.length;
}

describe('the surfaces and the compiler agree about the callees', () => {
  /**
   * The claim `SURFACES` makes in prose — that its namespaces are exactly the
   * compiler's `VALID_CALLEES` — read off the Rust source rather than trusted.
   *
   * Without this, a sixth callee added to the compiler is swept by nobody,
   * which is the precise failure the sweep exists to prevent: the surface grows
   * and the harness that measures it does not.
   */
  const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
  const constants = path.join(workspaceRoot, 'crates/stylex-constants/src/constants/common.rs');
  const callees = phfSetMembers(fs.readFileSync(constants, 'utf8'), 'VALID_CALLEES');

  test('the compiler declares the callees this test reads', () => {
    // Asserted on its own, so a moved or renamed declaration says that rather
    // than reporting the sweep's namespaces as the ones that drifted.
    expect(callees).not.toBeUndefined();
  });

  test('the namespace surfaces are exactly the callees the compiler declares', () => {
    const swept = SURFACES.filter(surface => surface.kind === 'namespace').map(
      surface => surface.name
    );

    expect(swept.toSorted()).toStrictEqual(callees?.toSorted());
  });
});
