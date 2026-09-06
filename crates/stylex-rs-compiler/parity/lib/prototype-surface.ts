/**
 * The subjects the prototype sweep asks about: every method the engine can
 * reach, crossed with both receiver shapes.
 *
 * The curated corpus carries one row per method someone measured, and that is
 * where a *reason* gets written down. What it cannot be is evidence of
 * coverage: a curated list of methods is itself a table, and the argument for
 * evaluating a call rather than matching its name was that the method nobody
 * listed is the next bug report. So the surface is read off the language at run
 * time — `Object.getOwnPropertyNames` over the prototypes and the namespaces
 * the fold reaches — and nothing here enumerates a method name.
 *
 * Two things a generated subject needs and a curated one is written with: an
 * argument list, and a value a declaration can carry. Both are *measured* here
 * rather than tabulated, by evaluating the expression in this process before
 * either compiler sees it:
 *
 * - **Arguments.** A small pool of argument texts is crossed up to arity two
 *   and the first vector real JavaScript accepts is the one both compilers are
 *   handed. So the sweep needs no table of which method takes what, and a
 *   method no vector satisfies is *reported as unexercised* rather than
 *   silently dropped — the count is the honest half of the coverage claim.
 * - **A terminal.** A method answering an array, an object or a boolean has no
 *   declaration form, so one further call renders it: `join`, `Object.keys`,
 *   `String`. What that costs is stated where `renderingFor` chooses it.
 *
 * **What it does not ask.** One variable moves per row, so a prototype method's
 * *arguments* are written out in both of its shapes: the receiver is the
 * question, and a row varying two things at once could not say which one
 * refused it. The consequence is that a callback — which is an argument —
 * reaches the named shape only through a namespace method, since that is where
 * the shape moves to the arguments. `Object.groupBy` is the one such row today,
 * and it is how the gap in issue 19 was found; on the array methods where
 * callbacks actually live, this sweep asks only the written-out form.
 *
 * Evaluating here also decides two exclusions that would otherwise be a list
 * of names. A method whose answer differs between two evaluations —
 * `Math.random` — cannot be compared across two compilers at all, and a method
 * answering a value no declaration carries would be asked as a subject both
 * compilers drop, which reads as agreement about nothing. Neither is written
 * down as a name: both fall out of the measurement.
 */

import vm from 'node:vm';

import type { LoadedCorpusEntry } from './types.js';

/** The set every generated subject is attributed to. */
export const SWEEP_SET = 'prototype-sweep';

/**
 * A subject as the sweep builds it: always a module, never a declaration.
 *
 * Narrowed to the one arm rather than left as the whole corpus union, because
 * every subject here carries a `const` above a `create` call — a declaration
 * entry cannot express that — and a reader of a failing row wants the source.
 */
export type ModuleSubject = Extract<LoadedCorpusEntry, { kind: 'module' }>;

/** Which half of the receiver question a subject asks. */
export type Shape = 'named' | 'written';

/** Both, in the order the report prints them. */
export const SHAPES = ['written', 'named'] as const satisfies readonly Shape[];

/**
 * One surface the fold reaches, and how a call on it is written.
 *
 * A prototype surface varies its *receiver* between the two shapes, which is
 * the question this whole effort turned on: giving a value a name must not
 * change whether the call on it folds. A namespace surface has no receiver to
 * vary — `Math` is the receiver, and it is an identifier either way — so there
 * the shape moves to the arguments, which is the same question read one step
 * along: `Math.trunc(ratio)` against `Math.trunc(1.5)`.
 */
type Reach =
  | {
      readonly kind: 'prototype';
      /** How the report names the surface. */
      readonly name: string;
      /** The object whose own property names are crossed. */
      readonly target: object;
      /** The receiver written out, parenthesised where the grammar needs it. */
      readonly receiver: string;
      /** The name the receiver takes in the named shape. */
      readonly binding: string;
    }
  | {
      readonly kind: 'namespace';
      /** The identifier the call is made on, which is also the report's name. */
      readonly name: string;
      readonly target: object;
    };

/**
 * A surface, with the coverage it is on record as reaching.
 *
 * The allowance is written beside the surface rather than in a table of its
 * own, because the two are one statement: this surface, and how much of it the
 * sweep is on record as *not* reaching.
 */
export type Surface = Reach & {
  /**
   * How many of this surface's own methods may answer nothing usable.
   *
   * Recorded from a run rather than derived, and that is the point of it. The
   * count of methods is read off the language, but how many of them the sweep
   * can actually *ask* depends on this file — the argument pool, the arities
   * tried, what `renderingFor` accepts — so a change here that stopped half a
   * prototype from answering would leave the sweep reporting agreement about
   * the half that was left. The allowance is the number a reader agreed to, and
   * moving it is an edit somebody has to make on purpose.
   *
   * What is recorded is the *shortfall* rather than the reached count, because
   * the reached count is not a property of this file alone: `Math.f16round`
   * exists on Node 24 and not on Node 22, so a reached count recorded on one
   * engine fails on the other while nothing about the sweep has changed. A
   * method the engine does not carry is neither asked nor missed, so the
   * allowance holds across engines and the floor moves with the language. See
   * `floorFor`.
   *
   * Lowering one is ordinary: a wider pool reaches more methods, so fewer are
   * left over. Raising one is the claim that needs an argument in the commit
   * that does it.
   */
  readonly unanswered: number;
};

/**
 * Every surface, with the receiver each prototype is asked about.
 *
 * The receiver values are chosen to answer something for as much of their
 * prototype as possible rather than to be interesting: a string with mixed
 * case, a space and a repeat so that case, trimming, splitting and searching
 * all have something to find; an array out of order so that a sort is
 * observable; an object with two string values so that keys, values and entries
 * differ from each other.
 *
 * `Number.prototype` is asked on a receiver written into the source as well,
 * even though both compilers refuse that — the refusal is the subject there,
 * and dropping the shape would leave it unmeasured. `Object.prototype`'s
 * receiver is parenthesised for the same reason a number's is: an object
 * literal at the head of an expression is a block.
 *
 * Which surfaces belong here is not a choice. The namespaces are exactly the
 * compiler's `VALID_CALLEES`, so a name added there and not here would go
 * unswept; the prototypes are the ones a value crossing the fold's bridge can
 * have, which is why `Boolean.prototype` is among them — a boolean is a value
 * the bridge carries, even though `Boolean` is not a callee — and why `Date`,
 * `RegExp` and `Function` are not, since no value the bridge carries has one.
 */
export const SURFACES: readonly Surface[] = [
  {
    kind: 'prototype',
    name: 'String.prototype',
    target: String.prototype,
    receiver: "'AbC dEf'",
    binding: 'text',
    unanswered: 1,
  },
  {
    kind: 'prototype',
    name: 'Array.prototype',
    target: Array.prototype,
    receiver: "['b', 'a']",
    binding: 'list',
    unanswered: 4,
  },
  {
    kind: 'prototype',
    name: 'Object.prototype',
    target: Object.prototype,
    receiver: "({ b: '1', a: '2' })",
    binding: 'config',
    unanswered: 4,
  },
  {
    kind: 'prototype',
    name: 'Number.prototype',
    target: Number.prototype,
    receiver: '(255)',
    binding: 'count',
    unanswered: 0,
  },
  {
    kind: 'prototype',
    name: 'Boolean.prototype',
    target: Boolean.prototype,
    receiver: '(true)',
    binding: 'enabled',
    unanswered: 0,
  },
  { kind: 'namespace', name: 'Math', target: Math, unanswered: 1 },
  { kind: 'namespace', name: 'Object', target: Object, unanswered: 5 },
  { kind: 'namespace', name: 'Number', target: Number, unanswered: 0 },
  { kind: 'namespace', name: 'String', target: String, unanswered: 1 },
  { kind: 'namespace', name: 'Array', target: Array, unanswered: 1 },
];

/**
 * The argument texts a call is tried with, in the order they are tried.
 *
 * Ordered by how likely each is to be what a method wants, since the first
 * vector JavaScript accepts is the one that runs: a string, then a separator,
 * then the small numbers a length or an index is spelled with, then the
 * compound values a few methods need. Every one is small on purpose — the
 * fold's allocation ceilings are the curated corpus's subject, and a sweep that
 * tripped them would be measuring a ceiling rather than a surface.
 */
const ARGUMENTS = [
  "'a'",
  "', '",
  "'AbC'",
  '1',
  '0',
  '2',
  '16',
  '-1',
  '1.5',
  "['x']",
  "{ a: '1' }",
  '(x) => x',
  '(a, b) => a + b',
] as const;

/**
 * The most arguments a call is tried with.
 *
 * Two rather than the widest arity the surface declares. The pool crossed with
 * itself is quadratic, so a third position multiplies the search by thirteen for
 * the three methods whose arity is greater — and each of those wants a shape the
 * pool has no member of anyway (a property descriptor, a target and a source).
 * They come back as unexercised, which is the report saying so rather than the
 * sweep quietly claiming them.
 */
const MOST_ARGUMENTS = 2;

/** The property a string-valued fold is written into. */
const STRING_PROPERTY = 'content';

/**
 * The property a number-valued fold is written into.
 *
 * `zIndex` rather than a length: it takes a bare number, so nothing about the
 * subject depends on a unit being appended, and a fractional or negative answer
 * still reaches the stylesheet.
 */
const NUMBER_PROPERTY = 'zIndex';

/** Why a method could not be asked, in the order a reason outranks another. */
export type Rejection =
  /** Every argument vector threw, so the sweep never reached a value. */
  | 'threw'
  /** It answered something no declaration carries. */
  | 'unusable'
  /** It answered differently on two evaluations, so no two runs can agree. */
  | 'nondeterministic';

const REJECTION_RANK = {
  threw: 0,
  unusable: 1,
  nondeterministic: 2,
} as const satisfies Record<Rejection, number>;

/** A method the sweep could not ask, and what stopped it. */
export interface Unexercised {
  readonly surface: string;
  readonly method: string;
  readonly rejection: Rejection;
  /** What the strongest rejection actually saw, for the report to print. */
  readonly detail: string;
}

/**
 * One question asked of both compilers: a method, a shape, and the module that
 * puts them together.
 *
 * The three travel together rather than being recovered from the subject's id.
 * A failing row has to name the method and the receiver shape, and an
 * expectation is written against the same two, so parsing them back out of a
 * generated identifier would put the sweep's own naming scheme between the
 * report and what it is reporting.
 */
export interface Asked {
  readonly surface: string;
  readonly method: string;
  readonly shape: Shape;
  /** The expression under test, as this shape spells it. */
  readonly expression: string;
  /** What JavaScript itself answers for it, as text. */
  readonly value: string;
  readonly subject: ModuleSubject;
}

/** A method the sweep asked, and the subjects it asked it with. */
export interface Exercised {
  readonly surface: string;
  readonly method: string;
  /** The expression both shapes evaluate, as the written shape spells it. */
  readonly expression: string;
  /** What JavaScript itself answers, for the report to print beside a failure. */
  readonly value: string;
  readonly asked: readonly Asked[];
}

/** What one pass over the surfaces produced. */
export interface Sweep {
  /** The surfaces this pass covered, which is what its counts answer for. */
  readonly surfaces: readonly Surface[];
  readonly exercised: readonly Exercised[];
  readonly unexercised: readonly Unexercised[];
  /** Every question, flattened, in the order the surfaces are declared. */
  readonly asked: readonly Asked[];
}

/** A surface that asked about fewer methods than it is on record as asking. */
export interface Shortfall {
  readonly surface: string;
  readonly exercised: number;
  /** The floor this engine holds the surface to, as `floorFor` derives it. */
  readonly floor: number;
}

/**
 * The fewest methods a surface must exercise on the engine that is running.
 *
 * Derived rather than recorded, from the two halves of the statement: the
 * methods the language carries here, less the ones the sweep is on record as
 * not reaching. So a Node release that adds a method raises the floor with it,
 * and one that does not carry it yet lowers the floor by exactly that method
 * instead of failing a run — while the allowance, which is the part this file
 * is answerable for, stays the number a reader agreed to.
 *
 * Never below one, which is the case a derived floor would otherwise let
 * through: a surface whose `target` stops resolving to the object it names
 * enumerates no methods at all, and a floor of zero would read that as a pass.
 *
 * An allowance that is not a count of methods — negative, fractional, infinite,
 * `NaN` — is read as no allowance at all rather than as licence. A mistyped
 * record then makes the gate stricter, which a run says out loud, instead of
 * looser, which nothing would.
 */
export function floorFor(surface: Surface): number {
  const allowance =
    Number.isInteger(surface.unanswered) && surface.unanswered > 0 ? surface.unanswered : 0;

  return Math.max(1, methodsOf(surface).length - allowance);
}

/**
 * The swept surfaces that fell below their recorded floor, in surface order.
 *
 * The check the coverage numbers are worth reading for. Every other gate in the
 * sweep is about a *row* — a divergence nothing accounts for, an account whose
 * reason is gone — and none of them fires when the sweep simply stops asking:
 * a method that no longer answers produces no row to disagree about, so a
 * regression that halved the coverage would print a smaller number beside a
 * green run. Read per surface rather than over the total, because a total hides
 * exactly the case worth catching — one prototype falling silent while the
 * rest carry the sum.
 *
 * Only the surfaces the pass covered are judged, so `--surface Math` is a
 * narrower run rather than a failing one.
 */
export function shortfalls(swept: Sweep): Shortfall[] {
  const asked = new Map<string, number>();
  for (const one of swept.exercised) {
    asked.set(one.surface, (asked.get(one.surface) ?? 0) + 1);
  }

  return swept.surfaces
    .map(surface => ({
      surface: surface.name,
      exercised: asked.get(surface.name) ?? 0,
      floor: floorFor(surface),
    }))
    .filter(one => one.exercised < one.floor);
}

/**
 * The function-valued own property names of a surface, in the language's order.
 *
 * Read rather than listed, which is the whole point. A getter is read inside a
 * `try` because `Object.prototype` carries one — `__proto__` — and a surface
 * that threw while being enumerated would take the sweep down with it rather
 * than losing one name.
 */
export function methodsOf(surface: Surface): string[] {
  const found: string[] = [];

  for (const name of Object.getOwnPropertyNames(surface.target)) {
    let isFunction = false;
    try {
      isFunction = typeof Reflect.get(surface.target, name) === 'function';
    } catch {
      isFunction = false;
    }
    if (isFunction) found.push(name);
  }

  return found;
}

/**
 * The arity the surface declares, or `undefined` where it cannot be read.
 *
 * Only the search order depends on it, so an unreadable one costs nothing but
 * a few extra candidates.
 */
function declaredArity(surface: Surface, method: string): number {
  try {
    const found: unknown = Reflect.get(surface.target, method);
    return typeof found === 'function' ? found.length : 0;
  } catch {
    return 0;
  }
}

/**
 * The arities tried, in order: the declared one first, then narrower, then
 * wider.
 *
 * The declared arity leads because a call made with fewer arguments than it
 * wants usually still answers something — `'AbC dEf'.replace()` is the string
 * itself — and a subject like that measures the fold on a call nobody writes.
 * Narrower before wider, because a method that declares more arguments than the
 * pool can fill is likelier to accept the ones it has than to want more.
 */
export function arityOrder(declared: number): number[] {
  const start = Math.min(declared, MOST_ARGUMENTS);
  const order: number[] = [];
  for (let arity = start; arity >= 0; arity -= 1) order.push(arity);
  for (let arity = start + 1; arity <= MOST_ARGUMENTS; arity += 1) order.push(arity);

  return order;
}

/**
 * Every argument vector of one arity, in pool order.
 *
 * Built one position at a time, so the first position varies slowest: the first
 * vector of any arity is the first pool member throughout, and the search order
 * a chosen subject came out of can be re-read from the pool.
 */
export function vectorsOfArity(arity: number): string[][] {
  let vectors: string[][] = [[]];

  for (let position = 0; position < arity; position += 1) {
    const wider: string[][] = [];
    for (const shorter of vectors) {
      for (const argument of ARGUMENTS) {
        const vector = shorter.slice();
        vector.push(argument);
        wider.push(vector);
      }
    }
    vectors = wider;
  }

  return vectors;
}

/** How a raw answer is turned into something a declaration can carry. */
interface Rendering {
  /** The expression that renders the raw call. */
  readonly render: (expression: string) => string;
  readonly property: string;
}

/**
 * How a value of this shape reaches a declaration, or `undefined` where it does
 * not reach one at all.
 *
 * Three of the four renderings add a call, and each one costs the same thing:
 * the subject is then about the method *and* the terminal. That is accepted
 * deliberately and it is the trade the curated rows already make —
 * `Object.keys(config).join(', ')` is how the curated static row is written —
 * because the alternative is asking a predicate, a key list or a group with no
 * declaration to put the answer in, which both compilers drop and the harness
 * then reads as agreement about nothing. The terminals are three names the fold
 * has to answer anyway, so a terminal that broke would break its own row too
 * rather than hiding behind these.
 *
 * What is deliberately *not* rendered is a function, a symbol, a promise,
 * `undefined`, `null`, and a number with no decimal spelling. Those are values
 * no CSS declaration has a form for at all, so a subject carrying one would
 * measure the two compilers agreeing to drop a property.
 */
export function renderingFor(value: unknown): Rendering | undefined {
  if (typeof value === 'string') {
    return { render: expression => expression, property: STRING_PROPERTY };
  }
  if (typeof value === 'number') {
    return Number.isFinite(value)
      ? { render: expression => expression, property: NUMBER_PROPERTY }
      : undefined;
  }
  if (typeof value === 'boolean') {
    return { render: expression => `String(${expression})`, property: STRING_PROPERTY };
  }
  if (Array.isArray(value)) {
    return { render: expression => `${expression}.join(', ')`, property: STRING_PROPERTY };
  }
  if (isPlainObject(value)) {
    return {
      render: expression => `Object.keys(${expression}).join(', ')`,
      property: STRING_PROPERTY,
    };
  }

  return undefined;
}

/** Whether a value is an ordinary object, rather than one of the exotic ones. */
function isPlainObject(value: unknown): boolean {
  if (typeof value !== 'object' || value === null) return false;
  const prototype: unknown = Object.getPrototypeOf(value);

  return prototype === Object.prototype || prototype === null;
}

/**
 * What this process answers for one expression, or the throw it produced.
 *
 * The text evaluated is the subject's own — the expression the module puts in a
 * style value — so what is measured here is what the compilers are being asked
 * about, and not a second encoding of it that could drift.
 *
 * `vm.runInThisContext` rather than the `Function` constructor, and rather than
 * a fresh context. It compiles the expression as a script with no enclosing
 * scope at all, so nothing in this file is reachable from it — which the
 * `Function` constructor also gives, but a script says so. A *new* context
 * would say more, and would be wrong: the intrinsics there belong to another
 * realm, and `isPlainObject` asks whether an answer's prototype is this realm's
 * `Object.prototype`, so every object a method returned would come back
 * unrenderable.
 *
 * Only the written shape is evaluated. The named shape differs from it by
 * binding a value to a name, which is exactly the transformation the compilers
 * are being asked about and never one JavaScript answers differently — so
 * evaluating both would measure the same value twice.
 */
function evaluate(expression: string): { ok: true; value: unknown } | { ok: false; threw: string } {
  try {
    // Parenthesised because a script's value is its last expression, and an
    // object literal at the head of a statement is a block.
    const answer: unknown = vm.runInThisContext(`(${expression})`);
    // A method answering a promise is unusable either way, and an *already
    // rejected* one takes the process down on the next tick if nothing reads it
    // -- `Array.fromAsync()` with no argument is one. Settled here rather than
    // where the value is classified, because by then the tick has passed.
    if (answer instanceof Promise) answer.catch(() => undefined);

    return { ok: true, value: answer };
  } catch (error: unknown) {
    return { ok: false, threw: error instanceof Error ? error.message : String(error) };
  }
}

/** What a call rendered to, once the sweep found arguments that answer. */
interface Chosen {
  readonly args: readonly string[];
  /** The call alone, before the terminal that renders its answer. */
  readonly rawCall: string;
  /** The rendered expression, as the written shape spells it. */
  readonly expression: string;
  readonly property: string;
  /** What JavaScript answers, as text. */
  readonly value: string;
}

/** The strongest reason a method could not be asked, as it accumulates. */
interface Refused {
  readonly rejection: Rejection;
  readonly detail: string;
}

/**
 * Arguments this call answers a usable value with, or why none did.
 *
 * The first vector that answers wins, so the search order in `arityOrder` and
 * in `ARGUMENTS` is what decides which subject a method is asked as. A vector
 * has to clear three separate bars — the call must not throw, its answer must
 * reach a declaration, and it must answer the same thing twice — and the
 * strongest bar any vector failed is what the report prints, because "every
 * argument threw" and "it answers a new number every time" send a reader to
 * different places.
 */
export function chooseArguments(surface: Surface, method: string): Chosen | Refused {
  let refused: Refused = { rejection: 'threw', detail: 'no argument list was tried' };

  const worse = (rejection: Rejection, detail: string): void => {
    if (REJECTION_RANK[rejection] >= REJECTION_RANK[refused.rejection]) {
      refused = { rejection, detail };
    }
  };

  for (const arity of arityOrder(declaredArity(surface, method))) {
    for (const args of vectorsOfArity(arity)) {
      const rawCall = callText(surface, method, args);
      const first = evaluate(rawCall);
      if (!first.ok) {
        worse('threw', first.threw);
        continue;
      }

      const rendering = renderingFor(first.value);
      if (rendering === undefined) {
        worse('unusable', `answers ${describe(first.value)}`);
        continue;
      }

      const expression = rendering.render(rawCall);
      const rendered = evaluate(expression);
      const again = evaluate(expression);
      if (!rendered.ok) {
        worse('threw', rendered.threw);
        continue;
      }
      if (!again.ok || !Object.is(rendered.value, again.value)) {
        worse('nondeterministic', `answered ${describe(rendered.value)}, then something else`);
        continue;
      }
      // The rendering answered for the raw value; the rendered one has to clear
      // the same bar, since `join` of an array of objects is a string of
      // `[object Object]` and `String` of an empty array is the empty string.
      if (!carriesADeclaration(rendered.value)) {
        worse('unusable', `renders to ${describe(rendered.value)}`);
        continue;
      }

      return {
        args,
        rawCall,
        expression,
        property: rendering.property,
        value: String(rendered.value),
      };
    }
  }

  return refused;
}

/**
 * Whether a rendered answer is one a declaration can be written from.
 *
 * The empty string is refused for the reason `identical-empty` exists: a
 * property whose value is empty is one both compilers may drop, and a row like
 * that reports agreement while measuring nothing.
 */
function carriesADeclaration(value: unknown): boolean {
  if (typeof value === 'string') return value.trim() !== '';

  return typeof value === 'number' && Number.isFinite(value);
}

/** A value as the report names it, for a reason a reader has to act on. */
function describe(value: unknown): string {
  if (value === null) return 'null';
  if (value === undefined) return 'undefined';
  if (Array.isArray(value)) return `an array of ${value.length}`;
  if (typeof value === 'string') return value === '' ? 'the empty string' : `the string ${value}`;
  if (typeof value === 'number') return `the number ${value}`;
  if (typeof value === 'object') return 'an object with no plain shape';

  return `a ${typeof value}`;
}

/** The call itself, with the receiver written out and the arguments in place. */
function callText(surface: Surface, method: string, args: readonly string[]): string {
  const receiver = surface.kind === 'prototype' ? surface.receiver : surface.name;

  return `${receiver}.${method}(${args.join(', ')})`;
}

/** The name an argument takes in the named shape of a namespace surface. */
function argumentBinding(index: number): string {
  return `given${index}`;
}

/**
 * One subject: the declarations it carries and the expression under test.
 *
 * A prototype surface names its receiver and leaves the arguments written out,
 * so the receiver is the only thing that moves between the two shapes. A
 * namespace surface names its arguments instead, for the reason `Surface` gives.
 */
function shapeOf(
  surface: Surface,
  method: string,
  chosen: Chosen,
  shape: Shape
): { prelude: string[]; expression: string } | undefined {
  if (surface.kind === 'prototype') {
    const receiver = shape === 'named' ? surface.binding : surface.receiver;
    const prelude = shape === 'named' ? [`const ${surface.binding} = ${surface.receiver};`] : [];
    const call = `${receiver}.${method}(${chosen.args.join(', ')})`;

    return { prelude, expression: rerender(chosen, call) };
  }

  if (shape === 'written') {
    return { prelude: [], expression: chosen.expression };
  }

  // A call with no arguments has nothing to name, so its two shapes would be
  // the same subject asked twice. Reported as one rather than as two.
  if (chosen.args.length === 0) return undefined;

  const prelude = chosen.args.map((text, index) => `const ${argumentBinding(index)} = ${text};`);
  const names = chosen.args.map((_, index) => argumentBinding(index));
  const call = `${surface.name}.${method}(${names.join(', ')})`;

  return { prelude, expression: rerender(chosen, call) };
}

/**
 * The chosen rendering, applied to a differently written call.
 *
 * The rendering was decided from the value, and the value does not depend on
 * how the receiver was written — so re-deriving it here would be a second
 * decision that could disagree with the first. What is done instead is textual:
 * the written call is replaced inside the expression that was already accepted.
 */
function rerender(chosen: Chosen, call: string): string {
  const at = chosen.expression.indexOf(chosen.rawCall);
  // Unreachable as the two are built: the rendering inserts the call verbatim.
  // It throws rather than falling back to the bare call, because that fallback
  // would drop the terminal — and a subject whose terminal is missing answers a
  // value no declaration carries, which reads as agreement about nothing on
  // both sides. A sweep is the wrong place for a silent wrong answer.
  if (at === -1) {
    throw new Error(`the rendered expression ${chosen.expression} no longer carries its call`);
  }

  const after = chosen.expression.slice(at + chosen.rawCall.length);

  return `${chosen.expression.slice(0, at)}${call}${after}`;
}

/** Every subject the surfaces produce, and every method they could not ask. */
export function sweep(surfaces: readonly Surface[] = SURFACES): Sweep {
  const exercised: Exercised[] = [];
  const unexercised: Unexercised[] = [];

  for (const surface of surfaces) {
    for (const method of methodsOf(surface)) {
      const chosen = chooseArguments(surface, method);
      if (!isChosen(chosen)) {
        unexercised.push({
          surface: surface.name,
          method,
          rejection: chosen.rejection,
          detail: chosen.detail,
        });
        continue;
      }

      const asked: Asked[] = [];
      for (const shape of SHAPES) {
        const written = shapeOf(surface, method, chosen, shape);
        if (written === undefined) continue;
        asked.push({
          surface: surface.name,
          method,
          shape,
          expression: written.expression,
          value: chosen.value,
          subject: subjectFor(surface, method, shape, chosen.property, written),
        });
      }

      exercised.push({
        surface: surface.name,
        method,
        expression: chosen.expression,
        value: chosen.value,
        asked,
      });
    }
  }

  return { surfaces, exercised, unexercised, asked: exercised.flatMap(one => one.asked) };
}

function isChosen(chosen: Chosen | Refused): chosen is Chosen {
  return 'expression' in chosen;
}

/** The corpus entry a subject is handed to the comparer as. */
function subjectFor(
  surface: Surface,
  method: string,
  shape: Shape,
  property: string,
  written: { prelude: string[]; expression: string }
): ModuleSubject {
  const declarations = written.prelude.length === 0 ? '' : `\n${written.prelude.join('\n')}\n`;

  return {
    kind: 'module',
    set: SWEEP_SET,
    id: `${SWEEP_SET}-${slug(surface.name)}-${method}-${shape}`,
    label: labelFor(surface, method, shape),
    source:
      "import * as stylex from '@stylexjs/stylex';\n" +
      `${declarations}\nexport const styles = stylex.create({\n` +
      `  x: { ${property}: ${written.expression} },\n});\n`,
    origin: 'parity/fuzz-prototype-sweep.ts (generated)',
  };
}

/** How one subject is named in a line of the report. */
export function labelFor(surface: Surface, method: string, shape: Shape): string {
  const how = shape === 'named' ? 'named' : 'written out';

  return surface.kind === 'prototype'
    ? `\`${method}\` on a receiver ${how}, off ${surface.name}`
    : `\`${surface.name}.${method}\` on arguments ${how}`;
}

/** A surface name as an identifier a subject id can carry. */
function slug(name: string): string {
  return name.replace(/\./g, '-').toLowerCase();
}
