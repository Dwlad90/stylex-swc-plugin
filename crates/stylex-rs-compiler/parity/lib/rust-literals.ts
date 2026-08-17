/**
 * A minimal scanner for Rust string literals.
 *
 * The corpus harvester reads CSS values out of Rust test sources. Doing that
 * with a bare regex over the file text goes wrong immediately: the tests use
 * raw strings (`r#"…"#`) for values containing quotes and backslashes, which
 * is precisely the interesting set — URLs, escapes, `content` values. So the
 * literals are scanned properly, escapes are decoded for cooked strings, and
 * raw strings are taken verbatim.
 *
 * This is not a Rust parser. It skips line and block comments and character
 * literals, which is enough to keep it from mistaking a comment for code, and
 * it does not need to understand anything else.
 */

export interface RustLiteral {
  /** The decoded string value. */
  value: string;
  /** Byte offset of the opening delimiter in the source. */
  start: number;
  /** Byte offset just past the closing delimiter. */
  end: number;
  /** 1-based line number of the opening delimiter. */
  line: number;
  /** True when the literal was written as a raw string. */
  raw: boolean;
}

/** Decode the escape sequences a Rust cooked string literal can contain. */
function decodeEscapes(body: string): string {
  let out = '';
  for (let i = 0; i < body.length; i++) {
    const char = body[i];
    if (char !== '\\') {
      out += char;
      continue;
    }
    const next = body[++i];
    // A trailing backslash has nothing to escape; the literal ends there.
    if (next === undefined) break;
    switch (next) {
      case 'n':
        out += '\n';
        break;
      case 'r':
        out += '\r';
        break;
      case 't':
        out += '\t';
        break;
      case '0':
        out += '\0';
        break;
      case '\\':
        out += '\\';
        break;
      case "'":
        out += "'";
        break;
      case '"':
        out += '"';
        break;
      case 'u': {
        // `\u{1F600}`
        const close = body.indexOf('}', i);
        if (body[i + 1] === '{' && close !== -1) {
          out += String.fromCodePoint(Number.parseInt(body.slice(i + 2, close), 16));
          i = close;
        }
        break;
      }
      case 'x': {
        out += String.fromCodePoint(Number.parseInt(body.slice(i + 1, i + 3), 16));
        i += 2;
        break;
      }
      case '\n': {
        // A backslash at end of line eats the newline and leading whitespace.
        while (i + 1 < body.length && /\s/.test(body[i + 1]!)) i++;
        break;
      }
      default:
        out += next;
    }
  }
  return out;
}

/** Every string literal in a Rust source file, in source order. */
export function scanRustLiterals(source: string): RustLiteral[] {
  const literals: RustLiteral[] = [];
  const lineStarts = [0];
  for (let i = 0; i < source.length; i++) {
    if (source[i] === '\n') lineStarts.push(i + 1);
  }
  const lineOf = (offset: number): number => {
    let low = 0;
    let high = lineStarts.length - 1;
    while (low < high) {
      const mid = (low + high + 1) >> 1;
      if (lineStarts[mid]! <= offset) low = mid;
      else high = mid - 1;
    }
    return low + 1;
  };

  let i = 0;
  while (i < source.length) {
    const char = source[i]!;

    if (char === '/' && source[i + 1] === '/') {
      const nl = source.indexOf('\n', i);
      i = nl === -1 ? source.length : nl + 1;
      continue;
    }

    if (char === '/' && source[i + 1] === '*') {
      let depth = 1;
      i += 2;
      while (i < source.length && depth > 0) {
        if (source[i] === '/' && source[i + 1] === '*') {
          depth++;
          i += 2;
        } else if (source[i] === '*' && source[i + 1] === '/') {
          depth--;
          i += 2;
        } else i++;
      }
      continue;
    }

    // Raw string: `r`, some run of `#`, then `"`.
    if (char === 'r' && !isIdentChar(source[i - 1])) {
      let hashes = 0;
      while (source[i + 1 + hashes] === '#') hashes++;
      if (source[i + 1 + hashes] === '"') {
        const bodyStart = i + 2 + hashes;
        const terminator = `"${'#'.repeat(hashes)}`;
        const close = source.indexOf(terminator, bodyStart);
        if (close !== -1) {
          literals.push({
            value: source.slice(bodyStart, close),
            start: i,
            end: close + terminator.length,
            line: lineOf(i),
            raw: true,
          });
          i = close + terminator.length;
          continue;
        }
      }
    }

    if (char === '"') {
      let j = i + 1;
      while (j < source.length) {
        if (source[j] === '\\') j += 2;
        else if (source[j] === '"') break;
        else j++;
      }
      literals.push({
        value: decodeEscapes(source.slice(i + 1, j)),
        start: i,
        // Clamped, because an unterminated literal — or a trailing `\` — leaves
        // `j` at or past the end. `maskLiterals` replaces `[start, end)` one
        // character at a time, so an out-of-range `end` would make the mask
        // longer than the source and silently invalidate every offset the
        // harvester later compares against it.
        end: Math.min(j + 1, source.length),
        line: lineOf(i),
        raw: false,
      });
      i = j + 1;
      continue;
    }

    // A char literal such as `'"'` would otherwise open a phantom string.
    if (char === "'" && source[i + 2] === "'") {
      i += 3;
      continue;
    }
    if (char === "'" && source[i + 1] === '\\') {
      const close = source.indexOf("'", i + 2);
      if (close !== -1) {
        i = close + 1;
        continue;
      }
    }

    i++;
  }

  return literals;
}

function isIdentChar(char: string | undefined): boolean {
  return char !== undefined && /[A-Za-z0-9_]/.test(char);
}
