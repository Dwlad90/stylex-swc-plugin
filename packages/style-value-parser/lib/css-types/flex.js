"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Flex = void 0;
var _tokenParser = require("../token-parser");
class Flex {
  constructor(fraction) {
    this.fraction = fraction;
  }
  toString() {
    return `${this.fraction}fr`;
  }
  static get parser() {
    return _tokenParser.TokenParser.tokens.Dimension.map(dim => dim[4].unit === 'fr' && dim[4].signCharacter !== '-' ? dim[4].value : null).where(val => val != null).map(value => new Flex(value));
  }
}
exports.Flex = Flex;