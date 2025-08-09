"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.TokenParser = exports.TokenOneOrMoreParsers = void 0;
var _tokenTypes = require("./token-types");
var _cssTokenizer = require("@csstools/css-tokenizer");
class TokenParser {
  constructor(parser) {
    let label = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 'UnknownParser';
    this.run = parser;
    this.label = label;
  }
  parse(css) {
    const tokens = new _tokenTypes.TokenList(css);
    return this.run(tokens);
  }
  parseToEnd(css) {
    const tokens = new _tokenTypes.TokenList(css);
    const initialIndex = tokens.currentIndex;
    const output = this.run(tokens);
    if (output instanceof Error) {
      const consumedTokens = tokens.slice(initialIndex);
      tokens.setCurrentIndex(initialIndex);
      throw new Error(`Expected ${this.toString()} but got ${output.message}\n` + `Consumed tokens: ${consumedTokens.map(token => token[0]).join(', ')}`);
    }
    if (tokens.peek() != null) {
      const token = tokens.peek();
      if (token == null) {
        return output;
      }
      const consumedTokens = tokens.slice(initialIndex);
      throw new Error(`Expected end of input, got ${token[0]} instead\n` + `Consumed tokens: ${consumedTokens.map(token => token[0]).join(', ')}`);
    }
    return output;
  }
  map(f, label) {
    return new TokenParser(input => {
      const currentIndex = input.currentIndex;
      const result = this.run(input);
      if (result instanceof Error) {
        input.setCurrentIndex(currentIndex);
        return result;
      }
      return f(result);
    }, `${this.label}.map(${label ?? ''})`);
  }
  flatMap(f, label) {
    return new TokenParser(input => {
      const currentIndex = input.currentIndex;
      const output1 = this.run(input);
      if (output1 instanceof Error) {
        input.setCurrentIndex(currentIndex);
        return output1;
      }
      const secondParser = f(output1);
      const output2 = secondParser.run(input);
      if (output2 instanceof Error) {
        input.setCurrentIndex(currentIndex);
        return output2;
      }
      return output2;
    }, `${this.label}.flatMap(${label ?? ''})`);
  }
  or(parser2) {
    return new TokenParser(input => {
      const currentIndex = input.currentIndex;
      const output1 = this.run(input);
      if (output1 instanceof Error) {
        input.setCurrentIndex(currentIndex);
        const output2 = parser2.run(input);
        if (output2 instanceof Error) {
          input.setCurrentIndex(currentIndex);
        }
        return output2;
      }
      return output1;
    }, parser2.label === 'optional' ? `Optional<${this.label}>` : `OneOf<${this.label}, ${parser2.label}>`);
  }
  surroundedBy(prefix) {
    let suffix = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : prefix;
    return TokenParser.sequence(prefix, this, suffix).map(_ref => {
      let [_prefix, value, _suffix] = _ref;
      return value;
    });
  }
  skip(skipParser) {
    return this.flatMap(output => skipParser.map(() => output));
  }
  get optional() {
    return new TokenOptionalParser(this);
  }
  prefix(prefixParser) {
    return prefixParser.flatMap(() => this);
  }
  suffix(suffixParser) {
    return this.flatMap(output => suffixParser.map(() => output));
  }
  where(predicate) {
    let label = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : '';
    return this.flatMap(output => {
      if (predicate(output)) {
        return TokenParser.always(output);
      }
      return TokenParser.never();
    }, label);
  }
  toString() {
    return this.label;
  }
  static never() {
    return new TokenParser(() => new Error('Never'), 'Never');
  }
  static always(output) {
    return new TokenParser(() => output, output === undefined ? 'optional' : `Always<${String(output)}>`);
  }
  static token(tokenType) {
    let label = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : tokenType;
    return new TokenParser(input => {
      const currentIndex = input.currentIndex;
      const token = input.consumeNextToken();
      if (token == null) {
        input.setCurrentIndex(currentIndex);
        return new Error('Expected token');
      }
      if (token[0] !== tokenType) {
        input.setCurrentIndex(currentIndex);
        return new Error(`Expected token type ${tokenType}, got ${token[0]}`);
      }
      return token;
    }, label);
  }
  static string(str) {
    return TokenParser.tokens.Ident.map(token => token[4].value, '.value').where(value => value === str, `=== ${str}`);
  }
  static fn(name) {
    return TokenParser.tokens.Function.map(token => token[4].value, '.value').where(value => value === name, `=== ${name}`);
  }
  static tokens = (() => ({
    Comment: TokenParser.token(_cssTokenizer.TokenType.Comment, 'Comment'),
    AtKeyword: TokenParser.token(_cssTokenizer.TokenType.AtKeyword, 'AtKeyword'),
    BadString: TokenParser.token(_cssTokenizer.TokenType.BadString, 'BadString'),
    BadURL: TokenParser.token(_cssTokenizer.TokenType.BadURL, 'BadURL'),
    CDC: TokenParser.token(_cssTokenizer.TokenType.CDC, 'CDC'),
    CDO: TokenParser.token(_cssTokenizer.TokenType.CDO, 'CDO'),
    Colon: TokenParser.token(_cssTokenizer.TokenType.Colon, 'Colon'),
    Comma: TokenParser.token(_cssTokenizer.TokenType.Comma, 'Comma'),
    Delim: TokenParser.token(_cssTokenizer.TokenType.Delim, 'Delim'),
    Dimension: TokenParser.token(_cssTokenizer.TokenType.Dimension, 'Dimension'),
    EOF: TokenParser.token(_cssTokenizer.TokenType.EOF, 'EOF'),
    Function: TokenParser.token(_cssTokenizer.TokenType.Function, 'Function'),
    Hash: TokenParser.token(_cssTokenizer.TokenType.Hash, 'Hash'),
    Ident: TokenParser.token(_cssTokenizer.TokenType.Ident, 'Ident'),
    Number: TokenParser.token(_cssTokenizer.TokenType.Number, 'Number'),
    Percentage: TokenParser.token(_cssTokenizer.TokenType.Percentage, 'Percentage'),
    Semicolon: TokenParser.token(_cssTokenizer.TokenType.Semicolon, 'Semicolon'),
    String: TokenParser.token(_cssTokenizer.TokenType.String, 'String'),
    URL: TokenParser.token(_cssTokenizer.TokenType.URL, 'URL'),
    Whitespace: TokenParser.token(_cssTokenizer.TokenType.Whitespace, 'Whitespace'),
    OpenParen: TokenParser.token(_cssTokenizer.TokenType.OpenParen, 'OpenParen'),
    CloseParen: TokenParser.token(_cssTokenizer.TokenType.CloseParen, 'CloseParen'),
    OpenSquare: TokenParser.token(_cssTokenizer.TokenType.OpenSquare, 'OpenSquare'),
    CloseSquare: TokenParser.token(_cssTokenizer.TokenType.CloseSquare, 'CloseSquare'),
    OpenCurly: TokenParser.token(_cssTokenizer.TokenType.OpenCurly, 'OpenCurly'),
    CloseCurly: TokenParser.token(_cssTokenizer.TokenType.CloseCurly, 'CloseCurly'),
    UnicodeRange: TokenParser.token(_cssTokenizer.TokenType.UnicodeRange, 'UnicodeRange')
  }))();
  static oneOf() {
    for (var _len = arguments.length, parsers = new Array(_len), _key = 0; _key < _len; _key++) {
      parsers[_key] = arguments[_key];
    }
    return new TokenParser(input => {
      const errors = [];
      const index = input.currentIndex;
      for (const parser of parsers) {
        const output = typeof parser === 'function' ? parser().run(input) : parser.run(input);
        if (!(output instanceof Error)) {
          return output;
        }
        input.setCurrentIndex(index);
        errors.push(output);
      }
      return new Error('No parser matched\n' + errors.map(err => '- ' + err.toString()).join('\n'));
    });
  }
  static sequence() {
    for (var _len2 = arguments.length, parsers = new Array(_len2), _key2 = 0; _key2 < _len2; _key2++) {
      parsers[_key2] = arguments[_key2];
    }
    return new TokenParserSequence(parsers);
  }
  static setOf() {
    for (var _len3 = arguments.length, parsers = new Array(_len3), _key3 = 0; _key3 < _len3; _key3++) {
      parsers[_key3] = arguments[_key3];
    }
    return new TokenParserSet(parsers);
  }
  static zeroOrMore(parser) {
    return new TokenZeroOrMoreParsers(parser);
  }
  static oneOrMore(parser) {
    return new TokenOneOrMoreParsers(parser);
  }
}
exports.TokenParser = TokenParser;
class TokenZeroOrMoreParsers extends TokenParser {
  constructor(parser, separator) {
    super(input => {
      const output = [];
      for (let i = 0; true; i++) {
        if (i > 0 && separator) {
          const currentIndex = input.currentIndex;
          const result = separator.run(input);
          if (result instanceof Error) {
            input.setCurrentIndex(currentIndex);
            return output;
          }
        }
        const currentIndex = input.currentIndex;
        const result = parser.run(input);
        if (result instanceof Error) {
          input.setCurrentIndex(currentIndex);
          return output;
        }
        output.push(result);
      }
      return output;
    }, `ZeroOrMore<${parser.label}>`);
    this.parser = parser;
    this.separator = separator;
  }
  separatedBy(separator) {
    const voidedSeparator = separator.map(() => undefined);
    const newSeparator = this.separator?.surroundedBy(voidedSeparator) ?? voidedSeparator;
    return new TokenZeroOrMoreParsers(this.parser, newSeparator);
  }
}
class TokenOneOrMoreParsers extends TokenParser {
  constructor(parser, separator) {
    super(input => {
      const output = [];
      for (let i = 0; true; i++) {
        if (i > 0 && separator) {
          const currentIndex = input.currentIndex;
          const result = separator.run(input);
          if (result instanceof Error) {
            input.setCurrentIndex(currentIndex);
            return output;
          }
        }
        const currentIndex = input.currentIndex;
        const result = parser.run(input);
        if (result instanceof Error) {
          if (i === 0) {
            input.setCurrentIndex(currentIndex);
            return result;
          }
          return output;
        }
        output.push(result);
      }
      return output;
    }, `OneOrMore<${parser.label}>`);
    this.parser = parser;
    this.separator = separator;
  }
  separatedBy(separator) {
    const voidedSeparator = separator.map(() => undefined);
    const newSeparator = this.separator?.surroundedBy(voidedSeparator) ?? voidedSeparator;
    return new TokenOneOrMoreParsers(this.parser, newSeparator);
  }
}
exports.TokenOneOrMoreParsers = TokenOneOrMoreParsers;
class TokenParserSequence extends TokenParser {
  constructor(parsers, _separator) {
    const separator = _separator?.map(() => undefined);
    super(input => {
      const currentIndex = input.currentIndex;
      let failed = null;
      const output = parsers.map(_parser => {
        if (failed) {
          return new Error('already failed');
        }
        let parser = _parser;
        if (separator != null && input.currentIndex > currentIndex) {
          if (parser instanceof TokenOptionalParser) {
            parser = TokenParser.sequence(separator, parser.parser).map(_ref2 => {
              let [_separator, value] = _ref2;
              return value;
            }).optional;
          } else {
            parser = TokenParser.sequence(separator, parser).map(_ref3 => {
              let [_separator, value] = _ref3;
              return value;
            });
          }
        }
        const result = parser.run(input);
        if (result instanceof Error) {
          failed = result;
        }
        return result;
      });
      if (failed) {
        const errorToReturn = failed;
        input.setCurrentIndex(currentIndex);
        return errorToReturn;
      }
      return output;
    }, `Sequence<${parsers.map(parser => parser.label).join(', ')}>`);
    this.parsers = parsers;
    this.separator = separator;
  }
  separatedBy(separator) {
    const newSeparator = this.separator?.surroundedBy(separator.map(() => undefined)) ?? separator.map(() => undefined);
    return new TokenParserSequence(this.parsers, newSeparator);
  }
}
class TokenOptionalParser extends TokenParser {
  constructor(parser) {
    super(parser.or(TokenParser.always(undefined)).run, `Optional<${parser.label}>`);
    this.parser = parser;
  }
}
class TokenParserSet extends TokenParser {
  constructor(_parsers, separator) {
    super(input => {
      const parsers = _parsers.map((parser, i) => [parser, i]).sort((_ref4, _ref5) => {
        let [a] = _ref4;
        let [b] = _ref5;
        if (a instanceof TokenOptionalParser) {
          return 1;
        }
        if (b instanceof TokenOptionalParser) {
          return -1;
        }
        return 0;
      });
      const currentIndex = input.currentIndex;
      let failed = null;
      const output = [];
      const indices = new Set();
      for (let i = 0; i < parsers.length; i++) {
        let found = false;
        const errors = [];
        for (let j = 0; j < parsers.length; j++) {
          if (indices.has(j)) {
            continue;
          }
          let [parser, index] = parsers[j];
          if (separator != null && i > 0) {
            if (parser instanceof TokenOptionalParser) {
              parser = TokenParser.sequence(separator, parser.parser).map(_ref6 => {
                let [_separator, value] = _ref6;
                return value;
              }).optional;
            } else {
              parser = TokenParser.sequence(separator, parser).map(_ref7 => {
                let [_separator, value] = _ref7;
                return value;
              });
            }
          }
          const currentIndex = input.currentIndex;
          const result = parser.run(input);
          if (result instanceof Error) {
            input.setCurrentIndex(currentIndex);
            errors.push(result);
          } else {
            found = true;
            output[index] = result;
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
      if (failed instanceof Error) {
        input.setCurrentIndex(currentIndex);
        return failed;
      }
      return output;
    });
    this.parsers = _parsers;
    this.separator = separator;
  }
  separatedBy(separator) {
    const voidedSeparator = separator.map(() => undefined);
    const sep = this.separator?.surroundedBy(voidedSeparator) ?? voidedSeparator;
    return new TokenParserSet(this.parsers, sep);
  }
}