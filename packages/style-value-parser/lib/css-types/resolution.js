"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Resolution = void 0;
var _tokenParser = require("../token-parser");
var _cssTokenizer = require("@csstools/css-tokenizer");
class Resolution {
  constructor(value, unit) {
    this.value = value;
    this.unit = unit;
  }
  toString() {
    return `${this.value}${this.unit}`;
  }
  static UNITS = ['dpi', 'dpcm', 'dppx'];
  static get parser() {
    return _tokenParser.TokenParser.token(_cssTokenizer.TokenType.Dimension).where(v => v[4].unit === 'dpi' || v[4].unit === 'dpcm' || v[4].unit === 'dppx').map(v => new Resolution(v[4].value, v[4].unit));
  }
}
exports.Resolution = Resolution;