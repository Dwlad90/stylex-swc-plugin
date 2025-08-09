"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.StepsKeyword = exports.StepsEasingFunction = exports.LinearEasingFunction = exports.EasingFunction = exports.CubicBezierKeyword = exports.CubicBezierEasingFunction = void 0;
var _tokenParser = require("../token-parser");
class EasingFunction {
  static get parser() {
    return _tokenParser.TokenParser.oneOf(LinearEasingFunction.parser, CubicBezierEasingFunction.parser, CubicBezierKeyword.parser, StepsEasingFunction.parser, StepsKeyword.parser);
  }
}
exports.EasingFunction = EasingFunction;
class LinearEasingFunction extends EasingFunction {
  constructor(points) {
    super();
    this.points = points;
  }
  toString() {
    return `linear(${this.points.join(', ')})`;
  }
  static get parser() {
    const pointsParser = _tokenParser.TokenParser.oneOrMore(_tokenParser.TokenParser.tokens.Number.map(v => v[4].value)).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'linear'), pointsParser, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref => {
      let [_linear, points, _end] = _ref;
      return new LinearEasingFunction(points);
    });
  }
}
exports.LinearEasingFunction = LinearEasingFunction;
class CubicBezierEasingFunction extends EasingFunction {
  constructor(points) {
    super();
    this.points = points;
  }
  toString() {
    return `cubic-bezier(${this.points.join(', ')})`;
  }
  static get parser() {
    const numbers = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value)).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'cubic-bezier'), numbers, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref2 => {
      let [_linear, points, _end] = _ref2;
      return new CubicBezierEasingFunction(points);
    });
  }
}
exports.CubicBezierEasingFunction = CubicBezierEasingFunction;
class CubicBezierKeyword extends EasingFunction {
  constructor(keyword) {
    super();
    this.keyword = keyword;
  }
  toString() {
    return this.keyword;
  }
  static get parser() {
    return _tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v === 'ease-in-out'), _tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v === 'ease-in'), _tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v === 'ease-out'), _tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v === 'ease')).map(keyword => new CubicBezierKeyword(keyword));
  }
}
exports.CubicBezierKeyword = CubicBezierKeyword;
class StepsEasingFunction extends EasingFunction {
  constructor(steps, start) {
    super();
    this.steps = steps;
    this.start = start;
  }
  toString() {
    return `steps(${this.steps}, ${this.start})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'steps'), _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v === 'start'), _tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v === 'end'))).separatedBy(_tokenParser.TokenParser.tokens.Comma.surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional)).surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional), _tokenParser.TokenParser.tokens.CloseParen).map(_ref3 => {
      let [_fn, [steps, start], _end] = _ref3;
      return new StepsEasingFunction(steps, start);
    });
  }
}
exports.StepsEasingFunction = StepsEasingFunction;
class StepsKeyword extends EasingFunction {
  constructor(keyword) {
    super();
    this.keyword = keyword;
  }
  toString() {
    return this.keyword;
  }
  static get parser() {
    return _tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v === 'step-start'), _tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v === 'step-end')).map(keyword => new StepsKeyword(keyword));
  }
}
exports.StepsKeyword = StepsKeyword;