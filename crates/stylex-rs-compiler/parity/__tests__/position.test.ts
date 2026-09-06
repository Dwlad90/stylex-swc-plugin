import { describe, expect, test } from 'vitest';

import { babelPosition, formatPosition, positionVerdict, rustPosition } from '../lib/position.js';

/**
 * The two parsers behind the position harness, pinned against the text each
 * compiler actually writes — copied out of a real run, decoration included.
 *
 * A parser that silently misreads a frame turns the whole set green, so every
 * shape either compiler produces is asserted here: the frame, the frame with no
 * caret row, the location line, and the sentences that carry neither.
 */

/** One `@babel/code-frame` excerpt, as thrown by `@stylexjs/babel-plugin`. */
const BABEL_FRAME = `/abs/path/to/positions.js: Referenced value is not a constant.


  1 | import * as stylex from '@stylexjs/stylex';
> 2 | let c = 'red';
    |     ^^^^^^^^^
  3 | c = 'blue';
  4 | export const styles = stylex.create({ x: { color: c } });
  5 |`;

describe('babelPosition', () => {
  test('reads the marked line and the caret column', () => {
    expect(babelPosition(BABEL_FRAME)).toEqual({ line: 2, column: 5 });
  });

  test('counts the column in the source, not in the excerpt', () => {
    const frame = `/abs/path/to/positions.js: Unsupported expression: FunctionDeclaration


  1 | import * as stylex from '@stylexjs/stylex';
> 2 | function f() {}
    | ^^^^^^^^^^^^^^^
  3 | export const styles = stylex.create({ x: { color: f } });`;

    expect(babelPosition(frame)).toEqual({ line: 2, column: 1 });
  });

  test('keeps counting past a wide gutter', () => {
    const frame = `/abs/path/to/positions.js: Referenced value is not a constant.

  999 | const before = 1;
> 1000 | let c = 'red';
       |     ^^^^^^^^^`;

    expect(babelPosition(frame)).toEqual({ line: 1000, column: 5 });
  });

  test('is not fooled by a pipe inside the framed source', () => {
    const frame = `/abs/path/to/positions.js: Referenced value is not a constant.

> 2 | let c = a | b;
    |     ^^^^^^^^^`;

    expect(babelPosition(frame)).toEqual({ line: 2, column: 5 });
  });

  test('reports nothing for a refusal with no frame at all', () => {
    expect(
      babelPosition('/abs/path/to/positions.js: A style value can only contain a string.')
    ).toBeUndefined();
  });

  test('reports nothing for a marked line with no caret row under it', () => {
    expect(babelPosition('> 2 | let c = 0;')).toBeUndefined();
  });
});

/** One code frame as written to stderr by `@stylexswc/rs-compiler`. */
const RUST_FRAME = `error: [StyleX] x > color > Referenced value is not a constant.


 --> /abs/path/to/positions.js:2:5
  |
2 | let c = 'red';
  |     ^^^^^^^^^
  |
  = warning: Line number isn't real, it's just a placeholder, Please check the actual line number in your editor.`;

describe('rustPosition', () => {
  test('reads the location line', () => {
    expect(rustPosition(RUST_FRAME)).toEqual({ line: 2, column: 5 });
  });

  test('reads a path that carries a colon of its own', () => {
    expect(rustPosition(' --> C:\\repo\\positions.js:12:34')).toEqual({ line: 12, column: 34 });
  });

  test('reports nothing when nothing was written', () => {
    expect(rustPosition('')).toBeUndefined();
  });

  test('reports nothing for a location line with no column', () => {
    expect(rustPosition(' --> /abs/path/to/positions.js')).toBeUndefined();
  });
});

/** One position, spelled the way a frame prints it. */
const at = (line: number, column: number) => ({ line, column });

describe('positionVerdict', () => {
  test('agreement needs the column as well as the line', () => {
    expect(positionVerdict(at(2, 5), at(2, 5), true)).toBe('identical');
    expect(positionVerdict(at(2, 5), at(2, 7), true)).toBe('divergent');
    expect(positionVerdict(at(2, 5), at(4, 5), true)).toBe('divergent');
  });

  test('a missing position is its own verdict, not a divergence', () => {
    expect(positionVerdict(undefined, at(2, 5), true)).toBe('no-position');
    expect(positionVerdict(at(2, 5), undefined, true)).toBe('no-position');
  });

  test('both compilers silent is a hole they share, not a difference between them', () => {
    // Which is every CSS refusal: neither compiler frames one. Reported apart
    // from `no-position` because that verdict is a row to act on -- one side
    // stopped without saying where -- and this one is agreement about a hole.
    expect(positionVerdict(undefined, undefined, true)).toBe('neither-position');
  });

  test('a subject one compiler compiled is not a position question', () => {
    expect(positionVerdict(at(2, 5), at(2, 5), false)).toBe('not-refused');
    // Asked before either position is: a subject nobody refused has no position
    // to be silent about, so it must not read `neither-position`.
    expect(positionVerdict(undefined, undefined, false)).toBe('not-refused');
  });
});

describe('formatPosition', () => {
  test('prints a position as a frame writes it', () => {
    expect(formatPosition({ line: 2, column: 5 })).toBe('2:5');
  });

  test('prints an absent position as a dash', () => {
    expect(formatPosition(undefined)).toBe('—');
  });
});
