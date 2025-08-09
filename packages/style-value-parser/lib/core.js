"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.ParserSequence = exports.Parser = void 0;
var _baseTypes = require("./base-types");
class Parser {
  constructor(parser) {
    this.run = parser;
  }
  parse(input) {
    return this.run(new _baseTypes.SubString(input));
  }
  parseToEnd(input) {
    const subStr = new _baseTypes.SubString(input);
    const output = this.run(subStr);
    if (output instanceof Error) {
      throw output;
    }
    if (!subStr.isEmpty) {
      throw new Error(`Expected end of input, got ${subStr.string.slice(subStr.startIndex)} instead`);
    }
    return output;
  }
  map(f) {
    return new Parser(input => {
      const oldOutput = this.run(input);
      if (oldOutput instanceof Error) {
        return oldOutput;
      }
      return f(oldOutput);
    });
  }
  flatMap(f) {
    return new Parser(input => {
      const {
        startIndex,
        endIndex
      } = input;
      const output1 = this.run(input);
      if (output1 instanceof Error) {
        return output1;
      }
      const secondParser = f(output1);
      const output2 = secondParser.run(input);
      if (output2 instanceof Error) {
        input.startIndex = startIndex;
        input.endIndex = endIndex;
        return output2;
      }
      return output2;
    });
  }
  or(parser2) {
    return new Parser(input => {
      const output1 = this.run(input);
      if (output1 instanceof Error) {
        return parser2.run(input);
      }
      return output1;
    });
  }
  surroundedBy(prefix) {
    let suffix = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : prefix;
    return this.prefix(prefix).skip(suffix);
  }
  skip(skipParser) {
    return this.flatMap(output => skipParser.map(() => output));
  }
  get optional() {
    return this.or(Parser.always(undefined));
  }
  prefix(prefixParser) {
    return prefixParser.flatMap(() => this);
  }
  static never() {
    return new Parser(() => new Error('Never'));
  }
  static always(output) {
    return new Parser(() => output);
  }
  where(predicate) {
    return this.flatMap(output => {
      if (predicate(output)) {
        return Parser.always(output);
      }
      return Parser.never();
    });
  }
  static oneOf() {
    for (var _len = arguments.length, parsers = new Array(_len), _key = 0; _key < _len; _key++) {
      parsers[_key] = arguments[_key];
    }
    return new Parser(input => {
      const errors = [];
      for (const parser of parsers) {
        const output = parser.run(input);
        if (!(output instanceof Error)) {
          return output;
        }
        errors.push(output);
      }
      return new Error('No parser matched\n' + errors.map(err => '- ' + err.toString()).join('\n'));
    });
  }
  static sequence() {
    for (var _len2 = arguments.length, parsers = new Array(_len2), _key2 = 0; _key2 < _len2; _key2++) {
      parsers[_key2] = arguments[_key2];
    }
    return new ParserSequence(parsers);
  }
  static setOf() {
    for (var _len3 = arguments.length, parsers = new Array(_len3), _key3 = 0; _key3 < _len3; _key3++) {
      parsers[_key3] = arguments[_key3];
    }
    return new ParserSet(parsers);
  }
  static zeroOrMore(parser) {
    return new ZeroOrMoreParsers(parser);
  }
  static oneOrMore(parser) {
    return new OneOrMoreParsers(parser);
  }
  static string(str) {
    return new Parser(input => {
      const {
        startIndex,
        endIndex
      } = input;
      if (startIndex + str.length - 1 > endIndex) {
        return new Error('End of input');
      }
      if (input.startsWith(str)) {
        input.startIndex += str.length;
        return str;
      }
      return new Error(`Expected ${str}, got ${input.string.slice(startIndex)}`);
    });
  }
  static get quotedString() {
    const doubleQuotes = Parser.sequence(Parser.string('"'), Parser.zeroOrMore(Parser.oneOf(Parser.string('\\"').map(() => '"'), Parser.string('\\\\').map(() => '\\'), Parser.takeWhile(char => char !== '"' && char !== '\\'))), Parser.string('"')).map(_ref => {
      let [_openQuote, chars, _closeQuote] = _ref;
      return chars.join('');
    });
    const singleQuotes = Parser.sequence(Parser.string("'"), Parser.zeroOrMore(Parser.oneOf(Parser.string("\\'").map(() => "'"), Parser.string('\\\\').map(() => '\\'), Parser.takeWhile(char => char !== "'" && char !== '\\'))), Parser.string("'")).map(_ref2 => {
      let [, chars] = _ref2;
      return chars.join('');
    });
    return Parser.oneOf(doubleQuotes, singleQuotes);
  }
  static regex(regex) {
    return new Parser(input => {
      const {
        startIndex,
        endIndex
      } = input;
      if (startIndex > endIndex) {
        return new Error('End of input');
      }
      const match = input.string.slice(startIndex).match(regex);
      if (match) {
        input.startIndex += match[0].length;
        return match[0];
      }
      return new Error(`Expected ${String(regex)}, got ${input.string.slice(startIndex)}`);
    });
  }
  static takeWhile(predicate) {
    return new Parser(input => {
      const {
        startIndex,
        endIndex
      } = input;
      if (startIndex > endIndex) {
        return new Error('End of input');
      }
      let i = startIndex;
      while (i <= endIndex && predicate(input.string[i])) {
        i++;
      }
      const output = input.string.slice(startIndex, i);
      input.startIndex = i;
      return output;
    });
  }
  static get digit() {
    return new Parser(input => {
      if (input.first >= '0' && input.first <= '9') {
        const first = input.first;
        input.startIndex++;
        return first;
      }
      return new Error(`Expected digit, got ${input.first}`);
    });
  }
  static get letter() {
    return new Parser(input => {
      if (input.first >= 'a' && input.first <= 'z' || input.first >= 'A' && input.first <= 'Z') {
        const returnValue = input.first;
        input.startIndex++;
        return returnValue;
      }
      return new Error(`Expected letter, got ${input.first}`);
    });
  }
  static get natural() {
    return Parser.sequence(Parser.digit.where(digit => digit !== '0'), Parser.zeroOrMore(Parser.digit)).map(_ref3 => {
      let [first, rest] = _ref3;
      return parseInt(first + rest.join(''), 10);
    });
  }
  static get whole() {
    return Parser.sequence(Parser.oneOrMore(Parser.digit)).map(_ref4 => {
      let [digits] = _ref4;
      return parseInt(digits.join(''), 10);
    });
  }
  static get integer() {
    return Parser.sequence(Parser.string('-').optional.map(char => char != null ? -1 : 1), Parser.whole.map(int => int || 0)).map(_ref5 => {
      let [sign, int] = _ref5;
      return sign * int;
    });
  }
  static get float() {
    return Parser.oneOf(Parser.sequence(Parser.string('-').optional.map(char => char != null ? -1 : 1), Parser.whole.map(int => int || 0), Parser.string('.'), Parser.oneOrMore(Parser.digit)).map(_ref6 => {
      let [sign, int, _, digits] = _ref6;
      return sign * parseFloat(int + '.' + digits.join(''));
    }), Parser.sequence(Parser.string('-').optional.map(char => char != null ? -1 : 1), Parser.string('.'), Parser.oneOrMore(Parser.digit)).map(_ref7 => {
      let [sign, _, digits] = _ref7;
      return sign * parseFloat('0.' + digits.join(''));
    }), Parser.integer);
  }
  static get space() {
    return Parser.oneOrMore(Parser.string(' ')).map(() => undefined);
  }
  static get whitespace() {
    return Parser.oneOrMore(Parser.oneOf(Parser.string(' '), Parser.string('\n'), Parser.string('\r\n'))).map(() => undefined);
  }
}
exports.Parser = Parser;
class ZeroOrMoreParsers extends Parser {
  constructor(parser) {
    super(input => {
      const output = [];
      for (let i = 0; true; i++) {
        const separator = getThis().separator;
        if (i > 0 && separator) {
          const result = separator.run(input);
          if (result instanceof Error) {
            return output;
          }
        }
        const result = getThis().parser.run(input);
        if (result instanceof Error) {
          return output;
        }
        output.push(result);
      }
      return output;
    });
    const getThis = () => this;
    this.parser = parser;
  }
  separatedBy(separator) {
    this.separator = separator;
    return this;
  }
}
class OneOrMoreParsers extends Parser {
  constructor(parser) {
    super(input => {
      const output = [];
      for (let i = 0; true; i++) {
        const separator = getThis().separator;
        if (i > 0 && separator) {
          const result = separator.run(input);
          if (result instanceof Error) {
            return output;
          }
        }
        const result = getThis().parser.run(input);
        if (result instanceof Error) {
          if (i === 0) {
            return result;
          }
          return output;
        }
        output.push(result);
      }
      return output;
    });
    const getThis = () => this;
    this.parser = parser;
  }
  separatedBy(separator) {
    this.separator = separator;
    return this;
  }
}
class ParserSequence extends Parser {
  constructor(parsers) {
    super(input => {
      const {
        startIndex,
        endIndex
      } = input;
      let failed = null;
      const output = parsers.map(parser => {
        if (failed) {
          return Error('already failed');
        }
        const result = parser.run(input);
        if (result instanceof Error) {
          failed = result;
        }
        return result;
      });
      if (failed) {
        input.startIndex = startIndex;
        input.endIndex = endIndex;
        return failed;
      }
      return output;
    });
    this.parsers = parsers;
  }
  separatedBy(separator) {
    const parsers = this.parsers.map((originalParser, index) => index === 0 ? originalParser : originalParser.prefix(separator.map(() => undefined)));
    return new ParserSequence(parsers);
  }
}
exports.ParserSequence = ParserSequence;
class ParserSet extends Parser {
  constructor(parsers, separator) {
    super(input => {
      const {
        startIndex,
        endIndex
      } = input;
      let failed = null;
      const output = [];
      const indices = new Set();
      for (let i = 0; i < parsers.length; i++) {
        let found = false;
        const errors = [];
        if (separator != null && i > 0) {
          const result = separator.run(input);
          if (result instanceof Error) {
            failed = new Error(`Expected ${separator.toString()} but got ${result.message}`);
            break;
          }
        }
        for (let j = 0; j < parsers.length && !indices.has(j); j++) {
          const parser = parsers[j];
          const result = parser.run(input);
          if (result instanceof Error) {
            errors.push(result);
          } else {
            found = true;
            output[j] = result;
            indices.add(j);
            break;
          }
        }
        if (found) {
          continue;
        } else {
          failed = new Error(`Expected one of ${parsers.map(parser => parser.toString()).join(', ')} but got ${errors.map(error => error.message).join(', ')}`);
          break;
        }
      }
      if (failed) {
        input.startIndex = startIndex;
        input.endIndex = endIndex;
        return failed;
      }
      return output;
    });
    this.parsers = parsers;
    this.separator = separator;
  }
  separatedBy(separator) {
    const sep = this.separator != null ? this.separator.prefix(separator.map(() => undefined)) : separator.map(() => undefined);
    return new ParserSet(this.parsers, sep);
  }
}