// The `Math` statics that the compiler replaces must fold to the same number as
// in the reference compiler, which folds them with the `Math` of Node. A
// difference in the last bit gives a different class name.
//
// The evaluator tests check the folded number. These cases check what the
// built addon makes of it: the printed CSS value and the class name hashed
// from it. Each expected rule is the output of the reference compiler for the
// same source.
import { execFileSync } from 'node:child_process';

import { describe, expect, test } from 'vitest';

import { transform } from '../dist/index.js';

/** Each expression goes into a `width` template, which prints every digit. */
const cases: [expression: string, rule: string][] = [
  // How each kind of value prints: zero, an integer, a fraction, an exponent,
  // the smallest subnormal number, infinity and NaN.
  ['Math.hypot()', '.xnalus7{width:0}'],
  ['Math.hypot(3, 4)', '.x1ftt334{width:5px}'],
  ['Math.hypot(0.1, 0.2)', '.x17p1j7s{width:.223606797749979px}'],
  ['Math.hypot(1e308, 1e308)', '.x15wz0cg{width:1.4142135623730951e+308px}'],
  ['Math.hypot(5e-324, 5e-324)', '.x6pb8ui{width:5e-324px}'],
  ['Math.hypot(NaN, Infinity)', '.x1fssspx{width:Infinitypx}'],
  ['Math.hypot(1, NaN)', '.x1c9rq88{width:NaNpx}'],
  // A fold of two-argument `hypot` calls gives a different last bit for these.
  ['Math.hypot(2, 3)', '.xjjti7{width:3.6055512754639896px}'],
  [
    'Math.hypot(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20)',
    '.xzpe2vy{width:53.57238094391549px}',
  ],
  ['Math.hypot(1e154, 1e154)', '.x1odrrsf{width:1.4142135623730953e+154px}'],
];

/**
 * A source of numbers in [0, 1) that is the same on every run: a linear
 * congruential generator with a modulus of 2^32.
 */
const seededRandom = (seed: number) => () => {
  seed = (Math.imul(seed, 1_103_515_245) + 12_345) >>> 0;
  return seed / 4_294_967_296;
};

/** A number with a random exponent and a random count of digits. */
const seededNumber = (random: () => number, exponent: number): number =>
  Number(((random() - 0.5) * 2 * 10 ** exponent).toPrecision(1 + Math.floor(random() * 17)));

/**
 * Argument lists for `Math.hypot`, the same on every run.
 *
 * The arguments of one list have a similar size, because then the method of
 * the sum changes the last bit of the result.
 */
const seededArgumentLists = (count: number): number[][] => {
  const random = seededRandom(1338);

  return Array.from({ length: count }, () => {
    const exponent = Math.floor(random() * 600 - 300);
    return Array.from({ length: 1 + Math.floor(random() * 6) }, () =>
      seededNumber(random, exponent)
    );
  });
};

/**
 * The seeded argument at `index` of a list, from `random`.
 *
 * Most of them are near zero, where the statics are most used and where the
 * C library and Node most often differ. A quarter are between -1 and 1, where
 * the inverse statics are defined. A quarter have any exponent, down to the
 * subnormal numbers.
 */
const seededArgument = (random: () => number, index: number): number => {
  if (index % 4 === 0) {
    return seededNumber(random, Math.floor(random() * 630 - 322));
  }
  return seededNumber(random, index % 4 === 1 ? 0 : random() * 3);
};

/** Arguments for a static of one argument, the same on every run. */
const seededArguments = (count: number): number[] => {
  const random = seededRandom(2024);

  return Array.from({ length: count }, (_, index) => seededArgument(random, index));
};

/** Values that `Math.atan2` treats specially, in either argument. */
const specialValues = [0, -0, 1, -1, Infinity, -Infinity];

/**
 * Argument pairs `(y, x)` for `Math.atan2`, the same on every run: every pair
 * of special values, then pairs of seeded arguments. The seeded arguments have
 * both signs, so the pairs are in all four quadrants.
 */
const seededArgumentPairs = (count: number): [y: number, x: number][] => {
  const random = seededRandom(2024);

  return [
    ...specialValues.flatMap(y => specialValues.map((x): [y: number, x: number] => [y, x])),
    ...Array.from({ length: count }, (_, index): [y: number, x: number] => [
      seededArgument(random, 2 * index),
      seededArgument(random, 2 * index + 1),
    ]),
  ];
};

/** The source of `value`, with the sign of a zero, which a template drops. */
const sourceOf = (value: number): string => (Object.is(value, -0) ? '-0' : String(value));

/**
 * The `Math` statics of one argument that Node computes with its own fdlibm.
 * With `Math.atan2`, which has its own test below, `FDLIBM_STATICS` in the
 * evaluator (`engine_fold/math.rs`) names the same statics. Change the two
 * together.
 */
const fdlibmStatics = [
  'acos',
  'acosh',
  'asin',
  'asinh',
  'atan',
  'atanh',
  'cbrt',
  'cos',
  'cosh',
  'exp',
  'expm1',
  'log',
  'log10',
  'log1p',
  'log2',
  'sin',
  'sinh',
  'tan',
  'tanh',
] as const;

/**
 * Node gives the reference answer of an fdlibm static only on x64. On arm64,
 * its C++ compiler fuses some multiplications and additions of fdlibm, and the
 * last bit can change. The compiler gives the x64 answer on every host (see
 * ADR 0008 of the evaluator), so a seeded comparison with Node runs on x64
 * only.
 */
const nodeGivesTheReference = process.arch === 'x64';

/**
 * Whether `**` and `Math.pow` of this Node call the `pow` of the C library, as
 * the fold does for each exponent other than 2 and one half. Node 24 and later
 * turn on the V8 flag `--use-std-math-pow` by default. Node 22 uses fdlibm and
 * gives a different last bit for some calls. ADR 0008 of the evaluator says
 * why the fold keeps the answer of Node 24.
 *
 * The test reads the flag and not the version of Node, so that the comparison
 * follows what each Node does.
 */
const nodeCallsHostPow = /\bdefault: --use-std-math-pow\b/.test(
  execFileSync(process.execPath, ['--v8-options'], { encoding: 'utf8' })
);

/** An expression and the number that it must compile to. */
type Case = readonly [expression: string, value: number];

/**
 * The three spellings of `base ** exponent`. Each one goes to a different fold:
 * the evaluator, the global native of the engine, and the native `Math.pow` of
 * the engine.
 */
const powSpellings: [name: string, spell: (base: string, exponent: string) => string][] = [
  ['** in the evaluator', (base, exponent) => `(${base}) ** (${exponent})`],
  ['** in the engine', (base, exponent) => `String((${base}) ** (${exponent}))`],
  ['Math.pow', (base, exponent) => `Math.pow(${base}, ${exponent})`],
];

/**
 * One call of each fdlibm static, and its value. Node gives this value on x64
 * and on arm64 (both were measured), and the C library of the engine does not.
 */
const fixedCases: Case[] = [
  ['Math.acos(0.05)', 1.5207754699891267],
  ['Math.acosh(1.1)', 0.4435682543851154],
  ['Math.asin(0.5)', 0.5235987755982989],
  ['Math.asinh(0.08)', 0.07991491149449678],
  ['Math.atan(0.5)', 0.4636476090008061],
  ['Math.atan2(0.08, 0.3)', 0.26060239174734096],
  ['Math.atanh(0.5)', 0.5493061443340548],
  ['Math.cbrt(0.02)', 0.27144176165949063],
  ['Math.cos(0.1)', 0.9950041652780257],
  ['Math.cosh(0.4)', 1.081072371838455],
  ['Math.exp(0.27)', 1.3099644507332475],
  ['Math.expm1(1)', 1.718281828459045],
  ['Math.log(0.09)', -2.407945608651872],
  ['Math.log10(0.52)', -0.2839966563652008],
  ['Math.log1p(0.2)', 0.18232155679395462],
  ['Math.log2(1.3)', 0.3785116232537299],
  ['Math.sin(0.51)', 0.48817724688290753],
  ['Math.sinh(0.2)', 0.20133600254109402],
  ['Math.tan(1)', 1.5574077246549023],
  ['Math.tanh(0.02)', 0.01999733375993093],
];

/**
 * Calls whose value x64 Node gives and arm64 Node does not. The compiler gives
 * the x64 value on every host, so these run on every host.
 */
const x64Cases: Case[] = [['Math.tan(1e22)', -1.628778225606899]];

/**
 * Calls that the engine runs from its own code: a static in a callback, a
 * string argument, and a base whose sign changes the result.
 */
const engineCases: Case[] = [
  ['[0.51].map((x) => Math.sin(x))[0]', 0.48817724688290753],
  ['[1].map((y, i) => Math.atan2(y, i))[0]', Math.PI / 2],
  ["Math.sin('0.51')", 0.48817724688290753],
  ['Math.pow(-Infinity, 3)', -Infinity],
  ['Math.pow(-0, -1)', -Infinity],
  ['String((-Infinity) ** 3)', -Infinity],
  ['String((-0) ** -1)', -Infinity],
  ['String((-Infinity) ** 0.5)', Infinity],
];

/**
 * `**` in the source that the engine runs, with an exponent of 2 or one half.
 * The engine computed these with the `pow` of the C library, which can round
 * them differently from Node. Every Node gives the same answer for these
 * exponents, so the answer of the Node that runs the test is the reference.
 */
const exponentiationCases: Case[] = [
  ['String(952.4673882682695 ** 0.5)', 952.4673882682695 ** 0.5],
  ['[NaN].map((x) => 1 ** x)[0]', NaN],
];

/**
 * `**` with any other exponent. The engine computed the first four with
 * repeated multiplication or with the `pow` of the C library, and each one
 * gave a different number from Node. These compare with Node only where
 * `nodeCallsHostPow` is true, because Node 22 gives a different `7 ** 30`.
 */
const hostPowCases: Case[] = [
  ['String(1.092492 ** 15)', 1.092492 ** 15],
  ['String(1.1 ** -9)', 1.1 ** -9],
  ['String(3 ** 40)', 3 ** 40],
  ['[15].map((n) => 1.092492 ** n)[0]', 1.092492 ** 15],
  ...powSpellings.map(([, spell]): Case => [spell('7', '30'), 7 ** 30]),
];

/** The CSS rule that the width `expression` compiles to. */
const ruleOf = (expression: string): string => {
  const source =
    `import * as stylex from '@stylexjs/stylex';\n` +
    `export const styles = stylex.create({ root: { width: \`\${${expression}}px\` } });\n`;
  const result = transform('page.tsx', source, {
    dev: false,
    unstable_moduleResolution: { type: 'commonJS' },
  });

  return result.metadata.stylex.map(([, rule]) => rule.ltr).join('');
};

/**
 * The number that the width `expression` compiles to. A printed width reads
 * back as the same number, except that a zero loses its sign and its unit.
 */
const widthOf = (expression: string): number =>
  Number(/width:(.*?)(?:px)?}/.exec(ruleOf(expression))?.[1]);

/** `value` with the sign of a zero removed, as a printed width removes it. */
const unsigned = (value: number): number => (value === 0 ? 0 : value);

/**
 * The width that each expression of `table` compiles to. A printed width reads
 * back as the same number, so a comparison with `valuesOf` compares every bit.
 */
const widthsOf = (table: readonly Case[]): number[] =>
  table.map(([expression]) => widthOf(expression));

/** The value of each case of `table`, as a printed width gives it. */
const valuesOf = (table: readonly Case[]): number[] => table.map(([, value]) => unsigned(value));

describe('Math.hypot', () => {
  test('folds every case to the rule of the reference compiler', () => {
    expect(
      Object.fromEntries(cases.map(([expression]) => [expression, ruleOf(expression)]))
    ).toEqual(Object.fromEntries(cases));
  });

  test('folds seeded argument lists to the number that Node gives', () => {
    const argumentLists = seededArgumentLists(300);

    // A printed width reads back as the same number, so this compares every bit.
    expect(argumentLists.map(values => widthOf(`Math.hypot(${values.join(', ')})`))).toEqual(
      argumentLists.map(values => Math.hypot(...values))
    );
  });
});

test('folds one call of each fdlibm static to the value of Node', () => {
  expect(widthsOf(fixedCases)).toEqual(valuesOf(fixedCases));
});

test.each([
  ['the x64 value of Node on every host', x64Cases],
  ['the calls that the engine runs to the value of Node', engineCases],
  ['`**` in the engine to the value of Node', exponentiationCases],
])('folds %s', (_, table) => {
  expect(widthsOf(table)).toEqual(valuesOf(table));
});

test.runIf(nodeCallsHostPow)(
  'folds `**` that fdlibm rounds differently to the value of Node',
  () => {
    expect(widthsOf(hostPowCases)).toEqual(valuesOf(hostPowCases));
  }
);

describe.each(fdlibmStatics)('Math.%s', name => {
  test.runIf(nodeGivesTheReference)('folds seeded arguments to the number that Node gives', () => {
    const values = seededArguments(400);

    expect(values.map(value => widthOf(`Math.${name}(${sourceOf(value)})`))).toEqual(
      values.map(value => unsigned(Math[name](value)))
    );
  });
});

describe('Math.atan2', () => {
  test.runIf(nodeGivesTheReference)(
    'folds seeded argument pairs to the number that Node gives',
    () => {
      const pairs = seededArgumentPairs(400);

      expect(pairs.map(([y, x]) => widthOf(`Math.atan2(${sourceOf(y)}, ${sourceOf(x)})`))).toEqual(
        pairs.map(([y, x]) => unsigned(Math.atan2(y, x)))
      );
    }
  );
});

/**
 * The exponents that Node computes without the `pow` of the C library: 2 as
 * one multiplication, and one half as a square root. The answer is the same
 * on every host and on every Node, so these comparisons run everywhere.
 */
const hostIndependentExponents = ['2', '0.5'] as const;

describe.each(hostIndependentExponents)('an exponent of %s', exponent => {
  const values = seededArguments(400).filter(value => value >= 0);

  test.each(powSpellings)(
    'folds seeded bases with %s to the number that Node gives',
    (_, spell) => {
      expect(values.map(value => widthOf(spell(sourceOf(value), exponent)))).toEqual(
        values.map(value => unsigned(value ** Number(exponent)))
      );
    }
  );
});

/**
 * An exponent for `**`, from `random`: an integer, a short fraction, or a
 * number with any count of digits, of either sign.
 */
const seededExponent = (random: () => number, index: number): number => {
  if (index % 3 === 0) {
    return Math.floor(random() * 81) - 40;
  }
  if (index % 3 === 1) {
    return Number(((random() - 0.5) * 8).toFixed(1 + Math.floor(random() * 3)));
  }
  return seededNumber(random, random() * 2);
};

/**
 * Pairs `(base, exponent)` for `**`, the same on every run. A base is below
 * 1000 and an exponent below 100, so that most results are finite and test the
 * rounding. A base is negative only under an integer exponent, because any
 * other exponent makes it NaN.
 */
const seededPowers = (count: number): [base: number, exponent: number][] => {
  const random = seededRandom(1339);

  return Array.from({ length: count }, (_, index): [base: number, exponent: number] => {
    const exponent = seededExponent(random, index);
    const base = seededNumber(random, random() * 3);
    return [Number.isInteger(exponent) ? base : Math.abs(base), exponent];
  });
};

describe.each(powSpellings)('%s', (_, spell) => {
  test.runIf(nodeCallsHostPow)('folds seeded powers to the number that Node gives', () => {
    const powers = seededPowers(300);

    expect(
      powers.map(([base, exponent]) => widthOf(spell(sourceOf(base), sourceOf(exponent))))
    ).toEqual(powers.map(([base, exponent]) => unsigned(base ** exponent)));
  });
});

describe('`**` in a large fold', () => {
  /** Bases below 100, so that no square and no sum passes the range of a number. */
  const random = seededRandom(1340);
  const bases = Array.from({ length: 10_000 }, () => Math.abs(seededNumber(random, 2)));

  /** A fold in the engine that raises each base to `exponent` and adds the powers. */
  const sumOfPowers = (exponent: string): string =>
    `[${bases.join(', ')}].map((x, i) => x ** (${exponent})).reduce((a, b) => a + b)`;

  test('folds 10,000 squares and square roots in the engine to the number that Node gives', () => {
    expect(widthOf(sumOfPowers('i % 2 ? 2 : 0.5'))).toBe(
      bases.map((x, i) => x ** (i % 2 ? 2 : 0.5)).reduce((a, b) => a + b)
    );
  });

  test.runIf(nodeCallsHostPow)(
    'folds 10,000 powers with any exponent in the engine to the number that Node gives',
    () => {
      expect(widthOf(sumOfPowers('i % 7 - 3.25'))).toBe(
        bases.map((x, i) => x ** ((i % 7) - 3.25)).reduce((a, b) => a + b)
      );
    }
  );
});

/**
 * `base`, raised in turn to 0.5, 2, 0.5 and so on, `levels` times, as one
 * nested expression. A base of 1.5 and two levels give `((1.5) ** 0.5) ** 2`.
 */
const nestedPower = (base: number, levels: number): string =>
  Array.from({ length: levels }, (_, index) => (index % 2 ? 2 : 0.5)).reduce(
    (source, exponent) => `(${source}) ** ${exponent}`,
    String(base)
  );

/** `count` squares as one sum: `0.5 ** 2 + 1.5 ** 2 + ...`. */
const sumOfSquares = (count: number): string =>
  Array.from({ length: count }, (_, index) => `${index + 0.5} ** 2`).join(' + ');

// The evaluator tests check where the fold stops. These check that the addon
// reports a deep source as an error, and that the process does not end. The
// sources are past the ceiling of 32 levels, but not so deep that the parser
// fills the 1 MB stack that Windows gives the main thread.
describe('a source past the depth ceiling of the fold', () => {
  test.each([
    ['200 nested powers', nestedPower(1.5, 200)],
    ['a sum of 500 powers', sumOfSquares(500)],
  ])('refuses %s with an error', (_, expression) => {
    expect(() => ruleOf(expression)).toThrow(/too deeply nested/);
  });
});

// The parser of the addon reads the source. The language does not allow a
// unary operator directly before `**`, so the parse error must reach the caller.
test.each(['-2 ** 2', '2 ** -1 ** 2'])('refuses the source %s with a parse error', expression => {
  expect(() => ruleOf(expression)).toThrow(/Failed to parse/);
});
