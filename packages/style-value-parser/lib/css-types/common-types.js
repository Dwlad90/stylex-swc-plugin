"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.unset = exports.revert = exports.numberOrPercentage = exports.initial = exports.inherit = exports.cssWideKeywords = exports.auto = exports.Percentage = exports.CssVariable = void 0;
var _tokenParser = require("../token-parser");
var _cssTokenizer = require("@csstools/css-tokenizer");
const cssWideKeywords = exports.cssWideKeywords = _tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v === 'inherit' || v === 'initial' || v === 'unset' || v === 'revert');
const inherit = exports.inherit = cssWideKeywords.where(v => v === 'inherit');
const initial = exports.initial = cssWideKeywords.where(v => v === 'initial');
const unset = exports.unset = cssWideKeywords.where(v => v === 'unset');
const revert = exports.revert = cssWideKeywords.where(v => v === 'revert');
const auto = exports.auto = _tokenParser.TokenParser.token(_cssTokenizer.TokenType.Ident).map(v => v[4].value).where(v => v === 'auto');
class CssVariable {
  constructor(name) {
    this.name = name;
  }
  toString() {
    return `var(${this.name})`;
  }
  static parse = (() => _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'var'), _tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(v => v.startsWith('--')), _tokenParser.TokenParser.tokens.CloseParen).map(_ref => {
    let [_, name, __] = _ref;
    return new CssVariable(name);
  }))();
}
exports.CssVariable = CssVariable;
class Percentage {
  constructor(value) {
    this.value = value;
  }
  toString() {
    return `${this.value}%`;
  }
  static get parser() {
    return _tokenParser.TokenParser.token(_cssTokenizer.TokenType.Percentage).map(v => new Percentage(v[4].value));
  }
}
exports.Percentage = Percentage;
const numberOrPercentage = exports.numberOrPercentage = _tokenParser.TokenParser.oneOf(Percentage.parser, _tokenParser.TokenParser.token(_cssTokenizer.TokenType.Number).map(v => v[4].signCharacter === '-' ? -v[4].value : v[4].value));