/**
 * The gate in front of the native `processStylexRules`.
 *
 * The native sheet must be the reference's sheet, byte for byte. That holds
 * when every read the reference makes of the arguments is a plain read of a
 * plain value: no getter, no Proxy trap, no `toString` of an object, no
 * string that JavaScript holds and Rust cannot (a lone surrogate). The gate
 * accepts the arguments only in that case, and only in the shapes a build
 * emits. It reads them through property descriptors, so it runs no code of the
 * program, and packs what the reference reads into the few values the addon
 * takes. For any other argument it answers `null`, and the caller runs the
 * reference, which writes the genuine sheet or throws the genuine error.
 *
 * The gate runs only once `builtInsAreIntact` holds. Then a read that misses
 * an own property finds nothing on a prototype either, and the gate can call
 * the guarded built-ins of this realm as they are (`Reflect`, `Array`,
 * `String` and their prototypes). It reads no other global.
 */

import { types } from 'node:util';

import nativeBinding from '../../dist/transform';
import type { NativeConfig } from '../../dist/transform';
import { freshRealm, isProxy } from './built-ins.js';
import type { NativeCollation } from './collation.js';

/**
 * Captured when the module loads, like `isProxy`. A module namespace object
 * throws on a read of a binding that is not yet set, even through a
 * descriptor.
 */
const isModuleNamespaceObject = types.isModuleNamespaceObject;

/** What a packed rule is, one byte per rule, as `process_stylex_rules.rs` defines it. */
export const RuleKind = {
  /** A style rule with an LTR text only. */
  Style: nativeBinding.RULE_KIND_STYLE,
  /** A style rule with an LTR and an RTL text. */
  StyleWithRtl: nativeBinding.RULE_KIND_STYLE_WITH_RTL,
  /** A `defineConsts` rule: its class name, `String(constKey)`, `String(constVal)`. */
  Constant: nativeBinding.RULE_KIND_CONSTANT,
};

/**
 * The rules, packed. Each rule gives its kind, its priority and two or three
 * texts: its class name, then its LTR and RTL texts or its constant's key
 * and value. The texts are joined into one string, and `lengths` holds the
 * length of each in UTF-16 units, so that the addon reads one string and not
 * one per text.
 */
export interface NativeInput {
  texts: string;
  lengths: Uint32Array;
  kinds: Uint8Array;
  priorities: Float64Array;
  config: NativeConfig;
}

/** A read the gate cannot prove is plain. */
const REFUSED = Symbol('refused');

type Read = unknown;

/** Printable ASCII, the text the native sort compares on any ICU. */
const PRINTABLE_ASCII = /^[ -~]*$/;

/**
 * An own property as a plain read: its value, `undefined` when it is
 * missing, or {@link REFUSED} for a getter. A missing property reads
 * `undefined` because the prototypes hold no such property while the
 * built-ins are intact.
 */
function readOwn(object: object, key: PropertyKey): Read {
  const descriptor = Reflect.getOwnPropertyDescriptor(object, key);
  if (descriptor === undefined) {
    return undefined;
  }
  return 'value' in descriptor ? descriptor.value : REFUSED;
}

let iterator: symbol | undefined;

/**
 * `Symbol.iterator`, read once from the fresh realm: well-known symbols are
 * shared by all realms, and the `Symbol` global is the program's to replace.
 */
function iteratorSymbol(): symbol {
  iterator ??= freshRealm().Symbol.iterator;
  return iterator;
}

/** A non-Proxy object whose prototype is `Object.prototype` or `null`. */
function isPlainObject(value: unknown): value is object {
  if (
    typeof value !== 'object' ||
    value === null ||
    isProxy(value) ||
    isModuleNamespaceObject(value)
  ) {
    return false;
  }
  const prototype = Reflect.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}

/**
 * A non-Proxy array of this realm. The Proxy test comes first: `Array.isArray`
 * throws on a revoked Proxy.
 */
function isPlainArray(value: unknown): value is unknown[] {
  return (
    !isProxy(value) && Array.isArray(value) && Reflect.getPrototypeOf(value) === Array.prototype
  );
}

/**
 * The elements of a small plain array with no hole and no other own
 * property, or `null`. The own keys of a dense array are its indices, in
 * order, then `length`.
 */
function denseElements(value: unknown): unknown[] | null {
  if (!isPlainArray(value)) {
    return null;
  }

  const keys = Reflect.ownKeys(value);
  const length = value.length;
  if (keys.length !== length + 1 || keys[length] !== 'length') {
    return null;
  }

  const elements: unknown[] = [];
  for (let index = 0; index < length; index++) {
    const descriptor = Reflect.getOwnPropertyDescriptor(value, index);
    if (descriptor === undefined || !('value' in descriptor)) {
      return null;
    }
    elements.push(descriptor.value);
  }
  return elements;
}

/** A string that Rust can hold as it is: no lone surrogate. */
function isWellFormedString(value: unknown): value is string {
  return typeof value === 'string' && value.isWellFormed();
}

/** `String(value)` of a primitive the gate accepts, or {@link REFUSED}. */
function spellingOf(value: unknown): string | typeof REFUSED {
  if (isWellFormedString(value)) {
    return value;
  }
  return typeof value === 'number' || typeof value === 'boolean' ? String(value) : REFUSED;
}

/**
 * `[...(names ?? [])]`, joined as `join` joins: `null` and `undefined` as
 * nothing. A string would spread by its characters; the gate leaves that to
 * the reference.
 */
function layerNamesOf(value: Read): string[] | null {
  if (value == null) {
    return [];
  }

  const elements = denseElements(value);
  if (elements === null) {
    return null;
  }

  const names: string[] = [];
  for (const element of elements) {
    const name = element == null ? '' : spellingOf(element);
    if (name === REFUSED) {
      return null;
    }
    names.push(name);
  }
  return names;
}

/** The four options the reference reads, as it reads them, or `null`. */
function configOf(config: unknown): NativeConfig | null {
  const defaults: NativeConfig = {
    useLayers: false,
    layersBefore: [],
    layersAfter: [],
    layerPrefix: '',
    enableLTRRTLComments: false,
    legacyDisableLayers: false,
    useLegacyClassnamesSort: false,
  };

  // `typeof config === 'boolean' ? { useLayers: config } : config ?? {}`.
  // Every other primitive but a symbol and a big integer reads as `{}`.
  if (typeof config === 'boolean') {
    return { ...defaults, useLayers: config };
  }
  if (config == null || typeof config === 'string' || typeof config === 'number') {
    return defaults;
  }
  if (!isPlainObject(config)) {
    return null;
  }

  const rawUseLayers = readOwn(config, 'useLayers');
  const enableLTRRTLComments = readOwn(config, 'enableLTRRTLComments');
  const legacyDisableLayers = readOwn(config, 'legacyDisableLayers');
  const useLegacyClassnamesSort = readOwn(config, 'useLegacyClassnamesSort');
  if (
    rawUseLayers === REFUSED ||
    enableLTRRTLComments === REFUSED ||
    legacyDisableLayers === REFUSED ||
    useLegacyClassnamesSort === REFUSED
  ) {
    return null;
  }

  // The three flags are read by truthiness, which runs no code. `!!` is used
  // and not `Boolean`, a global that the guard does not cover.
  const flags = {
    enableLTRRTLComments: !!enableLTRRTLComments,
    legacyDisableLayers: !!legacyDisableLayers,
    useLegacyClassnamesSort: !!useLegacyClassnamesSort,
  };
  const useLayers = rawUseLayers ?? false;

  if (typeof useLayers !== 'object') {
    return { ...defaults, ...flags, useLayers: useLayers !== false };
  }
  if (!isPlainObject(useLayers)) {
    return null;
  }

  const layersBefore = layerNamesOf(readOwn(useLayers, 'before'));
  const layersAfter = layerNamesOf(readOwn(useLayers, 'after'));
  // The reference tests the prefix by truthiness before it spells it, so a
  // falsy prefix is no prefix. A getter is truthy, and is refused.
  const prefix = readOwn(useLayers, 'prefix') ?? '';
  const layerPrefix = prefix ? spellingOf(prefix) : '';
  if (layersBefore === null || layersAfter === null || layerPrefix === REFUSED) {
    return null;
  }

  return { ...flags, useLayers: true, layersBefore, layersAfter, layerPrefix };
}

/** Packs the rules into one string and three typed arrays. */
class Packer {
  readonly texts: string[] = [];
  readonly lengths: Uint32Array;
  readonly kinds: Uint8Array;
  readonly priorities: Float64Array;
  private textCount = 0;

  constructor(ruleCount: number) {
    // The fresh realm's constructors, because a program can replace the
    // global ones.
    const realm = freshRealm();
    this.lengths = new realm.Uint32Array(ruleCount * 3);
    this.kinds = new realm.Uint8Array(ruleCount);
    this.priorities = new realm.Float64Array(ruleCount);
  }

  text(text: string): void {
    this.texts.push(text);
    this.lengths[this.textCount++] = text.length;
  }

  input(config: NativeConfig): NativeInput {
    return {
      texts: this.texts.join(''),
      lengths: this.lengths.subarray(0, this.textCount),
      kinds: this.kinds,
      priorities: this.priorities,
      config,
    };
  }
}

/**
 * Packs one rule, `[className, style, priority]`, or answers `false`. The
 * reference reads the tuple with an array iterator, so the tuple must be a
 * plain array that holds its three items itself, and reads the style with a
 * spread, so every own property of the style must be an enumerable value.
 */
function packRule(
  rule: unknown,
  index: number,
  packer: Packer,
  collation: NativeCollation,
  legacySort: boolean
): boolean {
  if (!isPlainArray(rule) || rule.length < 3) {
    return false;
  }

  // An own iterator would replace the array iterator.
  if (Reflect.getOwnPropertyDescriptor(rule, iteratorSymbol()) !== undefined) {
    return false;
  }

  const className = readOwn(rule, 0);
  const style = readOwn(rule, 1);
  const priority = readOwn(rule, 2);
  if (!isWellFormedString(className) || typeof priority !== 'number' || !isPlainObject(style)) {
    return false;
  }
  if (legacySort && collation === 'printable-ascii' && !PRINTABLE_ASCII.test(className)) {
    return false;
  }

  let ltr: unknown;
  let rtl: unknown;
  let constKey: unknown;
  let constVal: unknown;
  for (const key of Reflect.ownKeys(style)) {
    const descriptor = Reflect.getOwnPropertyDescriptor(style, key);
    if (descriptor === undefined || !('value' in descriptor) || descriptor.enumerable !== true) {
      return false;
    }
    if (key === 'ltr') ltr = descriptor.value;
    else if (key === 'rtl') rtl = descriptor.value;
    else if (key === 'constKey') constKey = descriptor.value;
    else if (key === 'constVal') constVal = descriptor.value;
  }

  packer.priorities[index] = priority;

  // `ruleObj?.constKey != null && ruleObj?.constVal != null`.
  if (constKey != null && constVal != null) {
    const key = spellingOf(constKey);
    const value = spellingOf(constVal);
    if (key === REFUSED || value === REFUSED) {
      return false;
    }
    if (collation === 'printable-ascii' && !PRINTABLE_ASCII.test(value)) {
      return false;
    }
    packer.kinds[index] = RuleKind.Constant;
    packer.text(className);
    packer.text(key);
    packer.text(value);
    return true;
  }

  // The sort compares the LTR texts and, under the legacy sort, the class
  // names. It never compares an RTL text.
  if (!isWellFormedString(ltr) || (collation === 'printable-ascii' && !PRINTABLE_ASCII.test(ltr))) {
    return false;
  }
  if (rtl != null && !isWellFormedString(rtl)) {
    return false;
  }

  packer.text(className);
  packer.text(ltr);
  // An empty RTL text is no RTL text: the reference tests it by truthiness.
  if (typeof rtl === 'string' && rtl !== '') {
    packer.kinds[index] = RuleKind.StyleWithRtl;
    packer.text(rtl);
  } else {
    packer.kinds[index] = RuleKind.Style;
  }
  return true;
}

/**
 * The arguments packed for the addon, or `null` when the reference must run.
 *
 * `rules` must be a plain array with no hole and no own `filter` or
 * `constructor`: the reference calls `rules.filter`, which reads
 * `rules.constructor` to make its result.
 */
export function nativeInputOf(
  rules: unknown,
  config: unknown,
  collation: NativeCollation
): NativeInput | null {
  if (collation === 'none' || !isPlainArray(rules)) {
    return null;
  }

  const nativeConfig = configOf(config);
  if (nativeConfig === null) {
    return null;
  }

  if (
    Reflect.getOwnPropertyDescriptor(rules, 'filter') !== undefined ||
    Reflect.getOwnPropertyDescriptor(rules, 'constructor') !== undefined
  ) {
    return null;
  }

  const ruleCount = rules.length;
  // A sparse array can have a length far above the rules it holds. The first
  // hole refuses it below, but the packer is sized first, and where the
  // system cannot give that much memory the allocation throws.
  let packer: Packer;
  try {
    packer = new Packer(ruleCount);
  } catch {
    return null;
  }

  for (let index = 0; index < ruleCount; index++) {
    const descriptor = Reflect.getOwnPropertyDescriptor(rules, index);
    if (descriptor === undefined || !('value' in descriptor)) {
      return null;
    }
    if (
      !packRule(descriptor.value, index, packer, collation, nativeConfig.useLegacyClassnamesSort)
    ) {
      return null;
    }
  }

  return packer.input(nativeConfig);
}
