/**
 * The guard on the JavaScript built-ins that the reference calls.
 *
 * The native sheet is the reference's sheet only while the built-ins the
 * reference reaches do what the specification says. A program can replace
 * one (`String.prototype.localeCompare = …`) or add a property that a read
 * then finds on a prototype (`Object.prototype.ltr = …`). Either change can
 * change the reference's sheet, and the addon cannot see it from Rust.
 *
 * So, on the first call, the guard compares every built-in the reference can
 * reach with the same built-in of a fresh realm (a new `vm` context), and
 * records what it found. On each later call it compares the built-ins with
 * that record. When either comparison finds a difference, the call goes to
 * the reference.
 *
 * The guard reads only through property descriptors and through the fresh
 * realm's `Reflect`, which no program can reach, so the guard itself runs no
 * code of the program. Once the guard holds, this realm's `Reflect` is intact
 * too, and the gate uses it: a call into the fresh realm costs much more.
 *
 * The first call costs about 10 ms, and the fresh realm stays alive for the
 * life of the process. A runtime with no working `node:vm` gets no record,
 * and each call then goes to the reference.
 */

import { types } from 'node:util';
import * as vm from 'node:vm';

/**
 * The globals that the reference reads on a call that writes a sheet, and
 * `Reflect`, which the gate reads. The sync test checks that the vendored
 * copy reads no other global, apart from {@link UNGUARDED_GLOBALS}.
 */
export const GUARDED_GLOBALS: readonly string[] = [
  'Array',
  'Map',
  'Math',
  'Object',
  'Reflect',
  'Set',
  'String',
];

/**
 * The globals that the reference reads and that need no guard. `undefined`
 * cannot change, and the reference makes an `Error` only to throw it: each
 * input on which it throws goes to the reference anyway.
 */
export const UNGUARDED_GLOBALS: readonly string[] = ['Error', 'undefined'];

/**
 * The globals whose prototypes the reference calls methods on, through a
 * primitive: `(5).toString()` and `true.toString()`.
 */
const PRIMITIVE_GLOBALS: readonly string[] = ['Number', 'Boolean'];

type Realm = typeof globalThis;

/** One own property of a built-in, as the first call found it. */
interface RecordedProperty {
  readonly holder: object;
  readonly key: PropertyKey;
  readonly descriptor: PropertyDescriptor;
}

/**
 * One built-in object, how many own keys it had on the first call, and its
 * prototype then. A read that misses an own key walks up to the prototype, so
 * a program that swaps it can run its own code inside the reference.
 */
interface RecordedHolder {
  readonly holder: object;
  readonly keyCount: number;
  readonly prototype: object | null;
}

interface BuiltInRecord {
  readonly holders: readonly RecordedHolder[];
  readonly properties: readonly RecordedProperty[];
}

/**
 * Captured when the module loads, because a program can replace it later.
 * Node gives no other way to tell a Proxy from its target.
 */
const isProxy = types.isProxy;

let cachedRealm: Realm | undefined;

/** The record, or `null` when the built-ins differed from a fresh realm's. */
let record: BuiltInRecord | null | undefined;

/**
 * A realm that only this module can reach, so its built-ins are always the
 * ones the specification describes.
 */
export function freshRealm(): Realm {
  cachedRealm ??= vm.runInNewContext('globalThis') as Realm;
  return cachedRealm;
}

export function freshReflect(): typeof Reflect {
  return freshRealm().Reflect;
}

/**
 * The value of an own data property, read without running code: `undefined`
 * for an accessor, a missing property, or a Proxy.
 */
function dataValue(holder: unknown, key: PropertyKey): unknown {
  if ((typeof holder !== 'object' && typeof holder !== 'function') || holder === null) {
    return undefined;
  }
  if (isProxy(holder)) {
    return undefined;
  }

  const descriptor = freshReflect().getOwnPropertyDescriptor(holder, key);
  return descriptor !== undefined && 'value' in descriptor ? descriptor.value : undefined;
}

/** The source text of a function, as the fresh realm prints it. */
function sourceOf(fn: unknown): string | undefined {
  const realm = freshRealm();
  // The fresh realm's `call` and `toString`, which no program can replace.
  return typeof fn === 'function' ? realm.Function.prototype.toString.call(fn) : undefined;
}

/**
 * Whether two properties are the same built-in: both data or both accessors,
 * and each function the same native function by its source text. A program
 * cannot give its own function the source text of a native one. A data value
 * that is not a function is not compared: such values (`Math.PI`,
 * `Symbol.unscopables`) are not called.
 */
function sameBuiltIn(own: PropertyDescriptor, other: PropertyDescriptor): boolean {
  if ('value' in own !== 'value' in other) {
    return false;
  }

  return (['value', 'get', 'set'] as const).every(
    field => sourceOf(own[field]) === sourceOf(other[field])
  );
}

/** Collects the record, and stops at the first difference. */
class Recorder {
  readonly holders: RecordedHolder[] = [];
  readonly properties: RecordedProperty[] = [];

  /** Records the named own properties of `holder`, compared with `other`'s. */
  compareProperties(holder: object, other: object, keys: readonly PropertyKey[]): boolean {
    const reflect = freshReflect();

    for (const key of keys) {
      const own = reflect.getOwnPropertyDescriptor(holder, key);
      const theirs = reflect.getOwnPropertyDescriptor(other, key);
      if (own === undefined || theirs === undefined || !sameBuiltIn(own, theirs)) {
        return false;
      }
      this.properties.push({ holder, key, descriptor: own });
    }

    return true;
  }

  /** The holders of the fresh realm, each at the index of its twin in `holders`. */
  readonly others: object[] = [];

  /** Records every own property of `holder`, its key count and its prototype. */
  compareHolder(holder: unknown, other: unknown): boolean {
    if (typeof holder !== 'object' && typeof holder !== 'function') {
      return false;
    }
    if (holder === null || other === null || typeof other !== typeof holder || isProxy(holder)) {
      return false;
    }

    const reflect = freshReflect();
    const keys = reflect.ownKeys(holder);
    if (keys.length !== reflect.ownKeys(other as object).length) {
      return false;
    }

    this.holders.push({ holder, keyCount: keys.length, prototype: reflect.getPrototypeOf(holder) });
    this.others.push(other as object);
    return this.compareProperties(holder, other as object, keys);
  }

  /**
   * Whether the prototype of each recorded holder is the twin of the
   * prototype that the same holder has in the fresh realm. `functionPrototype`
   * is this realm's `Function.prototype`, which is no recorded holder.
   */
  comparePrototypes(functionPrototype: object): boolean {
    const reflect = freshReflect();
    const twinOf = new Map<unknown, unknown>();
    twinOf.set(freshRealm().Function.prototype, functionPrototype);
    // Indexed loops: an early exit from a `for...of` reads `return` through
    // the very prototypes this checks.
    for (let index = 0; index < this.others.length; index++) {
      twinOf.set(this.others[index], (this.holders[index] as RecordedHolder).holder);
    }

    for (let index = 0; index < this.others.length; index++) {
      const theirs = reflect.getPrototypeOf(this.others[index] as object);
      const ours = (this.holders[index] as RecordedHolder).prototype;
      if (theirs === null ? ours !== null : twinOf.get(theirs) !== ours) {
        return false;
      }
    }

    return true;
  }
}

/**
 * The objects whose properties the reference can reach on a call that writes
 * a sheet, in this realm or in the fresh one: the guarded globals and their
 * prototypes, the prototypes of the primitives it calls methods on, and the
 * prototypes of the iterators its loops and spreads use. A read that misses
 * an own property walks up to these too. The iterator prototypes are read by
 * calling `values` and `entries`, which {@link recordBuiltIns} has compared
 * before it asks for them.
 */
function holdersOf(realm: unknown): unknown[] {
  const global = (name: string) => dataValue(realm, name);
  const prototypeOf = (name: string) => dataValue(global(name), 'prototype');

  return [
    global('Array'),
    prototypeOf('Array'),
    global('Map'),
    prototypeOf('Map'),
    global('Set'),
    prototypeOf('Set'),
    global('Math'),
    global('Object'),
    prototypeOf('Object'),
    global('Reflect'),
    global('String'),
    prototypeOf('String'),
    prototypeOf('Number'),
    prototypeOf('Boolean'),
    // A regular expression literal takes its prototype from the realm, not
    // from the `RegExp` global, which a program can replace.
    realm === globalThis ? freshReflect().getPrototypeOf(/x/) : prototypeOf('RegExp'),
  ];
}

/** `%ArrayIteratorPrototype%`, `%MapIteratorPrototype%` and `%IteratorPrototype%`. */
function iteratorPrototypesOf(realm: unknown): unknown[] {
  const reflect = freshReflect();
  const arrayPrototype = dataValue(dataValue(realm, 'Array'), 'prototype');
  const map = dataValue(realm, 'Map');
  const values = dataValue(arrayPrototype, 'values');
  const entries = dataValue(dataValue(map, 'prototype'), 'entries');

  if (typeof values !== 'function' || typeof entries !== 'function' || typeof map !== 'function') {
    return [];
  }

  const arrayIterator = reflect.getPrototypeOf(reflect.apply(values, [], []) as object);
  const mapIterator = reflect.getPrototypeOf(
    reflect.apply(entries, reflect.construct(map, []), []) as object
  );

  return [arrayIterator, mapIterator, arrayIterator && reflect.getPrototypeOf(arrayIterator)];
}

/**
 * Whether each native function of this realm sits where the same function
 * sits in the fresh realm. Source text alone cannot tell two native
 * functions of one name apart (`String.prototype.includes` and
 * `Array.prototype.includes`), so this walks every built-in object both
 * realms reach from the standard globals, in the same order, and numbers each
 * function where the walk first meets it. A native function that a program
 * moved to another place has another number than its twin.
 *
 * A subtree that the program replaced (a global it polyfilled) is skipped:
 * that is no move of a native function, and the guarded holders are compared
 * one by one anyway.
 */
function nativeFunctionsSitInPlace(): boolean {
  const realm = freshRealm();
  const reflect = realm.Reflect;
  const ourPlaces = new Map<unknown, number>();
  const theirPlaces = new Map<unknown, number>();
  const visited = new Set<unknown>();
  const queue: Array<[unknown, unknown]> = [];
  let place = 0;

  const isObject = (value: unknown): value is object =>
    (typeof value === 'object' && value !== null) || typeof value === 'function';

  for (const name of reflect.ownKeys(realm)) {
    if (typeof name === 'string') {
      queue.push([dataValue(globalThis, name), dataValue(realm, name)]);
    }
  }

  for (let next = queue.shift(); next !== undefined; next = queue.shift()) {
    const [ours, theirs] = next;
    if (!isObject(theirs) || visited.has(theirs) || !isObject(ours) || isProxy(ours)) {
      continue;
    }
    visited.add(theirs);
    queue.push([reflect.getPrototypeOf(ours), reflect.getPrototypeOf(theirs)]);

    for (const key of reflect.ownKeys(theirs)) {
      const own = reflect.getOwnPropertyDescriptor(ours, key);
      const other = reflect.getOwnPropertyDescriptor(theirs, key);
      if (own === undefined || other === undefined) {
        continue;
      }

      for (const field of ['value', 'get', 'set'] as const) {
        const ourValue: unknown = own[field];
        const theirValue: unknown = other[field];
        if (typeof ourValue === 'function' && typeof theirValue === 'function') {
          if (!ourPlaces.has(ourValue)) ourPlaces.set(ourValue, place);
          if (!theirPlaces.has(theirValue)) theirPlaces.set(theirValue, place);
          place++;
          if (
            sourceOf(ourValue) === sourceOf(theirValue) &&
            ourPlaces.get(ourValue) !== theirPlaces.get(theirValue)
          ) {
            return false;
          }
        }
        queue.push([ourValue, theirValue]);
      }
    }
  }

  return true;
}

/**
 * The record of this realm's built-ins, or `null` when one of them differs
 * from the fresh realm's. Of the global object, only the globals the
 * reference reads are recorded, because Node and each program add globals
 * of their own.
 */
function recordBuiltIns(): BuiltInRecord | null {
  const realm = freshRealm();
  const recorder = new Recorder();

  if (!recorder.compareProperties(globalThis, realm, [...GUARDED_GLOBALS, ...PRIMITIVE_GLOBALS])) {
    return null;
  }

  const ours = holdersOf(globalThis);
  const theirs = holdersOf(realm);
  if (!ours.every((holder, index) => recorder.compareHolder(holder, theirs[index]))) {
    return null;
  }

  // Only now are `values` and `entries` known to be the native ones.
  const ourIterators = iteratorPrototypesOf(globalThis);
  const theirIterators = iteratorPrototypesOf(realm);
  if (
    ourIterators.length === 0 ||
    !ourIterators.every((holder, index) => recorder.compareHolder(holder, theirIterators[index])) ||
    // A function of this module has this realm's own `Function.prototype`.
    !recorder.comparePrototypes(freshReflect().getPrototypeOf(recordBuiltIns) as object) ||
    !nativeFunctionsSitInPlace()
  ) {
    return null;
  }

  return { holders: recorder.holders, properties: recorder.properties };
}

/**
 * Whether every built-in the reference can reach is still the one a fresh
 * realm holds. The first call compares this realm with a fresh one; each
 * later call compares it with what the first call recorded.
 */
export function builtInsAreIntact(): boolean {
  try {
    record ??= recordBuiltIns();
  } catch {
    // No fresh realm: `node:vm` is missing or does not work in this runtime.
    record = null;
  }
  if (record === null) {
    return false;
  }

  const reflect = freshReflect();
  const { holders, properties } = record;

  // Indexed loops: an early exit from a `for...of` reads `return` through an
  // iterator prototype, which a program may have changed.
  for (let index = 0; index < holders.length; index++) {
    const { holder, keyCount, prototype } = holders[index]!;
    if (
      reflect.ownKeys(holder).length !== keyCount ||
      reflect.getPrototypeOf(holder) !== prototype
    ) {
      return false;
    }
  }

  for (let index = 0; index < properties.length; index++) {
    const { holder, key, descriptor } = properties[index]!;
    const now = reflect.getOwnPropertyDescriptor(holder, key);
    if (
      now === undefined ||
      now.value !== descriptor.value ||
      now.get !== descriptor.get ||
      now.set !== descriptor.set
    ) {
      return false;
    }
  }

  return true;
}

export { isProxy };
