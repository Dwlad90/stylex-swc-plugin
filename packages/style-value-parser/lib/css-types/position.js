"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Position = void 0;
var _tokenParser = require("../token-parser");
var _cssTokenizer = require("@csstools/css-tokenizer");
var _lengthPercentage = require("./length-percentage");
class Position {
  constructor(horizontal, vertical) {
    this.horizontal = horizontal;
    this.vertical = vertical;
  }
  toString() {
    const horizontal = Array.isArray(this.horizontal) ? this.horizontal.join(' ') : this.horizontal?.toString();
    const vertical = Array.isArray(this.vertical) ? this.vertical.join(' ') : this.vertical?.toString();
    return [horizontal, vertical].filter(Boolean).join(' ');
  }
  static get parser() {
    const horizontalKeyword = _tokenParser.TokenParser.token(_cssTokenizer.TokenType.Ident).map(token => token[4].value).where(str => str === 'left' || str === 'center' || str === 'right');
    const verticalKeyword = _tokenParser.TokenParser.token(_cssTokenizer.TokenType.Ident).map(token => token[4].value).where(str => str === 'top' || str === 'center' || str === 'bottom');
    const horizontal = _tokenParser.TokenParser.sequence(horizontalKeyword, _lengthPercentage.lengthPercentage.optional).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref => {
      let [keyword, length] = _ref;
      return length ? [keyword, length] : keyword;
    });
    const vertical = _tokenParser.TokenParser.sequence(verticalKeyword, _lengthPercentage.lengthPercentage.optional).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref2 => {
      let [keyword, length] = _ref2;
      return length ? [keyword, length] : keyword;
    });
    const bothKeywords = _tokenParser.TokenParser.setOf(horizontal, vertical).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref3 => {
      let [h, v] = _ref3;
      return new Position(h, v);
    });
    const numberPlusVertical = _tokenParser.TokenParser.sequence(_lengthPercentage.lengthPercentage, vertical).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref4 => {
      let [length, v] = _ref4;
      return new Position(length, v);
    });
    const numberPlusHorizontal = _tokenParser.TokenParser.sequence(_lengthPercentage.lengthPercentage, horizontal).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref5 => {
      let [length, h] = _ref5;
      return new Position(h, length);
    });
    const numbersOnly = _tokenParser.TokenParser.sequence(_lengthPercentage.lengthPercentage, _lengthPercentage.lengthPercentage.optional).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref6 => {
      let [length1, length2] = _ref6;
      return new Position(length1, length2 ?? length1);
    });
    return _tokenParser.TokenParser.oneOf(bothKeywords, numberPlusVertical, numberPlusHorizontal, horizontal.map(h => new Position(h, undefined)), vertical.map(v => new Position(undefined, v)), numbersOnly);
  }
}
exports.Position = Position;